import { belongsTo, sort, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  LifeCycles,
  Private_ISchemaOptions,
  PropDefinitionRule,
  Schema as ns,
  StringKey,
  Validator,
} from "./interfaces";
import { OptionsTool } from "./utils/options-tool";
import { ErrorTool } from "./utils/schema-error";

export const defaultOptions = {
  errors: "silent",
  timestamps: false,
} as ns.Options;

type OptionsKey = StringKey<ns.Options>;

const allRules = [
  "constant",
  "default",
  "dependent",
  "dependsOn",
  "onDelete",
  "onFailure",
  "onSuccess",
  "readonly",
  "resolver",
  "required",
  "sanitizer",
  "sideEffect",
  "shouldInit",
  "validator",
  "value",
] as PropDefinitionRule[];

const allowedOptions: OptionsKey[] = ["errors", "timestamps"];
const constantRules = ["constant", "onDelete", "onSuccess", "value"];
const sideEffectRules = [
  "sanitizer",
  "sideEffect",
  "onFailure",
  "onSuccess",
  "required",
  "shouldInit",
  "validator",
];

const lifeCycleRules: LifeCycles.Rule[] = [
  "onDelete",
  "onFailure",
  "onSuccess",
];

export abstract class SchemaCore<I extends ObjectType> {
  protected _options: ns.Options;
  protected _propDefinitions = {} as ns.Definitions<I>;

  protected context: I = {} as I;
  protected finalContext: I = {} as I;
  protected defaults: Partial<I> = {};
  protected values: Partial<I> = {};
  protected dependencyMap: ns.DependencyMap<I> = {};

  // props
  protected constants: StringKey<I>[] = [];
  protected dependents: StringKey<I>[] = [];
  protected laxProps: StringKey<I>[] = [];
  protected props: StringKey<I>[] = [];
  protected propsRequiredBy: StringKey<I>[] = [];
  protected readonlyProps: StringKey<I>[] = [];
  protected requiredProps: StringKey<I>[] = [];
  protected sideEffects: StringKey<I>[] = [];

  // helpers
  protected optionsTool: OptionsTool;

  constructor(
    propDefinitions: ns.Definitions<I>,
    options: ns.Options = defaultOptions
  ) {
    this._propDefinitions = propDefinitions;
    this._options = options;

    this._checkPropDefinitions();
    this._checkOptions();

    this.optionsTool = new OptionsTool(this._makeTimestamps());
  }

  // < context methods >
  protected _getContext = () => Object.freeze(Object.assign({}, this.context));

  protected _getFinalContext = () =>
    Object.freeze(Object.assign({}, this.finalContext));

  protected _initContexts = () => {
    this.context = { ...this.values } as I;
    this.finalContext = {} as I;

    const contstants = this._getKeysAsProps(this.context).filter(
      this._isConstant
    );

    if (contstants.length)
      contstants.forEach((prop) =>
        this._updateFinalContext({ [prop]: this.context[prop] } as I)
      );
  };

  protected _updateContext = (updates: Partial<I>) => {
    this.context = { ...this.context, ...updates };
  };

  protected _updateFinalContext = (updates: Partial<I>) => {
    this.finalContext = { ...this.finalContext, ...updates };
  };
  // < context methods />

  // < dependency map utils >
  private _addDependencies = (
    prop: StringKey<I>,
    dependsOn: StringKey<I> | StringKey<I>[]
  ) => {
    const _dependsOn = toArray(dependsOn) as StringKey<I>[];

    for (let _prop of _dependsOn)
      if (this.dependencyMap[_prop]) this.dependencyMap[_prop]?.push(prop);
      else this.dependencyMap[_prop] = [prop];
  };

  protected _getDependencies = (prop: StringKey<I>) =>
    this.dependencyMap[prop] ?? [];

