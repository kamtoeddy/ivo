import { makeUnique } from "../utils/functions";

type funcFilter = (data: any) => boolean;
type funcModifier = (data: any) => any;
type funcSorter = (a: any, b: any) => number;

export default function isArrayOk(
  value: any[] = [],
  {
    empty = false,
    sorted = true,
    sorter,
    sortOrder = -1,
    filter = (data) => false,
    modifier,
    unique = true,
    uniqueKey = "measureUnit",
  }: {
    empty?: boolean;
    sorted?: boolean;
    sortOrder?: number;
    unique?: boolean;
    uniqueKey?: string;
    filter?: funcFilter;
    modifier?: funcModifier;
    sorter?: funcSorter;
  } = {}
) {
  if (!Array.isArray(value))
    return { valid: false, reason: "Expected an array" };

  let _array = value.filter(filter);

  if (!empty && !_array.length)
    return { valid: false, reason: "Expected a non-empty array" };

  if (modifier) _array = _array.map(modifier);

  if (unique && _array.length) {
    _array =
      typeof _array[0] == "object"
        ? makeUnique({ data: _array, key: uniqueKey })
        : [...new Set(_array)];
  }

  if (sorted) {
    if (!sorter) {
      if (![-1, 1].includes(sortOrder)) sortOrder = -1;

      sorter = (a, b) => (a < b ? sortOrder : -sortOrder);
    }
    _array = _array.sort(sorter);
  }

  return { valid: true, validated: _array };
}
