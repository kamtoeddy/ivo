import { isEqual, ObjectType } from '../../dist'

export { findBy, getSubObject }

function findBy<T>(list: T[] = [], determinant: any): T | undefined {
  return list.find((dt) => {
    const sub = getSubObject(dt as ObjectType, determinant)

    return isEqual(determinant, sub)
  })
}

function getSubObject(obj: ObjectType, sampleSub: ObjectType) {
  const _obj: ObjectType = {},
    keys = Object.keys(sampleSub)

  keys.forEach((key) => (_obj[key] = getDeepValue(obj, key)))

  return _obj
}

function getDeepValue(data: ObjectType, key: string): any {
  return key.split('.').reduce((prev, next) => prev?.[next], data)
}
