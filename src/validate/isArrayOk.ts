import { getUniqueBy } from "../utils/getUniqueBy";

type funcFilter = (data: any) => boolean;
type funcModifier = (data: any) => any;
type funcSorter = (a: any, b: any) => number;

export async function isArrayOk(
  arr: any[] = [],
  {
    empty = false,
    sorted = false,
    filter = (data) => false,
    modifier,
    sorter,
    sortOrder = -1,
    unique = true,
    uniqueKey = "",
  }: {
    empty?: boolean;
    filter?: funcFilter;
    modifier?: funcModifier;
    sorted?: boolean;
    sorter?: funcSorter;
    sortOrder?: number;
    unique?: boolean;
    uniqueKey?: string;
  } = {}
) {
  if (!Array.isArray(arr))
    return { valid: false, reasons: ["Expected an array"] };

  let _array = await Promise.all(arr.filter(filter));

  if (!empty && !_array.length)
    return { valid: false, reasons: ["Expected a non-empty array"] };

  if (modifier) {
    const copy = [];

    for (let dt of _array) {
      let res = await modifier(dt);

      copy.push(res);
    }

    _array = [...copy];
  }

  if (unique && _array.length) {
    const asObject = typeof _array[0] === "object" && uniqueKey;

    _array = asObject ? getUniqueBy(_array, uniqueKey) : getUniqueBy(_array);
  }

  if (sorted) {
    if (!sorter) {
      if (![-1, 1].includes(sortOrder)) sortOrder = -1;

      sorter = (a, b) => (a < b ? sortOrder : -sortOrder);
    }
    _array = await Promise.all(_array.sort(sorter));
  }

  return { reasons: [], valid: true, validated: _array };
}
