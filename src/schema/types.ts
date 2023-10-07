import { Schema } from '.'
import { FieldError, InputPayload } from './utils'

export type {
  Context,
  DefinitionRule,
  KeyOf,
  Merge,
  NonEmptyArray,
  Summary,
  Schema as ISchema,
  RealType,
  ResponseInputObject,
  TypeOf,
  Validator,
  ValidationResponse,
  ValidatorResponse,
  InternalValidatorResponse,
  XOR
}

export {
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  DEFINITION_RULES,
  LIFE_CYCLES,
  VIRTUAL_RULES
}

type Context<Output, Input = Output> = Readonly<Merge<Output, Input>>

type Summary<Output, Input = Output> = (
  | Readonly<{
      changes: null
      context: Context<Output, Input>
      operation: 'creation'
      previousValues: null
      values: Readonly<Output>
    }>
  | Readonly<{
      changes: Partial<RealType<Output>>
      context: Context<Output, Input>
      operation: 'update'
      previousValues: Readonly<Output>
      values: Readonly<Output>
    }>
) & {}

type TypeOf<T> = Exclude<T, undefined>

type AsyncSetter<T, Output, Input> = (
  context: Context<Output, Input> & {}
) => TypeOf<T> | Promise<TypeOf<T>>

type Setter<T, Output, Input> = (
  context: Context<Output, Input> & {}
) => TypeOf<T>

type SetterWithSummary<T, Output, Input> = (
  summary: Summary<Output, Input> & {}
) => TypeOf<T>

