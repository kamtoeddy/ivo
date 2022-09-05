import { ApiError } from "../utils/ApiError";
import { belongsTo, sort, sortKeys, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  LifeCycle,
  Private_ISchemaOptions,
  PropDefinitionRule,
  Schema as ns,
  StringKeys,
} from "./interfaces";
import { makeResponse, SchemaOptionsHelper } from "./SchemaUtils";

export const defaultOptions = { timestamps: false };

const lifeCycleRules: LifeCycle.Rule[] = ["onChange", "onCreate", "onUpdate"];

export abstract class SchemaCore<T extends ObjectType> {
  protected error = new ApiError({ message: "Validation Error" });

  protected _helper: SchemaOptionsHelper;
  protected _options: ns.Options;
  protected _propDefinitions = {} as ns.PropertyDefinitions<T>;

  protected context: T = {} as T;
  protected defaults: Partial<T> = {};
  protected props: StringKeys<T>[] = [];
  protected updated: Partial<T> = {};
  protected values: Partial<T> = {};

  constructor(
    propDefinitions: ns.PropertyDefinitions<T>,
    options: ns.Options = defaultOptions
  ) {
    this._propDefinitions = propDefinitions;
    this._options = options;

    this._helper = new SchemaOptionsHelper(this._makeOptions(options));
  }

  // context methods
  protected _getContext = () => this.context;

  protected _initContext = () => (this.context = { ...this.values } as T);

  protected _updateContext = (updates: Partial<T>) =>
    (this.context = { ...this.context, ...updates });

  // error methods
  protected _throwErrors(_message?: string): void {
    const err = new ApiError(this.error.summary);

    this.error.clear();

    if (_message) err.setMessage(_message);

    throw err;
  }

  protected _checkPropDefinitions = () => {
    const error = new ApiError({
      message: "Invalid Schema",
      statusCode: 500,
    });

    if (
      !this._propDefinitions ||
      typeof this._propDefinitions !== "object" ||
      Array.isArray(this._propDefinitions)
    )
      throw error;

    let props: string[] = Object.keys(this._propDefinitions);

    if (!props.length)
      throw error.add("schema properties", "Insufficient Schema properties");

    for (let prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop);
      if (!isDefOk.valid) error.add(prop, isDefOk.reasons);
    }

