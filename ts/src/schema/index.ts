import {
  getKeysAsProps,
  isOneOf,
  isRecordLike,
  SchemaErrorTool,
  toArray,
} from "../utils";

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
  ArrayOfMinSizeOne,
  PostValidationConfig,
  TimeStampTool,
} from "../utils/types";
import { Model, ModelTool } from "../model";
import { ConstantBuilder } from "./fields/constants";
import { DependentBuilder, HasDependsOn } from "./fields/dependents";
import { BuildableLaxConfig, LaxBuilder } from "./fields/lax";
import { BlankRequiredBuilder, RequiredBuilder } from "./fields/required";
import { BlankVirtualBuilder, VirtualBuilder } from "./fields/virtual";

export { Schema };

class Schema<
  const I extends RealType<I>,
  const O extends RealType<O> = I,
  const CtxOptions extends ObjectType = {},
  const ErrorMetadata = DefaultFieldErrorMetadata,
  const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
> {
  private _definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>;
  private _options: NS.InternalOptions<
    I,
    O,
    CtxOptions,
    ErrorMetadata,
    ErrorPayload
  >;

  constructor(
    builder: (
      f: FieldBuilder<I, O, CtxOptions, ErrorMetadata>,
    ) => FieldBuilder<I, O, CtxOptions, ErrorMetadata>,
    options: NS.Options<I, O, CtxOptions, ErrorMetadata, ErrorPayload> = {},
  ) {
    const definitions = builder(new FieldBuilder())[FIELD_BUILDER_DEFINITIONS];

    const errorTool = new SchemaErrorTool();

    const aliasToVirtualMap = validateFields<I, O, CtxOptions, ErrorMetadata>(
      definitions,
      errorTool,
    );

    this._definitions = definitions;
    this._options = makeOptions<I, O, CtxOptions, ErrorMetadata, ErrorPayload>({
      aliasToVirtualMap,
      definitions,
      errorTool,
      options,
    });
  }

  get definitions() {
    return this._definitions as never as NS.Definitions<
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >;
  }

  get options() {
    return this._options;
  }

  extend<
    const ExtendedI extends RealType<ExtendedI>,
    const ExtendedO extends RealType<ExtendedO> = ExtendedI,
    const ExtendedCtxOptions extends ObjectType = CtxOptions,
    const ExtendedErrorMetadata = ErrorMetadata,
    const ExtendedErrorPayload = IvoErrorPayload<
      ExtendedErrorMetadata,
      KeyOf<ExtendedI>
    >,
  >(
    builder: (
      b: FieldBuilder<
        ExtendedI,
        ExtendedO,
        ExtendedCtxOptions,
        ExtendedErrorMetadata
      >,
    ) => FieldBuilder<
      ExtendedI,
      ExtendedO,
      ExtendedCtxOptions,
      ExtendedErrorMetadata
    >,
    options: NS.ExtensionOptions<
      I,
      O,
      ExtendedI,
      ExtendedO,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    > = {},
  ) {
    const { remove = [], useParentOptions = true, ...rest } = options;

    const _definitions = {
      ...this.definitions,
    } as unknown as NS.Definitions<
      ExtendedI,
      ExtendedO,
      ExtendedCtxOptions,
      ExtendedErrorMetadata
    >;

    toArray(remove ?? [])?.forEach(
      (prop) => delete (_definitions as never)?.[prop],
    );

    const options_ = {} as NS.Options<
      ExtendedI,
      ExtendedO,
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
      ExtendedI,
      ExtendedO,
      ExtendedCtxOptions,
      ExtendedErrorMetadata,
      ExtendedErrorPayload
    >(
      () => builder(new FieldBuilder(_definitions)),
      Object.assign({}, options_, rest),
    );
  }

  getModel() {
    return new Model<I, O, CtxOptions, ErrorMetadata, ErrorPayload>(
      () => new ModelTool(this.definitions, this.options),
    );
  }
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
  private _definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>;

  // Optional seed, used exclusively by `Schema.extend()` to carry a parent
  // schema's already-materialized definitions (real field configs, not
  // `Buildable`s) into a fresh builder before the extension closure runs -
  // `field()` itself stays `Buildable`-only, so this is the only legitimate
  // way to fold pre-built definitions back into the builder pattern.
  constructor(seed: NS.Definitions<I, O, CtxOptions, ErrorMetadata> = {}) {
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

  constant<K extends keyof O & string>(
    name: K,
    value: O[K] | NS.ConstantResolver<O[K], I, O, CtxOptions>,
  ): ConstantBuilder<O[K], I, O, CtxOptions> {
    return new ConstantBuilder<O[K], I, O, CtxOptions>(name, value);
  }

  dependent<K extends keyof O & string>(
    name: K,
    dependsOn:
      | NS.Dependables<K, I, O>
      | ArrayOfMinSizeOne<NS.Dependables<K, I, O>>,
  ): HasDependsOn<K, I, O, CtxOptions> {
    return new DependentBuilder<K, I, O, CtxOptions>(name, dependsOn);
  }

  lax<
    K extends keyof O & string,
    Default extends O[K] | NS.Resolver<O[K], I, O, CtxOptions>,
    DefaultState extends "value" | "resolver" = Default extends Function
      ? "resolver"
      : "value",
  >(
    name: K,
    value: Default,
  ): BuildableLaxConfig<O[K], I, O, CtxOptions, ErrorMetadata, DefaultState> {
    // @ts-expect-error ikr
    return new LaxBuilder<O[K], I, O, CtxOptions, ErrorMetadata>(name, value);
  }

  required<K extends keyof O & string>(
    name: K,
  ): BlankRequiredBuilder<O[K], I, O, CtxOptions, ErrorMetadata> {
    return new RequiredBuilder<O[K], I, O, CtxOptions, ErrorMetadata>(name);
  }

  virtual<K extends string>(
    name: K,
  ): BlankVirtualBuilder<any, I, O, CtxOptions, ErrorMetadata> {
    return new VirtualBuilder<any, I, O, CtxOptions, ErrorMetadata>(name);
  }

  get [FIELD_BUILDER_DEFINITIONS](): NS.Definitions<
    I,
    O,
    CtxOptions,
    ErrorMetadata
  > {
    return this._definitions;
  }
}

function validateFields<
  I extends RealType<I>,
  O extends RealType<O>,
  CtxOptions extends ObjectType,
  const ErrorMetadata = DefaultFieldErrorMetadata,
>(
  definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>,
  errorTool: SchemaErrorTool,
): Map<string, string> {
  const constantFieldNames = new Set<string>();
  const dependentFieldToParentFields = new Map<
    string,
    ArrayOfMinSizeOne<NS.Dependables<any, I, O>>
  >();
  const fieldNames = new Set<string>();
  const aliasToVirtualMap = new Map<string, string>();
  const dependentConfigs: Array<
    [string, NS.DependentField<any, I, O, CtxOptions>]
  > = [];

  const definitionEntries = Object.entries(definitions);

  for (const [fieldName, config] of definitionEntries) {
    if (fieldNames.has(fieldName)) {
      throw new Error(
        `\n[${fieldName}]: occurs more than once, please remove duplicatesn`,
      );
    }

    fieldNames.add(fieldName);

    if (config.type === "constant") {
      constantFieldNames.add(fieldName);
      continue;
    }

    if (config.type === "dependent") {
      const { dependsOn } = config;
      let hasErrors = false;

      if (typeof config.default === "undefined") {
        errorTool.add(
          fieldName,
          "Dependent fields must have a default value or default resolver",
        );

        hasErrors = true;
      }

      if (typeof config.resolver !== "function") {
        errorTool.add(fieldName, "Dependent fields must have a value resolver");

        hasErrors = true;
      }

      if (!dependsOn?.length) {
        errorTool.add(
          fieldName,
          "Dependent fields must depend on at least one lax, required, virtual or other dependent field on your schema",
        );
        hasErrors = true;
      }

      // @ts-expect-error
      if (dependsOn.includes(fieldName)) {
        errorTool.add(fieldName, "A field cannot depend on itself");
        hasErrors = true;
      }

      if (!hasErrors) {
        dependentConfigs.push([fieldName, config]);
        dependentFieldToParentFields.set(fieldName, dependsOn);
      }

      continue;
    }

    if (config.type === "lax") {
      if (typeof config.default === "undefined")
        errorTool.add(
          fieldName,
          "Lax fields must have a default value or default resolver",
        );

      continue;
    }

    if (config.type === "required") {
      if (!config.allow && !config.validator)
        errorTool.add(fieldName, "Required fields must have a validator");

      continue;
    }

    const { alias } = config;

    if (alias) {
      if (fieldName === alias) {
        errorTool.add(
          fieldName,
          "virtual alias name must be different from field name",
        );
        continue;
      }

      const otherField = aliasToVirtualMap.get(alias);

      if (otherField != null) {
        errorTool.add(
          fieldName,
          `"${alias}" is already the alias of "${otherField}"`,
        );
        continue;
      }

      for (const [name, cfg] of definitionEntries) {
        if (name !== alias) continue;

        if (cfg.type === "dependent") {
          // @ts-expect-error ikr
          if (!cfg.dependsOn.includes(fieldName))
            errorTool.add(
              fieldName,
              `"${alias}" is not a valid alias for field because "${alias}" does not depend on "${fieldName}"`,
            );

          continue;
        }

        errorTool.add(
          fieldName,
          `"${alias}" is not a valid alias for field because it is not a dependent field`,
        );
      }

      aliasToVirtualMap.set(alias, fieldName);
      continue;
    }

    let hasSufficientDependencies = false;

    for (const [, cfg] of definitionEntries)
      // @ts-expect-error ikr
      if (cfg.type === "dependent" && cfg.dependsOn.includes(fieldName)) {
        hasSufficientDependencies = true;
        break;
      }

    if (!hasSufficientDependencies)
      errorTool.add(
        fieldName,
        "Virtual fields are expected to have at least one dependency, but found none",
      );
  }

  for (const [fieldName, config] of dependentConfigs) {
    const parentFields = config.dependsOn!;

    const parentFieldsProvided = new Set<string>();

    for (const parentField of parentFields) {
      if (!fieldNames.has(parentField))
        errorTool.add(
          fieldName,
          `cannot depend on "${parentField}" because it is not a field on your schema`,
        );

      if (parentFieldsProvided.has(parentField))
        errorTool.add(
          fieldName,
          `"${parentField}" has been provided as a parent field multiple times. remove all duplicates to proceed`,
        );

      if (constantFieldNames.has(parentField))
        errorTool.add(
          fieldName,
          `cannot depend on "${parentField}" because it is a constant`,
        );

      parentFieldsProvided.add(parentField);
    }

    const redundant = getRedundantDependency(
      parentFields,
      dependentFieldToParentFields,
    );

    if (redundant) {
      const [parentField, redundantField, depth] = redundant;

      if (depth === 0) {
        errorTool.add(
          fieldName,
          `should not depend on "${parentField}" and "${redundantField}" because "${parentField}" depends on "${redundantField}"`,
        );
        continue;
      }

      errorTool.add(
        fieldName,
        `should not depend on "${parentField}" and "${redundantField}" because "${parentField}" indirectly depends on "${redundantField}"`,
      );
    }

    const circularChain = getCircularDependencyChain(
      fieldName,
      parentFields,
      dependentFieldToParentFields,
    );

    if (circularChain != null)
      errorTool.add(
        fieldName,
        `circular dependency identified between "${circularChain.sort().join(" <-> ")}"`,
      );
  }

  if (errorTool.isPayloadLoaded) errorTool.throw();

  if (!fieldNames.size)
    errorTool.add("schema fields", "Insufficient Schema fields").throw();

  return aliasToVirtualMap;
}

function makeOptions<
  I extends RealType<I>,
  O extends RealType<O>,
  CtxOptions extends ObjectType,
  const ErrorMetadata = DefaultFieldErrorMetadata,
  const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
>({
  aliasToVirtualMap,
  definitions,
  errorTool,
  options,
}: {
  aliasToVirtualMap: Map<string, string>;
  definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>;
  errorTool: SchemaErrorTool;
  options: NS.Options<I, O, CtxOptions, ErrorMetadata, ErrorPayload>;
}): NS.InternalOptions<I, O, CtxOptions, ErrorMetadata, ErrorPayload> {
  let sanitizeError = (p: IvoErrorPayload<ErrorMetadata, KeyOf<I>>) => p;

  if (typeof options.sanitizeError === "function") {
    // @ts-expect-error ikr
    sanitizeError = options.sanitizeError;
  }

  const normalizedOptions: NS.InternalOptions<
    I,
    O,
    CtxOptions,
    ErrorMetadata,
    ErrorPayload
  > = {
    equalityDepth: options.equalityDepth ?? 1,
    // @ts-expect-error ikr
    sanitizeError,
  };

  if (options.ignore) {
    const ignore: NS.IgnoreConfigOptionItem<I, O, CtxOptions>[] = [];
    let hasErrors = false;

    if (typeof options.ignore === "function") ignore.push(options.ignore);
    else {
      const optionName = "options.ignore";
      const type_not_allowed_error =
        "only lax and virtual fields can belong to grouped ignore configs";

      for (const config of toArray(options.ignore)) {
        if (typeof config === "function") {
          ignore.push(config);

          continue;
        }

        const { fields, resolver } = config;

        ignore.push({ fields, resolver });

        if (fields.length < 2)
          errorTool.add(optionName, "grouped ignore expects at least 2 fields");

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName)) {
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped ignore config`,
            );
            hasErrors = true;
          }

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField) {
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );
            hasErrors = true;
          }

          if (!isOneOf(definitions[fieldName]?.type, ["lax", "virtual"])) {
            errorTool.add(
              optionName,
              `${type_not_allowed_error} remove "${fieldName}"`,
            );
            hasErrors = true;
          }
        }
      }
    }

    if (!hasErrors) normalizedOptions.ignore = ignore;
  }

  if (options.ignoreUpdate) {
    const ignoreUpdate: NS.IgnoreUpdateConfigOptionItem<I, O, CtxOptions>[] =
      [];
    let hasErrors = false;

    if (typeof options.ignoreUpdate === "function")
      ignoreUpdate.push(options.ignoreUpdate);
    else {
      const optionName = "options.ignoreUpdate";
      const type_not_allowed_error =
        "only lax, required and virtual fields can belong to grouped ignore update configs";

      for (const config of toArray(options.ignoreUpdate)) {
        if (typeof config === "function") {
          ignoreUpdate.push(config);

          continue;
        }

        const { fields, resolver } = config;

        ignoreUpdate.push({ fields, resolver });

        if (fields.length < 2) {
          errorTool.add(
            optionName,
            "grouped ignore update expects at least 2 fields",
          );
          hasErrors = true;
        }

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName)) {
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped ignore update config`,
            );
            hasErrors = true;
          }

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField) {
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );
            hasErrors = true;
          }

          if (
            !isOneOf(definitions[fieldName]?.type, [
              "lax",
              "required",
              "virtual",
            ])
          ) {
            errorTool.add(
              optionName,
              `${type_not_allowed_error} remove "${fieldName}"`,
            );
            hasErrors = true;
          }
        }

        if (!hasErrors) normalizedOptions.ignoreUpdate = ignoreUpdate;
      }
    }
  }

  if (options.onSuccess) {
    const onSuccess: NS.OnSuccessConfigOptionItem<I, O, CtxOptions>[] = [];
    let hasErrors = false;

    if (typeof options.onSuccess === "function")
      onSuccess.push(options.onSuccess);
    else {
      const optionName = "options.on_success";

      for (const config of toArray(options.onSuccess)) {
        if (typeof config === "function") {
          onSuccess.push(config);

          continue;
        }

        const { fields, resolver } = config;

        onSuccess.push({ fields, resolver });

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName)) {
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped on success config`,
            );
            hasErrors = true;
          }

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField) {
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );
            hasErrors = true;
          }

          if (!definitions[fieldName]) {
            errorTool.add(
              optionName,
              `"${fieldName}" does not exist on your schema`,
            );
            hasErrors = true;
          }
        }
      }
    }

    if (!hasErrors) normalizedOptions.onSuccess = onSuccess;
  }

  if (options.postValidate) {
    const optionName = "options.postValidate";
    const type_not_allowed_error =
      "only lax, required and virtual fields can be post-validated";

    const postValidate: PostValidationConfig<
      any,
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >[] = [];
    let hasErrors = false;

    for (const { fields, validator } of toArray(options.postValidate)) {
      postValidate.push({ fields, validator });

      if (fields.length < 2)
        errorTool.add(
          optionName,
          "post-validation config expects at least 2 I fields",
        );

      const fieldNames = new Set<string>();

      for (const fieldName of fields) {
        if (fieldNames.has(fieldName))
          errorTool.add(
            optionName,
            `remove duplicates of "${fieldName}" in your grouped post-validation config`,
          );

        const virtualField = aliasToVirtualMap.get(fieldName);

        if (virtualField)
          errorTool.add(
            optionName,
            `"${fieldName}" is an alias; use "${virtualField}" instead`,
          );

        if (
          !isOneOf(definitions[fieldName]?.type, ["lax", "required", "virtual"])
        )
          errorTool.add(
            optionName,
            `${type_not_allowed_error} remove "${fieldName}"`,
          );
      }
    }

    if (!hasErrors) normalizedOptions.postValidate = postValidate;
  }

  if (options.required) {
    const optionName = "options.required";
    const type_not_allowed_error =
      "only lax and virtual fields can belong to grouped required configs";
    const required: NS.RequiredConfigObject<I, O, CtxOptions, ErrorMetadata>[] =
      [];
    let hasErrors = false;

    for (const { fields, handler } of toArray(options.required)) {
      required.push({ fields, handler });

      if (fields.length < 2)
        errorTool.add(
          optionName,
          "grouped required config expects at least 2 I fields",
        );

      const fieldNames = new Set<string>();

      for (const fieldName of fields) {
        if (fieldNames.has(fieldName))
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped required config`,
            );

        const virtualField = aliasToVirtualMap.get(fieldName);

        if (virtualField)
          errorTool.add(
            optionName,
            `"${fieldName}" is an alias; use "${virtualField}" instead`,
          );

        if (!isOneOf(definitions[fieldName]?.type, ["lax", "virtual"]))
          errorTool.add(
            optionName,
            `${type_not_allowed_error} remove "${fieldName}"`,
          );
      }
    }

    if (!hasErrors) normalizedOptions.required = required;
  }

  if (options.timestamps) {
    const isValid = isTimestampsConfigOptionOk(options.timestamps);

    if (!isValid.valid) errorTool.add("options.timestamps", isValid.reason);
    else normalizedOptions.timestamps = new TimeStampTool(options.timestamps);
  }

  if (errorTool.isPayloadLoaded) errorTool.throw();

  return normalizedOptions as any;
}

