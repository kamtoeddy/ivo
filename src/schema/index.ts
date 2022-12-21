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
    const props = [];

    for (let prop of this.props) {
      const { listeners, onChangeListeners } = this._getOperationListeners(
        prop,
        "onCreate"
      );

      if (listeners.length || onChangeListeners.length) props.push(prop);
    }

    return props;
  };

  private _areValuesOk = (values: any) => values && typeof values == "object";

  private _getValues(values: Partial<T>, allowSideEffects = true) {
    const keys = this._getKeysAsProps(values).filter(
      (key) =>
        this.optionsTool.isTimestampKey(key) ||
        this._isProp(key) ||
        (allowSideEffects && this._isSideEffect(key))
    );

    const _values = {} as Partial<T>;

    sort(keys).forEach((key) => (_values[key] = values[key]));

    return _values;
  }

  private _handleError = (error: ErrorTool) => {
    return this._options.errors === "throw"
      ? error.throw()
      : { data: undefined, error: error.summary, handleSuccess: undefined };
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

    const ctx = this._getContext();

    const cleanups = props.map(async (prop) => {
      const listeners = this._getListeners(prop, "onFailure");

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.all(_cleanups);
    });

    await Promise.all(cleanups);
  };

  private _handleInvalidData = () =>
    this._handleError(new ErrorTool({ message: "Invalid Data" }));

  private _handleRequiredBy = (error: ErrorTool) => {
    for (const prop of this.propsRequiredBy) {
      const isRequired = this._getValueBy(prop, "required");
      if (isRequired && this._isUpdatable(prop))
        return error.add(prop, this._getValueBy(prop, "requiredError"));
    }
  };

  private _isUpdatable = (prop: string) => {
    if (this._isSideEffect(prop)) return true;

    if (
      !this._isProp(prop) ||
      this._isConstant(prop) ||
      this._isDependentProp(prop)
    )
      return false;

    const isReadonly = this._isReadonly(prop);

    return (
      !isReadonly ||
      (isReadonly && isEqual(this.defaults[prop], this.values[prop]))
    );
  };

  private _isUpdatableInCTX = (
    prop: string,
    value: any,
    context: ObjectType
  ) => {
    if (!this._isProp(prop) || this._isConstant(prop)) return false;
    return !isEqual(value, context?.[prop]);
  };

  private _makeHandleSuccess = (
    data: Partial<T>,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const props = this._getKeysAsProps(data);

    let successListeners = [] as LifeCycles.SuccessListener<T>[];

    for (const prop of props)
      successListeners = successListeners.concat(
        this._getListeners(prop, "onSuccess") as LifeCycles.SuccessListener<T>[]
      );

    const ctx = this._getContext();

    return async () => {
      const successOperations = successListeners.map(
        async (listener) => await listener(ctx, lifeCycle)
      );

      await Promise.all(successOperations);
    };
  };

  private _prepareListeners = (
    listeners: LifeCycles.ChangeListener<T>[],
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const prepared = listeners.map(
      (listener) => async (ctx: Readonly<T>) => await listener(ctx, lifeCycle)
    );
    return prepared;
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
    const { listeners, onChangeListeners } = this._getOperationListeners(
      prop,
      lifeCycle
    );

    if (
      (!listeners.length && !onChangeListeners.length) ||
      (lifeCycle === "onUpdate" &&
        !this._isSideEffect(prop) &&
        !this._isUpdatableInCTX(prop, operationData[prop], this.values))
    )
      return;

    const isChangeLifeCycle = this._isChangeLifeCycle(lifeCycle);

    const context = Object.freeze({ ...this._getContext(), ...operationData });

    let prepared = isChangeLifeCycle
      ? this._prepareListeners(
          onChangeListeners,
          lifeCycle as LifeCycles.LifeCycle
        )
      : [];

    const handles = [...listeners, ...prepared].map(async (listener) => {
      const extra = await listener(context);

      if (!isChangeLifeCycle || typeof extra !== "object") return;

      const _props = this._getKeysAsProps(extra);

      const resolvedHandles = _props.map(async (prop) => {
        const _value = extra[prop];
        const isSideEffect = this._isSideEffect(prop);

        if (
          this._isReadonly(prop) &&
          !this._isDependentProp(prop) &&
          !this._isUpdatable(prop)
        )
          return;

        if (
          lifeCycle === "onUpdate" &&
          this._isReadonly(prop) &&
          !isEqual(context[prop], this.values[prop])
        )
          return;

        if (!isSideEffect && !this._isUpdatableInCTX(prop, _value, context))
          return;

        await this._validateAndSet(operationData, error, prop, _value);

        await this._resolveLinkedProps(operationData, error, prop, lifeCycle);
      });

      await Promise.all(resolvedHandles);
    });

    await Promise.all(handles);
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

  clone = async (
    values: Partial<T>,
    options: ns.CloneOptions<T> = { reset: [] }
  ) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this.setValues(values);

    const reset = toArray(options.reset).filter(this._isProp);

    const data = {} as T;
    const error = new ErrorTool({ message: "Validation Error" });

    const sideEffects = this._getKeysAsProps(this.values).filter(
      this._isSideInit
    );

    const props = [...this.props, ...sideEffects];

    const validations = props.map(async (prop) => {
      if (this._isConstant(prop)) {
        data[prop] = await this._getConstantValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      const isSideEffect = sideEffects.includes(prop);

      if (isSideEffect && !this._isSideInit(prop)) return;

      if (!isSideEffect && reset.includes(prop)) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] } as T);
      }

      const isLax = this._isLaxProp(prop);

      const isProvided = this.values.hasOwnProperty(prop);

      const isLaxInit =
        isLax && isProvided && !isEqual(this.values[prop], this.defaults[prop]);

      const isRequiredInit =
        this._isRequiredBy(prop) && this.values.hasOwnProperty(prop);

      if (
        (isLax &&
          this._hasAny(prop, "shouldInit") &&
          !this._getValueBy(prop, "shouldInit")) ||
        (!this._isDependentProp(prop) &&
          !isSideEffect &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
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

    await this._resolveLinked(
      data,
      error,
      [...linkedProps, ...sideEffects],
      "onCreate"
    );

    return {
      data: this._useConfigProps(data) as T,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data, "onCreate"),
    };
  };

  create = async (values: Partial<T>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this.setValues(values);

    const data = {} as T;
    const error = new ErrorTool({ message: "Validation Error" });

    const sideEffects = this._getKeysAsProps(this.values).filter(
      this._isSideInit
    );

    const props = [...this.props, ...sideEffects];

    const validations = props.map(async (prop) => {
      if (this._isConstant(prop)) {
        data[prop] = await this._getConstantValue(prop);
        return this._updateContext({ [prop]: data[prop] } as T);
      }

      const isSideEffect = sideEffects.includes(prop);
      if (isSideEffect && !this._isSideInit(prop)) return;

      const isProvided = this.values.hasOwnProperty(prop);

      const isLax = this._isLaxProp(prop);

      const isLaxInit = isLax && isProvided;

      const isRequiredInit = this._isRequiredBy(prop) && isProvided;

      if (
        (isLax &&
          this._hasAny(prop, "shouldInit") &&
          !this._getValueBy(prop, "shouldInit")) ||
        (!isSideEffect && !this._canInit(prop) && !isLaxInit && !isRequiredInit)
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

    await this._resolveLinked(
      data,
      error,
      [...linkedProps, ...sideEffects],
      "onCreate"
    );

    return {
      data: this._useConfigProps(data) as T,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data, "onCreate"),
    };
  };

  delete = async (values: Partial<T>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    const ctx = Object.freeze(
      Object.assign({}, this._getValues(values, false))
    ) as Readonly<T>;

    const cleanups = this.props.map(async (prop) => {
      const listeners = this._getListeners(prop, "onDelete");

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.all(_cleanups);
    });

    await Promise.all(cleanups);
  };

  setValues(values: Partial<T>) {
    this.values = this._getValues(values);

    this._initContext();
  }

  update = async (values: Partial<T>, changes: Partial<T>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

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

    await this._resolveLinked(
      updated,
      error,
      [...linkedProps, ...sideEffects],
      "onUpdate"
    );

    if (!Object.keys(updated).length) {
      await this._handleFailure(updated, error, sideEffects);
      return this._handleError(error.setMessage("Nothing to update"));
    }

    return {
      data: this._useConfigProps(updated, true),
      error: undefined,
      handleSuccess: this._makeHandleSuccess(updated, "onUpdate"),
    };
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

  delete = this.modelTool.delete;

  update = this.modelTool.update;

  validate = this.modelTool.validate;
}
