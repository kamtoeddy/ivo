//#region src/utils/types.d.ts
type ObjectType<T = Record<string, unknown>> = T extends object ? T extends unknown[] ? never : T & {} : never;
type DefaultFieldErrorMetadata = Record<string, unknown>;
type FieldError<Metadata = DefaultFieldErrorMetadata> = {
  reason: string;
  metadata: Metadata | null;
};
type InputFieldError<Metadata> = FieldError<Metadata> | {
  reason: FieldError['reason'];
} | {
  metadata: FieldError<Metadata>['metadata'];
};
type InputPayload = Record<string, string | FieldError>;
type ReadonlyIvoContext<Input, Output = Input, CtxOptions extends ObjectType = {}> = (Readonly<WithReadonlyCtxOptions<{
  readonly changes: null;
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly isUpdate: false;
  readonly previousValues: null;
  readonly values: Partial<RealType<Output>>;
}, CtxOptions>> | Readonly<WithReadonlyCtxOptions<{
  readonly changes: Partial<RealType<Output>>;
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly isUpdate: true;
  readonly previousValues: RealType<Output>;
  readonly values: RealType<Output>;
}, CtxOptions>>) & {};
type IvoContext<Input, Output = Input, CtxOptions extends ObjectType = {}> = (Readonly<WithCtxOptions<{
  readonly changes: null;
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly isUpdate: false;
  readonly previousValues: null;
  readonly values: Partial<RealType<Output>>;
}, CtxOptions>> | Readonly<WithCtxOptions<{
  readonly changes: Partial<RealType<Output>>;
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly isUpdate: true;
  readonly previousValues: RealType<Output>;
  readonly values: RealType<Output>;
}, CtxOptions>>) & {};
type IvoSuccessContext<Input, Output = Input, CtxOptions extends ObjectType = {}> = (Readonly<WithReadonlyCtxOptions<{
  readonly changes: null;
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly isUpdate: false;
  readonly previousValues: null;
  readonly values: RealType<Output>;
}, CtxOptions>> | Readonly<WithReadonlyCtxOptions<{
  readonly changes: Partial<RealType<Output>>;
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly isUpdate: true;
  readonly previousValues: RealType<Output>;
  readonly values: RealType<Output>;
}, CtxOptions>>) & {};
type ConstantResolverCtx<Input, Output = Input, CtxOptions extends ObjectType = {}> = Readonly<WithCtxOptions<{
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly values: Partial<Output>;
}, CtxOptions>> & {};
type InitResolverCtx<Input, CtxOptions extends ObjectType = {}> = Readonly<WithCtxOptions<{
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
}, CtxOptions>> & {};
type UpdateResolverCtx<Input, Output, CtxOptions extends ObjectType = {}> = Readonly<WithCtxOptions<{
  readonly input: Partial<RealType<Input>>;
  readonly rawInput: Partial<RealType<Input>>;
  readonly previousValues: RealType<Output>;
}, CtxOptions>> & {};
type WithReadonlyCtxOptions<T, CtxOptions extends ObjectType> = T & {
  readonly options: CtxOptions;
};
type WithCtxOptions<T, CtxOptions extends ObjectType> = WithReadonlyCtxOptions<T, CtxOptions> & {
  readonly updateOptions: (updates: Partial<CtxOptions>) => void;
} & {};
type TypeOf<T> = Exclude<T, undefined>;
type NotAllowedError<Metadata> = string | InputFieldError<Metadata>;
type RequiredHandlerRes<Metadata> = boolean | [boolean, string] | [boolean, InputFieldError<Metadata>] | readonly [boolean, string] | readonly [boolean, InputFieldError<Metadata>];
type RequiredHandler<Input, Output, CtxOptions extends ObjectType, Metadata> = (ctx: IvoContext<Input, Output, CtxOptions> & {}) => RequiredHandlerRes<Metadata> | Promise<RequiredHandlerRes<Metadata>>;
type RequiredOptionHandler<Input, Output, CtxOptions extends ObjectType, Metadata> = (ctx: IvoContext<Input, Output, CtxOptions> & {}) => ResponseErrorObject<Metadata, Input> | Promise<undefined | ResponseErrorObject<Metadata, Input>>;
type PostValidator<InputKeys extends KeyOf<Input>, Input, Output, CtxOptions extends ObjectType, Metadata> = (ctx: IvoContext<Input, Output, CtxOptions>) => undefined | true | void | ResponseErrorObject<Metadata, Input> | PostValidatorSanitizedResponse<InputKeys, Input, Output> | Promise<undefined | true | void | ResponseErrorObject<Metadata, Input> | PostValidatorSanitizedResponse<InputKeys, Input, Output>>;
type PostValidatorSanitizedResponse<K extends KeyOf<Input>, Input, Output> = { [Key in K]?: {
  validated: TypeOf<Key extends KeyOf<Output> ? Output[Key] : Input[Key]>;
}; };
type PostValidationConfig<K extends KeyOf<Input>, Input, Output, CtxOptions extends ObjectType, Metadata> = {
  fields: ArrayOfMinSizeTwo<K>;
  validator: PostValidator<K, Input, Output, CtxOptions, Metadata> | ArrayOfMinSizeOne<PostValidator<K, Input, Output, CtxOptions, Metadata> | ArrayOfMinSizeOne<PostValidator<K, Input, Output, CtxOptions, Metadata>>>;
};
type KeyOf<T> = Extract<keyof T, string>;
declare namespace NS {
  type DeleteHandler<Output, CtxOptions extends ObjectType> = (data: Readonly<Output>, options: Readonly<CtxOptions>) => unknown | Promise<unknown>;
  type FailureHandler<Input, Output, CtxOptions extends ObjectType = {}> = (ctx: ReadonlyIvoContext<Input, Output, CtxOptions>) => unknown | Promise<unknown>;
  type SuccessHandler<Input, Output, CtxOptions extends ObjectType = {}> = (ctx: IvoSuccessContext<Input, Output, CtxOptions>) => unknown | Promise<unknown>;
  type OnSuccessConfigObject<Input, Output, CtxOptions extends ObjectType> = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input & Output>>;
    handler: SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };
  type OnSuccessConfigOptionItem<Input, Output, CtxOptions extends ObjectType> = SuccessHandler<Input, Output, CtxOptions> | OnSuccessConfigObject<Input, Output, CtxOptions>;
  type OnSuccessConfigOption<Input, Output, CtxOptions extends ObjectType> = OnSuccessConfigOptionItem<Input, Output, CtxOptions> | ArrayOfMinSizeOne<OnSuccessConfigOptionItem<Input, Output, CtxOptions>>;
  type Resolver<T, Input, Output, CtxOptions extends ObjectType> = (ctx: IvoContext<Input, Output, CtxOptions>) => TypeOf<T> | Promise<TypeOf<T>>;
  type ConstantResolver<T, Input, Output, CtxOptions extends ObjectType> = (ctx: ConstantResolverCtx<Input, Output, CtxOptions>) => TypeOf<T> | Promise<TypeOf<T>>;
  type DefaultValueResolver<T, Input, CtxOptions extends ObjectType> = (ctx: InitResolverCtx<Input, CtxOptions>) => TypeOf<T> | Promise<TypeOf<T>>;
  type IgnoreUpdateHandler<Input, Output, CtxOptions extends ObjectType = {}> = (ctx: UpdateResolverCtx<Input, Output, CtxOptions>) => boolean | Promise<boolean>;
  type VirtualResolver<Value, Input, Output, CtxOptions extends ObjectType> = (ctx: IvoContext<Input, Output, CtxOptions>) => Value | Promise<Value>;
  type IgnoreConfigObject<Input, Output, CtxOptions extends ObjectType> = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    handler: Resolver<boolean, Input, Output, CtxOptions>;
  };
  type IgnoreConfigOptionItem<Input, Output, CtxOptions extends ObjectType> = Resolver<boolean, Input, Output, CtxOptions> | IgnoreConfigObject<Input, Output, CtxOptions>;
  type IgnoreConfigOption<Input, Output, CtxOptions extends ObjectType> = IgnoreConfigOptionItem<Input, Output, CtxOptions> | ArrayOfMinSizeOne<IgnoreConfigOptionItem<Input, Output, CtxOptions>>;
  type IgnoreUpdateConfigObject<Input, Output, CtxOptions extends ObjectType> = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    handler: IgnoreUpdateHandler<Input, Output, CtxOptions>;
  };
  type IgnoreUpdateConfigOptionItem<Input, Output, CtxOptions extends ObjectType> = IgnoreUpdateHandler<Input, Output, CtxOptions> | IgnoreUpdateConfigObject<Input, Output, CtxOptions>;
  type IgnoreUpdateConfigOption<Input, Output, CtxOptions extends ObjectType> = IgnoreUpdateConfigOptionItem<Input, Output, CtxOptions> | ArrayOfMinSizeOne<IgnoreUpdateConfigOptionItem<Input, Output, CtxOptions>>;
  type RequiredConfigObject<Input, Output, CtxOptions extends ObjectType, Metadata> = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    handler: RequiredOptionHandler<Input, Output, CtxOptions, Metadata>;
  };
  type RequiredConfigOption<Input, Output, CtxOptions extends ObjectType, Metadata> = RequiredConfigObject<Input, Output, CtxOptions, Metadata> | ArrayOfMinSizeOne<RequiredConfigObject<Input, Output, CtxOptions, Metadata>>;
  type FieldDefinition<_K extends keyof Input | keyof Output, Input, Output, CtxOptions extends ObjectType, Metadata> = Buildable<ConstantField<any, Input, Output, CtxOptions>> | Buildable<DependentField<any, Input, Output, CtxOptions>> | Buildable<LaxField<any, Input, Output, CtxOptions, Metadata>> | Buildable<RequiredField<any, Input, Output, CtxOptions, Metadata>> | Buildable<VirtualField<any, any, Input, Output, CtxOptions, Metadata>>;
  type Definitions<Input, Output, CtxOptions extends ObjectType, Metadata> = Record<string, ConstantField<any, Input, Output, CtxOptions> | DependentField<any, Input, Output, CtxOptions> | LaxField<any, Input, Output, CtxOptions, Metadata> | RequiredField<any, Input, Output, CtxOptions, Metadata> | VirtualField<any, any, Input, Output, CtxOptions, Metadata>>;
  type DefinitionsEntries<Input, Output, CtxOptions extends ObjectType, Metadata> = [string, Definitions<Input, Output, CtxOptions, Metadata>[string]][];
  type AliasToVirtualMap<T> = Record<string, KeyOf<T>>;
  type VirtualToAliasMap<T> = Record<KeyOf<T>, string>;
  type DependencyMap<T> = { [K in KeyOf<T>]?: KeyOf<T>[]; };
  type ConstantField<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType> = {
    name: string;
    type: 'constant';
    value: Value | ConstantResolver<Value, Input, Output, CtxOptions>;
    onDelete?: DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?: SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };
  type Dependables<K extends keyof Output, Input, Output> = Exclude<(KeyOf<Input> | KeyOf<Output>) | (string & {}), K>;
  type DependentField<K extends keyof Output, Input, Output, CtxOptions extends ObjectType> = {
    name: string;
    type: 'dependent';
    default: TypeOf<Output[K]> | Resolver<Output[K], Input, Output, CtxOptions>;
    dependsOn: ArrayOfMinSizeOne<Dependables<K, Input, Output>>;
    resolver: Resolver<Output[K], Input, Output, CtxOptions>;
    readonly?: true;
    onDelete?: DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?: SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };
  type LaxField<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, Metadata> = {
    name: string;
    type: 'lax';
    default: Value | Resolver<Value, Input, Output, CtxOptions>;
    allow?: ArrayOfMinSizeTwo<Value> | {
      values: ArrayOfMinSizeTwo<Value>;
      error?: NotAllowedError<Metadata> | ((value: unknown, allowedValues: ArrayOfMinSizeOne<Value>) => NotAllowedError<Metadata>);
    };
    readonly?: true;
    ignore?: Resolver<boolean, Input, Output, CtxOptions>;
    ignoreInit?: true;
    ignoreUpdate?: true;
    validator?: Validator<Value, Input, Output, CtxOptions, Metadata>;
    reValidator?: ReValidator<Value, Input, Output, CtxOptions, Metadata>;
    required?: RequiredHandler<Input, Output, CtxOptions, Metadata>;
    onDelete?: DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>> | undefined;
    onFailure?: FailureHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?: SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };
  type RequiredField<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, Metadata> = {
    name: string;
    type: 'required';
    allow?: ArrayOfMinSizeTwo<Value> | {
      values: ArrayOfMinSizeTwo<Value>;
      error?: NotAllowedError<Metadata> | ((value: unknown, allowedValues: ArrayOfMinSizeOne<Value>) => NotAllowedError<Metadata>);
    };
    requiredError?: string | ((ctx: InitResolverCtx<Input, CtxOptions>) => string);
    ignoreUpdate?: true | IgnoreUpdateHandler<Input, Output, CtxOptions>;
    validator?: Validator<Value, Input, Output, CtxOptions, Metadata>;
    reValidator?: ReValidator<Value, Input, Output, CtxOptions, Metadata>;
    onDelete?: DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>> | undefined;
    onFailure?: FailureHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?: SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };
  type VirtualField<Alias extends keyof Input | never, Value extends Input[keyof Input], Input, Output, CtxOptions extends ObjectType, Metadata> = {
    name: string;
    type: 'virtual';
    alias?: Alias;
    required?: RequiredHandler<Input, Output, CtxOptions, Metadata>;
    validator?: Validator<Value, Input, Output, CtxOptions, Metadata>;
    reValidator?: ReValidator<Value, Input, Output, CtxOptions, Metadata>;
    sanitizer?: VirtualResolver<Value, Input, Output, CtxOptions>;
    allow?: ArrayOfMinSizeTwo<Value> | {
      values: ArrayOfMinSizeTwo<Value>;
      error?: NotAllowedError<Metadata> | ((value: unknown, allowedValues: ArrayOfMinSizeOne<Value>) => NotAllowedError<Metadata>);
    };
    ignore?: Resolver<boolean, Input, Output, CtxOptions>;
    ignoreInit?: true;
    ignoreUpdate?: true;
    onFailure?: FailureHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?: SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };
  type InternalOptions<Input, Output, CtxOptions extends ObjectType, ErrorMetadata = DefaultFieldErrorMetadata, ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>> = {
    equalityDepth: number;
    ignore?: IgnoreConfigOptionItem<Input, Output, CtxOptions>[];
    ignoreUpdate?: IgnoreUpdateConfigOptionItem<Input, Output, CtxOptions>[];
    onDelete?: DeleteHandler<Output, CtxOptions>[];
    onSuccess?: (SuccessHandler<Input, Output, CtxOptions> | OnSuccessConfigObject<Input, Output, CtxOptions>)[];
    postValidate?: PostValidationConfig<string, Input, Output, CtxOptions, ErrorMetadata>[];
    required?: RequiredConfigObject<Input, Output, CtxOptions, ErrorMetadata>[];
    sanitizeError: (payload: IvoErrorPayload<ErrorMetadata, KeyOf<Input>>, ctxOptions: CtxOptions) => ErrorPayload;
    timestamps: TimeStampTool | null;
  };
  type Options<Input, Output, CtxOptions extends ObjectType = {}, ErrorMetadata = DefaultFieldErrorMetadata, ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>> = {
    equalityDepth?: number;
    onDelete?: DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?: OnSuccessConfigOption<Input, Output, CtxOptions>;
    postValidate?: PostValidationConfig<KeyOf<Input>, Input, Output, CtxOptions, ErrorMetadata> | ArrayOfMinSizeOne<PostValidationConfig<KeyOf<Input>, Input, Output, CtxOptions, ErrorMetadata>>;
    ignore?: IgnoreConfigOption<Input, Output, CtxOptions>;
    ignoreUpdate?: IgnoreUpdateConfigOption<Input, Output, CtxOptions>;
    required?: RequiredConfigOption<Input, Output, CtxOptions, ErrorMetadata>;
    sanitizeError?: (payload: IvoErrorPayload<ErrorMetadata, KeyOf<Input>>, ctxOptions: CtxOptions) => ErrorPayload;
    timestamps?: boolean | {
      createdAt?: boolean | string;
      updatedAt?: boolean | string | {
        key?: string;
        nullable?: boolean;
      };
    };
  };
  type OptionsKey<Input, Output> = KeyOf<Options<Input, Output>>;
  type PrivateOptions = {
    timestamps: Timestamp;
  };
  type Timestamp = {
    createdAt: string;
    updatedAt: string;
  };
  type ExtensionOptions<ParentInput, ParentOutput, Input, Output, CtxOptions extends ObjectType = {}, ErrorMetadata = DefaultFieldErrorMetadata, ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>> = Options<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload> & {
    remove?: (KeyOf<ParentInput> | KeyOf<ParentOutput>) | (KeyOf<ParentInput> | KeyOf<ParentOutput>)[];
    useParentOptions?: boolean;
  };
}
type ValidationResponse<T, Metadata = DefaultFieldErrorMetadata> = {
  valid: true;
  validated: T;
} | {
  metadata: FieldError<Metadata>['metadata'];
  reason: string;
  valid: false;
};
type InvalidValidatorResponse<Metadata> = {
  metadata?: FieldError<Metadata>['metadata'];
  reason?: string;
  valid: false;
  value?: unknown;
};
type ValidatorResponseObject<T, Metadata> = {
  valid: true;
  validated?: T;
} | InvalidValidatorResponse<Metadata>;
type ResponseErrorObject<Metadata, Input = object> = { [K in KeyOf<Input>]?: string | InputFieldError<Metadata>; };
type ValidatorResponse<T, Metadata> = boolean | (ValidatorResponseObject<T, Metadata> & {});
type Validator<T, Input, Output, CtxOptions extends ObjectType = {}, Metadata = DefaultFieldErrorMetadata> = (value: unknown, ctx: IvoContext<Input, Output, CtxOptions> & {}) => ValidatorResponse<T, Metadata> | Promise<ValidatorResponse<T, Metadata>>;
type ReValidator<T, Input, Output, CtxOptions extends ObjectType = {}, Metadata = DefaultFieldErrorMetadata> = (value: T, ctx: IvoContext<Input, Output, CtxOptions> & {}) => ValidatorResponse<T, Metadata> | Promise<ValidatorResponse<T, Metadata>>;
type ArrayOfMinSizeOne<T> = [T, ...T[]] | readonly [T, ...T[]];
type ArrayOfMinSizeTwo<T> = [T, T, ...T[]] | readonly [T, T, ...T[]];
type IvoErrorPayload<Metadata, Keys extends string> = { [K in Keys]?: FieldError<Metadata>; };
type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;
type RealType_<T> = T extends ((...args: never) => infer I) ? I : T;
type RealType<T> = { [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>; } & {};
declare const FIELD_CONFIG_BUILD_METHOD_NAME: unique symbol;
type Buildable<T> = {
  [FIELD_CONFIG_BUILD_METHOD_NAME]: () => T;
};
type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never; };
type XOR<T, U> = (T | U extends object ? (Without<T, U> & U) | ((Without<U, T> & T) & {}) : T | U) & {};
declare class TimeStampTool {
  private timestamps;
  private nullable;
  constructor(timestamps: NS.Options<any, any, any>['timestamps']);
  private _makeTimestamps;
  get keys(): NS.Timestamp;
  get isNullable(): boolean;
}
//#endregion
//#region src/model/index.d.ts
declare class ModelTool<I extends RealType<I>, O extends RealType<O>, CtxOptions extends ObjectType = {}, ErrorMetadata = DefaultFieldErrorMetadata, ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>> {
  private definitions;
  private options;
  private static _fieldInfoMapCache;
  constructor(definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>, options: NS.InternalOptions<I, O, CtxOptions, ErrorMetadata, ErrorPayload>);
  private _isUpdate;
  private _ctxInput;
  private _ctxRawInput;
  private _ctxValues;
  private _ctxPreviousValues;
  private _ctxOptions;
  private get isUpdate();
  private get ctxInput();
  private get ctxRawInput();
  private get ctxValues();
  private get ctxPreviousValues();
  private get ctxOptions();
  create(input: Partial<I>, options: CtxOptions): Promise<{
    data: O;
    error: null;
    options: CtxOptions;
    handleFailure: null;
    handleSuccess: () => Promise<void>;
  } | {
    data: null;
    error: ErrorPayload;
    options: CtxOptions;
    handleFailure: () => Promise<void>;
    handleSuccess: null;
  }>;
  delete(data: O, options: CtxOptions): Promise<void>;
  update(values: O, changes: Partial<I>, options: CtxOptions): Promise<{
    data: Partial<O>;
    error: null;
    options: CtxOptions;
    handleFailure: null;
    handleSuccess: () => Promise<void>;
  } | {
    data: null;
    error: {
      readonly isNothingToUpdate: true;
      readonly payload: null;
    } | {
      readonly isNothingToUpdate: false;
      readonly payload: ErrorPayload;
    };
    options: CtxOptions;
    handleFailure: () => Promise<void>;
    handleSuccess: null;
  }>;
  private _getFieldInfoCollection;
  private _getReadonlyCtx;
  private _getContext;
  private _getConstantCtx;
  private _getInitResolverCtx;
  private _getUpdateResolverCtx;
  private _setCxtInput;
  private _setCxtValues;
  private _updateCxtInput;
  private _updateCxtValues;
  private _updateCtxOptions;
  private _cleanInput;
  private _validateAllowedValues;
  private _handleInvalidValue;
  private _attachConstantValues;
  private _attachDefaultValues;
  private _attachTimestamps;
  private _filterInputFieldsAllowed;
  private _evaluateMissingRequiredFields;
  private _isValidUpdate;
  private _validate;
  private _reValidate;
  private _postValidate;
  private _handlePostValidator;
  private _sanitizeVirtuals;
  private _makeHandleFailure;
  private _makeHandleSuccess;
  private _resolveDependentChanges;
  private _sanitizeValidationResponse;
  private _handleError;
  private _handleUpdateError;
}
declare class Model<Input extends RealType<Input>, Output extends RealType<Output>, CtxOptions extends ObjectType = never, ErrorMetadata = DefaultFieldErrorMetadata, ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>> {
  private modelFactory;
  constructor(modelFactory: () => ModelTool<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload>);
  create: (values: Partial<Input>, options: CtxOptions) => Promise<{
    data: Output;
    error: null;
    options: CtxOptions;
    handleFailure: null;
    handleSuccess: () => Promise<void>;
  } | {
    data: null;
    error: ErrorPayload;
    options: CtxOptions;
    handleFailure: () => Promise<void>;
    handleSuccess: null;
  }>;
  delete: (values: Output, options: CtxOptions) => Promise<void>;
  update: (values: Output, changes: Partial<Input>, options: CtxOptions) => Promise<{
    data: Partial<Output>;
    error: null;
    options: CtxOptions;
    handleFailure: null;
    handleSuccess: () => Promise<void>;
  } | {
    data: null;
    error: {
      readonly isNothingToUpdate: true;
      readonly payload: null;
    } | {
      readonly isNothingToUpdate: false;
      readonly payload: ErrorPayload;
    };
    options: CtxOptions;
    handleFailure: () => Promise<void>;
    handleSuccess: null;
  }>;
}
//#endregion
//#region src/schema/fields/constants.d.ts
type BuildableConstantConfig<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, HasReadonly extends boolean = false, HasOnDelete extends boolean = false, HasOnSuccess extends boolean = false> = Buildable<NS.ConstantField<Value, Input, Output, CtxOptions>> & (HasOnDelete extends true ? {} : {
  onDelete(handler: NS.DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>): BuildableConstantConfig<Value, Input, Output, CtxOptions, HasReadonly, true, HasOnSuccess>;
}) & (HasOnSuccess extends true ? {} : {
  onSuccess(handler: NS.SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>): BuildableConstantConfig<Value, Input, Output, CtxOptions, HasReadonly, HasOnDelete, true>;
});
declare class ConstantBuilder<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType> implements BuildableConstantConfig<Value, Input, Output, CtxOptions> {
  private config;
  constructor(name: string, value: Value | NS.ConstantResolver<Value, Input, Output, CtxOptions>);
  onDelete(handler: NS.DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>): never;
  onSuccess(handler: NS.SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>): never;
  [FIELD_CONFIG_BUILD_METHOD_NAME](): NS.ConstantField<Value, Input, Output, CtxOptions>;
}
//#endregion
//#region src/schema/fields/dependents.d.ts
interface HasDependsOn<K extends keyof Output, Input, Output, CtxOptions extends ObjectType> {
  default<Default extends TypeOf<Output[K]> | NS.Resolver<Output[K], Input, Output, CtxOptions>, DefaultState extends 'value' | 'resolver' = Default extends Function ? 'resolver' : 'value'>(v: Default): HasDefault<K, Input, Output, CtxOptions, DefaultState>;
}
interface HasDefault<K extends keyof Output, Input, Output, CtxOptions extends ObjectType, DefaultState extends 'value' | 'resolver' = 'resolver'> {
  resolve(resolver: NS.Resolver<Output[K], Input, Output, CtxOptions>): BuildableDependentConfig<K, Input, Output, CtxOptions, DefaultState>;
}
type BuildableDependentConfig<K extends keyof Output, Input, Output, CtxOptions extends ObjectType, DefaultState extends 'value' | 'resolver' = 'resolver', HasReadonly extends boolean = false, HasOnDelete extends boolean = false, HasOnSuccess extends boolean = false> = Buildable<NS.DependentField<K, Input, Output, CtxOptions>> & (DefaultState extends 'resolver' ? {} : HasReadonly extends true ? {} : {
  readonly(): BuildableDependentConfig<K, Input, Output, CtxOptions, DefaultState, true, HasOnDelete, HasOnSuccess>;
}) & (HasOnDelete extends true ? {} : {
  onDelete(handler: NS.DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>): BuildableDependentConfig<K, Input, Output, CtxOptions, DefaultState, HasReadonly, true, HasOnSuccess>;
}) & (HasOnSuccess extends true ? {} : {
  onSuccess(handler: NS.SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>): BuildableDependentConfig<K, Input, Output, CtxOptions, DefaultState, HasReadonly, HasOnDelete, true>;
});
//#endregion
//#region src/schema/fields/lax.d.ts
type BuildableLaxConfig<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, Metadata, DefaultState extends 'value' | 'resolver' = 'resolver', ValidationState extends 'allow' | 'none' | 'validate' = 'none', HasAllowError extends boolean = false, HasReValidate extends boolean = false, HasRequired extends boolean = false, HasIgnore extends 'init' | 'update' | 'ignore' | 'none' = 'none', HasReadonly extends boolean = false, HasOnDelete extends boolean = false, HasOnFailure extends boolean = false, HasOnSuccess extends boolean = false> = Buildable<NS.LaxField<Value, Input, Output, CtxOptions, Metadata>> & (ValidationState extends 'none' ? {
  allow<const V extends Value>(values: ArrayOfMinSizeTwo<V>): BuildableLaxConfig<V, Input, Output, CtxOptions, Metadata, DefaultState, 'allow', HasAllowError, HasReValidate, HasRequired, HasIgnore, HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
  validate(validator: Validator<Value, Input, Output, CtxOptions, Metadata>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, 'validate', HasAllowError, HasReValidate, HasRequired, HasIgnore, HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
} : {}) & (ValidationState extends 'allow' ? HasAllowError extends true ? {} : {
  allowError(error: NotAllowedError<Metadata> | ((value: unknown, allowedValues: ArrayOfMinSizeOne<Value>) => NotAllowedError<Metadata>)): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, 'allow', true, HasReValidate, HasRequired, HasIgnore, HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
} : {}) & (ValidationState extends 'none' ? {} : HasReValidate extends true ? {} : {
  reValidate(validator: ReValidator<Value, Input, Output, CtxOptions, Metadata>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, true, HasRequired, HasIgnore, HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
}) & (HasRequired extends true ? {} : HasReadonly extends true ? {} : HasIgnore extends 'init' | 'update' ? {} : {
  required(handler: RequiredHandler<Input, Output, CtxOptions, Metadata>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, true, HasIgnore, HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
}) & (HasIgnore extends 'none' ? HasReadonly extends true ? {} : HasRequired extends true ? {
  ignore(resolver: NS.Resolver<boolean, Input, Output, CtxOptions>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, 'ignore', HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
} : {
  ignore(resolver: NS.Resolver<boolean, Input, Output, CtxOptions>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, 'ignore', HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
  ignoreInit(): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, 'init', HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
  ignoreUpdate(): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, 'update', HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
  readonly(): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, HasIgnore, true, HasOnDelete, HasOnFailure, HasOnSuccess>;
} : {}) & (DefaultState extends 'resolver' ? {} : HasReadonly extends true ? {
  ignore(resolver: NS.Resolver<boolean, Input, Output, CtxOptions>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, 'ignore', HasReadonly, HasOnDelete, HasOnFailure, HasOnSuccess>;
} : HasIgnore extends 'init' | 'update' ? {} : HasRequired extends true ? {} : {
  readonly(): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, HasIgnore, true, HasOnDelete, HasOnFailure, HasOnSuccess>;
}) & (HasOnDelete extends true ? {} : {
  onDelete(handler: NS.DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, HasIgnore, HasReadonly, true, HasOnFailure, HasOnSuccess>;
}) & (HasOnFailure extends true ? {} : {
  onFailure(handler: NS.FailureHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.FailureHandler<Input, Output, CtxOptions>>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, HasIgnore, HasReadonly, HasOnDelete, true, HasOnSuccess>;
}) & (HasOnSuccess extends true ? {} : {
  onSuccess(handler: NS.SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>): BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata, DefaultState, ValidationState, HasAllowError, HasReValidate, HasRequired, HasIgnore, HasReadonly, HasOnDelete, HasOnFailure, true>;
});
//#endregion
//#region src/schema/fields/required.d.ts
interface BlankRequiredBuilder<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, Metadata> {
  allow<const V extends Value>(values: ArrayOfMinSizeTwo<V>): BuildableRequiredConfig<V, Input, Output, CtxOptions, Metadata, 'allow'>;
  requiredError(error: string | ((ctx: InitResolverCtx<Input, CtxOptions>) => string)): HasRequiredError<Value, Input, Output, CtxOptions, Metadata>;
  validate(validator: Validator<Value, Input, Output, CtxOptions, Metadata>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, 'validate'>;
}
interface HasRequiredError<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, Metadata> {
  allow<const V extends Value>(values: ArrayOfMinSizeTwo<V>): BuildableRequiredConfig<V, Input, Output, CtxOptions, Metadata, 'allow'>;
  validate(validator: Validator<Value, Input, Output, CtxOptions, Metadata>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, 'validate'>;
}
type BuildableRequiredConfig<Value extends Output[keyof Output], Input, Output, CtxOptions extends ObjectType, Metadata, ValidationState extends 'allow' | 'none' | 'validate' = 'none', HasAllowError extends boolean = false, HasReValidate extends boolean = false, HasIgnoreUpdate extends 'yes' | 'yes-computed' | 'no' = 'no', HasOnDelete extends boolean = false, HasOnFailure extends boolean = false, HasOnSuccess extends boolean = false> = (ValidationState extends 'none' ? {} : Buildable<NS.RequiredField<Value, Input, Output, CtxOptions, Metadata>>) & (ValidationState extends 'allow' ? HasAllowError extends true ? {} : {
  allowError(error: NotAllowedError<Metadata> | ((value: unknown, allowedValues: ArrayOfMinSizeOne<Value>) => NotAllowedError<Metadata>)): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, 'allow', true, HasReValidate, HasIgnoreUpdate, HasOnDelete, HasOnFailure, HasOnSuccess>;
} : {}) & (ValidationState extends 'none' ? {} : HasReValidate extends true ? {} : {
  reValidate(validator: ReValidator<Value, Input, Output, CtxOptions, Metadata>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, ValidationState, HasAllowError, true, HasIgnoreUpdate, HasOnDelete, HasOnFailure, HasOnSuccess>;
}) & (HasIgnoreUpdate extends 'no' ? {
  readonly(): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, ValidationState, HasAllowError, HasReValidate, 'yes', HasOnDelete, HasOnFailure, HasOnSuccess>;
  ignoreUpdate(resolver: NS.IgnoreUpdateHandler<Input, Output, CtxOptions>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, ValidationState, HasAllowError, HasReValidate, 'yes-computed', HasOnDelete, HasOnFailure, HasOnSuccess>;
} : {}) & (HasOnDelete extends true ? {} : {
  onDelete(handler: NS.DeleteHandler<Output, CtxOptions> | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, ValidationState, HasAllowError, HasReValidate, HasIgnoreUpdate, true, HasOnFailure, HasOnSuccess>;
}) & (HasOnFailure extends true ? {} : {
  onFailure(handler: NS.FailureHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.FailureHandler<Input, Output, CtxOptions>>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, ValidationState, HasAllowError, HasReValidate, HasIgnoreUpdate, HasOnDelete, true, HasOnSuccess>;
}) & (HasOnSuccess extends true ? {} : {
  onSuccess(handler: NS.SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>): BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, ValidationState, HasAllowError, HasReValidate, HasIgnoreUpdate, HasOnDelete, HasOnFailure, true>;
});
//#endregion
//#region src/schema/fields/virtual.d.ts
type BlankVirtualBuilder<Value extends Input[keyof Input], Input, Output, CtxOptions extends ObjectType, Metadata, HasAlias extends boolean = false> = (HasAlias extends true ? {} : {
  alias<Alias extends keyof Input>(name: Alias): BlankVirtualBuilder<Input[Alias], Input, Output, CtxOptions, Metadata, true>;
}) & {
  allow<const V extends Value>(values: ArrayOfMinSizeTwo<V>): BuildableVirtualConfig<V, Input, Output, CtxOptions, Metadata, HasAlias, 'allow'>;
  validate(validator: Validator<Value, Input, Output, CtxOptions, Metadata>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, 'validate'>;
};
/**
 * A virtual field's `validator` is mandatory at runtime, so - mirroring
 * Rust's `VirtualFieldBuilder`, where every rule but `alias` requires
 * `HasValidator: Yes` - everything below except `[BUILD]` itself only
 * unlocks once `.allow()` or `.validate()` has been called (mutually
 * exclusive, same rule as lax/required).
 */
type BuildableVirtualConfig<Value extends Input[keyof Input], Input, Output, CtxOptions extends ObjectType, Metadata, HasAlias extends boolean = false, ValidationState extends 'allow' | 'none' | 'validate' = 'none', HasAllowError extends boolean = false, HasReValidate extends boolean = false, HasRequired extends boolean = false, HasSanitizer extends boolean = false, HasIgnore extends 'init' | 'update' | 'ignore' | 'none' = 'none', HasOnFailure extends boolean = false, HasOnSuccess extends boolean = false> = (HasAlias extends true ? {} : {
  alias<Alias extends keyof Input>(name: Alias): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, true, ValidationState, HasAllowError, HasSanitizer, HasRequired, HasSanitizer, HasIgnore, HasOnFailure, HasOnSuccess>;
}) & (ValidationState extends 'none' ? {} : Buildable<NS.VirtualField<never, Value, Input, Output, CtxOptions, Metadata>>) & (ValidationState extends 'allow' ? HasAllowError extends true ? {} : {
  allowError(error: NotAllowedError<Metadata> | ((value: unknown, allowedValues: ArrayOfMinSizeOne<unknown>) => NotAllowedError<Metadata>)): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, 'allow', true, HasReValidate, HasRequired, HasSanitizer, HasIgnore, HasOnFailure, HasOnSuccess>;
} : {}) & (ValidationState extends 'none' ? {} : HasReValidate extends true ? {} : {
  reValidate(validator: ReValidator<Value, Input, Output, CtxOptions, Metadata>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, true, HasRequired, HasSanitizer, HasIgnore, HasOnFailure, HasOnSuccess>;
}) & (ValidationState extends 'none' ? {} : HasRequired extends true ? {} : {
  required(handler: RequiredHandler<Input, Output, CtxOptions, Metadata>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, true, HasSanitizer, HasIgnore, HasOnFailure, HasOnSuccess>;
}) & (ValidationState extends 'none' ? {} : HasSanitizer extends true ? {} : {
  sanitize(sanitizer: NS.VirtualResolver<unknown, Input, Output, CtxOptions>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, HasRequired, true, HasIgnore, HasOnFailure, HasOnSuccess>;
}) & (ValidationState extends 'none' ? {} : HasIgnore extends 'none' ? {
  ignore(resolver: NS.Resolver<boolean, Input, Output, CtxOptions>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, HasRequired, HasSanitizer, 'ignore', HasOnFailure, HasOnSuccess>;
  ignoreInit(): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, HasRequired, HasSanitizer, 'init', HasOnFailure, HasOnSuccess>;
  ignoreUpdate(): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, HasRequired, HasSanitizer, 'update', HasOnFailure, HasOnSuccess>;
} : {}) & (ValidationState extends 'none' ? {} : HasOnFailure extends true ? {} : {
  onFailure(handler: NS.FailureHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.FailureHandler<Input, Output, CtxOptions>>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, HasRequired, HasSanitizer, HasIgnore, true, HasOnSuccess>;
}) & (ValidationState extends 'none' ? {} : HasOnSuccess extends true ? {} : {
  onSuccess(handler: NS.SuccessHandler<Input, Output, CtxOptions> | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>): BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, HasAlias, ValidationState, HasAllowError, HasReValidate, HasRequired, HasSanitizer, HasIgnore, HasOnFailure, true>;
});
//#endregion
//#region src/schema/index.d.ts
declare class Schema<const I extends RealType<I>, const O extends RealType<O> = I, const CtxOptions extends ObjectType = {}, const ErrorMetadata = DefaultFieldErrorMetadata, const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>> {
  private _definitions;
  private _options;
  constructor(builder: (f: FieldBuilder<I, O, CtxOptions, ErrorMetadata>) => FieldBuilder<I, O, CtxOptions, ErrorMetadata>, options?: NS.Options<I, O, CtxOptions, ErrorMetadata, ErrorPayload>);
  get definitions(): NS.Definitions<I, O, CtxOptions, ErrorMetadata>;
  get options(): NS.InternalOptions<I, O, CtxOptions, ErrorMetadata, ErrorPayload>;
  extend<const ExtendedI extends RealType<ExtendedI>, const ExtendedO extends RealType<ExtendedO> = ExtendedI, const ExtendedCtxOptions extends ObjectType = CtxOptions, const ExtendedErrorMetadata = ErrorMetadata, const ExtendedErrorPayload = IvoErrorPayload<ExtendedErrorMetadata, KeyOf<ExtendedI>>>(builder: (b: FieldBuilder<ExtendedI, ExtendedO, ExtendedCtxOptions, ExtendedErrorMetadata>) => FieldBuilder<ExtendedI, ExtendedO, ExtendedCtxOptions, ExtendedErrorMetadata>, options?: NS.ExtensionOptions<I, O, ExtendedI, ExtendedO, ExtendedCtxOptions, ExtendedErrorMetadata, ExtendedErrorPayload>): Schema<ExtendedI, ExtendedO, ExtendedCtxOptions, ExtendedErrorMetadata, ExtendedErrorPayload>;
  getModel(): Model<I, O, CtxOptions, ErrorMetadata, ErrorPayload>;
}
declare const FIELD_BUILDER_DEFINITIONS: unique symbol;
declare class FieldBuilder<const I extends RealType<I>, const O extends RealType<O> = I, const CtxOptions extends ObjectType = {}, const ErrorMetadata = DefaultFieldErrorMetadata> {
  private _definitions;
  private _seeded;
  constructor(seed?: NS.Definitions<I, O, CtxOptions, ErrorMetadata>);
  field<K extends keyof I | keyof O>(c: NS.FieldDefinition<K, I, O, CtxOptions, ErrorMetadata>): this;
  constant<K extends keyof O & string>(name: K, value: O[K] | NS.ConstantResolver<O[K], I, O, CtxOptions>): ConstantBuilder<O[K], I, O, CtxOptions>;
  dependent<K extends keyof O & string>(name: K, dependsOn: NS.Dependables<K, I, O> | ArrayOfMinSizeOne<NS.Dependables<K, I, O>>): HasDependsOn<K, I, O, CtxOptions>;
  lax<K extends keyof O & string, Default extends O[K] | NS.Resolver<O[K], I, O, CtxOptions>, DefaultState extends 'value' | 'resolver' = Default extends Function ? 'resolver' : 'value'>(name: K, value: Default): BuildableLaxConfig<O[K], I, O, CtxOptions, ErrorMetadata, DefaultState>;
  required<K extends keyof O & string>(name: K): BlankRequiredBuilder<O[K], I, O, CtxOptions, ErrorMetadata>;
  virtual<K extends string>(name: K): BlankVirtualBuilder<any, I, O, CtxOptions, ErrorMetadata>;
  get [FIELD_BUILDER_DEFINITIONS](): NS.DefinitionsEntries<I, O, CtxOptions, ErrorMetadata>;
}
//#endregion
//#region src/utils/index.d.ts
declare function isFieldError(data: unknown): data is FieldError;
declare function isInputFieldError<Metadata>(data: unknown): data is InputFieldError<Metadata>;
declare function makeFieldError<Metadata>(value: InputPayload[string] | InputFieldError<Metadata>, fallbackMessage?: string): FieldError<Metadata>;
/**
 * tells whether `a` & `b` are equals
 * @param  depth how deep in nesting should equality checks be performed for objects
 */
declare function isEqual<T>(a: unknown, b: T, depth?: number): a is T;
declare function isFunctionLike<T extends Function>(value: unknown): value is T;
declare function isNullOrUndefined(value: unknown): value is null | undefined;
declare function isOneOf<const T>(value: unknown, values: ArrayOfMinSizeTwo<T> | Readonly<ArrayOfMinSizeTwo<T>>): value is T;
declare function isRecordLike<T extends ObjectType>(value: unknown): value is ObjectType<T>;
declare function isPropertyOf<T>(prop: string | number | symbol, object: T): prop is keyof T;
declare function toArray<T>(value: T | T[] | readonly T[]): T[];
//#endregion
//#region src/validators.d.ts
type ArrayValidatorOptions<PreFilteredType, ModType, FinalType> = {
  max?: number | ValueError;
  min?: number | ValueError;
  unique?: boolean;
  uniqueKey?: string;
} & ArrayFilterOptions<PreFilteredType, ModType, FinalType> & ArraySortOptions<FinalType>;
type ArrayFilterOptions<PreFilteredType, ModType, FinalType> = {
  filter: ArrayFilterFn<FinalType>;
  modifier?: never;
  postModFilter?: never;
  map?: never;
} | {
  filter: ArrayFilterFn<PreFilteredType>;
  modifier: (item: PreFilteredType) => ModType | Promise<ModType>;
  postModFilter?: (item: ModType) => boolean | Promise<boolean>;
  map?: (item: ModType) => FinalType | Promise<FinalType>;
};
type ArrayFilterFn<T> = ((item: unknown) => item is T) | ((item: unknown) => boolean | Promise<boolean>);
type ArraySortOptions<T> = {
  sort?: (a: T, b: T) => number;
  sortOrder?: never;
} | {
  sort?: boolean;
  sortOrder?: 'asc' | 'desc';
};
declare function makeArrayValidator<const PreFilteredType, const ModType = PreFilteredType, const FinalType = ModType>({ filter, map, modifier, postModFilter, max, min, sort, sortOrder, unique, uniqueKey }: ArrayValidatorOptions<PreFilteredType, ModType, FinalType>): (value: unknown) => Promise<ValidationResponse<FinalType[]>>;
declare function validateBoolean(value: unknown): ValidationResponse<boolean, DefaultFieldErrorMetadata>;
declare const validateCreditCard: (value: unknown) => {
  valid: true;
  validated: string | number;
} | {
  metadata: DefaultFieldErrorMetadata | null;
  reason: string;
  valid: false;
};
declare const validateEmail: (value: unknown, regExp?: RegExp) => {
  valid: true;
  validated: string;
} | {
  metadata: DefaultFieldErrorMetadata | null;
  reason: string;
  valid: false;
};
type AllowConfig<T> = ArrayOfMinSizeTwo<T> | {
  values: ArrayOfMinSizeTwo<T>;
  error: string;
};
type ExclusionConfig<T> = T | ArrayOfMinSizeTwo<T> | {
  values: T | ArrayOfMinSizeTwo<T>;
  error: string;
};
type ValueError<T = number> = {
  value: T;
  error: string;
};
type NumberValidatorOptions<T extends number | unknown = number> = {
  exclude?: ExclusionConfig<T>;
} & XOR<{
  allow: AllowConfig<T>;
}, {
  max?: number | ValueError;
  min?: number | ValueError;
  nullable?: boolean;
}>;
type StringValidatorOptions<T extends string | unknown = string> = {
  exclude?: ExclusionConfig<T>;
} & XOR<{
  allow: AllowConfig<T>;
}, {
  max?: number | ValueError;
  min?: number | ValueError;
  normalForm?: 'NFC' | 'NFD' | 'NFKC' | 'NFKD';
  normalize?: boolean;
  nullable?: boolean;
  regExp?: ValueError<RegExp>;
  trim?: boolean;
}>;
declare function makeNumberValidator<const T extends number | unknown = number>({ exclude, allow, max, min, nullable }?: NumberValidatorOptions<T>): (value: unknown) => ValidationResponse<T>;
declare function makeStringValidator<const T extends string | unknown = string>({ exclude, allow, max, min, normalForm, normalize, nullable, regExp, trim }?: StringValidatorOptions<T>): (value: unknown) => ValidationResponse<T>;
//#endregion
export { type ArrayOfMinSizeOne, type ArrayOfMinSizeTwo, type ArrayValidatorOptions, type FieldError, type InputFieldError, type InputPayload, type IvoContext, type KeyOf, type NumberValidatorOptions, type ReadonlyIvoContext, type RealType, Schema, type StringValidatorOptions, type ValidatorResponse, type ValidatorResponseObject, type XOR, isEqual, isFieldError, isFunctionLike, isInputFieldError, isNullOrUndefined, isOneOf, isPropertyOf, isRecordLike, makeArrayValidator, makeFieldError, makeNumberValidator, makeStringValidator, toArray, validateBoolean, validateCreditCard, validateEmail };