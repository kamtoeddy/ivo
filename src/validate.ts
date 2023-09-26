import { getUniqueBy, isEqual, isObject, isOneOf, makeResponse } from './utils'
import { ResponseInputObject, ValidatorResponse, XOR } from './schema/types'

export {
  isArrayOk,
  isBooleanOk,
  isCreditCardOk,
  isEmailOk,
  isNumberOk,
  isStringOk
}

export type { ArrayOptions, NumberRangeType, RangeType, StringOptions }

type ArrayOptions<T> = {
  empty?: boolean
  filter?: (data: T) => boolean | Promise<boolean>
  modifier?: (data: T) => any | Promise<any>
  sorted?: boolean
  sorter?: (a: T, b: T) => number
  sortOrder?: 'asc' | 'desc'
  unique?: boolean
  uniqueKey?: string
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

const EMAIL_REGEXP =
  /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/

const invalidResponse = makeResponse({ reason: 'Invalid email', valid: false })

const isEmailOk = (value: any, regExp = EMAIL_REGEXP) => {
  if (typeof value !== 'string') return invalidResponse

  const validated = value?.trim()

  return regExp.test(validated)
    ? makeResponse<string>({ valid: true, validated })
    : invalidResponse
}

type NumberRangeType = {
  bounds: number[]
  inclusiveBottom?: boolean
  inclusiveTop?: boolean
}

type NumberRangeType_ = {
  min: number | null
  max: number | null
  inclusiveBottom: boolean
  inclusiveTop: boolean
}

type RangeType = NumberRangeType

function _isInNumberRange(
  value: number,
  range: NumberRangeType_
): ResponseInputObject<any, any, number> {
  const { max, min, inclusiveBottom, inclusiveTop } = range

  if (
    !isEqual(min, null) &&
    ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min))
  )
    return { reason: 'Too small', valid: false, metadata: range }

  if (
    !isEqual(max, null) &&
    ((inclusiveTop && value > max) || (!inclusiveTop && value >= max))
  )
    return { reason: 'Too large', valid: false, metadata: range }

  return { valid: true, validated: value }
}

function _makeNumberRage(range?: RangeType) {
  if (!isObject(range) || !range?.bounds) return undefined

  const range_ = {} as NumberRangeType_

  range_.min = typeof range?.bounds[0] == 'number' ? range.bounds[0] : null
  range_.max = typeof range?.bounds[1] == 'number' ? range.bounds[1] : null

  const { inclusiveBottom, inclusiveTop } = range

  range_.inclusiveBottom =
    typeof inclusiveBottom == 'boolean'
      ? inclusiveBottom
      : range_.inclusiveBottom ?? true

  range_.inclusiveTop =
    typeof inclusiveTop == 'boolean'
      ? inclusiveTop
      : range_.inclusiveTop ?? true

  return range_
}

function isNumberOk(num: any, { range }: { range?: RangeType } = {}) {
  const range_ = _makeNumberRage(range)

  if (!['number', 'string'].includes(typeof num) || isNaN(num))
    return makeResponse({
      reason: 'Expected a number',
      valid: false,
      metadata: range_
    })

  num = Number(num)

  if (range_) {
    const _isInRange = _isInNumberRange(num, range_)

    if (!_isInRange.valid) return makeResponse(_isInRange)
  }

  return makeResponse<number>({ valid: true, validated: num })
}

const MAX_LENGTH = 255,
  MIN_LENGTH = 1

type StringOptions<T extends string = string> = XOR<
  { enums: T[] | readonly T[] },
  { maxLength?: number; minLength?: number; regExp?: RegExp; trim?: boolean }
>

function isStringOk<T extends string = string>(
  str: any,
  {
    enums,
    maxLength = MAX_LENGTH,
    minLength = MIN_LENGTH,
    regExp,
    trim = false
  }: StringOptions<T> = {}
): ValidatorResponse<Exclude<T, undefined>> {
  if (isOneOf(str, [null, undefined]))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  if (enums)
    return isOneOf(str, enums as any)
      ? makeResponse({ valid: true, validated: str })
      : makeResponse({
          reason: 'Unacceptable value',
          valid: false,
          metadata: { allowed: enums }
        })

  if (regExp && !regExp.test(str))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  str = String(str)

  if (trim) str = str.trim()

  if (typeof maxLength != 'number' || maxLength < 0) maxLength = MAX_LENGTH

  if (minLength > maxLength) minLength--

  if (typeof minLength != 'number' || minLength < 0) minLength = MIN_LENGTH

  if (minLength == 0 && minLength == maxLength) {
    maxLength = MAX_LENGTH
    minLength = MIN_LENGTH
  }

  const metadata = { maxLength, minLength }

  if (str.length < minLength)
    return makeResponse({ reason: 'Too short', valid: false, metadata })

  if (str.length > maxLength)
    return makeResponse({ reason: 'Too long', valid: false, metadata })

  return makeResponse({ valid: true, validated: str })
}
