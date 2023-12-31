import { ObjectType } from '../utils';
import { FieldError, IErrorTool, ValidationErrorMessage } from './utils';

export type {
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  Context,
  DeleteContext,
  DefinitionRule,
  KeyOf,
  Merge,
  NS,
  NotAllowedError,
  PartialContext,
  Summary,
  RealType,
  ResponseErrorObject,
  TypeOf,
  Validator,
  ValidationResponse,
  ValidatorResponse,
  ValidatorResponseObject,
  InternalValidatorResponse,
  InvalidValidatorResponse,
  XOR
};

export {
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  DEFINITION_RULES,
  LIFE_CYCLES,
  OPERATIONS,
  VIRTUAL_RULES
};

type Context<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {}
> = Readonly<
  Merge<Input, Output> & { __getOptions__: () => Readonly<CtxOptions> }
> & {};

type DeleteContext<Output, CtxOptions extends ObjectType = {}> = Readonly<
  Output & { __getOptions__: () => Readonly<CtxOptions> }
> & {};

type PartialContext<Input, Output> = Readonly<Merge<Input, Output>>;

type Summary<Input, Output = Input, CtxOptions extends ObjectType = {}> = (
  | Readonly<{
      changes: null;
      context: Context<Input, Output, CtxOptions>;
      operation: 'creation';
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<RealType<Output>>;
      context: Context<Input, Output, CtxOptions>;
      operation: 'update';
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>
) & {};

type TypeOf<T> = Exclude<T, undefined>;

type AsyncSetter<T, Input, Output, CtxOptions extends ObjectType = {}> = (
  context: Context<Input, Output, CtxOptions>
) => TypeOf<T> | Promise<TypeOf<T>>;

type NotAllowedError = string | string[] | FieldError;

type Setter<T, Input, Output, CtxOptions extends ObjectType = {}> = (
  context: Context<Input, Output, CtxOptions>
) => TypeOf<T>;

type SetterWithSummary<T, Input, Output, CtxOptions extends ObjectType = {}> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) => TypeOf<T>;

type AsyncShouldUpdateResponse<CtxOptions extends ObjectType = {}> = {
  update: boolean;
  contextOptionsUpdate?: Partial<CtxOptions>;
};

type AsyncShouldUpdate<Input, Output, CtxOptions extends ObjectType = {}> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) =>
  | boolean
  | AsyncShouldUpdateResponse
  | Promise<boolean>
  | Promise<AsyncShouldUpdateResponse>;

type Resolver<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType = {}
> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>;

type VirtualResolver<
  K extends keyof Input,
  Input,
  Output,
  CtxOptions extends ObjectType = {}
> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) => TypeOf<Input[K]> | Promise<TypeOf<Input[K]>>;

type KeyOf<T> = Extract<keyof T, string>;

namespace NS {
  export type LifeCycle = (typeof LIFE_CYCLES)[number];

  export type Operation = (typeof OPERATIONS)[number];

  export type DeleteHandler<Output, CtxOptions extends ObjectType> = (
    data: DeleteContext<Output, CtxOptions>
  ) => any | Promise<any>;

  export type FailureHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = (context: Context<Input, Output, CtxOptions> & {}) => any | Promise<any>;

  export type SuccessHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = (summary: Summary<Input, Output, CtxOptions> & {}) => any | Promise<any>;

  export type Definitions<
    Input,
    Output,
    Aliases = {},
    CtxOptions extends ObjectType = {}
  > = {
    [K in keyof (Input & Output)]?: K extends keyof (Input | Output)
      ? PublicProperty<K, Input, Output, Aliases, CtxOptions>
      : K extends keyof Omit<Output, keyof Input>
      ? PrivateProperty<K, Input, Output, CtxOptions>
      : K extends keyof Omit<Input, keyof Output>
      ? Virtual<K, Input, Output, Aliases, CtxOptions>
      : never;
  };

  type PublicProperty<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Enumerable<Output[K]> &
    (
      | LaxProperty<K, Input, Output, Aliases, CtxOptions>
      | ReadOnly<K, Input, Output, Aliases, CtxOptions>
      | ReadonlyNoInit<K, Input, Output, Aliases, CtxOptions>
      | Required<K, Input, Output, Aliases, CtxOptions>
      | RequiredBy<K, Input, Output, Aliases, CtxOptions>
      | RequiredReadonly<K, Input, Output, Aliases, CtxOptions>
    );

  type PrivateProperty<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = XOR<
    Constant<K, Input, Output, CtxOptions>,
    Dependent<K, Input, Output, CtxOptions>
  >;

