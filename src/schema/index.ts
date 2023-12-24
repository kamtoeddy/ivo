import {
  makeResponse,
  getKeysAsProps,
  isEqual,
  isPropertyOf,
  sort,
  sortKeys,
  toArray,
  isObject,
  FieldKey,
  getSetValuesAsProps,
  ObjectType
} from '../utils';
import {
  Context,
  InternalValidatorResponse,
  LIFE_CYCLES,
  NS,
  RealType,
  ValidatorResponseObject,
  KeyOf,
  Summary,
  InvalidValidatorResponse
} from './types';
import {
  VALIDATION_ERRORS,
  DefaultErrorTool,
  IErrorTool,
  makeFieldError
} from './utils';
import { defaultOptions, SchemaCore } from './schema-core';

export { Model, ModelTool, Schema };

type DefaultExtendedErrorTool<
  ParentErrorTool,
  Keys extends FieldKey
> = ParentErrorTool extends DefaultErrorTool<any>
  ? DefaultErrorTool<Keys>
  : ParentErrorTool;

class Schema<
  Input extends RealType<Input>,
  Output extends RealType<Output> = Input,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<any> = DefaultErrorTool<KeyOf<Input & Aliases>>
> extends SchemaCore<Input, Output, CtxOptions, ErrorTool> {
  constructor(
    definitions: NS.Definitions<Input, Output, Aliases, CtxOptions>,
    options: NS.Options<Input, Output, ErrorTool, CtxOptions> = defaultOptions
  ) {
    super(definitions as any as NS.Definitions_<Input, Output>, options as any);
  }

  get definitions() {
    return this._definitions as any as NS.Definitions<Input, Output, Aliases>;
  }

  get options() {
    return this._options;
  }

  get reservedKeys() {
    const props = [
      ...this.props.values(),
      ...this.virtuals.values()
    ] as string[];

    const { createdAt, updatedAt } = this.timestampTool.getKeys();

    if (createdAt) props.push(createdAt);
    if (updatedAt) props.push(updatedAt);

    return sort(props);
  }

  extend<
    ExtendedInput extends RealType<ExtendedInput>,
    ExtendedOutput extends RealType<ExtendedOutput> = ExtendedInput,
    Aliases = {},
    ExtendedCtxOptions extends ObjectType = CtxOptions,
    ExtendedErrorTool extends IErrorTool<any> = DefaultExtendedErrorTool<
      ErrorTool,
      KeyOf<ExtendedInput & Aliases>
    >
  >(
    definitions: NS.Definitions<
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedCtxOptions
    >,
    options: NS.ExtensionOptions<
      Output,
      Input,
      ExtendedInput,
      ExtendedOutput,
      ExtendedErrorTool,
      ExtendedCtxOptions
    > = {}
  ) {
    const { remove = [], useParentOptions = true, ...rest } = options;

    const _definitions = { ...this.definitions } as unknown as NS.Definitions<
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedCtxOptions
    >;

    toArray(remove ?? [])?.forEach(
      (prop) => delete (_definitions as any)?.[prop]
    );

    const options_ = {} as NS.Options<
      ExtendedInput,
      ExtendedOutput,
      ExtendedErrorTool,
      ExtendedCtxOptions
    >;

    if (useParentOptions)
      getKeysAsProps(this.options)
        .filter(
          (prop) => ![...LIFE_CYCLES, 'shouldUpdate'].includes(prop as any)
        )
        .forEach((prop) => {
          options_[prop] = this.options[prop] as any;
        });

    return new Schema<
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedCtxOptions,
      ExtendedErrorTool
    >({ ..._definitions, ...definitions }, { ...options_, ...rest });
  }

  getModel() {
    return new Model(
      new ModelTool<Input, Output, Aliases, CtxOptions, ErrorTool>(this)
    );
  }
}

