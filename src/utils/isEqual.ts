export function isEqual(a: any, b: any) {
  const typeOf_a = typeof a

  if (typeOf_a != typeof b) return false

  if (typeOf_a == 'undefined') return true

  if (['bigint', 'boolean', 'number', 'string', 'symbol'].includes(typeOf_a))
    return a == b

  return JSON.stringify(a) == JSON.stringify(b)
}