  export type Definitions_<Input, Output> = {
    [K in keyof Input]?: Listenable<Input, Output> & {
      allow?: Readonly<ArrayOfMinSizeTwo<any>>;
      alias?: string;
      constant?: any;
      default?: any;
      dependsOn?: KeyOf<Input> | KeyOf<Input>[];
      readonly?: boolean | 'lax';
      resolver?: Function;
      required?:
        | boolean
        | SetterWithSummary<boolean | [boolean, string], Input, Output>;
      sanitizer?: VirtualResolver<K, Input, Output>;
      shouldInit?: false | Setter<boolean, Input, Output>;
      shouldUpdate?: false | Setter<boolean, Input, Output>;
      validator?: Function;
      value?: any;
      virtual?: boolean;
    };
  };

  export type AliasToVirtualMap<T> = Record<string, KeyOf<T>>;

  export type VirtualToAliasMap<T> = Record<KeyOf<T>, string>;

  export type DependencyMap<T> = {
    [K in KeyOf<T>]?: KeyOf<T>[];
  };

  type Listenable<Input, Output, CtxOptions extends ObjectType = {}> = {
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  type Constant<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = {
    constant: true;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    value:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
  };

  type Enumerable<T> = {
    allow?: Readonly<ArrayOfMinSizeTwo<T>>;
    notAllowedError?:
      | NotAllowedError
      | ((value: any, allowedValues: ArrayOfMinSizeTwo<T>) => NotAllowedError);
  };

  type Dependables<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Exclude<KeyOf<Context<Input, Output, CtxOptions>>, K | '__getOptions__'>;

  type Dependent<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    dependsOn:
      | Dependables<K, Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<Dependables<K, Input, Output, CtxOptions>>;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    readonly?: true;
    resolver: Resolver<K, Input, Output, CtxOptions>;
  };

  type InitAndUpdateBlockable<
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = XOR<
    {
      shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
      shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    },
    XOR<
      {
        shouldInit?: false | Setter<boolean, Input, Output, CtxOptions>;
        shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
      },
      {
        shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
        shouldUpdate?: false | Setter<boolean, Input, Output, CtxOptions>;
      }
    >
  >;

  type LaxProperty<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output> &
    InitAndUpdateBlockable<Input, Output, CtxOptions> & {
      default:
        | TypeOf<Output[K]>
        | AsyncSetter<Output[K], Input, Output, CtxOptions>;
      validator?: Validator<K, Input, Output, Aliases, CtxOptions>;
    };