function isTimestampsConfigOptionOk<I, O>(
  timestamps: NS.Options<I, O>["timestamps"],
) {
  const valid = false;

  const typeProveded = typeof timestamps;

  if (typeProveded === "boolean") return { valid: true };

  if (!isRecordLike(timestamps))
    return { valid, reason: "should be 'boolean' or 'non null object'" };

  if (!Object.keys(timestamps!).length)
    return { valid, reason: "cannot be an empty object" };

  const createdAt = timestamps.createdAt as string;
  let updatedAt = timestamps.updatedAt as string;

  if (typeof createdAt === "string" && !createdAt.trim().length)
    return { valid, reason: "'createdAt' cannot be an empty string" };

  if (typeof updatedAt === "string" && !updatedAt.trim().length)
    return { valid, reason: "'updatedAt' cannot be an empty string" };

  if (typeof timestamps.updatedAt === "object") {
    const updatedAtConfig = timestamps.updatedAt;

    const keys = Object.keys(updatedAtConfig).filter((prop) =>
      isOneOf(prop, ["key", "nullable"]),
    );

    if (!keys.length)
      return {
        valid,
        reason: "'updatedAt' can only accept fields 'key' and 'nullable'",
      };

    if (keys.includes("key")) {
      updatedAt = updatedAtConfig.key!;

      if (typeof updatedAt !== "string" || !updatedAt.trim().length)
        return { valid, reason: "'updatedAt.key' must be a valid string" };
    }

    if (
      keys.includes("nullable") &&
      typeof updatedAtConfig.nullable !== "boolean"
    )
      return {
        valid,
        reason: "'updatedAt.nullable' must be a boolean",
      };
  }

  if (createdAt === updatedAt)
    return { valid, reason: "createdAt & updatedAt cannot be same" };

  return { valid: true };
}

