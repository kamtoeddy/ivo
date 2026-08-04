import {
  cloneWithMethods,
  deepCloneValue,
  ErrorTool,
  getKeysAsProps,
  isEqual,
  isFunctionLike,
  isInputFieldError,
  isRecordLike,
  makeFieldError,
  makeResponse,
  toArray,
} from "../utils";
import {
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
  PostValidator,
  ReadonlyIvoContext,
  RealType,
  RequiredHandler,
  UpdateResolverCtx,
  ValidatorResponseObject,
} from "../utils/types";

export { type FieldMaker, Model, ModelTool, Schema };

const NotAllowedError = "value not allowed";
const validationFailedFieldError = makeFieldError("validation failed");

class ModelTool<
  I extends RealType<I>,
  O extends RealType<O>,
  CtxOptions extends ObjectType = {},
  ErrorMetadata = DefaultFieldErrorMetadata,
  ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
> extends SchemaCore<I, O, CtxOptions, ErrorMetadata, ErrorPayload> {
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

  constructor(schema: Schema<I, O, CtxOptions, ErrorMetadata, ErrorPayload>) {
    super(schema.definitions as never, schema.options as never);
  }

  async create(input: Partial<I>, options: CtxOptions) {
    if (!areValuesOk(input)) input = {};

    this._initContext({
      isUpdate: false,
      rawInput: this._cleanInput(input),
      previousValues: null,
      options,
    });

    const fieldsCollection = await this._filterInputFieldsAllowed();

    await this._attachDefaultValues(fieldsCollection);

    const requiredError =
      await this._evaluateMissingRequiredFields(fieldsCollection);
    if (requiredError.hasErrors) return this._handleError(requiredError);

    const primaryError = await this._runPrimaryValidators(fieldsCollection);

    if (primaryError.hasErrors) return this._handleError(primaryError);

    const secondaryError =
      await this._handleSecondaryValidations(fieldsCollection);
    if (secondaryError.hasErrors) return this._handleError(secondaryError);

    const postValidationError =
      await this._handlePostValidations(fieldsCollection);

    if (postValidationError.hasErrors)
      return this._handleError(postValidationError);

    await this._handleSanitizationOfVirtuals(fieldsCollection);

    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0)
      collection = await this._resolveDependentChanges(collection);

    this._attachTimestamps();

    const data = (await this._attachConstantValues()) as O;

    return {
      data,
      error: null,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
      options: this._getReadonlyCtx().options,
    };
  }

  async delete(data: O, options: CtxOptions) {
    if (!areValuesOk(data)) return;

    let handlers: NS.DeleteHandler<O, CtxOptions>[] = this._options.onDelete
      ? toArray(this._options.onDelete)
      : [];

    for (const prop of getSetValuesAsProps(this.props)) {
      const handlers_ = this._getHandlers<NS.DeleteHandler<O, CtxOptions>>(
        prop,
        "onDelete",
      );

      if (handlers_.length) handlers = handlers.concat(handlers_);
    }

    await Promise.allSettled(
      handlers.map(
        async (h) =>
          await Promise.try(
            h,
            this._getFrozenCopy(data),
            this._getFrozenCopy(options),
          ),
      ),
    );
  }

  async update(values: O, changes: Partial<I>, options: CtxOptions) {
    const emptyErrorTool = new ErrorTool<ErrorMetadata>();

    if (!areValuesOk(values) || !areValuesOk(changes))
      return this._handleUpdateError(emptyErrorTool);

    this._initContext({
      isUpdate: true,
      rawInput: this._cleanInput(changes),
      previousValues: values,
      options,
    });

    const fieldsCollection = await this._filterInputFieldsAllowed();

    if (!fieldsCollection.relevantConfigNames.size)
      return this._handleUpdateError(emptyErrorTool);

    const requiredError =
      await this._evaluateMissingRequiredFields(fieldsCollection);

    if (requiredError.hasErrors) return this._handleUpdateError(requiredError);

    const primaryError = await this._runPrimaryValidators(fieldsCollection);

    if (primaryError.hasErrors) return this._handleUpdateError(primaryError);

    fieldsCollection.relevantFieldsProvided =
      this._evaluateUpdateValidity(fieldsCollection);

    if (!fieldsCollection.relevantFieldsProvided.size)
      return this._handleUpdateError(emptyErrorTool);

    const secondaryError =
      await this._handleSecondaryValidations(fieldsCollection);

    if (secondaryError.hasErrors)
      return this._handleUpdateError(secondaryError);

    const postValidationError =
      await this._handlePostValidations(fieldsCollection);

    if (postValidationError.hasErrors)
      return this._handleUpdateError(postValidationError);

    await this._handleSanitizationOfVirtuals(fieldsCollection);

    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0)
      collection = await this._resolveDependentChanges(collection);

    const data = this._attachTimestamps();

    return {
      data,
      error: null,
      options: this._getReadonlyCtx().options,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
    };
  }

  private _getFieldInfoCollection(): FieldInfoCollection {
    const fields: Map<string, InputFieldInfo> = new Map();

    for (const configName of getKeysAsProps(this._definitions)) {
      const config = this._definitions[configName]!;

      if (config.type === "constant" || config.type === "dependent") continue;

      const isVirtual = config.type === "virtual";

      fields.set(
        configName,
        new InputFieldInfo({ name: configName, configName, isVirtual }),
      );

      const aliasName = (config as { alias?: string }).alias;

      if (aliasName)
        fields.set(
          aliasName,
          new InputFieldInfo({ name: aliasName, configName, isVirtual }),
        );
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
      ? this._getFrozenCopy({
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
            return Object.assign({}, previousValues, values);
          },
          get values() {
            return values as O;
          },
          get options() {
            return cloneWithMethods(options);
          },
        })
      : this._getFrozenCopy({
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

    return this._getFrozenCopy(
      Object.assign({}, this._getReadonlyCtx(), {
        get updateOptions() {
          return updateOptions;
        },
      }),
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

  private _initContext(props: {
    isUpdate: boolean;
    rawInput: Partial<I>;
    previousValues: O | null;
    options: CtxOptions;
  }) {
    this._isUpdate = props.isUpdate;
    this._ctxInput = {};
    this._ctxValues = {};
    this._ctxRawInput = deepCloneValue(props.rawInput); // TODO: cleanup and filter out non schema fields
    this._ctxPreviousValues = deepCloneValue(props.previousValues); // TODO: cleanup and filter out non schema fields
    this._ctxOptions = cloneWithMethods(props.options);
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

  private _cleanInput(input: Partial<I>) {
    const props = getKeysAsProps(input).filter(this._isInputOrAlias);
    const values: Partial<I> = {};

    for (const prop of props) values[prop] = input[prop] as never;

    return values;
  }

  private _getPrimaryValidator = (prop: string) => {
    const { validator } = this._getDefinition(prop) as NS.LaxField<
      any,
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >;

    return validator;
  };

  private _getSecondaryValidator = (prop: string) => {
    const { reValidator } = this._getDefinition(prop) as NS.LaxField<
      any,
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >;

    return reValidator;
  };

  private _validateAllowedValues(
    prop: string,
    value: unknown,
  ): InternalValidatorResponse<any, ErrorMetadata> {
    const { allow } = this._getDefinition(prop) as NS.LaxField<
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

    const isValid = !!values.find((v) =>
      isEqual(v, value, this._options.equalityDepth),
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

      if (typeof message === "string")
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
    validationResponse: InvalidValidatorResponse<ErrorMetadata>,
  ) {
    const { reason, metadata } = validationResponse;

    const fieldError = makeFieldError<ErrorMetadata>(
      reason || "validation failed",
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.set(name, fieldError);
  }

  private async _attachConstantValues() {
    const tasks: [string, NS.ConstantResolver<any, I, O, CtxOptions>][] = [];

    for (const [configName, config] of Object.entries(this._definitions)) {
      if (config.type !== "constant") continue;

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
    ).reduce((acc, [configName, value]) => {
      // @ts-expect-error ikr
      acc[configName] = value;

      return acc;
    }, {} as Partial<O>);

    this._updateCxtValues(constantValues);

    return this.ctxValues as O;
  }

  private async _attachDefaultValues(fieldsCollection: FieldInfoCollection) {
    const tasks: [string, NS.DefaultValueResolver<any, I, CtxOptions>][] = [];

    for (const [configName, config] of Object.entries(this._definitions)) {
      if (
        config.type === "dependent" ||
        (config.type === "lax" &&
          !fieldsCollection.relevantFieldsProvided.has(configName))
      ) {
        const value = config.default;

        if (isFunctionLike(value)) {
          tasks.push([configName, value]);
          continue;
        }

        return this._updateCxtValues({ [configName]: value } as never);
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
    ).reduce((acc, [configName, value]) => {
      // @ts-expect-error ikr
      acc[configName] = value;

      return acc;
    }, {} as Partial<O>);

    this._updateCxtValues(defaultValues);
  }

  private _attachTimestamps(): Partial<O> {
    const values = this.ctxValues;
    const isUpdate = this.isUpdate;

    if (!this.timestampTool.withTimestamps) values;

    const { createdAt, updatedAt } = this.timestampTool.getKeys();
    let results = Object.assign({}, values);

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

    this._updateCxtValues(results);

    return results;
  }

  private async _filterInputFieldsAllowed(): Promise<FieldInfoCollection> {
    const _previousValues = this.ctxPreviousValues;
    const fieldsCollection = this._getFieldInfoCollection();
    const rawInput: Partial<I> = this.ctxRawInput;
    const isUpdate = !!_previousValues;
    const previousValues: Partial<O> = _previousValues
      ? deepCloneValue(_previousValues)
      : {};
    const entityResolvers: NS.IgnoreUpdateResolver<I, O, CtxOptions>[] = [];
    const input = deepCloneValue(rawInput);
    const output: Partial<O> = {};
    const fieldsProvided = new Set<string>();
    const relevantFieldsProvided = new Set<string>();

    if (isUpdate) {
      for (const config of toArray(this._options.ignoreUpdate ?? [])) {
        if (typeof config === "function") entityResolvers.push(config);
        else if (config.fields.length === 0)
          entityResolvers.push(config.resolver);
      }

      for (const [fieldName, value] of Object.entries(rawInput)) {
        const fieldInfo = fieldsCollection.get(fieldName);

        if (
          fieldInfo.isVirtual ||
          !isEqual(
            value,
            // @ts-expect-error ikr
            _previousValues[fieldName],
            this._options.equalityDepth,
          )
        ) {
          relevantFieldsProvided.add(fieldName);

          if (!fieldInfo.isVirtual) {
            // @ts-expect-error ikr
            output[fieldName] = input[fieldName];
          }
        } else {
          // @ts-expect-error ikr
          delete input[fieldName];
        }

        fieldsProvided.add(fieldName);
      }
    } else {
      for (const fieldName of Object.keys(rawInput)) {
        const fieldInfo = fieldsCollection.get(fieldName);

        if (!fieldInfo.isVirtual) {
          // @ts-expect-error ikr
          output[fieldName] = input[fieldName];
        }

        fieldsProvided.add(fieldName);
        relevantFieldsProvided.add(fieldName);
      }
    }

    if (entityResolvers.length)
      for (const task of await Promise.allSettled(
        entityResolvers.map((resolver) =>
          Promise.try(resolver, this._getUpdateResolverCtx()).catch(
            () => false,
          ),
        ),
      )) {
        // if "task.value" is positive, it means "ignore"
        if (task.status === "fulfilled" && task.value) return fieldsCollection;
      }

    fieldsCollection.fieldsProvided = fieldsProvided;
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    this._updateCxtInput(input);
    this._updateCxtValues(output);

    const tasks: [
      string[] | readonly string[],
      () => boolean | Promise<boolean>,
    ][] = [];

    for (const fieldName of fieldsProvided.values()) {
      const fieldInfo = fieldsCollection.get(fieldName);
      const configName = fieldInfo?.configName ?? fieldName;

      const {
        default: defaultValue,
        ignore,
        ignoreInit,
        ignoreUpdate,
        readonly,
        type,
      } = this._getDefinition(configName) as NS.LaxField<
        any,
        I,
        O,
        CtxOptions,
        any
      >;

      if (ignore) {
        tasks.push([[fieldName], () => ignore(this._getContext())]);
        continue;
      }

      if (readonly && isUpdate) {
        const hasStaticDefault =
          defaultValue !== undefined && typeof defaultValue !== "function";

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
          if (fieldInfo?.isOutput) delete output[fieldName];

          continue;
        }
      }

      const source = isUpdate ? ignoreUpdate : ignoreInit;

      // @ts-expect-error ikr
      if (isUpdate && type === "required") {
        if (!readonly && typeof source === "function")
          tasks.push([[fieldName], () => source(this._getUpdateResolverCtx())]);

        continue;
      }

      if (!source) continue;

      if (typeof source === "function") {
        tasks.push([
          [fieldName],
          isUpdate
            ? () => source(this._getUpdateResolverCtx())
            : // @ts-expect-error ikr
              () => source(this._getInitResolverCtx()),
        ]);

        continue;
      }

      relevantFieldsProvided.delete(fieldName);

      // @ts-expect-error ikr
      delete input[fieldName];

      // @ts-expect-error ikr
      if (fieldInfo.isOutput) delete output[fieldName];
    }

    const relevantConfigNames = Array.from(
      new Set(
        Array.from(relevantFieldsProvided.values()).map(
          (name) => fieldsCollection.get(name).configName,
        ),
      ),
    );

    for (const config of toArray(this._options.ignore ?? [])) {
      if (typeof config === "function") {
        tasks.push([relevantConfigNames, () => config(this._getContext())]);
      } else if (config && typeof config === "object") {
        const fields = config.fields;

        if (fields.some((name: string) => relevantConfigNames.includes(name)))
          tasks.push([fields, () => config.resolver(this._getContext())]);
      }
    }

    if (isUpdate) {
      for (const config of toArray(this._options.ignoreUpdate ?? [])) {
        if (typeof config === "function") {
          tasks.push([
            relevantConfigNames,
            () => config(this._getUpdateResolverCtx()),
          ]);

          continue;
        }

        const fields = config.fields;

        if (fields.some((name: string) => relevantConfigNames.includes(name)))
          tasks.push([
            fields,
            () => config.resolver(this._getUpdateResolverCtx()),
          ]);
      }
    }

    if (!tasks.length) {
      this._setCxtInput(input);
      this._setCxtValues(output);

      fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

      return fieldsCollection;
    }

    for (const [configNames, ignore] of await Promise.all(
      tasks.map(
        async ([names, resolver]) => [names, await resolver()] as const,
      ),
    )) {
      for (const configName of configNames) {
        const fieldInfo = fieldsCollection.get(configName);
        const fieldName = fieldInfo.name;

        if (ignore) {
          // @ts-expect-error ikr
          delete input[fieldName];

          relevantFieldsProvided.delete(fieldName);

          // @ts-expect-error ikr
          if (fieldInfo.isOutput) delete output[fieldName];

          continue;
        }

        relevantFieldsProvided.add(fieldName);
      }
    }

    this._setCxtInput(input);
    this._setCxtValues(output);

    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    return fieldsCollection;
  }

  private async _evaluateMissingRequiredFields(
    fieldsCollection: FieldInfoCollection,
  ) {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    const propsToEvaluate = new Set<KeyOf<I>>(this.propsRequiredBy);

    const handlers: [
      string[],
      RequiredHandler<I, O, CtxOptions, ErrorMetadata>,
    ][] = [];

    for (const [configName, config] of Object.entries(this._definitions)) {
      if (config.type === "constant" || config.type === "dependent") continue;

      if (fieldsCollection.relevantConfigNames.has(configName)) continue;

      if (config.type === "required") {
        let error;

        if (typeof config.requiredError === "function") {
          try {
            let customError = config.requiredError(this._getInitResolverCtx());

            if (typeof customError === "string") {
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

      // @ts-expect-error ikr
      const { alias, required } = config;

      if (required) handlers.push([[alias ?? configName], required]);
    }

    await Promise.allSettled([
      ...Array.from(propsToEvaluate).map(async (name) => {}),

      ...toArray(this._options.required ?? []).map(async (config) => {
        const fields = config.fields as KeyOf<I>[];

        if (
          fields.some((prop) => fieldsCollection.relevantConfigNames.has(prop))
        )
          return;

        // Resolve each declared (config-name) property to the name a handler
        // would actually use — the alias, for aliased virtuals, since
        // `KeyOf<Input>` reflects the alias rather than the internal name.
        const resolvedNames = new Set(
          fields.map((prop) => this._getAliasByVirtual(prop) ?? prop),
        );

        const results = await Promise.allSettled(
          toArray(config.handler).map((handler) =>
            Promise.try(handler, ctx as never),
          ),
        );

        for (const result of results) {
          if (result.status !== "fulfilled" || !result.value) continue;

          for (const [prop, err] of Object.entries(result.value)) {
            if (!resolvedNames.has(prop as never)) continue;

            errorTool.set(
              prop as never,
              makeFieldError<ErrorMetadata>(err as never),
            );
          }
        }
      }),
    ]);

    return errorTool;
  }

  private _evaluateUpdateValidity(
    fieldsCollection: FieldInfoCollection,
  ): Set<string> {
    let input: Partial<I> = this.ctxInput;
    let updates: Partial<O> = this.ctxValues;
    let previousPartial: O = this.ctxPreviousValues!;

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
          this._options.equalityDepth,
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
          this._options.equalityDepth,
        )
      ) {
        // Drop unchanged output field
        delete (updates as any)[fieldName];
      }

    this._setCxtInput(input);
    this._setCxtValues(updates);

    return relevantFieldsProvided;
  }

  private async _runPrimaryValidators(
    fieldsCollection: FieldInfoCollection,
  ): Promise<ErrorTool<ErrorMetadata>> {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    await Promise.allSettled(
      Array.from(fieldsCollection.relevantFieldsProvided).map(async (name) => {
        const fieldInfo = fieldsCollection.get(name);
        const configName = fieldInfo.configName;

        // Read the raw value from filteredInput
        const rawValue = (ctx.input as any)[name];

        // ctx.input/ctx.values are always keyed by config name (never by the
        // literal alias a caller happened to use), so downstream consumers
        // (dependents' resolvers, etc.) can reliably read e.g. `input.virtual`
        // regardless of whether the caller provided `virtual` or its alias.
        let ctxUpdate = { [configName]: rawValue } as never;
        this._updateCxtInput(ctxUpdate);

        // Note: lax fields without a validator still go through `_validate`
        // below (which returns `{valid: true, validated: value}` as-is when
        // there's no validator) rather than short-circuiting here, because
        // `_validate` is also where the `allow` (allowed-values) check lives —
        // skipping straight past it here would let disallowed values through
        // whenever a lax field has no explicit validator.
        const isValid = (await Promise.try(() =>
          this._validate(fieldInfo, rawValue, ctx),
        )) as InternalValidatorResponse<O[KeyOf<O>], ErrorMetadata>;

        if (!isValid.valid)
          return this._handleInvalidValue(
            errorTool,
            name as KeyOf<I & O>,
            isValid,
          );

        let { validated } = isValid;

        if (isEqual(validated, undefined)) validated = rawValue;

        ctxUpdate = { [configName]: validated } as never;

        if (!fieldInfo.isVirtual) this._updateCxtValues(ctxUpdate);

        this._updateCxtInput(ctxUpdate);
      }),
    );

    return errorTool;
  }

  private async _handleSecondaryValidations(
    fieldsCollection: FieldInfoCollection,
  ) {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    await Promise.allSettled(
      fieldsCollection.relevantFieldsProvided.values().map(async (name) => {
        const fieldInfo = fieldsCollection.get(name);
        const configName = fieldInfo.configName;
        const validator = this._getSecondaryValidator(configName);

        if (!validator) return;

        // ctx.input is keyed by config name (see `_runPrimaryValidators`).
        // @ts-expect-error ikr
        const value = ctx.input?.[configName] as never as O[KeyOf<O>];

        let isValid: ValidatorResponseObject<unknown, ErrorMetadata>;

        try {
          isValid = this._sanitizeValidationResponse<unknown>(
            (await Promise.try(
              validator,
              value,
              ctx,
            )) as ValidatorResponseObject<unknown, ErrorMetadata>,
            value,
          );
        } catch {
          isValid = makeResponse<unknown, ErrorMetadata>({
            valid: false,
            reason: "validation failed",
          });
        }

        if (!isValid.valid)
          return this._handleInvalidValue(errorTool, name, isValid);

        const { validated } = isValid;

        if (
          !isEqual(validated, undefined) &&
          !isEqual(validated, value, this._options.equalityDepth)
        ) {
          const ctxUpdate = { [configName]: validated } as never;

          if (!fieldInfo.isVirtual) this._updateCxtValues(ctxUpdate);

          this._updateCxtInput(ctxUpdate);
        }
      }),
    );

    return errorTool;
  }

  private async _handlePostValidations(fieldsCollection: FieldInfoCollection) {
    const ctx = this._getContext();

    const errorTool = new ErrorTool<ErrorMetadata>();

    let configIds = new Set<string>();
    let validators: [
      string[],
      (
        | PostValidator<any, I, O, CtxOptions, ErrorMetadata>
        | PostValidator<any, I, O, CtxOptions, ErrorMetadata>[]
      ),
    ][] = [];

    // for (const configName of fieldsCollection.relevantConfigNames) {
    for (const config of toArray(this._options.postValidate)) {
      if (!config) continue;

      const { fields, validator } = config;

      if (
        fields.some((name) => fieldsCollection.relevantConfigNames.has(name))
      ) {
        // validators.push([fields, validator]);
      }
    }

    const handlers = Array.from(configIds).map((id) => {
      return {
        validator: this.postValidationConfigMap.get(id)!.validators,
        properties: id.split(",") as ArrayOfMinSizeTwo<
          Extract<keyof I, string>
        >,
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
          isEqual(validated, ctx.input[fieldName], this._options.equalityDepth)
        )
          return;

        const ctxUpdate = { [fieldName]: validated } as never;

        this._updateCxtInput(ctxUpdate);

        if (!fieldInfo.isVirtual) this._updateCxtValues(ctxUpdate);
        fieldsCollection.appendRelevantFieldProvided(fieldName);
      }
    };

    // Whether a group property counts as "provided" must also consider its
    // alias (relevantFieldsProvided is keyed by whichever literal key — alias
    // or config name — the caller actually used), and on a validator throw,
    // the fallback error must be reported under that same literal key rather
    // than always forcing alias resolution.
    const isProvided = (name: string) => {
      const alias = this._getAliasByVirtual(name as never);

      return (
        fieldsCollection.relevantFieldsProvided.has(name) ||
        (!!alias && fieldsCollection.relevantFieldsProvided.has(alias))
      );
    };

    const getDisplayKey = (name: string) => {
      const alias = this._getAliasByVirtual(name as never);

      if (alias && fieldsCollection.relevantFieldsProvided.has(alias))
        return alias;

      if (fieldsCollection.relevantFieldsProvided.has(name)) return name;

      return alias ?? name;
    };

    await Promise.allSettled(
      handlers.map(async ({ validator, properties }) => {
        const propsProvided = properties.filter(isProvided);

        if (!Array.isArray(validator)) {
          const { revalidatedData, success } = await Promise.try(() =>
            this._handlePostValidator({
              errorTool,
              propsProvided,
              ctx,
              validator: validator as any,
              properties,
              getDisplayKey,
            }),
          );

          if (!success || !revalidatedData) return;

          return handleRevalidatedData(revalidatedData);
        }

        for (const v1 of validator) {
          if (Array.isArray(v1)) {
            const ctx = this._getContext();

            const results = await Promise.all(
              v1.map(async (v2) => {
                const res = await Promise.try(() =>
                  this._handlePostValidator({
                    errorTool,
                    propsProvided,
                    ctx,
                    validator: v2 as any,
                    properties: properties,
                    getDisplayKey,
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
            errorTool,
            propsProvided,
            ctx: this._getContext(),
            validator: v1 as any,
            properties: properties,
            getDisplayKey,
          });

          if (!success) break;

          if (revalidatedData) handleRevalidatedData(revalidatedData);
        }
      }),
    );

    return errorTool;
  }

  private async _handlePostValidator({
    ctx,
    errorTool,
    propsProvided,
    properties,
    validator,
    getDisplayKey,
  }: {
    ctx: IvoContext<I, O, CtxOptions>;
    errorTool: ErrorTool<ErrorMetadata>;
    propsProvided: Extract<keyof I, string>[];
    properties: ArrayOfMinSizeTwo<string>;
    validator: PostValidator<KeyOf<I>, I, O, CtxOptions, ErrorMetadata>;
    getDisplayKey: (name: string) => string;
  }) {
    const revalidatedData: Partial<O> = {};

    try {
      const res = await Promise.try(validator, ctx, propsProvided);

      if (!isRecordLike(res)) return { revalidatedData: null, success: true };

      const { errors, validatedData } =
        this._handleObjectValidationResponse(res);

      for (const [fieldName, validated] of Object.entries(validatedData) as [
        KeyOf<I>,
        any,
      ][]) {
        const configName = this._getVirtualByAlias(fieldName) ?? fieldName;

        if (properties.includes(configName))
          // @ts-expect-error ikr
          revalidatedData[fieldName] = validated;
      }

      for (const [prop, error] of Object.entries(errors))
        errorTool.set(prop, makeFieldError(error));
    } catch {
      for (const configName of propsProvided) {
        // @ts-expect-error ikr
        errorTool.set(getDisplayKey(configName), validationFailedFieldError);
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

  private async _handleSanitizationOfVirtuals(
    fieldsCollection: FieldInfoCollection,
  ) {
    const sanitizers: [
      KeyOf<I>,
      NS.VirtualResolver<unknown, I, O, CtxOptions>,
    ][] = [];

    const ctx = this._getContext();

    // `relevantFieldsProvided` is narrowed to output fields only (see its
    // NS.setter), so virtuals — the only fields that can have a `sanitizer` —
    // never appear there. `relevantConfigNames` is the unfiltered, config-
    // name-mapped equivalent.
    for (const name of fieldsCollection.relevantConfigNames) {
      const fieldInfo = fieldsCollection.get(name);
      // @ts-expect-error ikr
      const sanitizer = this._getDefinition(fieldInfo.configName).sanitizer;

      if (sanitizer) sanitizers.push([name as KeyOf<I>, sanitizer]);
    }

    await Promise.allSettled(
      sanitizers.map(async ([name, sanitizer]) => {
        await Promise.try(sanitizer, ctx)
          .then((resolvedValue) =>
            this._updateCxtInput({ [name]: resolvedValue } as never),
          )
          .catch(() => null);
      }),
    );
  }

  private _handleObjectValidationResponse(data: Record<string, unknown>) {
    const validProperties = getKeysAsProps(data).filter((prop) =>
      this._isInputOrAlias(prop),
    );

    const errors = {} as Record<
      string,
      string | InputFieldError<ErrorMetadata>
    >;
    const validatedData = {} as Record<string, unknown>;

    for (const prop of validProperties) {
      const res = data[prop];

      if (typeof res === "object" && "validated" in (res as any)) {
        validatedData[prop] = (res as any).validated;

        continue;
      }

      if (isInputFieldError(res)) {
        errors[prop] = res as InputFieldError<ErrorMetadata>;

        continue;
      }

      if (typeof res === "string") {
        const message = res.trim();

        errors[prop] = message.length ? message : "validation failed";

        continue;
      }

      errors[prop] = "validation failed";
    }

    return { errors, validatedData };
  }

  private _isInputOrAlias = (prop: string) =>
    this._isInputProp(prop) || this._isVirtualAlias(prop);

  private _makeHandleFailure() {
    const ctx = this._getReadonlyCtx();
    const fieldsToCleanup = getKeysAsProps(ctx.rawInput);

    let cleanups: NS.FailureHandler<I, O, CtxOptions>[] = [];

    for (const prop of fieldsToCleanup)
      cleanups = cleanups.concat(
        this._getHandlers<NS.FailureHandler<I, O, CtxOptions>>(
          prop,
          "onFailure",
        ),
      );

    return async () => {
      await Promise.allSettled(
        cleanups.map((h) =>
          Promise.try(
            h,
            this._getFrozenCopy(ctx),
            this._getFrozenCopy(ctx.options),
          ),
        ),
      );
    };
  }

  private _makeHandleSuccess(fieldsCollection: FieldInfoCollection) {
    const relevantFields = new Set<string>(
      fieldsCollection.relevantConfigNames,
    );

    for (const name of Object.keys(this.ctxValues)) relevantFields.add(name);

    let successListeners = [] as NS.SuccessHandler<I, O, CtxOptions>[];

    for (const configName of relevantFields) {
      const { onSuccess } = this._definitions[configName];

      if (onSuccess) successListeners = successListeners.concat(onSuccess);
    }

    for (const config of toArray(this._options.onSuccess)) {
      if (!config) continue;

      if (typeof config === "function") {
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

    const isCreation = !ctx.isUpdate;
    const toResolve = new Set<string>();

    for (const configName in this._definitions) {
      const config = this._definitions[configName];

      if (config.type !== "dependent") continue;

      if (
        toArray(config.dependsOn).some((parent) =>
          fieldsCollection.relevantDependentConfigNames.has(parent),
        )
      )
        toResolve.add(configName);
    }

    const fieldsResolved = new Set<string>();
    const values = this.ctxValues;

    await Promise.allSettled(
      toResolve.values().map(async (name) => {
        const config = this._getDefinition(name) as NS.DependentField<
          any,
          I,
          O,
          CtxOptions
        >;

        // readonly dependents only re-resolve while their value still
        // matches the (static) default; once it has diverged, they're
        // frozen. A function/async default has no stable baseline to
        // compare against, so it's exempt and always re-resolves.
        if (
          !isCreation &&
          config.readonly &&
          typeof config.default !== "function" &&
          !isEqual(
            config.default,
            (values as any)[name],
            this._options.equalityDepth,
          )
        )
          return;

        const value = await Promise.try(config.resolver, ctx).catch(() =>
          // @ts-expect-error ikr
          isCreation ? null : ctx.previousValues?.[name],
        );

        if (
          !isCreation &&
          isEqual(value, (values as any)[name], this._options.equalityDepth)
        )
          return;

        fieldsResolved.add(name);
        this._updateCxtValues({ [name]: value } as never);
      }),
    );

    return fieldsCollection.newWithResolvedDependentFields(fieldsResolved);
  }

  private _sanitizeValidationResponse<T>(
    response: ValidatorResponseObject<T, ErrorMetadata>,
    value: unknown,
  ): ValidatorResponseObject<T, ErrorMetadata> {
    const responseType = typeof response;

    if (responseType === "boolean")
      return (
        response
          ? { valid: true, validated: value }
          : getValidationFailedResponse(value)
      ) as never;

    if (!response && responseType !== "object")
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

    if (response?.reason && typeof response?.reason === "string")
      _response.reason = response.reason;

    if (response?.metadata && isRecordLike(response.metadata))
      _response.metadata = response.metadata;
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

  private async _validate(
    { configName }: Pick<InputFieldInfo, "configName">,
    value: unknown,
    ctx: IvoContext<I, O, CtxOptions>,
  ) {
    const config = this._definitions[configName] as NS.LaxField<
      any,
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >;

    if (config.allow) return this._validateAllowedValues(configName, value);

    const validator = this._getPrimaryValidator(configName);

    if (validator) {
      try {
        return this._sanitizeValidationResponse<any>(
          (await Promise.try(
            validator,
            value,
            ctx as never,
          )) as ValidatorResponseObject<any, ErrorMetadata>,
          value,
        );
      } catch {
        return makeResponse<any>({ valid: false, reason: "validation failed" });
      }
    }

    return makeResponse<any, ErrorMetadata>({ valid: true, validated: value });
  }

  private _handleError(errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>) {
    return {
      data: null,
      error: this._options.sanitizeError(errorTool.payload, this.ctxOptions),
      options: this.ctxOptions,
      handleFailure: this._makeHandleFailure(),
      handleSuccess: null,
    };
  }

  private _handleUpdateError(errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>) {
    const options = this._getReadonlyCtx().options;
    const isNothingToUpdate = !errorTool.hasErrors;

    if (isNothingToUpdate) this._setCxtValues({});

    return {
      data: null,
      error: isNothingToUpdate
        ? ({ isNothingToUpdate, payload: null } as const)
        : ({
            isNothingToUpdate: false,
            payload: this._options.sanitizeError(errorTool.payload, options),
          } as const),
      options,
      handleFailure: this._makeHandleFailure(),
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
  return values && typeof values === "object";
}

function getValidationFailedResponse(value: unknown) {
  return {
    metadata: null,
    reason: "validation failed",
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
