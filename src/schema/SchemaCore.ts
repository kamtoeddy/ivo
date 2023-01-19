import { belongsTo, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  LifeCycles,
  Private_ISchemaOptions,
  PropDefinitionRule,
  Schema as ns,
  StringKey,
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
  "onChange",
  "onCreate",
  "onDelete",
  "onFailure",
  "onSuccess",
  "onUpdate",
  "readonly",
  "resolver",
  "required",
  "requiredError",
  "sideEffect",
  "shouldInit",
  "validator",
  "value",
] as PropDefinitionRule[];

const allowedOptions: OptionsKey[] = ["errors", "timestamps"];
const constantRules = [
  "constant",
  "onCreate",
  "onDelete",
  "onSuccess",
  "value",
];
const sideEffectRules = [
  "sideEffect",
  "onFailure",
  "onSuccess",
  "required",
  "requiredError",
  "shouldInit",
  "validator",
];

const lifeCycleRules: LifeCycles.Rule[] = [
  "onChange",
  "onCreate",
  "onDelete",
  "onFailure",
  "onSuccess",
  "onUpdate",
];

export abstract class SchemaCore<T extends ObjectType> {
  protected _options: ns.Options;
  protected _propDefinitions = {} as ns.Definitions<T>;

  protected context: T = {} as T;
  protected finalContext: T = {} as T;
  protected defaults: Partial<T> = {};
  protected values: Partial<T> = {};
  protected dependencyMap: ns.DependencyMap<T> = {};

  // props
  protected constants: StringKey<T>[] = [];
  protected dependents: StringKey<T>[] = [];
  protected laxProps: StringKey<T>[] = [];
  protected props: StringKey<T>[] = [];
  protected propsRequiredBy: StringKey<T>[] = [];
  protected readonlyProps: StringKey<T>[] = [];
  protected requiredProps: StringKey<T>[] = [];
  protected sideEffects: StringKey<T>[] = [];

  // helpers
  protected optionsTool: OptionsTool;

  constructor(
    propDefinitions: ns.Definitions<T>,
    options: ns.Options = defaultOptions
  ) {
    this._propDefinitions = propDefinitions;
    this._options = options;

    this._checkPropDefinitions();
    this._checkOptions();

    this.optionsTool = new OptionsTool(this._makeTimestamps());
  }

  // context methods
  protected _getContext = () => Object.freeze(Object.assign({}, this.context));

  protected _getFinalContext = () =>
    Object.freeze(Object.assign({}, this.finalContext));

  protected _initContexts = () => {
    this.context = { ...this.values } as T;
    this.finalContext = {} as T;

    const contstants = this._getKeysAsProps(this.context).filter(
      this._isConstant
    );

    if (contstants.length)
      contstants.forEach((prop) =>
        this._updateFinalContext({ [prop]: this.context[prop] } as T)
      );
  };

  protected _updateContext = (updates: Partial<T>) => {
    this.context = { ...this.context, ...updates };
  };

  protected _updateFinalContext = (updates: Partial<T>) => {
    this.finalContext = { ...this.finalContext, ...updates };
  };

  // dependency map utils
  private _addDependencies = (
    prop: StringKey<T>,
    dependsOn: StringKey<T> | StringKey<T>[]
  ) => {
    const _dependsOn = toArray(dependsOn) as StringKey<T>[];

    for (let _prop of _dependsOn)
      if (this.dependencyMap[_prop]) this.dependencyMap[_prop]?.push(prop);
      else this.dependencyMap[_prop] = [prop];
  };

  private _getDependencies = (prop: StringKey<T>) =>
    this.dependencyMap[prop] ?? [];

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

      const _dependsOn = toArray(dependsOn);

      if (_dependsOn.includes(prop))
        error.add(prop, "A property cannot depend on itself");
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
    this._getValueBy(prop, "value", "onCreate");

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

  protected _getDetailedListeners = (
    prop: string,
    lifeCycle: LifeCycles.Rule,
    valid = true
  ) => {
    const listeners = toArray(this._getDefinition(prop)?.[lifeCycle]);

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
      ? this._getListeners(prop, "onChange")
      : ([] as LifeCycles.ChangeListener<T>[]);

    if (this._isSideEffect(prop)) return { listeners: [], onChangeListeners };

    const listeners = this._getListeners(prop, lifeCycle);

    return { listeners, onChangeListeners };
  };

