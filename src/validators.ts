import { ArrayOfMinSizeTwo, ValidationResponse, XOR } from "./schema/types";
import { getUniqueBy, isOneOf, makeResponse, isNullOrUndefined } from "./utils";

export {
  makeArrayValidator,
  makeNumberValidator,
  makeStringValidator,
  validateBoolean,
  validateCreditCard,
  validateEmail,
};

export type {
  ArrayValidatorOptions,
  NumberValidatorOptions,
  StringValidatorOptions,
};

type ArrayValidatorOptions<T> = {
  filter?: (data: T) => boolean | Promise<boolean>;
  modifier?: (data: T) => unknown | Promise<unknown>;
  max?: number | ValueError;
  min?: number | ValueError;
  sorted?: boolean;
  sorter?: (a: T, b: T) => number;
  sortOrder?: "asc" | "desc";
  unique?: boolean;
  uniqueKey?: string;
};

function makeArrayValidator<const T>({
  sorted = false,
  filter,
  modifier,
  sorter,
  sortOrder = "asc",
  unique = true,
  uniqueKey = "",
  max,
  min,
}: ArrayValidatorOptions<T> = {}) {
  const {
    maxValue,
    minValue,
    hasMaxValue,
    hasMinValue,
    maxError,
    metadata,
    minError,
  } = _getMaxMinInfo({
    max,
    min,
    defaulMaxError: "Max limit reached",
    defaulMinError: "Expected a non-empty array",
  });

  return async (value: unknown): Promise<ValidationResponse<T[]>> => {
    if (!Array.isArray(value))
      return makeResponse({ reason: "Expected an array", valid: false });

    let _array = [...value];

    if (filter) _array = await Promise.all(value.filter(filter));

    if (modifier) _array = await Promise.all(_array.map(modifier));

    if (unique && _array.length)
      _array = uniqueKey ? getUniqueBy(_array, uniqueKey) : getUniqueBy(_array);

    if (hasMinValue && _array.length < minValue!)
      return makeResponse({ valid: false, reason: minError, metadata });

    if (hasMaxValue && _array.length > maxValue!)
      return makeResponse({ valid: false, reason: maxError, metadata });

    if (sorted || sorter) {
      if (!sorter) {
        const order = _getArrayOrder(sortOrder);
        sorter = (a, b) => (a < b ? order : -order);
      }

      _array = [..._array].sort(sorter);
    }

    return makeResponse({ valid: true, validated: _array });
  };
}

const _getArrayOrder = (sortOrder: unknown) => {
  if (!["asc", "desc"].includes(sortOrder as never)) return -1;

  return sortOrder === "asc" ? -1 : 1;
};

function validateBoolean(value: unknown) {
  return makeResponse<boolean>(
    typeof value === "boolean"
      ? { valid: true, validated: value }
      : { valid: false, reason: "Expected a boolean" },
  );
}

const invalidCardResponse = makeResponse({
  reason: "Invalid card number",
  valid: false,
});

const validateCreditCard = (value: unknown) => {
  const _value = String(value).trim();

  if (_value.length !== 16) return invalidCardResponse;

  const singleDigits = _getSingleDigits(_value);

  if (singleDigits.length !== 16) return invalidCardResponse;

  if (!_isCheckSumOk(singleDigits)) return invalidCardResponse;

  const validated = typeof value === "number" ? value : _value;

  return makeResponse<string | number>({ valid: true, validated });
};

function _isEven(num: number) {
  return num % 2 === 0;
}

function _getSingleDigits(value: number | string) {
  return String(value)
    .split("")
    .filter((v) => !isNaN(parseInt(v)))
    .map(Number);
}

function _getCheckSum(values: number[]) {
  const separated = _getSingleDigits(values.map((v) => String(v)).join(""));

  return separated.map(Number).reduce((prev, next) => (prev += next));
}

function _isCheckSumOk(values: number[]) {
  const controlNumber = values[15];
  const toCheck = values.slice(0, 15).map((v, i) => (_isEven(i) ? 2 * v : v));

  return 10 - (_getCheckSum(toCheck) % 10) === controlNumber;
}

