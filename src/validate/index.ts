import { makeResponse } from '../schema/utils'
import { ArrayOptions } from '../utils/types'
import { ResponseInputObject } from '../schema/types'
import { NumberRangeType } from '../utils/types'

import { ValidatorResponse } from '../schema/types'
import { getUniqueBy, isOneOf } from '../utils'
import { StringOptions } from '../utils/types'

export {
  isArrayOk,
  isBooleanOk,
  isCreditCardOk,
  isEmailOk,
  isNumberOk,
  isStringOk
}

async function isArrayOk<T>(
  arr: T[],
  {
    empty = false,
    sorted = false,
    filter,
    modifier,
    sorter,
    sortOrder = 'asc',
    unique = true,
    uniqueKey = ''
  }: ArrayOptions<T> = {}
) {
  if (!Array.isArray(arr))
    return makeResponse({ reason: 'Expected an array', valid: false })

  let _array = [...arr]

  if (filter) _array = await Promise.all(arr.filter(filter))

  if (!empty && !_array.length)
    return makeResponse({ reason: 'Expected a non-empty array', valid: false })

  if (modifier) _array = await Promise.all(_array.map(modifier))

  if (unique && _array.length)
    _array = uniqueKey ? getUniqueBy(_array, uniqueKey) : getUniqueBy(_array)

  if (sorted || sorter) {
    if (!sorter) {
      const order = _getArrayOrder(sortOrder)

      sorter = (a, b) => (a < b ? order : -order)
    }
    _array = await Promise.all(_array.sort(sorter))
  }

  return makeResponse<T[]>({ valid: true, validated: _array })
}

const _getArrayOrder = (sortOrder: any) => {
  if (!['asc', 'desc'].includes(sortOrder)) return -1

  return sortOrder === 'asc' ? -1 : 1
}

function isBooleanOk(value: any) {
  if (typeof value !== 'boolean')
    return makeResponse({ reason: 'Expected a boolean', valid: false })

  return makeResponse<boolean>({ valid: true, validated: value })
}

const failResponse = makeResponse({
  reason: 'Invalid card number',
  valid: false
})

const isCreditCardOk = (value: any) => {
  const _value = String(value).trim()

  if (_value.length !== 16) return failResponse

  const singleDigits = _getSingleDigits(_value)

  if (singleDigits.length !== 16) return failResponse

  if (!_isCheckSumOk(singleDigits)) return failResponse

  const validated = typeof value === 'number' ? value : _value

  return makeResponse<string | number>({ valid: true, validated })
}

function _isEven(num: number) {
  return num % 2 === 0
}

function _getSingleDigits(value: number | string) {
  return String(value)
    .split('')
    .filter((v) => !isNaN(parseInt(v)))
    .map(Number)
}

function _getCheckSum(values: number[]) {
  const separated = _getSingleDigits(values.map((v) => String(v)).join(''))

  return separated.map(Number).reduce((prev, next) => (prev += next))
}

function _isCheckSumOk(values: number[]) {
  const controlNumber = values[15]
  const toCheck = values.slice(0, 15).map((v, i) => (_isEven(i) ? 2 * v : v))

  return 10 - (_getCheckSum(toCheck) % 10) === controlNumber
}

let emailRegExp =
  /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/

const invalidResponse = makeResponse({
  reason: 'Invalid email',
  valid: false
})

const isEmailOk = (value: any, customRegExp?: RegExp) => {
  if (typeof value !== 'string') return invalidResponse

  const validated = value?.trim()

  if (customRegExp) emailRegExp = customRegExp

  return emailRegExp.test(validated)
    ? makeResponse<string>({ valid: true, validated })
    : invalidResponse
}

export type RangeType = undefined | NumberRangeType

function _isInNumberRange(
  value: number,
  range: NumberRangeType
): ResponseInputObject<any, any, number> {
  const { bounds, inclusiveBottom, inclusiveTop } = range
  const [min, max] = bounds

  if ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min))
    return { reason: 'Too small', valid: false }

  if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max))
    return { reason: 'Too large', valid: false }

  return { valid: true, validated: value }
}

function _makeNumberRage(range: RangeType): RangeType {
  if (!range?.bounds) return undefined

  const { inclusiveBottom, inclusiveTop } = range

  if (typeof inclusiveBottom !== 'boolean') range.inclusiveBottom = true
  if (typeof inclusiveTop !== 'boolean') range.inclusiveTop = true

  return range
}

function isNumberOk(num: any, { range }: { range?: RangeType } = {}) {
  if (!['number', 'string'].includes(typeof num) || isNaN(num))
    return makeResponse({ reason: 'Expected a number', valid: false })

  num = Number(num)

  range = _makeNumberRage(range)

  if (range) {
    const _isInRange = _isInNumberRange(num, range)

    if (!_isInRange.valid) return makeResponse(_isInRange)
  }

  return makeResponse<number>({ valid: true, validated: num })
}

function isStringOk<T extends string = string>(
  str: any,
  {
    enums,
    maxLength = 255,
    minLength = 1,
    regExp,
    trim = false
  }: StringOptions<T> = {}
): ValidatorResponse<Exclude<T, undefined>> {
  if (isOneOf(str, [null, undefined]))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  if (enums)
    return isOneOf(str, enums as any)
      ? makeResponse({ valid: true, validated: str })
      : makeResponse({ reason: 'Unacceptable value', valid: false })

  if (regExp && !regExp.test(str))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  str = String(str)

  if (trim) str = str.trim()

  if (str.length < minLength)
    return makeResponse({ reason: 'Too short', valid: false })

  if (str.length > maxLength)
    return makeResponse({ reason: 'Too long', valid: false })

  return makeResponse({ valid: true, validated: str })
}
