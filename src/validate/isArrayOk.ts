import { makeResponse } from '../schema/utils'
import { getUniqueBy } from '../utils/getUniqueBy'
import { IArrayOptions } from '../utils/types'

const getOrder = (sortOrder: any) => {
  if (!['asc', 'desc'].includes(sortOrder)) return -1

  return sortOrder === 'asc' ? -1 : 1
}

export async function isArrayOk<T>(
  arr: T[],
  {
    empty = false,
    sorted = false,
    filter,
    modifier,
    sorter,
    sortOrder = 'asc',
    unique = true,
    uniqueKey = ''
  }: IArrayOptions<T> = {}
) {
  if (!Array.isArray(arr))
    return makeResponse({ reason: 'Expected an array', valid: false })

  let _array = [...arr]

  if (filter) _array = await Promise.all(arr.filter(filter))

  if (!empty && !_array.length)
    return makeResponse({ reason: 'Expected a non-empty array', valid: false })

  if (modifier) _array = await Promise.all(_array.map(modifier))

  if (unique && _array.length)
    _array = uniqueKey ? getUniqueBy(_array, uniqueKey) : getUniqueBy(_array)

  if (sorted || sorter) {
    if (!sorter) {
      const order = getOrder(sortOrder)

      sorter = (a, b) => (a < b ? order : -order)
    }
    _array = await Promise.all(_array.sort(sorter))
  }

  return makeResponse<T[]>({ valid: true, validated: _array })
}
