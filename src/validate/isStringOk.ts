import { makeResponse } from '../schema/utils'
import { belongsTo } from '../utils/functions'
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
) {
  if (belongsTo(str, [null, undefined]))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  if (enums)
    return belongsTo(str, enums)
      ? makeResponse<T>({ valid: true, validated: str })
      : makeResponse<T>({ reason: 'Unacceptable value', valid: false })

  if (regExp && !regExp.test(str))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  str = String(str)

  if (trim) str = str.trim()

  if (str.length < minLength)
    return makeResponse({ reason: 'Too short', valid: false })

  if (str.length > maxLength)
    return makeResponse({ reason: 'Too long', valid: false })

  return makeResponse<T>({ valid: true, validated: str })
}
