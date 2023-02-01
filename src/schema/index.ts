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
    propDefinitions: ns.PropertyDefinitions<
      SpreadType<CombineTypes<O, I> & O>,
      O
    >,
    options: ns.Options = defaultOptions
  ) {
    super(propDefinitions as ns.Definitions<I>, options as ns.Options);
  }

  get options() {
    return this._options;
  }

  get propDefinitions() {
    return this._propDefinitions;
  }

  extend = <U extends ObjectType, V extends ObjectType = U>(
    propDefinitions: Partial<
      ns.PropertyDefinitions<
        SpreadType<CombineTypes<I, U> & U>,
        CombineTypes<O, V>
      >
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

  private _areValuesOk = (values: any) => values && typeof values == "object";

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
    let props = [
      ...this._getKeysAsProps({ ...data, ...error.payload }),
      ...sideEffects,
    ];

    props = Array.from(new Set(props));

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

  private _handleSanitizationOfSideEffects = async (
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    let sanitizers: [StringKey<I>, Function][] = [];

    const ctx = this._getContext();
    const finalCtx = this._getFinalContext();

    const successFulSideEffects = this._getKeysAsProps(finalCtx).filter(
      this._isSideEffect
    );

    for (const prop of successFulSideEffects) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, lifeCycle);

      if (!isSanitizable) continue;

      sanitizers.push([prop, sanitizer]);
    }

    const sanitizations = sanitizers.map(async ([prop, sanitizer]) => {
      const resolvedValue = await sanitizer(ctx, lifeCycle);

      this._updateContext({ [prop]: resolvedValue } as I);
    });

    await Promise.all(sanitizations);
  };

  private _isSanitizable = (
    prop: string,
    lifeCycle: LifeCycles.LifeCycle
  ): [false, undefined] | [true, Function] => {
    const { sanitizer, shouldInit } = this._getDefinition(prop);

    if (!sanitizer) return [false, undefined];
    if (lifeCycle == "creating" && isEqual(shouldInit, false))
      return [false, undefined];

    return [true, sanitizer];
  };

  private _isUpdatable = (prop: string) => {
    const isSideEffect = this._isSideEffect(prop);

    if (
      (!this._isProp(prop) ||
        this._isConstant(prop) ||
        this._isDependentProp(prop)) &&
      !isSideEffect
    )
      return false;

    const hasShouldUpdateRule = this._hasAny(prop, "shouldUpdate");
    const isUpdatable = this._getValueBy(prop, "shouldUpdate", "updating");

    if (isSideEffect) return hasShouldUpdateRule ? isUpdatable : true;

    const isReadonly = this._isReadonly(prop);

    if (!isReadonly) return hasShouldUpdateRule ? isUpdatable : true;

    if (hasShouldUpdateRule && !isUpdatable) return false;

    return isReadonly && isEqual(this.defaults[prop], this.values[prop]);
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

  private _resolveDependentChanges = async (
    data: Partial<I>,
    ctx: Partial<I>,
    lifeCycle: LifeCycles.LifeCycle
  ) => {
    let _updates = { ...data };

    const successFulChanges = this._getKeysAsProps(ctx);

    let toResolve = [] as StringKey<I>[];

    const isCreating = lifeCycle == "creating";

    for (const prop of successFulChanges) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length) continue;

      if (isCreating && this._isSideEffect(prop) && !this._isSideInit(prop))
        continue;

      if (
        isCreating &&
        (this._isDependentProp(prop) || this._isLaxProp(prop)) &&
        isEqual(this.defaults[prop], data[prop])
      )
        continue;

      toResolve = toResolve.concat(dependencies);
    }

    toResolve = Array.from(new Set(toResolve));

    const _ctx = this._getContext();

    const operations = toResolve.map(async (prop) => {
      if (
        this._isReadonly(prop) &&
        !isCreating &&
        !isEqual(this.values[prop], this.defaults[prop])
      )
        return;

      const { resolver } = this._getDefinition(prop);

      const value = await resolver!(_ctx, lifeCycle);

      data[prop] = value;

      const updates = { [prop]: value } as I;

      this._updateContext(updates);
      this._updateFinalContext(updates);

      const _data = await this._resolveDependentChanges(
        data,
        updates,
        lifeCycle
      );

      return (_updates = { ..._updates, ..._data });
    });

    await Promise.all(operations);

    return _updates;
  };

  private _setValues(
    values: Partial<I | O>,
    {
      allowSideEffects = true,
      allowTimestamps = false,
    }: {
      allowSideEffects?: boolean;
      allowTimestamps?: boolean;
    } = {
      allowSideEffects: true,
      allowTimestamps: false,
    }
  ) {
    const keys = this._getKeysAsProps(values).filter(
      (key) =>
        (allowTimestamps &&
          this.optionsTool.withTimestamps &&
          this.optionsTool.isTimestampKey(key)) ||
        this._isProp(key) ||
        (allowSideEffects && this._isSideEffect(key))
    );

    const _values = {} as Partial<I>;

    sort(keys).forEach((key) => (_values[key] = values[key]));

    this.values = _values;

    this._initContexts();
  }

  private _useConfigProps = (obj: I | Partial<I>, isUpdate = false) => {
    if (!this.optionsTool.withTimestamps) return sortKeys(obj);

    const createdAt = this.optionsTool.getCreateKey(),
      updatedAt = this.optionsTool.getUpdateKey();

    let results = { ...obj };

    if (updatedAt) results = { ...results, [updatedAt]: new Date() };

    if (!isUpdate && createdAt)
      results = { ...results, [createdAt]: new Date() };

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

    this._setValues(values);

    const reset = toArray<StringKey<I>>(options.reset ?? []).filter(
      this._isProp
    );

    let data = {} as Partial<I>;
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

    await this._handleSanitizationOfSideEffects("creating");

    data = await this._resolveDependentChanges(
      data,
      this._getFinalContext(),
      "creating"
    );

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, sideEffects);
      return this._handleError(error);
    }

    return {
      data: this._useConfigProps(data) as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data, "creating"),
    };
  };

  create = async (values: Partial<I>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this._setValues(values);

    let data = {} as Partial<I>;
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

    await this._handleSanitizationOfSideEffects("creating");

    data = await this._resolveDependentChanges(
      data,
      this._getFinalContext(),
      "creating"
    );

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, sideEffects);
      return this._handleError(error);
    }

    return {
      data: this._useConfigProps(data) as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data, "creating"),
    };
  };

  delete = async (values: O) => {
    if (!this._areValuesOk(values))
      return new ErrorTool({ message: "Invalid Data" }).throw();

    this._setValues(values, { allowSideEffects: false, allowTimestamps: true });

    const ctx = this._getContext() as Readonly<O>;

    const cleanups = this.props.map(async (prop) => {
      const listeners = this._getListeners<O>(prop, "onDelete");

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.all(_cleanups);
    });

    await Promise.all(cleanups);
  };

  update = async (values: O, changes: Partial<I>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this._setValues(values, { allowSideEffects: false, allowTimestamps: true });

    const error = new ErrorTool({ message: "Validation Error" });
    let updated = {} as Partial<I>;

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

    await this._handleSanitizationOfSideEffects("updating");

    updated = await this._resolveDependentChanges(
      updated,
      this._getFinalContext(),
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
