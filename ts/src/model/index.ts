import {
  cloneWithMethods,
  deepCloneValue,
  ErrorTool,
  getDefaultRequiredError,
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

import {
  allOrThrow,
  isThenable,
  type MaybePromise,
  mapMaybe,
  settleAll,
  settleEach,
} from './settle';

export { Model, ModelTool };

type DefinitionEntry = [string, NS.Definitions<any, any, any, any>[string]];

interface SchemaPlan {
  constants: DefinitionEntry[];
  /** dependent fields and lax fields: the ones that get default values */
  defaultables: DefinitionEntry[];
  /** every field that can be required: all but constants and dependents */
  requirables: DefinitionEntry[];
  /** dependent fields with the fields they depend on */
  dependents: [string, string[]][];
  /** the fields' own `onDelete` handlers */
  deleteHandlers: NS.DeleteHandler<any, any>[];
  /** whether any field is read under an alias */
  hasAliases: boolean;
  /** whether any field has an `onSuccess` handler */
  hasSuccessHandlers: boolean;
}

/** the `handleSuccess`/`handleFailure` of operations with no handlers to run */
const noopHandle = async () => {};

const NotAllowedError = 'value not allowed';
const validationFailedFieldError = makeFieldError('validation failed');

class ModelTool<
  I extends RealType<I>,
  O extends RealType<O>,
  CtxOptions extends ObjectType = {},
  ErrorMetadata = DefaultFieldErrorMetadata,
  ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
> {
  private static _fieldInfoMapCache = new WeakMap<
    NS.Definitions<any, any, any, any>,
    Map<string, InputFieldInfo>
  >();

  private static _planCache = new WeakMap<
    NS.Definitions<any, any, any, any>,
    SchemaPlan
  >();

  /**
   * The definitions grouped the way each phase reads them, built once per
   * schema instead of scanning every definition on every call. Each list
   * keeps definition order, which is the order handlers run in.
   */
  private get _plan(): SchemaPlan {
    let plan = ModelTool._planCache.get(this.definitions);

    if (!plan) {
      plan = {
        constants: [],
        defaultables: [],
        requirables: [],
        dependents: [],
        deleteHandlers: [],
        hasAliases: false,
        hasSuccessHandlers: false,
      };

      for (const entry of Object.entries(this.definitions)) {
        const config = entry[1];

        if (config.type === 'constant') plan.constants.push(entry);
        else if (config.type !== 'dependent') plan.requirables.push(entry);

        if (config.type === 'dependent') {
          plan.defaultables.push(entry);
          plan.dependents.push([entry[0], toArray(config.dependsOn)]);
        } else if (config.type === 'lax') plan.defaultables.push(entry);

        if (config.type !== 'virtual' && config.onDelete)
          plan.deleteHandlers = plan.deleteHandlers.concat(config.onDelete);

        if ((config as { alias?: string }).alias) plan.hasAliases = true;

        if (config.onSuccess) plan.hasSuccessHandlers = true;
      }

      ModelTool._planCache.set(this.definitions, plan);
    }

    return plan;
  }

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
    let fieldsCollection = this._filterInputFieldsAllowed({
      rawInput: input,
      previousValues: null,
      options,
    });
    if (isThenable(fieldsCollection)) fieldsCollection = await fieldsCollection;

    // each phase returns a plain value when all its handlers were sync, and
    // is only awaited when it returned a promise (awaiting a plain value
    // still costs a microtask hop)
    const defaults = this._attachDefaultValues(fieldsCollection);
    if (defaults) await defaults;

    let requiredError = this._evaluateMissingRequiredFields(fieldsCollection);
    if (isThenable(requiredError)) requiredError = await requiredError;

    if (requiredError.hasErrors)
      return this._handleError(requiredError, fieldsCollection);

    let primaryError = this._validate(fieldsCollection);
    if (isThenable(primaryError)) primaryError = await primaryError;

    if (primaryError.hasErrors)
      return this._handleError(primaryError, fieldsCollection);

    let secondaryError = this._reValidate(fieldsCollection);
    if (isThenable(secondaryError)) secondaryError = await secondaryError;
    if (secondaryError.hasErrors)
      return this._handleError(secondaryError, fieldsCollection);

    let postValidationError = this._postValidate(fieldsCollection);
    if (isThenable(postValidationError))
      postValidationError = await postValidationError;

    if (postValidationError.hasErrors)
      return this._handleError(postValidationError, fieldsCollection);

    const sanitized = this._sanitizeVirtuals(fieldsCollection);
    if (sanitized) await sanitized;

    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0) {
      const next = this._resolveDependentChanges(collection);
      collection = isThenable(next) ? await next : next;
    }

    this._attachTimestamps();

    const constants = this._attachConstantValues();
    const data = isThenable<O>(constants) ? await constants : constants;

    return {
      data,
      error: null,
      options: cloneWithMethods(this.ctxOptions),
      handleFailure: null,
      handleSuccess: this._makeHandleSuccess(fieldsCollection),
    };
  }

  async delete(data: O, options: CtxOptions) {
    const handlers: NS.DeleteHandler<O, CtxOptions>[] = this.options.onDelete
      ? this.options.onDelete.concat(this._plan.deleteHandlers)
      : this._plan.deleteHandlers;

    if (!handlers.length) return;

    // handlers get frozen copies: the caller's own objects stay untouched
    const frozenData = Object.freeze({ ...data });
    const frozenOptions = Object.freeze(cloneWithMethods(options));

    await Promise.allSettled(
      handlers.map(
        async (h) => await Promise.try(h, frozenData, frozenOptions),
      ),
    );
  }

  async update(values: O, changes: Partial<I>, options: CtxOptions) {
    const emptyErrorTool = new ErrorTool<ErrorMetadata>();

    let fieldsCollection = this._filterInputFieldsAllowed({
      rawInput: changes,
      previousValues: values,
      options,
    });
    if (isThenable(fieldsCollection)) fieldsCollection = await fieldsCollection;

    if (!fieldsCollection.relevantConfigNames.size)
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    let requiredError = this._evaluateMissingRequiredFields(fieldsCollection);
    if (isThenable(requiredError)) requiredError = await requiredError;

    if (requiredError.hasErrors)
      return this._handleUpdateError(requiredError, fieldsCollection);

    let primaryError = this._validate(fieldsCollection);
    if (isThenable(primaryError)) primaryError = await primaryError;

    if (primaryError.hasErrors)
      return this._handleUpdateError(primaryError, fieldsCollection);

    if (!this._isValidUpdate(fieldsCollection))
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    let secondaryError = this._reValidate(fieldsCollection);
    if (isThenable(secondaryError)) secondaryError = await secondaryError;

    if (secondaryError.hasErrors)
      return this._handleUpdateError(secondaryError, fieldsCollection);

    let postValidationError = this._postValidate(fieldsCollection);
    if (isThenable(postValidationError))
      postValidationError = await postValidationError;

    if (postValidationError.hasErrors)
      return this._handleUpdateError(postValidationError, fieldsCollection);

    if (!this._isValidUpdate(fieldsCollection))
      return this._handleUpdateError(emptyErrorTool, fieldsCollection);

    const sanitized = this._sanitizeVirtuals(fieldsCollection);
    if (sanitized) await sanitized;

    let collection = fieldsCollection.clonedFromRelevantFieldsProvided();

    while (collection.relevantDependentConfigNames.size > 0) {
      const next = this._resolveDependentChanges(collection);
      collection = isThenable(next) ? await next : next;
    }

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
    let fields = ModelTool._fieldInfoMapCache.get(this.definitions);

    if (!fields) {
      fields = new Map();

      for (const [configName, config] of Object.entries(this.definitions)) {
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

      ModelTool._fieldInfoMapCache.set(this.definitions, fields);
    }

    return new FieldInfoCollection(fields, this._plan.hasAliases);
  }

  // A ctx's getters capture the state below when the ctx is built (in-place
  // changes to those objects show through, replacements don't). So a built
  // ctx can be reused for as long as every captured reference is unchanged:
  // it then behaves exactly like a freshly built one.
  private _ctxCacheKey: readonly unknown[] = [];
  private _readonlyCtxCache: ReadonlyIvoContext<I, O, CtxOptions> | null = null;
  private _ctxCache: IvoContext<I, O, CtxOptions> | null = null;

  private _syncCtxCache() {
    const key = this._ctxCacheKey;

    if (
      key[0] === this._isUpdate &&
      key[1] === this._ctxInput &&
      key[2] === this._ctxRawInput &&
      key[3] === this._ctxValues &&
      key[4] === this._ctxPreviousValues &&
      key[5] === this._ctxOptions
    )
      return;

    this._ctxCacheKey = [
      this._isUpdate,
      this._ctxInput,
      this._ctxRawInput,
      this._ctxValues,
      this._ctxPreviousValues,
      this._ctxOptions,
    ];
    this._readonlyCtxCache = null;
    this._ctxCache = null;
  }

  private _getReadonlyCtx(): ReadonlyIvoContext<I, O, CtxOptions> {
    this._syncCtxCache();

    this._readonlyCtxCache ??= this._buildReadonlyCtx();

    return this._readonlyCtxCache;
  }

  private _getContext(): IvoContext<I, O, CtxOptions> {
    this._syncCtxCache();

    this._ctxCache ??= this._buildContext();

    return this._ctxCache;
  }

  private _buildReadonlyCtx(): ReadonlyIvoContext<I, O, CtxOptions> {
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

  // the readonly ctx plus `updateOptions`, built directly rather than by
  // copying the readonly ctx's property descriptors
  private _buildContext(): IvoContext<I, O, CtxOptions> {
    const values = this.ctxValues;
    const isUpdate = this.isUpdate,
      changes = isUpdate ? values : null,
      previousValues = isUpdate ? this.ctxPreviousValues : null,
      input = this.ctxInput,
      rawInput = this.ctxRawInput,
      options = this.ctxOptions,
      updateOptions = this._updateCtxOptions;

    return (
      isUpdate
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
            get updateOptions() {
              return updateOptions;
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
            get updateOptions() {
              return updateOptions;
            },
          })
    ) as IvoContext<I, O, CtxOptions>;
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

  private _setCxtInput(updates: Partial<I>) {
    this._ctxInput = updates;
  }

  private _setCxtValues(updates: Partial<O>) {
    this._ctxValues = updates;
  }

  private _updateCxtInput(updates: Partial<I>) {
    Object.assign(this.ctxInput, updates);
  }

  private _updateCxtValues(updates: Partial<O>) {
    Object.assign(this.ctxValues, updates);
  }

  private _boundUpdateCtxOptions:
    | ((options: Partial<CtxOptions>) => CtxOptions)
    | null = null;

  // handed out detached as `ctx.updateOptions`, so it's bound -- lazily, and
  // once per call, so every ctx of a call exposes the same function
  private get _updateCtxOptions() {
    if (!this._boundUpdateCtxOptions)
      this._boundUpdateCtxOptions = (options: Partial<CtxOptions>) =>
        Object.assign(this._ctxOptions, options);

    return this._boundUpdateCtxOptions;
  }

  private _cleanInput(
    rawInput: Partial<I>,
    fieldsCollection: FieldInfoCollection,
  ) {
    const input: Partial<I> = {};
    const output: Partial<O> = {};

    for (const name in rawInput) {
      if (!Object.hasOwn(rawInput, name)) continue;

      const info = fieldsCollection.getUnsafe(name);

      if (!info || info.name !== name) continue;

      const value = rawInput[name];

      input[name] = value;

      if (!info.isVirtual) (output as Record<string, unknown>)[name] = value;
    }

    return { input, output };
  }

  /**
   * `_cleanInput` for updates, in one pass: fields that are ignored on update
   * or unchanged are never written to `input`/`output` (rather than written,
   * then deleted). `ignoreUpdate` resolvers are queued in `tasks`; they only
   * run once filtering is done.
   */
  private _cleanUpdateInput(
    rawInput: Partial<I>,
    previousValues: O,
    fieldsCollection: FieldInfoCollection,
    fieldsProvided: Set<string>,
    relevantFieldsProvided: Set<string>,
    tasks: [string[] | readonly string[], () => boolean | Promise<boolean>][],
  ) {
    const input: Partial<I> = {};
    const output: Partial<O> = {};

    for (const fieldName in rawInput) {
      if (!Object.hasOwn(rawInput, fieldName)) continue;

      const fieldInfo = fieldsCollection.getUnsafe(fieldName);

      if (!fieldInfo) continue;

      fieldsProvided.add(fieldName);

      const value = rawInput[fieldName];
      const { ignoreUpdate } = this.definitions[
        fieldInfo.configName
      ] as NS.RequiredField<any, I, O, CtxOptions, any>;

      if (typeof ignoreUpdate === 'function') {
        tasks.push([
          [fieldName],
          () => ignoreUpdate(this._getUpdateResolverCtx()),
        ]);

        relevantFieldsProvided.add(fieldName);
      } else if (
        ignoreUpdate ||
        (!fieldInfo.isVirtual &&
          isEqual(
            value,
            (previousValues as Record<string, unknown>)[fieldName],
            this.options.equalityDepth,
          ))
      )
        continue;
      else relevantFieldsProvided.add(fieldName);

      // aliased fields are only read under their alias
      if (fieldInfo.name !== fieldName) continue;

      input[fieldName] = value;

      if (!fieldInfo.isVirtual)
        (output as Record<string, unknown>)[fieldName] = value;
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

  private _attachConstantValues(): MaybePromise<O> {
    const values = this._ctxValues as Record<string, unknown>;
    const tasks: [string, NS.ConstantResolver<any, I, O, CtxOptions>][] = [];

    for (const [configName, config] of this._plan.constants) {
      if (config.type !== 'constant') continue;

      const value = config.value;

      if (isFunctionLike(value)) {
        tasks.push([configName, value]);
        continue;
      }

      values[configName] = value;
    }

    if (!tasks.length) return this.ctxValues as O;

    return mapMaybe(
      settleAll(
        tasks.map(
          ([, resolver]) =>
            () =>
              resolver(this._getConstantCtx()),
        ),
      ),
      (outcomes) => {
        // a failed resolver leaves its constant `null`
        outcomes.forEach((outcome, i) => {
          values[tasks[i]![0]] = outcome.ok ? outcome.value : null;
        });

        return this.ctxValues as O;
      },
    );
  }

  private _attachDefaultValues(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<void> {
    const values = this._ctxValues as Record<string, unknown>;
    const tasks: [string, NS.DefaultValueResolver<any, I, CtxOptions>][] = [];

    for (const [configName, config] of this._plan.defaultables) {
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

        values[configName] = value;
      }
    }

    if (!tasks.length) return;

    return mapMaybe(
      settleAll(
        tasks.map(
          ([, resolver]) =>
            () =>
              resolver(this._getInitResolverCtx()),
        ),
      ),
      (outcomes) => {
        // a failed resolver leaves its default `null`
        outcomes.forEach((outcome, i) => {
          values[tasks[i]![0]] = outcome.ok ? outcome.value : null;
        });
      },
    );
  }

  private _attachTimestamps(): Partial<O> {
    const values = this.ctxValues;
    const isUpdate = this.isUpdate;

    if (!this.options.timestamps) return values;

    const { createdAt, updatedAt } = this.options.timestamps.keys;
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

  private _filterInputFieldsAllowed({
    options,
    previousValues: _previousValues,
    rawInput,
  }: {
    rawInput: Partial<I>;
    previousValues: O | null;
    options: CtxOptions;
  }): MaybePromise<FieldInfoCollection> {
    const fieldsCollection = this._getFieldInfoCollection();
    const isUpdate = !!_previousValues;

    const tasks: [
      string[] | readonly string[],
      () => boolean | Promise<boolean>,
    ][] = [];

    const fieldsProvided = new Set<string>();
    const relevantFieldsProvided = new Set<string>();

    const { input, output } = isUpdate
      ? this._cleanUpdateInput(
          rawInput,
          _previousValues!,
          fieldsCollection,
          fieldsProvided,
          relevantFieldsProvided,
          tasks,
        )
      : this._cleanInput(rawInput, fieldsCollection);

    this._isUpdate = isUpdate;
    this._ctxInput = input;
    this._ctxValues = output;
    this._ctxRawInput = deepCloneValue(rawInput);
    this._ctxPreviousValues = deepCloneValue(_previousValues);
    this._ctxOptions = cloneWithMethods(options);

    const previousValues: Partial<O> = _previousValues ?? {};

    if (!isUpdate) {
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

      // a required field's `ignoreUpdate` resolver was already queued while
      // reading the input (above)
      if (isUpdate && config.type === 'required') continue;

      if (!source) continue;

      relevantFieldsProvided.delete(fieldName);

      // @ts-expect-error ikr
      delete input[fieldName];

      // @ts-expect-error ikr
      if (!fieldInfo.isVirtual) delete output[fieldName];
    }

    let relevantConfigNamesCache: Set<string> | undefined;
    const relevantConfigNames = () => {
      if (!relevantConfigNamesCache) {
        relevantConfigNamesCache = new Set();

        for (const name of relevantFieldsProvided)
          relevantConfigNamesCache.add(fieldsCollection.get(name).configName);
      }

      return relevantConfigNamesCache;
    };

    if (this.options.ignore)
      for (const config of this.options.ignore) {
        if (typeof config === 'function')
          tasks.push([[], () => config(this._getContext())]);
        else if (config && typeof config === 'object') {
          const fields = config.fields;

          if (fields.some((name) => relevantConfigNames().has(name)))
            tasks.push([fields, () => config.handler(this._getContext())]);
        }
      }

    if (isUpdate && this.options.ignoreUpdate)
      for (const config of this.options.ignoreUpdate) {
        if (typeof config === 'function') {
          tasks.push([[], () => config(this._getUpdateResolverCtx())]);

          continue;
        }

        const fields = config.fields;

        if (fields.some((name: string) => relevantConfigNames().has(name)))
          tasks.push([
            fields,
            () => config.handler(this._getUpdateResolverCtx()),
          ]);
      }

    this._setCxtInput(input);
    this._setCxtValues(output);

    if (!tasks.length) {
      fieldsCollection.relevantFieldsProvided = relevantFieldsProvided;

      return fieldsCollection;
    }

    // every resolver runs; the first one to throw fails the whole call
    return mapMaybe(
      allOrThrow(tasks.map(([, resolver]) => resolver)),
      (verdicts) => {
        for (const [i, ignore] of verdicts.entries()) {
          const configNames = tasks[i]![0];

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
      },
    );
  }

  private _evaluateMissingRequiredFields(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<ErrorTool<ErrorMetadata>> {
    const errorTool = new ErrorTool<ErrorMetadata>();

    type Res = [string, RequiredHandlerRes<ErrorMetadata>][];

    const handlers: (() => Res | Promise<Res>)[] = [];

    for (const [configName, config] of this._plan.requirables) {
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

      // @ts-expect-error ikr
      const { alias, required } = config;

      if (required)
        handlers.push(() =>
          mapMaybe(
            required(this._getContext()),
            (res): Res => [[alias ?? configName, res]],
          ),
        );
    }

    if (this.options.required)
      for (const { fields, handler } of this.options.required) {
        if (
          fields.some((configName) =>
            fieldsCollection.relevantConfigNames.has(configName),
          )
        )
          continue;

        handlers.push(() =>
          mapMaybe(handler(this._getContext()), (result): Res => {
            if (!result) return [];

            const errors: Res = [];

            for (const [fieldName, value] of Object.entries(result)) {
              const fieldInfo = fieldsCollection.getUnsafe(fieldName);

              if (fieldInfo && fields.includes(fieldInfo.configName))
                // @ts-expect-error
                errors.push([fieldInfo.name, value]);
            }

            return errors;
          }),
        );
      }

    if (!handlers.length) return errorTool;

    return mapMaybe(settleAll(handlers), (results) => {
      for (const r of results) {
        if (!r.ok || !r.value) continue;

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
                  getDefaultRequiredError(fieldName),
                ),
              );
          }
        }
      }

      return errorTool;
    });
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

  private _validate(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<ErrorTool<ErrorMetadata>> {
    const ctx = this._getContext();
    const errorTool = new ErrorTool<ErrorMetadata>();

    // each field's outcome is applied as soon as its validator settles
    const settled = settleEach(
      fieldsCollection.relevantFieldsProvided,
      (fieldName): unknown => {
        const configName = fieldsCollection.get(fieldName).configName;
        const { allow, validator } = this.definitions[
          configName
        ] as NS.LaxField<any, I, O, CtxOptions, ErrorMetadata>;
        const rawValue = (ctx.input as any)[fieldName];

        if (allow) return this._validateAllowedValues(configName, rawValue);

        return validator ? validator(rawValue, ctx) : undefined;
      },
      (fieldName, outcome) => {
        const fieldInfo = fieldsCollection.get(fieldName);
        const { allow, validator } = this.definitions[
          fieldInfo.configName
        ] as NS.LaxField<any, I, O, CtxOptions, ErrorMetadata>;

        if (!allow && !validator) return;

        const rawValue = (ctx.input as any)[fieldName];
        const result = outcome.ok
          ? outcome.value
          : { valid: false, reason: 'validation failed' };

        const isValid = this._sanitizeValidationResponse<any>(
          (typeof result === 'boolean'
            ? { valid: result }
            : result) as ValidatorResponseObject<any, ErrorMetadata>,
          rawValue,
        );

        if (!isValid.valid)
          return this._handleInvalidValue(errorTool, fieldName, isValid);

        let { validated } = isValid;

        if (validated === undefined) validated = rawValue;

        if (!fieldInfo.isVirtual)
          (this.ctxValues as Record<string, unknown>)[fieldName] = validated;

        (this.ctxInput as Record<string, unknown>)[fieldName] = validated;
      },
    );

    return settled ? settled.then(() => errorTool) : errorTool;
  }

  private _reValidate(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<ErrorTool<ErrorMetadata>> {
    const errorTool = new ErrorTool<ErrorMetadata>();
    const fields: [
      string,
      InputFieldInfo,
      NS.LaxField<any, any, any, any, any>['reValidator'],
    ][] = [];

    for (const fieldName of fieldsCollection.relevantFieldsProvided) {
      const fieldInfo = fieldsCollection.get(fieldName);
      const { reValidator } = this.definitions[
        fieldInfo.configName
      ] as NS.LaxField<any, any, any, any, any>;

      if (reValidator) fields.push([fieldName, fieldInfo, reValidator]);
    }

    if (!fields.length) return errorTool;

    const ctx = this._getContext();

    // each field's outcome is applied as soon as its re-validator settles
    const settled = settleEach(
      fields,
      ([fieldName, , reValidator]) =>
        reValidator!((ctx.input as any)?.[fieldName], ctx),
      ([fieldName, fieldInfo], outcome) => {
        const value = (ctx.input as any)?.[fieldName];

        const isValid = outcome.ok
          ? this._sanitizeValidationResponse<unknown>(
              outcome.value as ValidatorResponseObject<unknown, ErrorMetadata>,
              value,
            )
          : makeResponse<unknown, ErrorMetadata>({
              valid: false,
              reason: 'validation failed',
            });

        if (!isValid.valid)
          return this._handleInvalidValue(errorTool, fieldName, isValid);

        const { validated } = isValid;

        if (
          validated !== undefined &&
          !isEqual(validated, value, this.options.equalityDepth)
        ) {
          if (!fieldInfo.isVirtual)
            (this.ctxValues as Record<string, unknown>)[fieldName] = validated;

          (this.ctxInput as Record<string, unknown>)[fieldName] = validated;
        }
      },
    );

    return settled ? settled.then(() => errorTool) : errorTool;
  }

  private _postValidate(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<ErrorTool<ErrorMetadata>> {
    const errorTool = new ErrorTool<ErrorMetadata>();

    if (
      !this.options.postValidate?.some(({ fields }) =>
        fields.some((name) => fieldsCollection.relevantConfigNames.has(name)),
      )
    )
      return errorTool;

    return this._runPostValidators(fieldsCollection, errorTool);
  }

  private async _runPostValidators(
    fieldsCollection: FieldInfoCollection,
    errorTool: ErrorTool<ErrorMetadata>,
  ) {
    const ctx = this._getContext();

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

      for (const fieldName of Object.keys(revalidatedData)) {
        const fieldInfo = fieldsCollection.getUnsafe(fieldName);

        // @ts-expect-error ikr
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

  private _sanitizeVirtuals(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<void> {
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

    if (!sanitizers.length) return;

    // a failed sanitizer leaves its value as it is
    return settleEach(
      sanitizers,
      ([, sanitizer]) => sanitizer(this._getContext()),
      ([fieldName], outcome) => {
        if (!outcome.ok) return;

        const input = this._ctxInput as Record<string, unknown>;

        if (
          !isEqual(outcome.value, input[fieldName], this.options.equalityDepth)
        )
          input[fieldName] = outcome.value;
      },
    );
  }

  private _makeHandleFailure(fieldsCollection: FieldInfoCollection) {
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

    if (!cleanups.length) return noopHandle;

    // the ctx as it is when the operation fails
    const ctx = this._getReadonlyCtx();

    return async () => {
      await Promise.allSettled(cleanups.map((h) => Promise.try(h, ctx)));
    };
  }

  private _makeHandleSuccess(fieldsCollection: FieldInfoCollection) {
    if (!this._plan.hasSuccessHandlers && !this.options.onSuccess?.length)
      return noopHandle;

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
        if (typeof config === 'function') {
          successListeners = successListeners.concat(config);
          continue;
        }

        if (config.fields.some((configName) => relevantFields.has(configName)))
          successListeners = successListeners.concat(config.handler);
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

  private _resolveDependentChanges(
    fieldsCollection: FieldInfoCollection,
  ): MaybePromise<FieldInfoCollection> {
    const ctx = this._getContext();

    const isUpdate = ctx.isUpdate;
    const toResolve = new Set<string>();

    for (const [configName, parents] of this._plan.dependents)
      if (
        parents.some((parent) =>
          fieldsCollection.relevantDependentConfigNames.has(parent),
        )
      )
        toResolve.add(configName);

    const fieldsResolved = new Set<string>();
    const values = this.ctxValues as Record<string, unknown>;
    const prevValues = (this.ctxPreviousValues ?? {}) as Record<
      string,
      unknown
    >;

    const resolvable: [string, NS.DependentField<any, I, O, CtxOptions>][] = [];

    for (const name of toResolve) {
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
        !isEqual(config.default, prevValues[name], this.options.equalityDepth)
      )
        continue;

      resolvable.push([name, config]);
    }

    // each value is applied as soon as its resolver settles
    const settled = settleEach(
      resolvable,
      ([, config]) => config.resolver(ctx),
      ([name], outcome) => {
        // a failed resolver keeps the previous value (or `null` on create)
        const value = outcome.ok
          ? outcome.value
          : isUpdate
            ? prevValues[name]
            : null;

        if (!isEqual(value, values[name], this.options.equalityDepth)) {
          fieldsResolved.add(name);
          values[name] = value;
        }
      },
    );

    const next = () =>
      fieldsCollection.newWithResolvedDependentFields(fieldsResolved);

    return settled ? settled.then(next) : next();
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
    // what `ctx.options` returns, without building a ctx just to read it
    const options = cloneWithMethods(this.ctxOptions);
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

  constructor(
    private _fields: Map<string, InputFieldInfo>,
    // without aliases, every field's config name is its own name
    private _hasAliases = true,
  ) {}

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
    let configNames: Set<string>;

    if (this._hasAliases) {
      configNames = new Set();

      for (const field_name of names)
        configNames.add(this.get(field_name).configName);
    } else configNames = new Set(names);

    this._relevantConfigNames = configNames;
    this._relevantFieldsProvided = names;
  }

  appendRelevantFieldProvided(name: string) {
    this._relevantConfigNames.add(this.get(name).configName);
    this._relevantFieldsProvided.add(name);
  }

  clonedFromRelevantFieldsProvided() {
    const col = new FieldInfoCollection(this._fields, this._hasAliases);

    col._relevantDependentConfigNames = new Set(this._relevantConfigNames);

    return col;
  }

  newWithResolvedDependentFields(names: Set<string>) {
    const col = new FieldInfoCollection(this._fields, this._hasAliases);

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
