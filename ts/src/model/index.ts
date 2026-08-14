import {
  cloneWithMethods,
  deepCloneValue,
  ErrorTool,
  getDefaultRequiredError,
  getKeysAsProps,
  isEqual,
  isFunctionLike,
  isInputFieldError,
  isRecordLike,
  makeFieldError,
  makeResponse,
  toArray,
} from '../utils';
import type {
  ArrayOfMinSizeTwo,
  ConstantResolverCtx,
  DefaultFieldErrorMetadata,
  InitResolverCtx,
  InputFieldError,
  InternalValidatorResponse,
  InvalidValidatorResponse,
  IvoContext,
  IvoErrorPayload,
  IvoSuccessContext,
  KeyOf,
  NS,
  ObjectType,
  PostValidationConfig,
  PostValidator,
  ReadonlyIvoContext,
  RealType,
  RequiredHandlerRes,
  UpdateResolverCtx,
  ValidatorResponseObject,
} from '../utils/types';

export { Model, ModelTool };

const NotAllowedError = 'value not allowed';
const validationFailedFieldError = makeFieldError('validation failed');

class ModelTool<
  I extends RealType<I>,
  O extends RealType<O>,
  CtxOptions extends ObjectType = {},
  ErrorMetadata = DefaultFieldErrorMetadata,
  ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
