import {
  makeResponse,
  getKeysAsProps,
  isEqual,
  isPropertyOf,
  sort,
  sortKeys,
  toArray,
  isObject
} from '../utils'
import {
  Context,
  InternalValidatorResponse,
  LIFE_CYCLES,
  ISchema as ns,
  RealType,
  ResponseInputObject,
  KeyOf,
  Summary,
  ValidationResponse
} from './types'
import { VALIDATION_ERRORS, ErrorTool } from './utils'
import { defaultOptions, SchemaCore } from './schema-core'

export { Model, ModelTool, Schema }

const validationFailedResponse = {
  metadata: null,
  reasons: ['validation failed'],
  valid: false
} as ResponseInputObject<any, any, any>

class Schema<
  Output extends RealType<Output>,
  Input extends RealType<Input> = Output,
  Aliases = {}
> extends SchemaCore<Output, Input> {
  constructor(
    definitions: ns.Definitions<Output, Input, Aliases>,
    options: ns.Options<Output, Input> = defaultOptions
  ) {
    super(definitions as any as ns.Definitions_<Output, Input>, options)
  }

  get definitions() {
    return this._definitions as any as ns.Definitions<Input, Output, Aliases>
  }

  get options() {
    return this._options
  }

  get reservedKeys() {
    const props = [...this.props, ...this.virtuals] as string[]

    const { createdAt, updatedAt } = this.timestampTool.getKeys()

    if (createdAt) props.push(createdAt)
    if (updatedAt) props.push(updatedAt)

    return sort(props)
  }

  extend = <
    ExtendedOutput extends RealType<ExtendedOutput>,
    ExtendedInput extends RealType<ExtendedInput> = ExtendedOutput,
    Aliases = {}
  >(
    definitions: Partial<
      ns.Definitions<ExtendedOutput, ExtendedInput, Aliases>
    >,
    options: ns.ExtensionOptions<
      Output,
      Input,
      ExtendedOutput,
      ExtendedInput
    > = {}
  ) => {
    const { remove = [], useParentOptions = true, ...rest } = options

    const _definitions = { ...this.definitions } as unknown as ns.Definitions<
      ExtendedOutput,
      ExtendedInput,
      Aliases
    >

    toArray(remove ?? [])?.forEach(
      (prop) => delete (_definitions as any)?.[prop]
    )

    const options_ = {} as ns.Options<ExtendedOutput, ExtendedInput>

    if (useParentOptions)
      getKeysAsProps(this.options)
        .filter(
          (prop) => ![...LIFE_CYCLES, 'shouldUpdate'].includes(prop as any)
        )
        .forEach((prop) => {
          options_[prop] = this.options[prop] as any
        })

    return new Schema<ExtendedOutput, ExtendedInput, Aliases>(
      { ..._definitions, ...definitions },
      { ...options_, ...rest }
    )
  }

  getModel = () => new Model(new ModelTool<Output, Input, Aliases>(this))
}

class ModelTool<
  Output extends RealType<Output>,
  Input extends RealType<Input> = Output,
  Aliases = {}
