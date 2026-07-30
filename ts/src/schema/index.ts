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
import { materializeFieldBuilders } from './fields';
import { defaultOptions, SchemaCore } from './schema-core';
import {
  type ArrayOfMinSizeTwo,
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
      materializeFieldBuilders(
        definitions as never,
      ) as never as NS.Definitions_<Input, Output, ErrorMetadata>,
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
          fieldInfo.isVirtual ||
          !isEqual(
            value,
            // @ts-expect-error ikr
            _previousValues[fieldName],
            this._options.equalityDepth,
          )
        ) {
          relevantFieldsProvided.add(fieldName);

          if (fieldInfo.isOutput) {
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

        if (fieldInfo.isOutput) {
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
          Promise.try(resolver, ctx).catch(() => false),
        ),
      )) {
        // if "task.value" is positive, it means "ignore"
        if (task.status === 'fulfilled' && task.value) return fieldsCollection;
      }

    fieldsCollection.fieldsProvided = fieldsProvided;
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    this._updateCxtInput(input);
    this._updateCxtValues(output);

    const tasks: [
      string[] | readonly string[],
      NS.Setter<boolean, I, O, CtxOptions>,
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
      } = this._getDefinition(configName);

      if (ignore) {
        tasks.push([[fieldName], ignore]);
        continue;
      }

      // readonly only restricts updates; creation is always allowed. This
      // must run before any ignoreInit/ignoreUpdate processing below (which
      // has its own early-continues), since a permanently-locked readonly
      // field must be dropped regardless of whether an ignoreInit/
      // ignoreUpdate rule is configured for it.
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
          if (fieldInfo?.isOutput) delete output[fieldName];

          continue;
        }
      }

      const source = isUpdate ? ignoreUpdate : ignoreInit;

      if (isUpdate && this._isRequired(fieldName)) {
        if (!readonly && typeof source === 'function')
          tasks.push([[fieldName], source]);

        continue;
      }

      if (!source) continue;

      if (typeof source === 'function') {
        tasks.push([[fieldName], source]);

        continue;
      }

      relevantFieldsProvided.delete(fieldName);

      // @ts-expect-error ikr
      delete input[fieldName];

      // @ts-expect-error ikr
      if (fieldInfo?.isOutput) delete output[fieldName];
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
        tasks.push([relevantConfigNames, config]);
      } else if (config && typeof config === 'object') {
        const fields = config.fields as string[];

        if (fields.some((name: string) => relevantConfigNames.includes(name)))
          tasks.push([fields, config.resolver]);
      }
    }

    if (isUpdate) {
      for (const config of toArray(this._options.ignoreUpdate ?? [])) {
        if (typeof config === 'function') {
          tasks.push([
            relevantConfigNames,
            (ctx: IvoContext<I, O, CtxOptions>) =>
              config(ctx.input as Partial<I>, ctx.previousValues as O, {
                options: ctx.options,
                updateOptions: ctx.updateOptions,
              }),
          ]);
        } else if (config && typeof config === 'object') {
          const fields = config.fields as string[];

          if (
            fields.some((name: string) => relevantConfigNames.includes(name))
          ) {
            tasks.push([
              fields,

              (ctx: IvoContext<I, O, CtxOptions>) =>
                config.resolver(
                  ctx.input as Partial<I>,
                  ctx.previousValues as O,
                  {
                    options: ctx.options,
                    updateOptions: ctx.updateOptions,
                  },
                ),
            ]);
          }
        }
      }
    }

    if (!tasks.length) {
      this._setCxtInput(input);
      this._setCxtValues(output);

      fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

      return fieldsCollection;
    }

    ctx = this._getContext();

    for (const [configNames, ignore] of await Promise.all(
      tasks.map(
        async ([names, resolver]) => [names, await resolver(ctx)] as const,
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

  private _setCxtInput = (updates: Partial<I>) => (this.ctxInput = updates);
  private _setCxtValues = (updates: Partial<O>) => (this.ctxValues = updates);

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
    const values: Partial<I> = {};

    for (const prop of props) values[prop] = input[prop] as never;

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

  private _getNotAllowedError(
    prop: string,
    value: unknown,
  ): InputFieldError<ErrorMetadata> {
    const allow = this._getDefinition(prop as never)?.allow;

    // Default metadata (used whenever the caller didn't supply their own via
    // an InputFieldError) exposes the allowed values so consumers of the
    // error can tell what would have been accepted.
    const values = Array.isArray(allow) ? allow : allow?.values;
    const defaultMetadata = { allowed: values } as never;

    if (Array.isArray(allow))
      return { reason: NotAllowedError, metadata: defaultMetadata };

    // @ts-expect-error: lol
    const error = allow?.error;

    if (isInputFieldError(error)) return error as never;

    if (isFunctionLike(error)) {
      let message: any;

      try {
        message = error(value, allow?.values);
      } catch {
        return { reason: NotAllowedError, metadata: defaultMetadata };
      }

      if (typeof message === 'string')
        return {
          reason: message || NotAllowedError,
          metadata: defaultMetadata,
        };

      return isInputFieldError(message)
        ? (message as never)
        : { reason: NotAllowedError, metadata: defaultMetadata };
    }

    return { reason: error || NotAllowedError, metadata: defaultMetadata };
  }

  private _handleError(errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>) {
    const options = this._getReadonlyCtx().options;
    return {
      data: null,
      error: errorTool.hasErrors
        ? this._options.sanitizeError(errorTool.payload, options)
        : null,
      options,
      handleFailure: this._makeHandleFailure(),
      handleSuccess: null,
    };
  }

  private _handleInvalidValue(
    errorTool: ErrorTool<ErrorMetadata>,
    name: string,
    validationResponse: InvalidValidatorResponse<ErrorMetadata>,
  ) {
    const { reason, metadata } = validationResponse;

    const fieldError = makeFieldError<ErrorMetadata>(
      reason || 'validation failed',
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.set(name, fieldError);
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

        // ── Lax (and conditionally-required, i.e. "requiredBy") fields that
        // were NOT provided in the filtered input ────────────────────────────
        // Mirrors Rust: `requiredBy` fields are just Lax fields with an extra
        // `required_fn`, so they're classified as `FieldType::Lax` there and
        // defaulted the same way. TS's `laxProps` set excludes anything with a
        // `required` rule (see `__isLax` in schema-core.ts), so requiredBy
        // fields must be included explicitly here (virtuals excluded: they
        // have no Output slot to default).
        if (
          (this._isLaxProp(configName) ||
            (this._isRequiredBy(configName) && !this._isVirtual(configName))) &&
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
            reason: 'validation failed',
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

    let configIds = new Set<string>();

    for (const configName of fieldsCollection.relevantConfigNames) {
      const config = this.propToPostValidationConfigIDsMap.get(configName);

      if (config) configIds = configIds.union(config);
    }

    const handlers = Array.from(configIds).map((id) => {
      return {
        validator: this.postValidationConfigMap.get(id)!.validators,
        properties: id.split(',') as ArrayOfMinSizeTwo<
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

        if (fieldInfo.isOutput) this._updateCxtValues(ctxUpdate);
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

  private async _evaluateMissingRequiredFields(
    fieldsCollection: FieldInfoCollection,
  ) {
    const ctx = this._getContext();

    const isUpdate = ctx.isUpdate;
    const errorTool = new ErrorTool<ErrorMetadata>();

    // Mirrors Rust's `evaluate_missing_required_fields`: conditionally-required
    // (`required: fn`) fields are evaluated on both create & update; strictly
    // required (`required: true`) fields are only ever evaluated at creation
    // (Rust's `FieldType::Required` match arm is guarded by `if !is_update`).
    const propsToEvaluate = new Set<KeyOf<I>>(this.propsRequiredBy);

    if (!isUpdate)
      for (const prop of this.requiredProps) propsToEvaluate.add(prop);

    await Promise.allSettled([
      ...Array.from(propsToEvaluate).map(async (prop) => {
        // Mirrors Rust's `is_relevant_config_name`: a field that was provided
        // but got ignored (or, during updates, whose value didn't actually
        // change) should still be treated as "not provided" for required
        // checks — unlike raw `fieldsProvided`, `relevantConfigNames` already
        // accounts for ignore/ignoreInit/ignoreUpdate/readonly filtering.
        if (fieldsCollection.relevantConfigNames.has(prop)) return;

        // Readonly fields are only re-evaluated for required-ness while
        // still updatable (i.e. their value still equals the default), and
        // virtuals are exempt entirely once blocked by an (unconditional or
        // conditional) `ignoreUpdate` rule — `_isUpdatable` answers both
        // reliably via its own dedicated branches for these two cases. Plain
        // (non-readonly, non-virtual) fields have no such gate: their own
        // `required` callback is the sole authority on whether they're
        // missing — `_isUpdatable`'s fallback for that case just compares
        // against the untouched `ctxValues` slot, which isn't meaningful
        // here (a field that was never touched trivially "equals" itself).
        if (
          isUpdate &&
          (this._isReadonly(prop) || this._isVirtual(prop)) &&
          !this._isUpdatable(prop, (ctx.rawInput as never)?.[prop])
        )
          return;

        const [isRequired, message] = await Promise.try(
          this._getRequiredState,
          prop,
          ctx as never,
        );

        if (!isRequired) return;

        // A strictly-required field with an `allow` list has no value at
        // all when missing, and `undefined` is never in that list — so the
        // more specific "value not allowed" error (with its metadata) takes
        // precedence over the generic "is required" message.
        if (
          this._isRequired(prop) &&
          this._getDefinition(prop as never)?.allow
        ) {
          const notAllowedError = makeFieldError<ErrorMetadata>(
            this._getNotAllowedError(prop, undefined),
          );

          errorTool.set(
            (this._getAliasByVirtual(prop) ?? prop) as never,
            notAllowedError,
          );

          return;
        }

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

      // Mirrors Rust's grouped `options.required`: a config's handler(s) only
      // run when NONE of its fields are currently relevant/provided — the
      // handler then decides which (if any) of the group's fields are missing.
      // Errors for fields outside the config's own `properties` are dropped,
      // matching Rust's `field_names.contains(...)` filter.
      ...toArray(this._options.required ?? []).map(async (config) => {
        const properties = config.properties as KeyOf<I>[];

        if (
          properties.some((prop) =>
            fieldsCollection.relevantConfigNames.has(prop),
          )
        )
          return;

        // Resolve each declared (config-name) property to the name a handler
        // would actually use — the alias, for aliased virtuals, since
        // `KeyOf<Input>` reflects the alias rather than the internal name.
        const resolvedNames = new Set(
          properties.map((prop) => this._getAliasByVirtual(prop) ?? prop),
        );

        const results = await Promise.allSettled(
          toArray(config.handler).map((handler) =>
            Promise.try(handler, ctx as never),
          ),
        );

        for (const result of results) {
          if (result.status !== 'fulfilled' || !result.value) continue;

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
      // Compare against the actual previous record (`ctxPreviousValues`),
      // not `ctxValues` — the latter only reflects fields that were already
      // touched/relevant in this call, so an untouched readonly field would
      // wrongly compare its default against `undefined` instead of its real
      // previous value.
      return isEqual(
        this.defaults[propName],
        (this.ctxPreviousValues as never)?.[propName],
        this._options.equalityDepth,
      );

    return !isEqual(
      this.ctxValues[propName],
      value,
      this._options.equalityDepth,
    );
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
          'onFailure',
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
    const ctx = this._getReadonlyCtx();

    const relevantFields = new Set<string>(
      fieldsCollection.relevantConfigNames,
    );

    // Mirrors Rust's `prepare_success_handlers`: every field present in the
    // final output (`ctx.values` at creation, `ctx.changes` at update) is
    // eligible for onSuccess — not just directly-provided fields. A dependent
    // field's onSuccess fires whenever the field is included in the output,
    // regardless of whether its resolver actually ran (it's always present,
    // resolved or at its default).
    const candidateFieldNames = ctx.isUpdate ? ctx.changes : ctx.values;

    for (const name of Object.keys(candidateFieldNames ?? {}))
      relevantFields.add(name);

    const setOfSuccessHandlerIDs = new Set<string>();

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
    const toResolve = new Set<string>();

    for (const configName in this._definitions) {
      const dependsOn = this._definitions[configName]?.dependsOn;

      if (!dependsOn) continue;

      if (
        toArray(dependsOn).some((parent) =>
          fieldsCollection.relevantDependentConfigNames.has(parent),
        )
      )
        toResolve.add(configName);
    }

    const fieldsResolved = new Set<string>();
    const values = cloneValue<Partial<O>>(ctx.values);

    await Promise.allSettled(
      toResolve.values().map(async (name) => {
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
          // @ts-expect-error ikr
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

  private async _validate(
    { configName }: Pick<FieldInfo, 'configName'>,
    value: unknown,
    ctx: IvoContext<I, O, CtxOptions>,
  ) {
    const allowedValues = this.propsToAllowedValuesMap.get(configName);

    if (allowedValues && !allowedValues.has(value)) {
      const fieldError = makeFieldError<ErrorMetadata>(
        this._getNotAllowedError(configName, value),
      );

      return makeResponse<any, ErrorMetadata>({
        valid: false,
        value,
        reason: fieldError.reason,
        metadata: fieldError.metadata,
      });
    }

    const validator = this._getPrimaryValidator(configName);

    if (validator) {
      let res: ValidatorResponseObject<any, ErrorMetadata>;

      try {
        res = this._sanitizeValidationResponse<any>(
          (await Promise.try(
            validator,
            value,
            ctx as never,
          )) as ValidatorResponseObject<any, ErrorMetadata>,
          value,
        );
      } catch {
        return makeResponse<any>({ valid: false, reason: 'validation failed' });
      }

      if (allowedValues && res.valid && !allowedValues.has(res.validated))
        return makeResponse<any, ErrorMetadata>({
          valid: true,
          validated: value,
        });

      return res;
    }

    return makeResponse<any, ErrorMetadata>({ valid: true, validated: value });
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

    // Build an initial ctx
    this._initContext({
      isUpdate: false,
      // clean raw input (strips unknown and non-input fields)
      rawInput: this._cleanInput(input),
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
    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0)
      collection = await this._resolveDependentChanges(collection);

    // Step 11 – attach timestamps
    const data = this._useConfigProps(false) as O;

    // Keep ctxValues up-to-date for handleSuccess
    this._updateCxtValues(data);

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

    // Build an initial ctx
    this._initContext({
      isUpdate: true,
      rawInput: this._cleanInput(changes),
      previousValues: values,
      options,
    });

    // Step 4 – filter which change fields are allowed for this update
    const fieldsCollection = await this._filterInputFieldsAllowed();

    // Capture previous values before any mutation for later comparison
    const previousPartial = cloneValue(values) as Partial<O>;

    // Step 5 – early-exit if nothing relevant to update
    // Use relevantConfigNames.size (not relevantFieldsProvided.size) because the
    // relevantFieldsProvided NS.setter strips virtuals, so virtual-only changes would
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

    let updates: Partial<O> = cloneValue(this.ctxValues);

    // Step 9 – re-filter: after validators, drop output fields whose validated
    // value still equals the old value. Virtual (input-only) fields are kept.
    // Mirrors Rust lines 439-467. Iterate `relevantConfigNames` (not
    // `relevantFieldsProvided`, which the NS.setter narrows to output fields
    // only) so virtual fields actually reach the "not output, always keep"
    // branch below instead of being silently absent from the loop entirely.
    const reFilteredRelevant = new Set<string>();

    // Iterate `relevantFieldsProvided` (the literal keys the caller actually
    // used — alias or config name) rather than `relevantConfigNames`, so a
    // virtual provided via its alias stays keyed by that alias afterwards —
    // needed downstream (secondary validators, post-validators, sanitizers,
    // handleSuccess) for correct alias-aware error reporting / lookups.
    for (const fieldName of fieldsCollection.relevantFieldsProvided) {
      const fieldInfo = fieldsCollection.get(fieldName);

      // Virtual (input-only) fields are always kept
      if (!fieldInfo.isOutput) {
        reFilteredRelevant.add(fieldName);
        continue;
      }

      const updatedValue = (updates as any)[fieldInfo.configName];
      const oldValue = (previousPartial as any)[fieldInfo.configName];

      if (!isEqual(updatedValue, oldValue, this._options.equalityDepth)) {
        reFilteredRelevant.add(fieldName);
      } else {
        // Drop unchanged output field
        delete (updates as any)[fieldInfo.configName];
      }
    }

    // Update fieldsCollection to reflect the re-filtered set
    fieldsCollection.relevantFieldsProvided = reFilteredRelevant;
    this._setCxtValues(updates);

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
    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0)
      collection = await this._resolveDependentChanges(collection);

    updates = cloneValue(this.ctxValues);

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

    this._setCxtValues(updates);

    // Step 16 – third early-exit if no actual updates remain
    if (!Object.keys(updates).length) return this._handleError(emptyErrorTool);

    // Step 17 – attach timestamps
    const data = this._useConfigProps(true);

    this._updateCxtValues(data as never);

    return {
      data,
      error: null,
      options: this._getReadonlyCtx().options,
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