function getRedundantDependency(
  parentFields: ArrayOfMinSizeOne<NS.Dependables<any, any, any>>,
  dependentFieldToParentFields: Map<
    string,
    ArrayOfMinSizeOne<NS.Dependables<any, any, any>>
  >,
): [string, string, number] | null {
  for (const parentName of parentFields) {
    for (const fieldName of parentFields) {
      if (fieldName === parentName) continue;

      const res = isFieldRedundantlyDependentOnParent(
        fieldName,
        parentName,
        dependentFieldToParentFields,
        0,
      );

      if (res) return [fieldName, ...res];
    }
  }

  return null;
}

function isFieldRedundantlyDependentOnParent(
  fieldName: string,
  parentName: string,
  dependentFieldToParentFields: Map<
    string,
    ArrayOfMinSizeOne<NS.Dependables<any, any, any>>
  >,
  depth: number,
): [string, number] | null {
  const parentDeps = dependentFieldToParentFields.get(fieldName);

  if (parentDeps) {
    // @ts-expect-error ikr
    if (parentDeps.includes(parentName)) return [parentName, depth];

    for (const next_fieldName of parentDeps) {
      const r = isFieldRedundantlyDependentOnParent(
        next_fieldName,
        parentName,
        dependentFieldToParentFields,
        depth + 1,
      );

      if (r) return r;
    }

    return null;
  }

  return null;
}

