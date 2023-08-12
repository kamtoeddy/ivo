export function isEqual(a: any, b: any) {
  const typeOfA = typeof a

  if (typeOfA != typeof b) return false

  if (typeOfA == 'undefined') return true

  if (['bigint', 'boolean', 'number', 'string', 'symbol'].includes(typeOfA))
    return a == b

  return JSON.stringify(a) == JSON.stringify(b)
}
