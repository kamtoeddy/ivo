import {
  ErrorTool,
  OptionsTool,
  getKeysAsProps,
  isEqual,
  isFunction,
  isKeyOf,
  isObject,
  isOneOf,
  sort,
  sortKeys,
  toArray
} from '../utils'
import { ObjectType } from '../utils/types'
import {
  DefinitionRule,
  Context,
  ISchema as ns,
  StringKey,
  Summary,
  Validator,
  ALLOWED_OPTIONS,
  DEFINITION_RULES,
  CONSTANT_RULES,
  VIRTUAL_RULES
} from './types'

export const defaultOptions = {
  equalityDepth: 1,
  errors: 'silent',
  setMissingDefaultsOnUpdate: false,
  shouldUpdate: true,
  timestamps: false
} as ns.Options<any, any>

const lifeCycleRules: ns.LifeCycles[] = ['onDelete', 'onFailure', 'onSuccess']

export abstract class SchemaCore<Output, Input> {
  protected _definitions = {} as ns.Definitions_<Output, Input>
  protected _options: ns.Options<Output, Input>

  // contexts & values
  protected context: Context<Output, Input> = {} as Context<Output, Input>
  protected defaults: Partial<Output> = {}
  protected partialContext: Context<Output, Input> = {} as Context<
    Output,
    Input
  >
  protected values: Output = {} as Output

  // maps
  protected aliasToVirtualMap: ns.AliasToVirtualMap<Input> = {}
  protected dependencyMap: ns.DependencyMap<Input> = {}
  protected virtualToAliasMap: ns.AliasToVirtualMap<Input> = {}

  // props
  protected constants: StringKey<Input>[] = []
  protected dependents: StringKey<Input>[] = []
  protected laxProps: StringKey<Input>[] = []
  protected props: StringKey<Output>[] = []
  protected propsRequiredBy: StringKey<Input>[] = []
  protected readonlyProps: StringKey<Input>[] = []
  protected requiredProps: StringKey<Input>[] = []
  protected virtuals: StringKey<Input>[] = []

  // helpers
  protected optionsTool: OptionsTool

  // handlers
  protected globalDeleteHandlers: ns.Handler<Output>[] = []
  protected globalSuccessHandlers: ns.SuccessHandler<Output, Input>[] = []

  constructor(
    definitions: ns.Definitions_<Output, Input>,
    options: ns.Options<Output, Input> = defaultOptions as ns.Options<
      Output,
      Input
    >
  ) {
    this._definitions = definitions
    this._options = options

    this._checkPropDefinitions()
    this._checkOptions()

    this.optionsTool = new OptionsTool(this._makeTimestamps())
  }

  // < context methods >
  protected _getContext = () =>
    this._getFrozenCopy(sortKeys(this.context)) as Context<Output, Input>

  protected _getPartialContext = () => this._getFrozenCopy(this.partialContext)

  protected _initializeContexts = () => {
    this.context = { ...this.values } as Context<Output, Input>
    this.partialContext = {} as Context<Output, Input>
  }

  protected _updateContext = (updates: Partial<Input>) => {
    this.context = { ...this.context, ...updates }
  }

  protected _updatePartialContext = (updates: Partial<Input>) => {
    this.partialContext = { ...this.partialContext, ...updates }
  }
  // < context methods />

  // < dependency map utils >
  private _addDependencies = (
    prop: StringKey<Input>,
    dependsOn: StringKey<Input> | StringKey<Input>[]
  ) => {
    const _dependsOn = toArray(dependsOn) as StringKey<Input>[]

    for (const _prop of _dependsOn)
      if (this.dependencyMap[_prop]) this.dependencyMap[_prop]?.push(prop)
      else this.dependencyMap[_prop] = [prop]
  }

  protected _getDependencies = (prop: string) =>
    this.dependencyMap[prop as StringKey<Input>] ?? []

  protected _getAliasByVirtual = (prop: StringKey<Input>): string | undefined =>
    this.virtualToAliasMap[prop]

  protected _getVirtualByAlias = (
    alias: string
  ): StringKey<Input> | undefined => this.aliasToVirtualMap[alias]

