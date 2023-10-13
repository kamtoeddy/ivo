/* eslint-disable @typescript-eslint/no-non-null-asserted-optional-chain */

import {
  getKeysAsProps,
  hasAnyOf,
  isEqual,
  isFunction,
  isPropertyOf,
  isObject,
  isOneOf,
  sort,
  sortKeys,
  toArray
} from '../utils';
import {
  DefaultErrorTool,
  IErrorTool,
  SchemaErrorTool,
  TimeStampTool
} from './utils';
import { ObjectType } from '../utils';
import {
  DefinitionRule,
  Context,
  NS as ns,
  KeyOf,
  Summary,
  Validator,
  ALLOWED_OPTIONS,
  DEFINITION_RULES,
  CONSTANT_RULES,
  VIRTUAL_RULES,
  LIFE_CYCLES,
  PartialContext
} from './types';

export const defaultOptions = {
  equalityDepth: 1,
  ErrorTool: DefaultErrorTool,
  errors: 'silent',
  setMissingDefaultsOnUpdate: false,
  shouldUpdate: true,
  timestamps: false
} as ns.Options<any, any, any>;

export abstract class SchemaCore<
  Input,
  Output,
  ErrorTool extends IErrorTool<any>,
  CtxOptions extends ObjectType = {}
> {
  protected _definitions = {} as ns.Definitions_<Input, Output>;
  protected _options: ns.InternalOptions<Input, Output, ErrorTool, CtxOptions>;

  // contexts & values
  protected context: Context<Input, Output, CtxOptions> = {} as Context<
    Input,
    Output,
    CtxOptions
  >;
  protected contextOptions: CtxOptions = {} as CtxOptions;

  protected defaults: Partial<Output> = {};
  protected partialContext: PartialContext<Input, Output> =
    {} as PartialContext<Input, Output>;
  protected values: Output = {} as Output;

  // maps
  protected readonly aliasToVirtualMap: ns.AliasToVirtualMap<Input> = {};
  protected readonly dependencyMap: ns.DependencyMap<Input> = {};
  protected readonly virtualToAliasMap: ns.AliasToVirtualMap<Input> = {};

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
  protected readonly globalDeleteHandlers: ns.Handler<Output, CtxOptions>[] =
    [];
  protected readonly globalSuccessHandlers: ns.SuccessHandler<
    Input,
    Output,
    CtxOptions
  >[] = [];

  constructor(
    definitions: ns.Definitions_<Input, Output>,
    options: ns.Options<Input, Output, any> = defaultOptions as ns.Options<
      Input,
      Output,
      ErrorTool
    >
  ) {
    this._checkPropDefinitions(definitions);
    this._checkOptions(options);

    this._definitions = sortKeys(definitions);
    this._options = sortKeys({ ...defaultOptions, ...options }) as any;

    if (!this._options.ErrorTool)
      this._options.ErrorTool = DefaultErrorTool as any;

    this.timestampTool = new TimeStampTool(this._options.timestamps);
  }

  // < context methods >
  protected _getContext(previousValues: Partial<Output> | null = null) {
    const values = { ...previousValues, ...this.context } as any;

    return this._getFrozenCopy({
      ...sortKeys(values),
      __getOptions__: () => this._getContextOptions()
    }) as Context<Input, Output, CtxOptions>;
  }

  protected _getContextOptions = () => this._getFrozenCopy(this.contextOptions);

  protected _getPartialContext = () => this._getFrozenCopy(this.partialContext);

  protected _initializeContexts = () => {
    this.context = { ...this.defaults, ...this.values } as any;
    this.partialContext = {} as PartialContext<Input, Output>;
  };

  protected _updateContext = (updates: Partial<Input>) => {
    this.context = { ...this.context, ...updates };
  };

  protected _updateContextOptions = (options: Partial<CtxOptions>) => {
    if (isObject(options))
      this.contextOptions = { ...this.contextOptions, ...options };

    return this._getContextOptions();
  };

  protected _updatePartialContext = (updates: Partial<Input>) => {
    this.partialContext = { ...this.partialContext, ...updates };
  };
  // < context methods />

  // < dependency map utils >
  private _addDependencies = (
    prop: KeyOf<Input>,
    dependsOn: KeyOf<Input> | KeyOf<Input>[]
  ) => {
    const _dependsOn = toArray(dependsOn) as KeyOf<Input>[];

    for (const _prop of _dependsOn)
      if (this.dependencyMap[_prop]) this.dependencyMap[_prop]?.push(prop);
      else this.dependencyMap[_prop] = [prop];
  };

  protected _getDependencies = (prop: string) =>
    this.dependencyMap[prop as KeyOf<Input>] ?? [];

  protected _getAliasByVirtual = (prop: KeyOf<Input>): string | undefined =>
    this.virtualToAliasMap[prop];

  protected _getVirtualByAlias = (alias: string): KeyOf<Input> | undefined =>
    this.aliasToVirtualMap[alias];

  private _getCircularDependenciesOf = ({
    definitions,
    property,
    propertyB = property,
    visitedNodes = []
  }: {
    definitions: ns.Definitions_<Input, Output>;
    property: KeyOf<Input>;
    propertyB?: KeyOf<Input>;
    visitedNodes?: KeyOf<Input>[];
  }) => {
    let circularDependencies: string[] = [];

    if (!this._isDependentProp(propertyB) || visitedNodes.includes(propertyB))
      return [];

    if (property != propertyB) visitedNodes.push(propertyB);

    const _dependsOn = toArray<KeyOf<Input>>(
      definitions?.[propertyB]?.dependsOn ?? []
    );

    for (const _prop of _dependsOn)
      if (_prop == property) circularDependencies.push(propertyB);
      else if (this._isDependentProp(_prop))
        circularDependencies = [
          ...circularDependencies,
          ...this._getCircularDependenciesOf({
            definitions,
            property,
            propertyB: _prop,
            visitedNodes
          })
        ];

    return sort(Array.from(new Set(circularDependencies)));
  };

  // < dependency map utils />

  private _areHandlersOk = (
    _handlers: any,
    lifeCycle: ns.LifeCycle,
    global = false
  ) => {
    const reasons: string[] = [];

    const handlers = toArray(_handlers);

    handlers.forEach((handler, i) => {
      if (!isFunction(handler))
        return reasons.push(
          `The '${lifeCycle}' handler @[${i}] is not a function`
        );

      if (!global) return;

      if (lifeCycle == 'onDelete')
        return this.globalDeleteHandlers.push(
          handler as ns.Handler<Output, CtxOptions>
        );

      if (lifeCycle == 'onSuccess')
        return this.globalSuccessHandlers.push(
          handler as ns.SuccessHandler<Input, Output>
        );
    });

    if (reasons.length) return { valid: false, reasons };

    return { valid: true };
  };

  protected _canInit = (prop: string) => {
    if (this._isDependentProp(prop)) return false;
    if (this._isRequired(prop)) return true;

    const { readonly } = this._getDefinition(prop);

    const shouldInit = this._getValueBy(prop, 'shouldInit');

    return (
      readonly === true &&
      isOneOf(shouldInit, [true, undefined]) &&
      !this._isRequiredBy(prop)
    );
  };

  protected _getFrozenCopy = <T>(data: T): Readonly<T> =>
    Object.freeze(Object.assign({}, data)) as Readonly<T>;

  protected _checkOptions = (options: ns.Options<Input, Output, any>) => {
    const error = new SchemaErrorTool();

    if (!isObject(options))
      error.add('schema options', 'Must be an object').throw();

    const optionsProvided = Object.keys(options) as ns.OptionsKey<
      Output,
      Input,
      any
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
            "'equalityDepth' must be a number between 0 and +Infinity"
          )
          .throw();
    }

    if (isPropertyOf('errors', options))
      if (!['silent', 'throw'].includes(options.errors!))
        error.add('errors', "should be 'silent' or 'throw'").throw();

    if (isPropertyOf('onDelete', options)) {
      const isValid = this._areHandlersOk(options.onDelete, 'onDelete', true);

      if (!isValid.valid) error.add('onDelete', isValid.reasons!).throw();
    }

    if (isPropertyOf('onSuccess', options)) {
      const isValid = this._areHandlersOk(options.onSuccess, 'onSuccess', true);

      if (!isValid.valid) error.add('onSuccess', isValid.reasons!).throw();
    }

    if (isPropertyOf('setMissingDefaultsOnUpdate', options)) {
      const typeProvided = typeof options.setMissingDefaultsOnUpdate;

      if (!['boolean', 'undefined'].includes(typeProvided))
        error
          .add(
            'setMissingDefaultsOnUpdate',
            "'setMissingDefaultsOnUpdate' should be a 'boolean'"
          )
          .throw();
    }

    if (isPropertyOf('shouldUpdate', options)) {
      const typeProvided = typeof options.shouldUpdate;

      if (!['boolean', 'function'].includes(typeProvided))
        error
          .add(
            'shouldUpdate',
            "'shouldUpdate' should either be a 'boolean' or a 'function'"
          )
          .throw();
    }

    if (isPropertyOf('timestamps', options)) {
      const isValid = this._isTimestampsOptionOk(options.timestamps);

      if (!isValid.valid) error.add('timestamps', isValid.reason!).throw();
    }
  };

  protected _checkPropDefinitions = (
    definitions: ns.Definitions_<Input, Output>
  ) => {
    const error = new SchemaErrorTool();

    if (!isObject(definitions)) error.throw();

    const props = getKeysAsProps(definitions);

    if (!props.length)
      error.add('schema properties', 'Insufficient Schema properties').throw();

    for (const prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop, definitions[prop]);

      if (!isDefOk.valid) error.add(prop, isDefOk.reasons!);
    }

    // make sure every virtual property has atleast one dependency
    for (const prop of this.virtuals) {
      const dependencies = this._getDependencies(prop);

      if (!dependencies.length)
        error.add(
          prop,
          'A virtual property must have atleast one property that depends on it'
        );
    }

    // make sure aliases respect the second validation rules
    for (const [alias, prop] of Object.entries(this.aliasToVirtualMap)) {
      const isValid = this.__isVirtualAliasOk2(alias);

      if (!isValid.valid) error.add(prop, isValid.reason);
    }

    // make sure every virtual has atleast one dependency
    for (const prop of this.dependents) {
      const definition = (definitions as any)?.[prop]!;

      const _dependsOn = toArray<KeyOf<Input>>(definition?.dependsOn ?? []);

      if (_dependsOn.includes(prop as any))
        error.add(prop, 'A property cannot depend on itself');

      const dependsOnConstantProp = _dependsOn.some(this._isConstant);

      if (dependsOnConstantProp)
        error.add(prop, 'A property cannot depend on a constant property');

      // check against circular dependencies
      const circularRelationShips = this._getCircularDependenciesOf({
        definitions,
        property: prop
      } as any);

      for (const _prop of circularRelationShips)
        error.add(prop, `Circular dependency identified with '${_prop}'`);

      // check against dependencies on invalid properties
      const invalidProps = _dependsOn.filter(
        (p) => !(this._isProp(p) || this._isVirtual(p))
      );

      for (const _prop of invalidProps)
        error.add(
          prop,
          `Cannot establish dependency with '${_prop}' as it is neither a property nor a virtual of your model`
        );
    }

    if (error.isPayloadLoaded) error.throw();
  };

  protected _getDefinition = (prop: string) =>
    this._definitions[prop as KeyOf<Input>]!;

  protected _getDefaultValue = async (prop: string) => {
    const _default = this._getDefinition(prop)?.default;

    const value = isFunction(_default)
      ? await _default(this._getContext())
      : this.defaults[prop as KeyOf<Output>];

    return isEqual(value, undefined)
      ? this.values[prop as KeyOf<Output>]
      : value;
  };

  protected _getConstantValue = async (prop: string) =>
    this._getValueBy(prop, 'value');

  protected _getValueBy = (
    prop: string,
    rule: DefinitionRule,
    extraCtx: ObjectType = {}
  ) => {
    const value = this._getDefinition(prop)?.[rule];

    return isFunction(value)
      ? value({ ...this._getContext(), ...extraCtx })
      : value;
  };

  protected _getRequiredState = (
    prop: string,
    summary: Summary<Input, Output>
  ): [boolean, string] => {
    const { required } = this._getDefinition(prop);

    if (!required) return [false, ''];

    const fallbackMessage = `'${prop}' is required!`;

    if (!isFunction(required)) return [required, fallbackMessage];

    let results = required(summary);

    const datatype = typeof results;

    if (datatype !== 'boolean' && !Array.isArray(results)) return [false, ''];

    if (datatype === 'boolean')
      return [results as boolean, results ? fallbackMessage : ''];

    results = results as [boolean, string];

    if (!results[1] || typeof results[1] != 'string')
      results[1] = fallbackMessage;

    return results;
  };

  protected _getHandlers = <T>(prop: string, lifeCycle: ns.LifeCycle) =>
    toArray((this._getDefinition(prop)?.[lifeCycle] ?? []) as any) as T[];

  protected _getValidator = <K extends KeyOf<Input>>(prop: K) => {
    return this._getDefinition(prop)?.validator as
      | Validator<K, Input, Output>
      | undefined;
  };

  protected _isDefaultable = (prop: string) =>
    isPropertyOf(prop, this.defaults);

  protected _isRuleInDefinition = (
    prop: string,
    rules: DefinitionRule | DefinitionRule[]
  ): boolean => {
    for (const _prop of toArray(rules))
      if (isPropertyOf(_prop, this._getDefinition(prop))) return true;

    return false;
  };

  private __isConstantProp = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const { constant, value } = definition!;

    const valid = false;

    if (constant !== true)
      return {
        valid,
        reason: "Constant properties must have constant as 'true'"
      };

    if (!isPropertyOf('value', definition))
      return {
        valid,
        reason: 'Constant properties must have a value or setter'
      };

    if (isEqual(value, undefined))
      return {
        valid,
        reason: "Constant properties cannot have 'undefined' as value"
      };

    const unAcceptedRules = DEFINITION_RULES.filter(
      (rule) => !CONSTANT_RULES.includes(rule)
    );

    if (hasAnyOf(definition, unAcceptedRules))
      return {
        valid,
        reason:
          "Constant properties can only have ('constant' & 'value') or 'onDelete' | 'onSuccess'"
      };

    return { valid: true };
  };

  protected _isConstant = (prop: string) =>
    this.constants.has(prop as KeyOf<Output>);

  private __isDependentProp = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const {
      default: _default,
      dependent,
      dependsOn,
      shouldInit,
      readonly,
      resolver
    } = definition!;

    const valid = false;

    if (!isEqual(dependent, undefined) && dependent !== true)
      return {
        valid,
        reason: "Dependent properties must have dependent as 'true'"
      };

    if (isEqual(_default, undefined))
      return {
        valid,
        reason: 'Dependent properties must have a default value'
      };

    if (isEqual(dependsOn, undefined) || !dependsOn?.length)
      return {
        valid,
        reason: 'Dependent properties must depend on atleast one property'
      };

    if (toArray(dependsOn).includes(prop as KeyOf<Input>))
      return { valid, reason: 'A property cannot depend on itself' };

    if (isEqual(resolver, undefined))
      return {
        valid,
        reason: 'Dependent properties must have a resolver'
      };

    if (!isFunction(resolver))
      return {
        valid,
        reason: 'The resolver of a dependent property must be a function'
      };

    if (isPropertyOf('validator', definition))
      return {
        valid,
        reason: 'Dependent properties cannot be validated'
      };

    if (isPropertyOf('required', definition))
      return {
        valid,
        reason: 'Dependent properties cannot be required'
      };

    if (readonly === 'lax')
      return { valid, reason: "Dependent properties cannot be readonly 'lax'" };

    if (!isEqual(shouldInit, undefined))
      return {
        valid,
        reason: 'Dependent properties cannot have shouldInit rule'
      };

    if (isPropertyOf('virtual', definition))
      return { valid, reason: 'Dependent properties cannot be virtual' };

    return { valid: true };
  };

  protected _isDependentProp = (prop: string) =>
    this.dependents.has(prop as KeyOf<Output>);

  protected _isLaxProp = (prop: string) =>
    this.laxProps.has(prop as KeyOf<Input>);

  protected _isProp = (prop: string) => this.props.has(prop as KeyOf<Output>);

  private __isPropDefinitionOk = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const propertyTypeProvided = typeof definition;

    if (!isObject(definition))
      return {
        reasons: [
          `Invalid property definition. Expected an object '{}' but received '${propertyTypeProvided}'`
        ],
        valid: false
      };

    let reasons: string[] = [];

    const invalidRulesProvided = getKeysAsProps(definition).filter(
      (r) => !DEFINITION_RULES.includes(r as DefinitionRule)
    );

    if (invalidRulesProvided.length)
      for (const rule of invalidRulesProvided)
        reasons.push(`'${rule}' is not a valid rule`);

    if (isPropertyOf('constant', definition)) {
      const { valid, reason } = this.__isConstantProp(definition);

      valid ? this.constants.add(prop as any) : reasons.push(reason!);
    } else if (isPropertyOf('value', definition))
      reasons.push("'value' rule can only be used with constant properties");

    if (hasAnyOf(definition, ['dependent', 'dependsOn'])) {
      const { valid, reason } = this.__isDependentProp(prop, definition);

      if (valid) {
        this.dependents.add(prop as any);
        this._addDependencies(prop, definition.dependsOn!);
      } else reasons.push(reason!);
    }

    if (isPropertyOf('readonly', definition)) {
      const { valid, reason } = this.__isReadonly(definition);

      valid ? this.readonlyProps.add(prop) : reasons.push(reason!);
    }

    if (isPropertyOf('required', definition)) {
      const { required } = definition;

      if (typeof required == 'function') {
        const { valid, reason } = this.__isRequiredBy(definition);

        valid ? this.propsRequiredBy.add(prop) : reasons.push(reason!);
      } else {
        const { valid, reason } = this.__isRequired(definition);

        valid ? this.requiredProps.add(prop) : reasons.push(reason!);
      }
    }

    if (isPropertyOf('virtual', definition)) {
      const { valid, reason } = this.__isVirtual(definition);

      valid ? this.virtuals.add(prop) : reasons.push(reason!);
    } else if (isPropertyOf('sanitizer', definition))
      reasons.push("'sanitizer' is only valid on virtuals");

    if (isPropertyOf('alias', definition)) {
      const { valid, reason } = this.__isVirtualAliasOk(prop, definition);

      if (valid) {
        const alias = definition?.alias!;

        this.aliasToVirtualMap[alias] = prop;
        this.virtualToAliasMap[prop] = alias as KeyOf<Input>;
      } else reasons.push(reason!);
    }

    if (isPropertyOf('shouldInit', definition)) {
      const { valid, reason } = this.__isShouldInitConfigOk(definition);

      if (!valid) reasons.push(reason!);
    }

    if (isPropertyOf('shouldUpdate', definition)) {
      const { valid, reason } = this.__isShouldUpdateConfigOk(definition);

      if (!valid) reasons.push(reason!);
    }

    if (
      isPropertyOf('validator', definition) &&
      !this._isValidatorOk(definition)
    )
      reasons.push('Invalid validator');

    if (
      isPropertyOf('onFailure', definition) &&
      !isPropertyOf('validator', definition)
    )
      reasons.push(
        "'onFailure' can only be used with properties that support and have validators"
      );

    // onDelete, onFailure, & onSuccess
    for (const rule of LIFE_CYCLES) {
      if (!isPropertyOf(rule, definition)) continue;

      const isValid = this._areHandlersOk(definition[rule], rule);

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
        'A property should at least be readonly, required, or have a default value'
      );
    }

    const valid = reasons.length <= 0;

    if (valid && !this._isVirtual(prop)) {
      this.props.add(prop as any);

      if (hasDefaultRule)
        this.defaults[prop as unknown as KeyOf<Output>] =
          typeof definition.default == 'function'
            ? undefined
            : definition.default;
    }

    return { reasons, valid };
  };

  private __isReadonly = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const {
      default: _default,
      dependent,
      readonly,
      required,
      shouldInit
    } = definition!;

    const valid = false;

    if (!isOneOf(readonly, [true, 'lax']))
      return {
        reason: "Readonly properties are either true | 'lax'",
        valid
      };

    if (isPropertyOf('required', definition) && typeof required != 'function')
      return {
        valid,
        reason:
          'Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule'
      };

    if (readonly === 'lax' && !isEqual(dependent, undefined))
      return { valid, reason: 'Readonly(lax) properties cannot be dependent' };

    if (
      (readonly === 'lax' || dependent === true || shouldInit === false) &&
      isEqual(_default, undefined)
    )
      return {
        valid,
        reason:
          'readonly properties must have a default value or a default setter'
      };

    if (readonly === 'lax' && !isEqual(shouldInit, undefined))
      return {
        valid,
        reason: 'Lax properties cannot have initialization blocked'
      };

    if (!isOneOf(readonly, [true, 'lax']))
      return {
        valid,
        reason: "Readonly properties have readonly true | 'lax'"
      };

    return { valid: true };
  };

  private __isRequiredCommon = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const valid = false;

    if (isPropertyOf('dependent', definition))
      return {
        valid,
        reason: 'Required properties cannot be dependent'
      };

    if (!this._isValidatorOk(definition))
      return { valid, reason: 'Required properties must have a validator' };

    return { valid: true };
  };

  private __isRequired = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const valid = false;

    if (definition?.required !== true)
      return {
        valid,
        reason: "Required properties must have required as 'true'"
      };

    if (isPropertyOf('default', definition))
      return {
        valid,
        reason:
          'Strictly required properties cannot have a default value or setter'
      };

    if (isPropertyOf('readonly', definition))
      return {
        valid,
        reason: 'Strictly required properties cannot be readonly'
      };

    if (isPropertyOf('shouldInit', definition))
      return {
        valid,
        reason:
          'Strictly Required properties cannot have a initialization blocked'
      };

    const isRequiredCommon = this.__isRequiredCommon(definition);

    if (!isRequiredCommon.valid) return isRequiredCommon;

    return { valid: true };
  };

  private __isRequiredBy = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const valid = false;

    const requiredType = typeof definition?.required;

    if (requiredType !== 'function')
      return {
        valid,
        reason: 'Callable required properties must have required as a function'
      };

    const hasVirtualRule = isPropertyOf('virtual', definition);

    if (isEqual(definition?.default, undefined) && !hasVirtualRule)
      return {
        valid,
        reason:
          'Callable required properties must have a default value or setter'
      };

    if (!hasVirtualRule) {
      const isRequiredCommon = this.__isRequiredCommon(definition);

      if (!isRequiredCommon.valid) return isRequiredCommon;
    }

    return { valid: true };
  };

  private __isShouldInitConfigOk = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const { shouldInit } = definition!;

    const valid = false;

    if (shouldInit !== false && !isFunction(shouldInit))
      return {
        valid,
        reason:
          "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean"
      };

    if (!hasAnyOf(definition, ['default', 'virtual']))
      return {
        valid,
        reason:
          'A property with initialization blocked must have a default value'
      };

    return { valid: true };
  };

  private __isShouldUpdateConfigOk = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const { readonly, shouldInit, shouldUpdate } = definition!;
    const valid = false;

    if (shouldUpdate !== false && !isFunction(shouldUpdate))
      return {
        valid,
        reason:
          "'shouldUpdate' only accepts false or a function that returns a boolean"
      };

    if (shouldInit === false && shouldUpdate === false)
      return {
        valid,
        reason: "Both 'shouldInit' & 'shouldUpdate' cannot be 'false'"
      };

    if (shouldUpdate === false && !isPropertyOf('virtual', definition))
      return {
        valid,
        reason: "Only 'Virtuals' are allowed to have 'shouldUpdate' as 'false'"
      };

    if (readonly === true && isEqual(shouldInit, undefined))
      return {
        valid,
        reason:
          "Cannot block the update of 'readonly' properties that do not have initialization('shouldInit') blocked. Either add 'shouldInit' or use readonly: 'lax'"
      };

    return { valid: true };
  };

  private __isVirtualRequiredBy = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    if (isPropertyOf('shouldInit', definition))
      return {
        valid: false,
        reason: 'Required virtuals cannot have initialization blocked'
      };

    const isRequiredBy = this.__isRequiredBy(definition);

    if (!isRequiredBy.valid) return isRequiredBy;

    return { valid: true };
  };

  private __isVirtual = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const valid = false;

    const { sanitizer, virtual } = definition!;

    if (virtual !== true)
      return { valid, reason: "Virtuals must have virtual as 'true'" };

    if (!this._isValidatorOk(definition))
      return { valid, reason: 'Invalid validator' };

    if (isPropertyOf('sanitizer', definition) && !isFunction(sanitizer))
      return { valid, reason: "'sanitizer' must be a function" };

    if (isPropertyOf('required', definition)) {
      const isValid = this.__isVirtualRequiredBy(definition);

      if (!isValid.valid) return isValid;
    }

    const invalidVirtualRules = DEFINITION_RULES.filter(
      (rule) => !VIRTUAL_RULES.includes(rule)
    );

    if (hasAnyOf(definition, invalidVirtualRules))
      return {
        valid,
        reason: `Virtual properties can only have (${VIRTUAL_RULES.join(
          ', '
        )}) as rules`
      };

    return { valid: true };
  };

  private __isVirtualAliasOk = (
    prop: KeyOf<Input>,
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => {
    const valid = false;

    const { alias } = definition!;

    if (!isPropertyOf('virtual', definition))
      return {
        valid,
        reason: 'Only virtual properties can have aliases'
      };

    if (typeof alias !== 'string' || !alias.length)
      return {
        valid,
        reason: 'An alias must be a string with atleast 1 character'
      };

    if (alias == prop)
      return {
        valid,
        reason: 'An alias cannot be the same as the virtual property'
      };

    const isTakenBy = this._getVirtualByAlias(alias);
    if (isTakenBy)
      return {
        valid,
        reason: `Sorry, alias provided '${alias}' already belongs to property '${isTakenBy}'`
      };

    return { valid: true };
  };

  private __isVirtualAliasOk2 = (alias: string | KeyOf<Input>) => {
    const prop = this._getVirtualByAlias(alias)!;

    const invalidResponse = {
      valid: false,
      reason: `'${alias}' cannot be used as the alias of '${prop}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`
    } as any;

    const isDependentOnVirtual = (
      this._getDependencies(prop) as string[]
    )?.includes(alias as KeyOf<Input>);

    return (this._isProp(alias) && !isDependentOnVirtual) ||
      this._isVirtual(alias)
      ? invalidResponse
      : { valid: true };
  };

  private _isTimestampsOptionOk(
    timestamps: ns.Options<Input, Output, any>['timestamps']
  ) {
    const valid = false;

    const typeProveded = typeof timestamps;

    if (typeProveded === 'boolean') return { valid: true };

    if (!isObject(timestamps))
      return {
        valid,
        reason: "should be 'boolean' or 'non null object'"
      };

    if (!Object.keys(timestamps!).length)
      return { valid, reason: 'cannot be an empty object' };

    const { createdAt, updatedAt } = timestamps as {
      createdAt: '';
      updatedAt: '';
    };

    const reservedKeys = [...this.props, ...this.virtuals] as string[];

    for (const key of [createdAt, updatedAt])
      if (key && reservedKeys?.includes(key))
        return { valid, reason: `'${key}' already belongs to your schema` };

    if (typeof createdAt == 'string' && !createdAt.trim().length)
      return { valid, reason: "'createdAt' cannot be an empty string" };

    if (typeof updatedAt == 'string' && !updatedAt.trim().length)
      return { valid, reason: "'updatedAt' cannot be an empty string" };

    if (createdAt === updatedAt)
      return { valid, reason: 'createdAt & updatedAt cannot be same' };

    return { valid: true };
  }

  private _isValidatorOk = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
  ) => isFunction(definition?.validator);

  private __isLax = (
    definition: ns.Definitions_<Input, Output>[KeyOf<Input>]
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
        typeof shouldInit != 'function')
    )
      return false;

    return true;
  };

  protected _isReadonly = (prop: string) =>
    this.readonlyProps.has(prop as KeyOf<Input>);

  protected _isRequired = (prop: string) =>
    this.requiredProps.has(prop as KeyOf<Input>);

  protected _isRequiredBy = (prop: string) =>
    this.propsRequiredBy.has(prop as KeyOf<Input>);

  protected _isVirtualAlias = (prop: string) => !!this.aliasToVirtualMap[prop];

  protected _isVirtual = (prop: string) =>
    this.virtuals.has(prop as KeyOf<Input>);

  protected _isVirtualInit = (prop: string, value: any = undefined) => {
    const isAlias = this._isVirtualAlias(prop);

    if (!this._isVirtual(prop) && !isAlias) return false;

    const definitionName = isAlias ? this._getVirtualByAlias(prop)! : prop;

    const { shouldInit } = this._getDefinition(definitionName);

    const extraCtx = isAlias ? { [definitionName]: value } : {};

    return (
      isEqual(shouldInit, undefined) ||
      this._getValueBy(definitionName, 'shouldInit', extraCtx)
    );
  };
}
