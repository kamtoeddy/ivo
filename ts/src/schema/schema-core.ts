/* eslint-disable @typescript-eslint/no-non-null-asserted-optional-chain */

import type { ObjectType } from "../utils";
import {
  getKeysAsProps,
  getUnique,
  hasAnyOf,
  isEqual,
  isFunctionLike,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  sort,
  toArray,
} from "../utils";
import {
  ALLOWED_OPTIONS,
  type DefinitionRule,
  type IvoErrorPayload,
  type KeyOf,
  LIFE_CYCLES,
  type NS as ns,
  type PostValidationConfig,
  type PostValidator,
} from "./types";
import {
  type DefaultFieldErrorMetadata,
  isInputFieldError,
  SchemaErrorTool,
  TimeStampTool,
} from "./utils";

export {
  defaultOptions,
  getInvalidConfigMessageForRepeatedFields,
  getInvalidOnSuccessConfigMessage,
  getInvalidPostValidateConfigMessage,
  SchemaCore,
};

const defaultOptions: ns.Options<unknown, unknown, never, never> = {
  equalityDepth: 1,
  sanitizeError: (p) => p,
  ignore: undefined,
  ignoreUpdate: undefined,
  timestamps: false,
};

abstract class SchemaCore<
  Input,
  Output,
  CtxOptions extends ObjectType,
  ErrorMetadata = DefaultFieldErrorMetadata,
  ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
