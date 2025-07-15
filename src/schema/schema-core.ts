/* eslint-disable @typescript-eslint/no-non-null-asserted-optional-chain */

import type { ObjectType } from '../utils';
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
  sortKeys,
  toArray,
} from '../utils';
import {
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  type Context,
  DEFINITION_RULES,
  type DefinitionRule,
  type ImmutableSummary,
  type KeyOf,
  LIFE_CYCLES,
  type MutableSummary,
  type NS as ns,
  type PartialContext,
  type PostValidationConfig,
  type PostValidator,
  type RealType,
  VIRTUAL_RULES,
} from './types';
import {
  cloneValue,
  DefaultErrorTool,
  type FieldError,
  type IErrorTool,
  type InputFieldError,
  isInputFieldError,
  makeFieldError,
  SchemaErrorTool,
  TimeStampTool,
} from './utils';

export {
  SchemaCore,
  defaultOptions,
  getInvalidPostValidateConfigMessage,
  getInvalidOnSuccessConfigMessage,
  getInvalidConfigMessageForRepeatedProperties,
};

const defaultOptions: ns.Options<unknown, unknown, unknown, never, never> = {
  equalityDepth: 1,
  ErrorTool: DefaultErrorTool as never,
  setMissingDefaultsOnUpdate: false,
  shouldUpdate: true,
  timestamps: false,
};

abstract class SchemaCore<
  Input,
  Output,
  CtxOptions extends ObjectType,
  ErrorTool extends IErrorTool<ObjectType>,
