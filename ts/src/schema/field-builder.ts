import type { ObjectType } from '../utils';
import {
  type ArrayOfMinSizeOne,
  BUILD,
  type IvoContext,
  type KeyOf,
  type NS,
  type TypeOf,
} from './types';

export { createFieldBuilder, materializeFieldBuilders };

/**
 * Prototype of a Rust-style typestate builder for the "dependent" field
 * nature (`rs/src/schema/fields/dependents.rs`'s `DependentFieldBuilder`).
 *
 * `default` and `dependsOn` can be supplied in either order, but `resolve`
 * only becomes callable once both are set — each stage is a distinct
 * interface exposing only the methods valid at that stage, so calling things
 * out of order is a compile error rather than a runtime one.
 * `readonly`/`onDelete`/`onSuccess` are only available on the finished
 * builder, matching the Rust impl.
 *
 * There is no user-facing `.build()`: the finished builder exposes its
 * resolved config only through the `BUILD` symbol hook, which isn't part of
 * `Buildable`'s public surface. So the chain's result can be dropped
 * directly into a `Definitions` object literal, and `Schema` resolves it
 * internally via `materializeFieldBuilders` - the only place that imports
 * `BUILD` as a value.
 */

type Dependables<K extends keyof Output, Input, Output> = Exclude<
  KeyOf<Input> | KeyOf<Output>,
  K
>;

type AsyncSetter<T, Input, Output, CtxOptions extends ObjectType> = (
  ctx: IvoContext<Input, Output, CtxOptions>,
) => TypeOf<T> | Promise<TypeOf<T>>;

type Resolver<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>;

export type DependentFieldConfig<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> = {
  default:
    | TypeOf<Output[K]>
    | AsyncSetter<Output[K], Input, Output, CtxOptions>;
  dependsOn:
    | Dependables<K, Input, Output>
    | ArrayOfMinSizeOne<Dependables<K, Input, Output>>;
  resolver: Resolver<K, Input, Output, CtxOptions>;
  readonly?: true;
  onDelete?:
    | NS.DeleteHandler<Output, CtxOptions>
    | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>;
  onSuccess?:
    | NS.SuccessHandler<Input, Output, CtxOptions>
    | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>;
};

interface HasNeither<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  default(
    value:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>,
  ): HasDefault<K, Input, Output, CtxOptions>;
  dependsOn(
    deps:
      | Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<Dependables<K, Input, Output>>,
  ): HasDependsOn<K, Input, Output, CtxOptions>;
}

interface HasDefault<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  dependsOn(
    deps:
      | Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<Dependables<K, Input, Output>>,
  ): ReadyToResolve<K, Input, Output, CtxOptions>;
}

interface HasDependsOn<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  default(
    value:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>,
  ): ReadyToResolve<K, Input, Output, CtxOptions>;
}

interface ReadyToResolve<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  resolve(
    resolver: Resolver<K, Input, Output, CtxOptions>,
  ): Buildable<K, Input, Output, CtxOptions>;
}

/**
 * Unlike the Rust builders - where `on_delete`/`on_success` are called once
 * per handler, chained repeatedly to attach several - the TS builder allows
 * exactly one call to each, accepting either a single handler or an array of
 * them (matching the plain object `Definitions` shape's
 * `T | ArrayOfMinSizeOne<T>` convention). `readonly`/`onDelete`/`onSuccess`
 * are independent and can be called in any order, but each becomes
 * unavailable on the returned type once used - three independent typestate
 * flags rather than a single linear chain.
 */
type Buildable<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = {
  [BUILD](): DependentFieldConfig<K, Input, Output, CtxOptions>;
} & (HasReadonly extends true
  ? {}
  : {
      readonly(): Buildable<
        K,
        Input,
        Output,
        CtxOptions,
        true,
        HasOnDelete,
        HasOnSuccess
      >;
    }) &
  (HasOnDelete extends true
    ? {}
    : {
        onDelete(
          handler:
            | NS.DeleteHandler<Output, CtxOptions>
            | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>,
        ): Buildable<
          K,
          Input,
          Output,
          CtxOptions,
          HasReadonly,
          true,
          HasOnSuccess
        >;
      }) &
  (HasOnSuccess extends true
    ? {}
    : {
        onSuccess(
          handler:
            | NS.SuccessHandler<Input, Output, CtxOptions>
            | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>,
        ): Buildable<
          K,
          Input,
          Output,
          CtxOptions,
          HasReadonly,
          HasOnDelete,
          true
        >;
      });

class DependentBuilder<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> implements
    HasNeither<K, Input, Output, CtxOptions>,
    HasDefault<K, Input, Output, CtxOptions>,
    HasDependsOn<K, Input, Output, CtxOptions>,
    ReadyToResolve<K, Input, Output, CtxOptions>,
    Buildable<K, Input, Output, CtxOptions>
{
  private config: Partial<DependentFieldConfig<K, Input, Output, CtxOptions>> =
    {};

  default(
    value:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>,
  ) {
    this.config.default = value;
    return this as never;
  }

  dependsOn(
    deps:
      | Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<Dependables<K, Input, Output>>,
  ) {
    this.config.dependsOn = deps;
    return this as never;
  }

  resolve(resolver: Resolver<K, Input, Output, CtxOptions>) {
    this.config.resolver = resolver;
    return this as never;
  }

  readonly() {
    this.config.readonly = true;
    return this as never;
  }

  onDelete(
    handler:
      | NS.DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>,
  ) {
    this.config.onDelete = handler;
    return this as never;
  }

  onSuccess(
    handler:
      | NS.SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>,
  ) {
    this.config.onSuccess = handler;
    return this as never;
  }

  [BUILD]() {
    return this.config as DependentFieldConfig<K, Input, Output, CtxOptions>;
  }
}

/**
 * Binds `Input`/`Output`/`CtxOptions` once (mirroring how they're pinned on
 * `Schema<Input, Output, CtxOptions>`), then infers `K` per-field from the
 * property name passed to `.dependent(...)`.
 */
function createFieldBuilder<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
>() {
  return {
    dependent<K extends keyof Output>(
      _key: K,
    ): HasNeither<K, Input, Output, CtxOptions> {
      return new DependentBuilder<K, Input, Output, CtxOptions>();
    },
  };
}

function isFieldBuilder(value: unknown): value is { [BUILD]: () => unknown } {
  return (
    typeof value === 'object' &&
    value !== null &&
    typeof (value as { [BUILD]?: unknown })[BUILD] === 'function'
  );
}

/**
 * Called by `Schema` before processing a `Definitions` object, so users
 * never get (or need) a way to resolve a field-builder chain themselves -
 * mirrors how the Rust schema macro resolves `impl BuildableFieldConfig`
 * under the hood. This is the only place in the codebase that imports
 * `BUILD` as a value, which is what keeps it out of reach elsewhere.
 */
function materializeFieldBuilders<T extends ObjectType>(definitions: T): T {
  const result: ObjectType = { ...definitions };

  for (const key of Object.keys(result)) {
    const value = result[key];

    if (isFieldBuilder(value)) result[key] = value[BUILD]();
  }

  return result as T;
}
