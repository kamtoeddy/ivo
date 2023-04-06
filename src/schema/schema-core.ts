import { belongsTo, sort, sortKeys, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  DefinitionRule,
  Context,
  ISchema as ns,
  StringKey,
  Summary,
  Validator,
  ALLOWED_OPTIONS,
  DEFINITION_RULES,
  CONSTANT_RULES,
  VIRTUAL_RULES,
} from "./interfaces";
import { OptionsTool } from "./utils/options-tool";
import { ErrorTool } from "./utils/schema-error";

export const defaultOptions = {
  errors: "silent",
  timestamps: false,
} as ns.Options<any, any>;

const lifeCycleRules: ns.LifeCycles[] = ["onDelete", "onFailure", "onSuccess"];

export abstract class SchemaCore<I, O> {
  protected _options: ns.Options<I, O>;
  protected _definitions = {} as ns.Definitions_<I, O>;

  // contexts & values
  protected context: Context<I, O> = {} as Context<I, O>;
  protected partialContext: Context<I, O> = {} as Context<I, O>;
  protected defaults: Partial<O> = {};
  protected values: O = {} as O;

  // maps
  protected aliasToVirtualMap: ns.AliasToVirtualMap<I> = {};
  protected dependencyMap: ns.DependencyMap<I> = {};
  protected virtualToAliasMap: ns.AliasToVirtualMap<I> = {};

  // props
  protected constants: StringKey<I>[] = [];
  protected dependents: StringKey<I>[] = [];
  protected laxProps: StringKey<I>[] = [];
  protected props: StringKey<O>[] = [];
  protected propsRequiredBy: StringKey<I>[] = [];
  protected readonlyProps: StringKey<I>[] = [];
  protected requiredProps: StringKey<I>[] = [];
  protected virtuals: StringKey<I>[] = [];

  // helpers
  protected optionsTool: OptionsTool;

  // listeners
  protected globalSuccessHandlers: ns.SuccessHandler<I, O>[] = [];

  constructor(
    definitions: ns.Definitions_<I, O>,
    options: ns.Options<I, O> = defaultOptions as ns.Options<I, O>
  ) {
    this._definitions = definitions;
    this._options = options;

    this._checkPropDefinitions();
    this._checkOptions();

    this.optionsTool = new OptionsTool(this._makeTimestamps());
  }

  // < context methods >
  protected _getContext = () =>
    this._getFrozenCopy(sortKeys(this.context)) as Context<I, O>;

  protected _getPartialContext = () => this._getFrozenCopy(this.partialContext);

  protected _initialiseContexts = () => {
    this.context = { ...this.values } as Context<I, O>;
    this.partialContext = {} as Context<I, O>;
  };

  protected _updateContext = (updates: Partial<I>) => {
    this.context = { ...this.context, ...updates };
  };

