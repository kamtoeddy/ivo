import { sort, sortKeys, toArray } from "../utils/functions";
import { isEqual } from "../utils/isEqual";
import {
  Context,
  ISchema as ns,
  RealType,
  ResponseInput_,
  StringKey,
  Summary,
  ValidatorResponse,
} from "./interfaces";
import { Merge } from "./merge-types";
import { defaultOptions, SchemaCore } from "./schema-core";
import { makeResponse } from "./utils";
import { ErrorTool } from "./utils/schema-error";

export { Schema };

const validationFailedResponse = {
  valid: false,
  reasons: ["validation failed"],
};

class Schema<
  I extends RealType<I>,
  O extends RealType<O> = I,
  A = {}
> extends SchemaCore<I, O> {
  constructor(
    definitions: ns.Definitions<I, O, A>,
    options: ns.Options<I, O> = defaultOptions
  ) {
    super(definitions as ns.Definitions_<I, O>, options);
  }

  get definitions() {
    return this._definitions as ns.Definitions<I, O, A>;
  }

  get options() {
    return this._options;
  }

  extend = <U extends RealType<U>, V extends RealType<V> = U, A = {}>(
    definitions: Partial<
      ns.Definitions<RealType<Merge<I, U> & U>, Merge<O, V>, A>
    >,
    options: ns.ExtensionOptions<
      StringKey<I>,
      RealType<Merge<I, U> & U>,
      Merge<O, V>
    > = {
      ...defaultOptions,
      remove: [],
    }
  ) => {
    const remove = toArray(options?.remove ?? []);
    delete options.remove;

    type InputType = RealType<Merge<I, U> & U>;
    type OutputType = Merge<O, V>;

    let _definitions = { ...this.definitions } as ns.Definitions<
      InputType,
      OutputType,
      A
    >;

    remove?.forEach(
      (prop) =>
        delete _definitions?.[
          prop as StringKey<ns.Definitions<InputType, OutputType, A>>
        ]
    );

    _definitions = { ..._definitions, ...definitions };

    return new ExtendedSchema<InputType, OutputType, A>(_definitions, options);
  };

  getModel = () => new Model(new ModelTool<I, O, A>(this));
}

class ExtendedSchema<
  I extends RealType<any>,
  O extends RealType<any> = I,
  A = {}
> extends Schema<I, O> {
  constructor(
    definitions: ns.Definitions<I, O, A>,
    options: ns.Options<I, O> = defaultOptions
  ) {
    super(definitions, options);
  }
}

class ModelTool<
  I extends RealType<I>,
  O extends RealType<O> = I,
  A = {}
