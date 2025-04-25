import {
  type ObjectType,
  getKeysAsProps,
  getSetValuesAsProps,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  makeResponse,
  sort,
  sortKeys,
  toArray,
} from '../utils';
import { SchemaCore, defaultOptions } from './schema-core';
import {
  type ImmutableContext,
  type InternalValidatorResponse,
  type InvalidValidatorResponse,
  type KeyOf,
  LIFE_CYCLES,
  type MutableSummary,
  type NS,
  type PartialContext,
  type PostValidator,
  type RealType,
  type Validator,
  type ValidatorResponseObject,
} from './types';
import {
  type DefaultErrorTool,
  type IErrorTool,
  type InputFieldError,
  VALIDATION_ERRORS,
  isInputFieldError,
  makeFieldError,
} from './utils';

export { Model, ModelTool, Schema };

const NotAllowedError = 'value not allowed';
const validationFailedFieldError = makeFieldError('validation failed');

class Schema<
  Input extends RealType<Input>,
  Output extends RealType<Output> = Input,
  Aliases = {},
  CtxOptions extends ObjectType = {},
  ErrorTool extends IErrorTool<ObjectType> = DefaultErrorTool<
    KeyOf<Input & Aliases>
  >,
> extends SchemaCore<Input, Output, CtxOptions, ErrorTool> {
  constructor(
    definitions: NS.Definitions<Input, Output, Aliases, CtxOptions>,
    options: NS.Options<
      Input,
      Output,
      Aliases,
      ErrorTool,
      CtxOptions
    > = defaultOptions as never,
  ) {
    super(
      definitions as never as NS.Definitions_<Input, Output>,
      options as never,
    );
  }

  get definitions() {
    return this._definitions as never as NS.Definitions<Input, Output, Aliases>;
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
    Aliases = object,
    ExtendedCtxOptions extends ObjectType = CtxOptions,
    ExtendedErrorTool extends IErrorTool<ObjectType> = DefaultErrorTool<
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
      (prop) => delete (_definitions as never)?.[prop],
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
          (prop) => ![...LIFE_CYCLES, 'shouldUpdate'].includes(prop as never),
        )
        .forEach((prop) => {
          options_[prop] = this.options[prop] as never;
        });

    return new Schema<
      ExtendedInput,
      ExtendedOutput,
      Aliases,
      ExtendedCtxOptions,
      ExtendedErrorTool
    >(
      Object.assign({}, _definitions, definitions),
      Object.assign({}, options_, rest),
    );
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
  ErrorTool extends IErrorTool<ObjectType> = DefaultErrorTool<
    KeyOf<Input & Aliases>
  >,
> extends SchemaCore<Input, Output, CtxOptions, ErrorTool> {
  private _regeneratedProps: KeyOf<Output>[] = [];
  private inputValues: Partial<RealType<Input>> = {};

  constructor(schema: Schema<Input, Output, Aliases, CtxOptions, ErrorTool>) {
    super(schema.definitions as never, schema.options as never);
  }

  private _cleanInput(input: Partial<Input | Aliases>) {
    const props = getKeysAsProps(input).filter(this._isInputOrAlias);

    const values = {} as never;

    for (const prop of props) {
      values[prop] = input[prop] as never;

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
        } catch {
          data[prop] = null as never;
        }

        const validCtxUpdate = { [prop]: data[prop] as never } as never;

        this._updatePartialContext(validCtxUpdate);

        return this._updateContext(validCtxUpdate);
      }),
    );

    return data;
  }

  private _isIngnorable = (prop: string) => {
    return !!this._getDefinition(prop).ignore;
  };

  private _shouldIgnore = ({
    prop,
    isUpdate = false,
  }: {
    prop: string;
    isUpdate?: boolean;
  }) => {
    const { ignore } = this._getDefinition(prop);

    if (ignore)
      return ignore(
        this._getMutableSummary({
          data: {},
          inputValues: this.inputValues,
          isUpdate,
        }),
      );
  };

  private _isInitAllowed = (prop: string, extraCtx: ObjectType = {}) => {
    if (isOneOf(this._getDefinition(prop).shouldInit, [true, undefined]))
      return true;

    return this._getValueBy(prop, 'shouldInit', extraCtx) === true;
  };

  private _canInit = (prop: string) => {
    if (this._isDependentProp(prop)) return false;
    if (this._isRequired(prop)) return true;

    if (this._isIngnorable(prop)) return !this._shouldIgnore({ prop });

    const isInitAllowed = this._isInitAllowed(prop);

    if (this._isLaxProp(prop)) return isInitAllowed;

    const { readonly } = this._getDefinition(prop);

    return readonly === true && isInitAllowed && !this._isRequiredBy(prop);
  };

  private _isUpdateAllowed = (prop: string, extraCtx: ObjectType = {}) => {
    if (isOneOf(this._getDefinition(prop).shouldUpdate, [true, undefined]))
      return true;

    return this._getValueBy(prop, 'shouldUpdate', extraCtx) === true;
  };

  private _isVirtualInit = (prop: string, value: unknown = undefined) => {
    const isAlias = this._isVirtualAlias(prop);

    if (!this._isVirtual(prop) && !isAlias) return false;

    const definitionName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    const { shouldInit } = this._getDefinition(definitionName);

    const extraCtx = isAlias ? { [definitionName]: value } : {};

    return (
      isEqual(shouldInit, undefined) ||
      this._isInitAllowed(definitionName, extraCtx)
    );
  };

  private _getValidationSummary = (isUpdate: boolean) =>
    this._getMutableSummary({
      data: this.values,
      isUpdate,
      inputValues: this.inputValues,
    });

  private _getPrimaryValidator = <K extends keyof (Output | Input)>(
    prop: string,
  ) => {
    const { validator } = this._getDefinition(prop as never);

    return (Array.isArray(validator) ? validator[0] : validator) as
      | Validator<K, Input, Output>
      | undefined;
  };

  private _getSecondaryValidator = <K extends keyof (Output | Input)>(
    prop: string,
  ) => {
    const { validator } = this._getDefinition(prop as never);

    return (Array.isArray(validator) ? validator[1] : undefined) as
      | Validator<K, Input, Output>
      | undefined;
  };

  private _getNotAllowedError(prop: string, value: unknown) {
    const allow = this._getDefinition(prop as never)?.allow;

    if (Array.isArray(allow)) return NotAllowedError;

    // @ts-ignore: lol
    const error = allow?.error;

    if (isInputFieldError(error)) return error;

    if (isFunctionLike(error)) {
      let message: any;

      try {
        message = error(value, allow?.values);
      } catch {
        return NotAllowedError;
      }

      if (typeof message === 'string') return message || NotAllowedError;

      return isInputFieldError(message) ? message : NotAllowedError;
    }

    return error || NotAllowedError;
  }

  private _handleError(
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

  private _handleInvalidValue(
    errorTool: ErrorTool,
    prop: KeyOf<Input & Output & Aliases>,
    validationResponse: InvalidValidatorResponse,
  ) {
    const { reason, metadata, value } = validationResponse;

    if (isRecordLike(reason)) {
      if (metadata) errorTool.set(prop, { metadata, reason: '' }, value);

      return Object.entries(reason).forEach(([key, message]) => {
        errorTool.set(key, makeFieldError(message as never));
      });
    }

    const fieldError = makeFieldError(
      // @ts-ignore: lol
      reason || 'validation failed',
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.set(prop, fieldError, value);
  }

  private async _handleCreationPrimaryValidations(
    data: Partial<Output>,
    input: never,
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

    await Promise.allSettled(
      props.map(async (prop) => {
        const isVirtualInit = virtuals.includes(prop);

        if (this._isVirtual(prop) && !isVirtualInit) return;

        const isDependent = this._isDependentProp(prop),
          isVirtualAlias = virtuals.includes(prop);

        if (isDependent) {
          data[prop] = await this._getDefaultValue(prop);

          const validCtxUpdate = { [prop]: data[prop] as never } as never;

          this._updatePartialContext(validCtxUpdate);
          this._updateContext(validCtxUpdate);

          if (!isVirtualAlias) return;
        }

        if (isVirtualAlias) {
          const propName = (this._getVirtualByAlias(prop) || prop)!;

          if (
            this._isIngnorable(propName) &&
            this._shouldIgnore({ prop: propName })
          )
            return;

          return this._validateAndSet(
            data,
            error,
            prop,
            input[prop as unknown as KeyOf<Input>],
          );
        }

        const isProvided = isPropertyOf(prop, this.values),
          isLax = this._isLaxProp(prop),
          isLaxInit = isLax && isProvided,
          isRequiredInit = this._isRequiredBy(prop) && isProvided,
          canInit = this._canInit(prop);

        if (
          (isLax && (!canInit || (canInit && !isProvided))) ||
          (!isVirtualInit && !canInit && !isLaxInit && !isRequiredInit)
        ) {
          data[prop] = await this._getDefaultValue(prop);

          const validCtxUpdate = { [prop]: data[prop] as never } as never;
          this._updatePartialContext(validCtxUpdate);

          return this._updateContext(validCtxUpdate);
        }

        return this._validateAndSet(data, error, prop, this.values[prop]);
      }),
    );

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
      new Set(getKeysAsProps<Output & Aliases>(changes as never)),
    ).filter((prop) => this._isUpdatable(prop, (changes as never)[prop]));

    await Promise.allSettled(
      toUpdate.map(async (prop) => {
        const value = (changes as never)[prop] as Output[KeyOf<Output>];

        const isValid = (await this._validate(
          prop as never,
          value,
          this._getValidationSummary(true),
        )) as InternalValidatorResponse<Output[KeyOf<Output>]>;

        if (!isValid.valid)
          return this._handleInvalidValue(error, prop, isValid);

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

        const validCtxUpdate = { [propName]: validated } as never;

        this._updateContext(validCtxUpdate);
        this._updatePartialContext(validCtxUpdate);
      }),
    );

    return { error, updates, virtuals };
  }

  private async _handleSecondaryValidations(
    data: Partial<Output>,
    isUpdate = false,
  ) {
    const summary = this._getMutableSummary({
        data,
        isUpdate,
        inputValues: this.inputValues,
      }),
      context = summary.context;

    const error = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      context.__getOptions__(),
    );

    const props: [KeyOf<Output>, string | undefined][] = [];

    for (const prop of this.propsWithSecondaryValidators.values()) {
      if (!isUpdate && !this._isInitAllowed(prop)) continue;

      const alias = this._getAliasByVirtual(prop as never);

      if (!this._isSuccessfulProp(prop, summary, alias)) continue;

      props.push([prop as KeyOf<Output>, alias]);
    }

    await Promise.allSettled(
      props.map(async ([prop, alias]) => {
        const validator = this._getSecondaryValidator(prop);

        if (!validator) return;

        const value = summary.context?.[prop] as Output[KeyOf<Output>];

        let isValid: ValidatorResponseObject<unknown>;

        try {
          isValid = this._sanitizeValidationResponse<unknown>(
            (await validator(
              value,
              summary as never,
            )) as ValidatorResponseObject<unknown>,
            value,
          );
        } catch {
          isValid = makeResponse<unknown>({
            valid: false,
            reason: 'validation failed',
          });
        }

        if (!isValid.valid) {
          const _prop =
            alias && isPropertyOf(alias, summary.inputValues) ? alias : prop;

          return this._handleInvalidValue(error, _prop as never, isValid);
        }

        let { validated } = isValid;

        if (isEqual(validated, undefined)) validated = value;
        if (isEqual(validated, this.values[prop], this._options.equalityDepth))
          return;

        if (!this._isVirtual(prop)) data[prop] = validated as never;

        const validCtxUpdate = { [prop]: validated } as never;

        this._updateContext(validCtxUpdate);
        this._updatePartialContext(validCtxUpdate);
      }),
    );

    return error;
  }

  private _isSuccessfulProp(
    prop: string,
    summary: MutableSummary<Input, Output, CtxOptions>,
    alias_?: string,
  ) {
    if (this._isVirtual(prop)) {
      if (isPropertyOf(prop, this.partialContext)) return true;

      const alias = alias_ || this._getAliasByVirtual(prop as never);

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
    const summary = this._getMutableSummary({
        data,
        isUpdate,
        inputValues: this.inputValues,
      }),
      context = summary.context;

    const errorTool = new this._options.ErrorTool(
      VALIDATION_ERRORS.VALIDATION_ERROR,
      context.__getOptions__(),
    );

    const handlerIds = new Set<string>(),
      handlerIdToProps = new Map<string, Set<string>>(),
      configIDsToAllPostValidatableProps = new Map<string, Set<string>>();

    for (const [
      prop,
      setOfConfigIDs,
    ] of this.propToPostValidationConfigIDsMap.entries()) {
      const isSuccessfulProp = this._isSuccessfulProp(prop, summary);

      for (const id of setOfConfigIDs.values()) {
        {
          const set = configIDsToAllPostValidatableProps.get(id) ?? new Set();
          configIDsToAllPostValidatableProps.set(id, set.add(prop));
        }

        if (!isSuccessfulProp) continue;

        handlerIds.add(id);

        const set = handlerIdToProps.get(id) ?? new Set();
        handlerIdToProps.set(id, set.add(prop));
      }
    }

    const handlers = Array.from(handlerIds).map((id) => ({
      id,
      validator: this.postValidationConfigMap.get(id)!.validators,
      postValidatableProps: Array.from(
        configIDsToAllPostValidatableProps.get(id)!,
      ) as KeyOf<Input>[],
    }));

    const handleRevalidatedData = (revalidatedData: Partial<Output> | null) => {
      if (!revalidatedData) return;

      for (const prop of getKeysAsProps(revalidatedData)) {
        const validated = revalidatedData[prop];

        if (!this._isVirtual(prop)) data[prop] = validated;

        const validCtxUpdate = { [prop]: validated } as never;

        this._updateContext(validCtxUpdate);
        this._updatePartialContext(validCtxUpdate);
      }
    };

    await Promise.allSettled(
      handlers.map(async ({ id, validator, postValidatableProps }) => {
        const propsProvided = Array.from(handlerIdToProps.get(id)!) as Extract<
          keyof Input,
          string
        >[];

        if (!Array.isArray(validator)) {
          const { revalidatedData, success } = await this._handlePostValidator({
            errorTool,
            propsProvided,
            summary,
            validator: validator as any,
            postValidatableProps,
          });

          if (!success || !revalidatedData) return;

          return handleRevalidatedData(revalidatedData);
        }

        for (const v1 of validator) {
          if (Array.isArray(v1)) {
            const summary = this._getMutableSummary({
              data: this.values,
              isUpdate,
              inputValues: this.inputValues,
            });

            const results = await Promise.all(
              v1.map(async (v2) => {
                const res = await this._handlePostValidator({
                  errorTool,
                  propsProvided,
                  summary,
                  validator: v2 as any,
                  postValidatableProps,
                });

                handleRevalidatedData(res.revalidatedData);

                return res;
              }),
            );

            if (results.some((r) => r.success === false)) break;

            continue;
          }

          const { revalidatedData, success } = await this._handlePostValidator({
            errorTool,
            propsProvided,
            summary: this._getMutableSummary({
              data: this.values,
              isUpdate,
              inputValues: this.inputValues,
            }),
            validator: v1 as any,
            postValidatableProps,
          });

          if (!success) break;

          if (revalidatedData) handleRevalidatedData(revalidatedData);
        }
      }),
    );

    return errorTool;
  }

  private async _handlePostValidator({
    errorTool,
    propsProvided,
    postValidatableProps,
    summary,
    validator,
  }: {
    errorTool: ErrorTool;
    propsProvided: Extract<keyof Input, string>[];
    postValidatableProps: Extract<keyof Input, string>[];
    summary: MutableSummary<Input, Output, CtxOptions>;
    validator: PostValidator<KeyOf<Input>, Input, Output, Aliases, CtxOptions>;
  }) {
    const revalidatedData: Partial<Output> = {};

    try {
      const res = await validator(summary, propsProvided);

      if (!isRecordLike(res)) return { revalidatedData: null, success: true };

      const { errors, validatedData } =
        this._handleObjectValidationResponse(res);

      for (const [prop, validated] of Object.entries(validatedData) as [
        KeyOf<Input>,
        any,
      ][]) {
        const propName = (this._getAliasByVirtual(prop as never) ??
          prop) as keyof Output;

        if (
          postValidatableProps.includes(prop) ||
          postValidatableProps.includes(propName as any)
        )
          revalidatedData[propName] = validated;
      }

      for (const [prop, error] of Object.entries(errors))
        errorTool.set(prop, makeFieldError(error));
    } catch {
      for (const prop of propsProvided) {
        const alias = this._getAliasByVirtual(prop as never);

        let errorField: string | undefined;

        if (alias && isPropertyOf(alias, summary.inputValues))
          errorField = alias;
        else if (isPropertyOf(prop, summary.inputValues)) errorField = prop;

        if (errorField) errorTool.set(errorField, validationFailedFieldError);
      }
    }

    const success = !errorTool.isLoaded;

    return {
      revalidatedData:
        !success || !Object.keys(revalidatedData).length
          ? null
          : revalidatedData,
      success,
    };
  }

  private async _handleRequiredBy(data: Partial<Output>, isUpdate = false) {
    const summary = this._getMutableSummary({
        data: this.values,
        isUpdate,
        inputValues: this.inputValues,
      }),
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
            (summary.inputValues as never)?.[prop],
          );

          if (!isUpdatable) return;
        }

        const [isRequired, message] = await this._getRequiredState(
          prop,
          summary as never,
        );

        if (
          !isRequired ||
          (isUpdate &&
            !isUpdatable &&
            !this._isUpdatable(prop, (summary.inputValues as never)?.[prop]))
        )
          return;

        const value = (data as never)[prop];

        const alias = this._getAliasByVirtual(prop);

        if (!alias) {
          errorTool.set(prop, makeFieldError(message), value);

          return;
        }

        errorTool.set(
          alias as never,
          makeFieldError(
            message === `'${prop}' is required`
              ? `'${alias}' is required`
              : message,
          ),
          value,
        );
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

    const summary = this._getMutableSummary({
      data,
      isUpdate,
      inputValues: this.inputValues,
    });

    await Promise.allSettled(
      sanitizers.map(async ([prop, sanitizer]) => {
        const resolvedValue = await sanitizer(summary);

        this._updateContext({ [prop]: resolvedValue } as never);
      }),
    );
  }

  private _handleObjectValidationResponse(data: Record<string, unknown>) {
    const validProperties = getKeysAsProps(data).filter((prop) =>
      this._isInputOrAlias(prop),
    );

    const errors = {} as Record<string, string | InputFieldError>;
    const validatedData = {} as Record<string, unknown>;

    for (const prop of validProperties) {
      const res = data[prop];

      if (typeof res === 'object' && 'validated' in (res as any)) {
        validatedData[prop] = (res as any).validated;

        continue;
      }

      if (isInputFieldError(res)) {
        errors[prop] = res as InputFieldError;

        continue;
      }

      if (typeof res === 'string') {
        const message = res.trim();

        errors[prop] = message.length ? message : 'validation failed';

        continue;
      }

      errors[prop] = 'validation failed';
    }

    return { errors, validatedData };
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

  private _isGloballyUpdatable(changes: unknown) {
    const { shouldUpdate = defaultOptions.shouldUpdate! } = this._options;

    if (typeof shouldUpdate === 'boolean') return shouldUpdate;

    return shouldUpdate(
      this._getMutableSummary({
        data: changes as never,
        isUpdate: true,
        inputValues: this.inputValues,
      }) as never,
    );
  }

  private _isUpdatable(prop: string, value: unknown = undefined) {
    if (!this._isInputOrAlias(prop)) return false;

    const isAlias = this._isVirtualAlias(prop);

    const propName = (
      isAlias ? this._getVirtualByAlias(prop)! : prop
    ) as KeyOf<Output>;

    if (
      this._isIngnorable(propName) &&
      this._shouldIgnore({ prop: propName, isUpdate: true })
    )
      return false;

    const hasShouldUpdateRule = this._isRuleInDefinition(
      propName,
      'shouldUpdate',
    );

    const extraCtx = isAlias ? { [propName]: value } : {};

    const isUpdatable = this._isUpdateAllowed(propName, extraCtx);

    if (this._isVirtual(prop)) return hasShouldUpdateRule ? isUpdatable : true;

    if (hasShouldUpdateRule && !isUpdatable) return false;

    if (this._isReadonly(propName))
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

    let cleanups: NS.FailureHandler<Input, Output, never>[] = [];

    for (const prop of props)
      cleanups = cleanups.concat(
        this._getHandlers<NS.FailureHandler<Input, Output, never>>(
          prop,
          'onFailure',
        ),
      );

    return async () => {
      await Promise.allSettled(
        cleanups.map(async (h) => await h(ctx as never)),
      );
    };
  }

  private _makeHandleSuccess(data: Partial<Output>, isUpdate = false) {
    const partialCtx = this._getPartialContext(),
      successProps = getKeysAsProps(partialCtx),
      summary = this._getSummary({
        data,
        isUpdate,
        inputValues: this.inputValues,
      }),
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

      successListeners = successListeners.concat(handlers as never);
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
    const isCreation = !isUpdate;
    const successFulChanges = getKeysAsProps<Output>(ctx as never);
    let _updates = Object.assign({}, data);
    let toResolve = [] as KeyOf<Output>[];

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

      toResolve = toResolve.concat(dependencies as never);
    }

    toResolve = Array.from(new Set(toResolve));

    const values = isUpdate ? data : Object.assign({}, this.values, data),
      _ctx = this._getContext(),
      summary = this._getMutableSummary({
        data: values,
        isUpdate,
        inputValues: this.inputValues,
      });

    await Promise.allSettled(
      toResolve.map(async (prop) => {
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
        let value: any;

        try {
          value = await resolver(summary);
        } catch {
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
        const updates = { [prop]: value } as never;

        this._updateContext(updates);
        this._updatePartialContext(updates);

        const _data = await this._resolveDependentChanges(
          data,
          updates,
          isUpdate,
        );

        _updates = Object.assign(_updates, _data);
      }),
    );

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

    const _values = {} as never;

    sort(keys).forEach((key) => {
      _values[key] = values[key] as never;
    });

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

        this._updateContext({ [prop]: value } as never);
        this._updatePartialContext({ [prop]: value } as never);
      }),
    );
  }

  private _setValidValue(
    prop: KeyOf<Output>,
    value: Output[KeyOf<Output>],
    operationData: Partial<Output> = {},
  ) {
    const isAlias = this._isVirtualAlias(prop),
      propName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    if (!this._isVirtual(propName))
      operationData[propName as KeyOf<Output>] = value;

    const validCtxUpdate = { [propName]: value } as never;

    this._updateContext(validCtxUpdate);
    this._updatePartialContext(validCtxUpdate);
  }

  private _sanitizeValidationResponse<T>(
    response: ValidatorResponseObject<T>,
    value: unknown,
  ): ValidatorResponseObject<T> {
    const responseType = typeof response;

    if (responseType === 'boolean')
      return (
        response
          ? { valid: true, validated: value }
          : getValidationFailedResponse(value)
      ) as never;

    if (!response && responseType !== 'object')
      return getValidationFailedResponse(value) as never;

    if (response?.valid) {
      const validated = isEqual(response?.validated, undefined)
        ? value
        : response.validated;

      return { valid: true, validated } as never;
    }

    const _response: InvalidValidatorResponse = {
      valid: false,
      value,
    } as never;

    if (response?.reason && typeof response?.reason === 'string')
      _response.reason = response.reason;

    if (response?.metadata && isRecordLike(response.metadata))
      _response.metadata = sortKeys(response.metadata);
    else _response.metadata = null;

    if (!_response.reason) {
      if (_response.metadata)
        return {
          ...getValidationFailedResponse(value),
          metadata: _response.metadata,
        } as never;

      return getValidationFailedResponse(value) as never;
    }

    return makeResponse(_response);
  }

  private _useConfigProps(obj: Partial<Output>, isUpdate = false) {
    if (!this.timestampTool.withTimestamps) return sortKeys(obj);

    const { createdAt, updatedAt } = this.timestampTool.getKeys();
    let results = Object.assign({}, obj);

    const now = new Date();

    if (updatedAt)
      results = Object.assign(results, {
        [updatedAt]: isUpdate
          ? now
          : this.timestampTool.isNullable
            ? null
            : now,
      });

    if (!isUpdate && createdAt)
      results = Object.assign(results, { [createdAt]: now });

    return sortKeys(results);
  }

  private async _validateAndSet(
    operationData: Partial<Output>,
    errorTool: ErrorTool,
    prop: KeyOf<Output>,
    value: unknown,
  ) {
    const isValid = (await this._validate(
      prop as never,
      value,
      this._getValidationSummary(false),
    )) as InternalValidatorResponse<Output[KeyOf<Output>]>;

    if (isValid.valid)
      return this._setValidValue(prop, isValid.validated, operationData);

    this._handleInvalidValue(errorTool, prop, isValid);
  }

  private async _validate<K extends KeyOf<Input & Aliases>>(
    prop: K,
    value: unknown,
    summary_: MutableSummary<Input, Output, CtxOptions>,
  ) {
    if (!this._isInputOrAlias(prop))
      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        value,
        reason: 'Invalid property',
      });

    const isAlias = this._isVirtualAlias(prop),
      propName = (isAlias ? this._getVirtualByAlias(prop) : prop)!,
      allowedValues = this.propsToAllowedValuesMap.get(propName);

    if (allowedValues && !allowedValues.has(value)) {
      const fieldError = makeFieldError(
        this._getNotAllowedError(propName, value),
      );

      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        value,
        reason: fieldError.reason,
        metadata: fieldError.metadata || { allowed: Array.from(allowedValues) },
      });
    }

    const validator = this._getPrimaryValidator(propName as never);

    if (validator) {
      let res: ValidatorResponseObject<(Input & Aliases)[K]>;

      try {
        res = this._sanitizeValidationResponse<(Input & Aliases)[K]>(
          (await validator(
            value,
            summary_ as never,
          )) as ValidatorResponseObject<(Input & Aliases)[K]>,
          value,
        );
      } catch {
        return makeResponse<(Input & Aliases)[K]>({
          valid: false,
          reason: 'validation failed',
        });
      }

      if (allowedValues && res.valid && !allowedValues.has(res.validated))
        return makeResponse<(Input & Aliases)[K]>({
          valid: true,
          validated: value as never,
        });

      return res;
    }

    return makeResponse<(Input & Aliases)[K]>({
      valid: true,
      validated: value as never,
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

    this._updateContext(finalData as never);
    this._updatePartialContext(finalData as never);

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

    const data = this._getFrozenCopy(
      Object.assign({}, this.values, { __getOptions__: () => ctxOptions }),
    );

    getSetValuesAsProps(this.props).map((prop) => {
      const handlers_ = this._getHandlers<NS.DeleteHandler<Output, CtxOptions>>(
        prop,
        'onDelete',
      );

      if (handlers_.length) handlers = handlers.concat(handlers_);
    });

    await Promise.allSettled(
      handlers.map(async (handler) => await handler(data)),
    );
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
          updates[prop] = this.context[prop] as never;
      });

    const finalData = this._useConfigProps(updates, true);

    this._updateContext(finalData as never);
    this._updatePartialContext(finalData as never);

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
  Aliases = never,
  CtxOptions extends ObjectType = never,
  ErrorTool extends IErrorTool<ObjectType> = DefaultErrorTool<
    KeyOf<Input & Aliases>
  >,
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

function areValuesOk(values: unknown) {
  return values && typeof values === 'object';
}

function getValidationFailedResponse(value: unknown) {
  return {
    metadata: null,
    reason: 'validation failed',
    valid: false,
    value,
  } as ValidatorResponseObject<unknown>;
}
