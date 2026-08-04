import { getKeysAsProps, toArray } from "../utils";
import { newFieldMaker } from "./fields";

import { defaultOptions, SchemaCore } from "./schema-core";
import {
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type Buildable,
  type IvoErrorPayload,
  type KeyOf,
  LIFE_CYCLES,
  type NS,
  type RealType,
  ObjectType,
  DefaultFieldErrorMetadata,
} from "../utils/types";

export { type FieldMaker, Schema };

class Schema<
  const Input extends RealType<Input>,
  const Output extends RealType<Output> = Input,
  const CtxOptions extends ObjectType = {},
  const ErrorMetadata = DefaultFieldErrorMetadata,
  const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
> extends SchemaCore<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload> {
  constructor(
    builder: (
      b: FieldBuilder<Input, Output, CtxOptions, ErrorMetadata>,
      m: FieldMaker<Input, Output, CtxOptions, ErrorMetadata>,
    ) => FieldBuilder<Input, Output, CtxOptions, ErrorMetadata>,
    options: NS.Options<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    > = defaultOptions as never,
  ) {
    super(
      builder(new FieldBuilder(), newFieldMaker())[FIELD_BUILDER_DEFINITIONS],
      options as never,
    );
  }

  get definitions() {
    return this._definitions as never as NS.Definitions<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >;
  }

  get options() {
    return this._options;
  }

  extend<
    const ExtendedInput extends RealType<ExtendedInput>,
    const ExtendedOutput extends RealType<ExtendedOutput> = ExtendedInput,
    const ExtendedCtxOptions extends ObjectType = CtxOptions,
    const ExtendedErrorMetadata = ErrorMetadata,
    const ExtendedErrorPayload = IvoErrorPayload<
      ExtendedErrorMetadata,
      KeyOf<ExtendedInput>
    >,
  >(
    builder: (
      b: FieldBuilder<
        ExtendedInput,
        ExtendedOutput,
        ExtendedCtxOptions,
        ExtendedErrorMetadata
      >,
      m: FieldMaker<
        ExtendedInput,
        ExtendedOutput,
        ExtendedCtxOptions,
        ExtendedErrorMetadata
      >,
    ) => FieldBuilder<
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata
    >,
    options: NS.ExtensionOptions<
      Input,
      Output,
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    > = {},
  ) {
    const { remove = [], useParentOptions = true, ...rest } = options;

    const _definitions = {
      ...this.definitions,
    } as unknown as NS.Definitions_<
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata
    >;

    toArray(remove ?? [])?.forEach(
      (prop) => delete (_definitions as never)?.[prop],
    );

    const options_ = {} as NS.Options<
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    >;

    if (useParentOptions)
      getKeysAsProps(this.options)
        .filter(
          (prop) => ![...LIFE_CYCLES, "shouldUpdate"].includes(prop as never),
        )
        .forEach((prop) => {
          options_[prop] = this.options[prop] as never;
        });

    return new Schema<
      ExtendedInput,
      ExtendedOutput,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    >(
      () => builder(new FieldBuilder(_definitions), newFieldMaker()),
      Object.assign({}, options_, rest),
    );
  }

  // getModel() {
  //   return new Model(
  //     () =>
  //       new ModelTool<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload>(
  //         this,
  //       ),
  //   );
  // }
}

const FIELD_BUILDER_DEFINITIONS: unique symbol = Symbol(
  "ivo-schema-field-builder",
);

function isBuildable(value: unknown): value is Buildable<unknown> {
  return (
    typeof value === "object" &&
    !!value &&
    typeof (value as { [FIELD_CONFIG_BUILD_METHOD_NAME]?: unknown })[
      FIELD_CONFIG_BUILD_METHOD_NAME
    ] === "function"
  );
}

class FieldBuilder<
  const I extends RealType<I>,
  const O extends RealType<O> = I,
  const CtxOptions extends ObjectType = {},
  const ErrorMetadata = DefaultFieldErrorMetadata,
> {
  private _definitions: NS.Definitions_<I, O, CtxOptions, ErrorMetadata>;

  // Optional seed, used exclusively by `Schema.extend()` to carry a parent
  // schema's already-materialized definitions (real field configs, not
  // `Buildable`s) into a fresh builder before the extension closure runs -
  // `field()` itself stays `Buildable`-only, so this is the only legitimate
  // way to fold pre-built definitions back into the builder pattern.
  constructor(seed: NS.Definitions_<I, O, CtxOptions, ErrorMetadata> = {}) {
    this._definitions = seed;
  }

  field<K extends keyof I | keyof O>(
    c: NS.FieldDefinition<K, I, O, CtxOptions, ErrorMetadata>,
  ) {
    if (isBuildable(c)) {
      const built = c[FIELD_CONFIG_BUILD_METHOD_NAME]();

      if (typeof built.name === "string") this._definitions[built.name] = built;
    }

    return this;
  }

  get [FIELD_BUILDER_DEFINITIONS](): NS.Definitions_<
    I,
    O,
    CtxOptions,
    ErrorMetadata
  > {
    return this._definitions;
  }
}

type FieldMaker<
  Input extends RealType<Input>,
  Output extends RealType<Output> = Input,
  CtxOptions extends ObjectType = {},
  ErrorMetadata = DefaultFieldErrorMetadata,
> = ReturnType<typeof newFieldMaker<Input, Output, CtxOptions, ErrorMetadata>>;
