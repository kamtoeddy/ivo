import { makeResponse } from '../schema/utils'
import { belongsTo } from '../utils/functions'
import { IStringOptions } from '../utils/types'

export function isStringOk(
  str: any,
  {
    enums,
    maxLength = 255,
    minLength = 1,
    regExp,
    trim = false
  }: IStringOptions = {}
) {
  if (belongsTo(str, [null, undefined]))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  if (enums && !belongsTo(str, enums))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  if (regExp && !regExp.test(str))
    return makeResponse({ reason: 'Unacceptable value', valid: false })

  str = String(str)

  if (trim) str = str.trim()

  if (str.length < minLength)
    return makeResponse({ reason: 'Too short', valid: false })

  if (str.length > maxLength)
    return makeResponse({ reason: 'Too long', valid: false })

  return makeResponse<string>({ valid: true, validated: str })
}
