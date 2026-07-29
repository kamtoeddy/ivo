import {
  getKeysAsProps,
  getSetValuesAsProps,
  isEqual,
  isFunctionLike,
  isPropertyOf,
  isRecordLike,
  makeResponse,
  type ObjectType,
  sort,
  sortKeys,
  toArray,
} from '../utils';
import { defaultOptions, SchemaCore } from './schema-core';
import {
  type InternalValidatorResponse,
  type InvalidValidatorResponse,
  type IvoContext,
  type IvoErrorPayload,
  type KeyOf,
  LIFE_CYCLES,
  type NS,
  type PostValidator,
  type ReadonlyIvoContext,
  type RealType,
  type Validator,
  type ValidatorResponseObject,
  type VirtualResolver,
} from './types';
import {
  cloneValue,
  type DefaultFieldErrorMetadata,
  ErrorTool,
  type FieldError,
  type InputFieldError,
  isInputFieldError,
  makeFieldError,
} from './utils';

export { Model, ModelTool, Schema };

export type IvoResultInfo<
  T extends Model<any, any, any>,
  Operation extends 'create' | 'update' = 'create',
> =
  | {
      data: NonNullable<Awaited<ReturnType<T[Operation]>>['data']>;
      error: null;
    }
  | {
      data: null;
      error: NonNullable<Awaited<ReturnType<T[Operation]>>['error']>;
    };

const NotAllowedError = 'value not allowed';
const validationFailedFieldError = makeFieldError('validation failed');

class Schema<
  const Input extends RealType<Input>,
  const Output extends RealType<Output> = Input,
  const CtxOptions extends ObjectType = {},
  const ErrorMetadata = DefaultFieldErrorMetadata,
  const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