> extends SchemaCore<I, O> {
  constructor(schema: Schema<I, O, A>) {
    super(schema.definitions as any, schema.options);
  }

  private _areValuesOk = (values: any) => values && typeof values == "object";

  private _getSummary = (data: Partial<O>, isUpdate = false) => {
    const context = this._getContext(),
      operation = isUpdate ? "update" : "creation",
      previousValues = isUpdate ? this._getFrozenCopy(this.values) : undefined,
      values = this._getFrozenCopy(
        isUpdate ? { ...this.values, ...data } : (data as O)
      );

    return this._getFrozenCopy({
      context,
      operation,
      previousValues,
      values,
    }) as Summary<I, O>;
  };

  private _getValidationSummary = (isUpdate = false) =>
    this._getSummary(this.values, isUpdate);

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
      const listeners = this._getListeners(prop, "onFailure") as ns.Listener<
        I,
        O
      >[];

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.allSettled(_cleanups);
    });

    await Promise.allSettled(cleanups);
  };

  private _handleInvalidData = () =>
    this._handleError(new ErrorTool({ message: "Invalid Data" }));

  private _handleRequiredBy = (
    data: Partial<O>,
    error: ErrorTool,
    isUpdate = false
  ) => {
    const summary = this._getSummary(data, isUpdate);

    for (const prop of this.propsRequiredBy) {
      const [isRequired, message] = this._getRequiredState(prop, summary);

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
    const partialCtx = this._getPartialContext();

    const successFulVirtuals = this._getKeysAsProps(partialCtx).filter(
      this._isVirtual
    );

    for (const prop of successFulVirtuals) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, lifeCycle);

      if (!isSanitizable) continue;

      sanitizers.push([prop as StringKey<I>, sanitizer]);
    }

    const sanitizations = sanitizers.map(async ([prop, sanitizer]) => {
      const resolvedValue = await sanitizer(ctx, lifeCycle);

      this._updateContext({ [prop]: resolvedValue } as any);
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
    const partialCtx = this._getPartialContext();

    const successProps = this._getKeysAsProps(partialCtx);

    let successListeners = [] as ns.SuccessListener<I, O>[];

    const summary = this._getSummary(data, isUpdate);

    for (const prop of successProps) {
      const listeners = this._getListeners(
        prop,
        "onSuccess"
      ) as ns.SuccessListener<I, O>[];

      successListeners = successListeners.concat(listeners);
    }

    successListeners = successListeners.concat(this.globalSuccessListeners);

    return async () => {
      const successOperations = successListeners.map(
        async (listener) => await listener(summary)
      );

      await Promise.allSettled(successOperations);
    };
  };

  private _resolveDependentChanges = async (
    data: Partial<O>,
    ctx: Partial<O> | Partial<Context<I, O>>,
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

      const resolver = this._getDefinition(prop).resolver!;

      const value = await resolver(_ctx, lifeCycle);

      if (!isCreating && isEqual(value, _ctx[prop as StringKey<Context<I, O>>]))
        return;

      data[prop] = value;

      const updates = { [prop]: value } as any;

      this._updateContext(updates);
      this._updatePartialContext(updates);

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

    this._initialiseContexts();
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
    value: any,
    isUpdate = false
  ) => {
    const isValid = (await this._validate(
      prop as any,
      value,
      this._getValidationSummary(isUpdate)
    )) as ValidatorResponse<O[StringKey<O>]>;

    if (!isValid.valid) return error.add(prop, isValid.reasons);

    const { validated } = isValid;

    const isAlias = this._isVirtualAlias(prop);

    const propName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    if (!this._isVirtual(propName))
      operationData[propName as StringKey<O>] = validated;

    const validCtxUpdate = { [propName]: validated } as unknown as any;

    this._updateContext(validCtxUpdate);
    this._updatePartialContext(validCtxUpdate);
  };

  private _sanitizeValidationResponse = <T>(
    response: any,
    value: any
  ): ResponseInput_<T> => {
    const responseType = typeof response;

    if (responseType == "boolean")
      return response
        ? { valid: true, validated: value }
        : validationFailedResponse;

    if (!response && (responseType != "object" || Array.isArray(response)))
      return validationFailedResponse;

    if (response?.valid) {
      const validated = isEqual(response?.validated, undefined)
        ? value
        : response.validated;

      return { valid: true, validated };
    }

    const _response: ResponseInput_<T> = { valid: false };

    if (response?.reason) _response.reason = response.reason;
    if (response?.reasons) _response.reasons = response.reasons;

    if (!_response.reason && !_response.reasons)
      return validationFailedResponse;

    return _response;
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

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }
      const isAlias = this._isVirtualAlias(prop),
        isDependent = this._isDependentProp(prop);

      if (isDependent && !isAlias) {
        const value = reset.includes(prop as any)
          ? this._getDefaultValue(prop)
          : this.values[prop as unknown as StringKey<O>];

        data[prop] = value;

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any;

        this._updatePartialContext(validCtxUpdate);
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

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any;

        this._updatePartialContext(validCtxUpdate);
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

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any;

        this._updatePartialContext(validCtxUpdate);
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

    this._handleRequiredBy(data, error);

    await this._handleSanitizationOfVirtuals("creation");

    data = await this._resolveDependentChanges(
      data,
      this._getPartialContext(),
      "creation"
    );

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, virtuals);
      return this._handleError(error);
    }

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(finalData),
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

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);
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

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.allSettled(validations);

    this._handleRequiredBy(data, error);

    await this._handleSanitizationOfVirtuals("creation");

    data = await this._resolveDependentChanges(
      data,
      this._getPartialContext(),
      "creation"
    );

    if (error.isPayloadLoaded) {
      await this._handleFailure(data, error, virtuals);
      return this._handleError(error);
    }

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as O,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(finalData),
    };
  };

  delete = async (values: O) => {
    if (!this._areValuesOk(values))
      return new ErrorTool({ message: "Invalid Data" }).throw();

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    const ctx = this._getContext();

    const cleanups = this.props.map(async (prop) => {
      const listeners = this._getListeners(prop, "onDelete") as ns.Listener<
        I,
        O
      >[];

      const _cleanups = listeners.map(async (listener) => await listener(ctx));

      await Promise.allSettled(_cleanups);
    });

    await Promise.allSettled(cleanups);
  };

  update = async (values: O, changes: Partial<Context<I, O>>) => {
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
        this._getValidationSummary(true)
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

      const validCtxUpdate = { [propName]: validated } as unknown as any;

      this._updateContext(validCtxUpdate);
      this._updatePartialContext(validCtxUpdate);
    });

    await Promise.allSettled(validations);

    this._handleRequiredBy(updated, error, true);

    if (error.isPayloadLoaded) {
      await this._handleFailure(updated as any, error, virtuals);
      return this._handleError(error);
    }

    await this._handleSanitizationOfVirtuals("update");

    updated = await this._resolveDependentChanges(
      updated,
      this._getPartialContext(),
      "update"
    );

    if (!Object.keys(updated).length) {
      await this._handleFailure(updated, error, virtuals);
      return this._handleError(error.setMessage("Nothing to update"));
    }

    const finalData = this._useConfigProps(updated, true);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as Partial<O>,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(finalData, true),
    };
  };

  _validate = async <K extends StringKey<I & A>>(
    prop: K,
    value: any,
    summary_: Summary<I, O>
  ) => {
    const isAlias = this._isVirtualAlias(prop);

    if (
      this._isConstant(prop) ||
      (this._isDependentProp(prop) && !isAlias) ||
      (!this._isProp(prop) && !this._isVirtual(prop) && !isAlias)
    )
      return makeResponse<(I & A)[K]>({
        valid: false,
        reason: "Invalid property",
      });

    const _prop = isAlias ? this._getVirtualByAlias(prop) : prop;

    const validator = this._getValidator(_prop as StringKey<I>);

    if (validator) {
      let res = (await validator(value, summary_)) as ResponseInput_<
        (I & A)[K]
      >;

      res = this._sanitizeValidationResponse<(I & A)[K]>(res, value);

      return makeResponse(res);
    }

    return makeResponse<(I & A)[K]>({ valid: true, validated: value });
  };
}

class Model<I extends RealType<I>, O extends RealType<O> = I, A = {}> {
  constructor(private modelTool: ModelTool<I, O, A>) {}

  clone = this.modelTool.clone;

  create = this.modelTool.create;

  delete = this.modelTool.delete;

  update = this.modelTool.update;

  validate = async <K extends StringKey<I & A>>(prop: K, value: any) => {
    const summary = {
      context: {},
      operation: "creation",
      previousValues: undefined,
      values: {},
    } as Summary<I, O>;

    return this.modelTool._validate(prop, value, summary);
  };
}
