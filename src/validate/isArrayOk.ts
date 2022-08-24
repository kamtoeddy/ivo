import { getUniqueBy } from "../utils/getUniqueBy";
import { IArrayOptions } from "../utils/interfaces";

const validated = undefined;

const getOrder = (sortOrder: any) => {
  if (!["asc", "desc"].includes(sortOrder)) return -1;

  return sortOrder === "asc" ? -1 : 1;
};

export async function isArrayOk<T>(
  arr: T[],
  {
    empty = false,
    sorted = false,
    filter,
    modifier,
    sorter,
    sortOrder = "asc",
    unique = true,
    uniqueKey = "",
  }: IArrayOptions<T> = {}
) {
  if (!Array.isArray(arr))
    return { valid: false, validated, reasons: ["Expected an array"] };

  let _array = [...arr];

  if (filter) _array = await Promise.all(arr.filter(filter));

  if (!empty && !_array.length)
    return { valid: false, validated, reasons: ["Expected a non-empty array"] };

  if (modifier) {
    const copy = [];

    for (let dt of _array) {
      let res = await modifier(dt);

      copy.push(res);
    }

    _array = [...copy];
  }

  if (unique && _array.length)
    _array = uniqueKey ? getUniqueBy(_array, uniqueKey) : getUniqueBy(_array);

  if (sorted || sorter) {
    if (!sorter) {
      const order = getOrder(sortOrder);

      sorter = (a, b) => (a < b ? order : -order);
    }
    _array = await Promise.all(_array.sort(sorter));
  }

  return { reasons: [], valid: true, validated: _array };
}
