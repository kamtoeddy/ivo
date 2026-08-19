/**
 * `Array.isArray` doesn't narrow a union containing a readonly tuple (like
 * `ArrayOfMinSizeTwo`'s `readonly [T, T, ...T[]]` branch) cleanly, so this
 * narrows from `unknown` instead of the generic union directly.
 */
export function extractAllowedValues(allow: unknown) {
  if (!allow) return undefined;

  return Array.isArray(allow) ? allow : (allow as { values: unknown }).values;
}
