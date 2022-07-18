import { ApiError } from "../utils/ApiError";
import { belongsTo } from "../utils/functions";
import { looseObject } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  fxLooseObject,
  ISchemaOptions,
  IValidateProps,
  LifeCycleRule,
  Private_ISchemaOptions,
  PropDefinitionRule,
  PropDefinitionRules,
} from "./interfaces";
import { SchemaOptions } from "./SchemaOptions";

export const defaultOptions: ISchemaOptions = { timestamps: false };

const lifeCycleRules: LifeCycleRule[] = ["onCreate", "onUpdate"];

export abstract class SchemaCore {
  [key: string]: any;

  protected error = new ApiError({ message: "Validation Error" });

  protected _helper: SchemaOptions;
  protected _options: ISchemaOptions;
  protected _propDefinitions: PropDefinitionRules = {};

  protected context: looseObject = {};
  protected updated: looseObject = {};

  protected defaults: looseObject = {};
  protected linkedUpdates: looseObject = {};
  protected props: string[] = [];
  protected values: looseObject = {};

  constructor(
    propDefinitions: PropDefinitionRules,
    options: ISchemaOptions = defaultOptions
  ) {
    this._propDefinitions = propDefinitions;
    this._options = options;

    this._helper = new SchemaOptions(this._makeOptions(options));
  }

  public get options() {
    return this._options;
  }

  public get propDefinitions() {
    return this._propDefinitions;
  }

