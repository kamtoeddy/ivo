import { sort, sortKeys, toArray } from "../utils/functions";
import { isEqual } from "../utils/isEqual";
import {
  GetContext,
  ISchema as ns,
  RealType,
  ResponseInput,
  StringKey,
  ValidatorResponse,
} from "./interfaces";
import { Merge } from "./merge-types";
import { defaultOptions, SchemaCore } from "./SchemaCore";
import { makeResponse } from "./utils";
import { ErrorTool } from "./utils/schema-error";

export { Schema };

class Schema<I, O = I, A = {}> extends SchemaCore<I, O> {
  constructor(
    definitions: ns.Definitions<RealType<I>, RealType<O>, A>,
    options: ns.Options = defaultOptions
  ) {
    super(definitions as ns.Definitions_<I, O>, options as ns.Options);
  }

  get definitions() {
    return this._definitions;
  }

  get options() {
    return this._options;
  }

  extend = <U, V = U, A = {}>(
    propDefinitions: Partial<ns.Definitions<Merge<I, U> & U, V, A>>,
    options: ns.ExtensionOptions<StringKey<RealType<I>>> = {
      ...defaultOptions,
      remove: [],
    }
  ) => {
    const remove = toArray(options?.remove ?? []);
    delete options.remove;

    type InputType = Merge<I, U> & U;

    let _definitions = {
      ...(this.definitions as ns.Definitions<InputType, V, A>),
    } as ns.Definitions<InputType, V, A>;

    remove?.forEach(
      (prop) => delete _definitions?.[prop as StringKey<InputType>]
    );

    _definitions = {
      ..._definitions,
      ...propDefinitions,
    } as ns.Definitions<InputType, V, A>;

    return new Schema<InputType, V, A>(_definitions as any, options);
  };

  getModel = (): Model<RealType<I>, RealType<O>, A> =>
    new Model(new ModelTool<RealType<I>, RealType<O>, A>(this as any));
}

class ModelTool<I, O = I, A = {}> extends SchemaCore<I, O> {
  constructor(schema: Schema<I, O, A>) {
    super(schema.definitions, schema.options);
  }

  private _areValuesOk = (values: any) => values && typeof values == "object";

  private _handleError = (error: ErrorTool) => {
    return this._options.errors === "throw"
      ? error.throw()
      : { data: undefined, error: error.summary, handleSuccess: undefined };
  };

  private _handleFailure = async (
    data: Partial<O>,
    error: ErrorTool,
    virtuals: StringKey<O>[] = []
  ) => {
    let props = [
      ...this._getKeysAsProps({ ...data, ...error.payload }),
      ...virtuals,
    ];

    props = Array.from(new Set(props));

    const ctx = this._getContext();

    const cleanups = props.map(async (prop) => {
      const listeners = this._getListeners(prop, "onFailure");

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.allSettled(_cleanups);
    });

    await Promise.allSettled(cleanups);
  };

  private _handleInvalidData = () =>
    this._handleError(new ErrorTool({ message: "Invalid Data" }));

  private _handleRequiredBy = (
    error: ErrorTool,
    lifeCycle: ns.OperationName
  ) => {
    for (const prop of this.propsRequiredBy) {
      const [isRequired, message] = this._getRequiredState(prop, lifeCycle);

      if (isRequired && this._isUpdatable(prop)) {
        error.add(prop, message);

        const alias = this._getAliasByVirtual(prop);

        if (!alias) continue;

        const _message =
          message == `'${prop}' is required!`
            ? `'${alias}' is required!`
            : message;

        error.add(alias, _message);
      }
    }
  };

  private _handleSanitizationOfVirtuals = async (
    lifeCycle: ns.OperationName
  ) => {
    let sanitizers: [StringKey<I>, Function][] = [];

    const ctx = this._getContext();
    const finalCtx = this._getFinalContext();

    const successFulVirtuals = this._getKeysAsProps(finalCtx).filter(
      this._isVirtual
    );

    for (const prop of successFulVirtuals) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, lifeCycle);

      if (!isSanitizable) continue;