  private _getCircularDependenciesOf = (a: StringKey<Input>) => {
    let circularDependencies: string[] = []

    const _dependsOn = toArray<StringKey<Input>>(
      this._getDefinition(a)?.dependsOn ?? []
    )

    for (const _prop of _dependsOn)
      circularDependencies = [
        ...circularDependencies,
        ...this._getCircularDependenciesOf_a_in_b(a, _prop)
      ]

    return sort(Array.from(new Set(circularDependencies)))
  }

  private _getCircularDependenciesOf_a_in_b = (
    a: StringKey<Input>,
    b: StringKey<Input>,
    visitedNodes: StringKey<Input>[] = []
  ) => {
    let circularDependencies: string[] = []

    if (!this._isDependentProp(b) || visitedNodes.includes(b)) return []

    visitedNodes.push(b)

    const _dependsOn = toArray<StringKey<Input>>(
      this._getDefinition(b)?.dependsOn ?? []
    )

    for (const _prop of _dependsOn) {
      if (_prop == a) circularDependencies.push(b)
      else if (this._isDependentProp(_prop))
        circularDependencies = [
          ...circularDependencies,
          ...this._getCircularDependenciesOf_a_in_b(a, _prop, visitedNodes)
        ]
    }

    return sort(Array.from(new Set(circularDependencies)))
  }
  // < dependency map utils />

  private _areHandlersOk = (
    _handlers: any,
    lifeCycle: ns.LifeCycles,
    global = false
  ) => {
    const reasons: string[] = []

    const handlers = toArray(_handlers)

    handlers.forEach((handler, i) => {
      if (!isFunction(handler))
        return reasons.push(
          `The '${lifeCycle}' handler @[${i}] is not a function`
        )

      if (!global) return

      if (lifeCycle == 'onDelete')
        return this.globalDeleteHandlers.push(handler as ns.Handler<Output>)

      if (lifeCycle == 'onSuccess')
        return this.globalSuccessHandlers.push(
          handler as ns.SuccessHandler<Output, Input>
        )
    })

    if (reasons.length) return { valid: false, reasons }

    return { valid: true }
  }

  protected _canInit = (prop: string) => {
    if (this._isDependentProp(prop)) return false
    if (this._isRequired(prop)) return true

    const { readonly } = this._getDefinition(prop)

    const shouldInit = this._getValueBy(prop, 'shouldInit')

    return (
      readonly === true &&
      isOneOf(shouldInit, [true, undefined]) &&
      !this._isRequiredBy(prop)
    )
  }

  protected _getFrozenCopy = <T>(data: T): Readonly<T> =>
    Object.freeze(Object.assign({}, data)) as Readonly<T>

  private _getInvalidRules = <K extends StringKey<Input>>(prop: K) => {
    const rulesProvided = getKeysAsProps(this._getDefinition(prop))

    return rulesProvided.filter(
      (r) => !DEFINITION_RULES.includes(r as DefinitionRule)
    )
  }

