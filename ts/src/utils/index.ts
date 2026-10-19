/* eslint-disable prefer-const */

import type {
  ArrayOfMinSizeTwo,
  DefaultFieldErrorMetadata,
  FieldError,
  InputFieldError,
  InputPayload,
  ObjectType,
  TypeOf,
  ValidationResponse,
  ValidatorResponseObject,
} from './types';

export * from './error-tool';
export * from './schema-error';

export {
  cloneWithMethods,
  deepCloneValue,
  getDefaultRequiredError,
  getSetValuesAsProps,
  getUnique,
  getUniqueBy,
  hasAnyOf,
  isEqual,
  isFieldError,
  isFunctionLike,
  isInputFieldError,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  makeFieldError,
  makeResponse,
  sort,
  sortKeys,
  toArray,
};

function deepCloneValue<T>(value: T): T {
  if (value === null || typeof value !== 'object') return value;

  try {
    return clonePlain(value, new Map());
  } catch (error) {
    if (error !== notPlainData) throw error;
  }

  try {
    return structuredClone(value);
  } catch {
    return JSON.parse(JSON.stringify(value));
  }
}

const notPlainData = Symbol('not plain data');

/**
 * `structuredClone` for plain data (primitives, Dates, plain arrays and
 * objects), without its serialization overhead. Like `structuredClone`, it
 * keeps shared references and cycles shared. Anything else throws
 * `notPlainData`, so the caller falls back to `structuredClone`.
 */
function clonePlain<T>(value: T, seen: Map<object, unknown>): T {
  if (value === null || typeof value !== 'object') {
    if (typeof value === 'function' || typeof value === 'symbol')
      throw notPlainData;

    return value;
  }

  const existing = seen.get(value);
  if (existing !== undefined) return existing as T;

  if (value instanceof Date) {
    const clone = new Date(value.getTime());
    seen.set(value, clone);
    return clone as T;
  }

  if (Array.isArray(value)) {
    if (Object.getPrototypeOf(value) !== Array.prototype) throw notPlainData;

    // arrays of primitives only: `structuredClone` copies those fastest
    // (and exactly, holes and extra properties included)
    if (value.every((item) => item === null || typeof item !== 'object')) {
      let clone: unknown[];

      try {
        clone = structuredClone(value);
      } catch {
        throw notPlainData;
      }

      seen.set(value, clone);
      return clone as T;
    }

    if (Object.keys(value).length !== value.length) throw notPlainData;

    const clone: unknown[] = new Array(value.length);
    seen.set(value, clone);
    for (let i = 0; i < value.length; i++)
      clone[i] = clonePlain(value[i], seen);
    return clone as T;
  }

  const proto = Object.getPrototypeOf(value);
  if (proto !== Object.prototype && proto !== null) throw notPlainData;

  const clone: Record<string, unknown> = {};
  seen.set(value, clone);
  for (const key of Object.keys(value))
    clone[key] = clonePlain((value as Record<string, unknown>)[key], seen);
  return clone as T;
}

// function cloneWithMethods<T extends object | null>(data: T): T {
//   if (!data || typeof data !== "object") return data;

//   const result: T = {} as T;

//   for (const [key, value] of Object.entries(data)) {
//     // @ts-expect-error ikr
//     result[key] =
//       typeof value === "function" ? value.bind(result) : deepCloneValue(value);
//   }

//   return result;
// }

// function cloneWithMethods<T extends object | null>(obj: T): T {
//   if (!obj || typeof obj !== "object") return obj;
//   // Create a new object matching the original's prototype
//   const copy = Object.create(Object.getPrototypeOf(obj));

//   // Get all property names including non-enumerable ones
//   const keys = Reflect.ownKeys(obj);

//   keys.forEach((key) => {
//     const descriptor = Object.getOwnPropertyDescriptor(obj, key)!;

//     // If it's a function/method, bind it to the new copy
//     if (typeof descriptor.value === "function")
//       descriptor.value = descriptor.value.bind(copy);
//     // else if (typeof descriptor.value === "object")
//     //   descriptor.value = deepCloneValue(descriptor.value);

//     Object.defineProperty(copy, key, descriptor);
//   });

//   return copy;
// }

