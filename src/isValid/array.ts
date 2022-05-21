import { getUnique } from "../utils/index";

type funcFilter = (data: any) => boolean;
type funcModifier = (data: any) => any;
type funcSorter = (a: any, b: any) => number;

interface arrayOptions {
  empty: boolean;
  sorted: boolean;
  sortOrder: number;
  unique: boolean;
  uniqueKey: string;

  filter: funcFilter;
  modifier: funcModifier;
  sorter: funcSorter;
}

// {
//     empty = false,
//     sorted = true,
//     sorter,
//     sortOrder = -1,
//     filter = (data) => false,
//     modifier,
//     unique = true,
//     uniqueKey = "measureUnit",
//   }

const array = (value: any = [], options: arrayOptions) => {
  let {
    empty,
    sorted,
    sorter,
    sortOrder,
    filter,
    modifier,
    unique,
    uniqueKey,
  } = options;
  if (!Array.isArray(value))
    return { valid: false, reasons: ["Expected an array"] };

  let _array = value.filter(filter);

  if (!empty && !_array.length)
    return { valid: false, reasons: ["Expected a non-empty array"] };

  if (modifier) _array = _array.map(modifier);

  if (unique && _array.length) {
    _array =
      typeof _array[0] == "object"
        ? getUnique({ data: _array, key: uniqueKey })
        : [...new Set(_array)];
  }

  if (sorted) {
    if (!sorter) {
      if (![-1, 1].includes(sortOrder)) sortOrder = -1;
      sorter = (a, b) => (a < b ? sortOrder : -sortOrder);
    }
    _array = _array.sort(sorter);
  }

  return { valid: true, valdated: _array };
};
