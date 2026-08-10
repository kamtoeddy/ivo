import {
  type ArrayOfMinSizeOne,
  type Buildable,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type NS,
  type ObjectType,
  type TypeOf,
} from '../../utils/types';

export { DependentBuilder, type HasDependsOn };

interface HasDependsOn<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  default<
    Default extends
      | TypeOf<Output[K]>
      | NS.Resolver<Output[K], Input, Output, CtxOptions>,
    DefaultState extends 'value' | 'resolver' = Default extends Function
      ? 'resolver'
      : 'value',
  >(v: Default): HasDefault<K, Input, Output, CtxOptions, DefaultState>;
}

interface HasDefault<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
  DefaultState extends 'value' | 'resolver' = 'resolver',
> {
  resolve(
    resolver: NS.Resolver<Output[K], Input, Output, CtxOptions>,
  ): BuildableDependentConfig<K, Input, Output, CtxOptions, DefaultState>;
}

type BuildableDependentConfig<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
  DefaultState extends 'value' | 'resolver' = 'resolver',
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = Buildable<NS.DependentField<K, Input, Output, CtxOptions>> &
  (DefaultState extends 'resolver'
    ? {}
    : HasReadonly extends true
      ? {}
      : {
          readonly(): BuildableDependentConfig<
            K,
            Input,
            Output,
            CtxOptions,
            DefaultState,
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
          DefaultState,
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
          DefaultState,
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
    HasDependsOn<K, Input, Output, CtxOptions>,
    HasDefault<K, Input, Output, CtxOptions>,
    BuildableDependentConfig<K, Input, Output, CtxOptions>
{
  private config: Partial<NS.DependentField<K, Input, Output, CtxOptions>> = {
    type: 'dependent',
  };

  constructor(
    name: string,
    dependsOn:
      | NS.Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<NS.Dependables<K, Input, Output>>,
  ) {
    this.config.name = name;
    // @ts-expect-error ikr
    this.config.dependsOn = Array.isArray(dependsOn) ? dependsOn : [dependsOn];
  }

  default(
    v: TypeOf<Output[K]> | NS.Resolver<Output[K], Input, Output, CtxOptions>,
  ) {
    this.config.default = v;
    return this as never;
  }

  resolve(resolver: NS.Resolver<Output[K], Input, Output, CtxOptions>) {
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

  [FIELD_CONFIG_BUILD_METHOD_NAME]() {
    return this.config as NS.DependentField<K, Input, Output, CtxOptions>;
  }
}