    if (error.isPayloadLoaded) throw error;
  };

  protected _getCloneObject = async (reset: StringKeys<T>[] = []) => {
    const data = {} as T;

    const sideEffects = Object.keys(this.values).filter(
      this._isSideInit
    ) as StringKeys<T>[];

    const props = [...this.props, ...sideEffects];

    const validations = props.map((prop) => {
      const isSideEffect = sideEffects.includes(prop);

      if (isSideEffect && !this._isSideInit(prop)) return;

      if (!isSideEffect && reset.includes(prop)) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      const isLaxInit =
        this._isLaxProp(prop) &&
        !isEqual(this.values[prop], this.defaults[prop]);

      if (!isSideEffect && !this._isRequired(prop) && !isLaxInit) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      return this._validateAndSet(data, prop, this.values[prop]);
    });

    await Promise.all(validations);

    if (this._isErroneous()) this._throwErrors();

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(linkedProps, data, "onCreate");

    await this._resolveLinked(sideEffects, data, "onCreate");

    return this._useConfigProps(data) as T;
  };

  protected _getCreatePropsWithListeners = () => {
    let listeners = [];

    for (let prop of this.props)
      if (this._getAllListeners(prop, "onCreate")?.length) listeners.push(prop);

    return listeners;
  };

  protected _getCreateObject = async () => {
    const data = {} as T;

    const sideEffects = Object.keys(this.values).filter(
      this._isSideInit
    ) as StringKeys<T>[];

    const props = [...this.props, ...sideEffects];

    const validations = props.map((prop) => {
      const isSideEffect = sideEffects.includes(prop);
      if (isSideEffect && !this._isSideInit(prop)) return;

      const isLaxInit =
        this._isLaxProp(prop) && this.values.hasOwnProperty(prop);

      if (!isSideEffect && !this._isRequired(prop) && !isLaxInit)
        return (data[prop] = this._getDefaultValue(prop));

      return this._validateAndSet(data, prop, this.values[prop]);
    });

    await Promise.all(validations);

    if (this._isErroneous()) this._throwErrors();

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(linkedProps, data, "onCreate");

    await this._resolveLinked(sideEffects, data, "onCreate");

    return this._useConfigProps(data) as T;
  };

  protected _getDefinition = (prop: string) => this._propDefinitions[prop]!;

  protected _getDefinitionValue = (prop: string, rule: PropDefinitionRule) =>
    this._getDefinition(prop)?.[rule];

  protected _getDefaultValue = (prop: string, alternate = true) => {
    const value = this._getDefinition(prop)?.default;

    if (alternate && isEqual(value, undefined)) return this.values[prop];

    return typeof value === "function" ? value(this._getContext()) : value;
  };

  protected _getDetailedListeners = (
    prop: string,
    lifeCycle: LifeCycle.Rule,
    valid = true
  ) => {
    const listeners = toArray(this._getDefinitionValue(prop, lifeCycle));

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

  protected _getAllListeners = (prop: string, lifeCycle: LifeCycle.Rule) => {
    const onChange = this._getListeners(prop, "onChange");

    if (this._isSideEffect(prop)) return onChange;

    const others = this._getListeners(prop, lifeCycle);

    return [...others, ...onChange];
  };

  protected _getListeners = (prop: string, lifeCycle: LifeCycle.Rule) => {
    return this._getDetailedListeners(prop, lifeCycle, true).map(
      (dt) => dt.listener
    ) as LifeCycle.Listener<T>[];
  };

  protected _getProps = (): StringKeys<T>[] => {
    const props = Object.keys(this._propDefinitions).filter(
      (prop) => this._isPropDefinitionOk(prop) && !this._isSideEffect(prop)
    );

    return sort(props);
  };

  protected _getValidator = (prop: string) =>
    this._getDefinition(prop)?.validator;

  protected _hasAny = (
    prop: string,
    rules: PropDefinitionRule | PropDefinitionRule[]
  ): boolean => {
    if (!this._isPropDefinitionObjectOk(prop).valid) return false;

    for (let _prop of toArray(rules))
      if (this._getDefinition(prop)?.hasOwnProperty(_prop)) return true;

    return false;
  };

  protected __isDependentProp = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    const {
      default: _default,
      dependent,
      sideEffect,
      shouldInit,
      required,
    } = this._getDefinition(prop);

    if (isEqual(_default, undefined))
      reasons.push("Dependent properties must have a default value");

    if (!isEqual(required, undefined))
      reasons.push("Dependent properties cannot be required");

    if (!isEqual(shouldInit, undefined))
      reasons.push("Dependent properties cannot have shouldInit rule");

    if (sideEffect) reasons.push("Dependent properties cannot be sideEffect");

    if (!dependent)
      reasons.push("Dependent properties must have dependent as 'true'");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isDependentProp = (prop: string): boolean =>
    this.__isDependentProp(prop).valid;

  protected _isErroneous = () => this.error.isPayloadLoaded;

  protected _isFunction = (obj: any): boolean => typeof obj === "function";

  protected __isLaxProp = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    const propDef = this._getDefinition(prop);

    const hasDefaultValue = !isEqual(propDef.default, undefined),
      isDependent = this._isDependentProp(prop);

    if (!hasDefaultValue) reasons.push("No default value");

    const { readonly, required, sideEffect } = propDef;

    if (isDependent || required || sideEffect)
      reasons.push("dependent, required and sideEffect should not be 'true'");

    const shouldInit = belongsTo(propDef?.shouldInit, [true, undefined]);

    if (readonly !== "lax" && !shouldInit)
      reasons.push("lax properties cannot have initialization blocked");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isLaxProp = (prop: string) => this.__isLaxProp(prop).valid;

  protected _isProp = (prop: string) =>
    this.props.includes(prop as StringKeys<T>);

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
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) reasons = reasons.concat(isPopDefOk.reasons!);

    const dependentDef = this.__isDependentProp(prop);

    if (this._hasAny(prop, "dependent") && !dependentDef.valid)
      reasons = reasons.concat(dependentDef.reasons!);

    const readonlyDef = this.__isReadonly(prop);

    if (this._hasAny(prop, "readonly") && !readonlyDef.valid)
      reasons = reasons.concat(readonlyDef.reasons!);

    const sideEffectDef = this.__isSideEffect(prop);

    if (this._hasAny(prop, "sideEffect") && !sideEffectDef.valid)
      reasons = reasons.concat(sideEffectDef.reasons!);

    if (this._hasAny(prop, "validator") && !this._isValidatorOk(prop))
      reasons.push("Invalid validator");

    for (let rule of lifeCycleRules) {
      if (!this._hasAny(prop, rule)) continue;

      const invalidHandlers = this._getDetailedListeners(prop, rule, false);

      if (!invalidHandlers?.length) continue;

      reasons = reasons.concat(
        invalidHandlers.map(
          (dt) => `'${dt.listener}' @${rule}[${dt.index}] is not a function`
        )
      );
    }

    const { readonly, shouldInit } = this._getDefinition(prop);

    if (
      (readonly === "lax" || shouldInit === false) &&
      !this._hasAny(prop, "default")
    )
      reasons.push(
        "A property that should not be initialized must have a default value other than 'undefined'"
      );

    if (
      !this._hasAny(prop, ["default", "readonly", "required"]) &&
      !this._isDependentProp(prop) &&
      !this._isLaxProp(prop) &&
      !this._isSideEffect(prop)
    )
      reasons.push(
        "A property should at least be readonly, required, or have a default value"
      );

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isPropDefinitionOk = (prop: string): boolean =>
    this.__isPropDefinitionOk(prop).valid;

  protected __isReadonly = (prop: string) => {
    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    let reasons: string[] = [];

    if (this._hasAny(prop, ["required"]))
      reasons.push("readonly properties are required by default");

    const {
      default: _default,
      dependent,
      readonly,
      shouldInit,
    } = this._getDefinition(prop);

    if (
      readonly === "lax" &&
      (dependent === true || shouldInit === false) &&
      isEqual(_default, undefined)
    )
      reasons.push(
        "readonly properties must have a default value or a default setter"
      );

    if (readonly === "lax" && !isEqual(shouldInit, undefined))
      reasons.push("lax properties cannot have initialization blocked");

    if (
      !belongsTo(readonly, [true, "lax"]) ||
      belongsTo(readonly, [false, undefined])
    )
      reasons.push("readonly properties have readonly true | 'lax'");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isReadonly = (prop: string) => this.__isReadonly(prop).valid;

  protected _isRequired = (prop: string): boolean => {
    if (this._isDependentProp(prop)) return false;

    const propDef = this._getDefinition(prop);

    const { readonly, required, shouldInit } = propDef;

    if (!isEqual(required, undefined)) return true;

    return readonly === true && belongsTo(shouldInit, [true, undefined]);
  };

  protected __isSideEffect = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    if (this._hasAny(prop, "default"))
      reasons.push(
        "SideEffects cannot have default values as they do not exist on instances of your model"
      );

    if (this._hasAny(prop, "dependent"))
      reasons.push("SideEffects cannot be dependent");

    if (this._hasAny(prop, ["readonly", "required"]))
      reasons.push("SideEffects cannot be readonly nor required");

    if (!this._isValidatorOk(prop)) reasons.push("Invalid validator");

    if (!this._getListeners(prop, "onChange").length)
      reasons.push("SideEffects must have at least one onChange listener");

    if (this._getListeners(prop, "onCreate").length)
      reasons.push("SideEffects do not support onCreate listeners");

    if (this._getListeners(prop, "onUpdate").length)
      reasons.push(
        "SideEffects do not support onUpdate listeners any more. Use onChange instead"
      );

    const { sideEffect } = this._getDefinition(prop);

    if (!sideEffect === true)
      reasons.push("SideEffects must have sideEffect as'true'");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isSideEffect = (prop: string): boolean =>
    this.__isSideEffect(prop).valid;

  protected _isSideInit = (prop: string): boolean => {
    const propDef = this._getDefinition(prop);

    if (!propDef) return false;

    const { shouldInit } = propDef;

    return this._isSideEffect(prop) && belongsTo(shouldInit, [true, undefined]);
  };

  protected _isUpdatable = (prop: string) => {
    if (this._isSideEffect(prop)) return true;

    if (!this._isProp(prop) || this._isDependentProp(prop)) return false;

    const { readonly } = this._getDefinition(prop);

    const _default = this._getDefaultValue(prop, false);

    return (
      !readonly || (readonly && isEqual(_default, this._getContext()?.[prop]))
    );
  };

  protected _isUpdatableInCTX = (
    prop: string,
    value: any,
    context: ObjectType = this._getContext()
  ) => {
    return !this._isProp(prop) ? false : !isEqual(value, context?.[prop]);
  };

  protected _isValidatorOk = (prop: string) =>
    this._isFunction(this._getDefinition(prop)?.validator);

  private _makeOptions(options: ns.Options): Private_ISchemaOptions {
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

    const _error = new ApiError({
      message: "Invalid schema options",
      statusCode: 500,
    });

    const custom_createdAt = timestamps?.createdAt;
    const custom_updatedAt = timestamps?.updatedAt;

    const _props = this._getProps();

    [custom_createdAt, custom_updatedAt].forEach((value) => {
      if (value && _props?.includes(value as StringKeys<T>))
        _error.add(value, `'${value}' already belong to your schema`);
    });

    if (custom_createdAt === custom_updatedAt)
      _error.add("timestamp", `createdAt & updatedAt cannot be same`);

    if (_error.isPayloadLoaded) throw _error;

    if (custom_createdAt) createdAt = custom_createdAt;
    if (custom_updatedAt) updatedAt = custom_updatedAt;

    return { ...options, timestamps: { createdAt, updatedAt } };
  }

  protected _resolveLinked = async (
    props: StringKeys<T>[],
    context: Partial<T>,
    lifeCycle: LifeCycle.Rule
  ) => {
    const listenersUpdates = props.map((prop) => {
      return this._resolveLinkedProps(context, prop, lifeCycle);
    });

    await Promise.all(listenersUpdates);
  };

  protected _resolveLinkedProps = async (
    operationData: Partial<T> = {},
    prop: StringKeys<T>,
    lifeCycle: LifeCycle.Rule
  ) => {
    const listeners = this._getAllListeners(prop, lifeCycle);

    if (
      !listeners.length ||
      (lifeCycle === "onUpdate" &&
        !this._isSideEffect(prop) &&
        !this._isUpdatableInCTX(prop, operationData[prop], this.values))
    )
      return;

    for (const listener of listeners) {
      const context = { ...this._getContext(), ...operationData };

      const extra = await listener(context);

      if (typeof extra !== "object") continue;

      const _props = Object.keys(extra) as StringKeys<T>[];

      for (let _prop of _props) {
        const _value = extra[_prop];
        const isSideEffect = this._isSideEffect(_prop);

        if (!isSideEffect && !this._isUpdatableInCTX(_prop, _value, context))
          continue;

        await this._validateAndSet(operationData, _prop, _value);

        await this._resolveLinkedProps(operationData, _prop, lifeCycle);
      }
    }
  };

  protected _useConfigProps = (obj: T | Partial<T>, asUpdate = false) => {
    if (!this._helper.withTimestamps) return sortKeys(obj);

    const createdAt = this._helper.getCreateKey(),
      updatedAt = this._helper.getUpdateKey();

    const results = asUpdate
      ? { ...obj, [updatedAt]: new Date() }
      : { ...obj, [createdAt]: new Date(), [updatedAt]: new Date() };

    return sortKeys(results);
  };

  protected _validate = async (prop = "", value: any) => {
    const isSideEffect = this._isSideEffect(prop);

    if (!this._isProp(prop) && !isSideEffect)
      return { valid: false, reasons: ["Invalid property"] };

    const validator = this._getValidator(prop);

    if (validator)
      return makeResponse<any>(await validator(value, this._getContext()));

    return makeResponse<any>({ valid: true, validated: value });
  };

  protected _validateAndSet = async (
    operationData: Partial<T> = {},
    prop: StringKeys<T>,
    value: any
  ) => {
    const { reasons, valid, validated } = await this._validate(prop, value);

    if (!valid) return this.error.add(prop, reasons);

    if (!this._isSideEffect(prop)) operationData[prop] = validated;

    this._updateContext({ [prop]: validated } as T);
  };
}