  protected _checkOptions = () => {
    const error = new ErrorTool({ message: 'Invalid Schema', statusCode: 500 })

    if (!isObject(this._options))
      error.add('schema options', 'Must be an object').throw()

    const options = Object.keys(this._options) as ns.OptionsKey<Output, Input>[]

    if (!options.length) error.add('schema options', 'Cannot be empty').throw()

    for (const option of options)
      if (!ALLOWED_OPTIONS.includes(option))
        error.add(option, 'Invalid option').throw()

    if (isKeyOf('equalityDepth', this._options)) {
      const typeProvided = typeof this._options.equalityDepth

      if (typeProvided == 'undefined')
        this._options.equalityDepth = defaultOptions.equalityDepth
      else if (typeProvided != 'number' || this._options.equalityDepth! < 0)
        error
          .add(
            'equalityDepth',
            "'equalityDepth' must be a number between 0 and +Infinity"
          )
          .throw()
    }

    if (isKeyOf('errors', this._options))
      if (!['silent', 'throw'].includes(this._options.errors!))
        error.add('errors', "should be 'silent' or 'throw'").throw()

    if (isKeyOf('onDelete', this._options)) {
      const isValid = this._areHandlersOk(
        this._options.onDelete,
        'onDelete',
        true
      )

      if (!isValid.valid) error.add('onDelete', isValid.reasons!).throw()
    }

    if (isKeyOf('onSuccess', this._options)) {
      const isValid = this._areHandlersOk(
        this._options.onSuccess,
        'onSuccess',
        true
      )

      if (!isValid.valid) error.add('onSuccess', isValid.reasons!).throw()
    }

    if (isKeyOf('setMissingDefaultsOnUpdate', this._options)) {
      const typeProvided = typeof this._options.setMissingDefaultsOnUpdate

      if (!['boolean', 'undefined'].includes(typeProvided))
        error
          .add(
            'setMissingDefaultsOnUpdate',
            "'setMissingDefaultsOnUpdate' should be a 'boolean'"
          )
          .throw()
    }

    if (isKeyOf('shouldUpdate', this._options)) {
      const typeProvided = typeof this._options.shouldUpdate

      if (!['boolean', 'function'].includes(typeProvided))
        error
          .add(
            'shouldUpdate',
            "'shouldUpdate' should either be a 'boolean' or a 'function'"
          )
          .throw()
    }

    if (isKeyOf('timestamps', this._options)) {
      const isValid = this._isTimestampsOptionOk()

      if (!isValid.valid) error.add('timestamps', isValid.reason!).throw()
    }
  }

  protected _checkPropDefinitions = () => {
    const error = new ErrorTool({ message: 'Invalid Schema', statusCode: 500 })

    if (!isObject(this._definitions)) error.throw()

    const props: string[] = Object.keys(this._definitions)

    if (!props.length)
      error.add('schema properties', 'Insufficient Schema properties').throw()

    for (const prop of props) {
      const isDefOk = this.__isPropDefinitionOk(prop)

      if (!isDefOk.valid) error.add(prop, isDefOk.reasons!)
    }

    // make sure every virtual property has atleast one dependency
    for (const prop of this.virtuals) {
      const dependencies = this._getDependencies(prop)

      if (!dependencies.length)
        error.add(
          prop,
          'A virtual property must have atleast one property that depends on it'
        )
    }

    // make sure aliases respect the second validation rules
    for (const [alias, prop] of Object.entries(this.aliasToVirtualMap)) {
      const isValid = this.__isVirtualAliasOk2(alias)

      if (!isValid.valid) error.add(prop, isValid.reason)
    }

    // make sure every virtual has atleast one dependency
    for (const prop of this.dependents) {
      const { dependsOn } = this._getDefinition(prop)

      const _dependsOn = toArray<StringKey<Input>>(dependsOn ?? [])

      if (_dependsOn.includes(prop))
        error.add(prop, 'A property cannot depend on itself')

      const dependsOnConstantProp = _dependsOn.some(this._isConstant)

      if (dependsOnConstantProp)
        error.add(prop, 'A property cannot depend on a constant property')

      // check against circular dependencies
      const circularRelationShips = this._getCircularDependenciesOf(prop)

      for (const _prop of circularRelationShips)
        error.add(prop, `Circular dependency identified with '${_prop}'`)

      // check against dependencies on invalid properties
      const invalidProps = _dependsOn.filter(
        (p) => !(this._isProp(p) || this._isVirtual(p))
      )

      for (const _prop of invalidProps)
        error.add(
          prop,
          `Cannot establish dependency with '${_prop}' as it is neither a property nor a virtual of your model`
        )
    }

    if (error.isPayloadLoaded) error.throw()
  }

  protected _getDefinition = (prop: string) =>
    this._definitions[prop as StringKey<Input>]!

  protected _getDefaultValue = async (prop: string) => {
    const _default = this._getDefinition(prop)?.default

    const value = isFunction(_default)
      ? await _default(this._getContext())
      : this.defaults[prop as StringKey<Output>]

    return isEqual(value, undefined)
      ? this.values[prop as StringKey<Output>]
      : value
  }

  protected _getConstantValue = async (prop: string) =>
    this._getValueBy(prop, 'value')

