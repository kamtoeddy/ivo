import { isEqual, isPropertyOf, isRecordLike } from "../../utils";
import type { FieldError, InputFieldError, InputPayload } from "./types";

export * from "./constants";
export * from "./error-tool";
export * from "./schema-error";
export * from "./timestamp-tool";
export * from "./types";

export {
  deepCloneValue,
  cloneWithMethods,
  isFieldError,
  isInputFieldError,
  getDefaultRequiredError,
  makeFieldError,
};

function deepCloneValue<T>(value: T): T {
  try {
    return structuredClone(value);
  } catch {
    return JSON.parse(JSON.stringify(value));
  }
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
  if (obj === null || typeof obj !== "object") return obj;

  if (obj instanceof Date) return new Date(obj.getTime()) as T;
  if (obj instanceof RegExp) return new RegExp(obj.source, obj.flags) as T;

  // Prevent infinite loops with circular references
  if (circularMap.has(obj)) return circularMap.get(obj);

  // 2. Initialize the clone preserving prototype (handles custom classes & Arrays)
  const copy: any = Array.isArray(obj)
    ? []
    : Object.create(Object.getPrototypeOf(obj));

  circularMap.set(obj, copy);

  // 3. Extract all owned property descriptors (includes non-enumerable properties & Symbols)
  const descriptors = Object.getOwnPropertyDescriptors(obj);

  for (const key of Reflect.ownKeys(descriptors)) {
    // @ts-expect-error ikr
    const descriptor = descriptors[key];

    // Handle data properties
    if ("value" in descriptor) {
      const value = descriptor.value;

      if (typeof value === "function") {
        // Bind functions directly to 'copy' (its own parent object/array)
        descriptor.value = value.bind(copy);
      } else if (typeof value === "object" && value !== null) {
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
    !isPropertyOf("metadata", data) ||
    !isFieldErrorMetadataOk(data) ||
    !isPropertyOf("reason", data)
  )
    return false;

  return typeof data?.reason === "string";
}

function isInputFieldError(data: unknown): data is Partial<FieldError> {
  if (isFieldError(data)) return true;

  if (!isRecordLike(data) || isEqual({}, data)) return false;

  const hasMetadata = isPropertyOf("metadata", data),
    hasReason = isPropertyOf("reason", data);

  if (!hasMetadata && !hasReason) return false;

  if (hasMetadata && !isFieldErrorMetadataOk(data?.metadata)) return false;
  if (hasReason && typeof data?.reason !== "string") return false;

  return true;
}

function isFieldErrorMetadataOk(data: unknown): data is FieldError["metadata"] {
  const metadata = (data as FieldError)?.metadata;

  return metadata == null || isRecordLike(metadata);
}

function makeFieldError<Metadata>(
  value: InputPayload[string] | InputFieldError<Metadata>,
  fallbackMessage = "validation failed",
): FieldError<Metadata> {
  if (isFieldError(value)) {
    if (!value.reason) value.reason = fallbackMessage;

    return value as never;
  }

  if (typeof value === "string") return { reason: value, metadata: null };

  return {
    reason: (value as any).reason ?? fallbackMessage,
    metadata: (value as any)?.metadata ?? null,
  };
}

function getDefaultRequiredError(fieldName: string): string {
  return `'${fieldName}' is required`;
}

// const original = {
//   name: "Root",
//   items: [1, 2, 3], // Has array methods like push()
//   nested: {
//     val: 10,
//     increment() {
//       this.val++;
//     },
//   },
//   push(value: number) {
//     this.items.push(value);
//   },
// };

// const cloned = cloneWithMethods(original);

// // Test 1: Nested array mutation
// cloned.items.push(4);
// cloned.push(24);
// console.log("cloned items:", cloned.items); // [1, 2, 3, 4]
// console.log("original items:", original.items); // [1, 2, 3] (unaffected)

// // Test 2: Nested method binding
// cloned.nested.increment();
// console.log("cloned nested val:", cloned.nested.val); // 11
// console.log("original nested val:", original.nested.val); // 10 (unaffected)