type Resolver<K extends keyof Output, Output, Input> = (
  summary: Summary<Output, Input> & {}
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>

type VirtualResolver<K extends keyof Input, Output, Input> = (
  summary: Summary<Output, Input> & {}
) => TypeOf<Input[K]> | Promise<TypeOf<Input[K]>>

type KeyOf<T> = Extract<keyof T, string>

namespace Schema {
  export type LifeCycle = (typeof LIFE_CYCLES)[number]

  export type OperationName = 'creation' | 'update'

  export type Handler<Output> = (data: Readonly<Output>) => any | Promise<any>

  export type FailureHandler<Output, Input> = (
    context: Context<Output, Input> & {}
  ) => any | Promise<any>

  export type SuccessHandler<Output, Input> = (
    summary: Summary<Output, Input> & {}
  ) => any | Promise<any>

  export type Definitions<Output, Input = Output, Aliases = {}> = {
    [K in keyof Merge<Output, Input>]?: K extends keyof (Input | Output)
      ? PublicProperty<K, Output, Input>
      : K extends keyof Omit<Output, keyof Input>
      ? PrivateProperty<K, Output, Input>
      : K extends keyof Omit<Input, keyof Output>
      ? Virtual<K, Output, Input, Aliases>
      : never
  }

  type PublicProperty<K extends keyof (Output | Input), Output, Input> =
    | LaxProperty<K, Output, Input>
    | ReadOnly<K, Output, Input>
    | ReadonlyNoInit<K, Output, Input>
    | Required<K, Output, Input>
    | RequiredBy<K, Output, Input>
    | RequiredReadonly<K, Output, Input>

  type PrivateProperty<K extends keyof Output, Output, Input> = XOR<
    Constant<K, Output, Input>,
    Dependent<K, Output, Input>
  >

  export type Definitions_<Output, Input> = {
    [K in keyof Input]?: Listenable<Output, Input> & {
      alias?: string
      constant?: any
      default?: any
      dependent?: boolean
      dependsOn?: KeyOf<Input> | KeyOf<Input>[]
      readonly?: boolean | 'lax'
      resolver?: Function
      required?:
        | boolean
        | SetterWithSummary<boolean | [boolean, string], Output, Input>
      sanitizer?: VirtualResolver<K, Output, Input>
      shouldInit?: false | Setter<boolean, Output, Input>
      shouldUpdate?: false | Setter<boolean, Output, Input>
      validator?: Function
      value?: any
      virtual?: boolean
    }
  }

  export type AliasToVirtualMap<T> = Record<string, KeyOf<T>>

  export type VirtualToAliasMap<T> = Record<KeyOf<T>, string>

  export type DependencyMap<T> = {
    [K in KeyOf<T>]?: KeyOf<T>[]
  }

  type Listenable<Output, Input> = {
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>
    onFailure?:
      | FailureHandler<Output, Input>
      | NonEmptyArray<FailureHandler<Output, Input>>
    onSuccess?:
      | SuccessHandler<Output, Input>
      | NonEmptyArray<SuccessHandler<Output, Input>>
  }

  type Constant<K extends keyof Output, Output, Input = Output> = {
    constant: true
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>
    onSuccess?:
      | SuccessHandler<Output, Input>
      | NonEmptyArray<SuccessHandler<Output, Input>>
    value: TypeOf<Output[K]> | AsyncSetter<Output[K], Output, Input>
  }

  type Dependent<K extends keyof Output, Output, Input = Output> = {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Output, Input>
    /** @deprecated `dependsOn` & a `resolver` function are enough to make a property dependent */
    dependent?: true
    dependsOn:
      | Exclude<KeyOf<Context<Output, Input>>, K>
      | Exclude<KeyOf<Context<Output, Input>>, K>[]
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>
    onSuccess?:
      | SuccessHandler<Output, Input>
      | NonEmptyArray<SuccessHandler<Output, Input>>
    readonly?: true
    resolver: Resolver<K, Output, Input>
  }

  type LaxProperty<
    K extends keyof (Output | Input),
    Output,
    Input = Output
  > = Listenable<Output, Input> & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Output, Input>
    readonly?: 'lax'
    shouldInit?: false | Setter<boolean, Output, Input>
    shouldUpdate?: Setter<boolean, Output, Input>
    validator?: Validator<K, Output, Input>
  }

  type ReadOnly<
    K extends keyof (Output | Input),
    Output,
    Input = Output
  > = Listenable<Output, Input> & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Output, Input>
    readonly: 'lax'
    shouldUpdate?: Setter<boolean, Output, Input>
    validator: Validator<K, Output, Input>
  }

  type ReadonlyNoInit<
    K extends keyof (Output | Input),
    Output,
    Input = Output
  > = Listenable<Output, Input> & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Output, Input>
    readonly: true
    shouldInit: false | Setter<boolean, Output, Input>
    shouldUpdate?: Setter<boolean, Output, Input>
    validator?: Validator<K, Output, Input>
  }

  type RequiredReadonly<
    K extends keyof (Output | Input),
    Output,
    Input = Output
  > = Listenable<Output, Input> & {
    readonly: true
    validator: Validator<K, Output, Input>
  }

  type Required<
    K extends keyof (Output | Input),
    Output,
    Input = Output
  > = Listenable<Output, Input> & {
    required: true
    shouldUpdate?: Setter<boolean, Output, Input>
    validator: Validator<K, Output, Input>
  }

  type RequiredBy<
    K extends keyof (Output | Input),
    Output,
    Input = Output
  > = Listenable<Output, Input> & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Output, Input>
    required: SetterWithSummary<boolean | [boolean, string], Output, Input>
    readonly?: true
    shouldInit?: Setter<boolean, Output, Input>
    shouldUpdate?: Setter<boolean, Output, Input>
    validator: Validator<K, Output, Input>
  }

  type Virtual<K extends keyof Input, Output, Input, A> = {
    alias?: Exclude<KeyOf<A>, K> extends undefined
      ? string
      : Exclude<KeyOf<A>, K>
    required?: SetterWithSummary<boolean | [boolean, string], Output, Input>
    virtual: true
    sanitizer?: VirtualResolver<K, Output, Input>
    onFailure?:
      | FailureHandler<Output, Input>
      | NonEmptyArray<FailureHandler<Output, Input>>
    onSuccess?:
      | SuccessHandler<Output, Input>
      | NonEmptyArray<SuccessHandler<Output, Input>>
    shouldInit?: false | Setter<boolean, Output, Input>
    shouldUpdate?: false | Setter<boolean, Output, Input>
    validator: Validator<K, Output, Input>
  }

  // options
  export type CloneOptions<T> = {
    reset?: KeyOf<T> | KeyOf<T>[]
  }

  export type Options<Output, Input> = {
    equalityDepth?: number
    errors?: 'silent' | 'throw'
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>
    onSuccess?:
      | SuccessHandler<Output, Input>
      | NonEmptyArray<SuccessHandler<Output, Input>>
    setMissingDefaultsOnUpdate?: boolean
    shouldUpdate?: boolean | SetterWithSummary<boolean, Output, Input>
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string }
  }

  export type OptionsKey<Output, Input> = KeyOf<Options<Output, Input>>

  export type PrivateOptions = { timestamps: Timestamp }

  export type Timestamp = { createdAt: string; updatedAt: string }

  export type ExtensionOptions<ParentOutput, ParentInput, Output, Input> =
    Options<Output, Input> & {
      remove?:
        | KeyOf<Merge<ParentInput, ParentOutput>>
        | KeyOf<Merge<ParentInput, ParentOutput>>[]
      useParentOptions?: boolean
    }
}

