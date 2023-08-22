import { ValidatorResponse } from '../schema/types'
import { makeResponse } from '../schema/utils'
import { isOneOf } from '../utils/functions'
import { StringOptions } from '../utils/types'

export function isStringOk<T extends string = string>(
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
