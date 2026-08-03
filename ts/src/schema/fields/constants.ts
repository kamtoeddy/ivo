import type { ObjectType } from "../../utils";
import {
  type ArrayOfMinSizeOne,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type Buildable,
  type NS,
} from "../types";

export { ConstantBuilder };

type BuildableConstantConfig<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnSuccess extends boolean = false,
> = Buildable<NS.ConstantField<Value, Input, Output, CtxOptions>> &
  (HasOnDelete extends true
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
> implements BuildableConstantConfig<Value, Input, Output, CtxOptions> {
  private config: Partial<NS.ConstantField<Value, Input, Output, CtxOptions>> =
    { type: "constant" };

  constructor(
    name: string,
    value: Value | NS.ConstantResolver<Value, Input, Output, CtxOptions>,
  ) {
    this.config.name = name;
    this.config.value = value;
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
    return this.config as NS.ConstantField<Value, Input, Output, CtxOptions>;
  }
}