type ValidationResponse<T> =
  | { valid: true; validated: T }
  | { metadata: FieldError['metadata']; reasons: string[]; valid: false }

type InternalValidatorResponse<T> =
  | { valid: true; validated: T }
  | {
      metadata?: FieldError['metadata']
      otherReasons?: InputPayload
      reasons: FieldError['reasons']
      valid: false
    }

type ResponseInputObject<K extends keyof (Output & Input), Output, Input> =
  | { valid: true; validated?: TypeOf<(Output & Input)[K]> }
  | {
      otherReasons?: {
        [Key in Exclude<keyof (Output & Input), K>]:
          | string
          | string[]
          | FieldError
      }
      metadata?: FieldError['metadata']
      reason?: FieldError['reasons'][number]
      reasons?: FieldError['reasons']
      valid: false
    }

type ValidatorResponse<K extends keyof (Output & Input), Output, Input> =
  | boolean
  | (ResponseInputObject<K, Output, Input> & {})

type Validator<K extends keyof (Output & Input), Output, Input> = (
  value: any,
  summary: Summary<Output, Input> & {}
) =>
  | ValidatorResponse<K, Output, Input>
  | Promise<ValidatorResponse<K, Output, Input>>

type NonEmptyArray<T> = [T, ...T[]]

const DEFINITION_RULES = [
  'alias',
  'constant',
  'default',
  'dependent',
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
] as const

type DefinitionRule = (typeof DEFINITION_RULES)[number]

const ALLOWED_OPTIONS: Schema.OptionsKey<any, any>[] = [
  'equalityDepth',
  'errors',
  'onDelete',
  'onSuccess',
  'setMissingDefaultsOnUpdate',
  'shouldUpdate',
  'timestamps'
]
const CONSTANT_RULES = ['constant', 'onDelete', 'onSuccess', 'value']
const VIRTUAL_RULES = [
  'alias',
  'sanitizer',
  'onFailure',
  'onSuccess',
  'required',
  'shouldInit',
  'shouldUpdate',
  'validator',
  'virtual'
]

const LIFE_CYCLES = ['onDelete', 'onFailure', 'onSuccess'] as const

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T

type RealType_<T> = T extends (...args: any) => infer I ? I : T

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>
} & {}

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
  >
}
type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never }

type XOR<T, U> = (T | U extends object
  ? (Without<T, U> & U) | ((Without<U, T> & T) & {})
  : T | U) & {}
