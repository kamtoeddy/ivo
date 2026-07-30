import type { ObjectType } from '../../utils';
import { type ArrayOfMinSizeOne, BUILD, type NS, type TypeOf } from '../types';

export { type BlankDependentBuilder, DependentBuilder };

interface BlankDependentBuilder<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  default(
    value: TypeOf<Output[K]> | NS.Setter<Output[K], Input, Output, CtxOptions>,
  ): HasDefault<K, Input, Output, CtxOptions>;
  dependsOn(
    deps:
      | NS.Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<NS.Dependables<K, Input, Output>>,
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
      | NS.Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<NS.Dependables<K, Input, Output>>,
  ): ReadyToResolve<K, Input, Output, CtxOptions>;
}

interface HasDependsOn<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  default(
    value: TypeOf<Output[K]> | NS.Setter<Output[K], Input, Output, CtxOptions>,
  ): ReadyToResolve<K, Input, Output, CtxOptions>;
}

interface ReadyToResolve<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  resolve(
    resolver: NS.Resolver<K, Input, Output, CtxOptions>,
  ): BuildableDependentConfig<K, Input, Output, CtxOptions>;
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
type BuildableDependentConfig<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = {
  [BUILD](): NS.DependentField<K, Input, Output, CtxOptions>;
} & (HasReadonly extends true
  ? {}
  : {
      readonly(): BuildableDependentConfig<
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
        ): BuildableDependentConfig<
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
        ): BuildableDependentConfig<
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
    BlankDependentBuilder<K, Input, Output, CtxOptions>,
    HasDefault<K, Input, Output, CtxOptions>,
    HasDependsOn<K, Input, Output, CtxOptions>,
    ReadyToResolve<K, Input, Output, CtxOptions>,
    BuildableDependentConfig<K, Input, Output, CtxOptions>
{
  private config: Partial<NS.DependentField<K, Input, Output, CtxOptions>> = {};

  default(
    value: TypeOf<Output[K]> | NS.Setter<Output[K], Input, Output, CtxOptions>,
  ) {
    this.config.default = value;
    return this as never;
  }

  dependsOn(
    deps:
      | NS.Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<NS.Dependables<K, Input, Output>>,
  ) {
    this.config.dependsOn = deps;
    return this as never;
  }

  resolve(resolver: NS.Resolver<K, Input, Output, CtxOptions>) {
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
    return this.config as NS.DependentField<K, Input, Output, CtxOptions>;
  }
}