> {
  constructor(
    private definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>,
    private options: NS.InternalOptions<
      I,
      O,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    >,
  ) {}

  // contexts & values
  private _isUpdate: boolean = false;
  private _ctxInput: Partial<I> = {};
  private _ctxRawInput: Partial<I> = {};
  private _ctxValues: Partial<O> = {};
  private _ctxPreviousValues: O | null = null;
  private _ctxOptions: CtxOptions = {} as CtxOptions;

  private get isUpdate() {
    return this._isUpdate;
  }

  private get ctxInput() {
    return this._ctxInput;
  }

  private get ctxRawInput() {
    return this._ctxRawInput;
  }

  private get ctxValues() {
    return this._ctxValues;
  }

  private get ctxPreviousValues() {
    return this._ctxPreviousValues;
  }

  private get ctxOptions() {
    return this._ctxOptions;
  }

  async create(input: Partial<I>, options: CtxOptions) {
    if (!areValuesOk(input)) input = {};

    const fieldsCollection = await this._filterInputFieldsAllowed({
      rawInput: input,
      previousValues: null,
      options,
    });

    await this._attachDefaultValues(fieldsCollection);

    const requiredError =
      await this._evaluateMissingRequiredFields(fieldsCollection);

    if (requiredError.hasErrors)
      return this._handleError(requiredError, fieldsCollection);

    const primaryError = await this._validate(fieldsCollection);

    if (primaryError.hasErrors)
      return this._handleError(primaryError, fieldsCollection);

    const secondaryError = await this._reValidate(fieldsCollection);
    if (secondaryError.hasErrors)
      return this._handleError(secondaryError, fieldsCollection);

    const postValidationError = await this._postValidate(fieldsCollection);

    if (postValidationError.hasErrors)
      return this._handleError(postValidationError, fieldsCollection);

    await this._sanitizeVirtuals(fieldsCollection);

    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0)
      collection = await this._resolveDependentChanges(collection);

    this._attachTimestamps();

    const data = (await this._attachConstantValues()) as O;

    return {
      data,
      error: null,
      options: cloneWithMethods(this.ctxOptions),
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
    };
  }

  async delete(data: O, options: CtxOptions) {
    if (!areValuesOk(data)) return;

    let handlers: NS.DeleteHandler<O, CtxOptions>[] = this.options.onDelete
      ? this.options.onDelete
      : [];

    for (const c of Object.values(this.definitions))
      if (c.type !== 'virtual' && c.onDelete)
        handlers = handlers.concat(c.onDelete);

    await Promise.allSettled(
      handlers.map(
        async (h) =>
          await Promise.try(h, Object.freeze(data), Object.freeze(options)),
      ),
    );
  }

  async update(values: O, changes: Partial<I>, options: CtxOptions) {
    const emptyErrorTool = new ErrorTool<ErrorMetadata>();

    // if (!areValuesOk(values) || !areValuesOk(changes))
    //   return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    const fieldsCollection = await this._filterInputFieldsAllowed({
      rawInput: changes,
      previousValues: values,
      options,
    });

    if (!fieldsCollection.relevantConfigNames.size)
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    const requiredError =
      await this._evaluateMissingRequiredFields(fieldsCollection);

    if (requiredError.hasErrors)
      return this._handleUpdateError(requiredError, fieldsCollection);

    const primaryError = await this._validate(fieldsCollection);

    if (primaryError.hasErrors)
      return this._handleUpdateError(primaryError, fieldsCollection);

    if (!this._isValidUpdate(fieldsCollection))
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    const secondaryError = await this._reValidate(fieldsCollection);

    if (secondaryError.hasErrors)
      return this._handleUpdateError(secondaryError, fieldsCollection);

    const postValidationError = await this._postValidate(fieldsCollection);

    if (postValidationError.hasErrors)
      return this._handleUpdateError(postValidationError, fieldsCollection);

    if (!this._isValidUpdate(fieldsCollection))
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    await this._sanitizeVirtuals(fieldsCollection);

    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0)
      collection = await this._resolveDependentChanges(collection);

    const updatedValues = this.ctxValues;
    let hasUpdates = false;

    for (const [fieldName, value] of Object.entries(updatedValues)) {
      // @ts-expect-error ikr
      if (isEqual(value, values[fieldName], this.options.equalityDepth))
        // @ts-expect-error ikr
        delete updatedValues[fieldName];
      else hasUpdates = true;
    }

    this._setCxtValues(updatedValues);

    if (!hasUpdates)
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    const data = this._attachTimestamps();

    return {
      data,
      error: null,
      options: cloneWithMethods(this.ctxOptions),
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
    };
  }

  private _getFieldInfoCollection(): FieldInfoCollection {
    const fields: Map<string, InputFieldInfo> = new Map();

    for (const configName of getKeysAsProps(this.definitions)) {
      const config = this.definitions[configName]!;

      if (config.type === 'constant' || config.type === 'dependent') continue;

      const isVirtual = config.type === 'virtual';

      fields.set(
        configName,
        new InputFieldInfo({ name: configName, configName, isVirtual }),
      );

      const aliasName = (config as { alias?: string }).alias;

      if (aliasName) {
        fields.set(
          aliasName,
          new InputFieldInfo({ name: aliasName, configName, isVirtual }),
        );

        fields.set(
          configName,
          new InputFieldInfo({ name: aliasName, configName, isVirtual }),
        );
      }
    }

    return new FieldInfoCollection(fields);
  }

  private _getReadonlyCtx(): ReadonlyIvoContext<I, O, CtxOptions> {
    const values = this.ctxValues;
    const isUpdate = this.isUpdate,
      changes = isUpdate ? values : null,
      previousValues = isUpdate ? this.ctxPreviousValues : null,
      input = this.ctxInput,
      rawInput = this.ctxRawInput,
      options = this.ctxOptions;

    return isUpdate
      ? Object.freeze({
          get changes() {
            return changes as Partial<O>;
          },
          get input() {
            return input;
          },
          get rawInput() {
            return rawInput;
          },
          get isUpdate() {
            return true as const;
          },
          get previousValues() {
            return previousValues as O;
          },
          get values() {
            return Object.assign({}, previousValues, values);
          },
          get options() {
            return cloneWithMethods(options);
          },
        })
      : Object.freeze({
          get changes() {
            return null;
          },
          get input() {
            return input;
          },
          get rawInput() {
            return rawInput;
          },
          get isUpdate() {
            return false as const;
          },
          get previousValues() {
            return null;
          },
          get values() {
            return values;
          },
          get options() {
            return cloneWithMethods(options);
          },
        });
  }

  private _getContext(): IvoContext<I, O, CtxOptions> {
    const updateOptions = this._updateCtxOptions;

    return Object.freeze(
      Object.defineProperties(
        {} as IvoContext<I, O, CtxOptions>,
        Object.assign(
          Object.getOwnPropertyDescriptors(this._getReadonlyCtx()),
          {
            updateOptions: {
              configurable: true,
              enumerable: true,
              get() {
                return updateOptions;
              },
            },
          },
        ),
      ),
    );
  }

  private _getConstantCtx(): ConstantResolverCtx<I, O, CtxOptions> {
    const input = this.ctxInput,
      options = this.ctxOptions,
      rawInput = this.ctxRawInput,
      updateOptions = this._updateCtxOptions,
      values = this.ctxValues;

    return Object.freeze({
      get input() {
        return input;
      },
      get options() {
        return cloneWithMethods(options);
      },
      get rawInput() {
        return rawInput;
      },
      get updateOptions() {
        return updateOptions;
      },
      get values() {
        return values;
      },
    });
  }

  private _getInitResolverCtx(): InitResolverCtx<I, CtxOptions> {
    const input = this.ctxInput,
      options = this.ctxOptions,
      rawInput = this.ctxRawInput,
      updateOptions = this._updateCtxOptions;

    return Object.freeze({
      get input() {
        return input;
      },
      get rawInput() {
        return rawInput;
      },
      get options() {
        return cloneWithMethods(options);
      },
      get updateOptions() {
        return updateOptions;
      },
    });
  }

  private _getUpdateResolverCtx(): UpdateResolverCtx<I, O, CtxOptions> {
    const input = this.ctxInput,
      options = this.ctxOptions,
      rawInput = this.ctxRawInput,
      previousValues = this.ctxPreviousValues,
      updateOptions = this._updateCtxOptions;

    return Object.freeze({
      get input() {
        return input;
      },
      get rawInput() {
        return rawInput;
      },
      get previousValues() {
        return previousValues!;
      },
      get options() {
        return cloneWithMethods(options);
      },
      get updateOptions() {
        return updateOptions;
      },
    });
  }

  private _setCxtInput = (updates: Partial<I>) => (this._ctxInput = updates);
  private _setCxtValues = (updates: Partial<O>) => (this._ctxValues = updates);

  private _updateCxtInput = (updates: Partial<I>) => {
    Object.assign(this.ctxInput, updates);
  };

  private _updateCxtValues = (updates: Partial<O>) => {
    Object.assign(this.ctxValues, updates);
  };

  private _updateCtxOptions = (options: Partial<CtxOptions>) =>
    Object.assign(this._ctxOptions, options);

  private _cleanInput(
    rawInput: Partial<I>,
    fieldsCollection: FieldInfoCollection,
  ) {
    const input: Partial<I> = {};
    const output: Partial<O> = {};

    for (const [name, value] of Object.entries(rawInput)) {
      const info = fieldsCollection.getUnsafe(name);

      if (!info || info.name !== name) continue;

      // @ts-expect-error ikr
      input[name] = value;

      if (!info.isVirtual) {
        // @ts-expect-error ikr
        output[name] = value;
      }
    }

    return { input, output };
  }

  private _validateAllowedValues(
    configName: string,
    value: unknown,
  ): InternalValidatorResponse<any, ErrorMetadata> {
    const { allow } = this.definitions[configName] as NS.LaxField<
      any,
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >;

    const values = (
      Array.isArray(allow) ? allow : allow?.values
    ) as ArrayOfMinSizeTwo<any>;
    const defaultMetadata = { allowed: values } as never;
    const isValid = values.some((v) =>
      isEqual(v, value, this.options.equalityDepth),
    );

    if (isValid) return makeResponse({ valid: true, validated: value });

    if (Array.isArray(allow))
      return makeResponse({
        valid: false,
        reason: NotAllowedError,
        metadata: defaultMetadata,
      });

    // @ts-expect-error: lol
    const error = allow?.error;

    if (isInputFieldError(error)) return error as never;

    if (isFunctionLike(error)) {
      let message: any;

      try {
        message = error(value, allow?.values);
      } catch {
        return makeResponse({
          valid: false,
          reason: NotAllowedError,
          metadata: defaultMetadata,
        });
      }

      if (typeof message === 'string')
        return makeResponse({
          valid: false,
          reason: message || NotAllowedError,
          metadata: defaultMetadata,
        });

      return isInputFieldError(message)
        ? (message as never)
        : makeResponse({
            valid: false,
            reason: NotAllowedError,
            metadata: defaultMetadata,
          });
    }

    return makeResponse({
      valid: false,
      reason: error || NotAllowedError,
      metadata: defaultMetadata,
    });
  }

  private _handleInvalidValue(
    errorTool: ErrorTool<ErrorMetadata>,
    name: string,
    { reason, metadata }: InvalidValidatorResponse<ErrorMetadata>,
  ) {
    const fieldError = makeFieldError<ErrorMetadata>(
      reason || 'validation failed',
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.set(name, fieldError);
  }

  private async _attachConstantValues() {
    const tasks: [string, NS.ConstantResolver<any, I, O, CtxOptions>][] = [];

    for (const [configName, config] of Object.entries(this.definitions)) {
      if (config.type !== 'constant') continue;

      const value = config.value;

      if (isFunctionLike(value)) {
        tasks.push([configName, value]);
        continue;
      }

      this._updateCxtValues({ [configName]: value } as never);
    }

    const constantValues = (
      await Promise.all(
        tasks.map(
          async ([configName, resolver]) =>
            [
              configName,
              await Promise.try(resolver, this._getConstantCtx()).catch(
                () => null,
              ),
            ] as const,
        ),
      )
    ).reduce(
      (acc, [configName, value]) => {
        // @ts-expect-error ikr
        acc[configName] = value;

        return acc;
      },
      {} as Partial<O>,
    );

    this._updateCxtValues(constantValues);

    return this.ctxValues as O;
  }

  private async _attachDefaultValues(fieldsCollection: FieldInfoCollection) {
    const tasks: [string, NS.DefaultValueResolver<any, I, CtxOptions>][] = [];

    for (const [configName, config] of Object.entries(this.definitions)) {
      if (
        config.type === 'dependent' ||
        (config.type === 'lax' &&
          !fieldsCollection.relevantFieldsProvided.has(configName))
      ) {
        const value = config.default;

        if (isFunctionLike(value)) {
          tasks.push([configName, value]);
          continue;
        }

        this._updateCxtValues({ [configName]: value } as never);
      }
    }

    const defaultValues = (
      await Promise.all(
        tasks.map(
          async ([configName, resolver]) =>
            [
              configName,
              await Promise.try(resolver, this._getInitResolverCtx()).catch(
                () => null,
              ),
            ] as const,
        ),
      )
    ).reduce(
      (acc, [configName, value]) => {
        // @ts-expect-error ikr
        acc[configName] = value;

        return acc;
      },
      {} as Partial<O>,
    );

    this._updateCxtValues(defaultValues);
  }

  private _attachTimestamps(): Partial<O> {
    const values = this.ctxValues;
    const isUpdate = this.isUpdate;

    if (!this.options.timestamps) return values;

    const { createdAt, updatedAt } = this.options.timestamps.getKeys();
    let results = Object.assign({}, values);

    const now = new Date();

    if (updatedAt)
      results = Object.assign(results, {
        [updatedAt]: isUpdate
          ? now
          : this.options.timestamps.isNullable
            ? null
            : now,
      });

    if (!isUpdate && createdAt)
      results = Object.assign(results, { [createdAt]: now });

    this._updateCxtValues(results);

    return results;
  }

  private async _filterInputFieldsAllowed({
    options,
    previousValues: _previousValues,
    rawInput,
  }: {
    rawInput: Partial<I>;
    previousValues: O | null;
    options: CtxOptions;
  }): Promise<FieldInfoCollection> {
    const fieldsCollection = this._getFieldInfoCollection();
    const { input, output } = this._cleanInput(rawInput, fieldsCollection);

    const isUpdate = !!_previousValues;

    this._isUpdate = isUpdate;
    this._ctxInput = input;
    this._ctxValues = output;
    this._ctxRawInput = deepCloneValue(rawInput);
    this._ctxPreviousValues = deepCloneValue(_previousValues);
    this._ctxOptions = cloneWithMethods(options);

    const previousValues: Partial<O> = _previousValues ?? {};
    const tasks: [
      string[] | readonly string[],
      () => boolean | Promise<boolean>,
    ][] = [];

    const fieldsProvided = new Set<string>();
    const relevantFieldsProvided = new Set<string>();

    if (isUpdate) {
      for (const [fieldName, value] of Object.entries(rawInput)) {
        const fieldInfo = fieldsCollection.getUnsafe(fieldName);

        if (!fieldInfo) continue;

        fieldsProvided.add(fieldName);

        const config = this.definitions[
          fieldInfo.configName
        ] as NS.RequiredField<any, I, O, CtxOptions, any>;

        const ignoreUpdate = config.ignoreUpdate;

        if (typeof ignoreUpdate === 'function') {
          tasks.push([
            [fieldName],
            () => ignoreUpdate(this._getUpdateResolverCtx()),
          ]);

          relevantFieldsProvided.add(fieldName);
          continue;
        }

        if (
          ignoreUpdate ||
          (!fieldInfo.isVirtual &&
            isEqual(
              value,
              // @ts-expect-error ikr
              _previousValues[fieldName],
              this.options.equalityDepth,
            ))
        ) {
          // @ts-expect-error ikr
          delete input[fieldName];

          if (!fieldInfo.isVirtual)
            // @ts-expect-error ikr
            delete output[fieldName];

          continue;
        }

        relevantFieldsProvided.add(fieldName);
      }
    } else {
      for (const fieldName of Object.keys(rawInput)) {
        const fieldInfo = fieldsCollection.getUnsafe(fieldName);

        if (!fieldInfo) continue;

        fieldsProvided.add(fieldName);

        const config = this.definitions[fieldInfo.configName] as NS.LaxField<
          any,
          I,
          O,
          CtxOptions,
          any
        >;

        if (config.ignoreInit) {
          // @ts-expect-error ikr
          delete input[fieldName];

          if (!fieldInfo.isVirtual) {
            // @ts-expect-error ikr
            delete output[fieldName];
          }

          continue;
        }

        relevantFieldsProvided.add(fieldName);
      }
    }

    this._setCxtInput(input);
    this._setCxtValues(output);
    fieldsCollection.fieldsProvided = fieldsProvided;
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    for (const fieldName of fieldsProvided.values()) {
      const fieldInfo = fieldsCollection.get(fieldName);
      const configName = fieldInfo?.configName ?? fieldName;

      const config = this.definitions[configName];

      const {
        default: defaultValue,
        ignore,
        ignoreInit,
        ignoreUpdate,
        readonly,
      } = config as NS.LaxField<any, I, O, CtxOptions, any>;

      if (ignore) {
        tasks.push([[fieldName], () => ignore(this._getContext())]);
        continue;
      }

      if (readonly && isUpdate) {
        const hasStaticDefault =
          defaultValue !== undefined && typeof defaultValue !== 'function';

        // readonly with a static default: only allow the update while the
        // previous value still equals that default. Otherwise (no default,
        // e.g. required properties, or a function/async default) the
        // property is permanently locked after creation.
        const stillAllowed =
          hasStaticDefault &&
          // @ts-expect-error ikr
          isEqual(previousValues[fieldName], defaultValue);

        if (!stillAllowed) {
          relevantFieldsProvided.delete(fieldName);

          // @ts-expect-error ikr
          delete input[fieldName];

          // @ts-expect-error ikr
          if (!fieldInfo?.isVirtual) delete output[fieldName];
        }
      }

      const source = isUpdate ? ignoreUpdate : ignoreInit;

      if (isUpdate && config.type === 'required') {
        const ignoreUpdate = config.ignoreUpdate;

        if (!readonly && typeof ignoreUpdate === 'function')
          tasks.push([
            [fieldName],
            () => ignoreUpdate(this._getUpdateResolverCtx()),
          ]);

        continue;
      }

      if (!source) continue;

      relevantFieldsProvided.delete(fieldName);

      // @ts-expect-error ikr
      delete input[fieldName];

      // @ts-expect-error ikr
      if (!fieldInfo.isVirtual) delete output[fieldName];
    }

    const relevantConfigNames = Array.from(
      new Set(
        Array.from(relevantFieldsProvided.values()).map(
          (name) => fieldsCollection.get(name).configName,
        ),
      ),
    );

    if (this.options.ignore)
      for (const config of this.options.ignore) {
        if (typeof config === 'function')
          tasks.push([[], () => config(this._getContext())]);
        else if (config && typeof config === 'object') {
          const fields = config.fields;

          if (fields.some((name) => relevantConfigNames.includes(name)))
            tasks.push([fields, () => config.resolver(this._getContext())]);
        }
      }

    if (isUpdate && this.options.ignoreUpdate)
      for (const config of this.options.ignoreUpdate) {
        if (typeof config === 'function') {
          tasks.push([[], () => config(this._getUpdateResolverCtx())]);

          continue;
        }

        const fields = config.fields;

        if (fields.some((name: string) => relevantConfigNames.includes(name)))
          tasks.push([
            fields,
            () => config.resolver(this._getUpdateResolverCtx()),
          ]);
      }

    this._setCxtInput(input);
    this._setCxtValues(output);

    if (!tasks.length) {
      fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

      return fieldsCollection;
    }

    for (const [configNames, ignore] of await Promise.all(
      tasks.map(
        async ([names, resolver]) => [names, await resolver()] as const,
      ),
    )) {
      if (ignore && !configNames.length) {
        fieldsCollection.clear();

        return fieldsCollection;
      }

      for (const configName of configNames) {
        const fieldInfo = fieldsCollection.get(configName);
        const fieldName = fieldInfo.name;

        if (ignore) {
          // @ts-expect-error ikr
          delete input[fieldName];

          relevantFieldsProvided.delete(fieldName);

          // @ts-expect-error ikr
          if (!fieldInfo.isVirtual) delete output[fieldName];
        }
      }
    }

    this._setCxtInput(input);
    this._setCxtValues(output);
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    return fieldsCollection;
  }

  private async _evaluateMissingRequiredFields(
    fieldsCollection: FieldInfoCollection,
  ): Promise<ErrorTool<ErrorMetadata>> {
    const errorTool = new ErrorTool<ErrorMetadata>();

    type Res = [string, RequiredHandlerRes<ErrorMetadata>][];

    const handlers: (() => Res | Promise<Res>)[] = [];

    for (const [configName, config] of Object.entries(this.definitions)) {
      if (config.type === 'constant' || config.type === 'dependent') continue;

      if (fieldsCollection.relevantConfigNames.has(configName)) continue;

      if (config.type === 'required' && !this.isUpdate) {
        let error: string;

        if (typeof config.requiredError === 'function') {
          try {
            let customError = config.requiredError(this._getInitResolverCtx());

            if (typeof customError === 'string') {
              customError = customError.trim();

              if (customError) error = customError;
            }
          } finally {
            error ??= getDefaultRequiredError(configName);
          }
        } else {
          error = config.requiredError ?? getDefaultRequiredError(configName);
        }

        errorTool.set(configName, makeFieldError(error, configName));

        continue;
      }

      if (
        this.isUpdate &&
        config.type === 'lax' &&
        config.readonly &&
        !isEqual(
          config.default,
          // @ts-expect-error
          this.ctxPreviousValues[config.name],
          this.options.equalityDepth,
        )
      ) {
        continue;
      }

      // @ts-expect-error ikr
      const { alias, required } = config;

      if (required)
        handlers.push(async () => [
          [alias ?? configName, await required(this._getContext())],
        ]);
    }

    if (this.options.required) {
      for (const { fields, handler } of this.options.required) {
        if (
          fields.some((configName) =>
            fieldsCollection.relevantConfigNames.has(configName),
          )
        )
          continue;

        handlers.push(async () => {
          const result = await handler(this._getContext());

          if (!result) return [];

          const errors: Res = [];

          for (const [fieldName, value] of Object.entries(result)) {
            const fieldInfo = fieldsCollection.getUnsafe(fieldName);

            if (fieldInfo && fields.includes(fieldInfo.configName))
              // @ts-expect-error
              errors.push([fieldInfo.name, value]);
          }

          return errors;
        });
      }
    }

    const results = await Promise.allSettled(
      toArray(handlers).map((h) => Promise.try(h)),
    );

    for (const r of results) {
      if (r.status !== 'fulfilled' || !r.value) continue;

      for (const [fieldName, err] of r.value) {
        if (!err) continue;

        if (typeof err === 'string') {
          errorTool.set(fieldName, makeFieldError<ErrorMetadata>(err));

          continue;
        }

        if (err === true) {
          errorTool.set(
            fieldName,
            makeFieldError<ErrorMetadata>(getDefaultRequiredError(fieldName)),
          );

          continue;
        }

        if (Array.isArray(err)) {
          const [isRequired, reason] = err;

          if (isRequired)
            errorTool.set(
              fieldName,
              makeFieldError<ErrorMetadata>(
                isInputFieldError(reason)
                  ? reason
                  : (typeof reason === 'string' ? reason.trim() : '') ||
                      getDefaultRequiredError(fieldName),
              ),
            );
        }
      }
    }

    return errorTool;
  }

  private _isValidUpdate(fieldsCollection: FieldInfoCollection): boolean {
    const input: Partial<I> = this.ctxInput;
    const updates: Partial<O> = this.ctxValues;
    const previousPartial: O = this.ctxPreviousValues!;

    const relevantFieldsProvided = new Set<string>();

    for (const fieldName of fieldsCollection.relevantFieldsProvided) {
      const fieldInfo = fieldsCollection.get(fieldName);

      if (fieldInfo.isVirtual) {
        relevantFieldsProvided.add(fieldName);
        continue;
      }

      if (
        isEqual(
          (updates as any)[fieldName],
          (previousPartial as any)[fieldName],
          this.options.equalityDepth,
        )
      ) {
        // Drop unchanged output field
        delete (input as any)[fieldName];
        delete (updates as any)[fieldName];
      } else relevantFieldsProvided.add(fieldName);
    }

    for (const [fieldName, value] of Object.entries(updates))
      if (
        isEqual(
          value,
          (previousPartial as any)[fieldName],
          this.options.equalityDepth,
        )
      ) {
        // Drop unchanged output field
        delete (updates as any)[fieldName];
      }

    this._setCxtInput(input);
    this._setCxtValues(updates);
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    return relevantFieldsProvided.size > 0;
  }

  private async _validate(
    fieldsCollection: FieldInfoCollection,
  ): Promise<ErrorTool<ErrorMetadata>> {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    await Promise.allSettled(
      Array.from(fieldsCollection.relevantFieldsProvided).map(
        async (fieldName) => {
          const fieldInfo = fieldsCollection.get(fieldName);
          const configName = fieldInfo.configName;

          const { allow, validator } = this.definitions[
            configName
          ] as NS.LaxField<any, I, O, CtxOptions, ErrorMetadata>;

          let isValid: ValidatorResponseObject<any, ErrorMetadata> | null =
            null;
          const rawValue = (ctx.input as any)[fieldName];

          if (allow)
            isValid = this._sanitizeValidationResponse<any>(
              this._validateAllowedValues(configName, rawValue),
              rawValue,
            );
          else if (validator) {
            const result = await Promise.try(validator, rawValue, ctx).catch(
              () => ({ valid: false, reason: 'validation failed' }),
            );

            isValid = this._sanitizeValidationResponse<any>(
              typeof result === 'boolean' ? { valid: result } : result,
              rawValue,
            );
          }

          if (!isValid) return;

          if (!isValid.valid)
            return this._handleInvalidValue(errorTool, fieldName, isValid);

          let { validated } = isValid;

          if (isEqual(validated, undefined)) validated = rawValue;

          const ctxUpdate = { [fieldName]: validated } as never;

          if (!fieldInfo.isVirtual) this._updateCxtValues(ctxUpdate);

          this._updateCxtInput(ctxUpdate);
        },
      ),
    );

    return errorTool;
  }

  private async _reValidate(fieldsCollection: FieldInfoCollection) {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    await Promise.allSettled(
      fieldsCollection.relevantFieldsProvided
        .values()
        .map(async (fieldName) => {
          const fieldInfo = fieldsCollection.get(fieldName);
          const configName = fieldInfo.configName;
          const { reValidator } = this.definitions[configName] as NS.LaxField<
            any,
            any,
            any,
            any,
            any
          >;

          if (!reValidator) return;

          // @ts-expect-error ikr
          const value = ctx.input?.[fieldName] as never as O[KeyOf<O>];

          let isValid: ValidatorResponseObject<unknown, ErrorMetadata>;

          try {
            isValid = this._sanitizeValidationResponse<unknown>(
              (await Promise.try(
                reValidator,
                value,
                ctx,
              )) as ValidatorResponseObject<unknown, ErrorMetadata>,
              value,
            );
          } catch {
            isValid = makeResponse<unknown, ErrorMetadata>({
              valid: false,
              reason: 'validation failed',
            });
          }

          if (!isValid.valid)
            return this._handleInvalidValue(errorTool, fieldName, isValid);

          const { validated } = isValid;

          if (
            !isEqual(validated, undefined) &&
            !isEqual(validated, value, this.options.equalityDepth)
          ) {
            const ctxUpdate = { [fieldName]: validated } as never;

            if (!fieldInfo.isVirtual) this._updateCxtValues(ctxUpdate);

            this._updateCxtInput(ctxUpdate);
          }
        }),
    );

    return errorTool;
  }

  private async _postValidate(fieldsCollection: FieldInfoCollection) {
    const ctx = this._getContext();

    const errorTool = new ErrorTool<ErrorMetadata>();

    const validators: [
      string[],
      PostValidationConfig<any, I, O, CtxOptions, ErrorMetadata>['validator'],
    ][] = [];

    if (this.options.postValidate)
      for (const { fields, validator } of this.options.postValidate) {
        if (
          fields.some((name) => fieldsCollection.relevantConfigNames.has(name))
        )
          // @ts-expect-error ikr
          validators.push([fields, validator]);
      }

    const handlers = Array.from(validators).map(([fields, validator]) => {
      return {
        fields,
        validator,
      };
    });

    const handleRevalidatedData = (revalidatedData: Partial<O> | null) => {
      if (!revalidatedData) return;

      for (const fieldName of getKeysAsProps(revalidatedData)) {
        const fieldInfo = fieldsCollection.getUnsafe(fieldName);

        const validated = revalidatedData[fieldName];
        if (!fieldInfo || isEqual(validated, undefined)) continue;

        if (
          // @ts-expect-error ikr
          isEqual(validated, ctx.input[fieldName], this.options.equalityDepth)
        )
          return;

        const ctxUpdate = { [fieldName]: validated } as never;

        this._updateCxtInput(ctxUpdate);

        if (!fieldInfo.isVirtual) this._updateCxtValues(ctxUpdate);
        fieldsCollection.appendRelevantFieldProvided(fieldName);
      }
    };

    await Promise.allSettled(
      handlers.map(async ({ validator, fields }) => {
        const configNamesOfFieldsProvided = fields.filter((name) =>
          fieldsCollection.relevantConfigNames.has(name),
        );

        if (!Array.isArray(validator)) {
          const { revalidatedData, success } = await Promise.try(() =>
            this._handlePostValidator({
              configNamesOfFieldsProvided,
              errorTool,
              fields,
              fieldsCollection,
              // @ts-expect-error ikr
              validator,
            }),
          );

          if (!success || !revalidatedData) return;

          return handleRevalidatedData(revalidatedData);
        }

        for (const v1 of validator) {
          if (Array.isArray(v1)) {
            const results = await Promise.all(
              v1.map(async (validator) => {
                const res = await Promise.try(() =>
                  this._handlePostValidator({
                    configNamesOfFieldsProvided,
                    errorTool,
                    fields,
                    fieldsCollection,
                    validator,
                  }),
                );

                handleRevalidatedData(res.revalidatedData);

                return res;
              }),
            );

            if (results.some((r) => r.success === false)) break;

            continue;
          }

          const { revalidatedData, success } = await this._handlePostValidator({
            configNamesOfFieldsProvided,
            errorTool,
            fields,
            fieldsCollection,
            validator: v1 as any,
          });

          if (!success) break;

          if (revalidatedData) handleRevalidatedData(revalidatedData);
        }
      }),
    );

    return errorTool;
  }

  private async _handlePostValidator({
    configNamesOfFieldsProvided,
    errorTool,
    fields,
    fieldsCollection,
    validator,
  }: {
    errorTool: ErrorTool<ErrorMetadata>;
    configNamesOfFieldsProvided: string[];
    fields: string[];
    fieldsCollection: FieldInfoCollection;
    validator: PostValidator<any, I, O, CtxOptions, ErrorMetadata>;
  }) {
    const revalidatedData: Partial<O> = {};

    try {
      const res = await Promise.try(validator, this._getContext());

      if (!isRecordLike(res)) return { revalidatedData: null, success: true };

      for (const [fieldName, value] of Object.entries(res)) {
        const info = fieldsCollection.getUnsafe(fieldName);

        if (!info || !fields.includes(info.configName)) continue;

        if (typeof value === 'object' && 'validated' in (value as any)) {
          // @ts-expect-error ikr
          revalidatedData[fieldName] = (value as any).validated;

          continue;
        }

        let error: string | InputFieldError<ErrorMetadata> =
          'validation failed';

        if (isInputFieldError<ErrorMetadata>(value)) error = value;

        if (typeof value === 'string') {
          const message = value.trim();

          if (message.length) error = message;
        }

        errorTool.set(info.name, makeFieldError(error));
      }
    } catch {
      for (const configName of configNamesOfFieldsProvided) {
        const info = fieldsCollection.getUnsafe(configName);

        if (info)
          // @ts-expect-error ikr
          errorTool.set(info.name, validationFailedFieldError);
      }
    }

    const success = !errorTool.hasErrors;

    return {
      revalidatedData:
        !success || !Object.keys(revalidatedData).length
          ? null
          : revalidatedData,
      success,
    };
  }

  private async _sanitizeVirtuals(fieldsCollection: FieldInfoCollection) {
    const sanitizers: [
      string,
      NS.VirtualResolver<unknown, I, O, CtxOptions>,
    ][] = [];

    // `relevantFieldsProvided` is narrowed to output fields only (see its
    // NS.setter), so virtuals — the only fields that can have a `sanitizer` —
    // never appear there. `relevantConfigNames` is the unfiltered, config-
    // name-mapped equivalent.
    for (const configName of fieldsCollection.relevantConfigNames) {
      const fieldInfo = fieldsCollection.get(configName);
      // @ts-expect-error ikr
      const sanitizer = this.definitions[fieldInfo.configName].sanitizer;

      if (sanitizer) sanitizers.push([fieldInfo.name, sanitizer]);
    }

    await Promise.allSettled(
      sanitizers.map(async ([fieldName, sanitizer]) => {
        await Promise.try(sanitizer, this._getContext()).then(
          (resolvedValue) => {
            if (
              !isEqual(
                resolvedValue,
                // @ts-expect-error ikr
                this._ctxInput[fieldName],
                this.options.equalityDepth,
              )
            )
              this._updateCxtInput({ [fieldName]: resolvedValue } as never);
          },
        );
      }),
    );
  }

  private _makeHandleFailure(fieldsCollection: FieldInfoCollection) {
    const ctx = this._getReadonlyCtx();

    let cleanups: NS.FailureHandler<I, O, CtxOptions>[] = [];

    for (const fieldName of fieldsCollection.fieldsProvided) {
      const info = fieldsCollection.getUnsafe(fieldName);
      if (!info) continue;

      const config = this.definitions[info.configName] as NS.LaxField<
        any,
        I,
        O,
        CtxOptions,
        ErrorMetadata
      >;

      if (config.onFailure) cleanups = cleanups.concat(config.onFailure);
    }

    return async () => {
      await Promise.allSettled(cleanups.map((h) => Promise.try(h, ctx)));
    };
  }

  private _makeHandleSuccess(fieldsCollection: FieldInfoCollection) {
    const relevantFields = new Set<string>(
      fieldsCollection.relevantConfigNames,
    );

    for (const name of Object.keys(this.ctxValues)) relevantFields.add(name);

    let successListeners = [] as NS.SuccessHandler<I, O, CtxOptions>[];

    for (const configName of relevantFields) {
      const config = this.definitions[configName];

      if (config?.onSuccess)
        successListeners = successListeners.concat(config.onSuccess);
    }

    if (this.options.onSuccess)
      for (const config of this.options.onSuccess) {
        if (!config) continue;

        if (typeof config === 'function') {
          successListeners = successListeners.concat(config);
          continue;
        }

        if (
          config.fields.length === 0 ||
          config.fields.some((name) => relevantFields.has(name))
        )
          successListeners = successListeners.concat(config.resolver);
      }

    return async () => {
      await Promise.allSettled(
        successListeners.map(
          async (h) =>
            await Promise.try(
              h,
              this._getReadonlyCtx() as IvoSuccessContext<I, O, CtxOptions>,
            ),
        ),
      );
    };
  }

  private async _resolveDependentChanges(
    fieldsCollection: FieldInfoCollection,
  ) {
    const ctx = this._getContext();

    const isUpdate = ctx.isUpdate;
    const toResolve = new Set<string>();

    for (const configName in this.definitions) {
      const config = this.definitions[configName];

      if (
        config.type === 'dependent' &&
        toArray(config.dependsOn).some((parent) =>
          fieldsCollection.relevantDependentConfigNames.has(parent),
        )
      )
        toResolve.add(configName);
    }

    const fieldsResolved = new Set<string>();
    const values = this.ctxValues;
    const prevValues = this.ctxPreviousValues ?? {};

    await Promise.allSettled(
      toResolve.values().map(async (name) => {
        const config = this.definitions[name] as NS.DependentField<
          any,
          I,
          O,
          CtxOptions
        >;

        const isReadonly =
          config.readonly && typeof config.default !== 'function';

        if (
          isUpdate &&
          isReadonly &&
          // @ts-expect-error ikr
          !isEqual(config.default, prevValues[name], this.options.equalityDepth)
        )
          return;

        const value = await Promise.try(config.resolver, ctx).catch(() =>
          // @ts-expect-error ikr
          isUpdate ? prevValues[name] : null,
        );

        if (
          // @ts-expect-error ikr
          !isEqual(value, values[name], this.options.equalityDepth)
        ) {
          fieldsResolved.add(name);
          this._updateCxtValues({ [name]: value } as never);
        }
      }),
    );

    return fieldsCollection.newWithResolvedDependentFields(fieldsResolved);
  }

  private _sanitizeValidationResponse<T>(
    response: ValidatorResponseObject<T, ErrorMetadata>,
    value: unknown,
  ): ValidatorResponseObject<T, ErrorMetadata> {
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

    const _response: InvalidValidatorResponse<ErrorMetadata> = {
      valid: false,
      value,
    } as never;

    if (response?.reason && typeof response?.reason === 'string')
      _response.reason = response.reason;

    _response.metadata = response.metadata ?? null;

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

  private _handleError(
    errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>,
    fieldsCollection: FieldInfoCollection,
  ) {
    return {
      data: null,
      error: this.options.sanitizeError(errorTool.payload, this.ctxOptions),
      options: cloneWithMethods(this.ctxOptions),
      handleFailure: this._makeHandleFailure(fieldsCollection),
      handleSuccess: null,
    };
  }

  private _handleUpdateError(
    errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>,
    fieldsCollection: FieldInfoCollection,
  ) {
    const options = this._getReadonlyCtx().options;
    const isNothingToUpdate = !errorTool.hasErrors;

    if (isNothingToUpdate) this._setCxtValues({});

    return {
      data: null,
      error: isNothingToUpdate
        ? ({ isNothingToUpdate, payload: null } as const)
        : ({
            isNothingToUpdate: false,
            payload: this.options.sanitizeError(errorTool.payload, options),
          } as const),
      options,
      handleFailure: this._makeHandleFailure(fieldsCollection),
      handleSuccess: null,
    };
  }
}

class Model<
  Input extends RealType<Input>,
  Output extends RealType<Output>,
  CtxOptions extends ObjectType = never,
  ErrorMetadata = DefaultFieldErrorMetadata,
  ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
> {
  constructor(
    private modelFactory: () => ModelTool<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    >,
  ) {}

  create = (values: Partial<Input>, options: CtxOptions) =>
    this.modelFactory().create(values, options);

  delete = (values: Output, options: CtxOptions) =>
    this.modelFactory().delete(values, options);

  update = (values: Output, changes: Partial<Input>, options: CtxOptions) =>
    this.modelFactory().update(values, changes, options);
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
  } as ValidatorResponseObject<unknown, unknown>;
}

