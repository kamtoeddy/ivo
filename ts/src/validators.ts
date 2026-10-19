import { getUniqueBy, isNullOrUndefined, isOneOf, makeResponse } from './utils';
import type { ArrayOfMinSizeTwo, ValidationResponse, XOR } from './utils/types';

export type {
  ArrayValidatorOptions,
  NumberValidatorOptions,
  StringValidatorOptions,
};
export {
  makeArrayValidator,
  makeNumberValidator,
  makeStringValidator,
  validateBoolean,
  validateCreditCard,
  validateEmail,
};

type ArrayValidatorOptions<PreFilteredType, ModType, FinalType> = {
  max?: number | ValueError;
  min?: number | ValueError;
  unique?: boolean;
  uniqueKey?: string;
} & ArrayFilterOptions<PreFilteredType, ModType, FinalType> &
  ArraySortOptions<FinalType>;

type ArrayFilterOptions<PreFilteredType, ModType, FinalType> =
  | {
      filter: ArrayFilterFn<FinalType>;
      modifier?: never;
      postModFilter?: never;
      map?: never;
    }
  | {
      filter: ArrayFilterFn<PreFilteredType>;
      modifier: (item: PreFilteredType) => ModType | Promise<ModType>;
      postModFilter?: (item: ModType) => boolean | Promise<boolean>;
      map?: (item: ModType) => FinalType | Promise<FinalType>;
    };

type ArrayFilterFn<T> =
  | ((item: unknown) => item is T)
  | ((item: unknown) => boolean | Promise<boolean>);

type ArraySortOptions<T> =
  | { sort?: (a: T, b: T) => number; sortOrder?: never }
  | { sort?: boolean; sortOrder?: 'asc' | 'desc' };

function makeArrayValidator<
  const PreFilteredType,
  const ModType = PreFilteredType,
  const FinalType = ModType,
>({
  filter,
  map,
  modifier,
  postModFilter,
  max,
  min,
  sort,
  sortOrder,
  unique,
  uniqueKey,
}: ArrayValidatorOptions<PreFilteredType, ModType, FinalType>) {
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
    defaulMaxError: 'Max limit reached',
    defaulMinError: 'Expected a non-empty array',
  });

  if (!filter) throw new Error('Array validator must have a filter function');

  return async (value: unknown): Promise<ValidationResponse<FinalType[]>> => {
    if (!Array.isArray(value))
      return makeResponse({ reason: 'Expected an array', valid: false });

    let _array = await Promise.all(await _filterBy(value, filter as never));

    if (modifier) _array = await Promise.all(_array.map(modifier));

    if (postModFilter)
      _array = await Promise.all(await _filterBy(_array, postModFilter));

    if (unique && _array.length)
      _array = uniqueKey ? getUniqueBy(_array, uniqueKey) : getUniqueBy(_array);

    if (hasMinValue && _array.length < minValue!)
      return makeResponse({ valid: false, reason: minError, metadata });

    if (hasMaxValue && _array.length > maxValue!)
      return makeResponse({ valid: false, reason: maxError, metadata });

    if (map) _array = await Promise.all(_array.map(map));

    if (sort) {
      const order = _getArrayOrder(sortOrder);

      const sorter =
        // @ts-expect-error lol
        typeof sort === 'boolean' ? (a, b) => (a < b ? order : -order) : sort;

      // `_array` is always a fresh array by now, so it can be sorted in place
      _array.sort(sorter);
    }

    return makeResponse({ valid: true, validated: _array });
  };
}

/** keeps the items `predicate` (sync or async) accepts */
async function _filterBy<T>(
  list: T[],
  predicate: (value: T, index: number, array: T[]) => unknown,
): Promise<T[]> {
  const verdicts = await Promise.all(list.map(predicate));

  return list.filter((_, index) => verdicts[index]);
}

const _getArrayOrder = (sortOrder: unknown) => {
  if (!['asc', 'desc'].includes(sortOrder as never)) return -1;

  return sortOrder === 'asc' ? -1 : 1;
};

function validateBoolean(value: unknown) {
  return makeResponse<boolean>(
    typeof value === 'boolean'
      ? { valid: true, validated: value }
      : { valid: false, reason: 'Expected a boolean' },
  );
}

const invalidCardResponse = makeResponse({
  reason: 'Invalid card number',
  valid: false,
});