  protected _getValueBy = (
    prop: string,
    rule: DefinitionRule,
    extraCtx: ObjectType = {}
  ) => {
    const value = this._getDefinition(prop)?.[rule]

    return isFunction(value)
      ? value({ ...this._getContext(), ...extraCtx })
      : value
  }

  protected _getRequiredState = (
    prop: string,
    summary: Summary<Output, Input>
  ): [boolean, string] => {
    const { required } = this._getDefinition(prop)

    if (!required) return [false, '']

    const fallbackMessage = `'${prop}' is required!`

    if (!isFunction(required)) return [required, fallbackMessage]

    let results = required(summary)

    const datatype = typeof results

    if (datatype !== 'boolean' && !Array.isArray(results)) return [false, '']

    if (datatype === 'boolean')
      return [results as boolean, results ? fallbackMessage : '']

    results = results as [boolean, string]

    if (!results[1] || typeof results[1] != 'string')
      results[1] = fallbackMessage

    return results
  }

  protected _getHandlers = <T>(prop: string, lifeCycle: ns.LifeCycles) =>
    toArray((this._getDefinition(prop)?.[lifeCycle] ?? []) as any) as T[]

  protected _getValidator = <K extends StringKey<Input>>(prop: K) => {
    return this._getDefinition(prop)?.validator as
      | Validator<K, Output, Input>
      | undefined
  }

  protected _isDefaultable = (prop: string) => isKeyOf(prop, this.defaults)

  protected _isRuleInDefinition = (
    prop: string,
    rules: DefinitionRule | DefinitionRule[]
  ): boolean => {
    for (const _prop of toArray(rules))
      if (isKeyOf(_prop, this._getDefinition(prop))) return true

    return false
  }

  private __isConstantProp = (prop: string) => {
    const { constant, value } = this._getDefinition(prop)

    const valid = false

    if (constant !== true)
      return {
        valid,
        reason: "Constant properties must have constant as 'true'"
      }

    if (!this._isRuleInDefinition(prop, 'value'))
      return {
        valid,
        reason: 'Constant properties must have a value or setter'
      }

    if (isEqual(value, undefined))
      return {
        valid,
        reason: "Constant properties cannot have 'undefined' as value"
      }

    const unAcceptedRules = DEFINITION_RULES.filter(
      (rule) => !CONSTANT_RULES.includes(rule)
    )

    if (this._isRuleInDefinition(prop, unAcceptedRules))
      return {
        valid,
        reason:
          "Constant properties can only have ('constant' & 'value') or 'onDelete' | 'onSuccess'"
      }

    this.constants.push(prop as StringKey<Input>)

    return { valid: true }
  }

  protected _isConstant = (prop: string) =>
    this.constants.includes(prop as StringKey<Input>)

  private __isDependentProp = (prop: string) => {
    const {
      default: _default,
      dependent,
      dependsOn,
      shouldInit,
      readonly,
      resolver
    } = this._getDefinition(prop)

    const valid = false

    if (dependent !== true)
      return {
        valid,
        reason: "Dependent properties must have dependent as 'true'"
      }

    if (isEqual(_default, undefined))
      return {
        valid,
        reason: 'Dependent properties must have a default value'
      }

    if (isEqual(dependsOn, undefined) || !dependsOn?.length)
      return {
        valid,
        reason: 'Dependent properties must depend on atleast one property'
      }

    if (toArray(dependsOn).includes(prop as StringKey<Input>))
      return { valid, reason: 'A property cannot depend on itself' }

    if (isEqual(resolver, undefined))
      return {
        valid,
        reason: 'Dependent properties must have a resolver'
      }

    if (!isFunction(resolver))
      return {
        valid,
        reason: 'The resolver of a dependent property must be a function'
      }

    if (this._isRuleInDefinition(prop, 'validator'))
      return {
        valid,
        reason: 'Dependent properties cannot be validated'
      }

    if (this._isRuleInDefinition(prop, 'required'))
      return {
        valid,
        reason: 'Dependent properties cannot be required'
      }

    if (readonly === 'lax')
      return { valid, reason: "Dependent properties cannot be readonly 'lax'" }

    if (!isEqual(shouldInit, undefined))
      return {
        valid,
        reason: 'Dependent properties cannot have shouldInit rule'
      }

    if (this._isRuleInDefinition(prop, 'virtual'))
      return { valid, reason: 'Dependent properties cannot be virtual' }

    this.dependents.push(prop as StringKey<Input>)
    this._addDependencies(prop as StringKey<Input>, dependsOn)

    return { valid: true }
  }