class FieldInfoCollection {
  private _fieldsProvided: Set<string> = new Set();
  private _relevantFieldsProvided: Set<string> = new Set();
  private _relevantDependentConfigNames: Set<string> = new Set();
  private _relevantConfigNames: Set<string> = new Set();

  constructor(private _fields: Map<string, InputFieldInfo>) {}

  get(fieldName: string): InputFieldInfo {
    return this._fields.get(fieldName)!;
  }

  getUnsafe(fieldName: string): InputFieldInfo | null {
    return this._fields.get(fieldName) ?? null;
  }

  get fieldsProvided() {
    return this._fieldsProvided;
  }

  set fieldsProvided(value: Set<string>) {
    this._fieldsProvided = value;
  }

  get relevantFieldsProvided() {
    return this._relevantFieldsProvided;
  }

  set relevantFieldsProvided(names: Set<string>) {
    const configNames = new Set<string>();

    for (const field_name of names)
      configNames.add(this.get(field_name).configName);

    this._relevantConfigNames = configNames;
    this._relevantFieldsProvided = names;
  }

  appendRelevantFieldProvided(name: string) {
    this._relevantConfigNames.add(this.get(name).configName);
    this._relevantFieldsProvided.add(name);
  }

  clonedFromRelevantFieldsProvided() {
    const col = new FieldInfoCollection(this._fields);

    col._relevantDependentConfigNames = new Set(this._relevantConfigNames);

    return col;
  }

  newWithResolvedDependentFields(names: Set<string>) {
    const col = new FieldInfoCollection(this._fields);

    col._relevantDependentConfigNames = names;

    return col;
  }

  clear() {
    this._relevantConfigNames.clear();
    this._relevantFieldsProvided.clear();
    this._relevantDependentConfigNames.clear();
  }

  get relevantDependentConfigNames() {
    return this._relevantDependentConfigNames;
  }

  get relevantConfigNames() {
    return this._relevantConfigNames;
  }

  set relevantConfigNames(configNames: Set<string>) {
    this._relevantConfigNames = configNames;
  }
}

class InputFieldInfo {
  name: string;
  configName: string;
  isVirtual: boolean;

  constructor({
    name,
    configName,
    isVirtual,
  }: {
    name: string;
    configName: string;
    isVirtual: boolean;
  }) {
    this.name = name;
    this.configName = configName;
    this.isVirtual = isVirtual;
  }
}