function cloneWithMethods<T>(obj: T, circularMap = new WeakMap()): T {
  // 1. Handle primitives, null, functions, or special objects
  if (obj === null || typeof obj !== 'object') return obj;

  if (obj instanceof Date) return new Date(obj.getTime()) as T;
  if (obj instanceof RegExp) return new RegExp(obj.source, obj.flags) as T;

  // Prevent infinite loops with circular references
  if (circularMap.has(obj)) return circularMap.get(obj);

  // Fast path for arrays: avoid descriptor overhead for the common case
  if (Array.isArray(obj)) {
    const copy: any[] = [];
    circularMap.set(obj, copy);

    for (let i = 0; i < obj.length; i++) {
      const value = obj[i];
      copy[i] =
        typeof value === 'object' && value !== null
          ? cloneWithMethods(value, circularMap)
          : value;
    }

    return copy as T;
  }

  // Fast path for plain objects: copy enumerable own properties directly.
  // Preserves methods by reference so they resolve `this` from the clone.
  const proto = Object.getPrototypeOf(obj);
  if (proto === Object.prototype || proto === null) {
    const copy: any = proto === null ? Object.create(null) : {};
    circularMap.set(obj, copy);

    for (const key of Object.keys(obj)) {
      const value = (obj as any)[key];
      copy[key] =
        typeof value === 'object' && value !== null
          ? cloneWithMethods(value, circularMap)
          : value;
    }

    return copy as T;
  }

  // 2. Initialize the clone preserving prototype (handles custom classes)
  const copy: any = Object.create(Object.getPrototypeOf(obj));

  circularMap.set(obj, copy);

  // 3. Extract all owned property descriptors (includes non-enumerable properties & Symbols)
  const descriptors = Object.getOwnPropertyDescriptors(obj);

  for (const key of Reflect.ownKeys(descriptors)) {
    // @ts-expect-error ikr
    const descriptor = descriptors[key];

    // Handle data properties
    if ('value' in descriptor) {
      const value = descriptor.value;

      if (typeof value === 'function') {
        // Copy functions as-is so they resolve `this` from the object they
        // are called on. This keeps methods on ctx-options clones operating
        // on the clone itself rather than being permanently bound to the
        // source object.
        descriptor.value = value;
      } else if (typeof value === 'object' && value !== null) {
        // Recursively clone without overriding the nested context
        descriptor.value = cloneWithMethods(value, circularMap);
      }
    }

    // Define property on the copy maintaining original descriptor settings (getters, setters, enumerability)
    Object.defineProperty(copy, key, descriptor);
  }

  return copy;
}

function isFieldError(data: unknown): data is FieldError {
  if (!isRecordLike(data) || isEqual({}, data)) return false;

  if (
    !isPropertyOf('metadata', data) ||
    !isFieldErrorMetadataOk(data) ||
    !isPropertyOf('reason', data)
  )
    return false;

  return typeof data?.reason === 'string';
}

function isInputFieldError<Metadata>(
  data: unknown,
): data is InputFieldError<Metadata> {
  if (isFieldError(data)) return true;

  if (!isRecordLike(data) || isEqual({}, data)) return false;

  const hasMetadata = isPropertyOf('metadata', data),
    hasReason = isPropertyOf('reason', data);

  if (!hasMetadata && !hasReason) return false;

  if (hasMetadata && !isFieldErrorMetadataOk(data?.metadata)) return false;
  if (hasReason && typeof data?.reason !== 'string') return false;

  return true;
}

function isFieldErrorMetadataOk(data: unknown): data is FieldError['metadata'] {
  const metadata = (data as FieldError)?.metadata;

  return metadata == null || isRecordLike(metadata);
}

function makeFieldError<Metadata>(
  value: InputPayload[string] | InputFieldError<Metadata>,
  fallbackMessage = 'validation failed',
): FieldError<Metadata> {
  if (isFieldError(value)) {
    if (!value.reason) value.reason = fallbackMessage;

    return value as never;
  }

  if (typeof value === 'string')
    return { reason: value || fallbackMessage, metadata: null };

  return {
    reason: (value as any).reason ?? fallbackMessage,
    metadata: (value as any)?.metadata ?? null,
  };
}

function getDefaultRequiredError(fieldName: string): string {
  return `'${fieldName}' is required`;
}

