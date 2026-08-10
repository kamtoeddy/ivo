import { Model, ModelTool } from '../model';
import {
  getKeysAsProps,
  isOneOf,
  isRecordLike,
  SchemaErrorTool,
  toArray,
} from '../utils';
import {
  type ArrayOfMinSizeOne,
  type Buildable,
  type DefaultFieldErrorMetadata,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type IvoErrorPayload,
  type KeyOf,
  LIFE_CYCLES,
  type NS,
  type ObjectType,
  type PostValidationConfig,
  type RealType,
  TimeStampTool,
} from '../utils/types';
import { ConstantBuilder } from './fields/constants';
import { DependentBuilder, type HasDependsOn } from './fields/dependents';
import { type BuildableLaxConfig, LaxBuilder } from './fields/lax';
import { type BlankRequiredBuilder, RequiredBuilder } from './fields/required';
import { type BlankVirtualBuilder, VirtualBuilder } from './fields/virtual';

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
    const errorTool = new SchemaErrorTool();
    let timestamps: TimeStampTool | null = null;

    if (options.timestamps) {
      const isValid = isTimestampsConfigOptionOk(options.timestamps);

      if (!isValid.valid) errorTool.add('options.timestamps', isValid.reason);
      else timestamps = new TimeStampTool(options.timestamps);
    }

    if (errorTool.isPayloadLoaded) errorTool.throw();

    const definitionsEntries = builder(new FieldBuilder())[
      FIELD_BUILDER_DEFINITIONS
    ];

    const { aliasToVirtualMap, definitions } = validateFields<
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >({
      definitionsEntries,
      errorTool,
      timestamps,
    });

    this._definitions = definitions;

    this._options = makeOptions<I, O, CtxOptions, ErrorMetadata, ErrorPayload>({
      aliasToVirtualMap,
      definitions,
      errorTool,
      options,
      timestamps,
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
          (prop) => ![...LIFE_CYCLES, 'shouldUpdate'].includes(prop as never),
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
  'ivo-schema-field-builder',
);

function isBuildable(value: unknown): value is Buildable<unknown> {
  return (
    typeof value === 'object' &&
    !!value &&
    typeof (value as { [FIELD_CONFIG_BUILD_METHOD_NAME]?: unknown })[
      FIELD_CONFIG_BUILD_METHOD_NAME
    ] === 'function'
  );
}

class FieldBuilder<
  const I extends RealType<I>,
  const O extends RealType<O> = I,
  const CtxOptions extends ObjectType = {},
  const ErrorMetadata = DefaultFieldErrorMetadata,
> {
  private _definitions: NS.DefinitionsEntries<I, O, CtxOptions, ErrorMetadata>;

  // Optional seed, used exclusively by `Schema.extend()` to carry a parent
  // schema's already-materialized definitions (real field configs, not
  // `Buildable`s) into a fresh builder before the extension closure runs -
  // `field()` itself stays `Buildable`-only, so this is the only legitimate
  // way to fold pre-built definitions back into the builder pattern.
  constructor(seed: NS.Definitions<I, O, CtxOptions, ErrorMetadata> = {}) {
    this._definitions = Object.entries(seed);
  }

  field<K extends keyof I | keyof O>(
    c: NS.FieldDefinition<K, I, O, CtxOptions, ErrorMetadata>,
  ) {
    if (isBuildable(c)) {
      const built = c[FIELD_CONFIG_BUILD_METHOD_NAME]();

      if (typeof built.name === 'string')
        this._definitions.push([built.name, built]);
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
    DefaultState extends 'value' | 'resolver' = Default extends Function
      ? 'resolver'
      : 'value',
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

  get [FIELD_BUILDER_DEFINITIONS](): NS.DefinitionsEntries<
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
  ErrorMetadata,
>({
  definitionsEntries,
  errorTool,
  timestamps,
}: {
  definitionsEntries: NS.DefinitionsEntries<I, O, CtxOptions, ErrorMetadata>;
  errorTool: SchemaErrorTool;
  timestamps: TimeStampTool | null;
}): {
  aliasToVirtualMap: Map<string, string>;
  definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>;
} {
  const constantFieldNames = new Set<string>();
  const dependentFieldToParentFields = new Map<
    string,
    ArrayOfMinSizeOne<NS.Dependables<any, I, O>>
  >();
  const fieldNames = new Set<string>();
  const aliasToVirtualMap = new Map<string, string>();
  const definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata> = {};
  const dependentConfigs: Array<
    [string, NS.DependentField<any, I, O, CtxOptions>]
  > = [];

  const timestampKeys = timestamps?.getKeys();

  for (const [fieldName, config] of definitionsEntries) {
    if (fieldNames.has(fieldName))
      errorTool.add(
        fieldName,
        `"${fieldName}" occurs more than once, please remove duplicates`,
      );

    fieldNames.add(fieldName);
    definitions[fieldName] = config;

    if (timestampKeys) {
      if (timestampKeys.createdAt === fieldName)
        errorTool.add(
          fieldName,
          `"${fieldName}" is not a valid field name. It is the creation timestamp`,
        );
      else if (timestampKeys.updatedAt === fieldName)
        errorTool.add(
          fieldName,
          `"${fieldName}" is not a valid field name. It is the update timestamp`,
        );
    }

    if (config.type === 'constant') {
      constantFieldNames.add(fieldName);

      continue;
    }

    if (config.type === 'dependent') {
      const { dependsOn } = config;

      if (typeof config.default === 'undefined')
        errorTool.add(
          fieldName,
          'Dependent fields must have a default value or default resolver',
        );

      if (typeof config.default === 'function' && config.readonly)
        errorTool.add(
          fieldName,
          'The readonly rule is only valid for fields with static default values',
        );

      if (typeof config.resolver !== 'function')
        errorTool.add(fieldName, 'Dependent fields must have a value resolver');

      if (!dependsOn?.length)
        errorTool.add(
          fieldName,
          'Dependent fields must depend on at least one lax, required, virtual or other dependent field on your schema',
        );

      // @ts-expect-error
      if (dependsOn.includes(fieldName))
        errorTool.add(fieldName, `"${fieldName}" cannot depend on itself`);

      dependentConfigs.push([fieldName, config]);
      dependentFieldToParentFields.set(fieldName, dependsOn);

      continue;
    }

    if (config.type === 'lax') {
      if (typeof config.default === 'undefined')
        errorTool.add(
          fieldName,
          'Lax fields must have a default value or default resolver',
        );

      if (typeof config.default === 'function' && config.readonly)
        errorTool.add(
          fieldName,
          'The readonly rule is only valid for fields with static default values',
        );

      continue;
    }

    if (config.type === 'required') {
      if (!config.allow && !config.validator)
        errorTool.add(fieldName, 'Required fields must have a validator');

      continue;
    }

    const { alias } = config;

    if (alias) {
      if (fieldName === alias)
        errorTool.add(
          fieldName,
          'virtual alias name must be different from field name',
        );

      if (timestampKeys) {
        if (timestampKeys.createdAt === alias)
          errorTool.add(
            fieldName,
            `"${fieldName}" is not a valid alias. It is the creation timestamp`,
          );
        else if (timestampKeys.updatedAt === alias)
          errorTool.add(
            fieldName,
            `"${alias}" is not a valid alias. It is the update timestamp`,
          );
      }

      const otherField = aliasToVirtualMap.get(alias);

      if (otherField)
        errorTool.add(
          fieldName,
          `"${alias}" is already the alias of "${otherField}"`,
        );

      for (const [name, config] of definitionsEntries) {
        if (name !== alias) continue;

        if (config.type === 'dependent') {
          // @ts-expect-error ikr
          if (!config.dependsOn.includes(fieldName))
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

    for (const [, config] of definitionsEntries) {
      // @ts-expect-error ikr
      if (config.type === 'dependent' && config.dependsOn.includes(fieldName)) {
        hasSufficientDependencies = true;
        break;
      }
    }

    if (!hasSufficientDependencies)
      errorTool.add(
        fieldName,
        'Virtual fields are expected to have at least one dependency, but found none',
      );
  }

  for (const [fieldName, config] of dependentConfigs) {
    const parentFields = config.dependsOn!;

    const parentFieldsProvided = new Set<string>();

    for (const parentField of parentFields) {
      if (timestampKeys) {
        if (timestampKeys.createdAt === parentField)
          errorTool.add(
            fieldName,
            `"${fieldName}" cannot depend on "${parentField}" because it is the creation timestamp`,
          );
        else if (timestampKeys.updatedAt === parentField)
          errorTool.add(
            fieldName,
            `"${fieldName}" cannot depend on "${parentField}" because it is the update timestamp`,
          );
      }

      if (!fieldNames.has(parentField))
        errorTool.add(
          fieldName,
          `"${fieldName}" cannot depend on "${parentField}" because it is not a field on your schema`,
        );

      if (parentFieldsProvided.has(parentField))
        errorTool.add(
          fieldName,
          `"${parentField}" has been provided as a parent field multiple times. remove all duplicates to proceed`,
        );

      if (constantFieldNames.has(parentField))
        errorTool.add(
          fieldName,
          `"${fieldName}" cannot depend on "${parentField}" because it is a constant`,
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
          `"${fieldName}" should not depend on "${parentField}" and "${redundantField}" because "${parentField}" depends on "${redundantField}"`,
        );
        continue;
      }

      errorTool.add(
        fieldName,
        `"${fieldName}" should not depend on "${parentField}" and "${redundantField}" because "${parentField}" indirectly depends on "${redundantField}"`,
      );
    }

    const circularChain = getCircularDependencyChain(
      fieldName,
      parentFields,
      dependentFieldToParentFields,
    );

    if (circularChain)
      errorTool.add(
        fieldName,
        `circular dependency identified between "${circularChain.sort().join(' <-> ')}"`,
      );
  }

  if (errorTool.isPayloadLoaded) errorTool.throw();

  if (!fieldNames.size)
    errorTool.add('schema fields', 'Insufficient Schema fields').throw();

  return { aliasToVirtualMap, definitions };
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
  timestamps,
}: {
  aliasToVirtualMap: Map<string, string>;
  definitions: NS.Definitions<I, O, CtxOptions, ErrorMetadata>;
  errorTool: SchemaErrorTool;
  options: NS.Options<I, O, CtxOptions, ErrorMetadata, ErrorPayload>;
  timestamps: TimeStampTool | null;
}): NS.InternalOptions<I, O, CtxOptions, ErrorMetadata, ErrorPayload> {
  let sanitizeError = (p: IvoErrorPayload<ErrorMetadata, KeyOf<I>>) => p;

  if (typeof options.sanitizeError === 'function') {
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
    timestamps,
  };

  if (options.ignore) {
    const ignore: NS.IgnoreConfigOptionItem<I, O, CtxOptions>[] = [];

    if (typeof options.ignore === 'function') ignore.push(options.ignore);
    else {
      const optionName = 'options.ignore';
      const type_not_allowed_error =
        'only lax and virtual fields can belong to grouped ignore configs';

      for (const config of toArray(options.ignore)) {
        if (typeof config === 'function') {
          ignore.push(config);

          continue;
        }

        const { fields, resolver } = config;

        ignore.push({ fields, resolver });

        if (fields.length < 2)
          errorTool.add(optionName, 'grouped ignore expects at least 2 fields');

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped ignore config`,
            );

          fieldNames.add(fieldName);

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField)
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );

          if (!isOneOf(definitions[fieldName]?.type, ['lax', 'virtual']))
            errorTool.add(
              optionName,
              `${type_not_allowed_error} remove "${fieldName}"`,
            );
        }
      }
    }

    normalizedOptions.ignore = ignore;
  }

  if (options.ignoreUpdate) {
    const ignoreUpdate: NS.IgnoreUpdateConfigOptionItem<I, O, CtxOptions>[] =
      [];

    if (typeof options.ignoreUpdate === 'function')
      ignoreUpdate.push(options.ignoreUpdate);
    else {
      const optionName = 'options.ignoreUpdate';
      const type_not_allowed_error =
        'only lax, required and virtual fields can belong to grouped ignore update configs';

      for (const config of toArray(options.ignoreUpdate)) {
        if (typeof config === 'function') {
          ignoreUpdate.push(config);

          continue;
        }

        const { fields, resolver } = config;

        ignoreUpdate.push({ fields, resolver });

        if (fields.length < 2)
          errorTool.add(
            optionName,
            'grouped ignore update expects at least 2 fields',
          );

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped ignore update config`,
            );

          fieldNames.add(fieldName);

          const virtualField = aliasToVirtualMap.get(fieldName);

          if (virtualField)
            errorTool.add(
              optionName,
              `"${fieldName}" is an alias; use "${virtualField}" instead`,
            );

          if (
            !isOneOf(definitions[fieldName]?.type, [
              'lax',
              'required',
              'virtual',
            ])
          )
            errorTool.add(
              optionName,
              `${type_not_allowed_error} remove "${fieldName}"`,
            );
        }
      }
    }

    normalizedOptions.ignoreUpdate = ignoreUpdate;
  }

  if (options.onDelete) {
    const onDelete: NS.DeleteHandler<O, CtxOptions>[] = [];

    if (typeof options.onDelete === 'function') onDelete.push(options.onDelete);
    else {
      for (const config of toArray(options.onDelete))
        if (typeof config === 'function') onDelete.push(config);
    }

    if (onDelete.length) normalizedOptions.onDelete = onDelete;
  }

  if (options.onSuccess) {
    const onSuccess: NS.OnSuccessConfigOptionItem<I, O, CtxOptions>[] = [];

    if (typeof options.onSuccess === 'function')
      onSuccess.push(options.onSuccess);
    else {
      const optionName = 'options.onSuccess';

      for (const config of toArray(options.onSuccess)) {
        if (typeof config === 'function') {
          onSuccess.push(config);

          continue;
        }

        const { fields, resolver } = config;

        onSuccess.push({ fields, resolver });

        const fieldNames = new Set<string>();

        for (const fieldName of fields) {
          if (fieldNames.has(fieldName))
            errorTool.add(
              optionName,
              `remove duplicates of "${fieldName}" in your grouped on success config`,
            );

          fieldNames.add(fieldName);

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

    normalizedOptions.onSuccess = onSuccess;
  }

  if (options.postValidate) {
    const optionName = 'options.postValidate';
    const type_not_allowed_error =
      'only lax, required and virtual fields can be post-validated';

    const postValidate: PostValidationConfig<
      any,
      I,
      O,
      CtxOptions,
      ErrorMetadata
    >[] = [];

    for (const { fields, validator } of toArray(options.postValidate)) {
      postValidate.push({ fields, validator });

      if (fields.length < 2)
        errorTool.add(
          optionName,
          'post-validation config expects at least 2 I fields',
        );

      const fieldNames = new Set<string>();

      for (const fieldName of fields) {
        if (fieldNames.has(fieldName))
          errorTool.add(
            optionName,
            `remove duplicates of "${fieldName}" in your grouped post-validation config`,
          );

        fieldNames.add(fieldName);

        const virtualField = aliasToVirtualMap.get(fieldName);

        if (virtualField)
          errorTool.add(
            optionName,
            `"${fieldName}" is an alias; use "${virtualField}" instead`,
          );

        if (
          !isOneOf(definitions[fieldName]?.type, ['lax', 'required', 'virtual'])
        )
          errorTool.add(
            optionName,
            `${type_not_allowed_error} remove "${fieldName}"`,
          );
      }
    }

    normalizedOptions.postValidate = postValidate;
  }

  if (options.required) {
    const optionName = 'options.required';
    const type_not_allowed_error =
      'only lax and virtual fields can belong to grouped required configs';
    const required: NS.RequiredConfigObject<I, O, CtxOptions, ErrorMetadata>[] =
      [];

    for (const { fields, handler } of toArray(options.required)) {
      required.push({ fields, handler });

      if (fields.length < 2)
        errorTool.add(
          optionName,
          'grouped required config expects at least 2 I fields',
        );

      const fieldNames = new Set<string>();

      for (const fieldName of fields) {
        if (fieldNames.has(fieldName))
          errorTool.add(
            optionName,
            `remove duplicates of "${fieldName}" in your grouped required config`,
          );

        fieldNames.add(fieldName);

        const virtualField = aliasToVirtualMap.get(fieldName);

        if (virtualField)
          errorTool.add(
            optionName,
            `"${fieldName}" is an alias; use "${virtualField}" instead`,
          );

        if (!isOneOf(definitions[fieldName]?.type, ['lax', 'virtual']))
          errorTool.add(
            optionName,
            `${type_not_allowed_error} remove "${fieldName}"`,
          );
      }
    }

    normalizedOptions.required = required;
  }

  if (errorTool.isPayloadLoaded) errorTool.throw();

  return normalizedOptions as any;
}

function isTimestampsConfigOptionOk<I, O>(
  timestamps: NS.Options<I, O>['timestamps'],
) {
  const valid = false;

  const typeProveded = typeof timestamps;

  if (typeProveded === 'boolean') return { valid: true };

  if (!isRecordLike(timestamps))
    return { valid, reason: "should be 'boolean' or 'non null object'" };

  if (!Object.keys(timestamps!).length)
    return { valid, reason: 'cannot be an empty object' };

  const createdAt =
    typeof timestamps.createdAt === 'string'
      ? timestamps.createdAt
      : 'createdAt';

  let updatedAt =
    typeof timestamps.updatedAt === 'string'
      ? timestamps.updatedAt
      : 'updatedAt';

  if (typeof createdAt === 'string' && !createdAt.trim().length)
    return { valid, reason: "'createdAt' cannot be an empty string" };

  if (typeof updatedAt === 'string' && !updatedAt.trim().length)
    return { valid, reason: "'updatedAt' cannot be an empty string" };

  if (typeof timestamps.updatedAt === 'object') {
    const updatedAtConfig = timestamps.updatedAt;

    const keys = Object.keys(updatedAtConfig).filter((prop) =>
      isOneOf(prop, ['key', 'nullable']),
    );

    if (!keys.length)
      return {
        valid,
        reason: "'updatedAt' can only accept fields 'key' and 'nullable'",
      };

    if (keys.includes('key')) {
      updatedAt = updatedAtConfig.key!;

      if (typeof updatedAt !== 'string' || !updatedAt.trim().length)
        return { valid, reason: "'updatedAt.key' must be a valid string" };
    }

    if (
      keys.includes('nullable') &&
      typeof updatedAtConfig.nullable !== 'boolean'
    )
      return { valid, reason: "'updatedAt.nullable' must be a boolean" };
  }

  if (createdAt === updatedAt)
    return { valid, reason: 'createdAt & updatedAt cannot be same' };

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
