export function isEqual(a: any, b: any) {
  const typeOf_a = typeof a
  const typeOf_b = typeof b

  if (typeOf_a !== typeOf_b) return false

  if (typeOf_a === 'undefined') return true

  let ref_a = a
  let ref_b = b

  if (!['bigint', 'boolean', 'number', 'string', 'symbol'].includes(typeOf_a)) {
    ref_a = JSON.stringify(ref_a)
    ref_b = JSON.stringify(ref_b)
  }

  return ref_a == ref_b
}
