import { isOneOf, SchemaErrorTool, toArray } from "../utils";
import {
  ArrayOfMinSizeOne,
  DefaultFieldErrorMetadata,
  IvoErrorPayload,
  KeyOf,
  NS,
  ObjectType,
} from "../utils/types";

export type FieldValue = unknown;

export type Yes = { readonly __brand: "Yes" };
export type No = { readonly __brand: "No" };

const STYLE_COLOR_RED = "\x1b[31m";
const STYLE_RESET = "\x1b[0m";

export enum FieldType {
  Constant = "Constant",
  Virtual = "Virtual",
  Dependent = "Dependent",
  Lax = "Lax",
  Required = "Required",
}

export interface IvoInputStruct<
  CtxOptions = unknown,
  ErrorSanitizer = unknown,
> {
  _ctx?: CtxOptions;
  _err?: ErrorSanitizer;
}

export interface IvoInputStructMeta {
  ivo_internal_fieldNames(): Set<string>;
  ivo_internal_name(): string;
}

export interface IvoStruct {
  [key: string]: unknown;
}

export interface IvoStructMeta {
  ivo_internal_fieldNames(): Set<string>;
  ivo_internal_name(): string;
}

export interface IvoErrorSanitizer<CtxOptions = unknown> {
  sanitize?(error: unknown, ctx: CtxOptions): unknown;
}

export interface InternalFieldConfig<
  I extends IvoInputStruct = IvoInputStruct,
  O extends IvoStruct = IvoStruct,
  CtxOptions = unknown,
  ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
    IvoErrorSanitizer<CtxOptions>,
> {
  type: FieldType;
  alias?: string | null;
  dependsOn?: string[] | null;
}

export type InternalFieldConfigs<
  I extends IvoInputStruct = IvoInputStruct,
  O extends IvoStruct = IvoStruct,
  CtxOptions = unknown,
  ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
    IvoErrorSanitizer<CtxOptions>,
> = Map<string, InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer>>;

export interface TimestampConfig<Timestamp = unknown> {
  created_at?: string | null;
  updated_at?: string | null;
  _phantom?: Timestamp;
}

export interface IgnoreOptionConfig {
  fields: string[];
}

export interface IgnoreUpdateOptionConfig {
  fields: string[];
}

export interface OnSuccessConfig {
  fields: string[];
}

export interface PostValidationConfig {
  fields: string[];
}

export interface RequiredOptionConfig {
  fields: string[];
}

export interface BuildableFieldConfig<
  I extends IvoInputStruct = IvoInputStruct,
  O extends IvoStruct = IvoStruct,
  CtxOptions = unknown,
  ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
    IvoErrorSanitizer<CtxOptions>,
> {
  build(): InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer>;
}

export interface BuildableTimestampConfig<Timestamp = unknown> {
  build(): TimestampConfig<Timestamp>;
}

export class TimestampConfigBuilder<Timestamp = unknown> {
  private config: TimestampConfig<Timestamp> = {};

  public static new<Timestamp = unknown>(): TimestampConfigBuilder<Timestamp> {
    return new TimestampConfigBuilder<Timestamp>();
  }

  public setCreatedAt(name: string): this {
    this.config.created_at = name;
    return this;
  }

  public setUpdatedAt(name: string): this {
    this.config.updated_at = name;
    return this;
  }

  public build(): TimestampConfig<Timestamp> {
    return this.config;
  }
}

export interface BuildableSchemaOptions<
  I extends IvoInputStruct = IvoInputStruct,
  O extends IvoStruct = IvoStruct,
  CtxOptions = unknown,
  ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
    IvoErrorSanitizer<CtxOptions>,
> {
  build(): SchemaOptions<I, O, CtxOptions, ErrorSanitizer>;
}

export class SchemaOptions<
  I extends IvoInputStruct = IvoInputStruct,
  O extends IvoStruct = IvoStruct,
  CtxOptions = unknown,
  ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
    IvoErrorSanitizer<CtxOptions>,
> {
  public ignore?: IgnoreOptionConfig[] | null;
  public ignoreUpdate?: IgnoreUpdateOptionConfig[] | null;
  public onSuccess?: OnSuccessConfig[] | null;
  public postValidate?: PostValidationConfig[] | null;
  public required?: RequiredOptionConfig[] | null;

  constructor(
    options: Partial<SchemaOptions<I, O, CtxOptions, ErrorSanitizer>> = {},
  ) {
    this.ignore = options.ignore ?? null;
    this.ignoreUpdate = options.ignoreUpdate ?? null;
    this.onSuccess = options.onSuccess ?? null;
    this.postValidate = options.postValidate ?? null;
    this.required = options.required ?? null;
  }

  public static new<
    I extends IvoInputStruct = IvoInputStruct,
    O extends IvoStruct = IvoStruct,
    CtxOptions = unknown,
    ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
      IvoErrorSanitizer<CtxOptions>,
  >(): SchemaOptionsBuilder<I, O, CtxOptions, ErrorSanitizer> {
    return new SchemaOptionsBuilder<I, O, CtxOptions, ErrorSanitizer>(
      new SchemaOptions<I, O, CtxOptions, ErrorSanitizer>(),
    );
  }
}

