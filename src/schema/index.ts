import {
  getKeysAsProps,
  isObject,
  isPropertyOn,
  sort,
  sortKeys,
  toArray,
} from "../utils/functions";
import { isEqual } from "../utils/isEqual";
import {
  Context,
  InternalValidatorResponse,
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

  extend = <
    U extends RealType<Merge<U, I>>,
    T = O,
    A = {},
    V extends RealType<Merge<T, O>> = RealType<Merge<T, O>>
  >(
    definitions: Partial<ns.Definitions<U, T, A>>,
    options: ns.ExtensionOptions<I, O, U, T> = {
      ...defaultOptions,
      remove: [],
    }
  ) => {
    const remove = toArray(options?.remove ?? []);
    delete options.remove;

    type Input = RealType<U>;
    type Output = RealType<V>;

    let _definitions = { ...this.definitions } as ns.Definitions<
      RealType<Input>,
      RealType<Output>,
      A
    >;

    remove?.forEach(
      (prop) =>
        delete _definitions?.[
          prop as unknown as StringKey<ns.Definitions<U, V, A>>
        ]
    );

    _definitions = { ..._definitions, ...definitions };

    return new Schema<RealType<Input>, RealType<Output>, A>(
      _definitions,
      options as ns.Options<RealType<Input>, RealType<Output>>
    );
  };

  getArchivedSchema = <T extends RealType<T> = O>(
    options?: ns.ArchivedOptions<T>
  ): ArchivedSchema<T, I, O> =>
    new ArchivedSchema<T, I, O>(this as Schema<I, O>, options);

  getModel = () => new Model(new ModelTool<I, O, A>(this));
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

  private _handleError = async (
    error: ErrorTool,
    data?: Partial<O>,
    virtuals: StringKey<O>[] = []
  ) => {
    if (data) await this._handleFailure(data, error, virtuals);

    return this._options.errors === "throw"
      ? error.throw()
      : { data: undefined, error: error.summary, handleSuccess: undefined };
  };

  private _handleFailure = async (
    data: Partial<O>,
    error: ErrorTool,
    virtuals: StringKey<O>[] = []
  ) => {
    let props = [...getKeysAsProps({ ...data, ...error.payload }), ...virtuals];

    props = Array.from(new Set(props));

    const ctx = this._getContext();

    const cleanups = props.map(async (prop) => {
      const handlers = this._getHandlers<ns.FailureHandler<I, O>>(
        prop,
        "onFailure"
      );

      const _cleanups = handlers.map(async (handler) => await handler(ctx));

      await Promise.allSettled(_cleanups);
    });

    await Promise.allSettled(cleanups);
  };

  private _handleInvalidData = () =>
    this._handleError(new ErrorTool({ message: "Invalid Data" }));

  private _handleRequiredBy = (data: Partial<O>, isUpdate = false) => {
    const error = new ErrorTool({ message: "Validation Error" });
    const summary = this._getSummary(data, isUpdate);

    for (const prop of this.propsRequiredBy) {
      const [isRequired, message] = this._getRequiredState(prop, summary);

      if (
        (isRequired && !isUpdate) ||
        (isRequired && isUpdate && this._isUpdatable(prop))
      ) {
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

    return error;
  };

  private _handleSanitizationOfVirtuals = async (
    data: Partial<O>,
    isUpdate = false
  ) => {
    const sanitizers: [StringKey<I>, Function][] = [];

    const partialCtx = this._getPartialContext();

    const successFulVirtuals = getKeysAsProps(partialCtx).filter(
      this._isVirtual
    );

    for (const prop of successFulVirtuals) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, !isUpdate);

      if (isSanitizable) sanitizers.push([prop as StringKey<I>, sanitizer]);
    }

    const summary = this._getSummary(data, isUpdate);

    const sanitizations = sanitizers.map(async ([prop, sanitizer]) => {
      const resolvedValue = await sanitizer(summary);

      this._updateContext({ [prop]: resolvedValue } as any);
    });

    await Promise.allSettled(sanitizations);
  };

  private _isSanitizable = (
    prop: string,
    isCreation: boolean
  ): [false, undefined] | [true, Function] => {
    const { sanitizer, shouldInit } = this._getDefinition(prop);

    if (!sanitizer) return [false, undefined];

    if (isCreation && isEqual(shouldInit, false)) return [false, undefined];

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

    const isUpdatable = this._getValueBy(propName, "shouldUpdate", extraCtx);

    if (isVirtual) return hasShouldUpdateRule ? isUpdatable : true;

    const isReadonly = this._isReadonly(propName);

    if (!isReadonly) return hasShouldUpdateRule ? isUpdatable : true;

    if (hasShouldUpdateRule && !isUpdatable) return false;

    return (
      isReadonly && isEqual(this.defaults[propName], this.values[propName])
    );
  };

  private _isValidProperty = (prop: string) => {
    if (this._isConstant(prop)) return false;

    const isAlias = this._isVirtualAlias(prop);

    if (this._isDependentProp(prop) && !isAlias) return false;

    return this._isProp(prop) || this._isVirtual(prop) || isAlias;
  };

  private _makeHandleSuccess = (data: Partial<O>, isUpdate = false) => {
    const partialCtx = this._getPartialContext();

    const successProps = getKeysAsProps(partialCtx);

    let successListeners = [] as ns.SuccessHandler<I, O>[];

    const summary = this._getSummary(data, isUpdate);

    for (const prop of successProps) {
      const handlers = this._getHandlers<ns.SuccessHandler<I, O>>(
        prop,
        "onSuccess"
      );

      successListeners = successListeners.concat(handlers);
    }

    successListeners = successListeners.concat(this.globalSuccessHandlers);

    return async () => {
      const successOperations = successListeners.map(
        async (handler) => await handler(summary)
      );

      await Promise.allSettled(successOperations);
    };
  };

  private _resolveDependentChanges = async (
    data: Partial<O>,
    ctx: Partial<O> | Partial<Context<I, O>>,
    isUpdate = false
  ) => {
    let _updates = { ...data };

    const successFulChanges = getKeysAsProps(ctx);

    let toResolve = [] as StringKey<O>[];

    const isCreation = !isUpdate;

    for (const prop of successFulChanges) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length) continue;

      if (isCreation && this._isVirtual(prop) && !this._isVirtualInit(prop))
        continue;

      if (
        isCreation &&
        (this._isDependentProp(prop) || this._isLaxProp(prop)) &&
        isEqual(this.defaults[prop], data[prop])
      )
        continue;

      toResolve = toResolve.concat(dependencies as any);
    }

    toResolve = Array.from(new Set(toResolve));

    const values = isUpdate ? data : { ...this.values, ...data };

    const _ctx = this._getContext(),
      summary = this._getSummary(values, isUpdate);

    const operations = toResolve.map(async (prop) => {
      if (
        this._isReadonly(prop) &&
        !isCreation &&
        !isEqual(this.values[prop], this.defaults[prop])
      )
        return;

      const resolver = this._getDefinition(prop).resolver!;

      const value = await resolver(summary);

      if (!isCreation && isEqual(value, _ctx[prop as StringKey<Context<I, O>>]))
        return;

      data[prop] = value;

      const updates = { [prop]: value } as any;

      this._updateContext(updates);
      this._updatePartialContext(updates);

      const _data = await this._resolveDependentChanges(
        data,
        updates as unknown as O,
        isUpdate
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
    const keys = getKeysAsProps(values).filter((key) => {
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

    this._initializeContexts();
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
    const isValid = (await this._validateInternally(
      prop as any,
      value,
      this._getValidationSummary(isUpdate)
    )) as InternalValidatorResponse<O[StringKey<O>]>;

    if (!isValid.valid) {
      const { otherReasons, reasons } = isValid;

      const hasOtherReasons = !!otherReasons;

      if (!hasOtherReasons) return error.add(prop, reasons);

      if (reasons.length) error.add(prop, reasons);
      else error.add(prop, "validation failed");

      return Object.entries(otherReasons).forEach(([key, reasons]) => {
        error.add(key, reasons);
      });
    }

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
  ): ResponseInput_<any, any, T> => {
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

    const _response: ResponseInput_<any, any, T> = { valid: false };

    if (response?.otherReasons) {
      const validProperties = getKeysAsProps(response.otherReasons).filter(
        this._isValidProperty
      );

      const otherReasons = {} as Record<string, any>;

      for (const prop of validProperties)
        otherReasons[prop] = toArray(response.otherReasons[prop]).map(
          (value) => {
            return typeof value === "string" ? value : "validation failed";
          }
        );

      _response.otherReasons = otherReasons;
    }

    if (response?.reason) _response.reason = response.reason;
    if (response?.reasons) _response.reasons = response.reasons;

    if (!_response.reason && !_response.reasons && !_response.otherReasons)
      return validationFailedResponse;

    return makeResponse(_response);
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
    const validationError = new ErrorTool({ message: "Validation Error" });

    const virtuals = getKeysAsProps<Partial<O>>(values as any).filter((prop) =>
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
          ? await this._getDefaultValue(prop)
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
          validationError,
          prop,
          values[prop as unknown as StringKey<I>]
        );

      if (reset.includes(prop as any)) {
        data[prop] = await this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any;

        this._updatePartialContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      const isLax = this._isLaxProp(prop);

      const isProvided = isPropertyOn(prop, this.values);

      const isLaxInit =
        isLax &&
        isProvided &&
        !isEqual(
          this.values[prop as unknown as StringKey<O>],
          this.defaults[prop as unknown as StringKey<O>]
        );

      const isRequiredInit =
        this._isRequiredBy(prop) && isPropertyOn(prop, this.values);

      if (
        (isLax &&
          this._isRuleInDefinition(prop, "shouldInit") &&
          !this._getValueBy(prop, "shouldInit")) ||
        (!isVirtualInit &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
      ) {
        data[prop] = await this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any;

        this._updatePartialContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(
        data,
        validationError,
        prop,
        this.values[prop as unknown as StringKey<O>]
      );
    });

    await Promise.allSettled(validations);

    if (validationError.isPayloadLoaded)
      return this._handleError(validationError, data, virtuals);

    const requiredError = this._handleRequiredBy(data);

    if (requiredError.isPayloadLoaded)
      return this._handleError(requiredError, data, virtuals);

    await this._handleSanitizationOfVirtuals(data);

    data = await this._resolveDependentChanges(data, this._getPartialContext());

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
    const validationError = new ErrorTool({ message: "Validation Error" });

    const virtuals = getKeysAsProps<Partial<O>>(values as any).filter((prop) =>
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
          validationError,
          prop,
          values[prop as unknown as StringKey<I>]
        );

      const isProvided = isPropertyOn(prop, this.values);

      const isLax = this._isLaxProp(prop);

      const isLaxInit = isLax && isProvided;

      const isRequiredInit = this._isRequiredBy(prop) && isProvided;

      if (
        (isLax &&
          this._isRuleInDefinition(prop, "shouldInit") &&
          !this._getValueBy(prop, "shouldInit")) ||
        (!isVirtualInit &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
      ) {
        data[prop] = await this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);
        return this._updateContext(validCtxUpdate);
      }

      return this._validateAndSet(
        data,
        validationError,
        prop,
        this.values[prop]
      );
    });

    await Promise.allSettled(validations);

    if (validationError.isPayloadLoaded)
      return this._handleError(validationError, data, virtuals);

    const requiredError = this._handleRequiredBy(data);

    if (requiredError.isPayloadLoaded)
      return this._handleError(requiredError, data, virtuals);

    await this._handleSanitizationOfVirtuals(data);

    data = await this._resolveDependentChanges(data, this._getPartialContext());

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

    let handlers: ns.Handler<O>[] = [...this.globalDeleteHandlers];

    const data = this._getFrozenCopy(this.values);

    this.props.map(async (prop) => {
      const handlers_ = this._getHandlers<ns.Handler<O>>(prop, "onDelete");

      if (handlers_.length) handlers = handlers.concat(handlers_);
    });

    const cleanups = handlers.map(async (handler) => await handler(data));

    await Promise.allSettled(cleanups);
  };

  update = async (values: O, changes: Partial<I & A>) => {
    if (!this._areValuesOk(values)) return this._handleInvalidData();

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    const validationError = new ErrorTool({ message: "Validation Error" });
    let updates = {} as Partial<O>;

    const toUpdate = getKeysAsProps(changes ?? {}).filter((prop) =>
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

      if (!isValid.valid) return validationError.add(prop, isValid.reasons);

      let { validated } = isValid;

      if (isEqual(validated, undefined)) validated = value;

      const isAlias = this._isVirtualAlias(prop);

      const propName = (isAlias
        ? this._getVirtualByAlias(prop)!
        : prop) as unknown as StringKey<O>;

      if (isEqual(validated, this.values[propName])) return;

      if (this._isVirtual(propName)) virtuals.push(propName);
      else {
        updates[propName as StringKey<O>] = validated;
        linkedProps.push(propName);
      }

      const validCtxUpdate = { [propName]: validated } as unknown as any;

      this._updateContext(validCtxUpdate);
      this._updatePartialContext(validCtxUpdate);
    });

    await Promise.allSettled(validations);

    if (validationError.isPayloadLoaded)
      return this._handleError(validationError, updates, virtuals);

    const requiredError = this._handleRequiredBy(updates, true);

    if (requiredError.isPayloadLoaded)
      return this._handleError(requiredError, updates, virtuals);

    await this._handleSanitizationOfVirtuals(updates, true);

    updates = await this._resolveDependentChanges(
      updates,
      this._getPartialContext(),
      true
    );

    if (!Object.keys(updates).length) {
      await this._handleFailure(updates, validationError, virtuals);
      return this._handleError(validationError.setMessage("Nothing to update"));
    }

    const finalData = this._useConfigProps(updates, true);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as Partial<O>,
      error: undefined,
      handleSuccess: this._makeHandleSuccess(finalData, true),
    };
  };

  _validateInternally = async <K extends StringKey<I & A>>(
    prop: K,
    value: any,
    summary_: Summary<I, O>
  ) => {
    if (!this._isValidProperty(prop))
      return makeResponse<(I & A)[K]>({
        valid: false,
        reason: "Invalid property",
      });

    const isAlias = this._isVirtualAlias(prop);

    const _prop = isAlias ? this._getVirtualByAlias(prop) : prop;

    const validator = this._getValidator(_prop as StringKey<I>);

    if (validator) {
      const res = (await validator(value, summary_)) as ResponseInput_<
        any,
        I,
        (I & A)[K]
      >;

      return this._sanitizeValidationResponse<(I & A)[K]>(res, value);
    }

    return makeResponse<(I & A)[K]>({ valid: true, validated: value });
  };

  _validate = async <K extends StringKey<I & A>>(
    prop: K,
    value: any,
    summary_: Summary<I, O>
  ) => {
    const res = await this._validateInternally(prop, value, summary_);

    return makeResponse<(I & A)[K]>(res);
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

class ArchivedSchema<
  O extends RealType<O>,
  Ip extends RealType<Ip>,
  Op extends RealType<Op>
> {
  props: StringKey<O>[] = [];
  private _options: ns.ArchivedOptions<O> = {};
  // private createdAtKey: StringKey<O>;

  constructor(parentSchema: Schema<Ip, Op>, options?: ns.ArchivedOptions<O>) {
    this._validateOptions(parentSchema, options);

    this._setProperties(parentSchema);
  }

  get options() {
    return this._options;
  }

  private _setProperties(parentSchema: Schema<Ip, Op>) {
    const parentProps = getKeysAsProps(parentSchema.definitions);

    for (const prop of parentProps) {
      const definition = parentSchema.definitions[prop];

      if (isPropertyOn("virtual", definition)) continue;

      this.props.push(prop as unknown as StringKey<O>);
    }
  }

  private _validateOptions(
    parentSchema: Schema<Ip, Op>,
    options?: ns.ArchivedOptions<O>
  ) {
    const error = new ErrorTool({ message: "Invalid Schema", statusCode: 500 });

    if (!isEqual(options, undefined) && !isObject(options))
      error.add("options", "expected an object");

    if (error.isPayloadLoaded) error.throw();

    if (parentSchema || options) return;
  }
}