function getCircularDependencyChain(
  dependentFieldName: string,
  parentFields: ArrayOfMinSizeOne<NS.Dependables<any, any, any>>,
  dependentFieldToParentFields: Map<
    string,
    ArrayOfMinSizeOne<NS.Dependables<any, any, any>>
  >,
): string[] | null {
  for (const parentName of parentFields) {
    const chain = isFieldCircularlyDependentOnParent(
      dependentFieldName,
      parentName,
      dependentFieldToParentFields,
      [dependentFieldName],
    );

    if (chain) return chain;
  }

  return null;
}

function isFieldCircularlyDependentOnParent(
  dependentFieldName: string,
  parentName: string,
  dependentFieldToParentFields: Map<
    string,
    ArrayOfMinSizeOne<NS.Dependables<any, any, any>>
  >,
  visited_nodes: string[],
): string[] | null {
  const parentDeps = dependentFieldToParentFields.get(parentName);
  if (parentDeps) {
    const next_visited = [...visited_nodes, parentName];

    // @ts-expect-error ikr
    if (parentDeps.includes(dependentFieldName)) return next_visited;

    for (const fieldName of parentDeps) {
      const r = isFieldCircularlyDependentOnParent(
        dependentFieldName,
        fieldName,
        dependentFieldToParentFields,
        next_visited,
      );

      if (r) return r;
    }

    return null;
  }

  return null;
}