const EMAIL_REGEXP =
  /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;

const invalidResponse = makeResponse({ reason: "Invalid email", valid: false });

const validateEmail = (value: unknown, regExp = EMAIL_REGEXP) => {
  if (typeof value !== "string") return invalidResponse;

  const validated = value?.trim();

  return regExp.test(validated)
    ? makeResponse<string>({ valid: true, validated })
    : invalidResponse;
};

type AllowConfig<T> =
  | ArrayOfMinSizeTwo<T>
  | { values: ArrayOfMinSizeTwo<T>; error: string };

type ExclusionConfig<T> =
  | T
  | ArrayOfMinSizeTwo<T>
  | { values: T | ArrayOfMinSizeTwo<T>; error: string };

type ValueError<T = number> = { value: T; error: string };

type NumberValidatorOptions<T extends number | unknown = number> = {
  exclude?: ExclusionConfig<T>;
} & XOR<
  { allow: AllowConfig<T> },
  {
    max?: number | ValueError;
    min?: number | ValueError;
    nullable?: boolean;
  }
>;

type StringValidatorOptions<T extends string | unknown = string> = {
  exclude?: ExclusionConfig<T>;
} & XOR<
  { allow: AllowConfig<T> },
  {
    max?: number | ValueError;
    min?: number | ValueError;
    nullable?: boolean;
    regExp?: ValueError<RegExp>;
    trim?: boolean;
  }
>;

function makeNumberValidator<const T extends number | unknown = number>({
  exclude,
  allow,
  max,
  min,
  nullable,
}: NumberValidatorOptions<T> = {}) {
  const {
    maxValue,
    minValue,
    hasMaxValue,
    hasMinValue,
    maxError,
    metadata,
    minError,
  } = _getMaxMinInfo({
    max,
    min,
    defaulMaxError: "too_big",
    defaulMinError: "too_small",
  });

  const exclusion = _getExclusionInfo(exclude);

  return (value: unknown): ValidationResponse<T> => {
    if (exclusion.hasExclusion && exclusion.excluded.includes(value as never))
      return makeResponse({
        valid: false,
        reason: exclusion.exclusionError,
        metadata: exclusion.metadata,
      });

    if (allow) {
      const { allowed, notAllowedError } = _getAllowedInfo(allow);

      return isOneOf(value, allowed)
        ? makeResponse({ valid: true, validated: value })
        : makeResponse({
            valid: false,
            reason: notAllowedError,
            metadata: { allowed },
          });
    }

    if (nullable && isOneOf(value, [null, undefined]))
      return makeResponse({ valid: true, validated: null as never as T });

    if (!["number", "bigint"].includes(typeof value) || isNaN(value as never))
      return makeResponse({ reason: "Expected a number", valid: false });

    const _value = Number(value);

    if (hasMinValue && _value < minValue!)
      return makeResponse({ valid: false, reason: minError, metadata });

    if (hasMaxValue && _value > maxValue!)
      return makeResponse({ valid: false, reason: maxError, metadata });

    return makeResponse({ valid: true, validated: _value as T });
  };
}

const MAX_STRING_LENGTH = 255,
  MIN_STRING_LENGTH = 1;

