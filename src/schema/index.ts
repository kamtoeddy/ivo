import {
  makeResponse,
  getKeysAsProps,
  isEqual,
  isPropertyOf,
  sort,
  sortKeys,
  toArray,
  isRecordLike,
  FieldKey,
  getSetValuesAsProps,
  ObjectType,
  isFunctionLike,
  isNullOrUndefined,
} from '../utils';
import {
  ImmutableContext,
  InternalValidatorResponse,
  LIFE_CYCLES,
  NS,
  RealType,
  KeyOf,
  ImmutableSummary,
  InvalidValidatorResponse,
  ValidatorResponseObject,
  Validator,
  MutableSummary,
  PartialContext,
  PostValidator,
} from './types';
import {
  VALIDATION_ERRORS,
  type DefaultErrorTool,
  IErrorTool,
  makeFieldError,
  isInputFieldError,
  InputFieldError,
} from './utils';
import { defaultOptions, SchemaCore } from './schema-core';

export { Model, ModelTool, Schema };

type DefaultExtendedErrorTool<
  ParentErrorTool,
  Keys extends FieldKey,
> = ParentErrorTool extends DefaultErrorTool<any>
  ? DefaultErrorTool<Keys>
  : ParentErrorTool;

const NotAllowedError = 'value not allowed';
const validationFailedFieldError = makeFieldError('validation failed');

class Schema<
  Input extends RealType<Input>,
  Output extends RealType<Output> = Input,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<any> = DefaultErrorTool<KeyOf<Input & Aliases>>,