  private _getCircularDependenciesOf = (a: StringKey<I>) => {
    let circularDependencies: string[] = [];

    const _dependsOn = toArray<StringKey<I>>(
      this._getDefinition(a)?.dependsOn ?? []
    );

    for (const _prop of _dependsOn)
      circularDependencies = [
        ...circularDependencies,
        ...this._getCircularDependenciesOf_a_in_b(a, _prop),
      ];

    return sort(Array.from(new Set(circularDependencies)));
  };

  private _getCircularDependenciesOf_a_in_b = (
    a: StringKey<I>,
    b: StringKey<I>,
    visitedNodes: StringKey<I>[] = []
  ) => {
    let circularDependencies: string[] = [];

    if (!this._isDependentProp(b) || visitedNodes.includes(b)) return [];

    visitedNodes.push(b);

    const _dependsOn = toArray<StringKey<I>>(
      this._getDefinition(b)?.dependsOn ?? []
    );

    for (const _prop of _dependsOn) {
      if (_prop == a) circularDependencies.push(b);
      else if (this._isDependentProp(_prop))
        circularDependencies = [
          ...circularDependencies,
          ...this._getCircularDependenciesOf_a_in_b(a, _prop, visitedNodes),
        ];
    }

    return sort(Array.from(new Set(circularDependencies)));
  };
  // < dependency map utils />

  protected _canInit = (prop: string) => {
    if (this._isDependentProp(prop)) return false;
    if (this._isRequired(prop)) return true;

    const { readonly, shouldInit } = this._getDefinition(prop);

    return (
      readonly === true &&
      belongsTo(shouldInit, [true, undefined]) &&
      !this._isRequiredBy(prop)
    );
  };

  protected _checkOptions = () => {
    const error = new ErrorTool({ message: "Invalid Schema", statusCode: 500 });

    if (
      !this._options ||
      typeof this._options !== "object" ||
      Array.isArray(this._options)
    )
      error.add("schema options", "Must be an object").throw();

    let options = Object.keys(this._options) as OptionsKey[];

    if (!options.length) error.add("schema options", "Cannot be empty").throw();

    for (let option of options)
      if (!allowedOptions.includes(option))
        error.add(option, "Invalid option").throw();

    if (this._options.hasOwnProperty("errors"))
      if (!["silent", "throw"].includes(this._options.errors!))
        error.add("errors", "should be 'silent' or 'throw'").throw();

    if (this._options.hasOwnProperty("timestamps")) {
      const ts_valid = this._isTimestampsOk();

      if (!ts_valid.valid) error.add("timestamps", ts_valid.reason!).throw();
    }
  };

  protected _checkPropDefinitions = () => {
    const error = new ErrorTool({ message: "Invalid Schema", statusCode: 500 });

    if (
      !this._propDefinitions ||
      typeof this._propDefinitions !== "object" ||
      Array.isArray(this._propDefinitions)
    )
      error.throw();

    let props: string[] = Object.keys(this._propDefinitions);

    if (!props.length)
      error.add("schema properties", "Insufficient Schema properties").throw();

    for (let prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop);
      if (!isDefOk.valid) error.add(prop, isDefOk.reasons!);
    }