  protected _updatePartialContext = (updates: Partial<I>) => {
    this.partialContext = { ...this.partialContext, ...updates };
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

  protected _getDependencies = (prop: string) =>
    this.dependencyMap[prop as StringKey<I>] ?? [];

  protected _getAliasByVirtual = (prop: StringKey<I>): string | undefined =>
    this.virtualToAliasMap[prop];

  protected _getVirtualByAlias = (alias: string): StringKey<I> | undefined =>
    this.aliasToVirtualMap[alias];

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

  protected _getFrozenCopy = <T>(data: T): Readonly<T> =>
    Object.freeze(Object.assign({}, data)) as Readonly<T>;

  protected _canInit = (prop: string) => {
    if (this._isDependentProp(prop)) return false;
    if (this._isRequired(prop)) return true;

    const { readonly } = this._getDefinition(prop);

    const shouldInit = this._getValueBy(prop, "shouldInit");

    return (
      readonly === true &&
      belongsTo(shouldInit, [true, undefined]) &&
      !this._isRequiredBy(prop)
    );
  };

  private _checkOptions = () => {
    const error = new ErrorTool({ message: "Invalid Schema", statusCode: 500 });

    if (
      !this._options ||
      typeof this._options !== "object" ||
      Array.isArray(this._options)
    )
      error.add("schema options", "Must be an object").throw();

    let options = Object.keys(this._options) as ns.OptionsKey<I, O>[];

    if (!options.length) error.add("schema options", "Cannot be empty").throw();

    for (let option of options)
      if (!ALLOWED_OPTIONS.includes(option))
        error.add(option, "Invalid option").throw();

    if (this._options.hasOwnProperty("errors"))
      if (!["silent", "throw"].includes(this._options.errors!))
        error.add("errors", "should be 'silent' or 'throw'").throw();

    if (this._options.hasOwnProperty("onSuccess")) {
      const isValid = this._isOnSuccessOptionOk();

      if (!isValid.valid) error.add("onSuccess", isValid.reasons!).throw();
    }

    if (this._options.hasOwnProperty("timestamps")) {
      const isValid = this._isTimestampsOptionOk();

      if (!isValid.valid) error.add("timestamps", isValid.reason!).throw();
    }
  };

  private _checkPropDefinitions = () => {
    const error = new ErrorTool({ message: "Invalid Schema", statusCode: 500 });

    if (
      !this._definitions ||
      typeof this._definitions !== "object" ||
      Array.isArray(this._definitions)
    )
      error.throw();

    let props: string[] = Object.keys(this._definitions);

    if (!props.length)
      error.add("schema properties", "Insufficient Schema properties").throw();

    for (let prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop);

      if (!isDefOk.valid) error.add(prop, isDefOk.reasons!);
    }

    // make sure every virtual property has atleast one dependency
    for (let prop of this.virtuals) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length)
        error.add(
          prop,
          "A virtual property must have atleast one property that depends on it"
        );
    }

    // make sure aliases respect the second validation rules
    for (const [alias, prop] of Object.entries(this.aliasToVirtualMap)) {
      const isValid = this.__isVirtualAliasOk2(alias);

      if (!isValid.valid) error.add(prop, isValid.reason);
    }

    // make sure every virtual has atleast one dependency
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
        (p) => !(this._isProp(p) || this._isVirtual(p))
      );

      for (const _prop of invalidProps)
        error.add(
          prop,
          `Cannot establish dependency with '${_prop}' as it is neither a property nor a virtual of your model`
        );
    }

    if (error.isPayloadLoaded) error.throw();
  };

  protected _getDefinition = (prop: string) =>
    this._definitions[prop as StringKey<I>]!;

  protected _getDefaultValue = async (prop: string) => {
    const _default = this._getDefinition(prop)?.default;

    let value = this._isFunction(_default)
      ? await _default(this._getContext())
      : this.defaults[prop as StringKey<O>];

    return isEqual(value, undefined)
      ? this.values[prop as StringKey<O>]
      : value;
  };

  protected _getConstantValue = async (prop: string) =>
    this._getValueBy(prop, "value");

  protected _getValueBy = (
    prop: string,
    rule: DefinitionRule,
    extraCtx: ObjectType = {}
  ) => {
    const value = this._getDefinition(prop)?.[rule];

    return this._isFunction(value)
      ? value({ ...this._getContext(), ...extraCtx })
      : value;
  };

  protected _getRequiredState = (
    prop: string,
    summary: Summary<I, O>
  ): [boolean, string] => {
    const { required } = this._getDefinition(prop);

    if (!required) return [false, ""];

    const fallbackMessage = `'${prop}' is required!`;

    if (!this._isFunction(required)) return [required, fallbackMessage];

    let results = required(summary);

    const datatype = typeof results;

    if (datatype !== "boolean" && !Array.isArray(results)) return [false, ""];

    if (datatype === "boolean")
      return [results as boolean, results ? fallbackMessage : ""];

    results = results as [boolean, string];

    if (!results[1] || typeof results[1] != "string")
      results[1] = fallbackMessage;

    return results;
  };

  protected _getHandlers = <T>(prop: string, lifeCycle: ns.LifeCycles) =>
    toArray((this._getDefinition(prop)?.[lifeCycle] ?? []) as any) as T[];

  private _getInvalidRules = <K extends StringKey<I>>(prop: K) => {
    const rulesProvided = this._getKeysAsProps(this._getDefinition(prop));

    return rulesProvided.filter(
      (r) => !DEFINITION_RULES.includes(r as DefinitionRule)
    );
  };

  protected _getValidator = <K extends StringKey<I>>(prop: K) => {
    return this._getDefinition(prop)?.validator as
      | Validator<K, I, O>
      | undefined;
  };

  protected _getKeysAsProps = <T extends ObjectType>(data: T) =>
    Object.keys(data) as StringKey<T>[];

  private _areHandlersOk = (prop: string, lifeCycle: ns.LifeCycles) => {
    const handlers = toArray<ns.FailureHandler<I, O>>(
      this._getDefinition(prop)?.[lifeCycle] as any
    );

    const reasons: string[] = [];

    handlers.forEach((handler, index) => {
      if (!this._isFunction(handler))
        reasons.push(
          `The '${lifeCycle}' handler @[${index}] is not a function`
        );
    });

    const valid = !reasons.length;

    return valid ? { listeners: handlers, valid } : { reasons, valid };
  };

  protected _isRuleInDefinition = (
    prop: string,
    rules: DefinitionRule | DefinitionRule[]
  ): boolean => {
    for (let _prop of toArray(rules))
      if (this._getDefinition(prop)?.hasOwnProperty(_prop)) return true;

    return false;
  };

  private __isConstantProp = (prop: string) => {
    const { constant, value } = this._getDefinition(prop);

    const valid = false;

    if (constant !== true)
      return {
        valid,
        reason: "Constant properties must have constant as 'true'",
      };

    if (!this._isRuleInDefinition(prop, "value"))
      return {
        valid,
        reason: "Constant properties must have a value or setter",
      };

    if (isEqual(value, undefined))
      return {
        valid,
        reason: "Constant properties cannot have 'undefined' as value",
      };

    const unAcceptedRules = DEFINITION_RULES.filter(
      (rule) => !CONSTANT_RULES.includes(rule)
    );

    if (this._isRuleInDefinition(prop, unAcceptedRules))
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

  private __isDependentProp = (prop: string) => {
    const {
      default: _default,
      dependent,
      dependsOn,
      shouldInit,
      readonly,
      resolver,
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

    if (this._isRuleInDefinition(prop, "validator"))
      return {
        valid,
        reason: "Dependent properties cannot be validated",
      };

    if (this._isRuleInDefinition(prop, "required"))
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

    if (this._isRuleInDefinition(prop, "virtual"))
      return { valid, reason: "Dependent properties cannot be virtual" };

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
    this.props.includes(prop as StringKey<O>);

  protected _isPropDefinitionObjectOk = (prop: string) => {
    const propDef = this._getDefinition(prop);

    const propertyTypeProvided = typeof propDef;

    return !propDef ||
      propertyTypeProvided !== "object" ||
      Array.isArray(propDef)
      ? {
          reasons: [
            `Invalid property definition. Expected an object '{}' but received '${propertyTypeProvided}'`,
          ],
          valid: false,
        }
      : { valid: true };
  };

  private __isPropDefinitionOk = (prop: string) => {
    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    let reasons: string[] = [];

    const invalidRulesProvided = this._getInvalidRules(prop as StringKey<I>);

    if (invalidRulesProvided.length)
      for (let rule of invalidRulesProvided)
        reasons.push(`'${rule}' is not a valid rule`);

    if (this._isRuleInDefinition(prop, "constant")) {
      const { valid, reason } = this.__isConstantProp(prop);

      if (!valid) reasons.push(reason!);
    } else if (this._isRuleInDefinition(prop, "value"))
      reasons.push("'value' rule can only be used with constant properties");

    if (this._isRuleInDefinition(prop, "dependent")) {
      const { valid, reason } = this.__isDependentProp(prop);

      if (!valid) reasons.push(reason!);
    } else if (this._isRuleInDefinition(prop, ["dependsOn", "resolver"]))
      reasons.push(
        "dependsOn & resolver rules can only belong to dependent properties"
      );

    if (this._isRuleInDefinition(prop, "readonly")) {
      const { valid, reason } = this.__isReadonly(prop);

      if (!valid) reasons.push(reason!);
    }

    if (this._isRuleInDefinition(prop, "required")) {
      const { required } = this._getDefinition(prop);

      const { valid, reason } =
        typeof required === "function"
          ? this.__isRequiredBy(prop)
          : this.__isRequired(prop);

      if (!valid) reasons.push(reason!);
    }

    if (this._isRuleInDefinition(prop, "virtual")) {
      const { valid, reason } = this.__isVirtual(prop);

      if (!valid) reasons.push(reason!);
    } else if (this._isRuleInDefinition(prop, "sanitizer"))
      reasons.push("'sanitizer' is only valid on virtuals");

    if (this._isRuleInDefinition(prop, "shouldInit")) {
      const { valid, reason } = this.__isShouldInitConfigOk(prop);

      if (!valid) reasons.push(reason!);
    }

    if (this._isRuleInDefinition(prop, "shouldUpdate")) {
      const { valid, reason } = this.__isShouldUpdateConfigOk(prop);

      if (!valid) reasons.push(reason!);
    }

    if (
      this._isRuleInDefinition(prop, "validator") &&
      !this._isValidatorOk(prop)
    )
      reasons.push("Invalid validator");

    if (
      this._isRuleInDefinition(prop, "onFailure") &&
      !this._isRuleInDefinition(prop, "validator")
    )
      reasons.push(
        "'onFailure' can only be used with properties that support and have validators"
      );

    // onDelete, onFailure, & onSuccess
    for (let rule of lifeCycleRules) {
      if (!this._isRuleInDefinition(prop, rule)) continue;

      const isValid = this._areHandlersOk(prop, rule);

      if (!isValid.valid) reasons = reasons.concat(isValid.reasons);
    }

    this._registerIfLax(prop);

    const hasDefaultRule = this._isRuleInDefinition(prop, "default");

    if (
      !hasDefaultRule &&
      !this._isConstant(prop) &&
      !this._isDependentProp(prop) &&
      !this._isLaxProp(prop) &&
      !this._isReadonly(prop) &&
      !this._isRequired(prop) &&
      !this._isVirtual(prop) &&
      !reasons.length
    ) {
      reasons.push(
        "A property should at least be readonly, required, or have a default value"
      );
    }

    const valid = reasons.length ? false : true;

    if (valid && !this._isVirtual(prop)) {
      this.props.push(prop as StringKey<O>);

      if (hasDefaultRule)
        this.defaults[prop as StringKey<O>] = this._getValueBy(prop, "default");
    }

    return { reasons, valid };
  };

  protected _isPropDefinitionOk = (prop: string): boolean =>
    this.__isPropDefinitionOk(prop).valid;

  private __isReadonly = (prop: string) => {
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

    if (
      this._isRuleInDefinition(prop, "required") &&
      typeof required != "function"
    )
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

    if (this._isRuleInDefinition(prop, "dependent"))
      return {
        valid,
        reason: "Required properties cannot be dependent",
      };

    if (this._isRuleInDefinition(prop, "shouldInit"))
      return {
        valid,
        reason: "Required properties cannot have a initialization blocked",
      };

    if (!this._isValidatorOk(prop))
      return { valid, reason: "Required properties must have a validator" };

    return { valid: true };
  };

  private __isRequired = (prop: string) => {
    const { required } = this._getDefinition(prop);

    const valid = false;

    if (required !== true)
      return {
        valid,
        reason: "Required properties must have required as 'true'",
      };

    if (this._isRuleInDefinition(prop, "default"))
      return {
        valid,
        reason:
          "Strictly required properties cannot have a default value or setter",
      };

    if (this._isRuleInDefinition(prop, "readonly"))
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

    const hasVirtualRule = this._isRuleInDefinition(prop, "virtual");

    if (isEqual(_default, undefined) && !hasVirtualRule)
      return {
        valid,
        reason:
          "Callable required properties must have a default value or setter",
      };

    if (!hasVirtualRule) {
      const isRequiredCommon = this.__isRequiredCommon(prop);

      if (!isRequiredCommon.valid) return isRequiredCommon;
    }

    if (!this._isRequiredBy(prop))
      this.propsRequiredBy.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected _isRequiredBy = (prop: string) =>
    this.propsRequiredBy.includes(prop as StringKey<I>);

  protected __isShouldInitConfigOk = (prop: string) => {
    const { shouldInit } = this._getDefinition(prop);

    const valid = false;

    if (shouldInit !== false && !this._isFunction(shouldInit))
      return {
        valid,
        reason:
          "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean",
      };

    if (!this._isRuleInDefinition(prop, ["default", "virtual"]))
      return {
        valid,
        reason:
          "A property with initialization blocked must have a default value",
      };

    return { valid: true };
  };

  protected __isShouldUpdateConfigOk = (prop: string) => {
    const { readonly, shouldInit, shouldUpdate } = this._getDefinition(prop);
    const valid = false;

    if (shouldUpdate !== false && !this._isFunction(shouldUpdate))
      return {
        valid,
        reason:
          "'shouldUpdate' only accepts false or a function that returns a boolean",
      };

    if (shouldInit === false && shouldUpdate === false)
      return {
        valid,
        reason: "Both 'shouldInit' & 'shouldUpdate' cannot be 'false'",
      };

    if (shouldUpdate === false && !this._isRuleInDefinition(prop, "virtual"))
      return {
        valid,
        reason: "Only 'Virtuals' are allowed to have 'shouldUpdate' as 'false'",
      };

    if (readonly === true && isEqual(shouldInit, undefined))
      return {
        valid,
        reason:
          "Cannot block the update of 'readonly' properties that do not have initialization('shouldInit') blocked. Either add 'shouldInit' or use readonly: 'lax'",
      };

    return { valid: true };
  };

  protected __isVirtualRequiredBy = (prop: string) => {
    if (this._isRuleInDefinition(prop, "shouldInit"))
      return {
        valid: false,
        reason: "Required virtuals cannot have initialization blocked",
      };

    const isRequiredBy = this.__isRequiredBy(prop);

    if (!isRequiredBy.valid) return isRequiredBy;

    return { valid: true };
  };

  protected __isVirtual = (prop: string) => {
    const valid = false;

    const { sanitizer, virtual } = this._getDefinition(prop);

    if (virtual !== true)
      return { valid, reason: "Virtuals must have virtual as 'true'" };

    if (!this._isValidatorOk(prop))
      return { valid, reason: "Invalid validator" };

    if (this._isRuleInDefinition(prop, "alias")) {
      const isValid = this.__isVirtualAliasOk(prop);

      if (!isValid.valid) return isValid;
    }

    if (
      this._isRuleInDefinition(prop, "sanitizer") &&
      !this._isFunction(sanitizer)
    )
      return { valid, reason: "'sanitizer' must be a function" };

    if (this._isRuleInDefinition(prop, "required")) {
      const isValid = this.__isVirtualRequiredBy(prop);

      if (!isValid.valid) return isValid;
    }

    const invalidVirtualRules = DEFINITION_RULES.filter(
      (rule) => !VIRTUAL_RULES.includes(rule)
    );

    if (this._isRuleInDefinition(prop, invalidVirtualRules))
      return {
        valid,
        reason: `Virtual properties can only have (${VIRTUAL_RULES.join(
          ", "
        )}) as rules`,
      };

    this.virtuals.push(prop as StringKey<I>);

    return { valid: true };
  };

  protected __isVirtualAliasOk = (prop: string) => {
    const valid = false;

    const { alias } = this._getDefinition(prop);

    if (typeof alias !== "string" || !alias.length)
      return {
        valid,
        reason: "An alias must be a string with atleast 1 character",
      };

    if (alias == prop)
      return {
        valid,
        reason: "An alias cannot be the same as the virtual property",
      };

    const isTakenBy = this._getVirtualByAlias(alias);
    if (isTakenBy)
      return {
        valid,
        reason: `Sorry, alias provided '${alias}' already belongs to property '${isTakenBy}'`,
      };

    this.aliasToVirtualMap[alias] = prop as StringKey<I>;
    this.virtualToAliasMap[prop] = alias as StringKey<I>;

    return { valid: true };
  };

  protected __isVirtualAliasOk2 = (alias: string | StringKey<I>) => {
    const prop = this._getVirtualByAlias(alias)!;

    const invalidResponse = {
      valid: false,
      reason: `'${alias}' cannot be used as the alias of '${prop}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
    } as any;

    const isDependentOnVirtual = (
      this._getDependencies(prop) as string[]
    )?.includes(alias as StringKey<I>);

    return (this._isProp(alias) && !isDependentOnVirtual) ||
      this._isVirtual(alias)
      ? invalidResponse
      : { valid: true };
  };

  protected _isVirtualAlias = (prop: string) => !!this.aliasToVirtualMap[prop];

  protected _isVirtual = (prop: string) =>
    this.virtuals.includes(prop as StringKey<I>);

  protected _isVirtualInit = (prop: string, value: any = undefined) => {
    const isAlias = this._isVirtualAlias(prop);

    if (!this._isVirtual(prop) && !isAlias) return false;

    const definitionName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    const { shouldInit } = this._getDefinition(definitionName);

    const extraCtx = isAlias ? { [definitionName]: value } : {};

    return (
      isEqual(shouldInit, undefined) ||
      this._getValueBy(definitionName, "shouldInit", extraCtx)
    );
  };

  private _isTimestampsOptionOk() {
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

  private _isOnSuccessOptionOk() {
    const { onSuccess } = this._options,
      reasons: string[] = [];

    const handlers = toArray<ns.SuccessHandler<any, any>>(onSuccess!);

    handlers.forEach((handler, i) => {
      if (this._isFunction(handler))
        return this.globalSuccessHandlers.push(handler);

      reasons.push(`The 'onSuccess' handler @[${i}] is not a function`);
    });

    if (reasons.length) return { valid: false, reasons };

    return { valid: true };
  }

  private _isValidatorOk = (prop: string) =>
    this._isFunction(this._getDefinition(prop)?.validator);

  private _makeTimestamps(): ns.PrivateOptions {
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

    if (custom_createdAt && typeof custom_createdAt == "string")
      createdAt = custom_createdAt;

    if (custom_createdAt === false) createdAt = "";

    if (custom_updatedAt && typeof custom_updatedAt == "string")
      updatedAt = custom_updatedAt;

    if (custom_updatedAt === false) updatedAt = "";

    return { ...options, timestamps: { createdAt, updatedAt } };
  }

  private _registerIfLax = (prop: string) => {
    const {
      default: _default,
      readonly,
      shouldInit,
    } = this._getDefinition(prop);

    // Lax properties must have a default value nor setter
    if (isEqual(_default, undefined)) return;

    // Lax properties cannot be dependent
    if (this._isRuleInDefinition(prop, "dependent")) return;

    // Lax properties cannot be required
    if (this._isRuleInDefinition(prop, "required")) return;

    // Lax properties cannot be virtual
    if (this._isRuleInDefinition(prop, "virtual")) return;

    // only readonly(lax) are lax props &
    // Lax properties cannot have initialization blocked
    if (
      (this._isRuleInDefinition(prop, "readonly") && readonly !== "lax") ||
      (this._isRuleInDefinition(prop, "shouldInit") &&
        typeof shouldInit != "function")
    )
      return;

    this.laxProps.push(prop as StringKey<I>);
  };
}