> {
  protected _definitions = {} as ns.Definitions_<Input, Output>;
  protected _options: ns.InternalOptions<Input, Output, CtxOptions, ErrorTool>;

  // contexts & values
  protected context = {} as Context<Input, Output, CtxOptions>;
  protected contextOptions: CtxOptions = {} as CtxOptions;

  protected defaults: Partial<Output> = {};
  protected partialContext = {} as PartialContext<Input, Output>;
  protected values: Output = {} as Output;

  // maps
  protected readonly aliasToVirtualMap: ns.AliasToVirtualMap<Input> = {};
  protected readonly dependencyMap: ns.DependencyMap<Input> = {};
  protected readonly propsToAllowedValuesMap = new Map<string, Set<unknown>>();
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
        unknown,
        CtxOptions
      >['validator'];
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
  protected readonly propToOnSuccessConfigIDMap = new Map<
    string,
    Set<string>
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

  // handlers
  protected readonly globalDeleteHandlers: ns.DeleteHandler<
    Output,
    CtxOptions
  >[] = [];
  protected readonly globalSuccessHandlers: ns.SuccessHandler<
    Input,
    Output,
    CtxOptions
  >[] = [];

  constructor(
    definitions: ns.Definitions_<Input, Output>,
    options: ns.Options<Input, Output, unknown> = defaultOptions as never,
  ) {
    this._checkPropDefinitions(definitions);
    this._checkOptions(options);

    this._definitions = sortKeys(definitions);
    this._options = sortKeys(
      Object.assign({}, defaultOptions, options),
    ) as never;

    if (!this._options.ErrorTool)
      this._options.ErrorTool = DefaultErrorTool as never;

    this.timestampTool = new TimeStampTool(this._options.timestamps);
  }

  protected _getContext(previousValues: Partial<Output> | null = null) {
    return this._getFrozenCopy({
      ...sortKeys(cloneValue(Object.assign({}, previousValues, this.context))),
      __getOptions__: () => this._getContextOptions(),
    }) as Context<Input, Output, CtxOptions>;
  }

  protected _getSummary({
    data,
    inputValues,
    isUpdate,
  }: {
    data: Partial<Output>;
    inputValues: Partial<RealType<Input>>;
    isUpdate: boolean;
  }) {
    const changes = isUpdate ? cloneValue(data) : null,
      previousValues = isUpdate ? cloneValue(this.values) : null,
      context = this._getContext(isUpdate ? previousValues : null),
      values = this._getFrozenCopy(
        cloneValue(
          isUpdate
            ? Object.assign({}, previousValues, this.values, data)
            : Object.assign({}, this.defaults, data),
        ),
      );

    return this._getFrozenCopy({
      changes,
      context,
      inputValues: cloneValue(inputValues),
      isUpdate,
      previousValues,
      values,
    }) as ImmutableSummary<Input, Output, CtxOptions>;
  }

  protected _getMutableSummary(props: {
    data: Partial<Output>;
    inputValues: Partial<RealType<Input>>;
    isUpdate: boolean;
  }) {
    const summary = this._getSummary(props);

    return this._getFrozenCopy(
      Object.assign({}, summary, {
        context: Object.assign({}, this._getContext(summary.previousValues), {
          __updateOptions__: this._updateContextOptions,
        }),
      }),
    ) as MutableSummary<Input, Output, CtxOptions>;
  }

  protected _getContextOptions = () => this._getFrozenCopy(this.contextOptions);

  protected _getPartialContext = () => this._getFrozenCopy(this.partialContext);

  protected _initializeImmutableContexts = () => {
    this.context = Object.assign({}, this.defaults, this.values) as never;
    this.partialContext = {} as PartialContext<Input, Output>;
  };

  protected _resetPartialContext = (updates: Partial<Output> = {}) => {
    for (const prop of getKeysAsProps(this.partialContext)) {
      if (this._isVirtual(prop)) continue;

      if (
        !isEqual(
          this.partialContext[prop],
          (updates as any)[prop],
          this._options.equalityDepth,
        )
      )
        delete this.partialContext[prop];
    }

    this.partialContext = Object.assign(
      {},
      this.partialContext,
      updates,
    ) as never;
  };

  protected _updateContext = (updates: Partial<Input>) => {
    Object.assign(this.context, updates);
  };

  protected _initializeContextOptions = (options: Partial<CtxOptions>) => {
    this.contextOptions = {} as CtxOptions;

    return this._updateContextOptions(options);
  };

  protected _updateContextOptions = (options: Partial<CtxOptions>) => {
    if (isRecordLike(options)) Object.assign(this.contextOptions, options);

    return this._getContextOptions();
  };

  protected _updatePartialContext = (updates: Partial<Input>) => {
    Object.assign(this.partialContext, updates);
  };

  protected _getAliasByVirtual = (prop: KeyOf<Input>): string | undefined =>
    this.virtualToAliasMap[prop];

  protected _getDependencies = (prop: string) =>
    this.dependencyMap[prop as KeyOf<Input>] ?? [];

  protected _getVirtualByAlias = (alias: string): KeyOf<Input> | undefined =>
    this.aliasToVirtualMap[alias];

  private _getCircularDependenciesOf = ({
    definitions,
    property,
    propertyB = property,
    visitedNodes = [],
  }: {
    definitions: ns.Definitions_<Input, Output>;
    property: KeyOf<Input>;
    propertyB?: KeyOf<Input>;
    visitedNodes?: KeyOf<Input>[];
  }) => {
    let circularDependencies: string[] = [];

    if (!this._isDependentProp(propertyB) || visitedNodes.includes(propertyB))
      return [];

    if (property !== propertyB) visitedNodes.push(propertyB);

    const _dependsOn = toArray<KeyOf<Input>>(
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
    definitions: ns.Definitions_<Input, Output>;
    property: KeyOf<Input>;
  }) => {
    const redundantParentProps: [string, string][] = [];

    if (!this._isDependentProp(property)) return [];

    const parentProps = toArray<KeyOf<Input>>(
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
    definitions: ns.Definitions_<Input, Output>;
    prop: KeyOf<Input>;
    parentProp: KeyOf<Input>;
  }): boolean => {
    if (!this._isDependentProp(prop)) return false;

    const parentProps = toArray<KeyOf<Input>>(
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

  private _areHandlersOk = (
    _handlers: unknown,
    lifeCycle: ns.LifeCycle,
    register: boolean,
  ) => {
    const reasons: string[] = [],
      handlers = toArray(_handlers);

    handlers.forEach((handler, i) => {
      if (!isFunctionLike(handler))
        return reasons.push(
          `The '${lifeCycle}' handler at index: ${i} is not a function`,
        );

      if (!register) return;

      if (lifeCycle === 'onDelete')
        return this.globalDeleteHandlers.push(
          handler as ns.DeleteHandler<Output, CtxOptions>,
        );
    });

    if (reasons.length) return { valid: false, reasons };

    return { valid: true };
  };

  protected _checkOptions = (options: ns.Options<Input, Output, unknown>) => {
    const error = new SchemaErrorTool();

    if (!isRecordLike(options))
      error.add('schema options', 'Must be an object').throw();

    const optionsProvided = Object.keys(options) as ns.OptionsKey<
      Output,
      Input,
      unknown,
      never
    >[];

    if (!optionsProvided.length) return;

    for (const option of optionsProvided)
      if (!ALLOWED_OPTIONS.includes(option))
        error.add(option, 'Invalid option').throw();

    if (isPropertyOf('equalityDepth', options)) {
      const typeProvided = typeof options.equalityDepth;

      if (
        !['number', 'undefined'].includes(typeProvided) ||
        options.equalityDepth! < 0
      )
        error
          .add(
            'equalityDepth',
            "'equalityDepth' must be a number between 0 and +Infinity",
          )
          .throw();
    }

    if (isPropertyOf('onDelete', options)) {
      const isValid = this._areHandlersOk(options.onDelete, 'onDelete', true);

      if (!isValid.valid) error.add('onDelete', isValid.reasons!).throw();
    }

    if (isPropertyOf('onSuccess', options)) {
      const isValid = this._isOnSuccessOptionOk(options.onSuccess as never);

      if (!isValid.valid) error.add('onSuccess', isValid.reason!).throw();
    }

    if (isPropertyOf('setMissingDefaultsOnUpdate', options)) {
      const typeProvided = typeof options.setMissingDefaultsOnUpdate;

      if (!['boolean', 'undefined'].includes(typeProvided))
        error
          .add(
            'setMissingDefaultsOnUpdate',
            "'setMissingDefaultsOnUpdate' should be a 'boolean'",
          )
          .throw();
    }

    if (isPropertyOf('shouldUpdate', options)) {
      const typeProvided = typeof options.shouldUpdate;

      if (!['boolean', 'function'].includes(typeProvided))
        error
          .add(
            'shouldUpdate',
            "'shouldUpdate' should either be a 'boolean' or a 'function'",
          )
          .throw();
    }

    if (isPropertyOf('postValidate', options)) {
      const isValid = this._isPostValidateOptionOk(
        options.postValidate as never,
      );

      if (!isValid.valid) error.add('postValidate', isValid.reason!).throw();
    }

    if (isPropertyOf('timestamps', options)) {
      const isValid = this._isTimestampsOptionOk(options.timestamps);

      if (!isValid.valid) error.add('timestamps', isValid.reason!).throw();
    }
  };

  protected _checkPropDefinitions = (
    definitions: ns.Definitions_<Input, Output>,
  ) => {
    const error = new SchemaErrorTool();

    if (!isRecordLike(definitions)) error.throw();

    const props = getKeysAsProps(definitions);

    if (!props.length)
      error.add('schema properties', 'Insufficient Schema properties').throw();

    for (const prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop, definitions[prop]);

      if (!isDefOk.valid) error.add(prop, isDefOk.reasons!);
    }

    // make sure every virtual property has at least one dependency
    for (const prop of this.virtuals) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length)
        error.add(
          prop,
          'A virtual property must have at least one property that depends on it',
        );
    }

    // make sure aliases respect the second validation rules
    for (const [alias, prop] of Object.entries(this.aliasToVirtualMap)) {
      const isValid = this.__isVirtualAliasOk2(alias);

      if (!isValid.valid) error.add(prop, isValid.reason);
    }

    // make sure every virtual has at least one dependency
    for (const prop of this.dependents) {
      // @ts-ignore: lol
      const definition = definitions?.[prop]!;

      const _dependsOn = toArray<KeyOf<Input>>(definition?.dependsOn ?? []);

      if (_dependsOn.includes(prop as never))
        error.add(prop, 'A property cannot depend on itself');

      const dependsOnConstantProp = _dependsOn.some(this._isConstant);

      if (dependsOnConstantProp)
        error.add(prop, 'A property cannot depend on a constant property');

      // check against dependencies on invalid properties
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

  protected _hasAllowedValues = (prop: string) =>
    this.propsToAllowedValuesMap.has(prop);

  protected _isConstant = (prop: string) =>
    this.constants.has(prop as KeyOf<Output>);

  protected _isDefaultable = (prop: string) =>
    isPropertyOf('default', this._getDefinition(prop));

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

  protected _getConstantValue = (prop: string) =>
    this._getValueBy(prop, 'value');

  protected _getDefinition = (prop: string) =>
    this._definitions[prop as KeyOf<Input>]!;

  protected _getDefaultValue = async (prop: string) => {
    const _default = this._getDefinition(prop)?.default;

    let value: any;

    try {
      value = isFunctionLike(_default)
        ? await Promise.try(() => _default(this._getContext()))
        : this.defaults[prop as KeyOf<Output>];
    } catch {
      value = null;
    }

    return isEqual(value, undefined)
      ? this.values[prop as KeyOf<Output>]
      : value;
  };

  protected _getFrozenCopy = <T>(data: T): Readonly<T> =>
    Object.freeze(Object.assign({}, data)) as Readonly<T>;

  protected _getHandlers = <T>(prop: string, lifeCycle: ns.LifeCycle) =>
    toArray((this._getDefinition(prop)?.[lifeCycle] ?? []) as never) as T[];

  protected _getRequiredState = async (
    prop: string,
    summary: MutableSummary<Input, Output>,
  ): Promise<[boolean, string | FieldError]> => {
    const { required } = this._getDefinition(prop);

    if (!required) return [false, ''];

    const fallbackMessage = `'${prop}' is required`;

    if (!isFunctionLike(required)) return [required, fallbackMessage];

    const results = await required(summary);
    const isBoolean = typeof results === 'boolean';

    if (!isBoolean && !Array.isArray(results)) return [false, ''];

    if (isBoolean) return [results as boolean, results ? fallbackMessage : ''];

    const [isRequired, message] = results as [
        boolean,
        string | InputFieldError,
      ],
      isString = typeof message === 'string';

    if (!isRequired || (!isString && !isInputFieldError(message)))
      return [isRequired, fallbackMessage];

    if (isString) return [true, message || fallbackMessage];

    const fieldError = makeFieldError(message, fallbackMessage);

    return [
      true,
      isPropertyOf('metadata', message)
        ? fieldError
        : ({ reason: fieldError.reason } as never),
    ];
  };

  protected _getValueBy = (
    prop: string,
    rule: DefinitionRule,
    extraCtx: ObjectType = {},
  ) => {
    const value = this._getDefinition(prop)?.[rule];

    return isFunctionLike(value)
      ? value(Object.assign({}, this._getContext(), extraCtx))
      : value;
  };

  private _isValidatorOk = (
    prop: string,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const { validator } = definition!,
      valid = false;

    if (Array.isArray(validator)) {
      if (validator.length !== 2)
        return {
          valid,
          reason: 'Validator array must contain exactly 2 functions',
        };

      const isPrimaryOk = isFunctionLike(validator[0]),
        isSecondaryOk = isFunctionLike(validator[1]);

      if (isPrimaryOk && isSecondaryOk) {
        this.propsWithSecondaryValidators.add(prop);

        return { valid: true };
      }

      if (!isPrimaryOk && isSecondaryOk)
        return { valid, reason: 'Validator at index 0 is invalid' };

      if (isPrimaryOk && !isSecondaryOk)
        return { valid, reason: 'Validator at index 1 is invalid' };

      return { valid, reason: 'Invalid validators' };
    }

    return { valid: isFunctionLike(validator), reason: 'Invalid validator' };
  };

  private __hasAllowedValues = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
    isRecursion = false,
  ): { valid: boolean; reason?: string } => {
    const { allow } = definition!,
      valid = false,
      isObject = isRecordLike(allow);

    if (isObject && !isRecursion) {
      const res = this.__hasAllowedValues(definition, true);

      if (!res.valid) return res;

      if (isPropertyOf('error', allow)) {
        const invalidErrorTypeMessage =
          'The "error" field of the allow rule can only accept a string, InputFieldError or an function that returns any of the above mentioned';

        const error = allow.error,
          isFunction = isFunctionLike(error),
          isString = typeof error === 'string';

        if (!isFunction && !isString && !isInputFieldError(error))
          return { valid, reason: invalidErrorTypeMessage };
      }

      if (Object.keys(allow).some((k) => !['error', 'values'].includes(k)))
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
      return { reason: 'Allowed values must be an array', valid };

    if (getUnique(allowedValues).length !== allowedValues.length)
      return {
        reason: 'Allowed values must be an array of unique values',
        valid,
      };

    if (allowedValues.length < 2)
      return { reason: 'Allowed values must have at least 2 values', valid };

    if (
      isPropertyOf('default', definition) &&
      !isOneOf(definition?.default, allowedValues as never)
    )
      return { reason: 'The default value must be an allowed value', valid };

    return { valid: true };
  };

  private __isConstantProp = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const { constant, value } = definition!;

    const valid = false;

    if (constant !== true)
      return {
        valid,
        reason: "Constant properties must have constant as 'true'",
      };

    if (!isPropertyOf('value', definition))
      return {
        valid,
        reason: 'Constant properties must have a value or setter',
      };

    if (isEqual(value, undefined))
      return {
        valid,
        reason: "Constant properties cannot have 'undefined' as value",
      };

    const unAcceptedRules = DEFINITION_RULES.filter(
      (rule) => !CONSTANT_RULES.includes(rule),
    );

    if (hasAnyOf(definition, unAcceptedRules))
      return {
        valid,
        reason:
          "Constant properties can only have ('constant' & 'value') or 'onDelete' | 'onSuccess'",
      };

    return { valid: true };
  };

  private __isDependentProp = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const {
      default: _default,
      dependsOn,
      shouldInit,
      readonly,
      resolver,
    } = definition!;

    const valid = false;

    if (isEqual(_default, undefined))
      return {
        valid,
        reason: 'Dependent properties must have a default value',
      };

    if (isEqual(dependsOn, undefined) || !dependsOn?.length)
      return {
        valid,
        reason: 'Dependent properties must depend on at least one property',
      };

    if (toArray(dependsOn).includes(prop as KeyOf<Input>))
      return { valid, reason: 'A property cannot depend on itself' };

    if (isEqual(resolver, undefined))
      return { valid, reason: 'Dependent properties must have a resolver' };

    if (!isFunctionLike(resolver))
      return {
        valid,
        reason: 'The resolver of a dependent property must be a function',
      };

    if (isPropertyOf('validator', definition))
      return { valid, reason: 'Dependent properties cannot be validated' };

    if (isPropertyOf('required', definition))
      return { valid, reason: 'Dependent properties cannot be required' };

    if (readonly === 'lax')
      return { valid, reason: "Dependent properties cannot be readonly 'lax'" };

    if (!isEqual(shouldInit, undefined))
      return {
        valid,
        reason: 'Dependent properties cannot have shouldInit rule',
      };

    if (isPropertyOf('virtual', definition))
      return { valid, reason: 'Dependent properties cannot be virtual' };

    return { valid: true };
  };

  private __isPropDefinitionOk = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const propertyTypeProvided = typeof definition;

    if (!isRecordLike(definition))
      return {
        reasons: [
          `Invalid property definition. Expected an object '{}' but received '${propertyTypeProvided}'`,
        ],
        valid: false,
      };

    let reasons: string[] = [];

    const invalidRulesProvided = getKeysAsProps(definition).filter(
      (r) => !DEFINITION_RULES.includes(r as DefinitionRule),
    );

    if (invalidRulesProvided.length)
      for (const rule of invalidRulesProvided)
        reasons.push(`'${rule}' is not a valid rule`);

    if (isPropertyOf('allow', definition)) {
      const { valid, reason } = this.__hasAllowedValues(definition);

      if (valid) {
        const allowedValues = Array.isArray(definition.allow)
          ? (definition.allow as never)
          : definition.allow!.values;

        this.propsToAllowedValuesMap.set(prop, new Set(allowedValues as never));
      } else reasons.push(reason!);
    }

    if (isPropertyOf('alias', definition)) {
      const { valid, reason } = this.__isVirtualAliasOk(prop, definition);

      if (valid) {
        const alias = definition?.alias!;

        this.aliasToVirtualMap[alias] = prop;
        this.virtualToAliasMap[prop] = alias as KeyOf<Input>;
      } else reasons.push(reason!);
    }

    if (isPropertyOf('constant', definition)) {
      const { valid, reason } = this.__isConstantProp(definition);

      valid ? this.constants.add(prop as never) : reasons.push(reason!);
    } else if (isPropertyOf('value', definition))
      reasons.push("'value' rule can only be used with constant properties");

    if (hasAnyOf(definition, ['dependsOn', 'resolver'])) {
      const { valid, reason } = this.__isDependentProp(prop, definition);

      if (valid) {
        this.dependents.add(prop as never);
        this._setDependencies(prop, definition.dependsOn!);
      } else reasons.push(reason!);
    }

    if (isPropertyOf('ignore', definition)) {
      const { valid, reason } = this.__isIgnoreConfigOk(definition);

      if (!valid) reasons.push(reason!);
    }

    if (isPropertyOf('readonly', definition)) {
      const { valid, reason } = this.__isReadonly(definition);

      valid ? this.readonlyProps.add(prop) : reasons.push(reason!);
    }

    if (isPropertyOf('required', definition)) {
      const { required } = definition;

      if (typeof required === 'function') {
        const { valid, reason } = this.__isRequiredBy(definition);

        valid ? this.propsRequiredBy.add(prop) : reasons.push(reason!);
      } else {
        const { valid, reason } = this.__isRequired(definition);

        valid ? this.requiredProps.add(prop) : reasons.push(reason!);
      }
    }

    if (isPropertyOf('virtual', definition)) {
      const { valid, reason } = this.__isVirtual(prop, definition);

      valid ? this.virtuals.add(prop) : reasons.push(reason!);
    } else if (isPropertyOf('sanitizer', definition))
      reasons.push("'sanitizer' is only valid on virtuals");

    if (isPropertyOf('shouldInit', definition)) {
      const { valid, reason } = this.__isShouldInitConfigOk(definition);

      if (!valid) reasons.push(reason!);
    }

    if (isPropertyOf('shouldUpdate', definition)) {
      const { valid, reason } = this.__isShouldUpdateConfigOk(definition);

      if (!valid) reasons.push(reason!);
    }

    const isValidatorOk = this._isValidatorOk(prop, definition);

    if (isPropertyOf('validator', definition) && !isValidatorOk.valid)
      reasons.push(isValidatorOk.reason!);

    if (
      isPropertyOf('onFailure', definition) &&
      !isPropertyOf('validator', definition)
    )
      reasons.push(
        "'onFailure' can only be used with properties that support and have validators",
      );

    // onDelete, onFailure, & onSuccess
    for (const rule of LIFE_CYCLES) {
      if (!isPropertyOf(rule, definition)) continue;

      const isValid = this._areHandlersOk(definition[rule], rule, false);

      if (!isValid.valid) reasons = reasons.concat(isValid.reasons!);
    }

    if (this.__isLax(definition)) this.laxProps.add(prop);

    const hasDefaultRule = isPropertyOf('default', definition);

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
        'A property should at least be readonly, required, or have a default value',
      );
    }

    const valid = reasons.length <= 0;

    if (valid && !this._isVirtual(prop)) {
      this.props.add(prop as never);

      if (hasDefaultRule && typeof definition.default !== 'function')
        this.defaults[prop as unknown as KeyOf<Output>] =
          definition.default as never;
    }

    return { reasons, valid };
  };

  private __isReadonly = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const {
      default: _default,
      readonly,
      required,
      shouldInit,
      shouldUpdate,
    } = definition!;

    const valid = false;

    if (!isOneOf(readonly, [true, 'lax'] as never))
      return {
        reason: "Readonly properties are either true | 'lax'",
        valid,
      };

    if (isPropertyOf('required', definition) && typeof required !== 'function')
      return {
        valid,
        reason:
          'Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule',
      };

    const hasDependentRule = isPropertyOf('dependsOn', definition);

    if (readonly === 'lax' && hasDependentRule)
      return { valid, reason: 'Readonly(lax) properties cannot be dependent' };

    if (
      (readonly === 'lax' || hasDependentRule || shouldInit === false) &&
      isEqual(_default, undefined)
    )
      return {
        valid,
        reason:
          'readonly properties must have a default value or a default setter',
      };

    if (readonly === 'lax' && !isEqual(shouldInit, undefined))
      return {
        valid,
        reason: 'Lax properties cannot have initialization blocked',
      };

    if (readonly === 'lax' && isEqual(shouldUpdate, false))
      return {
        valid,
        reason: 'Readonly(lax) properties cannot have updates strictly blocked',
      };

    return { valid: true };
  };

  private __isRequiredCommon = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const valid = false;

    if (isPropertyOf('dependsOn', definition))
      return { valid, reason: 'Required properties cannot be dependent' };

    if (
      !isPropertyOf('validator', definition) &&
      !isPropertyOf('allow', definition)
    )
      return { valid, reason: 'Required properties must have a validator' };

    return { valid: true };
  };

  private __isRequired = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const valid = false;

    if (definition?.required !== true)
      return {
        valid,
        reason: "Required properties must have required as 'true'",
      };

    if (isPropertyOf('default', definition))
      return {
        valid,
        reason:
          'Strictly required properties cannot have a default value or setter',
      };

    if (isPropertyOf('readonly', definition))
      return {
        valid,
        reason: 'Strictly required properties cannot be readonly',
      };

    if (isPropertyOf('shouldInit', definition))
      return {
        valid,
        reason:
          'Strictly Required properties cannot have a initialization blocked',
      };

    const isRequiredCommon = this.__isRequiredCommon(definition);

    if (!isRequiredCommon.valid) return isRequiredCommon;

    return { valid: true };
  };

  private __isRequiredBy = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const valid = false;

    const requiredType = typeof definition?.required;

    if (requiredType !== 'function')
      return {
        valid,
        reason: 'Callable required properties must have required as a function',
      };

    if (isPropertyOf('allow', definition))
      return {
        valid,
        reason:
          '"allow" rule is cannot be applied to conditionally required properties',
      };

    const hasVirtualRule = isPropertyOf('virtual', definition);

    if (isEqual(definition?.default, undefined) && !hasVirtualRule)
      return {
        valid,
        reason:
          'Callable required properties must have a default value or setter',
      };

    if (!hasVirtualRule) {
      const isRequiredCommon = this.__isRequiredCommon(definition);

      if (!isRequiredCommon.valid) return isRequiredCommon;
    }

    return { valid: true };
  };

  private __isIgnoreConfigOk = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const { ignore } = definition!;

    const valid = false;

    if (!isFunctionLike(ignore))
      return {
        valid,
        reason: '"ignore" must be a function that returns a boolean',
      };

    if (hasAnyOf(definition, ['shouldInit', 'shouldUpdate']))
      return {
        valid,
        reason: '"ignore" cannot be used with "shouldInit" or "shouldUpdate"',
      };

    if (!hasAnyOf(definition, ['default', 'virtual']))
      return {
        valid,
        reason:
          'For a property to be ignored, it must have a default value or be virtual',
      };

    return { valid: true };
  };

  private __isShouldInitConfigOk = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const { shouldInit } = definition!;

    const valid = false;

    if (shouldInit !== false && !isFunctionLike(shouldInit))
      return {
        valid,
        reason:
          "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean",
      };

    if (!hasAnyOf(definition, ['default', 'virtual']))
      return {
        valid,
        reason:
          'A property with initialization blocked must have a default value',
      };

    return { valid: true };
  };

  private __isShouldUpdateConfigOk = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const { readonly, shouldInit, shouldUpdate } = definition!;
    const valid = false;

    if (shouldUpdate !== false && !isFunctionLike(shouldUpdate))
      return {
        valid,
        reason:
          "'shouldUpdate' only accepts false or a function that returns a boolean",
      };

    if (shouldInit === false && shouldUpdate === false)
      return {
        valid,
        reason: "Both 'shouldInit' & 'shouldUpdate' cannot be 'false'",
      };

    if (readonly === true && isEqual(shouldInit, undefined))
      return {
        valid,
        reason:
          "Cannot block the update of 'readonly' properties that do not have initialization('shouldInit') blocked. Either add 'shouldInit' or use readonly: 'lax'",
      };

    return { valid: true };
  };

  private __isVirtualRequiredBy = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    if (isPropertyOf('shouldInit', definition))
      return {
        valid: false,
        reason: 'Required virtuals cannot have initialization blocked',
      };

    const isRequiredBy = this.__isRequiredBy(definition);

    if (!isRequiredBy.valid) return isRequiredBy;

    return { valid: true };
  };

  private __isVirtual = (
    prop: string,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const valid = false;
    const { sanitizer, virtual } = definition!;

    if (virtual !== true)
      return { valid, reason: "Virtuals must have virtual as 'true'" };

    const isValidatorOk = this._isValidatorOk(prop, definition);

    if (!isValidatorOk.valid) return { valid, reason: isValidatorOk.reason };

    if (isPropertyOf('sanitizer', definition) && !isFunctionLike(sanitizer))
      return { valid, reason: "'sanitizer' must be a function" };

    if (isPropertyOf('required', definition)) {
      const isValid = this.__isVirtualRequiredBy(definition);

      if (!isValid.valid) return isValid;
    }

    const invalidVirtualRules = DEFINITION_RULES.filter(
      (rule) => !VIRTUAL_RULES.includes(rule),
    );

    if (hasAnyOf(definition, invalidVirtualRules))
      return {
        valid,
        reason: `Virtual properties can only have (${VIRTUAL_RULES.join(
          ', ',
        )}) as rules`,
      };

    return { valid: true };
  };

  private __isVirtualAliasOk = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const valid = false;

    const { alias } = definition!;

    if (!isPropertyOf('virtual', definition))
      return { valid, reason: 'Only virtual properties can have aliases' };

    if (typeof alias !== 'string' || !alias.length)
      return {
        valid,
        reason: 'An alias must be a string with at least 1 character',
      };

    if (alias === prop)
      return {
        valid,
        reason: 'An alias cannot be the same as the virtual property',
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
      this._getDependencies(prop) as string[]
    )?.includes(alias as KeyOf<Input>);

    return (this._isProp(alias) && !isDependentOnVirtual) ||
      this._isVirtual(alias)
      ? invalidResponse
      : ({ valid: true } as typeof invalidResponse);
  };

  private __isLax = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>],
  ) => {
    const { readonly, shouldInit } = definition!;

    // Lax properties must have a default value nor setter
    if (isEqual(definition?.default, undefined)) return false;

    // Lax properties cannot be dependent
    if (isPropertyOf('dependent', definition)) return false;

    // Lax properties cannot be required
    if (isPropertyOf('required', definition)) return false;

    // Lax properties cannot be virtual
    if (isPropertyOf('virtual', definition)) return false;

    // only readonly(lax) are lax props &
    // Lax properties cannot have initialization blocked
    if (
      (isPropertyOf('readonly', definition) && readonly !== 'lax') ||
      (isPropertyOf('shouldInit', definition) &&
        typeof shouldInit !== 'function')
    )
      return false;

    return true;
  };

  private _isPostValidateSingleConfigOk(value: unknown, index?: number) {
    const valid = false;

    if (
      !value ||
      !isPropertyOf('properties', value) ||
      !isPropertyOf('validator', value) ||
      Object.keys(value).length > 2
    )
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(index),
      };

    // @ts-ignore: lol
    if (!Array.isArray(value.properties))
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          'properties-must-be-input-array',
        ),
      };

    // @ts-ignore: lol
    const properties = getUnique(value.properties);

    if (properties.length < 2)
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          'properties-must-be-input-array',
        ),
      };

    // @ts-ignore: lol
    if (properties.length < value.properties.length)
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          'properties-array-must-contain-unique-values',
        ),
      };

    const reasons: string[] = [];

    for (const prop of properties)
      if (!this._isInputProp(prop) && !this._isVirtual(prop)) {
        if (index !== undefined)
          reasons.push(
            `Config at index ${index}: "${prop}" cannot be post-validated`,
          );
        else reasons.push(`"${prop}" cannot be post-validated`);
      }

    if (reasons.length) return { valid, reason: reasons };

    // @ts-ignore: lol
    if (Array.isArray(value.validator)) {
      // @ts-ignore: lol
      const validators = value.validator as Exclude<
        PostValidationConfig<
          KeyOf<Input>,
          Input,
          Output,
          unknown,
          CtxOptions
        >['validator'],
        PostValidator<KeyOf<Input>, Input, Output, unknown, CtxOptions>
      >;

      if (!validators.length)
        return {
          valid,
          reason: getInvalidPostValidateConfigMessage(
            index,
            'validator-array-cannot-be-empty',
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
                  'validator-must-be-function',
                  i,
                  i2,
                ),
              );
          });

          if (!validator.length)
            return reasons.push(
              getInvalidPostValidateConfigMessage(
                index,
                'validator-array-cannot-be-empty',
                i,
              ),
            );
        } else if (!isFunctionLike(validator))
          reasons.push(
            getInvalidPostValidateConfigMessage(
              index,
              'validator-must-be-function-or-array',
              i,
            ),
          );
      });

      if (reasons.length) return { valid, reason: reasons };

      return { valid: true };
    }

    // @ts-ignore: lol
    if (!isFunctionLike(value.validator))
      return {
        valid,
        reason: getInvalidPostValidateConfigMessage(
          index,
          'validator-must-be-function',
        ),
      };

    return { valid: true };
  }

  private __isOnSuccessSingleConfigOk(value: unknown, index?: number) {
    if (isFunctionLike(value)) return { valid: true };

    const valid = false;

    if (
      !value ||
      !isPropertyOf('properties', value) ||
      !isPropertyOf('handler', value) ||
      Object.keys(value).length > 2
    )
      return { valid, reason: getInvalidOnSuccessConfigMessage(index) };

    // @ts-ignore: lol
    if (!Array.isArray(value.properties))
      return {
        valid,
        reason: getInvalidOnSuccessConfigMessage(
          index,
          'config-properties-must-be-array',
        ),
      };

    // @ts-ignore: lol
    const properties = getUnique(value.properties);

    if (properties.length < 2)
      return {
        valid,
        reason: getInvalidOnSuccessConfigMessage(
          index,
          'config-properties-must-be-array',
        ),
      };

    const reasons: string[] = [];

    for (const prop of properties)
      if (!this._isProp(prop) && !this._isVirtual(prop)) {
        reasons.push(
          `${
            index !== undefined ? `Config at index ${index}: ` : ''
          }"${prop}" is not a property or virtual on your schema`,
        );
      }

    if (reasons.length) return { valid, reason: reasons };

    // @ts-ignore: lol
    if (Array.isArray(value.handler)) {
      // @ts-ignore: lol
      const handlers = value.handler as unknown[];

      if (!handlers.length)
        return {
          valid,
          reason: getInvalidOnSuccessConfigMessage(
            index,
            'handler-array-cannot-be-empty',
          ),
        };

      const reasons: string[] = [];

      handlers.forEach((handler, i) => {
        if (!isFunctionLike(handler))
          reasons.push(
            getInvalidOnSuccessConfigMessage(
              index,
              'handler-must-be-function',
              i,
            ),
          );
      });

      if (reasons.length) return { valid, reason: reasons };

      return { valid: true };
    }

    // @ts-ignore: lol
    if (!isFunctionLike(value.handler))
      return {
        valid,
        reason: getInvalidOnSuccessConfigMessage(
          index,
          'config-handler-should-be-array-or-function',
        ),
      };

    return { valid: true };
  }

  private _registerPostValidator(
    {
      properties,
      validator,
    }: PostValidationConfig<KeyOf<Input>, Input, Output, unknown, CtxOptions>,
    index: number,
  ) {
    const sortedProps = sort(properties as any),
      sortedPropsId = sortedProps.toString();

    const config = this.postValidationConfigMap.get(sortedPropsId);

    if (config)
      return {
        valid: false,
        reason: getInvalidConfigMessageForRepeatedProperties(
          index,
          config.index,
        ),
      };

    for (const prop of sortedProps) {
      const setOfIDs =
        this.propToPostValidationConfigIDsMap.get(prop as any) ?? new Set();

      setOfIDs.add(sortedPropsId);

      this.propToPostValidationConfigIDsMap.set(prop as any, setOfIDs);
    }

    this.postValidationConfigMap.set(sortedPropsId, {
      index,
      validators: validator,
    });

    return { valid: true };
  }

  private _registerSuccessConfig(
    config: ns.OnSuccessConfig<Input, Output, CtxOptions>,
    isFunction: boolean,
    index: number,
  ) {
    if (isFunction) {
      this.globalSuccessHandlers.push(
        config as ns.SuccessHandler<Input, Output, CtxOptions>,
      );

      return { valid: true };
    }

    const { properties, handler } = config as ns.OnSuccessConfigObject<
      Input,
      Output,
      CtxOptions
    >;

    const sortedProps = sort(properties as any),
      sortedPropsId = sortedProps.toString();

    const existingConfig = this.onSuccessConfigMap.get(sortedPropsId);

    if (existingConfig)
      return {
        valid: false,
        reason: getInvalidConfigMessageForRepeatedProperties(
          index,
          existingConfig.index,
        ),
      };

    for (const prop of sortedProps) {
      const setOfIDs =
        this.propToOnSuccessConfigIDMap.get(prop as any) ?? new Set();

      setOfIDs.add(sortedPropsId);

      this.propToOnSuccessConfigIDMap.set(prop as any, setOfIDs);
    }

    this.onSuccessConfigMap.set(sortedPropsId, {
      index,
      handlers: toArray(handler) as any,
    });

    return { valid: true };
  }

  private _isOnSuccessOptionOk(
    option: ns.Options<Input, Output, unknown, never, CtxOptions>['onSuccess'],
  ) {
    const valid = false,
      isFunction = isFunctionLike(option),
      isObject = isRecordLike(option);

    if (!option || (!Array.isArray(option) && !isFunction && !isObject))
      return { valid, reason: getInvalidOnSuccessConfigMessage() };

    if (isFunction) {
      this._registerSuccessConfig(option, isFunction, 0);

      return { valid: true };
    }

    if (isObject) {
      const isValid = this.__isOnSuccessSingleConfigOk(option);

      if (!isValid.valid) return isValid;

      this._registerSuccessConfig(option, false, 0);

      return { valid: true };
    }

    const configs: ns.OnSuccessConfig<Input, Output, CtxOptions>[] = option;
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
      const isValid = this._registerSuccessConfig(
        config,
        isFunctionLike(config),
        i,
      );

      if (!isValid.valid) reasons.push(isValid.reason!);
    });

    return reasons.length ? { valid: false, reason: reasons } : { valid: true };
  }

  private _isPostValidateOptionOk(
    option: ns.Options<
      Input,
      Output,
      unknown,
      never,
      CtxOptions
    >['postValidate'],
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
      unknown,
      CtxOptions
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
    timestamps: ns.Options<Input, Output, unknown>['timestamps'],
  ) {
    const valid = false;

    const typeProveded = typeof timestamps;

    if (typeProveded === 'boolean') return { valid: true };

    if (!isRecordLike(timestamps))
      return { valid, reason: "should be 'boolean' or 'non null object'" };

    if (!Object.keys(timestamps!).length)
      return { valid, reason: 'cannot be an empty object' };

    const createdAt = timestamps?.createdAt as string;
    let updatedAt = timestamps?.updatedAt as string;

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
          reason: "'updatedAt' can only accept properties 'key' and 'nullable'",
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
      return { valid, reason: 'createdAt & updatedAt cannot be same' };

    return { valid: true };
  }
}