    // make sure every side effect has atleast one dependency
    for (let prop of this.sideEffects) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length)
        error.add(
          prop,
          "A side effect must have atleast one property that depends on it"
        );
    }

    // make sure every side effect has atleast one dependency
    for (let prop of this.dependents) {
      const { dependsOn } = this._getDefinition(prop);

      const _dependsOn = toArray<StringKey<I>>(dependsOn ?? []);

      if (_dependsOn.includes(prop))
        error.add(prop, "A property cannot depend on itself");

      const dependsOnConstantProp = _dependsOn.some(this._isConstant);

      if (dependsOnConstantProp)
        error.add(prop, "A property cannot depend on a constant property");

      // check against circular dependencies
      const circularRelationShips = this._getCircularDependenciesOf(prop);

      for (const _prop of circularRelationShips)
        error.add(prop, `Circular dependency identified with '${_prop}'`);

      // check against dependencies on invalid properties
      const invalidProps = _dependsOn.filter(
        (p) => !(this._isProp(p) || this._isSideEffect(p))
      );

      for (const _prop of invalidProps)
        error.add(
          prop,
          `Cannot establish dependency with '${_prop}' as it is neither a property nor a side effect of your model`
        );
    }

    if (error.isPayloadLoaded) error.throw();
  };

  protected _getDefinition = (prop: string) => this._propDefinitions[prop]!;

  protected _getDefaultValue = (prop: string) => {
    const _default = this._getDefinition(prop)?.default;

    const value = this._isFunction(_default)
      ? _default(this._getContext())
      : this.defaults[prop];

    return isEqual(value, undefined) ? this.values[prop] : value;
  };

  protected _getConstantValue = async (prop: string) =>
    this._getValueBy(prop, "value", "creating");

  protected _getValueBy = (
    prop: string,
    rule: PropDefinitionRule,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const _default = this._getDefinition(prop)?.[rule];

    return this._isFunction(_default)
      ? _default(this._getContext(), lifeCycle)
      : _default;
  };

  protected _getRequiredState = (
    prop: string,
    lifeCycle: LifeCycles.LifeCycle
  ): [boolean, string] => {
    const { required } = this._getDefinition(prop);

    if (!required) return [false, ""];

    const fallbackMessage = `'${prop}' is required!`;

    if (!this._isFunction(required)) return [required, fallbackMessage];

    const results = required(this._getContext(), lifeCycle);

    if (typeof results == "boolean")
      return [results, results ? fallbackMessage : ""];

    if (!results[1]) results[1] = fallbackMessage;

    return results;
  };

  protected _getDetailedListeners = <T>(
    prop: string,
    lifeCycle: LifeCycles.Rule,
    valid = true
  ) => {
    const listeners = toArray<LifeCycles.Listener<T>>(
      this._getDefinition(prop)?.[lifeCycle] as any
    );

    return (
      listeners
        ?.map((listener, index) => ({
          index,
          listener,
          valid: this._isFunction(listener),
        }))
        .filter((data) => data.valid === valid) ?? []
    );
  };

  protected _getOperationListeners = (
    prop: string,
    lifeCycle: LifeCycles.Rule
  ) => {
    const onChangeListeners = this._isChangeLifeCycle(lifeCycle)
      ? this._getListeners(prop, "onDelete")
      : [];

    if (this._isSideEffect(prop)) return { listeners: [], onChangeListeners };

    const listeners = this._getListeners(prop, lifeCycle);

    return { listeners, onChangeListeners };
  };

  protected _getListeners = <T>(prop: string, lifeCycle: LifeCycles.Rule) => {
    return this._getDetailedListeners<T>(prop, lifeCycle, true).map(
      (dt) => dt.listener
    ) as LifeCycles.Listener<T>[];
  };

  protected _getInvalidRules = <K extends StringKey<I>>(prop: K) => {
    const rulesProvided = this._getKeysAsProps<PropDefinitionRule>(
      this._getDefinition(prop)
    );

    return rulesProvided.filter((r) => !allRules.includes(r));
  };

  protected _getValidator = <K extends StringKey<I>>(prop: K) => {
    return this._getDefinition(prop)?.validator as Validator<K, I> | undefined;
  };

  protected _getKeysAsProps = <T = StringKey<I>>(data: any) =>
    Object.keys(data) as T[];

  protected _hasAny = (
    prop: string,
    rules: PropDefinitionRule | PropDefinitionRule[]
  ): boolean => {
    for (let _prop of toArray(rules))
      if (this._getDefinition(prop)?.hasOwnProperty(_prop)) return true;

    return false;
  };

  protected _isChangeLifeCycle = (lifeCycle: LifeCycles.Rule) =>
    ["onCreate", "onUpdate"].includes(lifeCycle);

  protected __isConstantProp = (prop: string) => {
    const { constant, value } = this._getDefinition(prop);

    const valid = false;

    if (constant !== true)
      return {
        valid,
        reason: "Constant properties must have constant as 'true'",
      };

    if (!this._hasAny(prop, "value"))
      return {
        valid,
        reason: "Constant properties must have a value or setter",
      };

    if (isEqual(value, undefined))
      return {
        valid,
        reason: "Constant properties cannot have 'undefined' as value",
      };

    const unAcceptedRules = allRules.filter(
      (rule) => !constantRules.includes(rule)
    );

    if (this._hasAny(prop, unAcceptedRules))
      return {
        valid,
        reason:
          "Constant properties can only have ('constant' & 'value') or 'onDelete' | 'onSuccess'",
      };

    this.constants.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected _isConstant = (prop: string) =>
    this.constants.includes(prop as StringKey<I>);

  protected __isDependentProp = (prop: string) => {
    const {
      default: _default,
      dependent,
      dependsOn,
      shouldInit,
      readonly,
      resolver,
      required,
    } = this._getDefinition(prop);

    const valid = false;

    if (dependent !== true)
      return {
        valid,
        reason: "Dependent properties must have dependent as 'true'",
      };

    if (isEqual(_default, undefined))
      return {
        valid,
        reason: "Dependent properties must have a default value",
      };

    if (isEqual(dependsOn, undefined) || !dependsOn?.length)
      return {
        valid,
        reason: "Dependent properties must depend on atleast one property",
      };

    if (toArray(dependsOn).includes(prop as StringKey<I>))
      return { valid, reason: "A property cannot depend on itself" };

    if (isEqual(resolver, undefined))
      return {
        valid,
        reason: "Dependent properties must have a resolver",
      };

    if (!this._isFunction(resolver))
      return {
        valid,
        reason: "The resolver of a dependent property must be a function",
      };

    if (this._hasAny(prop, "validator"))
      return {
        valid,
        reason: "Dependent properties cannot be validated",
      };

    if (this._hasAny(prop, "required"))
      return {
        valid,
        reason: "Dependent properties cannot be required",
      };

    if (readonly === "lax")
      return { valid, reason: "Dependent properties cannot be readonly 'lax'" };

    if (!isEqual(shouldInit, undefined))
      return {
        valid,
        reason: "Dependent properties cannot have shouldInit rule",
      };

    if (this._hasAny(prop, "sideEffect"))
      return { valid, reason: "Dependent properties cannot be sideEffect" };

    this.dependents.push(prop as StringKey<I>);
    this._addDependencies(prop as StringKey<I>, dependsOn);

    return { valid: true };
  };

  protected _isDependentProp = (prop: string) =>
    this.dependents.includes(prop as StringKey<I>);

  protected _isFunction = (v: any): v is Function => typeof v === "function";

  protected _isLaxProp = (prop: string) =>
    this.laxProps.includes(prop as StringKey<I>);

  protected _isProp = (prop: string) =>
    this.props.includes(prop as StringKey<I>);

  protected _isPropDefinitionObjectOk = (prop: string) => {
    const propDef = this._getDefinition(prop);

    return propDef && typeof propDef === "object" && !Array.isArray(propDef)
      ? { valid: true }
      : {
          reasons: ["Property definitions must be an object"],
          valid: false,
        };
  };

  protected __isPropDefinitionOk = (prop: string) => {
    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) isPopDefOk;

    let reasons: string[] = [];

    const invalidRulesProvided = this._getInvalidRules(prop as StringKey<I>);

    if (invalidRulesProvided.length)
      for (let rule of invalidRulesProvided)
        reasons.push(`'${rule}' is not a valid rule`);

    if (this._hasAny(prop, "constant")) {
      const constantDef = this.__isConstantProp(prop);

      if (!constantDef.valid) reasons.push(constantDef.reason!);
    } else if (this._hasAny(prop, "value"))
      reasons.push("'value' rule can only be used with constant properties");

    if (this._hasAny(prop, "dependent")) {
      const dependentDef = this.__isDependentProp(prop);

      if (!dependentDef.valid) reasons.push(dependentDef.reason!);
    } else if (this._hasAny(prop, ["dependsOn", "resolver"]))
      reasons.push(
        "dependsOn & resolver rules can only belong to dependent properties"
      );

    if (this._hasAny(prop, "readonly")) {
      const readonlyDef = this.__isReadonly(prop);

      if (!readonlyDef.valid) reasons.push(readonlyDef.reason!);
    }

    if (this._hasAny(prop, "required")) {
      const { required } = this._getDefinition(prop);

      const requiredDef =
        typeof required === "function"
          ? this.__isRequiredBy(prop)
          : this.__isRequired(prop);

      if (!requiredDef.valid) reasons.push(requiredDef.reason!);
    }

    if (this._hasAny(prop, "sideEffect")) {
      const sideEffectDef = this.__isSideEffect(prop);

      if (!sideEffectDef.valid) reasons.push(sideEffectDef.reason!);
    } else if (this._hasAny(prop, "sanitizer"))
      reasons.push("'sanitizer' is only valid on side effects");

    if (this._hasAny(prop, "shouldInit")) {
      const { shouldInit } = this._getDefinition(prop);

      if (!["boolean", "function"].includes(typeof shouldInit))
        reasons.push(
          "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean"
        );

      if (!this._hasAny(prop, ["default", "sideEffect"]))
        reasons.push(
          "A property with initialization blocked must have a default value"
        );
    }

    if (this._hasAny(prop, "validator") && !this._isValidatorOk(prop))
      reasons.push("Invalid validator");

    if (this._hasAny(prop, "onFailure") && !this._hasAny(prop, "validator"))
      reasons.push(
        "'onFailure' can only be used with properties that support and have validators"
      );

    // onDelete, onFailure, & onSuccess
    for (let rule of lifeCycleRules) {
      if (!this._hasAny(prop, rule)) continue;

      const invalidHandlers = this._getDetailedListeners(prop, rule, false);

      if (!invalidHandlers?.length) continue;

      reasons = reasons.concat(
        invalidHandlers.map(
          (dt) => `The listener for '${rule}' @[${dt.index}] is not a function`
        )
      );
    }

    this._registerIfLax(prop);

    if (
      !this._hasAny(prop, "default") &&
      !this._isConstant(prop) &&
      !this._isDependentProp(prop) &&
      !this._isLaxProp(prop) &&
      !this._isReadonly(prop) &&
      !this._isRequired(prop) &&
      !this._isSideEffect(prop)
    ) {
      reasons.push(
        "A property should at least be readonly, required, or have a default value"
      );
    }

    const valid = reasons.length ? false : true;

    if (valid && !this._isSideEffect(prop)) {
      this.props.push(prop as StringKey<I>);
      this._setDefaultOf(prop as StringKey<I>, "creating");
    }

    return { reasons, valid };
  };

  protected _isPropDefinitionOk = (prop: string): boolean =>
    this.__isPropDefinitionOk(prop).valid;

  protected __isReadonly = (prop: string) => {
    const {
      default: _default,
      dependent,
      readonly,
      required,
      shouldInit,
    } = this._getDefinition(prop);

    const valid = false;

    if (!belongsTo(readonly, [true, "lax"]))
      return {
        reason: "Readonly properties are either true | 'lax'",
        valid,
      };

    if (this._hasAny(prop, "required") && typeof required != "function")
      return {
        valid,
        reason:
          "Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule",
      };

    if (readonly === "lax" && !isEqual(dependent, undefined))
      return { valid, reason: "Readonly(lax) properties cannot be dependent" };

    if (
      (readonly === "lax" || dependent === true || shouldInit === false) &&
      isEqual(_default, undefined)
    )
      return {
        valid,
        reason:
          "readonly properties must have a default value or a default setter",
      };

    if (readonly === "lax" && !isEqual(shouldInit, undefined))
      return {
        valid,
        reason: "Lax properties cannot have initialization blocked",
      };

    if (!belongsTo(readonly, [true, "lax"]))
      return {
        valid,
        reason: "Readonly properties have readonly true | 'lax'",
      };

    this.readonlyProps.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected _isReadonly = (prop: string) =>
    this.readonlyProps.includes(prop as StringKey<I>);

  protected __isRequiredCommon = (prop: string) => {
    const valid = false;

    if (this._hasAny(prop, "dependent"))
      return {
        valid,
        reason: "Required properties cannot be dependent",
      };

    if (this._hasAny(prop, "shouldInit"))
      return {
        valid,
        reason: "Required properties cannot have a initialization blocked",
      };

    if (!this._isValidatorOk(prop))
      return { valid, reason: "Required properties must have a validator" };

    return { valid: true };
  };

  protected __isRequired = (prop: string) => {
    const { required } = this._getDefinition(prop);

    const valid = false;

    if (required !== true)
      return {
        valid,
        reason: "Required properties must have required as 'true'",
      };

    if (this._hasAny(prop, "default"))
      return {
        valid,
        reason:
          "Strictly required properties cannot have a default value or setter",
      };

    if (this._hasAny(prop, "readonly"))
      return {
        valid,
        reason: "Strictly required properties cannot be readonly",
      };

    const isRequiredCommon = this.__isRequiredCommon(prop);

    if (!isRequiredCommon.valid) return isRequiredCommon;

    this.requiredProps.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected _isRequired = (prop: string) =>
    this.requiredProps.includes(prop as StringKey<I>);

  protected __isRequiredBy = (prop: string) => {
    const { default: _default, required } = this._getDefinition(prop);

    const valid = false;

    const requiredType = typeof required;

    if (requiredType !== "function")
      return {
        valid,
        reason: "Callable required properties must have required as a function",
      };

    const hasSideEffect = this._hasAny(prop, "sideEffect");

    if (isEqual(_default, undefined) && !hasSideEffect)
      return {
        valid,
        reason:
          "Callable required properties must have a default value or setter",
      };

    if (!hasSideEffect) {
      const isRequiredCommon = this.__isRequiredCommon(prop);

      if (!isRequiredCommon.valid) return isRequiredCommon;
    }

    this.propsRequiredBy.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected _isRequiredBy = (prop: string) =>
    this.propsRequiredBy.includes(prop as StringKey<I>);

  protected __isSideEffectRequiredBy = (prop: string) => {
    if (this._hasAny(prop, "shouldInit"))
      return {
        valid: false,
        reason: "Required sideEffects cannot have initialization blocked",
      };

    const isRequiredBy = this.__isRequiredBy(prop);

    if (!isRequiredBy.valid) return isRequiredBy;

    return { valid: true };
  };

  protected __isSideEffect = (prop: string) => {
    const valid = false;

    const { sanitizer, sideEffect, shouldInit } = this._getDefinition(prop);

    if (sideEffect !== true)
      return { valid, reason: "SideEffects must have sideEffect as 'true'" };

    if (!this._isValidatorOk(prop))
      return { valid, reason: "Invalid validator" };

    if (this._hasAny(prop, "sanitizer") && !this._isFunction(sanitizer))
      return { valid, reason: "'sanitizer' must be a function" };

    const hasRequired = this._hasAny(prop, "required");

    if (hasRequired) {
      const isRequiredBy = this.__isSideEffectRequiredBy(prop);

      if (!isRequiredBy.valid) return isRequiredBy;
    }

    if (
      !hasRequired &&
      this._hasAny(prop, "shouldInit") &&
      shouldInit !== false
    )
      return {
        valid,
        reason:
          "To block the initialization of side effects shouldInit must be 'false'",
      };

    const unAcceptedRules = allRules.filter(
      (rule) => !sideEffectRules.includes(rule)
    );

    if (this._hasAny(prop, unAcceptedRules))
      return {
        valid,
        reason: `SideEffects properties can only have (${sideEffectRules.join(
          ", "
        )}) as rules`,
      };

    this.sideEffects.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected _isSideEffect = (prop: string) =>
    this.sideEffects.includes(prop as StringKey<I>);

  protected _isSideInit = (prop: string) => {
    const propDef = this._getDefinition(prop);

    if (!propDef) return false;

    const { shouldInit } = propDef;

    return this._isSideEffect(prop) && belongsTo(shouldInit, [true, undefined]);
  };

  protected _registerIfLax = (prop: string) => {
    const {
      default: _default,
      readonly,
      shouldInit,
    } = this._getDefinition(prop);

    // Lax properties must have a default value nor setter
    if (isEqual(_default, undefined)) return;

    // Lax properties cannot be dependent
    if (this._hasAny(prop, "dependent")) return;

    // Lax properties cannot be required
    if (this._hasAny(prop, "required")) return;

    // Lax properties cannot be side effects
    if (this._hasAny(prop, "sideEffect")) return;

    // only readonly(lax) are lax props &
    // Lax properties cannot have initialization blocked
    if (
      (this._hasAny(prop, "readonly") && readonly !== "lax") ||
      (this._hasAny(prop, "shouldInit") && typeof shouldInit != "function")
    )
      return;

    this.laxProps.push(prop as StringKey<I>);
  };

  private _isTimestampsOk() {
    const { timestamps } = this._options,
      valid = false;

    const ts_type = typeof timestamps;

    if (ts_type === "boolean") return { valid: true };

    if (
      ts_type !== "object" ||
      (ts_type === "object" && (!timestamps || Array.isArray(timestamps)))
    )
      return {
        valid,
        reason: "should be 'boolean' or 'non null object'",
      };

    if (!Object.keys(timestamps!).length)
      return { valid, reason: "cannot be an empty object" };

    const { createdAt, updatedAt } = timestamps as {
      createdAt: "";
      updatedAt: "";
    };

    const _props = this.props as string[];

    for (const ts_key of [createdAt, updatedAt]) {
      if (ts_key && _props?.includes(ts_key))
        return { valid, reason: `'${ts_key}' already belongs to your schema` };
    }

    if (createdAt === updatedAt)
      return { valid, reason: "createdAt & updatedAt cannot be same" };

    return { valid: true };
  }

  private _isValidatorOk = (prop: string) =>
    this._isFunction(this._getDefinition(prop)?.validator);

  private _makeTimestamps(): Private_ISchemaOptions {
    const options = this._options;

    if (!options) return { timestamps: { createdAt: "", updatedAt: "" } };

    let { timestamps } = options;

    let createdAt = "createdAt",
      updatedAt = "updatedAt";

    if (!timestamps || timestamps === true) {
      let _timestamps = timestamps
        ? { createdAt, updatedAt }
        : { createdAt: "", updatedAt: "" };

      return { ...options, timestamps: _timestamps };
    }

    const custom_createdAt = timestamps?.createdAt;
    const custom_updatedAt = timestamps?.updatedAt;

    if (custom_createdAt) createdAt = custom_createdAt;
    if (custom_updatedAt) updatedAt = custom_updatedAt;

    return { ...options, timestamps: { createdAt, updatedAt } };
  }

  private _setDefaultOf = (
    prop: StringKey<I>,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const _default = this._getValueBy(prop, "default", lifeCycle);

    if (!isEqual(_default, undefined)) this.defaults[prop] = _default;
  };
}