const validateCreditCard = (value: unknown) => {
  const _value = String(value).trim();

  if (_value.length !== 16) return invalidCardResponse;

  // Luhn check in one pass: every other digit (from the first) is doubled,
  // and a doubled value's digits are summed (12 -> 1 + 2, i.e. 12 - 9)
  let sum = 0;

  for (let i = 0; i < 15; i++) {
    const digit = _value.charCodeAt(i) - 48;

    if (digit < 0 || digit > 9) return invalidCardResponse;

    const v = i % 2 === 0 ? digit * 2 : digit;

    sum += v > 9 ? v - 9 : v;
  }

  const controlNumber = _value.charCodeAt(15) - 48;

  // `% 10` again so a sum that's already a multiple of 10 expects a control
  // number of 0, not 10
  if (
    controlNumber < 0 ||
    controlNumber > 9 ||
    (10 - (sum % 10)) % 10 !== controlNumber
  )
    return invalidCardResponse;

  const validated = typeof value === 'number' ? value : _value;

  return makeResponse<string | number>({ valid: true, validated });
};

const EMAIL_REGEXP =
  // biome-ignore lint/suspicious/noControlCharactersInRegex: lol
  /^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$/;

const getInvalidEmailResponse = (value: unknown) =>
  makeResponse({ reason: 'Invalid email', valid: false, value });

const validateEmail = (value: unknown, regExp = EMAIL_REGEXP) => {
  if (typeof value !== 'string') return getInvalidEmailResponse(value);

  const validated = value?.trim();

  return regExp.test(validated)
    ? makeResponse<string>({ valid: true, validated })
    : getInvalidEmailResponse(value);
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
    /**
     * escape `&`, `<`, `>`, `"` and `'` as HTML entities in the validated
     * value (default: `true`). Length limits apply to the escaped text.
     */
    escapeHTML?: boolean;
    max?: number | ValueError;
    min?: number | ValueError;
    normalForm?: 'NFC' | 'NFD' | 'NFKC' | 'NFKD';
    normalize?: boolean;
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
    defaulMaxError: 'too_big',
    defaulMinError: 'too_small',
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

    if (
      !['number', 'bigint'].includes(typeof value) ||
      (typeof value === 'number' && isNaN(value))
    )
      return makeResponse({ reason: 'Expected a number', valid: false });

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

const HTML_ESCAPES: Record<string, string> = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
  "'": '&#39;',
};

const HTML_SPECIAL_CHARS = /[&<>"']/;
const HTML_SPECIAL_CHARS_GLOBAL = /[&<>"']/g;

function _escapeHTML(value: string) {
  // most strings have nothing to escape: skip building a copy for them
  return HTML_SPECIAL_CHARS.test(value)
    ? value.replace(HTML_SPECIAL_CHARS_GLOBAL, (char) => HTML_ESCAPES[char]!)
    : value;
}

function makeStringValidator<const T extends string | unknown = string>({
  exclude,
  allow,
  escapeHTML = true,
  max = MAX_STRING_LENGTH,
  min = MIN_STRING_LENGTH,
  normalForm,
  normalize,
  nullable,
  regExp,
  trim,
}: StringValidatorOptions<T> = {}) {
  normalize ??= true;

  if (normalForm && !['NFC', 'NFD', 'NFKC', 'NFKD'].includes(normalForm))
    throw new Error('Invalid string normalization form');

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
    defaulMaxError: 'too_long',
    defaulMinError: 'too_short',
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

    if (nullable && isOneOf(value, ['', null, undefined]))
      return makeResponse({ valid: true, validated: null as never as T });

    if (typeof value !== 'string')
      return makeResponse({ reason: 'Expected a string', valid: false });

    if (regExp && !regExp.value.test(value))
      return makeResponse({ valid: false, reason: regExp.error });

    let _value = String(value);

    if (normalize) _value = _value.normalize(normalForm);

    if (trim) _value = _value.trim();

    if (escapeHTML) _value = _escapeHTML(_value);

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
    allowed: isArray ? allow : (allow.values as any),
    notAllowedError: isArray ? 'Value not allowed' : (allow as any).error,
  };
}

function _getExclusionInfo<T>(exclude?: ExclusionConfig<T>): {
  excluded: T[];
  exclusionError?: string;
  hasExclusion: boolean;
  metadata: { excluded: T[] } | null;
} {
  const isConfigObject = typeof exclude === 'object' && !Array.isArray(exclude);

  const hasExclusion = !isNullOrUndefined(exclude);
  let excluded = isConfigObject
    ? (exclude as { values: T[] }).values
    : (exclude as T);

  if (!Array.isArray(excluded)) excluded = [excluded];

  const exclusionError = isConfigObject
    ? ((exclude as { error: string })?.error ?? 'Value not allowed')
    : 'Value not allowed';

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
  const isMaxConfigObject = typeOfMaxConfig === 'object';
  const typeOfMinConfig = typeof min;
  const isMinConfigObject = typeOfMinConfig === 'object';

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