  type ReadOnly<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly: 'lax';
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, Aliases, CtxOptions>;
  };

  type ReadonlyNoInit<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly: true;
    shouldInit: false | Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator?: Validator<K, Input, Output, Aliases, CtxOptions>;
  };

  type RequiredReadonly<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    readonly: true;
    validator: Validator<K, Input, Output, Aliases, CtxOptions>;
  };

  type Required<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    required: true;
    shouldUpdate?: false | Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, Aliases, CtxOptions>;
  };

  type RequiredBy<
    K extends keyof (Output | Input),
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    required: SetterWithSummary<
      boolean | [boolean, string],
      Input,
      Output,
      CtxOptions
    >;
    readonly?: true;
    shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, Aliases, CtxOptions>;
  };

  type Virtual<
    K extends keyof Input,
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = InitAndUpdateBlockable<Input, Output, CtxOptions> &
    Enumerable<Input[K]> & {
      alias?: Exclude<KeyOf<Aliases>, K> extends undefined
        ? string
        : Exclude<KeyOf<Aliases>, K>;
      required?: SetterWithSummary<
        boolean | [boolean, string],
        Input,
        Output,
        CtxOptions
      >;
      virtual: true;
      sanitizer?: VirtualResolver<K, Input, Output, CtxOptions>;
      onFailure?:
        | FailureHandler<Input, Output, CtxOptions>
        | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
      onSuccess?:
        | SuccessHandler<Input, Output, CtxOptions>
        | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
      validator: VirtualValidator<K, Input, Output, Aliases, CtxOptions>;
    };

  export type InternalOptions<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
    ErrorTool extends IErrorTool<any> = any
  > = {
    ErrorTool: ErrorToolClass<ErrorTool, CtxOptions>;
    equalityDepth: number;
    errors: 'silent' | 'throw';
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | AsyncShouldUpdate<Input, Output, CtxOptions>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type Options<
    Input,
    Output,
    ErrorTool extends IErrorTool<any> = any,
    CtxOptions extends ObjectType = {}
  > = {
    ErrorTool?: ErrorToolClass<ErrorTool, CtxOptions>;
    equalityDepth?: number;
    errors?: 'silent' | 'throw';
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | AsyncShouldUpdate<Input, Output, CtxOptions>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type OptionsKey<
    Input,
    Output,
    ErrorTool extends IErrorTool<any>
  > = KeyOf<Options<Input, Output, ErrorTool>>;

  export type PrivateOptions = { timestamps: Timestamp };

  export type Timestamp = { createdAt: string; updatedAt: string };

  export type ExtensionOptions<
    ParentInput,
    ParentOutput,
    Input,
    Output,
    ErrorTool extends IErrorTool<any>,
    CtxOptions extends ObjectType = {}
  > = Options<Input, Output, ErrorTool, CtxOptions> & {
    remove?:
      | KeyOf<Merge<ParentInput, ParentOutput>>
      | KeyOf<Merge<ParentInput, ParentOutput>>[];
    useParentOptions?: boolean;
  };
}

type ValidationResponse<T> =
  | { valid: true; validated: T }
  | { metadata: FieldError['metadata']; reason: string[]; valid: false };

type InvalidValidatorResponse<Input = {}, Aliases = {}> = {
  metadata?: FieldError['metadata'];
  reason?: string | string[] | ResponseErrorObject<Input, Aliases>;
  valid: false;
  value?: any;
};

type InternalValidatorResponse<T> =
  | { valid: true; validated: T }
  | InvalidValidatorResponse;

type ValidatorResponseObject<T, Input = {}, Aliases = {}> =
  | { valid: true; validated?: T }
  | InvalidValidatorResponse<Input, Aliases>;

type ResponseErrorObject<Input = {}, Aliases = {}> = {
  [K in KeyOf<Input & Aliases>]: string | string[] | InputFieldError;
};

type InputFieldError = {
  reason: FieldError['reasons'][number] | FieldError['reasons'];
  metadata?: FieldError['metadata'];
};

type ValidatorResponse<T, Input, Aliases = {}> =
  | boolean
  | (ValidatorResponseObject<T, Input, Aliases> & {});

type Validator<
  K extends keyof (Output | Input),
  Input,
  Output,
  Aliases = {},
  CtxOptions extends ObjectType = {}
> = (
  value: any,
  summary: Summary<Input, Output, CtxOptions> & {}
) =>
  | ValidatorResponse<TypeOf<Output[K]>, Input, Aliases>
  | Promise<ValidatorResponse<TypeOf<Output[K]>, Input, Aliases>>;

type VirtualValidator<
  K extends keyof Input,
  Input,
  Output,
  Aliases = {},
  CtxOptions extends ObjectType = {}
> = (
  value: any,
  summary: Summary<Input, Output, CtxOptions> & {}
) =>
  | ValidatorResponse<TypeOf<Input[K]>, Input, Aliases>
  | Promise<ValidatorResponse<TypeOf<Input[K]>, Input, Aliases>>;

type ArrayOfMinSizeOne<T> = [T, ...T[]];
type ArrayOfMinSizeTwo<T> = [T, T, ...T[]];

const DEFINITION_RULES = [
  'alias',
  'allow',
  'constant',
  'default',
  'dependsOn',
  'onDelete',
  'onFailure',
  'onSuccess',
  'readonly',
  'resolver',
  'required',
  'sanitizer',
  'shouldInit',
  'shouldUpdate',
  'validator',
  'value',
  'virtual'
] as const;

type DefinitionRule = (typeof DEFINITION_RULES)[number];

const ALLOWED_OPTIONS: NS.OptionsKey<any, any, any>[] = [
  'ErrorTool',
  'equalityDepth',
  'errors',
  'onDelete',
  'onSuccess',
  'setMissingDefaultsOnUpdate',
  'shouldUpdate',
  'timestamps'
];
const CONSTANT_RULES = ['constant', 'onDelete', 'onSuccess', 'value'];
const VIRTUAL_RULES = [
  'alias',
  'allow',
  'sanitizer',
  'onFailure',
  'onSuccess',
  'required',
  'shouldInit',
  'shouldUpdate',
  'validator',
  'virtual'
];

const LIFE_CYCLES = ['onDelete', 'onFailure', 'onSuccess'] as const;
const OPERATIONS = ['creation', 'update'] as const;

interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

type RealType_<T> = T extends (...args: any) => infer I ? I : T;

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>;
} & {};

type Merge<A, B> = {
  [K in keyof A | keyof B]: Exclude<
    K extends keyof A
      ? K extends keyof B
        ? A[K] | B[K]
        : A[K]
      : K extends keyof B
      ? B[K]
      : never,
    undefined
  >;
};

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };

type XOR<T, U> = (T | U extends object
  ? (Without<T, U> & U) | ((Without<U, T> & T) & {})
  : T | U) & {};
