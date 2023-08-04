import { toArray } from '../../utils/functions'
import { ResponseInput_, ValidatorResponse, TypeOf } from '../types'

export const makeResponse = <T = undefined>(
  input: ResponseInput_<any, any, T>
) => {
  if (input.valid) {
    const { valid, validated } = input

    return { valid, validated } as ValidatorResponse<TypeOf<T>>
  }

  // eslint-disable-next-line prefer-const
  let { otherReasons, reason, reasons, valid } = input as any

  if (reasons) reasons = toArray(reasons)
  else reasons = []

  if (reason) reasons = [...reasons, ...toArray(reason)]

  return { otherReasons, reasons, valid } as ValidatorResponse<TypeOf<T>>
}