function makeResponse<T = undefined, Metadata = DefaultFieldErrorMetadata>(
  input: ValidatorResponseObject<T, Metadata>,
): ValidationResponse<TypeOf<T>, Metadata> {
  if (input.valid) {
    const { valid, validated } = input;

    return { valid, validated } as ValidationResponse<TypeOf<T>, Metadata>;
  }

  const { metadata = null, reason: inputReason, valid } = input;

  const reason = inputReason
    ? isRecordLike(inputReason)
      ? inputReason
      : inputReason
    : 'validation failed';

  return {
    metadata,
    reason,
    valid,
  } as ValidationResponse<TypeOf<T>, Metadata>;
}

function getSetValuesAsProps<T>(set: Set<T>) {
  return Array.from(set.values()) as T[];
}

function hasAnyOf(object: unknown, props: string[]): boolean {
  return toArray(props).some((prop) => isPropertyOf(prop, object));
}

/**
 * tells whether `a` & `b` are equals
 * @param  depth how deep in nesting should equality checks be performed for objects
 */

function isEqual<T>(a: unknown, b: T, depth = 1): a is T {
  if (a instanceof Date || b instanceof Date)
    return (
      a instanceof Date && b instanceof Date && a.getTime() === b.getTime()
    );

  if (!a || !b || typeof a !== 'object' || typeof b !== 'object')
    return a === b;

  if (Array.isArray(a) !== Array.isArray(b)) return false;

  const keysOfA = Object.keys(a);

  // same key sets: same size, and every key of `a` is one of `b`'s
  if (keysOfA.length !== Object.keys(b as never).length) return false;

  for (const key of keysOfA) if (!Object.hasOwn(b as never, key)) return false;

  if (depth > 0 && keysOfA.length)
    return keysOfA.every((key) =>
      isEqual(a[key as never], (b as never)[key], depth - 1),
    );

  return (
    JSON.stringify(sortKeys(a as never)) ===
    JSON.stringify(sortKeys(b as never))
  );
}

function isFunctionLike<T extends Function>(value: unknown): value is T {
  return typeof value === 'function';
}

function isNullOrUndefined(value: unknown): value is null | undefined {
  return isOneOf(value, [null, undefined]);
}

function isOneOf<const T>(
  value: unknown,
  values: ArrayOfMinSizeTwo<T> | Readonly<ArrayOfMinSizeTwo<T>>,
): value is T {
  return values.includes(value as never);
}

function isRecordLike<T extends ObjectType>(
  value: unknown,
): value is ObjectType<T> {
  return !!value && typeof value === 'object' && !Array.isArray(value);
}

function isPropertyOf<T>(
  prop: string | number | symbol,
  object: T,
): prop is keyof T {
  return Object.hasOwn(object as any, prop);
}

function toArray<T>(value: T | T[] | readonly T[]): T[] {
  // @ts-expect-error ikr
  return Array.isArray(value) ? value : [value];
}

function sort<T>(data: T[]): T[] {
  return [...data].sort((a, b) => (a < b ? -1 : 1));
}

function sortKeys<T extends ObjectType>(object: T): T {
  const keys = sort(Object.keys(object));

  return keys.reduce((prev, next: keyof T) => {
    prev[next] = object[next];

    return prev;
  }, {} as T);
}

/**
 * the items whose serialization hasn't been seen yet, in order; the items
 * themselves are kept (a JSON round-trip would turn Dates into strings and
 * drop class instances' prototypes)
 */
function getUnique<T>(list: T[]) {
  const seen = new Set<unknown>();
  const unique: T[] = [];

  for (const dt of list) {
    const key = _serialize(dt);

    if (seen.has(key)) continue;

    seen.add(key);
    unique.push(dt);
  }

  return unique;
}

function getUniqueBy<T>(list: T[], key?: string) {
  if (!key) return getUnique(list);

  // a Map keeps keys of different types apart (`1` vs `"1"`) and keeps the
  // order in which each key was first seen; the last item for a key wins
  const byKey = new Map<unknown, T>();

  for (const dt of list) byKey.set(_getDeepValue(dt as ObjectType, key), dt);

  return Array.from(byKey.values());
}

function _getDeepValue(data: ObjectType, key: string): unknown {
  return key.split('.').reduce((prev, next) => prev?.[next] as never, data);
}

function _serialize(dt: unknown, revert = false) {
  try {
    return revert ? JSON.parse(dt as never) : JSON.stringify(dt);
  } catch {
    return dt;
  }
}