type InvalidPostValidateConfigMessage =
  | 'default'
  | 'validator-array-cannot-be-empty'
  | 'validator-must-be-function'
  | 'validator-must-be-function-or-array'
  | 'properties-must-be-input-array'
  | 'properties-array-must-contain-unique-values';

function getInvalidPostValidateConfigMessage(
  index?: number,
  message: InvalidPostValidateConfigMessage = 'default',
  secondIndex?: number,
  thirdIndex?: number,
) {
  const hasIndex = typeof index === 'number',
    hasSecondIndex = typeof secondIndex === 'number',
    hasThirdIndex = typeof thirdIndex === 'number';

  if (message === 'default')
    return `Config${
      hasIndex ? ` at index ${index},` : ''
    } must be an object with keys "properties" and "validator" or an array of "PostValidateConfig"`;

  if (message === 'properties-must-be-input-array')
    return `${
      hasIndex ? `Config at index ${index}:  ` : ''
    }"properties" must be an array of at least 2 input properties of your schema`;

  if (message === 'properties-array-must-contain-unique-values')
    return `${
      hasIndex ? `Config at index ${index}:  ` : ''
    }"properties" array must contain unique values`;

  if (message === 'validator-array-cannot-be-empty')
    return `${
      hasIndex ? `Config at index ${index}:  ` : ''
    }"validator" cannot be an empty array`;

  if (message === 'validator-must-be-function')
    if (hasThirdIndex)
      return `${
        hasIndex ? `Config at index ${index}:  ` : ''
      }"validator" at index [${secondIndex}][${thirdIndex}] must be a function`;

  return `${hasIndex ? `Config at index ${index}:  ` : ''}"validator" ${
    hasSecondIndex ? `at index ${secondIndex} ` : ''
  }must be a function or array of functions`;
}

