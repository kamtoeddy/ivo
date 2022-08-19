import { ApiError } from "../utils/ApiError";
import { asArray } from "../utils/asArray";
import { belongsTo } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  LifeCycleRule,
  Private_ISchemaOptions,
  PropDefinitionRule,
  PropDefinitionRules,
  SchemaOptions,
  ValidatorResponse,
} from "./interfaces";
import { SchemaOptionsHelper } from "./SchemaOptionsHelper";

export const defaultOptions: SchemaOptions = { timestamps: false };

const lifeCycleRules: LifeCycleRule[] = ["onChange", "onCreate", "onUpdate"];

export abstract class SchemaCore<T extends ObjectType> {
  protected error = new ApiError({ message: "Validation Error" });

  protected _helper: SchemaOptionsHelper;
  protected _options: SchemaOptions;
  protected _propDefinitions: PropDefinitionRules = {};

  protected context: T = {} as T;
  protected defaults: Partial<T> = {};
  protected props: string[] = [];
  protected updated: Partial<T> = {};
  protected values: Partial<T> = {};

  constructor(
    propDefinitions: PropDefinitionRules,
    options: SchemaOptions = defaultOptions
  ) {
    this._propDefinitions = propDefinitions;
    this._options = options;

    this._helper = new SchemaOptionsHelper(this._makeOptions(options));
  }

  public get options() {
    return this._options;
  }

  public get propDefinitions() {
    return this._propDefinitions;
  }

  // context methods
  protected _getContext = (): T => {
    const data = this.props.reduce((prev, prop) => {
      prev[prop as keyof T] = this.values[prop]!;

      return prev;
    }, {} as T);

    this._updateContext({ ...data, ...this.updated });

    return this.context;
  };

  protected _resetContext = () => (this.context = {} as T);

  protected _updateContext = (updates: Partial<T>) =>
    (this.context = { ...this.context, ...updates });

  // error methods
  protected _throwErrors(_message?: string): void {
    let err = new ApiError(this.error.getInfo());

    this.error.clear();

    if (_message) err.setMessage(_message);

    throw err;
  }

  protected _canInit = (prop: string): boolean => {
    if (this._isDependentProp(prop)) return false;

    const propDef = this._propDefinitions[prop];

    if (!propDef) return false;
    const { readonly, required, shouldInit } = propDef;

    if (required) return true;

    return readonly === true && belongsTo(shouldInit, [true, undefined]);
  };

  protected _checkPropDefinitions = () => {
    const error = new ApiError({
      message: "Invalid Schema",
      statusCode: 500,
    });

    let props: string[] = Object.keys(this._propDefinitions);

    for (let prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop);
      if (!isDefOk.valid) error.add(prop, isDefOk.reasons);
    }