> extends SchemaCore<Input, Output, CtxOptions, ErrorTool> {
  constructor(
    definitions: NS.Definitions<Input, Output, Aliases, CtxOptions>,
    options: NS.Options<
      Input,
      Output,
      Aliases,
      ErrorTool,
      CtxOptions
    > = defaultOptions,
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
      ...this.virtuals.values(),
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
    // @ts-ignore
    ExtendedErrorTool extends IErrorTool<any> = DefaultExtendedErrorTool<
      ErrorTool,
      KeyOf<ExtendedInput & Aliases>
    >,
  >(
    definitions: Partial<
      NS.Definitions<ExtendedInput, ExtendedOutput, Aliases, ExtendedCtxOptions>
    >,
    options: NS.ExtensionOptions<
      Input,
      Output,
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedErrorTool,
      ExtendedCtxOptions
    > = {},
  ) {
    const { remove = [], useParentOptions = true, ...rest } = options;

    const _definitions = { ...this.definitions } as unknown as NS.Definitions<
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedCtxOptions
    >;

    toArray(remove ?? [])?.forEach(
      (prop) => delete (_definitions as any)?.[prop],
    );

    const options_ = {} as NS.Options<
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedErrorTool,
      ExtendedCtxOptions
    >;

    if (useParentOptions)
      getKeysAsProps(this.options)
        .filter(
          (prop) => ![...LIFE_CYCLES, 'shouldUpdate'].includes(prop as any),
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
      new ModelTool<Input, Output, Aliases, CtxOptions, ErrorTool>(this),
    );
  }
}

class ModelTool<
  Input extends RealType<Input>,
  Output extends RealType<Output>,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<any> = DefaultErrorTool<KeyOf<Input & Aliases>>,
> extends SchemaCore<Input, Output, CtxOptions, ErrorTool> {
  private _regeneratedProps: KeyOf<Output>[] = [];
  private inputValues: Partial<RealType<Input>> = {};

  constructor(schema: Schema<Input, Output, Aliases, CtxOptions, ErrorTool>) {
    super(schema.definitions as any, schema.options as any);
  }

  private _cleanInput(input: Partial<Input | Aliases>) {
    const props = getKeysAsProps(input).filter(this._isInputOrAlias);

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

    this.inputValues = values;

    return values;
  }

  private async _generateConstants() {
    const data = {} as Partial<Output>;

    await Promise.allSettled(
      getSetValuesAsProps(this.constants).map(async (prop) => {
        try {
          data[prop] = await this._getConstantValue(prop);
        } catch (_) {
          data[prop] = null as any;
        }

        const validCtxUpdate = { [prop]: data[prop] as any } as any;

        this._updatePartialContext(validCtxUpdate);

        return this._updateContext(validCtxUpdate);
      }),
    );

    return data;
  }

  private _getSummary(data: Partial<Output>, isUpdate = false) {
    const changes = isUpdate ? data : null,
      previousValues = isUpdate ? this._getFrozenCopy(this.values) : null,
      context = this._getContext(isUpdate ? previousValues : null),
      values = this._getFrozenCopy(
        isUpdate
          ? { ...previousValues, ...this.values, ...data }
          : { ...this.defaults, ...data },
      );

    return this._getFrozenCopy({
      changes,
      context,
      inputValues: this.inputValues,
      isUpdate,
      previousValues,
      values,
    }) as ImmutableSummary<Input, Output, CtxOptions>;
  }

  private _getMutableSummary(data: Partial<Output>, isUpdate = false) {
    const summary = this._getSummary(data, isUpdate);

    return this._getFrozenCopy({
      ...summary,
      context: {
        ...this._getContext(summary.previousValues),
        __updateOptions__: this._updateContextOptions,
      },
    }) as MutableSummary<Input, Output, CtxOptions>;
  }

  private _getValidationSummary = (isUpdate: boolean) =>
    this._getMutableSummary(this.values, isUpdate);

  private _getPrimaryValidator = <K extends keyof (Output | Input)>(
    prop: string,
  ) => {
    const { validator } = this._getDefinition(prop as any);

    return (Array.isArray(validator) ? validator[0] : validator) as
      | Validator<K, Input, Output>
      | undefined;
  };

  private _getSecondaryValidator = <K extends keyof (Output | Input)>(
    prop: string,
  ) => {
    const { validator } = this._getDefinition(prop as any);

    return (Array.isArray(validator) ? validator[1] : undefined) as
      | Validator<K, Input, Output>
      | undefined;
  };

  private _getNotAllowedError(prop: string, value: any) {
    const allow = this._getDefinition(prop as any)?.allow;

    if (Array.isArray(allow)) return NotAllowedError;

    const error = (allow as any).error;

    if (Array.isArray(error) || isInputFieldError(error)) return error;

    if (isFunctionLike(error)) {
      let message;

      try {
        message = error(value, allow?.values);
      } finally {
        if (!message) return NotAllowedError;

        if (typeof message == 'string') return message || NotAllowedError;

        if (Array.isArray(message)) return message;

        return isInputFieldError(message) ? message : NotAllowedError;
      }
    }

    return error || NotAllowedError;
  }

  private async _handleError(
    data: Partial<Output>,
    errorTool: ErrorTool,
    virtuals: KeyOf<Output>[],
  ) {
    return {
      data: null,
      error: errorTool.data as ErrorTool['data'],
      handleFailure: this._makeHandleFailure(data, errorTool, virtuals),
      handleSuccess: null,
    };
  }

  private async _handleInvalidValue(
    errorTool: ErrorTool,
    prop: KeyOf<Input & Output & Aliases>,
    validationResponse: InvalidValidatorResponse,
  ) {
    const { reason, metadata, value } = validationResponse;

    if (isRecordLike(reason)) {
      if (metadata) errorTool.add(prop, { metadata, reasons: [] }, value);

      return Object.entries(reason).forEach(([key, message]) => {
        errorTool.add(key, makeFieldError(message));
      });
    }

    const fieldError = makeFieldError(
      // @ts-ignore
      reason?.length ? reason : 'validation failed',
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.add(prop, fieldError, value);
  }

  private async _handleCreationPrimaryValidations(
    data: Partial<Output>,
    input: any,
  ) {
    const error = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      this.contextOptions,
    );

    const virtuals = getKeysAsProps<Partial<Output>>(input).filter((prop) =>
      this._isVirtualInit(prop, input[prop as unknown as KeyOf<Input>]),
    );

    const props = Array.from(
      new Set([
        ...getSetValuesAsProps(this.props).filter(
          (prop) => !this._isConstant(prop),
        ),
        ...virtuals,
      ]),
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
          error,
          prop,
          input[prop as unknown as KeyOf<Input>],
        );

      const isProvided = isPropertyOf(prop, this.values),
        isLax = this._isLaxProp(prop),
        isLaxInit = isLax && isProvided,
        isRequiredInit = this._isRequiredBy(prop) && isProvided;

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

      return this._validateAndSet(data, error, prop, this.values[prop]);
    });

    await Promise.allSettled(validations);

    return { data, error, virtuals };
  }

  private async _handleUpdatePrimaryValidations(
    changes: Partial<Input & Aliases>,
  ) {
    const error = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      this.contextOptions,
    );

    const updates = {} as Partial<Output>;
    const virtuals: KeyOf<Output>[] = [];

    const toUpdate = Array.from(
      new Set(getKeysAsProps<Output & Aliases>(changes as any)),
    ).filter((prop) => this._isUpdatable(prop, (changes as any)[prop]));

    const validations = toUpdate.map(async (prop) => {
      const value = (changes as any)[prop] as Output[KeyOf<Output>];

      const isValid = (await this._validate(
        prop as any,
        value,
        this._getValidationSummary(true),
      )) as InternalValidatorResponse<Output[KeyOf<Output>]>;

      if (!isValid.valid) return this._handleInvalidValue(error, prop, isValid);

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
      else updates[propName as KeyOf<Output>] = validated;

      const validCtxUpdate = { [propName]: validated } as any;

      this._updateContext(validCtxUpdate);
      this._updatePartialContext(validCtxUpdate);
    });

    await Promise.allSettled(validations);

    return { error, updates, virtuals };
  }

  private async _handleSecondaryValidations(
    data: Partial<Output>,
    isUpdate = false,
  ) {
    const summary = this._getMutableSummary(data, isUpdate),
      context = summary.context;

    const error = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      context.__getOptions__(),
    );

    const props: [KeyOf<Output>, string | undefined][] = [];

    for (const prop of this.propsWithSecondaryValidators.values()) {
      if (!isUpdate && !this._isInitAllowed(prop)) continue;

      const alias = this._getAliasByVirtual(prop as any);

      if (!this._isSuccessfulProp(prop, summary, alias)) continue;

      props.push([prop as KeyOf<Output>, alias]);
    }

    const validations = props.map(async ([prop, alias]) => {
      const validator = this._getSecondaryValidator(prop);

      if (!validator) return;

      const value = summary.context?.[prop] as Output[KeyOf<Output>];

      let isValid: ValidatorResponseObject<any>;

      try {
        isValid = this._sanitizeValidationResponse<any>(
          (await validator(value, summary)) as ValidatorResponseObject<any>,
          value,
        );
      } catch (_) {
        isValid = makeResponse<any>({
          valid: false,
          reason: 'validation failed',
        });
      }

      if (!isValid.valid) {
        const _prop =
          alias && isPropertyOf(alias, summary.inputValues) ? alias : prop;

        return this._handleInvalidValue(error, _prop as any, isValid);
      }

      let { validated } = isValid;

      if (isEqual(validated, undefined)) validated = value;
      if (isEqual(validated, this.values[prop], this._options.equalityDepth))
        return;

      if (!this._isVirtual(prop)) data[prop as KeyOf<Output>] = validated;

      const validCtxUpdate = { [prop]: validated } as any;

      this._updateContext(validCtxUpdate);
      this._updatePartialContext(validCtxUpdate);
    });

    await Promise.allSettled(validations);

    return error;
  }

  private _isSuccessfulProp(
    prop: string,
    summary: MutableSummary<Input, Output, CtxOptions>,
    alias_?: string,
  ) {
    if (this._isVirtual(prop)) {
      if (isPropertyOf(prop, this.partialContext)) return true;

      const alias = alias_ || this._getAliasByVirtual(prop as any);

      return (
        !isNullOrUndefined(alias) && isPropertyOf(alias, summary.inputValues)
      );
    }

    return !summary.isUpdate || isPropertyOf(prop, summary.changes);
  }

  private async _handlePostValidations(
    data: Partial<Output>,
    isUpdate = false,
  ) {
    const summary = this._getMutableSummary(data, isUpdate),
      context = summary.context;

    const errorTool = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      context.__getOptions__(),
    );

    const handlerIds = new Set<string>(),
      handlerIdToProps = new Map<string, Set<string>>();

    for (const [
      prop,
      setOfIDs,
    ] of this.propToPostValidationConfigIDsMap.entries()) {
      if (!this._isSuccessfulProp(prop, summary)) continue;

      for (const id of setOfIDs.values()) {
        handlerIds.add(id);

        const set = handlerIdToProps.get(id) ?? new Set();
        handlerIdToProps.set(id, set.add(prop));
      }
    }

    const handlers = Array.from(handlerIds).map((id) => ({
      id,
      validator: this.postValidationConfigMap.get(id)!.validators,
    }));

    await Promise.allSettled(
      handlers.map(async ({ id, validator }) => {
        const propsProvided = Array.from(handlerIdToProps.get(id)!) as Extract<
          keyof Input,
          string
        >[];

        if (!Array.isArray(validator))
          return await this._handlePostValidator({
            errorTool,
            propsProvided,
            summary,
            validator,
          });

        for (const v1 of validator) {
          if (Array.isArray(v1)) {
            const summary = this._getMutableSummary(data, isUpdate);

            const results = await Promise.all(
              v1.map(
                async (v2) =>
                  await this._handlePostValidator({
                    errorTool,
                    propsProvided,
                    summary,
                    validator: v2,
                  }),
              ),
            );

            if (results.some((r) => r.success == false)) break;

            continue;
          }

          const { success } = await this._handlePostValidator({
            errorTool,
            propsProvided,
            summary: this._getMutableSummary(data, isUpdate),
            validator: v1,
          });

          if (!success) break;
        }
      }),
    );

    return errorTool;
  }

  private async _handlePostValidator({
    errorTool,
    propsProvided,
    summary,
    validator,
  }: {
    errorTool: ErrorTool;
    propsProvided: Extract<keyof Input, string>[];
    summary: MutableSummary<Input, Output, CtxOptions>;
    validator: PostValidator<Input, Output, any, CtxOptions>;
  }) {
    try {
      const res = await validator(summary, propsProvided);

      if (!isRecordLike(res)) return { success: true };

      for (const [prop, error] of Object.entries(
        this._handleObjectValidationResponse(res),
      ))
        errorTool.add(prop, makeFieldError(error));
    } catch (_) {
      for (const prop of propsProvided) {
        const alias = this._getAliasByVirtual(prop as any);

        let errorField: string | undefined;

        if (alias && isPropertyOf(alias, summary.inputValues))
          errorField = alias;
        else if (isPropertyOf(prop, summary.inputValues)) errorField = prop;

        if (errorField) errorTool.add(errorField, validationFailedFieldError);
      }

      // return { success: false };
    }

    return { success: !errorTool.isLoaded };
  }

  private async _handleRequiredBy(data: Partial<Output>, isUpdate = false) {
    const summary = this._getMutableSummary(data, isUpdate),
      context = summary.context;

    const errorTool = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      context.__getOptions__(),
    );

    await Promise.allSettled(
      Array.from(this.propsRequiredBy.keys()).map(async (prop) => {
        let isUpdatable = false;

        if (isUpdate && this._isReadonly(prop)) {
          isUpdatable = this._isUpdatable(
            prop,
            (summary.inputValues as any)?.[prop],
          );

          if (!isUpdatable) return;
        }

        const [isRequired, message] = await this._getRequiredState(
          prop,
          summary,
        );

        if (
          !isRequired ||
          (isUpdate &&
            !isUpdatable &&
            !this._isUpdatable(prop, (summary.inputValues as any)?.[prop]))
        )
          return;

        const value = (data as any)[prop];

        const alias = this._getAliasByVirtual(prop);

        if (!alias) {
          errorTool.add(prop, makeFieldError(message), value);

          return;
        }

        const _message =
          message == `'${prop}' is required`
            ? `'${alias}' is required`
            : message;

        errorTool.add(alias as any, makeFieldError(_message), value);
      }),
    );

    return errorTool;
  }

  private async _handleSanitizationOfVirtuals(
    data: Partial<Output>,
    isUpdate = false,
  ) {
    const sanitizers: [KeyOf<Input>, Function][] = [];

    const partialCtx = this._getPartialContext();

    const successFulVirtuals = getKeysAsProps(partialCtx).filter(
      this._isVirtual,
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

  private _handleObjectValidationResponse(data: Record<string, any>) {
    const validProperties = getKeysAsProps(data).filter(
      (prop) =>
        this._isInputOrAlias(prop) ||
        this._isInputOrAlias(prop.split('.')?.[0]),
    );

    const otherReasons = {} as Record<
      string,
      string | string[] | InputFieldError
    >;

    for (const prop of validProperties) {
      const fieldError = data[prop];

      const isArray = Array.isArray(fieldError),
        isString = typeof fieldError == 'string';

      if (isInputFieldError(fieldError)) {
        otherReasons[prop] = fieldError as InputFieldError;

        continue;
      }

      if (isArray) {
        const messages = (fieldError as any[]).filter(
          (v) => typeof v == 'string',
        );

        otherReasons[prop] = messages.length ? messages : 'validation failed';

        continue;
      }

      if (isString) {
        const message = fieldError.trim();

        otherReasons[prop] = message.length ? message : 'validation failed';

        continue;
      }

      otherReasons[prop] = 'validation failed';
    }

    return otherReasons;
  }

  private _isSanitizable(
    prop: string,
    isCreation: boolean,
  ): [false, undefined] | [true, Function] {
    const { sanitizer, shouldInit } = this._getDefinition(prop);

    if (!sanitizer) return [false, undefined];

    if (isCreation && isEqual(shouldInit, false)) return [false, undefined];

    return [true, sanitizer];
  }

  private async _isGloballyUpdatable(changes: any) {
    const { shouldUpdate = defaultOptions.shouldUpdate! } = this._options;

    if (typeof shouldUpdate == 'boolean') return shouldUpdate;

    const response = await shouldUpdate(this._getMutableSummary(changes, true));

    return response;
  }

  private _isUpdatable(prop: string, value: any = undefined) {
    if (!this._isInputOrAlias(prop)) return false;

    const isAlias = this._isVirtualAlias(prop);

    const propName = (
      isAlias ? this._getVirtualByAlias(prop)! : prop
    ) as KeyOf<Output>;

    const hasShouldUpdateRule = this._isRuleInDefinition(
      propName,
      'shouldUpdate',
    );

    const extraCtx = isAlias ? { [propName]: value } : {};

    const isUpdatable = this._isUpdateAllowed(propName, extraCtx);

    if (this._isVirtual(prop)) return hasShouldUpdateRule ? isUpdatable : true;

    const isReadonly = this._isReadonly(propName);

    if (hasShouldUpdateRule && !isUpdatable) return false;

    if (isReadonly)
      return isEqual(
        this.defaults[propName],
        this.values[propName],
        this._options.equalityDepth,
      );

    return !isEqual(this.values[propName], value, this._options.equalityDepth);
  }

  private _isInputOrAlias = (prop: string) =>
    this._isVirtualAlias(prop) || this._isInputProp(prop);

  private _makeHandleFailure(
    data: Partial<Output>,
    errorTool: ErrorTool,
    virtuals: KeyOf<Output>[] = [],
  ) {
    const ctx = this._getContext(),
      props = Array.from(
        new Set([...getKeysAsProps(data), ...errorTool.fields, ...virtuals]),
      );

    let cleanups: NS.FailureHandler<Input, Output, {}>[] = [];

    for (const prop of props)
      cleanups = cleanups.concat(
        this._getHandlers<NS.FailureHandler<Input, Output, {}>>(
          prop,
          'onFailure',
        ),
      );

    return async () => {
      await Promise.allSettled(cleanups.map(async (h) => await h(ctx)));
    };
  }

  private _makeHandleSuccess(data: Partial<Output>, isUpdate = false) {
    const partialCtx = this._getPartialContext(),
      successProps = getKeysAsProps(partialCtx),
      summary = this._getSummary(data, isUpdate),
      setOfSuccessHandlerIDs = new Set<string>();

    let successListeners = [] as NS.SuccessHandler<Input, Output, CtxOptions>[];

    for (const prop of successProps) {
      const handlers = this._getHandlers<NS.SuccessHandler<Input, Output>>(
        prop,
        'onSuccess',
      );

      const setOfHandlerIDs = this.propToOnSuccessConfigIDMap.get(prop);

      if (setOfHandlerIDs)
        setOfHandlerIDs.forEach((id) => setOfSuccessHandlerIDs.add(id));

      successListeners = successListeners.concat(handlers);
    }

    successListeners = successListeners.concat(this.globalSuccessHandlers);

    for (const id of setOfSuccessHandlerIDs.values())
      successListeners = successListeners.concat(
        this.onSuccessConfigMap.get(id)!.handlers,
      );

    return async () => {
      await Promise.allSettled(
        successListeners.map(async (handler) => await handler(summary)),
      );
    };
  }

  private async _resolveDependentChanges(
    data: Partial<Output>,
    ctx: PartialContext<Input, Output>,
    isUpdate = false,
  ) {
    let _updates = { ...data };

    const successFulChanges = getKeysAsProps<Output>(ctx as any);

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
          this._options.equalityDepth,
        )
      )
        return;

      const resolver = this._getDefinition(prop).resolver!;

      let value;

      try {
        value = await resolver(summary);
      } catch (_) {
        value = isCreation ? null : summary.previousValues?.[prop];
      }

      if (
        !isCreation &&
        isEqual(
          value,
          _ctx[prop as KeyOf<ImmutableContext<Input, Output>>],
          this._options.equalityDepth,
        )
      )
        return;

      data[prop] = value;

      const updates = { [prop]: value } as any;

      this._updateContext(updates);
      this._updatePartialContext(updates);

      const _data = await this._resolveDependentChanges(
        data,
        updates,
        isUpdate,
      );

      return (_updates = { ..._updates, ..._data });
    });

    await Promise.allSettled(operations);

    return _updates;
  }

  private _setValues(
    values: Partial<Input | Output | Aliases>,
    {
      allowVirtuals = true,
      allowTimestamps = false,
    }: {
      allowVirtuals?: boolean;
      allowTimestamps?: boolean;
    } = {
      allowVirtuals: true,
      allowTimestamps: false,
    },
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

    this._initializeImmutableContexts();
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
      }),
    );
  }

  private async _setValidValue(
    operationData: Partial<Output> = {},
    prop: KeyOf<Output>,
    value: Output[KeyOf<Output>],
  ) {
    const isAlias = this._isVirtualAlias(prop);

    const propName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    if (!this._isVirtual(propName))
      operationData[propName as KeyOf<Output>] = value;

    const validCtxUpdate = { [propName]: value } as any;

    this._updateContext(validCtxUpdate);
    this._updatePartialContext(validCtxUpdate);
  }

  private _sanitizeValidationResponse<T>(
    response: ValidatorResponseObject<T>,
    value: any,
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

    const _response: InvalidValidatorResponse = { valid: false, value } as any;

    if (response?.reason) _response.reason = response.reason;

    if (isRecordLike(response?.reason))
      _response.reason = this._handleObjectValidationResponse(response.reason);

    if (response?.metadata && isRecordLike(response.metadata))
      _response.metadata = sortKeys(response.metadata);
    else _response.metadata = null;

    if (!_response.reason) {
      if (_response.metadata)
        return {
          ...getValidationFailedResponse(value),
          metadata: _response.metadata,
        } as any;

      return getValidationFailedResponse(value);
    }

    return makeResponse(_response);
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
    value: any,
  ) {
    const isValid = (await this._validate(
      prop as any,
      value,
      this._getValidationSummary(false),
    )) as InternalValidatorResponse<Output[KeyOf<Output>]>;

    if (isValid.valid)
      return this._setValidValue(operationData, prop, isValid.validated);

    this._handleInvalidValue(errorTool, prop, isValid);
  }

  private async _validate<K extends KeyOf<Input & Aliases>>(
    prop: K,
    value: any,
    summary_: MutableSummary<Input, Output>,
  ) {
    if (!this._isInputOrAlias(prop))
      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        value,
        reason: 'Invalid property',
      });

    const isAlias = this._isVirtualAlias(prop);

    const _prop = (isAlias ? this._getVirtualByAlias(prop) : prop)!;

    const allowedValues = this.propsToAllowedValuesMap.get(_prop);

    if (allowedValues && !allowedValues.has(value)) {
      const fieldError = makeFieldError(this._getNotAllowedError(_prop, value));

      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        value,
        reason: fieldError.reasons,
        metadata: fieldError.metadata || { allowed: Array.from(allowedValues) },
      });
    }

    const validator = this._getPrimaryValidator(_prop as any);

    if (validator) {
      let res: ValidatorResponseObject<(Input & Aliases)[K]>;

      try {
        res = this._sanitizeValidationResponse<(Input & Aliases)[K]>(
          (await validator(value, summary_)) as ValidatorResponseObject<
            (Input & Aliases)[K]
          >,
          value,
        );
      } catch (_) {
        return makeResponse<(Input & Aliases)[K]>({
          valid: false,
          reason: 'validation failed',
        });
      }

      if (allowedValues && res.valid && !allowedValues.has(res.validated))
        return makeResponse<(Input & Aliases)[K]>({
          valid: true,
          validated: value,
        });

      return res;
    }

    return makeResponse<(Input & Aliases)[K]>({
      valid: true,
      validated: value,
    });
  }

  async create(
    input: Partial<Input & Aliases> = {},
    contextOptions: Partial<CtxOptions> = {},
  ) {
    this._initializeContextOptions(contextOptions);

    if (!areValuesOk(input)) input = {};

    const _input = this._cleanInput(input);

    this._setValues(_input);

    let data = await this._generateConstants();

    const {
      data: dt,
      error,
      virtuals,
    } = await this._handleCreationPrimaryValidations(data, _input);

    if (error.isLoaded) return this._handleError(data, error, virtuals);

    data = dt;

    const requiredError = await this._handleRequiredBy(data);
    if (requiredError.isLoaded)
      return this._handleError(data, requiredError, virtuals);

    const error2 = await this._handleSecondaryValidations(data);
    if (error2.isLoaded) return this._handleError(data, error2, virtuals);

    const postValidationError = await this._handlePostValidations(data);
    if (postValidationError.isLoaded)
      return this._handleError(data, postValidationError, virtuals);

    await this._handleSanitizationOfVirtuals(data);
    data = await this._resolveDependentChanges(data, this._getPartialContext());

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as any);
    this._updatePartialContext(finalData as any);

    return {
      data: finalData as Output,
      error: null,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(finalData),
    };
  }

  async delete(values: Output, contextOptions: Partial<CtxOptions> = {}) {
    const ctxOptions = this._initializeContextOptions(contextOptions);

    if (!areValuesOk(values)) return;

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    let handlers: NS.DeleteHandler<Output, CtxOptions>[] = [
      ...this.globalDeleteHandlers,
    ];

    const data = this._getFrozenCopy({
      ...this.values,
      __getOptions__: () => ctxOptions,
    });

    getSetValuesAsProps(this.props).map(async (prop) => {
      const handlers_ = this._getHandlers<NS.DeleteHandler<Output, CtxOptions>>(
        prop,
        'onDelete',
      );

      if (handlers_.length) handlers = handlers.concat(handlers_);
    });

    const cleanups = handlers.map(async (handler) => await handler(data));

    await Promise.allSettled(cleanups);
  }

  async update(
    values: Output,
    changes: Partial<Input & Aliases>,
    ctxOptions: Partial<CtxOptions> = {},
  ) {
    const ctxOpts = this._initializeContextOptions(ctxOptions);

    const errorNothingToUpdate = new this._options.ErrorTool(
      VALIDATION_ERRORS.NOTHING_TO_UPDATE,
      ctxOpts,
    );

    if (!areValuesOk(values) || !areValuesOk(changes))
      return this._handleError({}, errorNothingToUpdate, []);

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    if (this._options?.setMissingDefaultsOnUpdate)
      await this._setMissingDefaults();

    const _changes = this._cleanInput(changes);

    if (!(await this._isGloballyUpdatable(_changes)))
      return this._handleError({}, errorNothingToUpdate, []);

    const {
      error,
      updates: dt,
      virtuals,
    } = await this._handleUpdatePrimaryValidations(_changes);

    if (error.isLoaded) return this._handleError(dt, error, virtuals);

    let updates = dt;

    const requiredErrorTool = await this._handleRequiredBy(updates, true);
    if (requiredErrorTool.isLoaded)
      return this._handleError(updates, requiredErrorTool, virtuals);

    const error2 = await this._handleSecondaryValidations(updates, true);
    if (error2.isLoaded) return this._handleError(updates, error2, virtuals);

    const postValidationError = await this._handlePostValidations(
      updates,
      true,
    );
    if (postValidationError.isLoaded)
      return this._handleError(updates, postValidationError, virtuals);

    await this._handleSanitizationOfVirtuals(updates, true);

    updates = await this._resolveDependentChanges(
      updates,
      this._getPartialContext(),
      true,
    );

    if (!Object.keys(updates).length) {
      errorNothingToUpdate.setMessage(VALIDATION_ERRORS.NOTHING_TO_UPDATE);

      return this._handleError(updates, errorNothingToUpdate, virtuals);
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
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(finalData, true),
    };
  }
}

class Model<
  Input extends RealType<Input>,
  Output extends RealType<Output>,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<any> = DefaultErrorTool<KeyOf<Input & Aliases>>,
> {
  constructor(
    private modelTool: ModelTool<Input, Output, Aliases, CtxOptions, ErrorTool>,
  ) {}

  create = (
    values: Partial<Input & Aliases> = {},
    contextOptions: Partial<CtxOptions> = {},
  ) => this.modelTool.create(values, contextOptions);

  delete = (values: Output, contextOptions: Partial<CtxOptions> = {}) =>
    this.modelTool.delete(values, contextOptions);

  update = (
    values: Output,
    changes: Partial<Input & Aliases>,
    contextOptions: Partial<CtxOptions> = {},
  ) => this.modelTool.update(values, changes, contextOptions);
}

function areValuesOk(values: any) {
  return values && typeof values == 'object';
}

function getValidationFailedResponse(value: any) {
  return {
    metadata: null,
    reason: ['validation failed'],
    valid: false,
    value,
  } as ValidatorResponseObject<any>;
}
