import { sort, sortKeys, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import { LifeCycles, Schema as ns, StringKey } from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";
import { makeResponse } from "./utils";
import { ErrorTool } from "./utils/schema-error";

export class Schema<T extends ObjectType> extends SchemaCore<T> {
  constructor(
    propDefinitions: ns.PropertyDefinitions<T>,
    options: ns.Options = defaultOptions
  ) {
    super(propDefinitions, options);
  }

  get options() {
    return this._options;
  }

  get propDefinitions() {
    return this._propDefinitions;
  }

  private _useExtensionOptions = <T extends ObjectType>(
    options: ns.ExtensionOptions<T>
  ) => {
    const remove = toArray(options?.remove ?? []);

    remove?.forEach((prop) => delete this._propDefinitions?.[prop]);

    return this;
  };

  extend = <U extends ObjectType>(
    parent: Schema<U>,
    options: ns.ExtensionOptions<U> = { remove: [] }
  ) => {
    this._propDefinitions = {
      ...parent.propDefinitions,
      ...this._propDefinitions,
    } as ns.PropertyDefinitions<T>;

    return this._useExtensionOptions(options);
  };

  getModel = () => new Model(new ModelTool(this));
}

class ModelTool<T extends ObjectType> extends SchemaCore<T> {
  constructor(schema: Schema<T>) {
    super(schema.propDefinitions, schema.options);
  }

  private _getCreatePropsWithListeners = () => {
    const listeners = [];

    for (let prop of Array.from(this.props))
      if (this._getOperationListeners(prop, "onCreate")?.length)
        listeners.push(prop);

    return listeners;
  };

  private _isUpdatable = (prop: string) => {
    if (this._isSideEffect(prop)) return true;

    if (this._isConstant(prop)) return false;

    if (!this._isProp(prop) || this._isDependentProp(prop)) return false;

    const { readonly } = this._getDefinition(prop);

    const _default = this._getDefaultValue(prop, false);

    return !readonly || (readonly && isEqual(_default, this.values[prop]));
  };

  private _isUpdatableInCTX = (
    prop: string,
    value: any,
    context: ObjectType = this._getContext()
  ) => {
    return this._isConstant(prop) || !this._isProp(prop)
      ? false
      : !isEqual(value, context?.[prop]);
  };

  private _resolveLinked = async (
    operationData: Partial<T>,
    error: ErrorTool,
    props: StringKey<T>[],
    lifeCycle: LifeCycles.Rule
  ) => {
    const listenersUpdates = props.map((prop) => {
      return this._resolveLinkedProps(operationData, error, prop, lifeCycle);
    });

    await Promise.all(listenersUpdates);
  };

  private _resolveLinkedProps = async (
    operationData: Partial<T> = {},
    error: ErrorTool,
    prop: StringKey<T>,
    lifeCycle: LifeCycles.Rule
  ) => {
    const listeners = this._getOperationListeners(prop, lifeCycle);

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

      const _props = this._getKeysAsProps(extra);

      for (let _prop of _props) {
        const _value = extra[_prop];
        const isSideEffect = this._isSideEffect(_prop);

        if (!isSideEffect && !this._isUpdatableInCTX(_prop, _value, context))
          continue;

        await this._validateAndSet(operationData, error, _prop, _value);

        await this._resolveLinkedProps(operationData, error, _prop, lifeCycle);
      }
    }
  };

  private _validateAndSet = async (
    operationData: Partial<T> = {},
    error: ErrorTool,
    prop: StringKey<T>,
    value: any
  ) => {
    let { reasons, valid, validated } = await this.validate(prop, value);

    if (!valid) return error.add(prop, reasons);

    if (isEqual(validated, undefined)) validated = value;

    if (!this._isSideEffect(prop)) operationData[prop] = validated;

    this._updateContext({ [prop]: validated } as T);
  };

  private _useConfigProps = (obj: T | Partial<T>, asUpdate = false) => {
    if (!this.optionsTool.withTimestamps) return sortKeys(obj);

    const createdAt = this.optionsTool.getCreateKey(),
      updatedAt = this.optionsTool.getUpdateKey();

    const results = asUpdate
      ? { ...obj, [updatedAt]: new Date() }
      : { ...obj, [createdAt]: new Date(), [updatedAt]: new Date() };

    return sortKeys(results);
  };

  private _handleRequiredBy = (error: ErrorTool) => {
    for (const prop of this.propsRequiredBy) {
      const isRequired = this._getValueBy(prop, "required");
      if (isRequired && this._isUpdatable(prop))
        return error.add(prop, this._getValueBy(prop, "requiredError"));
    }
  };

  private _handleError = (error: ErrorTool) => {
    return this._options.errors === "throw"
      ? error.throw()
      : { data: undefined, error: error.summary };
  };

  private _handleFailure = async (
    data: Partial<T>,
    error: ErrorTool,
    sideEffects: StringKey<T>[] = []
  ) => {
    const props = [
      ...this._getKeysAsProps({ ...data, ...error.payload }),
      ...sideEffects,
    ];

    const cleanups = props.map(async (prop) => {
      const listeners = this._getListeners(prop, "onFailure");

      for (const listener of listeners) await listener(this._getContext());
    });

    await Promise.all(cleanups);
  };

  clone = async (
    values: Partial<T>,
    options: ns.CloneOptions<T> = { reset: [] }
  ) => {
    this.setValues(values);

    const reset = toArray(options.reset).filter(this._isProp);

    const data = {} as T;
    const error = new ErrorTool({ message: "Validation Error" });

    const sideEffects = this._getKeysAsProps(this.values).filter(
      this._isSideInit
    );

    const props = [...Array.from(this.props), ...sideEffects];

    const validations = props.map((prop) => {
      if (this._isConstant(prop))
        return (data[prop] = this._getValueBy(prop, "value"));

      const isSideEffect = sideEffects.includes(prop);

      if (isSideEffect && !this._isSideInit(prop)) return;

      if (!isSideEffect && reset.includes(prop)) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      const isLaxInit =
        this._isLaxProp(prop) &&
        this.values.hasOwnProperty(prop) &&
        !isEqual(this.values[prop], this.defaults[prop]);

      const isRequiredInit =
        this._isRequiredBy(prop) && this.values.hasOwnProperty(prop);

      if (
        !isSideEffect &&
        !this._canInit(prop) &&
        !isLaxInit &&
        !isRequiredInit
      ) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.all(validations);

    this._handleRequiredBy(error);

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, sideEffects);
      return this._handleError(error);
    }

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(data, error, linkedProps, "onCreate");

    await this._resolveLinked(data, error, sideEffects, "onCreate");

    return { data: this._useConfigProps(data) as T, error: undefined };
  };

  create = async (values: Partial<T>) => {
    this.setValues(values);

    const data = {} as T;
    const error = new ErrorTool({ message: "Validation Error" });

    const sideEffects = this._getKeysAsProps(this.values).filter(
      this._isSideInit
    );

    const props = [...Array.from(this.props), ...sideEffects];

    const validations = props.map((prop) => {
      if (this._isConstant(prop)) {
        data[prop] = this._getValueBy(prop, "value");

        return this._updateContext({ [prop]: data[prop] } as T);
      }

      const isSideEffect = sideEffects.includes(prop);
      if (isSideEffect && !this._isSideInit(prop)) return;

      const isProvided = this.values.hasOwnProperty(prop);

      const isLaxInit = this._isLaxProp(prop) && isProvided;

      const isRequiredInit = this._isRequiredBy(prop) && isProvided;

      if (
        !isSideEffect &&
        !this._canInit(prop) &&
        !isLaxInit &&
        !isRequiredInit
      ) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] } as T);
      }

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.all(validations);

    this._handleRequiredBy(error);

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, sideEffects);
      return this._handleError(error);
    }

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(data, error, linkedProps, "onCreate");

    await this._resolveLinked(data, error, sideEffects, "onCreate");

    return { data: this._useConfigProps(data) as T, error: undefined };
  };

  setValues(values: Partial<T>) {
    const keys = this._getKeysAsProps(values).filter(
      (key) =>
        this.optionsTool.isTimestampKey(key) ||
        this._isProp(key) ||
        this._isSideEffect(key)
    );

    this.values = {};

    sort(keys).forEach((key) => (this.values[key] = values[key]));

    this._initContext();
  }

  update = async (values: Partial<T>, changes: Partial<T>) => {
    this.setValues(values);

    const error = new ErrorTool({ message: "Validation Error" });
    const updated = {} as Partial<T>;

    const toUpdate = this._getKeysAsProps(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop)
    );

    const linkedProps: StringKey<T>[] = [];
    const sideEffects: StringKey<T>[] = [];

    const validations = toUpdate.map(async (prop) => {
      const value = changes[prop];
      let { reasons, valid, validated } = await this.validate(prop, value);

      if (!valid) return error.add(prop, reasons);

      if (isEqual(validated, undefined)) validated = value;

      if (isEqual(validated, this.values[prop])) return;

      if (this._isSideEffect(prop)) sideEffects.push(prop);
      else {
        updated[prop] = validated;
        linkedProps.push(prop);
      }

      this._updateContext({ [prop]: validated } as T);
    });

    await Promise.all(validations);

    this._handleRequiredBy(error);

    if (error.isPayloadLoaded) {
      await this._handleFailure(updated, error, sideEffects);
      return this._handleError(error);
    }

    if (!Object.keys(updated).length && !sideEffects.length) {
      await this._handleFailure(updated, error, sideEffects);
      return this._handleError(error.setMessage("Nothing to update"));
    }

    await this._resolveLinked(updated, error, linkedProps, "onUpdate");

    await this._resolveLinked(updated, error, sideEffects, "onUpdate");

    if (!Object.keys(updated).length) {
      await this._handleFailure(updated, error, sideEffects);
      return this._handleError(error.setMessage("Nothing to update"));
    }

    return { data: this._useConfigProps(updated, true), error: undefined };
  };

  validate = async <K extends StringKey<T>>(prop: K, value: any) => {
    if (
      this._isConstant(prop) ||
      (!this._isProp(prop) && !this._isSideEffect(prop))
    )
      return { valid: false, reasons: ["Invalid property"] };

    const validator = this._getValidator(prop);

    if (validator)
      return makeResponse<T[K]>(await validator(value, this._getContext()));

    return makeResponse<T[K]>({ valid: true, validated: value });
  };
}

class Model<T extends ObjectType> {
  constructor(private modelTool: ModelTool<T>) {}

  clone = this.modelTool.clone;

  create = this.modelTool.create;

  update = this.modelTool.update;

  validate = this.modelTool.validate;
}
