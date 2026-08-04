import {
  type ArrayOfMinSizeOne,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type Buildable,
  type NS,
  type TypeOf,
  ObjectType,
} from "../../utils/types";

export { type HasDependsOn, DependentBuilder };

interface HasDependsOn<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  default(
    v: TypeOf<Output[K]> | NS.Resolver<Output[K], Input, Output, CtxOptions>,
  ): HasDefault<K, Input, Output, CtxOptions>;
}

interface HasDefault<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  resolve(
    resolver: NS.Resolver<Output[K], Input, Output, CtxOptions>,
  ): BuildableDependentConfig<K, Input, Output, CtxOptions>;
}

type BuildableDependentConfig<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = Buildable<NS.DependentField<K, Input, Output, CtxOptions>> &
  (HasReadonly extends true
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
>
  implements
    HasDependsOn<K, Input, Output, CtxOptions>,
    HasDefault<K, Input, Output, CtxOptions>,
    BuildableDependentConfig<K, Input, Output, CtxOptions>
{
  private config: Partial<NS.DependentField<K, Input, Output, CtxOptions>> = {
    type: "dependent",
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