> extends SchemaCore<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload> {
  constructor(
    definitions: NS.Definitions<Input, Output, CtxOptions, ErrorMetadata>,
    options: NS.Options<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    > = defaultOptions as never,
  ) {
    super(
      definitions as never as NS.Definitions_<Input, Output, ErrorMetadata>,
      options as never,
    );
  }

  get definitions() {
    return this._definitions as never as NS.Definitions<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >;
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
    const ExtendedInput extends RealType<ExtendedInput>,
    const ExtendedOutput extends RealType<ExtendedOutput> = ExtendedInput,
    const ExtendedCtxOptions extends ObjectType = CtxOptions,
    const ExtendedErrorMetadata = ErrorMetadata,
    const ExtendedErrorPayload = IvoErrorPayload<
      ExtendedErrorMetadata,
      KeyOf<ExtendedInput>
    >,
  >(
    definitions: Partial<
      NS.Definitions<
        ExtendedInput,
        ExtendedOutput,
        ExtendedCtxOptions,
        ExtendedErrorMetadata
      >
    >,
    options: NS.ExtensionOptions<
      Input,
      Output,
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    > = {},
  ) {
    const { remove = [], useParentOptions = true, ...rest } = options;

    const _definitions = { ...this.definitions } as unknown as NS.Definitions<
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata
    >;

    toArray(remove ?? [])?.forEach(
      (prop) => delete (_definitions as never)?.[prop],
    );

    const options_ = {} as NS.Options<
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
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
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    >(
      Object.assign({}, _definitions, definitions),
      Object.assign({}, options_, rest),
    );
  }

  getModel() {
    return new Model(
      () =>
        new ModelTool<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload>(
          this,
        ),
    );
  }
}

class ModelTool<
  I extends RealType<I>,
  O extends RealType<O>,
  CtxOptions extends ObjectType = {},
  ErrorMetadata = DefaultFieldErrorMetadata,
  ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
> extends SchemaCore<I, O, CtxOptions, ErrorMetadata, ErrorPayload> {
  // contexts & values
  private isUpdate: boolean = false;
  private ctxInput: Partial<I> = {};
  private ctxRawInput: Partial<I> = {};
  private ctxValues: Partial<O> = {};
  private ctxPreviousValues: O | null = null;
  private _ctxOptions: CtxOptions = {} as CtxOptions;

  constructor(schema: Schema<I, O, CtxOptions, ErrorMetadata, ErrorPayload>) {
    super(schema.definitions as never, schema.options as never);
  }

  private _getFieldInfoCollection(): FieldInfoCollection {
    const fields: Map<string, FieldInfo> = new Map();

    for (const configName of getKeysAsProps(this._definitions)) {
      const config = this._definitions[configName]!;

      if (config.constant || config.dependsOn) continue;

      const isVirtual = !!config.virtual;

      fields.set(
        configName,
        new FieldInfo({
          name: configName,
          configName,
          isInput: true,
          isOutput: !isVirtual,
        }),
      );

      const aliasName = config.alias;

      if (aliasName)
        fields.set(
          aliasName,
          new FieldInfo({
            name: aliasName,
            configName,
            isInput: true,
            isOutput: !isVirtual,
          }),
        );
    }

    return new FieldInfoCollection(fields);
  }

  private async _filterInputFieldsAllowed(): Promise<FieldInfoCollection> {
    let ctx = this._getContext();
    const _previousValues = ctx.previousValues;
    const fieldsCollection = this._getFieldInfoCollection();
    // @ts-expect-error ikr
    const rawInput: Partial<I> = ctx.rawInput;
    const isUpdate = !!_previousValues;
    const previousValues: Partial<O> = _previousValues
      ? cloneValue(_previousValues)
      : {};
    const entityResolvers = [];
    const input = cloneValue(rawInput);
    const output: Partial<O> = {};
    const fieldsProvided = new Set<string>();
    const relevantFieldsProvided = new Set<string>();

    if (isUpdate) {
      for (const config of toArray(this._options.ignoreUpdate ?? [])) {
        if (config && typeof config === 'object' && 'fields' in config) {
          if ((config as any).fields.length === 0)
            entityResolvers.push((config as any).resolver);
        } else if (typeof config === 'function') entityResolvers.push(config);
      }

      for (const [fieldName, value] of Object.entries(rawInput)) {
        const fieldInfo = fieldsCollection.get(fieldName);

        if (
          (fieldInfo.isInput && !fieldInfo.isOutput) ||
          // @ts-expect-error ikr
          !isEqual(value, previousValues[fieldName])
        ) {
          relevantFieldsProvided.add(fieldName);

          if (fieldInfo.isOutput) {
            // @ts-expect-error ikr
            output[fieldName] = input[fieldName];
          }
        } else {
          // @ts-expect-error ikr
          input[fieldName] = undefined;
        }

        fieldsProvided.add(fieldName);
      }
    } else {
      for (const fieldName of Object.keys(rawInput)) {
        const fieldInfo = fieldsCollection.get(fieldName);

        if (fieldInfo.isOutput) {
          // @ts-expect-error ikr
          output[fieldName] = input[fieldName];
        }

        fieldsProvided.add(fieldName);
        relevantFieldsProvided.add(fieldName);
      }
    }

    if (entityResolvers.length) {
      for (const task of await Promise.allSettled(
        entityResolvers.map((resolver) =>
          Promise.try(resolver, ctx).catch(() => false),
        ),
      )) {
        // if "task.value" is positive, it means "ignore"
        if (task.status === 'fulfilled' && task.value) return fieldsCollection;
      }
    }

    fieldsCollection.fieldsProvided = fieldsProvided;
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    this._updateCxtInput(input);
    this._updateCxtValues(output);
    ctx = this._getContext();

    const tasks: [string[] | readonly string[], Promise<boolean>][] = [];

    for (const fieldName of fieldsProvided.values()) {
      const fieldInfo = fieldsCollection.get(fieldName);
      const configName = fieldInfo?.configName ?? fieldName;

      const {
        default: defaultValue,
        ignore,
        ignoreInit,
        ignoreUpdate,
        readonly,
      } = this._getDefinition(configName);

      if (ignore) {
        tasks.push([[fieldInfo?.name ?? fieldName], Promise.try(ignore, ctx)]);
        continue;
      }

      const source = isUpdate ? ignoreUpdate : ignoreInit;

      if (
        source === undefined ||
        (isUpdate && this._isRequired(fieldName) && readonly)
      )
        continue;

      if (source === true) {
        relevantFieldsProvided.delete(fieldName);

        // @ts-expect-error ikr
        delete input[fieldName];
        delete (this.ctxInput as any)[fieldName];
        delete (this.ctxRawInput as any)[fieldName];

        if (fieldInfo?.isOutput) {
          // @ts-expect-error ikr
          delete output[fieldName];
        }

        continue;
      }

      if (typeof source === 'function')
        tasks.push([
          [fieldInfo?.name ?? fieldName],
          Promise.try(source as any, ctx),
        ]);

      // readonly only restricts updates; creation is always allowed
      if (readonly && isUpdate) {
        const hasStaticDefault =
          defaultValue !== undefined && typeof defaultValue !== 'function';

        // readonly with a static default: only allow the update while the
        // previous value still equals that default. Otherwise (no default,
        // e.g. required properties, or a function/async default) the
        // property is permanently locked after creation.
        if (
          hasStaticDefault &&
          // @ts-expect-error ikr
          isEqual(previousValues[fieldName], defaultValue)
        ) {
          continue;
        }

        relevantFieldsProvided.delete(fieldName);

        // @ts-expect-error ikr
        delete input[fieldName];

        if (fieldInfo?.isOutput) {
          // @ts-expect-error ikr
          delete output[fieldName];
        }
      }
    }

    const relevantConfigNames = Array.from(
      new Set(
        Array.from(relevantFieldsProvided.values()).map(
          (name) => fieldsCollection.get(name).configName,
        ),
      ),
    );

    for (const config of toArray(this._options.ignore ?? [])) {
      if (typeof config === 'function') {
        tasks.push([
          relevantConfigNames,
          Promise.try(config, ctx)
            .then((v) => !!v)
            .catch(() => false),
        ]);
      } else if (config && typeof config === 'object') {
        const fields = config.fields as string[];

        if (fields.some((name: string) => relevantConfigNames.includes(name))) {
          tasks.push([
            fields,
            Promise.try(config.resolver, ctx)
              .then((v) => !!v)
              .catch(() => false),
          ]);
        }
      }
    }

    if (isUpdate) {
      for (const config of toArray(this._options.ignoreUpdate ?? [])) {
        if (typeof config === 'function') {
          tasks.push([
            relevantConfigNames,
            Promise.try(config as any, ctx)
              .then((v) => !!v)
              .catch(() => false),
          ]);
        } else if (config && typeof config === 'object') {
          const fields = config.fields as string[];

          if (
            fields.some((name: string) => relevantConfigNames.includes(name))
          ) {
            tasks.push([
              fields,
              Promise.try(config.resolver, input, _previousValues, {
                options: ctx.options,
                updateOptions: ctx.updateOptions,
              })
                .then((v) => !!v)
                .catch(() => false),
            ]);
          }
        }
      }
    }

    if (!tasks.length) {
      fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

      return fieldsCollection;
    }

    for (const [configNames, ignore] of await Promise.all(
      tasks.map(async ([names, promise]) => [names, await promise] as const),
    )) {
      for (const configName of configNames) {
        const fieldInfo = fieldsCollection.get(configName);
        const fieldName = fieldInfo?.name ?? configName;

        if (ignore) {
          // @ts-expect-error ikr
          delete input[fieldName];
          // @ts-expect-error ikr
          delete input[configName];
          delete (this.ctxInput as any)[fieldName];
          delete (this.ctxInput as any)[configName];
          delete (this.ctxRawInput as any)[fieldName];
          delete (this.ctxRawInput as any)[configName];

          relevantFieldsProvided.delete(fieldName);

          if (fieldInfo.isOutput) {
            // @ts-expect-error ikr
            delete output[fieldName];
          }

          continue;
        }

        relevantFieldsProvided.add(fieldName);
      }
    }

    this._updateCxtInput(input);
    this._updateCxtValues(output);

    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    return fieldsCollection;
  }

  private _getReadonlyCtx() {
    const data = this.ctxValues;
    const isUpdate = this.isUpdate,
      changes = isUpdate ? cloneValue(data) : null,
      previousValues = isUpdate ? cloneValue(this.ctxPreviousValues) : null,
      input = this._getFrozenCopy(cloneValue(this.ctxInput)),
      values = this._getFrozenCopy(
        cloneValue(
          isUpdate
            ? Object.assign({}, previousValues, data)
            : Object.assign({}, data),
        ),
      );

    return this._getFrozenCopy({
      changes,
      input,
      rawInput: cloneValue(this.ctxRawInput),
      isUpdate,
      previousValues,
      values,
      options: this._getCtxOptions(),
    }) as ReadonlyIvoContext<I, O, CtxOptions>;
  }

  private _getContext() {
    return this._getFrozenCopy(
      Object.assign({}, this._getReadonlyCtx(), {
        updateOptions: this._updateCtxOptions,
      }),
    ) as never as IvoContext<I, O, CtxOptions>;
  }

  private _initContext(props: {
    rawInput: Partial<I>;
    isUpdate: boolean;
    previousValues: O | null;
    options: CtxOptions;
  }) {
    this.ctxInput = {};
    this.ctxValues = {};
    this.isUpdate = props.isUpdate;
    this.ctxRawInput = props.rawInput;
    this.ctxPreviousValues = props.previousValues;
    this._ctxOptions = props.options;
  }

  private _updateCxtInput = (updates: Partial<I>) => {
    Object.assign(this.ctxInput, updates);
  };

  private _updateCxtValues = (updates: Partial<O>) => {
    Object.assign(this.ctxValues, updates);
  };

  private _getCtxOptions = () => this._getFrozenCopy(this._ctxOptions);

  private _updateCtxOptions = (options: Partial<CtxOptions>) =>
    Object.assign(this._ctxOptions, options);

  private _getDefaultValue = async (
    prop: string,
    ctx: IvoContext<I, O, CtxOptions>,
  ) => {
    const _default = this._getDefinition(prop)?.default;

    let value: any;

    try {
      value = isFunctionLike(_default)
        ? await Promise.try(_default as any, ctx)
        : this.defaults[prop as KeyOf<O>];
    } catch {
      value = null;
    }

    return isEqual(value, undefined) ? this.ctxValues[prop as KeyOf<O>] : value;
  };

  private _getRequiredState = async (
    prop: string,
    ctx: IvoContext<I, O, CtxOptions>,
  ): Promise<[boolean, string | FieldError<ErrorMetadata>]> => {
    const { required } = this._getDefinition(prop);

    if (!required) return [false, ''];

    const fallbackMessage = `'${prop}' is required`;

    if (!isFunctionLike(required)) return [required, fallbackMessage];

    const results = await required(ctx);
    const isBoolean = typeof results === 'boolean';

    if (!isBoolean && !Array.isArray(results)) return [false, ''];

    if (isBoolean) return [results as boolean, results ? fallbackMessage : ''];

    const [isRequired, message] = results as [
        boolean,
        string | InputFieldError<ErrorMetadata>,
      ],
      isString = typeof message === 'string';

    if (!isRequired || (!isString && !isInputFieldError(message)))
      return [isRequired, fallbackMessage];

    if (isString) return [true, message || fallbackMessage];

    const fieldError = makeFieldError<ErrorMetadata>(message, fallbackMessage);

    return [
      true,
      isPropertyOf('metadata', message)
        ? fieldError
        : ({ reason: fieldError.reason } as never),
    ];
  };

  private _cleanInput(input: Partial<I>) {
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
        else if (virtual) {
          values[virtual] = input[prop] as never;
          delete values[prop];
        }
      }
    }

    this.ctxRawInput = values;

    return values;
  }

  private _isIngnorable = (prop: string) => {
    return !!this._getDefinition(prop).ignore;
  };

  private _shouldIgnore = (prop: string) => {
    const { ignore } = this._getDefinition(prop);

    return ignore ? ignore(this._getContext()) : undefined;
  };

  private _ignoreUpdate = (prop: string, _extraCtx?: ObjectType) => {
    const { ignoreUpdate } = this._getDefinition(prop);

    if (ignoreUpdate === undefined) return false;
    if (ignoreUpdate === true) return true;
    if (typeof ignoreUpdate === 'function') {
      const res = ignoreUpdate(this._getContext() as never);
      return !!res;
    }

    return false;
  };

  private _getPrimaryValidator = <K extends keyof (O | I)>(prop: string) => {
    const { validator } = this._getDefinition(prop as never);

    return (Array.isArray(validator) ? validator[0] : validator) as
      | Validator<K, I, O>
      | undefined;
  };

  private _getSecondaryValidator = <K extends keyof (O | I)>(prop: string) => {
    const { validator } = this._getDefinition(prop as never);

    return (Array.isArray(validator) ? validator[1] : undefined) as
      | Validator<K, I, O>
      | undefined;
  };

  private _getNotAllowedError(prop: string, value: unknown) {
    const allow = this._getDefinition(prop as never)?.allow;

    if (Array.isArray(allow)) return NotAllowedError;

    // @ts-expect-error: lol
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

  private _handleError(errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>) {
    return {
      data: null,
      error: errorTool.hasErrors
        ? this._options.sanitizeError(
            errorTool.payload,
            this._getContext().options,
          )
        : null,
      handleFailure: this._makeHandleFailure(),
      handleSuccess: null,
    };
  }

  private _handleInvalidValue(
    errorTool: ErrorTool<ErrorMetadata>,
    prop: KeyOf<I & O>,
    validationResponse: InvalidValidatorResponse<ErrorMetadata>,
  ) {
    const { reason, metadata } = validationResponse;

    const fieldError = makeFieldError<ErrorMetadata>(
      reason || 'validation failed',
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.set(prop, fieldError);
  }

  /**
   * Mirrors Rust's `attach_constants_and_defaults`.
   *
   * Resolves constant field values and default values for lax/dependent fields
   * that are NOT present in `filteredInput`. Updates `ctxValues` / `partialContext`
   * in place and returns the accumulated output partial.
   */
  private async _attachConstantsAndDefaults(): Promise<Partial<O>> {
    const data = {} as Partial<O>;
    const input = cloneValue(this.ctxInput);
    const ctx = this._getContext();
    const fieldsProvidedNames = new Set(Object.keys(input));

    await Promise.allSettled(
      getKeysAsProps(this._definitions).map(async (configName) => {
        const config = this._definitions[configName]!;

        // ── Constants ──────────────────────────────────────────────────────────
        if (config.constant) {
          const _val = config.value;
          let value: any;

          try {
            value = isFunctionLike(_val)
              ? await Promise.try(_val as any, ctx)
              : _val;
          } catch {
            value = null;
          }

          (data as any)[configName] = value;

          this._updateCxtValues({ [configName]: value } as never);

          return;
        }

        // ── Lax fields that were NOT provided in the filtered input ─────────────
        if (
          this._isLaxProp(configName) &&
          !fieldsProvidedNames.has(configName)
        ) {
          const value = await this._getDefaultValue(configName, ctx);

          (data as any)[configName] = value;

          this._updateCxtValues({ [configName]: value } as never);

          return;
        }

        // ── Dependent fields: attach default value ──────────────────────────────
        if (config.dependsOn) {
          const value = await this._getDefaultValue(configName, ctx);

          (data as any)[configName] = value;

          this._updateCxtValues({ [configName]: value } as never);
        }
      }),
    );

    return data;
  }

  /**
   * Mirrors Rust's `validate`.
   *
   * Runs the primary validator for every field in `fieldsCollection.relevantConfigNames`
   * (which includes virtual config names unlike `relevantFieldsProvided` which only keeps
   * output fields). For lax fields without a validator, the raw input value is accepted
   * as-is (matching the Rust branch that sets input+output directly when field is Lax).
   *
   * Returns `{ data, error }` where `data` is the accumulated validated output partial.
   */
  private async _runPrimaryValidators(
    fieldsCollection: FieldInfoCollection,
  ): Promise<ErrorTool<ErrorMetadata>> {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    // relevantConfigNames includes virtuals (the setter for relevantFieldsProvided
    // only keeps output fields, so we must use configNames to include virtuals)

    await Promise.allSettled(
      Array.from(fieldsCollection.fieldsProvided).map(async (name) => {
        const fieldInfo = fieldsCollection.get(name);

        // Read the raw value from filteredInput
        const rawValue = (ctx.input as any)[name];
        const hasValidator = !!this._getPrimaryValidator(fieldInfo.configName);

        let ctxUpdate = { [name]: rawValue } as never;
        this._updateCxtInput(ctxUpdate);

        // For lax fields with no validator: accept value as-is (Rust: set input+output)
        if (this._isLaxProp(name) && !hasValidator) {
          if (fieldInfo.isOutput) this._updateCxtValues(ctxUpdate);

          return;
        }

        const isValid = (await Promise.try(() =>
          this._validate(name as never, rawValue, ctx),
        )) as InternalValidatorResponse<O[KeyOf<O>], ErrorMetadata>;

        if (!isValid.valid)
          return this._handleInvalidValue(
            errorTool,
            name as KeyOf<I & O>,
            isValid,
          );

        let { validated } = isValid;

        if (isEqual(validated, undefined)) validated = rawValue;

        ctxUpdate = { [name]: validated } as never;

        if (fieldInfo.isOutput) this._updateCxtValues(ctxUpdate);

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
      fieldsCollection.relevantConfigNames.values().map(async (name) => {
        const fieldInfo = fieldsCollection.get(name);
        const validator = this._getSecondaryValidator(fieldInfo.configName);

        if (!validator) return;

        // @ts-expect-error ikr
        const value = ctx.input?.[name] as never as O[KeyOf<O>];

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
            reason: 'validation failed',
          });
        }

        if (!isValid.valid)
          return this._handleInvalidValue(errorTool, name as any, isValid);

        const { validated } = isValid;

        if (
          !isEqual(validated, undefined) &&
          !isEqual(validated, value, this._options.equalityDepth)
        ) {
          const ctxUpdate = { [name]: validated } as never;

          if (fieldInfo.isOutput) this._updateCxtValues(ctxUpdate);

          this._updateCxtInput(ctxUpdate);
        }
      }),
    );

    return errorTool;
  }

  private async _handlePostValidations(fieldsCollection: FieldInfoCollection) {
    const ctx = this._getContext();

    const errorTool = new ErrorTool<ErrorMetadata>();

    const handlerIds = new Set<string>(),
      handlerIdToProps = new Map<string, Set<string>>(),
      configIDsToAllPostValidatableProps = new Map<string, Set<string>>();

    for (const [
      configName,
      setOfConfigIDs,
    ] of this.propToPostValidationConfigIDsMap.entries()) {
      if (!fieldsCollection.relevantConfigNames.has(configName)) continue;

      for (const id of setOfConfigIDs.values()) {
        {
          const set = configIDsToAllPostValidatableProps.get(id) ?? new Set();
          configIDsToAllPostValidatableProps.set(id, set.add(configName));
        }

        handlerIds.add(id);

        const set = handlerIdToProps.get(id) ?? new Set();
        handlerIdToProps.set(id, set.add(configName));
      }
    }

    const handlers = Array.from(handlerIds).map((id) => ({
      id,
      validator: this.postValidationConfigMap.get(id)!.validators,
      postValidatableProps: Array.from(
        configIDsToAllPostValidatableProps.get(id)!,
      ) as KeyOf<I>[],
    }));

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

        if (fieldInfo.isOutput) this._updateCxtValues(ctxUpdate);
      }
    };

    await Promise.allSettled(
      handlers.map(async ({ id, validator, postValidatableProps }) => {
        const propsProvided = Array.from(handlerIdToProps.get(id)!) as Extract<
          keyof I,
          string
        >[];

        if (!Array.isArray(validator)) {
          const { revalidatedData, success } = await Promise.try(() =>
            this._handlePostValidator({
              errorTool,
              propsProvided,
              ctx,
              validator: validator as any,
              postValidatableProps,
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
                    postValidatableProps,
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
    ctx,
    errorTool,
    propsProvided,
    postValidatableProps,
    validator,
  }: {
    ctx: IvoContext<I, O, CtxOptions>;
    errorTool: ErrorTool<ErrorMetadata>;
    propsProvided: Extract<keyof I, string>[];
    postValidatableProps: Extract<keyof I, string>[];
    validator: PostValidator<KeyOf<I>, I, O, CtxOptions, ErrorMetadata>;
  }) {
    const revalidatedData: Partial<O> = {};

    try {
      const res = await Promise.try(validator, ctx, propsProvided);

      if (!isRecordLike(res)) return { revalidatedData: null, success: true };

      const { errors, validatedData } =
        this._handleObjectValidationResponse(res);

      for (const [prop, validated] of Object.entries(validatedData) as [
        KeyOf<I>,
        any,
      ][]) {
        const propName = (this._getAliasByVirtual(prop as never) ??
          prop) as keyof O;

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

        if (alias && isPropertyOf(alias, ctx.rawInput)) errorField = alias;
        else if (isPropertyOf(prop, ctx.rawInput)) errorField = prop;

        // @ts-expect-error ikr
        if (errorField) errorTool.set(errorField, validationFailedFieldError);
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

  private async _evaluateMissingRequiredFields(
    fieldsCollection: FieldInfoCollection,
  ) {
    const ctx = this._getReadonlyCtx();

    const isUpdate = ctx.isUpdate;
    const errorTool = new ErrorTool<ErrorMetadata>();

    await Promise.allSettled(
      Array.from(this.propsRequiredBy.keys()).map(async (prop) => {
        if (fieldsCollection.fieldsProvided.has(prop)) return;

        let isUpdatable = false;

        if (isUpdate && this._isReadonly(prop)) {
          isUpdatable = this._isUpdatable(
            prop,
            (ctx.rawInput as never)?.[prop],
          );

          if (!isUpdatable) return;
        }

        const [isRequired, message] = await Promise.try(
          this._getRequiredState,
          prop,
          ctx as never,
        );

        if (
          !isRequired ||
          (isUpdate &&
            !isUpdatable &&
            !this._isUpdatable(prop, (ctx.rawInput as never)?.[prop]))
        )
          return;

        const alias = this._getAliasByVirtual(prop);

        if (!alias) {
          errorTool.set(prop, makeFieldError<ErrorMetadata>(message));

          return;
        }

        errorTool.set(
          alias as never,
          makeFieldError<ErrorMetadata>(
            message === `'${prop}' is required`
              ? `'${alias}' is required`
              : message,
          ),
        );
      }),
    );

    return errorTool;
  }

  private async _handleSanitizationOfVirtuals(
    fieldsCollection: FieldInfoCollection,
  ) {
    const sanitizers: [KeyOf<I>, VirtualResolver<unknown, I, O, CtxOptions>][] =
      [];

    const ctx = this._getContext();

    for (const name of fieldsCollection.relevantFieldsProvided) {
      const fieldInfo = fieldsCollection.get(name);
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

      if (typeof res === 'object' && 'validated' in (res as any)) {
        validatedData[prop] = (res as any).validated;

        continue;
      }

      if (isInputFieldError(res)) {
        errors[prop] = res as InputFieldError<ErrorMetadata>;

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

  private _isUpdatable(prop: string, value: unknown = undefined) {
    if (!this._isInputOrAlias(prop)) return false;

    const isAlias = this._isVirtualAlias(prop);

    const propName = (
      isAlias ? this._getVirtualByAlias(prop)! : prop
    ) as KeyOf<O>;

    if (this._isIngnorable(propName) && this._shouldIgnore(propName))
      return false;

    const hasIgnoreUpdateRule = this._isRuleInDefinition(
      propName,
      'ignoreUpdate',
    );

    const extraCtx = isAlias ? { [propName]: value } : {};

    const ignoreUpdate = this._ignoreUpdate(propName, extraCtx);

    if (this._isVirtual(prop))
      return hasIgnoreUpdateRule ? !ignoreUpdate : true;

    if (hasIgnoreUpdateRule && ignoreUpdate) return false;

    if (this._isReadonly(propName))
      return isEqual(
        this.defaults[propName],
        this.ctxValues[propName],
        this._options.equalityDepth,
      );

    return !isEqual(
      this.ctxValues[propName],
      value,
      this._options.equalityDepth,
    );
  }

  private _isInputOrAlias = (prop: string) =>
    this._isVirtualAlias(prop) || this._isInputProp(prop);

  private _makeHandleFailure() {
    const ctx = this._getReadonlyCtx();
    const fieldsToCleanup = getKeysAsProps(ctx.rawInput);

    let cleanups: NS.FailureHandler<I, O, CtxOptions>[] = [];

    for (const prop of fieldsToCleanup)
      cleanups = cleanups.concat(
        this._getHandlers<NS.FailureHandler<I, O, CtxOptions>>(
          prop,
          'onFailure',
        ),
      );

    return async () => {
      await Promise.allSettled(
        cleanups.map(
          async (h) =>
            await Promise.try(
              h,
              this._getFrozenCopy(ctx),
              this._getFrozenCopy(ctx.options),
            ),
        ),
      );
    };
  }

  private _makeHandleSuccess(fieldsCollection: FieldInfoCollection) {
    const relevantFields = fieldsCollection.relevantConfigNames.union(
      fieldsCollection.relevantDependentConfigNames,
    );
    const ctx = this._getReadonlyCtx(),
      setOfSuccessHandlerIDs = new Set<string>();

    let successListeners = [] as NS.SuccessHandler<I, O, CtxOptions>[];

    for (const prop of relevantFields) {
      const handlers = this._getHandlers<NS.SuccessHandler<I, O>>(
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
        successListeners.map(async (h) => await Promise.try(h, ctx)),
      );
    };
  }

  private async _resolveDependentChanges(
    fieldsCollection: FieldInfoCollection,
  ) {
    const ctx = this._getContext();

    const isCreation = !ctx.isUpdate;
    const fieldsResolved = new Set<string>();
    const values = cloneValue<Partial<O>>(ctx.values);

    let toResolve = [] as KeyOf<O>[];

    for (const prop of fieldsCollection.relevantDependentConfigNames.values()) {
      const fieldInfo = fieldsCollection.get(prop);

      const dependencies = this._getDependencies(fieldInfo.configName);

      if (dependencies.length)
        toResolve = toResolve.concat(dependencies as never);
    }

    toResolve = Array.from(new Set(toResolve));

    await Promise.allSettled(
      toResolve.map(async (name) => {
        const config = this._getDefinition(name);

        // readonly dependents only re-resolve while their value still
        // matches the (static) default; once it has diverged, they're
        // frozen. A function/async default has no stable baseline to
        // compare against, so it's exempt and always re-resolves.
        if (
          !isCreation &&
          config.readonly &&
          typeof config.default !== 'function' &&
          !isEqual(
            config.default,
            (values as any)[name],
            this._options.equalityDepth,
          )
        )
          return;

        const resolver = config.resolver!;
        let value: any;

        try {
          value = await Promise.try(resolver, ctx);
        } catch {
          value = isCreation ? null : ctx.previousValues?.[name];
        }

        if (
          !isCreation &&
          isEqual(value, (values as any)[name], this._options.equalityDepth)
        )
          return;

        fieldsResolved.add(name);
        this._updateCxtValues({ [name]: value } as never);
      }),
    );

    return fieldsResolved;
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

  private _useConfigProps(isUpdate = false) {
    const values = cloneValue(this.ctxValues);

    if (!this.timestampTool.withTimestamps) return sortKeys(values);

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

    return sortKeys(results);
  }

  private async _validate<K extends KeyOf<I>>(
    prop: K,
    value: unknown,
    ctx: IvoContext<I, O, CtxOptions>,
  ) {
    if (!this._isInputOrAlias(prop))
      return makeResponse<I[K]>({
        valid: false,
        value,
        reason: 'Invalid property',
      });

    const isAlias = this._isVirtualAlias(prop),
      propName = (isAlias ? this._getVirtualByAlias(prop) : prop)!,
      allowedValues = this.propsToAllowedValuesMap.get(propName);

    if (allowedValues && !allowedValues.has(value)) {
      const fieldError = makeFieldError<ErrorMetadata>(
        this._getNotAllowedError(propName, value),
      );

      return makeResponse<I[K], ErrorMetadata>({
        valid: false,
        value,
        reason: fieldError.reason,
        metadata: fieldError.metadata,
      });
    }

    const validator = this._getPrimaryValidator(propName as never);

    if (validator) {
      let res: ValidatorResponseObject<I[K], ErrorMetadata>;

      try {
        res = this._sanitizeValidationResponse<I[K]>(
          (await Promise.try(
            validator,
            value,
            ctx as never,
          )) as ValidatorResponseObject<I[K], ErrorMetadata>,
          value,
        );
      } catch {
        return makeResponse<I[K]>({
          valid: false,
          reason: 'validation failed',
        });
      }

      if (allowedValues && res.valid && !allowedValues.has(res.validated))
        return makeResponse<I[K], ErrorMetadata>({
          valid: true,
          validated: value as never,
        });

      return res;
    }

    return makeResponse<I[K], ErrorMetadata>({
      valid: true,
      validated: value as never,
    });
  }

  /**
   * Mirrors Rust's `create` method step-by-step.
   *
   * Steps:
   *  1. Clean / filter raw input
   *  2. Filter allowed fields  (filter_input_fields_allowed)
   *  3. Attach constants + defaults  (attach_constants_and_defaults)
   *  4. Rebuild ctx with post-filter + post-defaults state
   *  5. Evaluate missing required fields  (evaluate_missing_required_fields)
   *  6. Run primary validators  (validate)
   *  7. Run secondary re-validators  (re_validate)
   *  8. Run post-validators  (post_validate)
   *  9. Sanitize virtuals  (sanitize_virtuals)
   * 10. Resolve dependent field values  (resolve_dependent_values)
   * 11. Attach timestamps  (attach_timestamps)
   * 12. Build & return result
   */
  async create(input: Partial<I>, contextOptions: CtxOptions) {
    const options = this._updateCtxOptions(contextOptions);

    if (!areValuesOk(input)) input = {};

    // Step 1 – clean raw input (strips unknown / conflicting virtual+alias pairs)
    this._cleanInput(input);

    // Build an initial ctx
    this._initContext({
      isUpdate: false,
      rawInput: this.ctxRawInput,
      previousValues: null,
      options,
    });

    // Step 2 – filter which input fields are allowed for this creation
    const fieldsCollection = await this._filterInputFieldsAllowed();

    // Step 3 – attach constants and defaults for non-provided fields
    await this._attachConstantsAndDefaults();

    // Step 5 – evaluate missing required fields
    const requiredError =
      await this._evaluateMissingRequiredFields(fieldsCollection);
    if (requiredError.hasErrors) return this._handleError(requiredError);

    // Step 6 – run primary validators over relevantConfigNames (includes virtuals)
    const primaryError = await this._runPrimaryValidators(fieldsCollection);

    if (primaryError.hasErrors) return this._handleError(primaryError);

    // Step 7 – run secondary (re-)validators
    const secondaryError =
      await this._handleSecondaryValidations(fieldsCollection);
    if (secondaryError.hasErrors) return this._handleError(secondaryError);

    // Step 8 – run post-validators
    const postValidationError =
      await this._handlePostValidations(fieldsCollection);
    if (postValidationError.hasErrors)
      return this._handleError(postValidationError);

    // Step 9 – sanitize virtuals
    await this._handleSanitizationOfVirtuals(fieldsCollection);

    // Step 10 – resolve dependent field values
    let fieldsToResolve = cloneValue(fieldsCollection.fieldsProvided);

    while (fieldsToResolve.size > 0)
      fieldsToResolve = await this._resolveDependentChanges(
        fieldsCollection.withRelevantDependentFields(fieldsToResolve),
      );

    // Step 11 – attach timestamps
    const finalData = this._useConfigProps(false);

    // Keep ctxValues up-to-date for handleSuccess
    this._updateCxtValues(finalData as never);

    return {
      data: finalData as O,
      error: null,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
    };
  }

  async delete(data: O, options: CtxOptions) {
    if (!areValuesOk(data)) return;

    let handlers: NS.DeleteHandler<O, CtxOptions>[] = [
      ...this.globalDeleteHandlers,
    ];

    for (const prop of getSetValuesAsProps(this.props)) {
      const handlers_ = this._getHandlers<NS.DeleteHandler<O, CtxOptions>>(
        prop,
        'onDelete',
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

  /**
   * Mirrors Rust's `update` method step-by-step.
   *
   * Steps:
   *  1. Validate the `values` and `changes` arguments
   *  2. Set previous values into ctxValues
   *  3. Clean / filter raw changes
   *  4. Filter allowed update fields  (filter_input_fields_allowed)
   *  5. Early-exit if relevant_fields_provided is empty  ("nothing to update")
   *  6. Rebuild ctx with post-filter state
   *  7. Evaluate missing required fields  (evaluate_missing_required_fields)
   *  8. Run primary validators  (validate)
   *  9. Re-filter: after validators, drop output fields whose value == old value
   * 10. Second early-exit if nothing remains
   * 11. Run secondary re-validators  (re_validate)
   * 12. Run post-validators  (post_validate)
   * 13. Sanitize virtuals  (sanitize_virtuals)
   * 14. Resolve dependent field values  (resolve_dependent_values)
   * 15. Get actual updates from partial (compare to previous values)
   * 16. Third early-exit if no actual updates
   * 17. Attach timestamps  (attach_timestamps)
   * 18. Build & return result
   */
  async update(values: O, changes: Partial<I>, options: CtxOptions) {
    const emptyErrorTool = new ErrorTool<ErrorMetadata>();

    // Step 1 – validate arguments
    if (!areValuesOk(values) || !areValuesOk(changes))
      return this._handleError(emptyErrorTool);

    // Step 3 – clean raw changes
    this._cleanInput(changes);

    // Build an initial ctx
    this._initContext({
      isUpdate: true,
      rawInput: changes,
      previousValues: values,
      options,
    });

    // Step 4 – filter which change fields are allowed for this update
    const fieldsCollection = await this._filterInputFieldsAllowed();

    // Capture previous values before any mutation for later comparison
    const previousPartial = cloneValue(values) as Partial<O>;

    // Step 5 – early-exit if nothing relevant to update
    // Use relevantConfigNames.size (not relevantFieldsProvided.size) because the
    // relevantFieldsProvided setter strips virtuals, so virtual-only changes would
    // be incorrectly treated as empty.
    if (!fieldsCollection.relevantConfigNames.size)
      return this._handleError(emptyErrorTool);

    // Step 7 – evaluate missing required fields
    const requiredError =
      await this._evaluateMissingRequiredFields(fieldsCollection);
    if (requiredError.hasErrors) return this._handleError(requiredError);

    // Step 8 – run primary validators over relevantConfigNames (includes virtuals)
    const primaryError = await this._runPrimaryValidators(fieldsCollection);

    if (primaryError.hasErrors) return this._handleError(primaryError);

    const updates: Partial<O> = cloneValue(this.ctxValues);

    // Step 9 – re-filter: after validators, drop output fields whose validated
    // value still equals the old value. Virtual (input-only) fields are kept.
    // Mirrors Rust lines 439-467.
    const reFilteredRelevant = new Set<string>();

    for (const fieldName of fieldsCollection.relevantFieldsProvided) {
      const fieldInfo = fieldsCollection.get(fieldName);

      // Virtual (input-only) fields are always kept
      if (!fieldInfo.isOutput) {
        reFilteredRelevant.add(fieldName);
        continue;
      }

      const updatedValue = (updates as any)[fieldInfo.name];
      const oldValue = (previousPartial as any)[fieldInfo.name];

      if (!isEqual(updatedValue, oldValue, this._options.equalityDepth)) {
        reFilteredRelevant.add(fieldName);
      } else {
        // Drop unchanged output field
        delete (updates as any)[fieldInfo.name];
      }
    }

    // Update fieldsCollection to reflect the re-filtered set
    fieldsCollection.relevantFieldsProvided = reFilteredRelevant;

    // Step 10 – second early-exit if nothing changed after validators
    if (!reFilteredRelevant.size) return this._handleError(emptyErrorTool);

    // Step 11 – run secondary (re-)validators
    const secondaryError =
      await this._handleSecondaryValidations(fieldsCollection);
    if (secondaryError.hasErrors) return this._handleError(secondaryError);

    // Step 12 – run post-validators
    const postValidationError =
      await this._handlePostValidations(fieldsCollection);
    if (postValidationError.hasErrors)
      return this._handleError(postValidationError);

    // Step 13 – sanitize virtuals
    await this._handleSanitizationOfVirtuals(fieldsCollection);

    // Step 14 – resolve dependent field values
    let fieldsToResolve = cloneValue(fieldsCollection.fieldsProvided);

    while (fieldsToResolve.size > 0)
      fieldsToResolve = await this._resolveDependentChanges(
        fieldsCollection.withRelevantDependentFields(fieldsToResolve),
      );

    // Step 15 – drop fields that still equal the old value after all resolvers
    for (const prop of getKeysAsProps(updates)) {
      if (
        isEqual(
          (updates as any)[prop],
          (previousPartial as any)[prop],
          this._options.equalityDepth,
        )
      ) {
        delete (updates as any)[prop];
      }
    }

    // Step 16 – third early-exit if no actual updates remain
    if (!Object.keys(updates).length) return this._handleError(emptyErrorTool);

    // Step 17 – attach timestamps
    const finalData = this._useConfigProps();

    this._updateCxtInput(finalData as never);

    return {
      data: finalData as Partial<O>,
      error: null,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
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

  create = (values: Partial<Input>, contextOptions: CtxOptions) =>
    this.modelFactory().create(values, contextOptions);

  delete = (values: Output, contextOptions: CtxOptions) =>
    this.modelFactory().delete(values, contextOptions);

  update = (
    values: Output,
    changes: Partial<Input>,
    contextOptions: CtxOptions,
  ) => this.modelFactory().update(values, changes, contextOptions);
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

  constructor(private _fields: Map<string, FieldInfo>) {}

  get(fieldName: string): FieldInfo {
    return this._fields.get(fieldName)!;
  }

  getUnsafe(fieldName: string): FieldInfo | null {
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
    const outputFieldsChanged = new Set<string>();

    for (const field_name of names) {
      const info = this.get(field_name);

      configNames.add(info.configName);

      if (info.isOutput) outputFieldsChanged.add(field_name);
    }

    this._relevantConfigNames = configNames;
    this._relevantFieldsProvided = outputFieldsChanged;
  }

  withRelevantDependentFields(names: Set<string>) {
    const col = cloneValue(this);

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

class FieldInfo {
  name: string;
  configName: string;
  isInput: boolean;
  isOutput: boolean;

  constructor({
    name,
    configName,
    isInput,
    isOutput,
  }: {
    name: string;
    configName: string;
    isInput: boolean;
    isOutput: boolean;
  }) {
    this.name = name;
    this.configName = configName;
    this.isInput = isInput;
    this.isOutput = isOutput;
  }

  get isVirtual() {
    return this.isInput && !this.isOutput;
  }
}