  protected _isDependentProp = (prop: string) =>
    this.dependents.includes(prop as StringKey<Input>)

  protected _isLaxProp = (prop: string) =>
    this.laxProps.includes(prop as StringKey<Input>)

  protected _isProp = (prop: string) =>
    this.props.includes(prop as StringKey<Output>)

  private _isPropDefinitionObjectOk = (prop: string) => {
    const propDef = this._getDefinition(prop)

    const propertyTypeProvided = typeof propDef

    return !isObject(propDef)
      ? {
          reasons: [
            `Invalid property definition. Expected an object '{}' but received '${propertyTypeProvided}'`
          ],
          valid: false
        }
      : { valid: true }
  }

  private __isPropDefinitionOk = (prop: string) => {
    const isPopDefOk = this._isPropDefinitionObjectOk(prop)

    if (!isPopDefOk.valid) return isPopDefOk

    let reasons: string[] = []

    const invalidRulesProvided = this._getInvalidRules(prop as StringKey<Input>)

    if (invalidRulesProvided.length)
      for (const rule of invalidRulesProvided)
        reasons.push(`'${rule}' is not a valid rule`)

    if (this._isRuleInDefinition(prop, 'constant')) {
      const { valid, reason } = this.__isConstantProp(prop)

      if (!valid) reasons.push(reason!)
    } else if (this._isRuleInDefinition(prop, 'value'))
      reasons.push("'value' rule can only be used with constant properties")

    if (this._isRuleInDefinition(prop, 'dependent')) {
      const { valid, reason } = this.__isDependentProp(prop)

      if (!valid) reasons.push(reason!)
    } else if (this._isRuleInDefinition(prop, ['dependsOn', 'resolver']))
      reasons.push(
        'dependsOn & resolver rules can only belong to dependent properties'
      )

    if (this._isRuleInDefinition(prop, 'readonly')) {
      const { valid, reason } = this.__isReadonly(prop)

      if (!valid) reasons.push(reason!)
    }

    if (this._isRuleInDefinition(prop, 'required')) {
      const { required } = this._getDefinition(prop)

      const { valid, reason } =
        typeof required === 'function'
          ? this.__isRequiredBy(prop)
          : this.__isRequired(prop)

      if (!valid) reasons.push(reason!)
    }

    if (this._isRuleInDefinition(prop, 'virtual')) {
      const { valid, reason } = this.__isVirtual(prop)

      if (!valid) reasons.push(reason!)
    } else if (this._isRuleInDefinition(prop, 'sanitizer'))
      reasons.push("'sanitizer' is only valid on virtuals")

    if (this._isRuleInDefinition(prop, 'shouldInit')) {
      const { valid, reason } = this.__isShouldInitConfigOk(prop)

      if (!valid) reasons.push(reason!)
    }

    if (this._isRuleInDefinition(prop, 'shouldUpdate')) {
      const { valid, reason } = this.__isShouldUpdateConfigOk(prop)

      if (!valid) reasons.push(reason!)
    }

    if (
      this._isRuleInDefinition(prop, 'validator') &&
      !this._isValidatorOk(prop)
    )
      reasons.push('Invalid validator')

    if (
      this._isRuleInDefinition(prop, 'onFailure') &&
      !this._isRuleInDefinition(prop, 'validator')
    )
      reasons.push(
        "'onFailure' can only be used with properties that support and have validators"
      )

    // onDelete, onFailure, & onSuccess
    for (const rule of lifeCycleRules) {
      if (!this._isRuleInDefinition(prop, rule)) continue

      const isValid = this._areHandlersOk(this._getDefinition(prop)[rule], rule)

      if (!isValid.valid) reasons = reasons.concat(isValid.reasons!)
    }

    this._registerIfLax(prop)

    const hasDefaultRule = this._isRuleInDefinition(prop, 'default')

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
      )
    }

    const valid = reasons.length ? false : true

    if (valid && !this._isVirtual(prop)) {
      this.props.push(prop as StringKey<Output>)

      if (hasDefaultRule)
        this.defaults[prop as StringKey<Output>] = this._getValueBy(
          prop,
          'default'
        )
    }

    return { reasons, valid }
  }

  private __isReadonly = (prop: string) => {
    const {
      default: _default,
      dependent,
      readonly,
      required,
      shouldInit
    } = this._getDefinition(prop)

    const valid = false

    if (!isOneOf(readonly, [true, 'lax']))
      return {
        reason: "Readonly properties are either true | 'lax'",
        valid
      }

    if (
      this._isRuleInDefinition(prop, 'required') &&
      typeof required != 'function'
    )
      return {
        valid,
        reason:
          'Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule'
      }

    if (readonly === 'lax' && !isEqual(dependent, undefined))
      return { valid, reason: 'Readonly(lax) properties cannot be dependent' }

    if (
      (readonly === 'lax' || dependent === true || shouldInit === false) &&
      isEqual(_default, undefined)
    )
      return {
        valid,
        reason:
          'readonly properties must have a default value or a default setter'
      }

    if (readonly === 'lax' && !isEqual(shouldInit, undefined))
      return {
        valid,
        reason: 'Lax properties cannot have initialization blocked'
      }

    if (!isOneOf(readonly, [true, 'lax']))
      return {
        valid,
        reason: "Readonly properties have readonly true | 'lax'"
      }

    this.readonlyProps.push(prop as StringKey<Input>)

    return { valid: true }
  }

  private __isRequiredCommon = (prop: string) => {
    const valid = false

    if (this._isRuleInDefinition(prop, 'dependent'))
      return {
        valid,
        reason: 'Required properties cannot be dependent'
      }

    if (!this._isValidatorOk(prop))
      return { valid, reason: 'Required properties must have a validator' }

    return { valid: true }
  }

  private __isRequired = (prop: string) => {
    const { required } = this._getDefinition(prop)

    const valid = false

    if (required !== true)
      return {
        valid,
        reason: "Required properties must have required as 'true'"
      }

    if (this._isRuleInDefinition(prop, 'default'))
      return {
        valid,
        reason:
          'Strictly required properties cannot have a default value or setter'
      }

    if (this._isRuleInDefinition(prop, 'readonly'))
      return {
        valid,
        reason: 'Strictly required properties cannot be readonly'
      }

    if (this._isRuleInDefinition(prop, 'shouldInit'))
      return {
        valid,
        reason:
          'Strictly Required properties cannot have a initialization blocked'
      }

    const isRequiredCommon = this.__isRequiredCommon(prop)

    if (!isRequiredCommon.valid) return isRequiredCommon

    this.requiredProps.push(prop as StringKey<Input>)

    return { valid: true }
  }

  private __isRequiredBy = (prop: string) => {
    const { default: _default, required } = this._getDefinition(prop)

    const valid = false

    const requiredType = typeof required

    if (requiredType !== 'function')
      return {
        valid,
        reason: 'Callable required properties must have required as a function'
      }

    const hasVirtualRule = this._isRuleInDefinition(prop, 'virtual')

    if (isEqual(_default, undefined) && !hasVirtualRule)
      return {
        valid,
        reason:
          'Callable required properties must have a default value or setter'
      }

    if (!hasVirtualRule) {
      const isRequiredCommon = this.__isRequiredCommon(prop)

      if (!isRequiredCommon.valid) return isRequiredCommon
    }

    if (!this._isRequiredBy(prop))
      this.propsRequiredBy.push(prop as StringKey<Input>)

    return { valid: true }
  }

  private __isShouldInitConfigOk = (prop: string) => {
    const { shouldInit } = this._getDefinition(prop)

    const valid = false

    if (shouldInit !== false && !isFunction(shouldInit))
      return {
        valid,
        reason:
          "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean"
      }

    if (!this._isRuleInDefinition(prop, ['default', 'virtual']))
      return {
        valid,
        reason:
          'A property with initialization blocked must have a default value'
      }

    return { valid: true }
  }

  private __isShouldUpdateConfigOk = (prop: string) => {
    const { readonly, shouldInit, shouldUpdate } = this._getDefinition(prop)
    const valid = false

    if (shouldUpdate !== false && !isFunction(shouldUpdate))
      return {
        valid,
        reason:
          "'shouldUpdate' only accepts false or a function that returns a boolean"
      }

    if (shouldInit === false && shouldUpdate === false)
      return {
        valid,
        reason: "Both 'shouldInit' & 'shouldUpdate' cannot be 'false'"
      }

    if (shouldUpdate === false && !this._isRuleInDefinition(prop, 'virtual'))
      return {
        valid,
        reason: "Only 'Virtuals' are allowed to have 'shouldUpdate' as 'false'"
      }

    if (readonly === true && isEqual(shouldInit, undefined))
      return {
        valid,
        reason:
          "Cannot block the update of 'readonly' properties that do not have initialization('shouldInit') blocked. Either add 'shouldInit' or use readonly: 'lax'"
      }

    return { valid: true }
  }

  private __isVirtualRequiredBy = (prop: string) => {
    if (this._isRuleInDefinition(prop, 'shouldInit'))
      return {
        valid: false,
        reason: 'Required virtuals cannot have initialization blocked'
      }

    const isRequiredBy = this.__isRequiredBy(prop)

    if (!isRequiredBy.valid) return isRequiredBy

    return { valid: true }
  }

  private __isVirtual = (prop: string) => {
    const valid = false

    const { sanitizer, virtual } = this._getDefinition(prop)

    if (virtual !== true)
      return { valid, reason: "Virtuals must have virtual as 'true'" }

    if (!this._isValidatorOk(prop))
      return { valid, reason: 'Invalid validator' }

    if (this._isRuleInDefinition(prop, 'alias')) {
      const isValid = this.__isVirtualAliasOk(prop)

      if (!isValid.valid) return isValid
    }

    if (this._isRuleInDefinition(prop, 'sanitizer') && !isFunction(sanitizer))
      return { valid, reason: "'sanitizer' must be a function" }

    if (this._isRuleInDefinition(prop, 'required')) {
      const isValid = this.__isVirtualRequiredBy(prop)

      if (!isValid.valid) return isValid
    }

    const invalidVirtualRules = DEFINITION_RULES.filter(
      (rule) => !VIRTUAL_RULES.includes(rule)
    )

    if (this._isRuleInDefinition(prop, invalidVirtualRules))
      return {
        valid,
        reason: `Virtual properties can only have (${VIRTUAL_RULES.join(
          ', '
        )}) as rules`
      }

    this.virtuals.push(prop as StringKey<Input>)

    return { valid: true }
  }

  private __isVirtualAliasOk = (prop: string) => {
    const valid = false

    const { alias } = this._getDefinition(prop)

    if (typeof alias !== 'string' || !alias.length)
      return {
        valid,
        reason: 'An alias must be a string with atleast 1 character'
      }

    if (alias == prop)
      return {
        valid,
        reason: 'An alias cannot be the same as the virtual property'
      }

    const isTakenBy = this._getVirtualByAlias(alias)
    if (isTakenBy)
      return {
        valid,
        reason: `Sorry, alias provided '${alias}' already belongs to property '${isTakenBy}'`
      }

    this.aliasToVirtualMap[alias] = prop as StringKey<Input>
    this.virtualToAliasMap[prop] = alias as StringKey<Input>

    return { valid: true }
  }

  private __isVirtualAliasOk2 = (alias: string | StringKey<Input>) => {
    const prop = this._getVirtualByAlias(alias)!

    const invalidResponse = {
      valid: false,
      reason: `'${alias}' cannot be used as the alias of '${prop}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`
    } as any

    const isDependentOnVirtual = (
      this._getDependencies(prop) as string[]
    )?.includes(alias as StringKey<Input>)

    return (this._isProp(alias) && !isDependentOnVirtual) ||
      this._isVirtual(alias)
      ? invalidResponse
      : { valid: true }
  }

  private _isTimestampsOptionOk() {
    const { timestamps } = this._options,
      valid = false

    const typeProveded = typeof timestamps

    if (typeProveded === 'boolean') return { valid: true }

    if (!isObject(timestamps))
      return {
        valid,
        reason: "should be 'boolean' or 'non null object'"
      }

    if (!Object.keys(timestamps!).length)
      return { valid, reason: 'cannot be an empty object' }

    const { createdAt, updatedAt } = timestamps as {
      createdAt: ''
      updatedAt: ''
    }

    const reservedKeys = [...this.props, ...this.virtuals] as string[]

    for (const key of [createdAt, updatedAt])
      if (key && reservedKeys?.includes(key))
        return { valid, reason: `'${key}' already belongs to your schema` }

    if (typeof createdAt == 'string' && !createdAt.trim().length)
      return { valid, reason: "'createdAt' cannot be an empty string" }

    if (typeof updatedAt == 'string' && !updatedAt.trim().length)
      return { valid, reason: "'updatedAt' cannot be an empty string" }

    if (createdAt === updatedAt)
      return { valid, reason: 'createdAt & updatedAt cannot be same' }

    return { valid: true }
  }

  private _isValidatorOk = (prop: string) =>
    isFunction(this._getDefinition(prop)?.validator)

  private _makeTimestamps(): ns.PrivateOptions {
    const options = this._options

    if (!options) return { timestamps: { createdAt: '', updatedAt: '' } }

    const { timestamps } = options

    let createdAt = 'createdAt',
      updatedAt = 'updatedAt'

    if (!timestamps || timestamps === true) {
      const _timestamps = timestamps
        ? { createdAt, updatedAt }
        : { createdAt: '', updatedAt: '' }

      return { ...options, timestamps: _timestamps }
    }

    const custom_createdAt = timestamps?.createdAt
    const custom_updatedAt = timestamps?.updatedAt

    if (custom_createdAt && typeof custom_createdAt == 'string')
      createdAt = custom_createdAt.trim()

    if (custom_createdAt === false) createdAt = ''

    if (custom_updatedAt && typeof custom_updatedAt == 'string')
      updatedAt = custom_updatedAt.trim()

    if (custom_updatedAt === false) updatedAt = ''

    return { ...options, timestamps: { createdAt, updatedAt } }
  }

  private _registerIfLax = (prop: string) => {
    const {
      default: _default,
      readonly,
      shouldInit
    } = this._getDefinition(prop)

    // Lax properties must have a default value nor setter
    if (isEqual(_default, undefined)) return

    // Lax properties cannot be dependent
    if (this._isRuleInDefinition(prop, 'dependent')) return

    // Lax properties cannot be required
    if (this._isRuleInDefinition(prop, 'required')) return

    // Lax properties cannot be virtual
    if (this._isRuleInDefinition(prop, 'virtual')) return

    // only readonly(lax) are lax props &
    // Lax properties cannot have initialization blocked
    if (
      (this._isRuleInDefinition(prop, 'readonly') && readonly !== 'lax') ||
      (this._isRuleInDefinition(prop, 'shouldInit') &&
        typeof shouldInit != 'function')
    )
      return

    this.laxProps.push(prop as StringKey<Input>)
  }

  protected _isReadonly = (prop: string) =>
    this.readonlyProps.includes(prop as StringKey<Input>)

  protected _isRequired = (prop: string) =>
    this.requiredProps.includes(prop as StringKey<Input>)

  protected _isRequiredBy = (prop: string) =>
    this.propsRequiredBy.includes(prop as StringKey<Input>)

  protected _isVirtualAlias = (prop: string) => !!this.aliasToVirtualMap[prop]

  protected _isVirtual = (prop: string) =>
    this.virtuals.includes(prop as StringKey<Input>)

  protected _isVirtualInit = (prop: string, value: any = undefined) => {
    const isAlias = this._isVirtualAlias(prop)

    if (!this._isVirtual(prop) && !isAlias) return false

    const definitionName = isAlias ? this._getVirtualByAlias(prop)! : prop

    const { shouldInit } = this._getDefinition(definitionName)

    const extraCtx = isAlias ? { [definitionName]: value } : {}

    return (
      isEqual(shouldInit, undefined) ||
      this._getValueBy(definitionName, 'shouldInit', extraCtx)
    )
  }
}
