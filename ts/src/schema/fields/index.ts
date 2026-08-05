import {
  ArrayOfMinSizeOne,
  NS,
  DefaultFieldErrorMetadata,
  ObjectType,
} from "../../utils/types";
import { ConstantBuilder } from "./constants";
import { type HasDependsOn, DependentBuilder } from "./dependents";
import { type BlankLaxBuilder, LaxBuilder } from "./lax";
import { type BlankRequiredBuilder, RequiredBuilder } from "./required";
import { type BlankVirtualBuilder, VirtualBuilder } from "./virtual";

export { newFieldMaker };

function newFieldMaker<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
>() {
  return {
    constant<K extends keyof Output & string>(
      name: K,
      value:
        | Output[K]
        | NS.ConstantResolver<Output[K], Input, Output, CtxOptions>,
    ): ConstantBuilder<Output[K], Input, Output, CtxOptions> {
      return new ConstantBuilder<Output[K], Input, Output, CtxOptions>(
        name,
        value,
      );
    },
    dependent<K extends keyof Output & string>(
      name: K,
      dependsOn:
        | NS.Dependables<K, Input, Output>
        | ArrayOfMinSizeOne<NS.Dependables<K, Input, Output>>,
    ): HasDependsOn<K, Input, Output, CtxOptions> {
      return new DependentBuilder<K, Input, Output, CtxOptions>(
        name,
        dependsOn,
      );
    },
    lax<K extends keyof Output & string>(
      name: K,
    ): BlankLaxBuilder<Output[K], Input, Output, CtxOptions, Metadata> {
      return new LaxBuilder<
        (Input & Output)[K],
        Input,
        Output,
        CtxOptions,
        Metadata
      >(name);
    },
    required<K extends keyof Output & string>(
      name: K,
    ): BlankRequiredBuilder<Output[K], Input, Output, CtxOptions, Metadata> {
      return new RequiredBuilder<
        Output[K],
        Input,
        Output,
        CtxOptions,
        Metadata
      >(name);
    },
    virtual<K extends string>(
      name: K,
    ): BlankVirtualBuilder<any, Input, Output, CtxOptions, Metadata> {
      return new VirtualBuilder<any, Input, Output, CtxOptions, Metadata>(name);
    },
  };
}