function makeStringValidator<const T extends string | unknown = string>({
  exclude,
  allow,
  max = MAX_STRING_LENGTH,
  min = MIN_STRING_LENGTH,
  nullable,
  regExp,
  trim,
}: StringValidatorOptions<T> = {}) {
  const {
    maxValue: maxLength,
    minValue: minLength,
    hasMaxValue: hasMinLength,
    hasMinValue: hasMaxLength,
    maxError,
    metadata,
    minError,
  } = _getMaxMinInfo({
    max,
    min,
    defaulMaxError: "too_long",
    defaulMinError: "too_short",
  });

  const exclusion = _getExclusionInfo(exclude);

  return (value: unknown): ValidationResponse<T> => {
    if (exclusion.hasExclusion && exclusion.excluded.includes(value as never))
      return makeResponse({
        valid: false,
        reason: exclusion.exclusionError,
        metadata: exclusion.metadata,
      });

    if (allow) {
      const { allowed, notAllowedError } = _getAllowedInfo(allow);

      return isOneOf(value, allowed)
        ? makeResponse({ valid: true, validated: value })
        : makeResponse({
            valid: false,
            reason: notAllowedError,
            metadata: { allowed },
          });
    }

    if (nullable && isOneOf(value, ["", null, undefined]))
      return makeResponse({ valid: true, validated: null as never as T });

    if (typeof value !== "string")
      return makeResponse({ reason: "Expected a string", valid: false });

    if (regExp && !regExp.value.test(value))
      return makeResponse({ valid: false, reason: regExp.error });

    let _value = String(value);

    if (trim) _value = _value.trim();

    if (hasMinLength && _value.length < minLength!)
      return makeResponse({ valid: false, reason: minError, metadata });

    if (hasMaxLength && _value.length > maxLength!)
      return makeResponse({ valid: false, reason: maxError, metadata });

    return makeResponse({ valid: true, validated: _value as T });
  };
}

function _getAllowedInfo<T>(allow: AllowConfig<T>): {
  allowed: ArrayOfMinSizeTwo<T>;
  notAllowedError?: string;
} {
  const isArray = Array.isArray(allow);

  return {
    allowed: isArray ? allow : allow.values,
    notAllowedError: isArray ? "Value not allowed" : allow.error,
  };
}

function _getExclusionInfo<T>(exclude?: ExclusionConfig<T>): {
  excluded: T[];
  exclusionError?: string;
  hasExclusion: boolean;
  metadata: { excluded: T[] } | null;
} {
  const isConfigObject = typeof exclude == "object" && !Array.isArray(exclude);

  const hasExclusion = !isNullOrUndefined(exclude);
  let excluded = isConfigObject
    ? (exclude as { values: T[] }).values
    : (exclude as T);

  if (!Array.isArray(excluded)) excluded = [excluded];

  const exclusionError = isConfigObject
    ? ((exclude as { error: string })?.error ?? "Value not allowed")
    : "Value not allowed";

  const metadata = hasExclusion ? { excluded } : null;

  return { excluded, exclusionError, hasExclusion, metadata };
}

function _getMaxMinInfo({
  max,
  min,
  defaulMaxError,
  defaulMinError,
}: {
  max?: number | ValueError;
  min?: number | ValueError;
  defaulMaxError: string;
  defaulMinError: string;
}): {
  maxValue: number | null;
  minValue: number | null;
  maxError?: string;
  minError?: string;
  hasMaxValue: boolean;
  hasMinValue: boolean;
  metadata: { max?: number; min?: number } | null;
} {
  const typeOfMaxConfig = typeof max;
  const isMaxConfigObject = typeOfMaxConfig == "object";
  const typeOfMinConfig = typeof min;
  const isMinConfigObject = typeOfMinConfig == "object";

  const maxValue = isMaxConfigObject
    ? (max as ValueError).value
    : ((max as number) ?? null);
  const maxError = isMaxConfigObject
    ? (max as ValueError).error
    : defaulMaxError;

  const minValue = isMinConfigObject
    ? (min as ValueError).value
    : ((min as number) ?? null);
  const minError = isMinConfigObject
    ? (min as ValueError).error
    : defaulMinError;

  const hasMaxValue = !isNullOrUndefined(maxValue),
    hasMinValue = !isNullOrUndefined(minValue);

  let metadata: { max?: number; min?: number } | null = {
    max: maxValue,
    min: minValue,
  };

  if (!hasMaxValue && !hasMinValue) metadata = null;
  else if (!hasMaxValue) delete metadata.max;
  else if (!hasMinValue) delete metadata.min;

  return {
    maxValue,
    maxError,
    minValue,
    minError,
    metadata,
    hasMaxValue,
    hasMinValue,
  };
}