    if (error.isPayloadLoaded) throw error;
  };

  protected _getCloneObject = async (reset: string[] = []) => {
    const linkedProps = this._getCreatePropsWithListeners().filter(
      (prop) => !reset.includes(prop)
    );

    const createProps = this.props.filter(
      (prop) => !linkedProps.includes(prop)
    );

    const obj = createProps.reduce((values: T, next) => {
      values[next as keyof T] = (
        reset.includes(next)
          ? this.defaults[next] ?? this.values[next]
          : this.values[next] ?? this.defaults[next]
      )!;

      return values;
    }, {} as T);

    await this._resolveLinked(linkedProps, obj, this.values, "onCreate");

    await this._useSideInitProps(obj);

    return this._useConfigProps(obj) as T;
  };

  protected _getCreatePropsWithListeners = () => {
    let actions = [];

    for (let prop of this.props) {
      const _actions = this._getAllListeners(prop, "onCreate");

      if (_actions?.length) actions.push(prop);
    }

    return actions;
  };

  protected _getCreateObject = async () => {
    const obj = {} as T;

    const linkedProps = this._getCreatePropsWithListeners();

    const createProps = this.props.filter(
      (prop) => !linkedProps.includes(prop)
    );

    const validations = createProps.map((prop) => {
      const isLaxInit =
        this._isLaxProp(prop) && this.values.hasOwnProperty(prop);

      const canInit = this._canInit(prop);

      if (canInit || isLaxInit) return this.validate(prop, this.values[prop]);

      return {
        reasons: [],
        valid: true,
        validated: this.defaults[prop],
      } as ValidatorResponse;
    });

    const results = await Promise.all(validations);

    createProps.forEach((prop, index) => {
      const { reasons, valid, validated } = results[index];

      if (valid) return (obj[prop as keyof T] = validated);

      this.error.add(prop, reasons);
    });

    await this._resolveLinked(linkedProps, obj, this.values, "onCreate");

    await this._useSideInitProps(obj);

    return this._useConfigProps(obj) as T;
  };

  protected _getDefinitionValue = (prop: string, rule: PropDefinitionRule) => {
    return this._propDefinitions[prop]?.[rule];
  };

  protected _getDetailedListeners = (
    prop: string,
    lifeCycle: LifeCycleRule,
    valid = true
  ) => {
    const propDef = this._propDefinitions[prop];

    const listeners = asArray(propDef?.[lifeCycle]);

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

  protected _getAllListeners = (prop: string, lifeCycle: LifeCycleRule) => {
    const onChange = this._getListeners(prop, "onChange");

    if (this._isSideEffect(prop)) return onChange;

    const others = this._getListeners(prop, lifeCycle);

    return [...others, ...onChange];
  };

  protected _getListeners = (prop: string, lifeCycle: LifeCycleRule) => {
    return this._getDetailedListeners(prop, lifeCycle, true).map(
      (dt) => dt.listener
    );
  };

  protected _getProps = () => {
    let props: string[] = Object.keys(this._propDefinitions);

    props = props.filter(
      (prop) => this._isPropDefinitionOk(prop) && !this._isSideEffect(prop)
    );

    return this._sort(props);
  };

  protected _getSideEffects = () => {
    let props: string[] = Object.keys(this._propDefinitions);

    props = props.filter((prop) => {
      const propDef = this._propDefinitions[prop];

      if (typeof propDef !== "object") return false;

      return this._isSideEffect(prop);
    });

    return props;
  };

  protected _getValidator = (prop: string) =>
    this._propDefinitions[prop]?.validator;

  protected _hasChanged = (prop: string) => {
    const propDef = this._propDefinitions[prop];

    if (!propDef) return false;

    return !isEqual(propDef.default, this._getContext()?.[prop]);
  };

  protected _hasProp = (
    prop: string,
    rules: PropDefinitionRule | PropDefinitionRule[]
  ): boolean => {
    if (!this._isPropDefinitionObjectOk(prop).valid) return false;

    const propDef = this._propDefinitions[prop];

    if (!Array.isArray(rules)) rules = [rules];

    for (let _prop of rules)
      if (Object(propDef).hasOwnProperty(_prop)) return true;

    return false;
  };

  protected __isDependentProp = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    const propDef = this._propDefinitions[prop];

    const { dependent, sideEffect } = propDef;

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

    const propDef = this._propDefinitions[prop];

    const hasDefaultValue = !isEqual(propDef.default, undefined),
      isDependent = this._isDependentProp(prop);

    if (!hasDefaultValue) reasons.push("No default value");

    const { readonly, required, sideEffect } = propDef;

    if (isDependent || required || sideEffect) {
      reasons.push("dependent, required and sideEffect should not be 'true'");
    }

    const shouldInit = belongsTo(propDef?.shouldInit, [true, undefined]);

    if (readonly !== "lax" && !shouldInit)
      reasons.push("shouldInit must be true or undefined");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isLaxProp = (prop: string): boolean =>
    this.__isLaxProp(prop).valid;

  protected _isProp = (prop: string): boolean => this.props.includes(prop);

  protected _isPropDefinitionObjectOk = (prop: string) => {
    const propDef = this._propDefinitions[prop];

    return propDef && typeof propDef === "object"
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

    if (this._hasProp(prop, "dependent") && !dependentDef.valid)
      reasons = reasons.concat(dependentDef.reasons!);

    const sideEffectDef = this.__isSideEffect(prop);

    if (this._hasProp(prop, "sideEffect") && !sideEffectDef.valid)
      reasons = reasons.concat(sideEffectDef.reasons!);

    if (this._hasProp(prop, "validator") && !this._isValidatorOk(prop))
      reasons.push("Invalid validator");

    for (let rule of lifeCycleRules) {
      if (!this._hasProp(prop, rule)) continue;

      const invalidHandlers = this._getDetailedListeners(prop, rule, false);

      if (!invalidHandlers?.length) continue;

      reasons = reasons.concat(
        invalidHandlers.map(
          (dt) => `'${dt.listener}' @${rule}[${dt.index}] is not a function`
        )
      );
    }

    if (
      this._getDefinitionValue(prop, "shouldInit") === false &&
      !this._hasProp(prop, "default")
    )
      reasons.push(
        "A property that should not be initialized must have a default value other than 'undefined'"
      );

    if (
      !this._hasProp(prop, ["default", "readonly", "required"]) &&
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

  protected __isSideEffect = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) return isPopDefOk;

    if (this._hasProp(prop, "default"))
      reasons.push(
        "SideEffects cannot have default values as they do not exist on instances of your model"
      );

    if (this._hasProp(prop, "dependent"))
      reasons.push("SideEffects cannot be dependent");

    if (this._hasProp(prop, ["readonly", "required"]))
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

    const { sideEffect } = this._propDefinitions[prop];

    if (!sideEffect === true)
      reasons.push("SideEffects must have sideEffect as'true'");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isSideEffect = (prop: string): boolean =>
    this.__isSideEffect(prop).valid;

  protected _isSideInit = (prop: string): boolean => {
    const propDef = this._propDefinitions[prop];

    if (!propDef) return false;

    const { shouldInit } = propDef;

    return this._isSideEffect(prop) && belongsTo(shouldInit, [true, undefined]);
  };

  protected _isUpdatable = (prop: string) => {
    if (!this._isProp(prop)) return false;

    if (this._isDependentProp(prop)) return false;

    const readonly = this._propDefinitions?.[prop]?.readonly;

    return !readonly || (readonly && !this._hasChanged(prop));
  };

  protected _isUpdatableInCTX = (
    prop: string,
    value: any,
    context: ObjectType = this._getContext()
  ) => {
    if (!this._isProp(prop)) return false;

    return !isEqual(value, context?.[prop]);
  };

  protected _isValidatorOk = (prop: string) => {
    const propDef = this._propDefinitions[prop];

    return this._isFunction(propDef?.validator);
  };

  private _makeOptions(options: SchemaOptions): Private_ISchemaOptions {
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
      if (value && _props?.includes(value)) {
        _error.add(value, `'${value}' already belong to your schema`);
      }
    });

    if (custom_createdAt === custom_updatedAt) {
      _error.add("timestamp", `createdAt & updatedAt cannot be same`);
    }

    if (_error.isPayloadLoaded) throw _error;

    if (custom_createdAt) createdAt = custom_createdAt;
    if (custom_updatedAt) updatedAt = custom_updatedAt;

    return { ...options, timestamps: { createdAt, updatedAt } };
  }

  protected _resolveLinked = async (
    props: string[],
    context: Partial<T>,
    values: Partial<T>,
    lifeCycle: LifeCycleRule
  ) => {
    const listenersUpdates = props.map((prop) => {
      return this._resolveLinkedValue(context, prop, values[prop], lifeCycle);
    });

    await Promise.all(listenersUpdates);
  };

  protected _resolveLinkedValue = async (
    contextObject: ObjectType = {},
    prop: string,
    value: any,
    lifeCycle: LifeCycleRule
  ) => {
    const listeners = this._getAllListeners(prop, lifeCycle),
      isSideEffect = this._isSideEffect(prop);

    if (!listeners.length) return;

    const { reasons, valid, validated } = await this.validate(prop, value);

    if (!valid) return this.error.add(prop, reasons);

    const hasChanged = !isEqual(this.values[prop], validated);

    if (lifeCycle === "onUpdate" && !isSideEffect && !hasChanged) return;

    if (isSideEffect) this._updateContext({ [prop]: validated } as Partial<T>);
    else contextObject[prop] = validated;

    const context = { ...this._getContext(), ...contextObject };

    for (const cb of listeners) {
      const extra = await cb(context);

      if (typeof extra !== "object") continue;

      const _props = Object.keys(extra);

      for (let _prop of _props) {
        const _value = extra[_prop];
        const isSideEffect = this._isSideEffect(_prop);

        if (!isSideEffect && !this._isUpdatableInCTX(_prop, _value, context))
          continue;

        if (!isSideEffect) contextObject[_prop] = _value;

        await this._resolveLinkedValue(contextObject, _prop, _value, lifeCycle);
      }
    }
  };

  protected _sort = (data: any[]): any[] =>
    data.sort((a, b) => (a < b ? -1 : 1));

  protected _sortKeys = (obj: Partial<T>): Partial<T> => {
    const keys = this._sort(Object.keys(obj));

    return keys.reduce((prev, next) => {
      prev[next] = obj[next];

      return prev;
    }, {});
  };

  protected _useConfigProps = (obj: T | Partial<T>, asUpdate = false) => {
    if (!this._helper.withTimestamps) return obj;

    const createdAt = this._helper.getCreateKey(),
      updatedAt = this._helper.getUpdateKey();

    const results = asUpdate
      ? { ...obj, [updatedAt]: new Date() }
      : { ...obj, [createdAt]: new Date(), [updatedAt]: new Date() };

    return this._sortKeys(results);
  };

  protected _useSideInitProps = async (data: T) => {
    const sideEffectProps = Object.keys(this.values).filter(this._isSideInit);

    await this._resolveLinked(sideEffectProps, data, this.values, "onCreate");
  };

  protected validate = async (prop = "", value: any) => {
    const isSideEffect = this._isSideEffect(prop);

    if (!this._isProp(prop) && !isSideEffect)
      return { valid: false, reasons: ["Invalid property"] };

    const validator = this._getValidator(prop);

    if (validator) return validator(value, this._getContext());

    return { reasons: [], valid: true, validated: value };
  };
}
