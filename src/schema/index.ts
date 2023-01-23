import { sort, sortKeys, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  CombineTypes,
  LifeCycles,
  Schema as ns,
  SpreadType,
  StringKey,
} from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";
import { makeResponse } from "./utils";
import { ErrorTool } from "./utils/schema-error";

export { Schema };

class Schema<
  I extends ObjectType,
  O extends ObjectType = I
> extends SchemaCore<I> {
  constructor(
    propDefinitions: ns.PropertyDefinitions<I>,
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

  extend = <U extends ObjectType, V extends ObjectType = U>(
    propDefinitions: ns.PropertyDefinitions<
      SpreadType<CombineTypes<I, U> & U>,
      CombineTypes<O, V>
    >,
    options: ns.ExtensionOptions<StringKey<I>> = {
      ...defaultOptions,
      remove: [],
    }
  ) => {
    const remove = toArray(options?.remove ?? []);
    delete options.remove;

    type InputType = CombineTypes<I, U>;
    type OutputType = CombineTypes<O, V>;

    let _propDefinitions = {
      ...this.propDefinitions,
    } as ns.PropertyDefinitions<InputType, OutputType>;

    remove?.forEach(
      (prop) => delete _propDefinitions?.[prop as StringKey<InputType>]
    );

    _propDefinitions = {
      ..._propDefinitions,
      ...propDefinitions,
    } as ns.PropertyDefinitions<InputType, OutputType>;

    return new Schema<InputType, OutputType>(_propDefinitions as any, options);
  };

  getModel = () => new Model(new ModelTool<I, O>(this));
}

class ModelTool<
  I extends ObjectType,
  O extends ObjectType = I
> extends SchemaCore<I> {
  constructor(schema: Schema<I, O>) {
    super(schema.propDefinitions, schema.options);
  }

  private _getCreatePropsWithListeners = () => {
    const props = [];

    // for (let prop of this.props) {
    //   const { listeners, onChangeListeners } = this._getOperationListeners(
    //     prop,
    //     "onCreate"
    //   );

    //   if (listeners.length || onChangeListeners.length) props.push(prop);
    // }

    return [];
  };

  private _areValuesOk = (values: any) => values && typeof values == "object";

  private _getValues(values: Partial<I | O>, allowSideEffects = true) {
    const keys = this._getKeysAsProps(values).filter(
      (key) =>
        (this.optionsTool.withTimestamps &&
          this.optionsTool.isTimestampKey(key)) ||
        this._isProp(key) ||
        (allowSideEffects && this._isSideEffect(key))
    );

    const _values = {} as Partial<I>;

    sort(keys).forEach((key) => (_values[key] = values[key]));

    return _values;
  }

  private _handleError = (error: ErrorTool) => {
    return this._options.errors === "throw"
      ? error.throw()
      : { data: undefined, error: error.summary, handleSuccess: undefined };
  };

  private _handleFailure = async (
    data: Partial<I>,
    error: ErrorTool,
    sideEffects: StringKey<I>[] = []
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

  private _handleRequiredBy = (
    error: ErrorTool,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    for (const prop of this.propsRequiredBy) {
      const [isRequired, message] = this._getRequiredState(prop, lifeCycle);

      if (isRequired && this._isUpdatable(prop)) error.add(prop, message);
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
    if (
      !this._isProp(prop) ||
      this._isConstant(prop) ||
      this._isDependentProp(prop)
    )
      return false;
    return !isEqual(value, context?.[prop]);
  };

  private _makeHandleSuccess = (
    data: Partial<I>,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const ctx = this._getContext();
    const finalCtx = this._getFinalContext();

    const successFulSideEffects = this._getKeysAsProps(finalCtx).filter(
      this._isSideEffect
    );

    const props = this._getKeysAsProps(data);

    const successProps = [...props, ...successFulSideEffects];

    let successListeners = [] as LifeCycles.SuccessListener<I>[];

    for (const prop of successProps)
      successListeners = successListeners.concat(
        this._getListeners(prop, "onSuccess") as LifeCycles.SuccessListener<I>[]
      );

    return async () => {
      const successOperations = successListeners.map(
        async (listener) => await listener(ctx, lifeCycle)
      );

      await Promise.all(successOperations);
    };
  };

  private _resolveLinked = async (
    operationData: Partial<I>,
    error: ErrorTool,
    props: StringKey<I>[],
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    const listenersUpdates = props.map((prop) => {
      return this._resolveLinkedProps(operationData, error, prop, lifeCycle);
    });

    await Promise.all(listenersUpdates);
  };

  private _resolveLinkedProps = async (
    operationData: Partial<I> = {},
    error: ErrorTool,
    prop: StringKey<I>,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    // const { listeners, onChangeListeners } = this._getOperationListeners(
    //   prop,
    //   lifeCycle
    // );
    // if (
    //   (!listeners.length && !onChangeListeners.length) ||
    //   (lifeCycle === "onUpdate" &&
    //     !this._isSideEffect(prop) &&
    //     !this._isUpdatableInCTX(prop, operationData[prop], this.values))
    // )
    //   return;
    // const isChangeLifeCycle = this._isChangeLifeCycle(lifeCycle);
    // const context = Object.freeze({ ...this._getContext(), ...operationData });
    // let prepared = isChangeLifeCycle
    //   ? this._prepareListeners(
    //       onChangeListeners,
    //       lifeCycle as LifeCycles.LifeCycle
    //     )
    //   : [];
    // const handles = [...listeners, ...prepared].map(async (listener) => {
    //   const extra = (await listener(context)) as Partial<I>;
    //   if (!isChangeLifeCycle || typeof extra !== "object") return;
    //   const _props = this._getKeysAsProps(extra);
    //   const resolvedHandles = _props.map(async (prop) => {
    //     const _value = extra[prop];
    //     const isSideEffect = this._isSideEffect(prop);
    //     if (this._isReadonly(prop) && !this._isUpdatable(prop)) return;
    // if (
    //   lifeCycle === "onUpdate" &&
    //   this._isReadonly(prop) &&
    //   !isEqual(context[prop], this.values[prop])
    // )
    //   return;
    //   if (!isSideEffect && !this._isUpdatableInCTX(prop, _value, context))
    //     return;
    //   await this._validateAndSet(operationData, error, prop, _value);
    //   await this._resolveLinkedProps(operationData, error, prop, lifeCycle);
    // });
    // await Promise.all(resolvedHandles);
    // });
    // await Promise.all(handles);
  };

  private _useConfigProps = (obj: I | Partial<I>, asUpdate = false) => {
    if (!this.optionsTool.withTimestamps) return sortKeys(obj);

    const createdAt = this.optionsTool.getCreateKey(),
      updatedAt = this.optionsTool.getUpdateKey();

    const results = asUpdate
      ? { ...obj, [updatedAt]: new Date() }
      : { ...obj, [createdAt]: new Date(), [updatedAt]: new Date() };

    return sortKeys(results);
  };

  private _validateAndSet = async (
    operationData: Partial<I> = {},
    error: ErrorTool,
    prop: StringKey<I>,
    value: any
  ) => {
    const isValid = await this._validate(prop, value, this._getContext());

    if (!isValid.valid) return error.add(prop, isValid.reasons);

    let { validated } = isValid;

    if (isEqual(validated, undefined)) validated = value;

    if (!this._isSideEffect(prop)) operationData[prop] = validated;

    const validCtxUpdate = { [prop]: validated } as I;

    this._updateContext(validCtxUpdate);
    this._updateFinalContext(validCtxUpdate);
  };

  clone = async (
    values: Partial<I>,
    options: ns.CloneOptions<I> = { reset: [] }
  ) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this.setValues(values);

    const reset = toArray<StringKey<I>>(options.reset ?? []).filter(
      this._isProp
    );

    const data = {} as I;
    const error = new ErrorTool({ message: "Validation Error" });

    const sideEffects = this._getKeysAsProps(this.values).filter(
      this._isSideInit
    );

    const props = [...this.props, ...sideEffects];

    const validations = props.map(async (prop) => {
      if (this._isConstant(prop)) {
        data[prop] = await this._getConstantValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      if (this._isDependentProp(prop)) {
        const value = reset.includes(prop)
          ? this._getDefaultValue(prop)
          : this.values[prop];

        data[prop] = value;

        const validCtxUpdate = { [prop]: data[prop] } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      const isSideEffect = sideEffects.includes(prop);

      if (isSideEffect && !this._isSideInit(prop)) return;

      if (!isSideEffect && reset.includes(prop)) {
        data[prop] = this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
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
          !this._getValueBy(prop, "shouldInit", "creating")) ||
        (!isSideEffect && !this._canInit(prop) && !isLaxInit && !isRequiredInit)
      ) {
        data[prop] = this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.all(validations);

    this._handleRequiredBy(error, "creating");

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, sideEffects);
      return this._handleError(error);
    }

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(
      data,
      error,
      [...linkedProps, ...sideEffects],
      "creating"
    );

    return {
      data: this._useConfigProps(data) as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data, "creating"),
    };
  };

  create = async (values: Partial<I>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this.setValues(values);

    const data = {} as I;
    const error = new ErrorTool({ message: "Validation Error" });

    const sideEffects = this._getKeysAsProps(this.values).filter(
      this._isSideInit
    );

    const props = [...this.props, ...sideEffects];

    const validations = props.map(async (prop) => {
      if (this._isConstant(prop)) {
        data[prop] = await this._getConstantValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
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
          !this._getValueBy(prop, "shouldInit", "creating")) ||
        (!isSideEffect && !this._canInit(prop) && !isLaxInit && !isRequiredInit)
      ) {
        data[prop] = this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.all(validations);

    this._handleRequiredBy(error, "creating");

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, sideEffects);
      return this._handleError(error);
    }

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(
      data,
      error,
      [...linkedProps, ...sideEffects],
      "creating"
    );

    return {
      data: this._useConfigProps(data) as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data, "creating"),
    };
  };

  delete = async (values: O) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    const ctx = Object.freeze(
      Object.assign({}, this._getValues(values, false))
    ) as Readonly<O>;

    const cleanups = this.props.map(async (prop) => {
      const listeners = this._getListeners<O>(prop, "onDelete");

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.all(_cleanups);
    });

    await Promise.all(cleanups);
  };

  setValues(values: Partial<I>) {
    this.values = this._getValues(values);

    this._initContexts();
  }

  update = async (values: Partial<I>, changes: Partial<I>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this.setValues(values);

    const error = new ErrorTool({ message: "Validation Error" });
    const updated = {} as Partial<I>;

    const toUpdate = this._getKeysAsProps(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop)
    );

    const linkedProps: StringKey<I>[] = [];
    const sideEffects: StringKey<I>[] = [];

    const validations = toUpdate.map(async (prop) => {
      const value = changes[prop] as Exclude<
        I[Extract<keyof I, string>],
        undefined
      >;
      const isValid = await this._validate(prop, value, this._getContext());

      if (!isValid.valid) return error.add(prop, isValid.reasons);

      let { validated } = isValid;

      if (isEqual(validated, undefined)) validated = value;

      if (isEqual(validated, this.values[prop])) return;

      if (this._isSideEffect(prop)) sideEffects.push(prop);
      else {
        updated[prop] = validated;
        linkedProps.push(prop);
      }

      const validCtxUpdate = { [prop]: validated } as I;

      this._updateContext(validCtxUpdate);
      this._updateFinalContext(validCtxUpdate);
    });

    await Promise.all(validations);

    this._handleRequiredBy(error, "updating");

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
      "updating"
    );

    if (!Object.keys(updated).length) {
      await this._handleFailure(updated, error, sideEffects);
      return this._handleError(error.setMessage("Nothing to update"));
    }

    return {
      data: this._useConfigProps(updated, true),
      error: undefined,
      handleSuccess: this._makeHandleSuccess(updated, "updating"),
    };
  };

  _validate = async <K extends StringKey<I>>(
    prop: K,
    value: any,
    ctx: Readonly<I>
  ) => {
    if (
      this._isConstant(prop) ||
      this._isDependentProp(prop) ||
      (!this._isProp(prop) && !this._isSideEffect(prop))
    )
      return makeResponse<I[K]>({ valid: false, reason: "Invalid property" });

    const validator = this._getValidator(prop);

    if (validator) {
      const res = await validator(value, ctx);

      if (res.valid && isEqual(res.validated, undefined)) res.validated = value;

      return makeResponse<I[K]>(res);
    }

    return makeResponse<I[K]>({ valid: true, validated: value });
  };
}

class Model<I extends ObjectType, O extends ObjectType = I> {
  constructor(private modelTool: ModelTool<I, O>) {}

  clone = this.modelTool.clone;

  create = this.modelTool.create;

  delete = this.modelTool.delete;

  update = this.modelTool.update;

  validate = async <K extends StringKey<I>>(prop: K, value: any) =>
    this.modelTool._validate(prop, value, {} as Readonly<I>);
}
