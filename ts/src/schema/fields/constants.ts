import type { ObjectType } from '../../utils';
import { type ArrayOfMinSizeOne, BUILD, type NS } from '../types';

export { type BlankConstantBuilder, ConstantBuilder };

interface BlankConstantBuilder<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
> {
  value(
    value: Value | NS.Resolver<Value, Input, Output, CtxOptions>,
  ): BuildableConstantConfig<Value, Input, Output, CtxOptions>;
}

type BuildableConstantConfig<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = {
  [BUILD](): NS.ConstantField<Value, Input, Output, CtxOptions>;
} & (HasOnDelete extends true
  ? {}
  : {
      onDelete(
        handler:
          | NS.DeleteHandler<Output, CtxOptions>
          | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>,
      ): BuildableConstantConfig<
        Value,
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
          Value,
          Input,
          Output,
          CtxOptions,
          HasReadonly,
          HasOnDelete,
          true
        >;
      });

class ConstantBuilder<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
> implements
    BlankConstantBuilder<Value, Input, Output, CtxOptions>,
    BuildableConstantConfig<Value, Input, Output, CtxOptions>
{
  private config: Partial<NS.ConstantField<Value, Input, Output, CtxOptions>> =
    {
      type: 'constant',
    };

  value(value: Value | NS.Resolver<Value, Input, Output, CtxOptions>) {
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
    return this.config as NS.ConstantField<Value, Input, Output, CtxOptions>;
  }
}