> {
  protected _definitions = {} as ns.Definitions_<
    Input,
    Output,
    CtxOptions,
    ErrorMetadata
  >;
  protected _options: ns.InternalOptions<
    Input,
    Output,
    CtxOptions,
    ErrorMetadata,
    ErrorPayload
  >;

  protected defaults: Partial<Output> = {};

  // maps
  protected readonly aliasToVirtualMap: ns.AliasToVirtualMap<Input> = {};
  protected readonly dependencyMap: ns.DependencyMap<Input> = {};
  protected readonly propsWithSecondaryValidators = new Set<string>();
  protected readonly virtualToAliasMap: ns.AliasToVirtualMap<Input> = {};
  protected readonly postValidationConfigMap = new Map<
    string,
    {
      index: number;
      validators: PostValidationConfig<
        KeyOf<Input>,
        Input,
        Output,
        CtxOptions,
        ErrorMetadata
      >["validator"];
    }
  >();
  protected readonly propToPostValidationConfigIDsMap = new Map<
    string,
    Set<string>
  >();
  protected readonly onSuccessConfigMap = new Map<
    string,
    { index: number; handlers: ns.SuccessHandler<Input, Output, CtxOptions>[] }
  >();

  // props
  protected readonly constants = new Set<KeyOf<Output>>();
  protected readonly dependents = new Set<KeyOf<Output>>();
  protected readonly laxProps = new Set<KeyOf<Input>>();
  protected readonly props = new Set<KeyOf<Output>>();
  protected readonly propsRequiredBy = new Set<KeyOf<Input>>();
  protected readonly readonlyProps = new Set<KeyOf<Input>>();
  protected readonly requiredProps = new Set<KeyOf<Input>>();
  protected readonly virtuals = new Set<KeyOf<Input>>();

  // helpers
  protected timestampTool: TimeStampTool;

  constructor(
    definitions: ns.Definitions_<Input, Output, CtxOptions, ErrorMetadata>,
    options: ns.Options<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    > = defaultOptions as never,
  ) {
    this._checkPropDefinitions(definitions);
    this._checkOptions(options);

    this._definitions = definitions;
    // @ts-expect-error ikr
    this._options = Object.assign({}, defaultOptions, options);

    if (!this._options.sanitizeError)
      this._options.sanitizeError = defaultOptions.sanitizeError as never;

    this.timestampTool = new TimeStampTool(this._options.timestamps);
  }

  protected _getAliasByVirtual = (prop: KeyOf<Input>): string | undefined =>
    this.virtualToAliasMap[prop];

  protected _getVirtualByAlias = (alias: string): KeyOf<Input> | undefined =>
    this.aliasToVirtualMap[alias];

  private _getCircularDependenciesOf = ({
    definitions,
    property,
    propertyB = property,
    visitedNodes = [],
  }: {
    definitions: ns.Definitions_<Input, Output, CtxOptions, ErrorMetadata>;
    property: KeyOf<Input>;
    propertyB?: KeyOf<Input>;
    visitedNodes?: KeyOf<Input>[];
  }) => {
    let circularDependencies: string[] = [];

    if (!this._isDependentProp(propertyB) || visitedNodes.includes(propertyB))
      return [];

    if (property !== propertyB) visitedNodes.push(propertyB);

    const _dependsOn = toArray<KeyOf<Input>>(
      // @ts-expect-error ikr
      definitions?.[propertyB]?.dependsOn ?? [],
    );

    for (const _prop of _dependsOn)
      if (_prop === property) circularDependencies.push(propertyB);
      else if (this._isDependentProp(_prop))
        circularDependencies = [
          ...circularDependencies,
          ...this._getCircularDependenciesOf({
            definitions,
            property,
            propertyB: _prop,
            visitedNodes,
          }),
        ];

    return sort(Array.from(new Set(circularDependencies)));
  };

  private _getRedundantDependenciesOf = ({
    definitions,
    property,
  }: {
    definitions: ns.Definitions_<Input, Output, CtxOptions, ErrorMetadata>;
    property: KeyOf<Input>;
  }) => {
    const redundantParentProps: [string, string][] = [];

    if (!this._isDependentProp(property)) return [];

    const parentProps = toArray<KeyOf<Input>>(
      // @ts-expect-error ikr
      definitions?.[property]?.dependsOn ?? [],
    );

    for (const parentProp of parentProps) {
      for (const prop of parentProps) {
        if (prop === parentProp) continue;

        if (this._isRedundantDependencyOf({ definitions, parentProp, prop }))
          redundantParentProps.push([parentProp, prop]);
      }
    }

    return redundantParentProps;
  };

  private _isRedundantDependencyOf = ({
    definitions,
    prop,
    parentProp,
  }: {
    definitions: ns.Definitions_<Input, Output, CtxOptions, ErrorMetadata>;
    prop: KeyOf<Input>;
    parentProp: KeyOf<Input>;
  }): boolean => {
    if (!this._isDependentProp(prop)) return false;

    const parentProps = toArray<KeyOf<Input>>(
      // @ts-expect-error ikr
      definitions?.[prop]?.dependsOn ?? [],
    );

    if (parentProps.includes(parentProp)) return true;

    return parentProps.some((prop) =>
      this._isRedundantDependencyOf({ definitions, parentProp, prop }),
    );
  };

  private _setDependencies = (
    prop: KeyOf<Input>,
    dependsOn: KeyOf<Input> | KeyOf<Input>[],
  ) => {
    const _dependsOn = toArray(dependsOn) as KeyOf<Input>[];

    for (const _prop of _dependsOn)
      if (this.dependencyMap[_prop]) this.dependencyMap[_prop]?.push(prop);
      else this.dependencyMap[_prop] = [prop];
  };

  private _areHandlersOk = ({
    handlers: _handlers,
    lifeCycle,
  }: {
    handlers: unknown;
    lifeCycle: ns.LifeCycle;
  }) => {
    const reasons: string[] = [],
      handlers = toArray(_handlers);

    handlers.forEach((handler, i) => {
      if (!isFunctionLike(handler))
        return reasons.push(
          `The '${lifeCycle}' handler at index: ${i} is not a function`,
        );
    });

    if (reasons.length) return { valid: false, reasons };

    return { valid: true };
  };

  protected _checkOptions = (
    options: ns.Options<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload>,
  ) => {
    const error = new SchemaErrorTool();

    if (!isRecordLike(options))
      error.add("schema options", "Must be an object").throw();

    const optionsProvided = Object.keys(options) as ns.OptionsKey<
      Output,
      Input
    >[];

    if (!optionsProvided.length) return;

    for (const option of optionsProvided)
      if (!ALLOWED_OPTIONS.includes(option))
        error.add(option, "Invalid option").throw();

    if (isPropertyOf("equalityDepth", options)) {
      const typeProvided = typeof options.equalityDepth;

      if (
        !["number", "undefined"].includes(typeProvided) ||
        options.equalityDepth! < 0
      )
        error
          .add(
            "equalityDepth",
            "'equalityDepth' must be a number between 0 and +Infinity",
          )
          .throw();
    }

    if (isPropertyOf("onDelete", options)) {
      const isValid = this._areHandlersOk({
        handlers: options.onDelete,
        lifeCycle: "onDelete",
      });

      if (!isValid.valid) error.add("onDelete", isValid.reasons!).throw();
    }

    if (isPropertyOf("onSuccess", options)) {
      const isValid = this._isOnSuccessOptionOk(options.onSuccess as never);

      if (!isValid.valid) error.add("onSuccess", isValid.reason!).throw();
    }

    if (isPropertyOf("ignore", options)) {
      const isValid = this._isIgnoreOptionOk(options.ignore);

      if (!isValid.valid) error.add("ignore", isValid.reason!).throw();
    }

    if (isPropertyOf("ignoreUpdate", options)) {
      const isValid = this._isIgnoreUpdateOptionOk(options.ignoreUpdate);

      if (!isValid.valid) error.add("ignoreUpdate", isValid.reason!).throw();
    }

    if (isPropertyOf("postValidate", options)) {
      const isValid = this._isPostValidateOptionOk(
        options.postValidate as never,
      );

      if (!isValid.valid) error.add("postValidate", isValid.reason!).throw();
    }

    if (isPropertyOf("timestamps", options)) {
      const isValid = this._isTimestampsOptionOk(options.timestamps);

      if (!isValid.valid) error.add("timestamps", isValid.reason!).throw();
    }

    if (isPropertyOf("required", options)) {
      const isValid = this._isRequiredOptionOk(
        options.required as never,
        options.timestamps,
      );

      if (!isValid.valid) error.add("required", isValid.reason!).throw();
    }
  };

  protected _checkPropDefinitions = (
    definitions: ns.Definitions_<Input, Output, CtxOptions, ErrorMetadata>,
  ) => {
    const error = new SchemaErrorTool();

    if (!isRecordLike(definitions)) error.throw();

    const props = getKeysAsProps(definitions);

    if (!props.length)
      error.add("schema fields", "Insufficient Schema fields").throw();

    for (const prop of props) {
      // @ts-expect-error ikr
      const isDefOk = this.__isPropDefinitionOk(prop, definitions[prop]);

      if (!isDefOk.valid) error.add(prop, isDefOk.reasons!);
    }

    // make sure every virtual property has at least one dependency
    for (const prop of this.virtuals) {
      const dependencies = this.dependencyMap[prop as KeyOf<Input>] ?? [];

      if (!dependencies.length)
        error.add(
          prop,
          "A virtual property must have at least one property that depends on it",
        );
    }

    // make sure aliases respect the second validation rules
    for (const [alias, prop] of Object.entries(this.aliasToVirtualMap)) {
      const isValid = this.__isVirtualAliasOk2(alias);

      if (!isValid.valid) error.add(prop, isValid.reason);
    }

    // make sure every virtual has at least one dependency
    for (const prop of this.dependents) {
      const definition = definitions[prop];

      const _dependsOn = toArray<KeyOf<Input>>(
        (definition as any)?.dependsOn ?? [],
      );

      if (_dependsOn.includes(prop as never))
        error.add(prop, "A property cannot depend on itself");

      const dependsOnConstantProp = _dependsOn.some(this._isConstant);

      if (dependsOnConstantProp)
        error.add(prop, "A property cannot depend on a constant property");

      // check against dependencies on invalid fields
      const invalidProps = _dependsOn.filter(
        (p) => !(this._isProp(p) || this._isVirtual(p)),
      );

      for (const _prop of invalidProps)
        error.add(
          prop,
          `Cannot establish dependency with '${_prop}' as it is neither a property nor a virtual of your model`,
        );

      // check against circular dependencies
      const circularRelationShips = this._getCircularDependenciesOf({
        definitions,
        property: prop,
      } as never);

      for (const _prop of circularRelationShips)
        error.add(prop, `Circular dependency identified with '${_prop}'`);

      // check against circular dependencies
      const redundantRelationShips = this._getRedundantDependenciesOf({
        definitions,
        property: prop,
      } as never);

      for (const [parentProp, _prop] of redundantRelationShips)
        error.add(
          prop,
          `Dependency on '${parentProp}' is redundant because of dependency on '${_prop}'`,
        );
    }

    if (error.isPayloadLoaded) error.throw();
  };

  protected _isConstant = (prop: string) =>
    this.constants.has(prop as KeyOf<Output>);

  protected _isDefaultable = (prop: string) =>
    isPropertyOf("default", this._getDefinition(prop));

  protected _isDependentProp = (prop: string) =>
    this.dependents.has(prop as KeyOf<Output>);

  protected _isInputProp = (prop: string) => {
    return (
      (this._isProp(prop) || this._isVirtual(prop)) &&
      !this._isConstant(prop) &&
      !this._isDependentProp(prop)
    );
  };

  protected _isLaxProp = (prop: string) =>
    this.laxProps.has(prop as KeyOf<Input>);

  protected _isProp = (prop: string) => this.props.has(prop as KeyOf<Output>);

  protected _isReadonly = (prop: string) =>
    this.readonlyProps.has(prop as KeyOf<Input>);

  protected _isRequired = (prop: string) =>
    this.requiredProps.has(prop as KeyOf<Input>);

  protected _isRequiredBy = (prop: string) =>
    this.propsRequiredBy.has(prop as KeyOf<Input>);

  protected _isRuleInDefinition = (
    prop: string,
    rules: DefinitionRule | DefinitionRule[],
  ): boolean => {
    for (const _prop of toArray(rules))
      if (isPropertyOf(_prop, this._getDefinition(prop))) return true;

    return false;
  };

  protected _isVirtualAlias = (prop: string) => !!this.aliasToVirtualMap[prop];

  protected _isVirtual = (prop: string) =>
    this.virtuals.has(prop as KeyOf<Input>);

  protected _getDefinition = (prop: string) =>
    this._definitions[prop as KeyOf<Input>]!;

  protected _getFrozenCopy = <T>(data: T): Readonly<T> =>
    Object.freeze(Object.assign({}, data)) as Readonly<T>;

  protected _getHandlers = <T>(prop: string, lifeCycle: ns.LifeCycle) =>
    // @ts-expect-error ikr
    toArray((this._getDefinition(prop)?.[lifeCycle] ?? []) as never) as T[];

  private __hasAllowedValues = (
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
    isRecursion = false,
  ): { valid: boolean; reason?: string } => {
    const { allow } = definition as any,
      valid = false,
      isObject = isRecordLike(allow);

    if (isObject && !isRecursion) {
      const res = this.__hasAllowedValues(definition, true);

      if (!res.valid) return res;

      if (isPropertyOf("error", allow)) {
        const invalidErrorTypeMessage =
          'The "error" field of the allow rule can only accept a string, InputFieldError or an function that returns any of the above mentioned';

        const error = allow.error,
          isFunction = isFunctionLike(error),
          isString = typeof error === "string";

        if (!isFunction && !isString && !isInputFieldError(error))
          return { valid, reason: invalidErrorTypeMessage };
      }

      if (Object.keys(allow).some((k) => !["error", "values"].includes(k)))
        return {
          valid,
          reason:
            'The "allow" rule only accepts "error" & "values" as configuration. Please remove the extra keys',
        };

      return { valid: true };
    }

    const allowedValues = (isObject
      ? allow.values
      : allow) as unknown as never[];

    if (!Array.isArray(allowedValues))
      return { reason: "Allowed values must be an array", valid };

    if (getUnique(allowedValues).length !== allowedValues.length)
      return {
        reason: "Allowed values must be an array of unique values",
        valid,
      };

    if (allowedValues.length < 2)
      return { reason: "Allowed values must have at least 2 values", valid };

    if (
      isPropertyOf("default", definition) &&
      !isOneOf((definition as any)?.default, allowedValues as never)
    )
      return { reason: "The default value must be an allowed value", valid };

    return { valid: true };
  };

  private __isDependentProp = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
  ) => {
    const { default: _default, dependsOn } = definition as any;

    const valid = false;

    if (!dependsOn?.length)
      return {
        valid,
        reason: "Dependent fields must depend on at least one property",
      };

    if (toArray(dependsOn).includes(prop as KeyOf<Input>))
      return { valid, reason: "A property cannot depend on itself" };

    return { valid: true };
  };

  private __isPropDefinitionOk = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
  ) => {
    let reasons: string[] = [];

    if (isPropertyOf("allow", definition)) {
      const { valid, reason } = this.__hasAllowedValues(definition);

      if (!valid) reasons.push(reason!);
    }

    if (isPropertyOf("alias", definition)) {
      const { valid, reason } = this.__isVirtualAliasOk(prop, definition);

      if (valid) {
        const alias = (definition as any).alias!;

        this.aliasToVirtualMap[alias] = prop;
        this.virtualToAliasMap[prop] = alias as KeyOf<Input>;
      } else reasons.push(reason!);
    }

    if ((definition as any)?.type === "constant")
      this.constants.add(prop as never);

    if (hasAnyOf(definition, ["dependsOn", "resolver"])) {
      const { valid, reason } = this.__isDependentProp(prop, definition);

      if (valid) {
        this.dependents.add(prop as never);
        this._setDependencies(prop, (definition as any).dependsOn!);
      } else reasons.push(reason!);
    }

    if ((definition as any)?.type === "required") {
      const { valid, reason } = this.__isRequired(definition);

      valid ? this.requiredProps.add(prop) : reasons.push(reason!);
    }

    if ((definition as any)?.type === "virtual") this.virtuals.add(prop);

    // onDelete, onFailure, & onSuccess
    for (const rule of LIFE_CYCLES) {
      if (!isPropertyOf(rule, definition)) continue;

      const isValid = this._areHandlersOk({
        handlers: definition[rule],
        lifeCycle: rule,
      });

      if (!isValid.valid) reasons = reasons.concat(isValid.reasons!);
    }

    if (this.__isLax(definition)) this.laxProps.add(prop);

    const hasDefaultRule = isPropertyOf("default", definition);

    if (
      !hasDefaultRule &&
      !this._isConstant(prop) &&
      !this._isDependentProp(prop) &&
      !this._isLaxProp(prop) &&
      !this._isReadonly(prop) &&
      !this._isRequired(prop) &&
      !this._isVirtual(prop) &&
      !reasons.length
    ) {
      reasons.push(
        "A property should at least be readonly, required, or have a default value",
      );
    }

    const valid = reasons.length <= 0;

    if (valid && !this._isVirtual(prop)) {
      this.props.add(prop as never);

      if (hasDefaultRule && typeof (definition as any).default !== "function")
        this.defaults[prop as unknown as KeyOf<Output>] = (definition as any)
          .default as never;
    }

    return { reasons, valid };
  };

  private __isRequiredCommon = (
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
  ) => {
    const valid = false;

    if (isPropertyOf("dependsOn", definition))
      return { valid, reason: "Required fields cannot be dependent" };

    return { valid: true };
  };

  private __isRequired = (
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
  ) => {
    const valid = false;

    if ((definition as any)?.type !== "required")
      return {
        valid,
        reason: "Required fields must have required as 'true'",
      };

    if (
      !isPropertyOf("allow", definition) &&
      !isPropertyOf("validator", definition)
    )
      return { valid, reason: "Required fields must have a validator" };

    const isRequiredCommon = this.__isRequiredCommon(definition);

    if (!isRequiredCommon.valid) return isRequiredCommon;

    return { valid: true };
  };

  private __isVirtualAliasOk = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
  ) => {
    const valid = false;

    const { alias } = definition as any;

    if ((definition as any)?.type !== "virtual")
      return { valid, reason: "Only virtual fields can have aliases" };

    if (typeof alias !== "string" || !alias.length)
      return {
        valid,
        reason: "An alias must be a string with at least 1 character",
      };

    if (alias === prop)
      return {
        valid,
        reason: "An alias cannot be the same as the virtual property",
      };

    const isTakenBy = this._getVirtualByAlias(alias);
    if (isTakenBy)
      return {
        valid,
        reason: `Sorry, alias provided '${alias}' already belongs to property '${isTakenBy}'`,
      };

    return { valid: true };
  };

  private __isVirtualAliasOk2 = (alias: string | KeyOf<Input>) => {
    const prop = this._getVirtualByAlias(alias)!;

    const invalidResponse = {
      valid: false,
      reason: `'${alias}' cannot be used as the alias of '${prop}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
    };

    const isDependentOnVirtual = (
      this.dependencyMap[prop as KeyOf<Input>] ?? []
    )?.includes(alias as KeyOf<Input>);

    return (this._isProp(alias) && !isDependentOnVirtual) ||
      this._isVirtual(alias)
      ? invalidResponse
      : ({ valid: true } as typeof invalidResponse);
  };

  private __isLax = (
    definition: ns.Definitions_<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[KeyOf<Input>],
  ) => {
    // Lax fields must have a default value nor setter
    if (isEqual((definition as any)?.default, undefined)) return false;

    // Lax fields cannot be dependent
    if (isPropertyOf("dependent", definition)) return false;

    // Lax fields cannot be required
    if (isPropertyOf("required", definition)) return false;

    // Lax fields cannot be virtual
    if (isPropertyOf("virtual", definition)) return false;

    return true;
  };

  private _isPostValidateSingleConfigOk(value: unknown, index?: number) {
    const valid = false;

    if (
      !value ||
      !isPropertyOf("fields", value) ||
      !isPropertyOf("validator", value) ||
      Object.keys(value).length > 2
    )
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(index),
      };

    // @ts-expect-error: lol
    if (!Array.isArray(value.fields))
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          "fields-must-be-input-array",
        ),
      };

    // @ts-expect-error: lol
    const fields = getUnique(value.fields);

    if (fields.length < 2)
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          "fields-must-be-input-array",
        ),
      };

    // @ts-expect-error: lol
    if (fields.length < value.fields.length)
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          "fields-array-must-contain-unique-values",
        ),
      };

    const reasons: string[] = [];

    for (const prop of fields)
      if (!this._isInputProp(prop) && !this._isVirtual(prop)) {
        if (index !== undefined)
          reasons.push(
            `Config at index ${index}: "${prop}" cannot be post-validated`,
          );
        else reasons.push(`"${prop}" cannot be post-validated`);
      }

    if (reasons.length) return { valid, reason: reasons };

    // @ts-expect-error: lol
    if (Array.isArray(value.validator)) {
      // @ts-expect-error: lol
      const validators = value.validator as Exclude<
        PostValidationConfig<
          KeyOf<Input>,
          Input,
          Output,
          CtxOptions,
          ErrorMetadata
        >["validator"],
        PostValidator<KeyOf<Input>, Input, Output, CtxOptions, ErrorMetadata>
      >;

      if (!validators.length)
        return {
          valid,
          reason: getInvalidPostValidateConfigMessage(
            index,
            "validator-array-cannot-be-empty",
          ),
        };

      const reasons: string[] = [];

      validators.forEach((validator, i) => {
        if (Array.isArray(validator)) {
          validator.forEach((v, i2) => {
            if (!isFunctionLike(v))
              reasons.push(
                getInvalidPostValidateConfigMessage(
                  index,
                  "validator-must-be-function",
                  i,
                  i2,
                ),
              );
          });

          if (!validator.length)
            return reasons.push(
              getInvalidPostValidateConfigMessage(
                index,
                "validator-array-cannot-be-empty",
                i,
              ),
            );
        } else if (!isFunctionLike(validator))
          reasons.push(
            getInvalidPostValidateConfigMessage(
              index,
              "validator-must-be-function-or-array",
              i,
            ),
          );
      });

      if (reasons.length) return { valid, reason: reasons };

      return { valid: true };
    }

    // @ts-expect-error: lol
    if (!isFunctionLike(value.validator))
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          "validator-must-be-function",
        ),
      };

    return { valid: true };
  }

  private __isOnSuccessSingleConfigOk(value: unknown, index?: number) {
    if (isFunctionLike(value)) return { valid: true };

    const valid = false;

    if (
      !value ||
      !isPropertyOf("fields", value) ||
      !isPropertyOf("handler", value) ||
      Object.keys(value).length > 2
    )
      return { valid, reason: getInvalidOnSuccessConfigMessage(index) };

    // @ts-expect-error: lol
    if (!Array.isArray(value.fields))
      return {
        valid,
        reason: getInvalidOnSuccessConfigMessage(
          index,
          "config-fields-must-be-array",
        ),
      };

    // @ts-expect-error: lol
    const fields = getUnique(value.fields);

    if (fields.length < 2)
      return {
        valid,
        reason: getInvalidOnSuccessConfigMessage(
          index,
          "config-fields-must-be-array",
        ),
      };

    const reasons: string[] = [];

    for (const prop of fields)
      if (!this._isProp(prop) && !this._isVirtual(prop)) {
        reasons.push(
          `${
            index !== undefined ? `Config at index ${index}: ` : ""
          }"${prop}" is not a property or virtual on your schema`,
        );
      }

    if (reasons.length) return { valid, reason: reasons };

    // @ts-expect-error: lol
    if (Array.isArray(value.handler)) {
      // @ts-expect-error: lol
      const handlers = value.handler as unknown[];

      if (!handlers.length)
        return {
          valid,
          reason: getInvalidOnSuccessConfigMessage(
            index,
            "handler-array-cannot-be-empty",
          ),
        };

      const reasons: string[] = [];

      handlers.forEach((handler, i) => {
        if (!isFunctionLike(handler))
          reasons.push(
            getInvalidOnSuccessConfigMessage(
              index,
              "handler-must-be-function",
              i,
            ),
          );
      });

      if (reasons.length) return { valid, reason: reasons };

      return { valid: true };
    }

    // @ts-expect-error: lol
    if (!isFunctionLike(value.handler))
      return {
        valid,
        reason: getInvalidOnSuccessConfigMessage(
          index,
          "config-handler-should-be-array-or-function",
        ),
      };

    return { valid: true };
  }

  private _isIgnoreOptionOk(val: unknown) {
    if (val === undefined || typeof val === "boolean" || isFunctionLike(val))
      return { valid: true };

    const configs = toArray(val);

    if (!configs || !configs.length)
      return {
        valid: false,
        reason:
          "'ignore' option must be a boolean, function, config object, or array of config objects",
      };

    for (let i = 0; i < configs.length; i++) {
      const c = configs[i];

      if (typeof c === "boolean" || isFunctionLike(c)) continue;

      if (
        !c ||
        typeof c !== "object" ||
        !Array.isArray((c as any).fields) ||
        !isFunctionLike((c as any).resolver)
      )
        return {
          valid: false,
          reason: `'ignore' config at index ${i} must have 'fields' array and 'resolver' function`,
        };
    }

    return { valid: true };
  }

  private _isIgnoreUpdateOptionOk(val: unknown) {
    if (val === undefined || typeof val === "boolean" || isFunctionLike(val))
      return { valid: true };

    const configs = toArray(val);

    if (!configs || !configs.length)
      return {
        valid: false,
        reason:
          "'ignoreUpdate' option must be a boolean, function, config object, or array of config objects",
      };

    for (let i = 0; i < configs.length; i++) {
      const c = configs[i];

      if (typeof c === "boolean" || isFunctionLike(c)) continue;

      if (
        !c ||
        typeof c !== "object" ||
        !Array.isArray((c as any).fields) ||
        !isFunctionLike((c as any).resolver)
      )
        return {
          valid: false,
          reason: `'ignoreUpdate' config at index ${i} must have 'fields' array and 'resolver' function`,
        };
    }

    return { valid: true };
  }

  /**
   * Mirrors Rust's `options.required` validation in `schema/mod.rs::make_options`:
   * each config needs at least 2 fields, no duplicates, no aliases (the
   * virtual's real name must be used instead), and only lax/virtual fields
   * (including conditionally-required "requiredBy" ones, which Rust classifies
   * as plain `FieldType::Lax`) — constants, dependents, strictly-required
   * fields, and timestamps are rejected, matching Rust's panic-on-first-violation
   * behaviour (no accumulation).
   */
  private _isRequiredOptionOk(
    val: unknown,
    timestamps: ns.Options<Input, Output>["timestamps"],
  ) {
    if (val === undefined) return { valid: true };

    const configs = toArray(val);

    if (!configs || !configs.length)
      return {
        valid: false,
        reason:
          "'required' option must be a config object ({ fields, handler }) or an array of config objects",
      };

    // `this.timestampTool` isn't constructed yet at this point in the
    // constructor (`_checkOptions` runs before it), so build a throwaway one
    // from the raw (already-validated) `timestamps` option just to read keys.
    const { createdAt, updatedAt } = new TimeStampTool(timestamps).getKeys();

    for (let i = 0; i < configs.length; i++) {
      const c = configs[i] as { fields?: unknown; handler?: unknown };

      if (
        !c ||
        typeof c !== "object" ||
        !Array.isArray(c.fields) ||
        !(isFunctionLike(c.handler) || Array.isArray(c.handler))
      )
        return {
          valid: false,
          reason: `'required' config at index ${i} must have a 'fields' array and a 'handler' function or array of functions`,
        };

      if (c.fields.length < 2)
        return {
          valid: false,
          reason: "grouped required expects at least 2 fields",
        };

      const seen = new Set<string>();

      for (const field of c.fields as string[]) {
        if (seen.has(field))
          return {
            valid: false,
            reason: `remove duplicates of '${field}' in your grouped required config`,
          };

        seen.add(field);

        const virtualField = this._getVirtualByAlias(field);

        if (virtualField)
          return {
            valid: false,
            reason: `'${field}' is an alias; use '${virtualField}' instead`,
          };

        const notAllowedReason = `only lax and virtual fields can belong to grouped required configs; remove '${field}'`;

        if (this._isInputProp(field)) {
          if (this._isRequired(field))
            return { valid: false, reason: notAllowedReason };

          continue;
        }

        if (this._isProp(field) || field === createdAt || field === updatedAt)
          return { valid: false, reason: notAllowedReason };

        return {
          valid: false,
          reason: `'${field}' does not exist on your schema`,
        };
      }
    }

    return { valid: true };
  }

  private _registerPostValidator(
    {
      fields,
    }: PostValidationConfig<
      KeyOf<Input>,
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >,
    index: number,
  ) {
    const sortedProps = sort(fields as any),
      sortedPropsId = sortedProps.toString();

    const config = this.postValidationConfigMap.get(sortedPropsId);

    if (config)
      return {
        valid: false,
        reason: getInvalidConfigMessageForRepeatedFields(index, config.index),
      };

    return { valid: true };
  }

  private _registerSuccessConfig(
    config: ns.OnSuccessConfigOption<Input, Output, CtxOptions>,
    index: number,
  ) {
    const configObj = config as any;
    const fields = configObj.fields ?? configObj.fields;

    const sortedProps = sort(fields as any),
      sortedPropsId = sortedProps.toString();

    const existingConfig = this.onSuccessConfigMap.get(sortedPropsId);

    if (existingConfig)
      return {
        valid: false,
        reason: getInvalidConfigMessageForRepeatedFields(
          index,
          existingConfig.index,
        ),
      };

    return { valid: true };
  }

  private _isOnSuccessOptionOk(
    option: ns.Options<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata,
      ErrorPayload
    >["onSuccess"],
  ) {
    const valid = false,
      isFunction = isFunctionLike(option),
      isObject = isRecordLike(option);

    if (!option || (!Array.isArray(option) && !isFunction && !isObject))
      return { valid, reason: getInvalidOnSuccessConfigMessage() };

    if (isObject) return this.__isOnSuccessSingleConfigOk(option);

    const configs: ns.OnSuccessConfigOption<Input, Output, CtxOptions>[] =
      option;
    let reasons: string[] = [];

    configs.forEach((config, i) => {
      const isValid = this.__isOnSuccessSingleConfigOk(config, i);

      if (!isValid.valid) {
        const reason = isValid.reason!;

        if (Array.isArray(reason)) reasons = reasons.concat(reason);
        else reasons.push(reason);
      }
    });

    if (reasons.length) return { valid: false, reason: reasons };

    configs.forEach((config, i) => {
      const isValid = this._registerSuccessConfig(config, i);

      if (!isValid.valid) reasons.push(isValid.reason!);
    });

    return reasons.length ? { valid: false, reason: reasons } : { valid: true };
  }

  private _isPostValidateOptionOk(
    option: ns.Options<
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >["postValidate"],
  ) {
    const valid = false,
      isObject = isRecordLike(option);

    if (!option || (!Array.isArray(option) && !isObject))
      return { valid, reason: getInvalidPostValidateConfigMessage() };

    if (isObject) {
      const isValid = this._isPostValidateSingleConfigOk(option);

      if (!isValid.valid) return isValid;

      this._registerPostValidator(option as never, 0);

      return { valid: true };
    }

    const configs: PostValidationConfig<
      KeyOf<Input>,
      Input,
      Output,
      CtxOptions,
      ErrorMetadata
    >[] = option;
    let reasons: string[] = [];

    configs.forEach((config, i) => {
      const isValid = this._isPostValidateSingleConfigOk(config, i);

      if (!isValid.valid) {
        const reason = isValid.reason!;

        if (Array.isArray(reason)) reasons = reasons.concat(reason);
        else reasons.push(reason);
      }
    });

    if (reasons.length) return { valid: false, reason: reasons };

    configs.forEach((config, i) => {
      const isValid = this._registerPostValidator(config, i);

      if (!isValid.valid) reasons.push(isValid.reason!);
    });

    return reasons.length ? { valid: false, reason: reasons } : { valid: true };
  }

  private _isTimestampsOptionOk(
    timestamps: ns.Options<Input, Output>["timestamps"],
  ) {
    const valid = false;

    const typeProveded = typeof timestamps;

    if (typeProveded === "boolean") return { valid: true };

    if (!isRecordLike(timestamps))
      return { valid, reason: "should be 'boolean' or 'non null object'" };

    if (!Object.keys(timestamps!).length)
      return { valid, reason: "cannot be an empty object" };

    const createdAt = timestamps?.createdAt as string;
    let updatedAt = timestamps?.updatedAt as string;

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

    const reservedKeys = [...this.props, ...this.virtuals] as string[];

    for (const key of [createdAt, updatedAt])
      if (key && reservedKeys?.includes(key))
        return { valid, reason: `'${key}' already belongs to your schema` };

    if (createdAt === updatedAt)
      return { valid, reason: "createdAt & updatedAt cannot be same" };

    return { valid: true };
  }
}

type InvalidPostValidateConfigMessage =
  | "default"
  | "validator-array-cannot-be-empty"
  | "validator-must-be-function"
  | "validator-must-be-function-or-array"
  | "fields-must-be-input-array"
  | "fields-array-must-contain-unique-values";

function getInvalidPostValidateConfigMessage(
  index?: number,
  message: InvalidPostValidateConfigMessage = "default",
  secondIndex?: number,
  thirdIndex?: number,
) {
  const hasIndex = typeof index === "number",
    hasSecondIndex = typeof secondIndex === "number",
    hasThirdIndex = typeof thirdIndex === "number";

  if (message === "default")
    return `Config${
      hasIndex ? ` at index ${index},` : ""
    } must be an object with keys "fields" and "validator" or an array of "PostValidateConfig"`;

  if (message === "fields-must-be-input-array")
    return `${
      hasIndex ? `Config at index ${index}:  ` : ""
    }"fields" must be an array of at least 2 input fields of your schema`;

  if (message === "fields-array-must-contain-unique-values")
    return `${
      hasIndex ? `Config at index ${index}:  ` : ""
    }"fields" array must contain unique values`;

  if (message === "validator-array-cannot-be-empty")
    return `${
      hasIndex ? `Config at index ${index}:  ` : ""
    }"validator" cannot be an empty array`;

  if (message === "validator-must-be-function")
    if (hasThirdIndex)
      return `${
        hasIndex ? `Config at index ${index}:  ` : ""
      }"validator" at index [${secondIndex}][${thirdIndex}] must be a function`;

  return `${hasIndex ? `Config at index ${index}:  ` : ""}"validator" ${
    hasSecondIndex ? `at index ${secondIndex} ` : ""
  }must be a function or array of functions`;
}

type InvalidOnSuccessConfigMessage =
  | "default"
  | "handler-must-be-function"
  | "config-handler-should-be-array-or-function"
  | "handler-array-cannot-be-empty"
  | "config-fields-must-be-array";
function getInvalidOnSuccessConfigMessage(
  index?: number,
  message: InvalidOnSuccessConfigMessage = "default",
  secondIndex?: number,
) {
  const hasIndex = typeof index === "number",
    hasSecondIndex = typeof secondIndex === "number";

  if (message === "default")
    return `${
      hasIndex ? `Config at index ${index}, must be` : "Expected"
    } a function, an object with keys "fields" and "handler" or an array of functions or objects`;

  if (message === "config-fields-must-be-array")
    return `${
      hasIndex ? `Config at index ${index}:  ` : ""
    }"fields" must be an array of at least 2 fields or virtuals of your schema`;

  if (message === "handler-array-cannot-be-empty")
    return `${
      hasIndex ? `Config at index ${index}:  ` : ""
    }"handler" cannot be an empty array`;

  if (hasSecondIndex)
    return `${
      hasIndex ? `Config at index ${index}:  ` : ""
    }"handler" at index ${secondIndex} must be a function`;

  return `${
    hasIndex ? `Config at index ${index}:  ` : ""
  }"handler" must be a function or array of functions`;
}

function getInvalidConfigMessageForRepeatedFields(
  index: number,
  existingIndex: number,
) {
  return `Config at index ${index} has the same fields as config at index ${existingIndex}`;
}
