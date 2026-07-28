import {
  getKeysAsProps,
  getSetValuesAsProps,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  makeResponse,
  type ObjectType,
  sort,
  sortKeys,
  toArray,
} from "../utils";
import { defaultOptions, SchemaCore } from "./schema-core";
import {
  type DefinitionRule,
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
} from "./types";
import {
  cloneValue,
  type DefaultFieldErrorMetadata,
  ErrorTool,
  type FieldError,
  type InputFieldError,
  isInputFieldError,
  makeFieldError,
} from "./utils";

export { Model, ModelTool, Schema };

export type IvoResultInfo<
  T extends Model<any, any, any>,
  Operation extends "create" | "update" = "create",
> =
  | {
      data: NonNullable<Awaited<ReturnType<T[Operation]>>["data"]>;
      error: null;
    }
  | {
      data: null;
      error: NonNullable<Awaited<ReturnType<T[Operation]>>["error"]>;
    };

const NotAllowedError = "value not allowed";
const validationFailedFieldError = makeFieldError("validation failed");

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
          (prop) => ![...LIFE_CYCLES, "shouldUpdate"].includes(prop as never),
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
  private _regeneratedProps: KeyOf<O>[] = [];
  private partialContext = {} as I;

  // contexts & values
  private ctxInput: Partial<I> = {};
  private ctxRawInput: Partial<I> = {};
  private ctxChanges: Partial<O> = {};
  private ctxValues: Partial<O> = {};
  private _ctxOptions: CtxOptions = {} as CtxOptions;

  constructor(schema: Schema<I, O, CtxOptions, ErrorMetadata, ErrorPayload>) {
    super(schema.definitions as never, schema.options as never);
  }

  private _getFieldInfoCollection(): FieldInfoCollection {
    const fields: Map<string, FieldInfo> = new Map();

    for (const configName of getKeysAsProps(this._definitions)) {
      const config = this._definitions[configName]!;

      if (config.constant || config.dependsOn) continue;

      let isVirtual = !!config.virtual;

      fields.set(configName, {
        name: configName,
        configName,
        isInput: true,
        isOutput: !isVirtual,
      });

      let aliasName = config.alias;

      if (aliasName)
        fields.set(aliasName, {
          name: aliasName,
          configName,
          isInput: true,
          isOutput: !isVirtual,
        });
    }

    return new FieldInfoCollection(fields);
  }

  private async _filterInputFieldsAllowed(
    _previousValues: O | null,
    ctx: IvoContext<I, O, CtxOptions>,
  ): Promise<{
    input: Partial<I>;
    output: Partial<O>;
    fieldsCollection: FieldInfoCollection;
  }> {
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
        if (config.fields.length === 0) entityResolvers.push(config.resolver);
      }

      for (const [fieldName, value] of Object.entries(rawInput)) {
        let fieldInfo = fieldsCollection.get(fieldName);

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
        let fieldInfo = fieldsCollection.get(fieldName);

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
          Promise.try(resolver, rawInput, _previousValues as O, {
            options: ctx.options,
            updateOptions: ctx.updateOptions,
          }).catch(() => false),
        ),
      )) {
        // if "task.value" is positive, it means "ignore"
        if (task.status === "fulfilled" && task.value)
          return {
            input,
            output,
            fieldsCollection,
          };
      }
    }

    fieldsCollection.fieldsProvided = fieldsProvided;
    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    const tasks: [string[] | readonly string[], Promise<boolean>][] = [];

    for (const fieldName of fieldsCollection.relevantFieldsProvided.values()) {
      let fieldInfo = fieldsCollection.get(fieldName);

      const {
        default: defaultValue,
        ignore,
        ignoreInit,
        ignoreUpdate,
        readonly,
      } = this._getDefinition(fieldInfo.configName);

      if (ignore) {
        tasks.push([[fieldInfo.name], Promise.try(ignore, ctx)]);
        continue;
      }

      let source = isUpdate ? ignoreUpdate : ignoreInit;

      if (source === undefined) continue;

      if (!source) {
        relevantFieldsProvided.delete(fieldName);

        // @ts-expect-error ikr
        delete input[fieldName];

        if (fieldInfo.isOutput) {
          // @ts-expect-error ikr
          delete output[fieldName];
        }

        continue;
      }

      if (readonly) {
        if (defaultValue === undefined || typeof defaultValue === "function") {
          continue;
        }

        // readonly means: only allow update if value previousValue == defaultValue

        // @ts-expect-error ikr
        if (isEqual(previousValues[fieldName], defaultValue)) {
          continue;
        }

        relevantFieldsProvided.delete(fieldName);

        // @ts-expect-error ikr
        delete input[fieldName];

        if (fieldInfo.isOutput) {
          // @ts-expect-error ikr
          delete output[fieldName];
        }

        continue;
      }

      tasks.push([[fieldInfo.name], Promise.try(source, ctx)]);
    }

    let relevantConfigNames = relevantFieldsProvided
      .values()
      .map((name) => fieldsCollection.get(name).configName);

    for (const config of toArray(this._options.ignore ?? [])) {
      if (
        config.fields.some(
          (name) => !!relevantConfigNames.find((n) => n === name),
        )
      )
        tasks.push([
          config.fields,
          Promise.try(config.resolver, ctx).catch(() => false),
        ]);
    }

    if (isUpdate) {
      for (const config of toArray(this._options.ignoreUpdate ?? [])) {
        if (
          config.fields.some(
            (name) => !!relevantConfigNames.find((n) => n === name),
          )
        )
          tasks.push([
            config.fields,
            Promise.try(config.resolver, rawInput, _previousValues as O, {
              options: ctx.options,
              updateOptions: ctx.updateOptions,
            }).catch(() => false),
          ]);
      }
    }

    if (!tasks.length) {
      fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

      return {
        input: rawInput,
        output,
        fieldsCollection,
      };
    }

    for (const [configNames, ignore] of await Promise.all(
      tasks.map(async ([names, promise]) => [names, await promise] as const),
    )) {
      for (const configName of configNames) {
        let fieldInfo = fieldsCollection.get(configName);
        let fieldName = fieldInfo.name;

        if (ignore) {
          // @ts-expect-error ikr
          input[fieldName] = undefined;

          relevantFieldsProvided.delete(fieldName);

          if (fieldInfo.isOutput) {
            // @ts-expect-error ikr
            output[fieldName] = undefined;
          }

          continue;
        }

        relevantFieldsProvided.add(fieldName);
      }
    }

    fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

    return {
      input: rawInput,
      output,
      fieldsCollection,
    };
  }

  private _getReadonlyCtx({
    data,
    rawInput,
    isUpdate,
  }: {
    data: Partial<O>;
    rawInput: Partial<RealType<I>>;
    isUpdate: boolean;
  }) {
    const changes = isUpdate ? cloneValue(data) : null,
      previousValues = isUpdate ? cloneValue(this.ctxValues) : null,
      input = this._getFrozenCopy(cloneValue(rawInput)),
      values = this._getFrozenCopy(
        cloneValue(
          isUpdate
            ? Object.assign({}, previousValues, this.ctxValues, data)
            : Object.assign({}, this.defaults, data),
        ),
      );

    return this._getFrozenCopy({
      changes,
      input,
      rawInput: cloneValue(rawInput),
      isUpdate,
      previousValues,
      values,
      options: this._getCtxOptions(),
    }) as ReadonlyIvoContext<I, O, CtxOptions>;
  }

  private _getContext(props: {
    data: Partial<O>;
    rawInput: Partial<RealType<I>>;
    isUpdate: boolean;
  }) {
    return this._getFrozenCopy(
      Object.assign({}, this._getReadonlyCtx(props), {
        updateOptions: this._updateCtxOptions,
      }),
    ) as never as IvoContext<I, O, CtxOptions>;
  }

  private _getPartialContext = () => this._getFrozenCopy(this.partialContext);

  private _updateContext = (updates: Partial<I>) => {
    Object.assign(this.ctxInput, updates);
  };

  private _updatePartialContext = (updates: Partial<I>) => {
    Object.assign(this.partialContext, updates);
  };

  private _getCtxOptions = () => this._getFrozenCopy(this._ctxOptions);

  private _updateCtxOptions = (options: Partial<CtxOptions>) => {
    if (isRecordLike(options)) Object.assign(this._ctxOptions, options);

    return this._getCtxOptions();
  };

  private _getDefaultValue = async (prop: string) => {
    const _default = this._getDefinition(prop)?.default;

    let value: any;

    try {
      // value = isFunctionLike(_default)
      //   ? await Promise.try(() => _default(this._getLegacyContext()))
      //   : this.defaults[prop as KeyOf<O>];
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

    if (!required) return [false, ""];

    const fallbackMessage = `'${prop}' is required`;

    if (!isFunctionLike(required)) return [required, fallbackMessage];

    const results = await required(ctx);
    const isBoolean = typeof results === "boolean";

    if (!isBoolean && !Array.isArray(results)) return [false, ""];

    if (isBoolean) return [results as boolean, results ? fallbackMessage : ""];

    const [isRequired, message] = results as [
        boolean,
        string | InputFieldError<ErrorMetadata>,
      ],
      isString = typeof message === "string";

    if (!isRequired || (!isString && !isInputFieldError(message)))
      return [isRequired, fallbackMessage];

    if (isString) return [true, message || fallbackMessage];

    const fieldError = makeFieldError<ErrorMetadata>(message, fallbackMessage);

    return [
      true,
      isPropertyOf("metadata", message)
        ? fieldError
        : ({ reason: fieldError.reason } as never),
    ];
  };

  private _getConstantValue = (prop: string) => this._getValueBy(prop, "value");

  private _getValueBy = (prop: string, rule: DefinitionRule) => {
    const value = this._getDefinition(prop)?.[rule];

    return value;
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
      }
    }

    this.ctxRawInput = values;

    return values;
  }

  private async _generateConstants() {
    const data = {} as Partial<O>;

    await Promise.allSettled(
      getSetValuesAsProps(this.constants).map(async (prop) => {
        try {
          // data[prop] = await Promise.try(this._getConstantValue, prop);
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

    return ignore
      ? ignore(
          this._getContext({
            data: {},
            rawInput: this.ctxRawInput,
            isUpdate,
          }),
        )
      : undefined;
  };

  private _isInitAllowed = (prop: string, extraCtx: ObjectType = {}) => {
    if (isOneOf(this._getDefinition(prop).ignoreInit, [true, undefined]))
      return true;

    return this._getValueBy(prop, "ignoreInit") === true;
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

  private _ignoreUpdate = (prop: string, extraCtx: ObjectType = {}) => {
    if (isOneOf(this._getDefinition(prop).ignoreUpdate, [true, undefined]))
      return true;

    return this._getValueBy(prop, "ignoreUpdate") === true;
  };

  private _isVirtualInit = (prop: string, value: unknown = undefined) => {
    const isAlias = this._isVirtualAlias(prop);

    if (!this._isVirtual(prop) && !isAlias) return false;

    const definitionName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    const { ignoreInit } = this._getDefinition(definitionName);

    const extraCtx = isAlias ? { [definitionName]: value } : {};

    return (
      isEqual(ignoreInit, undefined) ||
      this._isInitAllowed(definitionName, extraCtx)
    );
  };

  private _getValidationSummary = (isUpdate: boolean) =>
    this._getContext({
      data: this.ctxValues,
      isUpdate,
      rawInput: this.ctxRawInput,
    });

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

      if (typeof message === "string") return message || NotAllowedError;

      return isInputFieldError(message) ? message : NotAllowedError;
    }

    return error || NotAllowedError;
  }

  private _handleError(
    ctx: ReadonlyIvoContext<I, O, CtxOptions>,
    options: CtxOptions,
    errorTool: ErrorTool<ErrorMetadata, KeyOf<I>>,
  ) {
    return {
      data: null,
      error: this._options.sanitizeError(errorTool.payload, options),
      handleFailure: this._makeHandleFailure(ctx, options),
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
      reason || "validation failed",
    );

    if (metadata) fieldError.metadata = metadata;

    errorTool.set(prop, fieldError);
  }

  private async _handleCreationPrimaryValidations(
    data: Partial<O>,
    input: never,
  ) {
    const errorTool = new ErrorTool<ErrorMetadata>();

    const virtuals = getKeysAsProps<Partial<O>>(input).filter((prop) =>
      this._isVirtualInit(prop, input[prop as unknown as KeyOf<I>]),
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
            errorTool,
            prop,
            input[prop as unknown as KeyOf<I>],
          );
        }

        const isProvided = isPropertyOf(prop, this.ctxValues),
          isLax = this._isLaxProp(prop),
          isLaxInit = isLax && isProvided,
          isRequiredInit = this._isRequiredBy(prop) && isProvided,
          canInit = this._canInit(prop);

        if (
          (isLax && (!canInit || (canInit && !isProvided))) ||
          (!isVirtualInit && !canInit && !isLaxInit && !isRequiredInit)
        ) {
          data[prop] = await Promise.try(this._getDefaultValue, prop);

          const validCtxUpdate = { [prop]: data[prop] as never } as never;
          this._updatePartialContext(validCtxUpdate);

          return this._updateContext(validCtxUpdate);
        }

        return this._validateAndSet(
          data,
          errorTool,
          prop,
          this.ctxValues[prop],
        );
      }),
    );

    return { data, error: errorTool, virtuals };
  }

  private async _handleUpdatePrimaryValidations(changes: Partial<I>) {
    const error = new ErrorTool<ErrorMetadata>();

    const updates = {} as Partial<O>;

    const toUpdate = Array.from(
      new Set(getKeysAsProps<O>(changes as never)),
    ).filter((prop) => this._isUpdatable(prop, (changes as never)[prop]));

    await Promise.allSettled(
      toUpdate.map(async (prop) => {
        const value = (changes as never)[prop] as O[KeyOf<O>];

        const isValid = (await Promise.try(() =>
          this._validate(
            prop as never,
            value,
            this._getValidationSummary(true),
          ),
        )) as InternalValidatorResponse<O[KeyOf<O>], ErrorMetadata>;

        if (!isValid.valid)
          return this._handleInvalidValue(error, prop, isValid);

        let { validated } = isValid;

        if (isEqual(validated, undefined)) validated = value;

        const isAlias = this._isVirtualAlias(prop);

        const propName = (isAlias
          ? this._getVirtualByAlias(prop)!
          : prop) as unknown as KeyOf<O>;

        if (
          isEqual(
            validated,
            this.ctxValues[propName],
            this._options.equalityDepth,
          )
        )
          return;

        if (!this._isVirtual(propName))
          updates[propName as KeyOf<O>] = validated;

        const validCtxUpdate = { [propName]: validated } as never;

        this._updateContext(validCtxUpdate);
        this._updatePartialContext(validCtxUpdate);
      }),
    );

    return { error, updates };
  }

  private async _handleSecondaryValidations(
    data: Partial<O>,
    isUpdate = false,
  ) {
    const ctx = this._getContext({
      data,
      isUpdate,
      rawInput: this.ctxRawInput,
    });

    const error = new ErrorTool<ErrorMetadata>();

    const props: [KeyOf<O>, string | undefined][] = [];

    for (const prop of this.propsWithSecondaryValidators.values()) {
      if (!isUpdate && !this._isInitAllowed(prop)) continue;

      const alias = this._getAliasByVirtual(prop as never);

      if (!this._isSuccessfulProp(prop, ctx, alias)) continue;

      props.push([prop as KeyOf<O>, alias]);
    }

    await Promise.allSettled(
      props.map(async ([prop, alias]) => {
        const validator = this._getSecondaryValidator(prop);

        if (!validator) return;

        // @ts-expect-error ikr
        const value = ctx.input?.[prop] as never as O[KeyOf<O>];

        let isValid: ValidatorResponseObject<unknown, ErrorMetadata>;

        try {
          isValid = this._sanitizeValidationResponse<unknown>(
            (await Promise.try(
              validator,
              value,
              ctx as never,
            )) as ValidatorResponseObject<unknown, ErrorMetadata>,
            value,
          );
        } catch {
          isValid = makeResponse<unknown, ErrorMetadata>({
            valid: false,
            reason: "validation failed",
          });
        }

        if (!isValid.valid) {
          const _prop =
            alias && isPropertyOf(alias, ctx.rawInput) ? alias : prop;

          return this._handleInvalidValue(error, _prop as never, isValid);
        }

        let { validated } = isValid;

        if (isEqual(validated, undefined)) validated = value;

        // @ts-expect-error ikr
        if (isEqual(validated, ctx.input[prop], this._options.equalityDepth))
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
    ctx: IvoContext<I, O, CtxOptions>,
    alias_?: string,
  ) {
    if (this._isVirtual(prop)) {
      if (isPropertyOf(prop, this.partialContext)) return true;

      const alias = alias_ || this._getAliasByVirtual(prop as never);

      return !isNullOrUndefined(alias) && isPropertyOf(alias, ctx.rawInput);
    }

    return !ctx.isUpdate || isPropertyOf(prop, ctx.changes);
  }

  private async _handlePostValidations(data: Partial<O>, isUpdate = false) {
    const summary = this._getContext({
      data,
      isUpdate,
      rawInput: this.ctxRawInput,
    });

    const errorTool = new ErrorTool<ErrorMetadata>();

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
      ) as KeyOf<I>[],
    }));

    const handleRevalidatedData = (revalidatedData: Partial<O> | null) => {
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
          keyof I,
          string
        >[];

        if (!Array.isArray(validator)) {
          const { revalidatedData, success } = await Promise.try(() =>
            this._handlePostValidator({
              errorTool,
              propsProvided,
              ctx: summary,
              validator: validator as any,
              postValidatableProps,
            }),
          );

          if (!success || !revalidatedData) return;

          return handleRevalidatedData(revalidatedData);
        }

        for (const v1 of validator) {
          if (Array.isArray(v1)) {
            const summary = this._getContext({
              data: this.ctxValues,
              isUpdate,
              rawInput: this.ctxRawInput,
            });

            const results = await Promise.all(
              v1.map(async (v2) => {
                const res = await Promise.try(() =>
                  this._handlePostValidator({
                    errorTool,
                    propsProvided,
                    ctx: summary,
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
            ctx: this._getContext({
              data: this.ctxValues,
              isUpdate,
              rawInput: this.ctxRawInput,
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

  private async _handleRequiredBy(ctx: ReadonlyIvoContext<I, O, CtxOptions>) {
    const isUpdate = ctx.isUpdate;
    const errorTool = new ErrorTool<ErrorMetadata>();

    await Promise.allSettled(
      Array.from(this.propsRequiredBy.keys()).map(async (prop) => {
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
    data: Partial<O>,
    isUpdate = false,
  ) {
    const sanitizers: [KeyOf<I>, Function][] = [];

    const partialCtx = this._getPartialContext();

    const successFulVirtuals = getKeysAsProps(partialCtx).filter(
      this._isVirtual,
    );

    for (const prop of successFulVirtuals) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, !isUpdate);

      if (isSanitizable) sanitizers.push([prop as KeyOf<I>, sanitizer]);
    }

    const summary = this._getContext({
      data,
      isUpdate,
      rawInput: this.ctxRawInput,
    });

    await Promise.allSettled(
      sanitizers.map(async ([prop, sanitizer]) => {
        // @ts-expect-error
        const resolvedValue = await Promise.try(sanitizer, summary);

        this._updateContext({ [prop]: resolvedValue } as never);
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

  private _isSanitizable(
    prop: string,
    isCreation: boolean,
  ): [false, undefined] | [true, Function] {
    const { sanitizer, ignoreInit } = this._getDefinition(prop);

    if (!sanitizer) return [false, undefined];

    if (isCreation && isEqual(ignoreInit, false)) return [false, undefined];

    return [true, sanitizer];
  }

  private _isGloballyUpdatable(changes: unknown) {
    const { ignoreUpdate = defaultOptions.ignoreUpdate! } = this._options;

    if (typeof ignoreUpdate === "boolean") return ignoreUpdate;

    // return !ignoreUpdate(
    //   this._getMutableSummary({
    //     data: changes as never,
    //     isUpdate: true,
    //     rawInput: this.rawInput,
    //   }) as never,
    // );

    return false;
  }

  private _isUpdatable(prop: string, value: unknown = undefined) {
    if (!this._isInputOrAlias(prop)) return false;

    const isAlias = this._isVirtualAlias(prop);

    const propName = (
      isAlias ? this._getVirtualByAlias(prop)! : prop
    ) as KeyOf<O>;

    if (
      this._isIngnorable(propName) &&
      this._shouldIgnore({ prop: propName, isUpdate: true })
    )
      return false;

    const hasIgnoreUpdateRule = this._isRuleInDefinition(
      propName,
      "ignoreUpdate",
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

  private _makeHandleFailure(
    ctx: ReadonlyIvoContext<I, O, CtxOptions>,
    options: CtxOptions,
  ) {
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
        cleanups.map(
          async (h) =>
            await Promise.try(
              h,
              this._getFrozenCopy(ctx),
              this._getFrozenCopy(options),
            ),
        ),
      );
    };
  }

  private _makeHandleSuccess(data: Partial<O>, isUpdate = false) {
    const partialCtx = this._getPartialContext(),
      successProps = getKeysAsProps(partialCtx),
      summary = this._getReadonlyCtx({
        data,
        isUpdate,
        rawInput: this.ctxRawInput,
      }),
      setOfSuccessHandlerIDs = new Set<string>();

    let successListeners = [] as NS.SuccessHandler<I, O, CtxOptions>[];

    for (const prop of successProps) {
      const handlers = this._getHandlers<NS.SuccessHandler<I, O>>(
        prop,
        "onSuccess",
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
        successListeners.map(async (h) => await Promise.try(h, summary)),
      );
    };
  }

  private async _resolveDependentChanges(
    data: Partial<O>,
    ctx: I,
    isUpdate = false,
  ) {
    const isCreation = !isUpdate;
    const successFulChanges = getKeysAsProps<O>(ctx as never);
    let _updates = Object.assign({}, data);
    let toResolve = [] as KeyOf<O>[];

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

    const values = isUpdate ? data : Object.assign({}, this.ctxValues, data),
      _ctx = this._getContext({
        data: values,
        isUpdate,
        rawInput: this.ctxRawInput,
      });

    await Promise.allSettled(
      toResolve.map(async (prop) => {
        if (
          this._isReadonly(prop) &&
          !isCreation &&
          !isEqual(
            this.ctxValues[prop],
            this.defaults[prop],
            this._options.equalityDepth,
          )
        )
          return;

        const resolver = this._getDefinition(prop).resolver!;
        let value: any;

        try {
          value = await Promise.try(resolver as any, _ctx);
        } catch {
          value = isCreation ? null : _ctx.previousValues?.[prop];
        }

        if (
          !isCreation &&
          isEqual(
            value,
            // @ts-expect-error ikr
            _ctx.input[prop as KeyOf<I>],
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
    values: Partial<I | O>,
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

    this.ctxValues = _values as O;
    this.ctxInput = Object.assign({}, this.defaults, this.ctxValues) as never;
  }

  private async _setMissingDefaults() {
    this._regeneratedProps = getSetValuesAsProps(this.props).filter((prop) => {
      return (
        this._isDefaultable(prop) && isEqual(this.ctxValues[prop], undefined)
      );
    });

    await Promise.allSettled(
      this._regeneratedProps.map(async (prop) => {
        const value = await Promise.try(this._getDefaultValue, prop);

        this._updateContext({ [prop]: value } as never);
        this._updatePartialContext({ [prop]: value } as never);
      }),
    );
  }

  private _setValidValue(
    prop: KeyOf<O>,
    value: O[KeyOf<O>],
    operationData: Partial<O> = {},
  ) {
    const isAlias = this._isVirtualAlias(prop),
      propName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    if (!this._isVirtual(propName)) operationData[propName as KeyOf<O>] = value;

    const validCtxUpdate = { [propName]: value } as never;

    this._updateContext(validCtxUpdate);
    this._updatePartialContext(validCtxUpdate);
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

  private _useConfigProps(obj: Partial<O>, isUpdate = false) {
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
    operationData: Partial<O>,
    errorTool: ErrorTool<ErrorMetadata>,
    prop: KeyOf<O>,
    value: unknown,
  ) {
    const isValid = (await Promise.try(() =>
      this._validate(prop as never, value, this._getValidationSummary(false)),
    )) as InternalValidatorResponse<O[KeyOf<O>], ErrorMetadata>;

    if (isValid.valid)
      return this._setValidValue(prop, isValid.validated, operationData);

    this._handleInvalidValue(errorTool, prop, isValid);
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
        reason: "Invalid property",
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
          reason: "validation failed",
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

  async create(input: Partial<I>, contextOptions: CtxOptions) {
    const options = this._updateCtxOptions(contextOptions);

    if (!areValuesOk(input)) input = {};

    const _input = this._cleanInput(input);

    this._setValues(_input);

    let data = await Promise.try(() => this._generateConstants());

    const ctx = this._getContext({
      data: this.ctxValues,
      isUpdate: false,
      rawInput: this.ctxRawInput,
    });

    const { fieldsCollection, ...rest } = await this._filterInputFieldsAllowed(
      null,
      ctx,
    );

    const {
      data: dt,
      error,
      virtuals,
    } = await Promise.try(() =>
      this._handleCreationPrimaryValidations(data, _input),
    );

    if (error.hasErrors) return this._handleError(ctx, options, error);

    data = dt;

    const requiredError = await Promise.try(() => this._handleRequiredBy(ctx));
    if (requiredError.hasErrors)
      return this._handleError(ctx, options, requiredError);

    const error2 = await Promise.try(() =>
      this._handleSecondaryValidations(data),
    );
    if (error2.hasErrors) return this._handleError(ctx, options, error2);

    const postValidationError = await Promise.try(() =>
      this._handlePostValidations(data),
    );
    if (postValidationError.hasErrors)
      return this._handleError(ctx, options, postValidationError);

    await this._handleSanitizationOfVirtuals(data);
    data = await Promise.try(() =>
      this._resolveDependentChanges(data, this._getPartialContext()),
    );

    const finalData = this._useConfigProps(data);

    this._updateContext(finalData as never);
    this._updatePartialContext(finalData as never);

    return {
      data: finalData as O,
      error: null,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(finalData),
    };
  }

  async delete(data: O, options: CtxOptions) {
    if (!areValuesOk(data)) return;

    this._setValues(data, { allowVirtuals: false, allowTimestamps: true });

    let handlers: NS.DeleteHandler<O, CtxOptions>[] = [
      ...this.globalDeleteHandlers,
    ];

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

  async update(values: O, changes: Partial<I>, ctxOptions: CtxOptions) {
    const ctxOpts = this._updateCtxOptions(ctxOptions);
    const errorTool = new ErrorTool<ErrorMetadata>();
    const ctx = this._getReadonlyCtx({
      data: this.ctxValues,
      isUpdate: false,
      rawInput: this.ctxRawInput,
    });

    if (!areValuesOk(values) || !areValuesOk(changes))
      return this._handleError(ctx, ctxOpts, errorTool);

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true });

    if (this._options?.setMissingDefaultsOnUpdate)
      await this._setMissingDefaults();

    const _changes = this._cleanInput(changes);

    if (!(await this._isGloballyUpdatable(_changes)))
      return this._handleError(ctx, ctxOpts, errorTool);

    const { error, updates: dt } =
      await this._handleUpdatePrimaryValidations(_changes);

    if (error.hasErrors) return this._handleError(ctx, ctxOpts, error);

    let updates = dt;

    const requiredErrorTool = await this._handleRequiredBy(ctx);
    if (requiredErrorTool.hasErrors)
      return this._handleError(ctx, ctxOpts, requiredErrorTool);

    const error2 = await this._handleSecondaryValidations(updates, true);
    if (error2.hasErrors) return this._handleError(ctx, ctxOpts, error2);

    const postValidationError = await this._handlePostValidations(
      updates,
      true,
    );
    if (postValidationError.hasErrors)
      return this._handleError(ctx, ctxOpts, postValidationError);

    await this._handleSanitizationOfVirtuals(updates, true);

    updates = await this._resolveDependentChanges(
      updates,
      this._getPartialContext(),
      true,
    );

    for (const prop of getKeysAsProps(updates))
      if (
        isEqual(
          updates[prop],
          this.ctxValues[prop],
          this._options.equalityDepth,
        )
      )
        delete updates[prop];

    if (!Object.keys(updates).length) {
      return this._handleError(ctx, ctxOpts, errorTool);
    }

    if (this._options?.setMissingDefaultsOnUpdate)
      this._regeneratedProps.forEach((prop) => {
        if (isEqual(updates[prop], undefined))
          // @ts-expect-error ikr
          updates[prop] = this.ctxInput[prop] as never;
      });

    const finalData = this._useConfigProps(updates, true);

    this._updateContext(finalData as never);

    return {
      data: finalData as Partial<O>,
      error: null,
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(finalData, true),
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

  set relevantFieldsProvided(value: Set<string>) {
    const configNames = new Set<string>();
    const outputFieldsChanged = new Set<string>();

    for (const field_name of value) {
      const info = this.get(field_name);

      configNames.add(info.configName);

      if (info.isOutput) {
        outputFieldsChanged.add(field_name);
      }
    }

    this._relevantConfigNames = configNames;
    this._relevantFieldsProvided = outputFieldsChanged;
  }
}

type FieldInfo = {
  name: string;
  configName: string;
  isInput: boolean;
  isOutput: boolean;
};