  protected _getListeners = (prop: string, lifeCycle: LifeCycles.Rule) => {
    return this._getDetailedListeners(prop, lifeCycle, true).map(
      (dt) => dt.listener
    ) as LifeCycles.Listener<T>[];
  };

  protected _getValidator = (prop: string) =>
    this._getDefinition(prop)?.validator;

  protected _getKeysAsProps = (data: any) =>
    Object.keys(data) as StringKey<T>[];

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
          "Constant properties can only have ('constant' & 'value') or 'onCreate' | 'onDelete' | 'onSuccess'",
      };

    this.constants.push(prop as StringKey<T>);

    return { valid: true };
  };

  protected _isConstant = (prop: string) =>
    this.constants.includes(prop as StringKey<T>);

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

    if (toArray(dependsOn).includes(prop))
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

    if (!isEqual(required, undefined) && typeof required !== "function")
      return {
        valid,
        reason: "Dependent properties cannot be strictly required",
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

    this.dependents.push(prop as StringKey<T>);
    this._addDependencies(prop as StringKey<T>, dependsOn);

    return { valid: true };
  };

  protected _isDependentProp = (prop: string) =>
    this.dependents.includes(prop as StringKey<T>);

  protected _isFunction = (obj: any) => typeof obj === "function";

  protected _isLaxProp = (prop: string) =>
    this.laxProps.includes(prop as StringKey<T>);

  protected _isProp = (prop: string) =>
    this.props.includes(prop as StringKey<T>);

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

    if (this._hasAny(prop, "constant")) {
      const constantDef = this.__isConstantProp(prop);

      if (!constantDef.valid) reasons.push(constantDef.reason!);
    }

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

    if (this._hasAny(prop, "requiredError")) {
      const { required, requiredError } = this._getDefinition(prop);

      if (typeof required != "function")
        reasons.push(
          "RequiredError can only be used with a callable required rule"
        );
      if (!belongsTo(typeof requiredError, ["string", "function"]))
        reasons.push("RequiredError must be a string or setter");
    }

    if (this._hasAny(prop, "sideEffect")) {
      const sideEffectDef = this.__isSideEffect(prop);

      if (!sideEffectDef.valid) reasons.push(sideEffectDef.reason!);
    }

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

    // onChange, onCreate, onDelete, onFailure, onSuccess & onUpdate
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
      this.props.push(prop as StringKey<T>);
      this._setDefaultOf(prop as StringKey<T>, "onCreate");
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

    this.readonlyProps.push(prop as StringKey<T>);

    return { valid: true };
  };

  protected _isReadonly = (prop: string) =>
    this.readonlyProps.includes(prop as StringKey<T>);

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

    if (this._hasAny(prop, "requiredError"))
      return {
        valid,
        reason: "Strictly required properties cannot have a requiredError",
      };

    const isRequiredCommon = this.__isRequiredCommon(prop);

    if (!isRequiredCommon.valid) return isRequiredCommon;

    this.requiredProps.push(prop as StringKey<T>);

    return { valid: true };
  };

  protected _isRequired = (prop: string) =>
    this.requiredProps.includes(prop as StringKey<T>);

  protected __isRequiredBy = (prop: string) => {
    const {
      default: _default,
      required,
      requiredError,
    } = this._getDefinition(prop);

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

    if (isEqual(requiredError, undefined))
      return {
        valid,
        reason:
          "Callable required properties must have a requiredError or setter",
      };

    if (!hasSideEffect) {
      const isRequiredCommon = this.__isRequiredCommon(prop);

      if (!isRequiredCommon.valid) return isRequiredCommon;
    }

    this.propsRequiredBy.push(prop as StringKey<T>);

    return { valid: true };
  };

  protected _isRequiredBy = (prop: string) =>
    this.propsRequiredBy.includes(prop as StringKey<T>);

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

    const { sideEffect, shouldInit } = this._getDefinition(prop);

    if (sideEffect !== true)
      return { valid, reason: "SideEffects must have sideEffect as 'true'" };

    if (!this._isValidatorOk(prop))
      return { valid, reason: "Invalid validator" };

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

    this.sideEffects.push(prop as StringKey<T>);

    return { valid: true };
  };

  protected _isSideEffect = (prop: string) =>
    this.sideEffects.includes(prop as StringKey<T>);

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

    this.laxProps.push(prop as StringKey<T>);
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
    prop: StringKey<T>,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const _default = this._getValueBy(prop, "default", lifeCycle);

    if (!isEqual(_default, undefined)) this.defaults[prop] = _default;
  };
}