  protected _canInit = (prop: string): boolean => {
    const propDef = this._propDefinitions[prop];

    if (!propDef) return false;

    const { readonly, required, shouldInit } = propDef;

    if (this._isDependentProp(prop)) return false;

    if (!readonly && !required) return false;

    return belongsTo(shouldInit, [true, undefined]);
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

  protected _getCloneObject = async (toReset: string[] = []) => {
    let obj: looseObject = this.props.reduce((values: looseObject, next) => {
      values[next] = toReset.includes(next)
        ? this.defaults[next] ?? this.values[next]
        : this.values[next] ?? this.defaults[next];

      return values;
    }, {});

    obj = await this._useSideInitProps(obj);

    return this._useConfigProps(obj);
  };

  protected _getContext = (): looseObject => {
    this.props.forEach((prop) => (this.context[prop] = this.values[prop]));

    return { ...this.context, ...this.updated };
  };

  protected _getCreateActions = () => {
    let actions: fxLooseObject[] = [];

    for (let prop of this.props) {
      const _actions = (this._getHandlers(prop, "onCreate") ?? []).map(
        (dt) => dt.method
      );

      if (_actions?.length) actions = [...actions, ..._actions];
    }

    return actions;
  };

  protected _getCreateObject = async () => {
    const createProps = this._getCreateProps();

    let obj: looseObject = {};

    for (let prop of this.props) {
      const checkLax = this._isLaxProp(prop) && this.hasOwnProperty(prop);
      const isSideInit = this._isSideInit(prop) && this.hasOwnProperty(prop);

      if (createProps.includes(prop) || checkLax || isSideInit) {
        const { reasons, valid, validated } = await this.validate({
          prop,
          value: this.values[prop],
        });

        if (valid) {
          obj[prop] = validated;

          continue;
        }

        this.error.add(prop, reasons);
      }

      obj[prop] = this.defaults[prop];
    }

    obj = await this._useSideInitProps(obj);

    return this._useConfigProps(obj);
  };

  protected _getCreateProps = () => {
    const createProps = [];

    for (let prop of this.props)
      if (this._canInit(prop)) createProps.push(prop);

    return this._sort(createProps);
  };

  protected _getDefaults = () => {
    const defaults: looseObject = {};

    for (let prop of this.props) {
      const _default = this._propDefinitions[prop]?.default;

      if (_default !== undefined) defaults[prop] = _default;
    }

    return defaults;
  };

  protected _getDefinitionValue = (prop: string, rule: PropDefinitionRule) => {
    return this._propDefinitions[prop]?.[rule];
  };

  protected _getHandlers = (
    prop: string,
    type: LifeCycleRule,
    valid = true
  ) => {
    const propDef = this._propDefinitions[prop];

    return propDef?.[type]
      ?.map((method, index) => {
        return { index, method, valid: this._isFunction(method) };
      })
      .filter((data) => data.valid === valid);
  };

  protected _getLinkedMethods = (prop: string): fxLooseObject[] => {
    const methods = this._isSideEffect(prop)
      ? this._propDefinitions[prop]?.onUpdate
      : this.linkedUpdates?.[prop];

    return methods ?? [];
  };

  protected _getLinkedUpdates = () => {
    const _linkedUpdates: looseObject = {};

    for (let prop of this.props) {
      let _updates = this._propDefinitions[prop]?.onUpdate ?? [];

      _updates = _updates.filter((action) => this._isFunction(action));

      if (_updates?.length) _linkedUpdates[prop] = _updates;
    }

    return _linkedUpdates;
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

  protected _handleCreateActions = async (data: looseObject = {}) => {
    const actions = this._getCreateActions();

    for (const cb of actions) {
      const extra = await cb(data);

      if (typeof extra !== "object") continue;

      const _props = Object.keys(extra);

      for (let _prop of _props) {
        if (!this._isProp(_prop)) continue;

        const _value = extra[_prop];

        data[_prop] = _value;

        await this._resolveLinkedValue(data, _prop, _value);
      }
    }

    this.context = {};

    return data;
  };

  protected _hasChanged = (prop: string) => {
    const propDef = this._propDefinitions[prop];

    if (!propDef) return false;

    return !isEqual(propDef.default, this._getContext()?.[prop]);
  };

  protected _hasDefault = (prop: string) => {
    const propDef = this._propDefinitions[prop];

    if (!propDef) return false;

    return !isEqual(propDef.default, undefined);
  };

  protected _has = (
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

  protected _hasHandlersFor = (
    prop: string,
    type: "onCreate" | "onUpdate" = "onUpdate",
    _number = 1
  ) => {
    return this._getHandlers(prop, type)?.length ?? 0 >= _number;
  };

  protected __isDependentProp = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) reasons = reasons.concat(isPopDefOk.reasons!);

    const propDef = this._propDefinitions[prop];

    const { dependent, sideEffect } = propDef;

    if (sideEffect) reasons.push("Dependent props cannot be sideEffect");

    if (!dependent)
      reasons.push("Dependent props must have dependent as 'true'");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isDependentProp = (prop: string): boolean =>
    this.__isDependentProp(prop).valid;

  protected _isErroneous = () => this.error.isPayloadLoaded;

  protected _isFunction = (obj: any): boolean => typeof obj === "function";

  protected __isLaxProp = (prop: string) => {
    let reasons: string[] = [];

    const isPopDefOk = this._isPropDefinitionObjectOk(prop);

    if (!isPopDefOk.valid) reasons = reasons.concat(isPopDefOk.reasons!);

    const hasDefaultValue = this._hasDefault(prop),
      isDependent = this._isDependentProp(prop);

    if (!hasDefaultValue) reasons.push("No default value");

    const propDef = this._propDefinitions[prop];

    const { readonly, required, sideEffect } = propDef;

    if (isDependent || readonly || required || sideEffect) {
      reasons.push(
        "dependent, readonly, required and sideEffect should not be 'true'"
      );
    }

    const shouldInit = belongsTo(propDef?.shouldInit, [true, undefined]);

    if (!shouldInit) reasons.push("shouldInit must be true");

    return { reasons, valid: reasons.length === 0 };
  };

  protected _isLaxProp = (prop: string): boolean =>
    this.__isLaxProp(prop).valid;

  protected _isLinkedUpdate = (prop: string): boolean =>
    (this._getHandlers(prop, "onUpdate")?.length ?? 0) > 0;

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

    if (this._has(prop, "dependent") && !dependentDef.valid)
      reasons = reasons.concat(dependentDef.reasons);

    const sideEffectDef = this.__isSideEffect(prop);

    if (this._has(prop, "sideEffect") && !sideEffectDef.valid)
      reasons = reasons.concat(sideEffectDef.reasons);

    if (this._has(prop, "validator") && !this._isValidatorOk(prop))
      reasons.push("Invalid validator");

    for (let rule of lifeCycleRules) {
      if (!this._has(prop, rule)) continue;

      const invalidHandlers = this._getHandlers(prop, rule, false);

      if (!invalidHandlers?.length) continue;

      reasons = reasons.concat(
        invalidHandlers.map(
          (dt) => `'${dt.method}' @${rule}[${dt.index}] is not a function`
        )
      );
    }

    if (
      this._getDefinitionValue(prop, "shouldInit") === false &&
      !this._has(prop, "default")
    )
      reasons.push(
        "A property that should not be initialized must have a default value other than 'undefined'"
      );

    if (
      !this._has(prop, ["default", "readonly", "required"]) &&
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

    if (!isPopDefOk.valid) reasons = reasons.concat(isPopDefOk.reasons!);

    if (this._has(prop, ["default", "readonly", "required"]))
      reasons.push(
        "SideEffects cannot have default, readonly, required as 'true'"
      );

    if (!this._isValidatorOk(prop)) reasons.push("Invalid validator");

    if (!this._hasHandlersFor(prop, "onUpdate"))
      reasons.push("SideEffects must have at least on onUpdate handler");

    const { sideEffect } = this._propDefinitions[prop];

    if (!sideEffect === true) reasons.push("sideEffect should be 'true'");

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

    const readonly = this._propDefinitions[prop]?.readonly;

    return !readonly || (readonly && !this._hasChanged(prop));
  };

  protected _isUpdatableInCTX = (
    prop: string,
    value: any,
    context: looseObject = this._getContext()
  ) => {
    if (!this._isProp(prop)) return false;

    return !isEqual(value, context?.[prop]);
  };

  protected _isValidatorOk = (prop: string) => {
    const propDef = this._propDefinitions[prop];

    return this._isFunction(propDef?.validator);
  };

  private _makeOptions(options: ISchemaOptions): Private_ISchemaOptions {
    if (!options) return { timestamps: { createdAt: "", updatedAt: "" } };

    let { timestamp, timestamps } = options;

    timestamps = timestamps ?? timestamp;

    let createdAt = "createdAt",
      updatedAt = "updatedAt";

    if (!timestamps || timestamps === true) {
      let _timestamp = timestamps
        ? { createdAt, updatedAt }
        : { createdAt: "", updatedAt: "" };

      return { ...options, timestamps: _timestamp };
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

  protected _resolveLinkedValue = async (
    contextObject: looseObject = {},
    prop: string,
    value: any
  ) => {
    const isLinked = this._isLinkedUpdate(prop),
      isSideEffect = this._isSideEffect(prop);

    if (!isSideEffect && !isLinked) return;

    const { reasons, valid, validated } = await this.validate({
      prop,
      value,
    });

    if (!valid) return this.error.add(prop, reasons);

    const hasChanged = !isEqual(this.values[prop], validated);

    if (!isSideEffect && !hasChanged) return;

    if (isSideEffect) this.context[prop] = validated;

    const context = { ...this._getContext(), ...contextObject };

    const methods = this._getLinkedMethods(prop);

    for (const cb of methods) {
      const extra = await cb(context);

      if (typeof extra !== "object") continue;

      const _props = Object.keys(extra);

      for (let _prop of _props) {
        const _value = extra[_prop];
        const isSideEffect = this._isSideEffect(_prop);

        if (!isSideEffect && !this._isUpdatableInCTX(_prop, _value, context))
          continue;

        if (!isSideEffect) contextObject[_prop] = _value;

        await this._resolveLinkedValue(contextObject, _prop, _value);
      }
    }
  };

  protected _throwErrors(_message?: string): void {
    let err = new ApiError(this.error.getInfo());

    this.error.clear();

    if (_message) err.setMessage(_message);

    throw err;
  }

  protected _sort = (data: any[]): any[] =>
    data.sort((a, b) => (a < b ? -1 : 1));

  protected _sortKeys = (obj: looseObject): looseObject => {
    const keys = this._sort(Object.keys(obj));

    return keys.reduce((prev, next) => {
      prev[next] = obj[next];

      return prev;
    }, {});
  };

  protected _useConfigProps = (obj: looseObject, asUpdate = false) => {
    if (!this._helper.withTimestamps) return obj;

    const createdAt = this._helper.getCreateKey(),
      updatedAt = this._helper.getUpdateKey();

    const results = asUpdate
      ? { ...obj, [updatedAt]: new Date() }
      : { ...obj, [createdAt]: new Date(), [updatedAt]: new Date() };

    return this._sortKeys(results);
  };

  protected _useSideInitProps = async (obj: looseObject) => {
    const sideEffectProps = this._getSideEffects();

    for (let prop of sideEffectProps) {
      if (!this._isSideInit(prop) || !this.hasOwnProperty(prop)) continue;

      const { reasons, valid, validated } = await this.validate({
        prop,
        value: this[prop],
      });

      if (valid) {
        const methods = this._getLinkedMethods(prop);
        const context = this._getContext();

        context[prop] = validated;

        for (const cb of methods) {
          const extra = await cb(context);

          if (typeof extra !== "object") continue;

          Object.keys(extra).forEach((_prop) => {
            if (this._isProp(_prop)) obj[_prop] = extra[_prop];
          });
        }

        continue;
      }

      this.error.add(prop, reasons);
    }

    return obj;
  };

  protected validate = async ({ prop = "", value }: IValidateProps) => {
    const isSideEffect = this._isSideEffect(prop);

    if (!this._isProp(prop) && !isSideEffect)
      return { valid: false, reasons: ["Invalid property"] };

    const validator = this._getValidator(prop);

    if (!validator && isEqual(value, "undefined")) {
      return { valid: false, reasons: ["Invalid value"] };
    }

    if (validator) return validator(value, this._getContext());

    return { reasons: [], valid: true, validated: value };
  };
}