      sanitizers.push([prop as StringKey<I>, sanitizer]);
    }

    const sanitizations = sanitizers.map(async ([prop, sanitizer]) => {
      const resolvedValue = await sanitizer(ctx, lifeCycle);

      this._updateContext({ [prop]: resolvedValue } as I);
    });

    await Promise.allSettled(sanitizations);
  };

  private _isSanitizable = (
    prop: string,
    lifeCycle: ns.OperationName
  ): [false, undefined] | [true, Function] => {
    const { sanitizer, shouldInit } = this._getDefinition(prop);

    if (!sanitizer) return [false, undefined];
    if (lifeCycle == "creation" && isEqual(shouldInit, false))
      return [false, undefined];

    return [true, sanitizer];
  };

  private _isUpdatable = (prop: string, value?: any) => {
    const isAlias = this._isVirtualAlias(prop),
      isVirtual = this._isVirtual(prop);

    if (
      (!this._isProp(prop) ||
        this._isConstant(prop) ||
        this._isDependentProp(prop)) &&
      !isVirtual &&
      !isAlias
    )
      return false;

    const propName = (
      isAlias ? this._getVirtualByAlias(prop)! : prop
    ) as StringKey<O>;

    const hasShouldUpdateRule = this._isRuleInDefinition(
      propName,
      "shouldUpdate"
    );

    const extraCtx = isAlias ? { [propName]: value } : {};

    const isUpdatable = this._getValueBy(
      propName,
      "shouldUpdate",
      "update",
      extraCtx
    );

    if (isVirtual) return hasShouldUpdateRule ? isUpdatable : true;

    const isReadonly = this._isReadonly(propName);

    if (!isReadonly) return hasShouldUpdateRule ? isUpdatable : true;

    if (hasShouldUpdateRule && !isUpdatable) return false;

    return (
      isReadonly && isEqual(this.defaults[propName], this.values[propName])
    );
  };

  private _makeHandleSuccess = (data: Partial<O>, isUpdate = false) => {
    const finalCtx = this._getFinalContext();

    const successFulSideEffects = this._getKeysAsProps(finalCtx).filter(
      this._isVirtual
    );

    const props = this._getKeysAsProps(data);

    const successProps = [...props, ...successFulSideEffects];

    let successListeners = [] as (() => any)[];

    for (const prop of successProps) {
      const methods = this._getListeners(
        prop,
        "onSuccess"
      ) as ns.SuccessListener<I, O>[];

      const operationSummary = this._makeOperationSummary(data, isUpdate);

      successListeners = successListeners.concat(
        methods.map((m) => () => m(operationSummary as any))
      );
    }

    return async () => {
      const successOperations = successListeners.map(
        async (listener) => await listener()
      );

      await Promise.allSettled(successOperations);
    };
  };

  private _makeOperationSummary = (
    data: Partial<O>,
    isUpdate = false
  ): ns.GetSummary<I, O> => {
    const context = this._getContext(),
      previousValues = Object.freeze(this.values),
      values = Object.freeze(data as O);

    return isUpdate
      ? Object.freeze({ context, operation: "update", previousValues, values })
      : Object.freeze({
          context,
          operation: "creation",
          previousValues: undefined,
          values,
        });
  };

  private _resolveDependentChanges = async (
    data: Partial<O>,
    ctx: Partial<O> | Partial<GetContext<I, O>>,
    lifeCycle: ns.OperationName
  ) => {
    let _updates = { ...data };

    const successFulChanges = this._getKeysAsProps(ctx);

    let toResolve = [] as StringKey<O>[];

    const isCreating = lifeCycle == "creation";

    for (const prop of successFulChanges) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length) continue;

      if (isCreating && this._isVirtual(prop) && !this._isVirtualInit(prop))
        continue;

      if (
        isCreating &&
        (this._isDependentProp(prop) || this._isLaxProp(prop)) &&
        isEqual(this.defaults[prop], data[prop])
      )
        continue;

      toResolve = toResolve.concat(dependencies as any);
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

      if (
        !isCreating &&
        isEqual(value, _ctx[prop as StringKey<GetContext<I, O>>])
      )
        return;

      data[prop] = value;

      const updates = { [prop]: value } as I;

      this._updateContext(updates);
      this._updateFinalContext(updates);

      const _data = await this._resolveDependentChanges(
        data,
        updates as unknown as O,
        lifeCycle
      );

      return (_updates = { ..._updates, ..._data });
    });

    await Promise.allSettled(operations);

    return _updates;
  };

  private _setValues(
    values: Partial<I | O | A>,
    {
      allowVirtuals = true,
      allowTimestamps = false,
    }: {
      allowVirtuals?: boolean;
      allowTimestamps?: boolean;
    } = {
      allowVirtuals: true,
      allowTimestamps: false,
    }
  ) {
    const keys = this._getKeysAsProps(values).filter((key) => {
      if (
        allowTimestamps &&
        this.optionsTool.withTimestamps &&
        this.optionsTool.isTimestampKey(key)
      )
        return true;

      if (allowVirtuals && this._isVirtual(key)) return true;

      return this._isProp(key);
    });

    const _values = {} as any;

    sort(keys).forEach((key) => (_values[key] = values[key]));

    this.values = _values as O;

    this._initContexts();
  }

  private _useConfigProps = (obj: Partial<O>, isUpdate = false) => {
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
    operationData: Partial<O> = {},
    error: ErrorTool,
    prop: StringKey<O>,
    value: any
  ) => {
    const isValid = (await this._validate(
      prop as any,
      value,
      this._getContext()
    )) as ValidatorResponse<O[StringKey<O>]>;

    if (!isValid.valid) return error.add(prop, isValid.reasons);

    const { validated } = isValid;

    const isAlias = this._isVirtualAlias(prop);

    const propName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    if (!this._isVirtual(propName))
      operationData[propName as StringKey<O>] = validated;

    const validCtxUpdate = { [propName]: validated } as unknown as I;

    this._updateContext(validCtxUpdate);
    this._updateFinalContext(validCtxUpdate);
  };

  clone = async (
    values: Partial<I & A>,
    options: ns.CloneOptions<I> = { reset: [] }
  ) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this._setValues(values);

    const reset = toArray<StringKey<I>>(options.reset ?? []).filter(
      this._isProp
    );

    let data = {} as Partial<O>;
    const error = new ErrorTool({ message: "Validation Error" });

    const virtuals = this._getKeysAsProps<Partial<O>>(values as any).filter(
      (prop) =>
        this._isVirtualInit(prop, values[prop as unknown as StringKey<I>])
    );

    const props = [...this.props, ...virtuals];

    const validations = props.map(async (prop) => {
      if (this._isConstant(prop)) {
        data[prop] = await this._getConstantValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }
      const isAlias = this._isVirtualAlias(prop),
        isDependent = this._isDependentProp(prop);

      if (isDependent && !isAlias) {
        const value = reset.includes(prop as any)
          ? this._getDefaultValue(prop)
          : this.values[prop as unknown as StringKey<O>];

        data[prop] = value;

        const validCtxUpdate = { [prop]: data[prop] } as unknown as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      const isVirtualInit = virtuals.includes(prop);

      if (this._isVirtual(prop) && !isVirtualInit) return;

      if (isAlias && !isDependent)
        return this._validateAndSet(
          data,
          error,
          prop,
          values[prop as unknown as StringKey<I>]
        );

      if (reset.includes(prop as any)) {
        data[prop] = this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] } as unknown as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      const isLax = this._isLaxProp(prop);

      const isProvided = Object(this.values).hasOwnProperty(prop);

      const isLaxInit =
        isLax &&
        isProvided &&
        !isEqual(
          this.values[prop as unknown as StringKey<O>],
          this.defaults[prop as unknown as StringKey<O>]
        );

      const isRequiredInit =
        this._isRequiredBy(prop) && Object(this.values).hasOwnProperty(prop);

      if (
        (isLax &&
          this._isRuleInDefinition(prop, "shouldInit") &&
          !this._getValueBy(prop, "shouldInit", "creation")) ||
        (!isVirtualInit &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
      ) {
        data[prop] = this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] } as unknown as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(
        data,
        error,
        prop,
        this.values[prop as unknown as StringKey<O>]
      );
    });

    await Promise.allSettled(validations);

    this._handleRequiredBy(error, "creation");

    await this._handleSanitizationOfVirtuals("creation");

    data = await this._resolveDependentChanges(
      data,
      this._getFinalContext(),
      "creation"
    );

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, virtuals);
      return this._handleError(error);
    }

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as I);
    this._updateFinalContext(finalData as I);

    return {
      data: finalData as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data),
    };
  };

  create = async (values: Partial<I & A>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this._setValues(values);

    let data = {} as Partial<O>;
    const error = new ErrorTool({ message: "Validation Error" });

    const virtuals = this._getKeysAsProps<Partial<O>>(values as any).filter(
      (prop) =>
        this._isVirtualInit(prop, values[prop as unknown as StringKey<I>])
    );

    const props = [...this.props, ...virtuals];

    const validations = props.map(async (prop) => {
      if (this._isConstant(prop)) {
        data[prop] = await this._getConstantValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      const isVirtualInit = virtuals.includes(prop);

      if (this._isVirtual(prop) && !isVirtualInit) return;

      if (this._isVirtualAlias(prop) && !this._isDependentProp(prop))
        return this._validateAndSet(
          data,
          error,
          prop,
          values[prop as unknown as StringKey<I>]
        );

      const isProvided = Object(this.values).hasOwnProperty(prop);

      const isLax = this._isLaxProp(prop);

      const isLaxInit = isLax && isProvided;

      const isRequiredInit = this._isRequiredBy(prop) && isProvided;

      if (
        (isLax &&
          this._isRuleInDefinition(prop, "shouldInit") &&
          !this._getValueBy(prop, "shouldInit", "creation")) ||
        (!isVirtualInit &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
      ) {
        data[prop] = this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as I;

        this._updateFinalContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.allSettled(validations);

    this._handleRequiredBy(error, "creation");

    await this._handleSanitizationOfVirtuals("creation");

    data = await this._resolveDependentChanges(
      data,
      this._getFinalContext(),
      "creation"
    );

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, virtuals);
      return this._handleError(error);
    }

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as I);
    this._updateFinalContext(finalData as I);

    return {
      data: finalData as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(data),
    };
  };

  delete = async (values: O) => {
    if (!this._areValuesOk(values))
      return new ErrorTool({ message: "Invalid Data" }).throw();

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    const ctx = this._getContext() as Readonly<O>;

    const cleanups = this.props.map(async (prop) => {
      const listeners = this._getListeners<O>(prop, "onDelete");

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.allSettled(_cleanups);
    });

    await Promise.allSettled(cleanups);
  };

  update = async (values: O, changes: Partial<GetContext<I, O>>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    const error = new ErrorTool({ message: "Validation Error" });
    let updated = {} as Partial<O>;

    const toUpdate = this._getKeysAsProps(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop, changes[prop])
    );

    const linkedProps: StringKey<O>[] = [];
    const virtuals: StringKey<O>[] = [];

    const validations = toUpdate.map(async (prop) => {
      const value = changes[prop] as unknown as O[StringKey<O>];
      const isValid = (await this._validate(
        prop as any,
        value,
        this._getContext()
      )) as ValidatorResponse<O[StringKey<O>]>;

      if (!isValid.valid) return error.add(prop, isValid.reasons);

      let { validated } = isValid;

      if (isEqual(validated, undefined)) validated = value;

      const isAlias = this._isVirtualAlias(prop);

      const propName = (
        isAlias ? this._getVirtualByAlias(prop)! : prop
      ) as StringKey<O>;

      if (isEqual(validated, this.values[propName])) return;

      if (this._isVirtual(propName)) virtuals.push(propName);
      else {
        updated[propName as StringKey<O>] = validated;
        linkedProps.push(propName);
      }

      const validCtxUpdate = { [propName]: validated } as unknown as I;

      this._updateContext(validCtxUpdate);
      this._updateFinalContext(validCtxUpdate);
    });

    await Promise.allSettled(validations);

    this._handleRequiredBy(error, "update");

    if (error.isPayloadLoaded) {
      await this._handleFailure(updated as any, error, virtuals);
      return this._handleError(error);
    }

    await this._handleSanitizationOfVirtuals("update");

    updated = await this._resolveDependentChanges(
      updated,
      this._getFinalContext(),
      "update"
    );

    if (!Object.keys(updated).length) {
      await this._handleFailure(updated, error, virtuals);
      return this._handleError(error.setMessage("Nothing to update"));
    }

    const finalData = this._useConfigProps(updated, true);

    this._updateContext(finalData as I);
    this._updateFinalContext(finalData as I);

    return {
      data: finalData as Partial<O>,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(updated, true),
    };
  };

  _validate = async <K extends StringKey<I>>(
    prop: K,
    value: any,
    ctx: GetContext<I, O>
  ) => {
    const isAlias = this._isVirtualAlias(prop);

    if (
      this._isConstant(prop) ||
      (this._isDependentProp(prop) && !isAlias) ||
      (!this._isProp(prop) && !this._isVirtual(prop) && !isAlias)
    )
      return makeResponse<I[K]>({
        valid: false,
        reason: "Invalid property",
      });

    const _prop = isAlias ? this._getVirtualByAlias(prop) : prop;

    const validator = this._getValidator(_prop as StringKey<I>);

    if (validator) {
      const res = await validator(value, ctx);

      if (res.valid && isEqual(res.validated, undefined)) res.validated = value;

      return makeResponse<I[K]>(res as ResponseInput<I[K]>);
    }

    return makeResponse<I[K]>({ valid: true, validated: value });
  };
}

class Model<I, O = I, A = {}> {
  constructor(private modelTool: ModelTool<I, O, A>) {}

  clone = this.modelTool.clone;

  create = this.modelTool.create;

  delete = this.modelTool.delete;

  update = this.modelTool.update;

  validate = async <K extends StringKey<GetContext<I, O>> | StringKey<A>>(
    prop: K,
    value: any
  ) =>
    this.modelTool._validate(
      prop as unknown as StringKey<I>,
      value,
      {} as GetContext<I, O>
    );
}