type InvalidOnSuccessConfigMessage =
  | 'default'
  | 'handler-must-be-function'
  | 'config-handler-should-be-array-or-function'
  | 'handler-array-cannot-be-empty'
  | 'config-properties-must-be-array';
function getInvalidOnSuccessConfigMessage(
  index?: number,
  message: InvalidOnSuccessConfigMessage = 'default',
  secondIndex?: number,
) {
  const hasIndex = typeof index === 'number',
    hasSecondIndex = typeof secondIndex === 'number';

  if (message === 'default')
    return `${
      hasIndex ? `Config at index ${index}, must be` : 'Expected'
    } a function, an object with keys "properties" and "handler" or an array of functions or objects`;

  if (message === 'config-properties-must-be-array')
    return `${
      hasIndex ? `Config at index ${index}:  ` : ''
    }"properties" must be an array of at least 2 properties or virtuals of your schema`;

  if (message === 'handler-array-cannot-be-empty')
    return `${
      hasIndex ? `Config at index ${index}:  ` : ''
    }"handler" cannot be an empty array`;

  if (hasSecondIndex)
    return `${
      hasIndex ? `Config at index ${index}:  ` : ''
    }"handler" at index ${secondIndex} must be a function`;

  return `${
    hasIndex ? `Config at index ${index}:  ` : ''
  }"handler" must be a function or array of functions`;
}

function getInvalidConfigMessageForRepeatedProperties(
  index: number,
  existingIndex: number,
) {
  return `Config at index ${index} has the same properties as config at index ${existingIndex}`;
}