export class SchemaOptionsBuilder<
  I extends IvoInputStruct = IvoInputStruct,
  O extends IvoStruct = IvoStruct,
  CtxOptions = unknown,
  ErrorSanitizer extends IvoErrorSanitizer<CtxOptions> =
    IvoErrorSanitizer<CtxOptions>,
> implements BuildableSchemaOptions<I, O, CtxOptions, ErrorSanitizer> {
  private options: SchemaOptions<I, O, CtxOptions, ErrorSanitizer>;

  constructor(options: SchemaOptions<I, O, CtxOptions, ErrorSanitizer>) {
    this.options = options;
  }

  public build(): SchemaOptions<I, O, CtxOptions, ErrorSanitizer> {
    return this.options;
  }
}

export class IvoModel {
  public static new<
    I extends IvoInputStruct,
    O extends IvoStruct,
    CtxOptions extends ObjectType,
    const ErrorMetadata = DefaultFieldErrorMetadata,
    const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
  >() {
    const errorTool = new SchemaErrorTool();

    // I extends IvoInputStruct,
    // O extends IvoStruct,
    // CtxOptions extends ObjectType,
    // const ErrorMetadata = DefaultFieldErrorMetadata,
    // const ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<I>>,
    const definitions = {};

    const aliasToVirtualMap = validateFields<I, O, CtxOptions, ErrorMetadata>(
      definitions,
      errorTool,
    );

    const options = validateOptions<
      I,
      O,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    >({ aliasToVirtualMap, definitions, errorTool, options: {} });
  }
}

function validateFields<
  I extends IvoInputStruct,
  O extends IvoStruct,
  CtxOptions extends ObjectType,
  const ErrorMetadata = DefaultFieldErrorMetadata,
>(
  definitions: NS.Definitions_<I, O, CtxOptions, ErrorMetadata>,
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
        `\n${STYLE_COLOR_RED}[${fieldName}]: occurs more than once, please remove duplicates${STYLE_RESET}\n`,
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
      if (config.default)
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

    if (circularChain != null) {
      const chainStr = circularChain.sort().join(" <-> ");
      throw new Error(
        `\n${STYLE_COLOR_RED}[${fieldName}]: circular dependency identified between "${chainStr}"${STYLE_RESET}\n`,
      );
    }
  }

  if (!fieldNames.size)
    errorTool.add("schema fields", "Insufficient Schema fields").throw();

  return aliasToVirtualMap;
}

function validateOptions<
  I extends IvoInputStruct,
  O extends IvoStruct,
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
  definitions: NS.Definitions_<I, O, CtxOptions, ErrorMetadata>;
  errorTool: SchemaErrorTool;
  options: NS.Options<I, O, CtxOptions, ErrorPayload>;
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
    // @ts-expect-error ikr
    sanitizeError,
  };

  if (options.ignore) {
    if (typeof options.ignore === "function")
      normalizedOptions.ignore = options.ignore;
    else {
      const optionName = "options.ignore";
      const type_not_allowed_error =
        "only lax and virtual fields can belong to grouped ignore configs;";

      for (const { fields } of toArray(options.ignore)) {
        if (fields.length < 2)
          errorTool.add(optionName, "grouped ignore expects at least 2 fields");

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped ignore config`,
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
    }
  }

  if (options.ignoreUpdate) {
    if (typeof options.ignoreUpdate === "function")
      normalizedOptions.ignoreUpdate = options.ignoreUpdate;
    else {
      const optionName = "options.ignoreUpdate";
      const type_not_allowed_error =
        "only lax, required and virtual fields can belong to grouped ignore update configs;";

      for (const { fields } of toArray(options.ignoreUpdate)) {
        if (fields.length < 2)
          errorTool.add(
            optionName,
            "grouped ignore update expects at least 2 fields",
          );

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped ignore update config`,
            );

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField)
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );

          if (
            !isOneOf(definitions[fieldName]?.type, [
              "lax",
              "required",
              "virtual",
            ])
          )
            errorTool.add(
              optionName,
              `${type_not_allowed_error} remove "${fieldName}"`,
            );
        }
      }
    }
  }

  if (options.onSuccess) {
    if (typeof options.onSuccess === "function")
      normalizedOptions.onSuccess = options.onSuccess;
    else {
      const optionName = "options.on_success";

      for (const { fields } of toArray(options.onSuccess)) {
        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped on success config`,
            );

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField)
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );

          if (!definitions[fieldName])
            errorTool.add(
              optionName,
              `"${fieldName}" does not exist on your schema`,
            );
        }
      }
    }
  }

  if (options.postValidate) {
    const optionName = "options.postValidate";
    const type_not_allowed_error =
      "only lax, required and virtual fields can be post-validated;";

    for (const { fields } of toArray(options.postValidate)) {
      if (fields.length < 2)
        errorTool.add(
          optionName,
          "post-validation config expects at least 2 input fields",
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
  }

  if (options.required) {
    const optionName = "options.required";
    const type_not_allowed_error =
      "only lax and virtual fields can belong to grouped required configs;";

    for (const { fields } of toArray(options.required)) {
      if (fields.length < 2)
        errorTool.add(
          optionName,
          "grouped required config expects at least 2 input fields",
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
  }

  return normalizedOptions as any;
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
