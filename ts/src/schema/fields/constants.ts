import type { ObjectType } from '../../utils';
import { type ArrayOfMinSizeOne, BUILD, type NS, type TypeOf } from '../types';

export { type BlankConstantBuilder, ConstantBuilder };

interface BlankConstantBuilder<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  value(
    value: TypeOf<Output[K]> | NS.Setter<Output[K], Input, Output, CtxOptions>,
  ): BuildableConstantConfig<K, Input, Output, CtxOptions>;
}

type BuildableConstantConfig<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = {
  [BUILD](): NS.ConstantField<K, Input, Output, CtxOptions>;
} & (HasOnDelete extends true
  ? {}
  : {
      onDelete(
        handler:
          | NS.DeleteHandler<Output, CtxOptions>
          | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>,
      ): BuildableConstantConfig<
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
        ): BuildableConstantConfig<
          K,
          Input,
          Output,
          CtxOptions,
          HasReadonly,
          HasOnDelete,
          true
        >;
      });

class ConstantBuilder<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> implements
    BlankConstantBuilder<K, Input, Output, CtxOptions>,
    BuildableConstantConfig<K, Input, Output, CtxOptions>
{
  private config: Partial<NS.ConstantField<K, Input, Output, CtxOptions>> = {
    constant: true,
  };

  value(
    value: TypeOf<Output[K]> | NS.Setter<Output[K], Input, Output, CtxOptions>,
  ) {
    this.config.value = value;
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
    return this.config as NS.ConstantField<K, Input, Output, CtxOptions>;
  }
}