> extends SchemaCore<Output, Input> {
  private _regeneratedProps: KeyOf<Output>[] = []

  constructor(schema: Schema<Output, Input, Aliases>) {
    super(schema.definitions as any, schema.options)
  }

  private async _generateConstants() {
    const data = {} as Partial<Output>

    await Promise.allSettled(
      this.constants.map(async (prop) => {
        data[prop] = await this._getConstantValue(prop)

        const validCtxUpdate = { [prop]: data[prop] as any } as any

        this._updatePartialContext(validCtxUpdate)
        return this._updateContext(validCtxUpdate)
      })
    )

    return data
  }

  private _getSummary(data: Partial<Output>, isUpdate = false) {
    const changes = isUpdate ? data : null,
      context = this._getContext(),
      operation = isUpdate ? 'update' : 'creation',
      previousValues = isUpdate ? this._getFrozenCopy(this.values) : null,
      values = this._getFrozenCopy(
        isUpdate ? { ...this.values, ...data } : (data as Output)
      )

    return this._getFrozenCopy({
      changes,
      context,
      operation,
      previousValues,
      values
    }) as Summary<Output, Input>
  }

  private _getValidationSummary = (isUpdate = false) =>
    this._getSummary(this.values, isUpdate)

  private async _handleError(
    error: ErrorTool<KeyOf<Input & Aliases>>,
    data?: Partial<Output>,
    virtuals: KeyOf<Output>[] = []
  ) {
    if (data) await this._handleFailure(data, error, virtuals)

    return this._options.errors === 'throw'
      ? error.throw()
      : { data: null, error: error.data, handleSuccess: null }
  }

  private async _handleFailure(
    data: Partial<Output>,
    error: ErrorTool<KeyOf<Input & Aliases>>,
    virtuals: KeyOf<Output>[] = []
  ) {
    let props = [...getKeysAsProps({ ...data, ...error.payload }), ...virtuals]

    props = Array.from(new Set(props))

    const ctx = this._getContext()

    const cleanups = props.map(async (prop) => {
      const handlers = this._getHandlers<ns.FailureHandler<Output, Input>>(
        prop,
        'onFailure'
      )

      const _cleanups = handlers.map(async (handler) => await handler(ctx))

      await Promise.allSettled(_cleanups)
    })

    await Promise.allSettled(cleanups)
  }

  private _handleInvalidData = () =>
    this._handleError(new ErrorTool(VALIDATION_ERRORS.INVALID_DATA))

  private _handleRequiredBy(data: Partial<Output>, isUpdate = false) {
    const error = new ErrorTool(VALIDATION_ERRORS.VALIDATION_ERROR)
    const summary = this._getSummary(data, isUpdate)

    for (const prop of this.propsRequiredBy) {
      const [isRequired, message] = this._getRequiredState(prop, summary)

      if (
        (isRequired && !isUpdate) ||
        (isRequired && isUpdate && this._isUpdatable(prop))
      ) {
        error.add(prop, message)

        const alias = this._getAliasByVirtual(prop)

        if (!alias) continue

        const _message =
          message == `'${prop}' is required!`
            ? `'${alias}' is required!`
            : message

        error.add(alias, _message)
      }
    }

    return error
  }

  private async _handleSanitizationOfVirtuals(
    data: Partial<Output>,
    isUpdate = false
  ) {
    const sanitizers: [KeyOf<Input>, Function][] = []

    const partialCtx = this._getPartialContext()

    const successFulVirtuals = getKeysAsProps(partialCtx).filter(
      this._isVirtual
    )

    for (const prop of successFulVirtuals) {
      const [isSanitizable, sanitizer] = this._isSanitizable(prop, !isUpdate)

      if (isSanitizable) sanitizers.push([prop as KeyOf<Input>, sanitizer])
    }

    const summary = this._getSummary(data, isUpdate)

    const sanitizations = sanitizers.map(async ([prop, sanitizer]) => {
      const resolvedValue = await sanitizer(summary)

      this._updateContext({ [prop]: resolvedValue } as any)
    })

    await Promise.allSettled(sanitizations)
  }

  private _isSanitizable(
    prop: string,
    isCreation: boolean
  ): [false, undefined] | [true, Function] {
    const { sanitizer, shouldInit } = this._getDefinition(prop)

    if (!sanitizer) return [false, undefined]

    if (isCreation && isEqual(shouldInit, false)) return [false, undefined]

    return [true, sanitizer]
  }

  private _isGloballyUpdatable(changes: any) {
    const { shouldUpdate = defaultOptions.shouldUpdate! } = this._options

    if (typeof shouldUpdate == 'boolean') return shouldUpdate

    return shouldUpdate(this._getSummary(changes, true))
  }

  private _isUpdatable(prop: string, value?: any) {
    const isAlias = this._isVirtualAlias(prop),
      isVirtual = this._isVirtual(prop)

    if (
      (!this._isProp(prop) ||
        this._isConstant(prop) ||
        this._isDependentProp(prop)) &&
      !isVirtual &&
      !isAlias
    )
      return false

    const propName = (
      isAlias ? this._getVirtualByAlias(prop)! : prop
    ) as KeyOf<Output>

    const hasShouldUpdateRule = this._isRuleInDefinition(
      propName,
      'shouldUpdate'
    )

    const extraCtx = isAlias ? { [propName]: value } : {}

    const isUpdatable = this._getValueBy(propName, 'shouldUpdate', extraCtx)

    if (isVirtual) return hasShouldUpdateRule ? isUpdatable : true

    const isReadonly = this._isReadonly(propName)

    if (!isReadonly) return hasShouldUpdateRule ? isUpdatable : true

    if (hasShouldUpdateRule && !isUpdatable) return false

    return (
      isReadonly &&
      isEqual(
        this.defaults[propName],
        this.values[propName],
        this._options.equalityDepth
      )
    )
  }

  private _isValidProperty = (prop: string) => {
    if (this._isConstant(prop)) return false

    const isAlias = this._isVirtualAlias(prop)

    if (this._isDependentProp(prop) && !isAlias) return false

    return this._isProp(prop) || this._isVirtual(prop) || isAlias
  }

  private _makeHandleSuccess(data: Partial<Output>, isUpdate = false) {
    const partialCtx = this._getPartialContext()

    const successProps = getKeysAsProps(partialCtx)

    let successListeners = [] as ns.SuccessHandler<Output, Input>[]

    const summary = this._getSummary(data, isUpdate)

    for (const prop of successProps) {
      const handlers = this._getHandlers<ns.SuccessHandler<Output, Input>>(
        prop,
        'onSuccess'
      )

      successListeners = successListeners.concat(handlers)
    }

    successListeners = successListeners.concat(this.globalSuccessHandlers)

    return async () => {
      const successOperations = successListeners.map(
        async (handler) => await handler(summary)
      )

      await Promise.allSettled(successOperations)
    }
  }

  private async _resolveDependentChanges(
    data: Partial<Output>,
    ctx: Partial<Output> | Partial<Context<Output, Input>>,
    isUpdate = false
  ) {
    let _updates = { ...data }

    const successFulChanges = getKeysAsProps(ctx)

    let toResolve = [] as KeyOf<Output>[]

    const isCreation = !isUpdate

    for (const prop of successFulChanges) {
      if (this._regeneratedProps.includes(prop) && !isPropertyOf(prop, data))
        continue

      const dependencies = this._getDependencies(prop)

      if (!dependencies.length) continue

      if (isCreation && this._isVirtual(prop) && !this._isVirtualInit(prop))
        continue

      if (
        isCreation &&
        (this._isDependentProp(prop) || this._isLaxProp(prop)) &&
        isEqual(this.defaults[prop], data[prop], this._options.equalityDepth)
      )
        continue

      toResolve = toResolve.concat(dependencies as any)
    }

    toResolve = Array.from(new Set(toResolve))

    const values = isUpdate ? data : { ...this.values, ...data }

    const _ctx = this._getContext(),
      summary = this._getSummary(values, isUpdate)

    const operations = toResolve.map(async (prop) => {
      if (
        this._isReadonly(prop) &&
        !isCreation &&
        !isEqual(
          this.values[prop],
          this.defaults[prop],
          this._options.equalityDepth
        )
      )
        return

      const resolver = this._getDefinition(prop).resolver!

      const value = await resolver(summary)

      if (
        !isCreation &&
        isEqual(
          value,
          _ctx[prop as KeyOf<Context<Output, Input>>],
          this._options.equalityDepth
        )
      )
        return

      data[prop] = value

      const updates = { [prop]: value } as any

      this._updateContext(updates)
      this._updatePartialContext(updates)

      const _data = await this._resolveDependentChanges(
        data,
        updates as unknown as Output,
        isUpdate
      )

      return (_updates = { ..._updates, ..._data })
    })

    await Promise.allSettled(operations)

    return _updates
  }

  private _setValues(
    values: Partial<Input | Output | Aliases>,
    {
      allowVirtuals = true,
      allowTimestamps = false
    }: {
      allowVirtuals?: boolean
      allowTimestamps?: boolean
    } = {
      allowVirtuals: true,
      allowTimestamps: false
    }
  ) {
    const keys = getKeysAsProps(values).filter((key) => {
      if (
        allowTimestamps &&
        this.timestampTool.withTimestamps &&
        this.timestampTool.isTimestampKey(key)
      )
        return true

      if (allowVirtuals && this._isVirtual(key)) return true

      return this._isProp(key)
    })

    const _values = {} as any

    sort(keys).forEach((key) => (_values[key] = values[key]))

    this.values = _values as Output

    this._initializeContexts()
  }

  private async _setMissingDefaults() {
    this._regeneratedProps = this.props.filter((prop) => {
      return this._isDefaultable(prop) && isEqual(this.values[prop], undefined)
    })

    await Promise.allSettled(
      this._regeneratedProps.map(async (prop) => {
        const value = await this._getDefaultValue(prop)

        this._updateContext({ [prop]: value } as any)
        this._updatePartialContext({ [prop]: value } as any)
      })
    )
  }

  private _useConfigProps(obj: Partial<Output>, isUpdate = false) {
    if (!this.timestampTool.withTimestamps) return sortKeys(obj)

    const { createdAt, updatedAt } = this.timestampTool.getKeys()

    let results = { ...obj }

    if (updatedAt) results = { ...results, [updatedAt]: new Date() }

    if (!isUpdate && createdAt)
      results = { ...results, [createdAt]: new Date() }

    return sortKeys(results)
  }

  private async _validateAndSet(
    operationData: Partial<Output> = {},
    error: ErrorTool<KeyOf<Input & Aliases>>,
    prop: KeyOf<Output>,
    value: any,
    isUpdate = false
  ) {
    const isValid = (await this._validateInternally(
      prop as any,
      value,
      this._getValidationSummary(isUpdate)
    )) as InternalValidatorResponse<Output[KeyOf<Output>]>

    if (!isValid.valid) {
      const { otherReasons, reasons, metadata } = isValid

      const hasOtherReasons = !!otherReasons

      if (metadata) error.add(prop as any, { reasons: [], metadata })

      if (!hasOtherReasons) return error.add(prop as any, reasons)

      if (reasons.length) error.add(prop as any, reasons)
      else error.add(prop as any, 'validation failed')

      return Object.entries(otherReasons).forEach(([key, reasons]) => {
        error.add(key as any, reasons)
      })
    }

    const { validated } = isValid

    const isAlias = this._isVirtualAlias(prop)

    const propName = isAlias ? this._getVirtualByAlias(prop)! : prop

    if (!this._isVirtual(propName))
      operationData[propName as KeyOf<Output>] = validated

    const validCtxUpdate = { [propName]: validated } as unknown as any

    this._updateContext(validCtxUpdate)
    this._updatePartialContext(validCtxUpdate)
  }

  private _sanitizeValidationResponse<T>(
    response: any,
    value: any
  ): ResponseInputObject<any, any, T> {
    const responseType = typeof response

    if (responseType == 'boolean')
      return response
        ? { valid: true, validated: value }
        : validationFailedResponse

    if (!response && (responseType != 'object' || Array.isArray(response)))
      return validationFailedResponse

    if (response?.valid) {
      const validated = isEqual(response?.validated, undefined)
        ? value
        : response.validated

      return { valid: true, validated }
    }

    const _response: ResponseInputObject<any, any, T> = { valid: false }

    if (response?.otherReasons) {
      const validProperties = getKeysAsProps(response.otherReasons).filter(
        this._isValidProperty
      )

      const otherReasons = {} as Record<string, any>

      for (const prop of validProperties)
        otherReasons[prop] = toArray(response.otherReasons[prop]).map(
          (value) => {
            return typeof value === 'string' ? value : 'validation failed'
          }
        )

      _response.otherReasons = otherReasons
    }

    if (response?.reason) _response.reason = response.reason
    if (response?.reasons) _response.reasons = response.reasons

    if (response?.metadata && isObject(response.metadata))
      _response.metadata = sortKeys(response.metadata)
    else _response.metadata = null

    if (!_response.reason && !_response.reasons && !_response.otherReasons) {
      if (_response.metadata)
        (validationFailedResponse as any).metadata = _response.metadata

      return validationFailedResponse
    }

    return makeResponse(_response)
  }

  async clone(
    values: Partial<Input & Aliases>,
    options: ns.CloneOptions<Input> = { reset: [] }
  ) {
    if (!areValuesOk(values)) return this._handleInvalidData()

    this._setValues(values)

    const reset = toArray<KeyOf<Input>>(options.reset ?? []).filter(
      this._isProp
    )

    let data = await this._generateConstants()

    const validationError = new ErrorTool<KeyOf<Input & Aliases>>(
      VALIDATION_ERRORS.VALIDATION_ERROR
    )

    const virtuals = getKeysAsProps<Partial<Output>>(values as any).filter(
      (prop) =>
        this._isVirtualInit(prop, values[prop as unknown as KeyOf<Input>])
    )

    const props = [
      ...this.props.filter((prop) => !this._isConstant(prop)),
      ...virtuals
    ]

    const validations = props.map(async (prop) => {
      const isAlias = this._isVirtualAlias(prop),
        isDependent = this._isDependentProp(prop)

      if (isDependent && !isAlias) {
        const value = reset.includes(prop as any)
          ? await this._getDefaultValue(prop)
          : this.values[prop as unknown as KeyOf<Output>]

        data[prop] = value

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any

        this._updatePartialContext(validCtxUpdate)
        return this._updateContext(validCtxUpdate)
      }

      const isVirtualInit = virtuals.includes(prop)

      if (this._isVirtual(prop) && !isVirtualInit) return

      if (isAlias && !isDependent)
        return this._validateAndSet(
          data,
          validationError,
          prop,
          values[prop as unknown as KeyOf<Input>]
        )

      if (reset.includes(prop as any)) {
        data[prop] = await this._getDefaultValue(prop)

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any

        this._updatePartialContext(validCtxUpdate)
        return this._updateContext(validCtxUpdate)
      }

      const isLax = this._isLaxProp(prop)

      const isProvided = isPropertyOf(prop, this.values)

      const isLaxInit =
        isLax &&
        isProvided &&
        !isEqual(
          this.values[prop as unknown as KeyOf<Output>],
          this.defaults[prop as unknown as KeyOf<Output>],
          this._options.equalityDepth
        )

      const isRequiredInit =
        this._isRequiredBy(prop) && isPropertyOf(prop, this.values)

      if (
        (isLax &&
          this._isRuleInDefinition(prop, 'shouldInit') &&
          !this._getValueBy(prop, 'shouldInit')) ||
        (!isVirtualInit &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
      ) {
        data[prop] = await this._getDefaultValue(prop)

        const validCtxUpdate = { [prop]: data[prop] } as unknown as any

        this._updatePartialContext(validCtxUpdate)
        return this._updateContext(validCtxUpdate)
      }

      return this._validateAndSet(
        data,
        validationError,
        prop,
        this.values[prop as unknown as KeyOf<Output>]
      )
    })

    await Promise.allSettled(validations)

    if (validationError.isLoaded)
      return this._handleError(validationError, data, virtuals)

    const requiredError = this._handleRequiredBy(data)

    if (requiredError.isLoaded)
      return this._handleError(requiredError, data, virtuals)

    await this._handleSanitizationOfVirtuals(data)

    data = await this._resolveDependentChanges(data, this._getPartialContext())

    const finalData = this._useConfigProps(data)

    this._updateContext(finalData as any)
    this._updatePartialContext(finalData as any)

    return {
      data: finalData as Output,
      error: null,
      handleSuccess: this._makeHandleSuccess(finalData)
    }
  }

  async create(values: Partial<Input & Aliases> = {}) {
    if (!areValuesOk(values)) return this._handleInvalidData()

    this._setValues(values)

    let data = await this._generateConstants()

    const validationError = new ErrorTool<KeyOf<Input & Aliases>>(
      VALIDATION_ERRORS.VALIDATION_ERROR
    )

    const virtuals = getKeysAsProps<Partial<Output>>(values as any).filter(
      (prop) =>
        this._isVirtualInit(prop, values[prop as unknown as KeyOf<Input>])
    )

    const props = [
      ...this.props.filter((prop) => !this._isConstant(prop)),
      ...virtuals
    ]

    const validations = props.map(async (prop) => {
      const isVirtualInit = virtuals.includes(prop)

      if (this._isVirtual(prop) && !isVirtualInit) return

      if (this._isVirtualAlias(prop) && !this._isDependentProp(prop))
        return this._validateAndSet(
          data,
          validationError,
          prop,
          values[prop as unknown as KeyOf<Input>]
        )

      const isProvided = isPropertyOf(prop, this.values)

      const isLax = this._isLaxProp(prop)

      const isLaxInit = isLax && isProvided

      const isRequiredInit = this._isRequiredBy(prop) && isProvided

      if (
        (isLax &&
          this._isRuleInDefinition(prop, 'shouldInit') &&
          !this._getValueBy(prop, 'shouldInit')) ||
        (!isVirtualInit &&
          !this._canInit(prop) &&
          !isLaxInit &&
          !isRequiredInit)
      ) {
        data[prop] = await this._getDefaultValue(prop)

        const validCtxUpdate = { [prop]: data[prop] as any } as any

        this._updatePartialContext(validCtxUpdate)
        return this._updateContext(validCtxUpdate)
      }

      return this._validateAndSet(
        data,
        validationError,
        prop,
        this.values[prop]
      )
    })

    await Promise.allSettled(validations)

    if (validationError.isLoaded)
      return this._handleError(validationError, data, virtuals)

    const requiredError = this._handleRequiredBy(data)

    if (requiredError.isLoaded)
      return this._handleError(requiredError, data, virtuals)

    await this._handleSanitizationOfVirtuals(data)

    data = await this._resolveDependentChanges(data, this._getPartialContext())

    const finalData = this._useConfigProps(data)

    this._updateContext(finalData as any)
    this._updatePartialContext(finalData as any)

    return {
      data: finalData as Output,
      error: null,
      handleSuccess: this._makeHandleSuccess(finalData)
    }
  }

  async delete(values: Output) {
    if (!areValuesOk(values))
      return new ErrorTool(VALIDATION_ERRORS.INVALID_DATA).throw()

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true })

    let handlers: ns.Handler<Output>[] = [...this.globalDeleteHandlers]

    const data = this._getFrozenCopy(this.values)

    this.props.map(async (prop) => {
      const handlers_ = this._getHandlers<ns.Handler<Output>>(prop, 'onDelete')

      if (handlers_.length) handlers = handlers.concat(handlers_)
    })

    const cleanups = handlers.map(async (handler) => await handler(data))

    await Promise.allSettled(cleanups)
  }

  async update(values: Output, changes: Partial<Input & Aliases>) {
    if (!areValuesOk(values)) return this._handleInvalidData()

    this._setValues(values, { allowVirtuals: false, allowTimestamps: true })

    if (this._options?.setMissingDefaultsOnUpdate)
      await this._setMissingDefaults()

    const validationError = new ErrorTool<KeyOf<Input & Aliases>>(
      VALIDATION_ERRORS.VALIDATION_ERROR
    )

    if (!this._isGloballyUpdatable(changes as any))
      return this._handleError(
        validationError.setMessage(VALIDATION_ERRORS.NOTHING_TO_UPDATE)
      )

    let updates = {} as Partial<Output>

    const toUpdate = getKeysAsProps(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop, changes[prop])
    )

    const linkedProps: KeyOf<Output>[] = []
    const virtuals: KeyOf<Output>[] = []

    const validations = toUpdate.map(async (prop) => {
      const value = changes[prop] as unknown as Output[KeyOf<Output>]
      const isValid = (await this._validate(
        prop as any,
        value,
        this._getValidationSummary(true)
      )) as ValidationResponse<Output[KeyOf<Output>]>

      if (!isValid.valid)
        return validationError.add(prop as any, isValid.reasons)

      let { validated } = isValid

      if (isEqual(validated, undefined)) validated = value

      const isAlias = this._isVirtualAlias(prop)

      const propName = (isAlias
        ? this._getVirtualByAlias(prop)!
        : prop) as unknown as KeyOf<Output>

      if (
        isEqual(validated, this.values[propName], this._options.equalityDepth)
      )
        return

      if (this._isVirtual(propName)) virtuals.push(propName)
      else {
        updates[propName as KeyOf<Output>] = validated
        linkedProps.push(propName)
      }

      const validCtxUpdate = { [propName]: validated } as unknown as any

      this._updateContext(validCtxUpdate)
      this._updatePartialContext(validCtxUpdate)
    })

    await Promise.allSettled(validations)

    if (validationError.isLoaded)
      return this._handleError(validationError, updates, virtuals)

    const requiredError = this._handleRequiredBy(updates, true)

    if (requiredError.isLoaded)
      return this._handleError(requiredError, updates, virtuals)

    await this._handleSanitizationOfVirtuals(updates, true)

    updates = await this._resolveDependentChanges(
      updates,
      this._getPartialContext(),
      true
    )

    if (!Object.keys(updates).length) {
      await this._handleFailure(updates, validationError, virtuals)
      return this._handleError(
        validationError.setMessage(VALIDATION_ERRORS.NOTHING_TO_UPDATE)
      )
    }

    if (this._options?.setMissingDefaultsOnUpdate)
      this._regeneratedProps.forEach((prop) => {
        if (isEqual(updates[prop], undefined))
          updates[prop] = this.context[prop] as any
      })

    const finalData = this._useConfigProps(updates, true)

    this._updateContext(finalData as any)
    this._updatePartialContext(finalData as any)

    return {
      data: finalData as Partial<Output>,
      error: null,
      handleSuccess: this._makeHandleSuccess(finalData, true)
    }
  }

  async _validateInternally<K extends KeyOf<Input & Aliases>>(
    prop: K,
    value: any,
    summary_: Summary<Output, Input>
  ) {
    if (!this._isValidProperty(prop))
      return makeResponse<(Input & Aliases)[K]>({
        valid: false,
        reason: 'Invalid property'
      })

    const isAlias = this._isVirtualAlias(prop)

    const _prop = isAlias ? this._getVirtualByAlias(prop) : prop

    const validator = this._getValidator(_prop as KeyOf<Input>)

    if (validator) {
      const res = (await validator(value, summary_)) as ResponseInputObject<
        any,
        Input,
        (Input & Aliases)[K]
      >

      return this._sanitizeValidationResponse<(Input & Aliases)[K]>(res, value)
    }

    return makeResponse<(Input & Aliases)[K]>({ valid: true, validated: value })
  }

  async _validate<K extends KeyOf<Input & Aliases>>(
    prop: K,
    value: any,
    summary_: Summary<Output, Input>
  ) {
    const res = await this._validateInternally(prop, value, summary_)

    return makeResponse<(Input & Aliases)[K]>(res)
  }
}

class Model<
  Output extends RealType<Output>,
  Input extends RealType<Input> = Output,
  Aliases = {}
> {
  constructor(private modelTool: ModelTool<Output, Input, Aliases>) {}

  clone = (
    values: Partial<Input & Aliases>,
    options: ns.CloneOptions<Input> = { reset: [] }
  ) => this.modelTool.clone(values, options)

  create = (values: Partial<Input & Aliases> = {}) =>
    this.modelTool.create(values)

  delete = (values: Output) => this.modelTool.delete(values)

  update = (values: Output, changes: Partial<Input & Aliases>) =>
    this.modelTool.update(values, changes)

  validate = <K extends KeyOf<Input & Aliases>>(prop: K, value: any) => {
    return this.modelTool._validate(prop, value, {
      context: {},
      operation: 'creation',
      previousValues: undefined,
      values: {}
    } as unknown as Summary<Output, Input>)
  }
}

function areValuesOk(values: any) {
  return values && typeof values == 'object'
}
