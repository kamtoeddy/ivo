import { Summary, StringOptions } from '../../../../../dist'
import {
  IOtherMeasureUnit,
  IOtherQuantity,
  IStoreItem,
  StoreItemType
} from './types'

type SummaryType = Summary<StoreItemType, IStoreItem>

import { isArrayOk, isNumberOk, isStringOk } from '../../../../../dist/validate'
import { findBy } from '../../utils'

export const validateName = (val: any) => {
  const isValid = isStringOk(val, { trim: true })

  if (!isValid.valid) return { valid: false }

  return { valid: true, validated: isValid.validated }
}

export const validateString = (
  errorMessage = '',
  options: StringOptions = {}
) => {
  return (val: any) => {
    const isValid = isStringOk(val, options)

    if (!isValid.valid && errorMessage)
      isValid.reasons = [errorMessage, ...isValid.reasons]

    return isValid
  }
}

export const validateOtherUnit = (value: any) => {
  const isValidCoeff = isNumberOk(value?.coefficient, {
    range: { bounds: [0], inclusiveBottom: false }
  })
  const isValidName = isStringOk(value?.name)

  if (!isValidCoeff.valid || !isValidName.valid) return { valid: false }

  return {
    valid: true,
    validated: {
      coefficient: isValidCoeff.validated,
      name: isValidName.validated
    }
  }
}

export const validateOtherUnits = (value: any) => {
  return isArrayOk<IOtherMeasureUnit>(value, {
    empty: true,
    sorted: true,
    filter: (v) => validateOtherUnit(v).valid,
    modifier: (v) => validateOtherUnit(v).validated,
    sorter: (a, b) => (a.name < b.name ? -1 : 1),
    uniqueKey: 'name'
  })
}

export const validatePrice = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } })

export const validateQuantity = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } })

export const validateOtherQuantity = (value: any, ctx: IStoreItem) => {
  const mu = getMeasureUnit(ctx.otherMeasureUnits!, value?.name)

  const isValidQty = isNumberOk(value?.quantity, {
    range: { bounds: [0], inclusiveBottom: false }
  })

  if (!mu || !isValidQty.valid) return { valid: false }

  return {
    valid: true,
    validated: { name: value.name, quantity: isValidQty.validated }
  }
}

export const validateQuantities = async (
  value: any,
  { context }: SummaryType
) => {
  return isArrayOk<IOtherQuantity>(value, {
    empty: true,
    unique: false,
    filter: (v) => validateOtherQuantity(v, context).valid,
    modifier: (v) => validateOtherQuantity(v, context).validated
  })
}

const getMeasureUnit = (
  otherMeasureUnits: IOtherMeasureUnit[],
  name: string
) => {
  return findBy(otherMeasureUnits, { name })
}

export const sanitizeQuantities = ({
  context: { quantities, otherMeasureUnits }
}: SummaryType) => {
  return (quantities as IOtherQuantity[]).reduce((prev, { name, quantity }) => {
    const mu = getMeasureUnit(otherMeasureUnits!, name)

    if (!mu) return prev

    return (prev += quantity * mu.coefficient)
  }, 0)
}