class ModelTool<
  Input extends RealType<Input>,
  Output extends RealType<Output> = Input,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<any> = DefaultErrorTool<KeyOf<Input & Aliases>>
> extends SchemaCore<Input, Output, CtxOptions, ErrorTool> {
  private _regeneratedProps: KeyOf<Output>[] = [];

  constructor(schema: Schema<Input, Output, Aliases, CtxOptions, ErrorTool>) {
    super(schema.definitions as any, schema.options as any);
  }

  private async _generateConstants() {
    const data = {} as Partial<Output>;

    await Promise.allSettled(
      getSetValuesAsProps(this.constants).map(async (prop) => {
        data[prop] = await this._getConstantValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);

        return this._updateContext(validCtxUpdate);
      })
    );

    return data;
  }

  private _getSummary(data: Partial<Output>, isUpdate = false) {
    const changes = isUpdate ? data : null,
      operation = isUpdate ? 'update' : 'creation',
      previousValues = isUpdate ? this._getFrozenCopy(this.values) : null,
      context = this._getContext(isUpdate ? previousValues : null),
      values = this._getFrozenCopy(
        isUpdate
          ? { ...previousValues, ...this.values, ...data }
          : { ...this.defaults, ...data }
      );

    return this._getFrozenCopy({
      changes,
      context,
      operation,
      previousValues,
      values
    }) as Summary<Input, Output, CtxOptions>;
  }

  private _getValidationSummary = (isUpdate = false) =>
    this._getSummary(this.values, isUpdate);

  private async _handleError(
    errorTool: ErrorTool,
    data?: Partial<Output>,
    virtuals: KeyOf<Output>[] = []
  ) {
    if (data) await this._handleFailure(data, errorTool, virtuals);

    if (this._options.errors === 'throw') throw errorTool.error;

    return {
      data: null,
      error: errorTool.data as ErrorTool['data'],
      handleSuccess: null
    };
  }

  private async _handleFailure(
    data: Partial<Output>,
    errorTool: ErrorTool,
    virtuals: KeyOf<Output>[] = []
  ) {
    let props = [...getKeysAsProps(data), ...errorTool.fields, ...virtuals];

    props = Array.from(new Set(props));

    const ctx = this._getContext();

    const cleanups = props.map(async (prop) => {
      const handlers = this._getHandlers<NS.FailureHandler<Input, Output>>(
        prop,
        'onFailure'
      );

      const _cleanups = handlers.map(async (handler) => await handler(ctx));

      await Promise.allSettled(_cleanups);
    });

    await Promise.allSettled(cleanups);
  }

  private _handleRequiredBy(data: Partial<Output>, isUpdate = false) {
    const summary = this._getSummary(data, isUpdate);

    const errorTool = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      summary.context.__getOptions__()
    );

    for (const prop of this.propsRequiredBy) {
      const [isRequired, message] = this._getRequiredState(prop, summary);

      if (
        (isRequired && !isUpdate) ||
        (isRequired && isUpdate && this._isUpdatable(prop, undefined))
      ) {
        const value = (data as any)[prop];

        const alias = this._getAliasByVirtual(prop);

        if (!alias) {
          errorTool.add(prop, makeFieldError(message), value);

          continue;
        }

        const _message =
          message == `'${prop}' is required`
            ? `'${alias}' is required`
            : message;

        errorTool.add(alias as any, makeFieldError(_message), value);
      }
    }

    return errorTool;
  }

  private async _handleSanitizationOfVirtuals(
    data: Partial<Output>,
    isUpdate = false
  ) {
    const sanitizers: [KeyOf<Input>, Function][] = [];

    const partialCtx = this._getPartialContext();

    const successFulVirtuals = getKeysAsProps(partialCtx).filter(
      this._isVirtual
    );

    for (const prop of successFulVirtuals) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, !isUpdate);

      if (isSanitizable) sanitizers.push([prop as KeyOf<Input>, sanitizer]);
    }

    const summary = this._getSummary(data, isUpdate);

    const sanitizations = sanitizers.map(async ([prop, sanitizer]) => {
      const resolvedValue = await sanitizer(summary);

      this._updateContext({ [prop]: resolvedValue } as any);
    });

    await Promise.allSettled(sanitizations);
  }

  private _isSanitizable(
    prop: string,
    isCreation: boolean
  ): [false, undefined] | [true, Function] {
    const { sanitizer, shouldInit } = this._getDefinition(prop);

    if (!sanitizer) return [false, undefined];

    if (isCreation && isEqual(shouldInit, false)) return [false, undefined];

    return [true, sanitizer];
  }

  private async _isGloballyUpdatable(changes: any) {
    const { shouldUpdate = defaultOptions.shouldUpdate! } = this._options;

    if (typeof shouldUpdate == 'boolean') return shouldUpdate;

    const response = await shouldUpdate(this._getSummary(changes, true));

    if (typeof response == 'boolean') return response;

    if (response?.contextOptionsUpdate)
      this._updateContextOptions(response.contextOptionsUpdate);

    return !!response?.update;
  }

  private _isUpdatable(prop: string, value: any = undefined) {
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
    ) as KeyOf<Output>;

    const hasShouldUpdateRule = this._isRuleInDefinition(
      propName,
      'shouldUpdate'
    );

    const extraCtx = isAlias ? { [propName]: value } : {};

    const isUpdatable = this._getValueBy(propName, 'shouldUpdate', extraCtx);

    if (isVirtual) return hasShouldUpdateRule ? isUpdatable : true;

    const isReadonly = this._isReadonly(propName);

    if (hasShouldUpdateRule && !isUpdatable) return false;

    if (isReadonly)
      return isEqual(
        this.defaults[propName],
        this.values[propName],
        this._options.equalityDepth
      );

    return !isEqual(this.values[propName], value, this._options.equalityDepth);
  }

  private _isValidProperty = (prop: string) => {
    if (this._isConstant(prop)) return false;

    const isAlias = this._isVirtualAlias(prop);

    if (this._isDependentProp(prop) && !isAlias) return false;

    return this._isProp(prop) || this._isVirtual(prop) || isAlias;
  };

  private _makeHandleSuccess(data: Partial<Output>, isUpdate = false) {
    const partialCtx = this._getPartialContext();

    const successProps = getKeysAsProps(partialCtx);

    let successListeners = [] as NS.SuccessHandler<Input, Output, CtxOptions>[];

    const summary = this._getSummary(data, isUpdate);

    for (const prop of successProps) {
      const handlers = this._getHandlers<NS.SuccessHandler<Input, Output>>(
        prop,
        'onSuccess'
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
  }

  private async _resolveDependentChanges(
    data: Partial<Output>,
    ctx: Partial<Context<Input, Output>>,
    isUpdate = false
  ) {
    let _updates = { ...data };
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { __getOptions__, ...context } = ctx;

    const successFulChanges = getKeysAsProps<Output>(context as any);

    let toResolve = [] as KeyOf<Output>[];

    const isCreation = !isUpdate;

    for (const prop of successFulChanges) {
      if (this._regeneratedProps.includes(prop) && !isPropertyOf(prop, data))
        continue;

      const dependencies = this._getDependencies(prop);

      if (!dependencies.length) continue;

      if (isCreation && this._isVirtual(prop) && !this._isVirtualInit(prop))
        continue;

      if (
        isCreation &&
        (this._isDependentProp(prop) || this._isLaxProp(prop)) &&
        isEqual(this.defaults[prop], data[prop], this._options.equalityDepth)
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
        !isEqual(
          this.values[prop],
          this.defaults[prop],
          this._options.equalityDepth
        )
      )
        return;

      const resolver = this._getDefinition(prop).resolver!;

      const value = await resolver(summary);

      if (
        !isCreation &&
        isEqual(
          value,
          _ctx[prop as KeyOf<Context<Input, Output>>],
          this._options.equalityDepth
        )
      )
        return;

      data[prop] = value;

      const updates = { [prop]: value } as any;

      this._updateContext(updates);
      this._updatePartialContext(updates);

      const _data = await this._resolveDependentChanges(
        data,
        updates as any,
        isUpdate
      );

      return (_updates = { ..._updates, ..._data });
    });

    await Promise.allSettled(operations);

    return _updates;
  }

  private _cleanInput(input: Partial<Input | Aliases>) {
    const props = getKeysAsProps(input).filter(this._isValidProperty);

    const values = {} as any;

    for (const prop of props) {
      values[prop] = input[prop];

      if (this._isVirtual(prop)) {
        const alias = this._getAliasByVirtual(prop);

        if (alias && values[alias]) delete values[alias];
      } else if (this._isVirtualAlias(prop)) {
        const virtual = this._getVirtualByAlias(prop);

        if (virtual && values[virtual]) delete values[virtual];
      }
    }

    return values;
  }

  private _setValues(
    values: Partial<Input | Output | Aliases>,
    {
      allowVirtuals = true,
      allowTimestamps = false
    }: {
      allowVirtuals?: boolean;
      allowTimestamps?: boolean;
    } = {
      allowVirtuals: true,
      allowTimestamps: false
    }
  ) {
    const keys = getKeysAsProps(values).filter((key) => {
      if (
        allowTimestamps &&
        this.timestampTool.withTimestamps &&
        this.timestampTool.isTimestampKey(key)
      )
        return true;

      if (allowVirtuals && this._isVirtual(key)) return true;

      return this._isProp(key);
    });

    const _values = {} as any;

    sort(keys).forEach((key) => (_values[key] = values[key]));

    this.values = _values as Output;

    this._initializeContexts();
  }

  private async _setMissingDefaults() {
    this._regeneratedProps = getSetValuesAsProps(this.props).filter((prop) => {
      return this._isDefaultable(prop) && isEqual(this.values[prop], undefined);
    });

    await Promise.allSettled(
      this._regeneratedProps.map(async (prop) => {
        const value = await this._getDefaultValue(prop);

        this._updateContext({ [prop]: value } as any);
        this._updatePartialContext({ [prop]: value } as any);
      })
    );
  }

  private async _handleInvalidValue(
    errorTool: ErrorTool,
    prop: KeyOf<Input & Output & Aliases>,
    validationResponse: InvalidValidatorResponse
  ) {
    const { otherReasons, reasons, metadata, value } = validationResponse;

    const fieldError = makeFieldError(
      reasons.length ? reasons : 'validation failed'
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.add(prop, fieldError, value);

    return (
      otherReasons &&
      Object.entries(otherReasons).forEach(([key, reasons]) => {
        errorTool.add(key, makeFieldError(reasons));
      })
    );
  }

  private async _setValidValue(
    operationData: Partial<Output> = {},
    prop: KeyOf<Output>,
    value: Output[KeyOf<Output>]
  ) {
    const isAlias = this._isVirtualAlias(prop);

    const propName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    if (!this._isVirtual(propName))
      operationData[propName as KeyOf<Output>] = value;

    const validCtxUpdate = { [propName]: value } as unknown as any;

    this._updateContext(validCtxUpdate);
    this._updatePartialContext(validCtxUpdate);
  }

  private _sanitizeValidationResponse<T>(
    response: ValidatorResponseObject<any>,
    value: any
  ): ValidatorResponseObject<T> {
    const responseType = typeof response;

    if (responseType == 'boolean')
      return response
        ? { valid: true, validated: value }
        : getValidationFailedResponse(value);

    if (!response && responseType != 'object')
      return getValidationFailedResponse(value);

    if (response?.valid) {
      const validated = isEqual(response?.validated, undefined)
        ? value
        : response.validated;

      return { valid: true, validated };
    }

    const _response: ValidatorResponseObject<T> = { valid: false, value };

    if (response?.otherReasons) {
      const validProperties = getKeysAsProps(response.otherReasons).filter(
        this._isValidProperty
      );

      const otherReasons = {} as Record<string, any>;

      for (const prop of validProperties) {
        const fieldError = response.otherReasons[prop];

        const isArray = Array.isArray(fieldError),
          isString = typeof fieldError == 'string';

        if (!isObject(fieldError) && !isArray && !isString) {
          otherReasons[prop] = 'validation failed';

          continue;
        }

        if (isArray) {
          const message = (fieldError as any[]).filter(
            (v) => typeof v == 'string'
          );

          otherReasons[prop] = message.length ? message : 'validation failed';

          continue;
        }

        if (isString) {
          const message = fieldError.trim();

          otherReasons[prop] = message.length ? message : 'validation failed';

          continue;
        }

        otherReasons[prop] = fieldError;
      }

      _response.otherReasons = otherReasons;
    }

    if (response?.reason) _response.reason = response.reason;
    if (response?.reasons) _response.reasons = response.reasons;

    if (response?.metadata && isObject(response.metadata))
      _response.metadata = sortKeys(response.metadata);
    else _response.metadata = null;

    if (!_response.reason && !_response.reasons && !_response.otherReasons) {
      if (_response.metadata)
        return {
          ...getValidationFailedResponse(value),
          metadata: _response.metadata
        } as any;

      return getValidationFailedResponse(value);
    }

    return makeResponse(_response) as ValidatorResponseObject<T>;
  }

  private _useConfigProps(obj: Partial<Output>, isUpdate = false) {
    if (!this.timestampTool.withTimestamps) return sortKeys(obj);

    const { createdAt, updatedAt } = this.timestampTool.getKeys();

    let results = { ...obj };

    if (updatedAt) results = { ...results, [updatedAt]: new Date() };

    if (!isUpdate && createdAt)
      results = { ...results, [createdAt]: new Date() };

    return sortKeys(results);
  }

  private async _validateAndSet(
    operationData: Partial<Output>,
    errorTool: ErrorTool,
    prop: KeyOf<Output>,
    value: any
  ) {
    const isValid = (await this._validate(
      prop as any,
      value,
      this._getValidationSummary(false)
    )) as InternalValidatorResponse<Output[KeyOf<Output>]>;

    if (isValid.valid)
      return this._setValidValue(operationData, prop, isValid.validated);

    this._handleInvalidValue(errorTool, prop, isValid);
  }

  private async _validate<K extends KeyOf<Input & Aliases>>(
    prop: K,
    value: any,
    summary_: Summary<Input, Output>
  ) {
    if (!this._isValidProperty(prop))
      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        value,
        reason: 'Invalid property'
      });

    const isAlias = this._isVirtualAlias(prop);

    const _prop = (isAlias ? this._getVirtualByAlias(prop) : prop)!;

    const allowedValues = this.enumeratedPropsToAllowedValuesMap.get(_prop);

    if (allowedValues && !allowedValues.has(value))
      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        value,
        reason: 'value not allowed',
        metadata: { allowed: Array.from(allowedValues) }
      });

    const validator = this._getValidator(_prop as any);

    if (validator) {
      const res = this._sanitizeValidationResponse<(Input & Aliases)[K]>(
        (await validator(value, summary_)) as ValidatorResponseObject<
          (Input & Aliases)[K]
        >,
        value
      );

      if (allowedValues && res.valid && !allowedValues.has(res.validated))
        return makeResponse<(Input & Aliases)[K]>({
          valid: true,
          validated: value
        });

      return res;
    }

    return makeResponse<(Input & Aliases)[K]>({
      valid: true,
      validated: value
    });
  }

  async create(
    input: Partial<Input & Aliases> = {},
    ctxOptions: Partial<CtxOptions> = {}
  ) {
    const ctxOpts = this._updateContextOptions(ctxOptions);

    if (!areValuesOk(input)) input = {};

    const _input = this._cleanInput(input);

    this._setValues(_input);

    let data = await this._generateConstants();

    const errorTool = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      ctxOpts
    );

    const virtuals = getKeysAsProps<Partial<Output>>(_input).filter((prop) =>
      this._isVirtualInit(prop, _input[prop as unknown as KeyOf<Input>])
    );

    const props = Array.from(
      new Set([
        ...getSetValuesAsProps(this.props).filter(
          (prop) => !this._isConstant(prop)
        ),
        ...virtuals
      ])
    );

    const validations = props.map(async (prop) => {
      const isVirtualInit = virtuals.includes(prop);

      if (this._isVirtual(prop) && !isVirtualInit) return;

      const isDependent = this._isDependentProp(prop),
        isVirtualAlias = virtuals.includes(prop);

      if (isDependent) {
        data[prop] = await this._getDefaultValue(prop);

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);

        this._updateContext(validCtxUpdate);

        if (!isVirtualAlias) return;
      }

      if (isVirtualAlias)
        return this._validateAndSet(
          data,
          errorTool,
          prop,
          _input[prop as unknown as KeyOf<Input>]
        );

      const isProvided = isPropertyOf(prop, this.values);

      const isLax = this._isLaxProp(prop);

      const isLaxInit = isLax && isProvided;

      const isRequiredInit = this._isRequiredBy(prop) && isProvided;

      if (
        (isLax &&
          this._isRuleInDefinition(prop, 'shouldInit') &&
          !this._getValueBy(prop, 'shouldInit', {})) ||
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

      return this._validateAndSet(data, errorTool, prop, this.values[prop]);
    });

    await Promise.allSettled(validations);

    if (errorTool.isLoaded) return this._handleError(errorTool, data, virtuals);

    const requiredError = this._handleRequiredBy(data);

    if (requiredError.isLoaded)
      return this._handleError(requiredError, data, virtuals);

    await this._handleSanitizationOfVirtuals(data);

    data = await this._resolveDependentChanges(
      data,
      this._getPartialContext() as any
    );

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as Output,
      error: null,
      handleSuccess: this._makeHandleSuccess(finalData)
    };
  }

  async delete(values: Output, contextOptions: Partial<CtxOptions> = {}) {
    const ctxOptions = this._updateContextOptions(contextOptions);

    if (!areValuesOk(values))
      throw new this._options.ErrorTool(
        VALIDATION_ERRORS.VALIDATION_ERROR,
        ctxOptions
      ).error;

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    let handlers: NS.DeleteHandler<Output, CtxOptions>[] = [
      ...this.globalDeleteHandlers
    ];

    const data = this._getFrozenCopy({
      ...this.values,
      __getOptions__: () => ctxOptions
    });

    getSetValuesAsProps(this.props).map(async (prop) => {
      const handlers_ = this._getHandlers<NS.DeleteHandler<Output, CtxOptions>>(
        prop,
        'onDelete'
      );

      if (handlers_.length) handlers = handlers.concat(handlers_);
    });

    const cleanups = handlers.map(async (handler) => await handler(data));

    await Promise.allSettled(cleanups);
  }

  async update(
    values: Output,
    changes: Partial<Input & Aliases>,
    ctxOptions: Partial<CtxOptions> = {}
  ) {
    const ctxOpts = this._updateContextOptions(ctxOptions);

    const errorTool = new this._options.ErrorTool(
      VALIDATION_ERRORS.NOTHING_TO_UPDATE,
      ctxOpts
    );

    if (!areValuesOk(values) || !areValuesOk(changes))
      return this._handleError(errorTool, {}, []);

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    if (this._options?.setMissingDefaultsOnUpdate)
      await this._setMissingDefaults();

    const _changes = this._cleanInput(changes);

    if (!(await this._isGloballyUpdatable(_changes)))
      return this._handleError(errorTool);

    errorTool.setMessage(VALIDATION_ERRORS.VALIDATION_ERROR);

    let updates = {} as Partial<Output>;

    const toUpdate = Array.from(
      new Set(getKeysAsProps<Output & Aliases>(_changes as any))
    ).filter((prop) => this._isUpdatable(prop, (_changes as any)[prop]));

    const linkedProps: KeyOf<Output>[] = [];
    const virtuals: KeyOf<Output>[] = [];

    const validations = toUpdate.map(async (prop) => {
      const value = (_changes as any)[prop] as Output[KeyOf<Output>];

      const isValid = (await this._validate(
        prop as any,
        value,
        this._getValidationSummary(true)
      )) as InternalValidatorResponse<Output[KeyOf<Output>]>;

      if (!isValid.valid)
        return this._handleInvalidValue(errorTool, prop, isValid);

      let { validated } = isValid;

      if (isEqual(validated, undefined)) validated = value;

      const isAlias = this._isVirtualAlias(prop);

      const propName = (isAlias
        ? this._getVirtualByAlias(prop)!
        : prop) as unknown as KeyOf<Output>;

      if (
        isEqual(validated, this.values[propName], this._options.equalityDepth)
      )
        return;

      if (this._isVirtual(propName)) virtuals.push(propName);
      else {
        updates[propName as KeyOf<Output>] = validated;
        linkedProps.push(propName);
      }

      const validCtxUpdate = { [propName]: validated } as unknown as any;

      this._updateContext(validCtxUpdate);
      this._updatePartialContext(validCtxUpdate);
    });

    await Promise.allSettled(validations);

    if (errorTool.isLoaded)
      return this._handleError(errorTool, updates, virtuals);

    const requiredErrorTool = this._handleRequiredBy(updates, true);

    if (requiredErrorTool.isLoaded)
      return this._handleError(requiredErrorTool, updates, virtuals);

    await this._handleSanitizationOfVirtuals(updates, true);

    updates = await this._resolveDependentChanges(
      updates,
      this._getPartialContext() as any,
      true
    );

    if (!Object.keys(updates).length) {
      errorTool.setMessage(VALIDATION_ERRORS.NOTHING_TO_UPDATE);

      await this._handleFailure(updates, errorTool, virtuals);

      return this._handleError(errorTool);
    }

    if (this._options?.setMissingDefaultsOnUpdate)
      this._regeneratedProps.forEach((prop) => {
        if (isEqual(updates[prop], undefined))
          updates[prop] = this.context[prop] as any;
      });

    const finalData = this._useConfigProps(updates, true);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as Partial<Output>,
      error: null,
      handleSuccess: this._makeHandleSuccess(finalData, true)
    };
  }
}

class Model<
  Input extends RealType<Input>,
  Output extends RealType<Output>,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<any> = DefaultErrorTool<KeyOf<Input & Aliases>>
> {
  constructor(
    private modelTool: ModelTool<Input, Output, Aliases, CtxOptions, ErrorTool>
  ) {}

  create = (
    values: Partial<Input & Aliases> = {},
    contextOptions: Partial<CtxOptions> = {}
  ) => this.modelTool.create(values, contextOptions);

  delete = (values: Output, contextOptions: Partial<CtxOptions> = {}) =>
    this.modelTool.delete(values, contextOptions);

  update = (
    values: Output,
    changes: Partial<Input & Aliases>,
    contextOptions: Partial<CtxOptions> = {}
  ) => this.modelTool.update(values, changes, contextOptions);
}

function areValuesOk(values: any) {
  return values && typeof values == 'object';
}

function getValidationFailedResponse(value: any) {
  return {
    metadata: null,
    reasons: ['validation failed'],
    valid: false,
    value
  } as ValidatorResponseObject<any>;
}
