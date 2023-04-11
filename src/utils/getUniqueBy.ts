import { ObjectType } from "./interfaces";

export const getDeepValue = (data: ObjectType, key: string): any => {
  return key.split(".").reduce((prev, next) => prev?.[next], data);
};

export const serialize = (dt: any, revert = false) => {
  try {
    return revert ? JSON.parse(dt) : JSON.stringify(dt);
  } catch (err) {
    return dt;
  }
};

export const getUnique = <T>(list: T[]) => {
  let _list = list.map((dt) => serialize(dt));

  _list = Array.from(new Set(_list));

  return _list.map((dt) => serialize(dt, true));
};

export const getUniqueBy = <T>(list: T[], key?: string) => {
  if (!key) return getUnique(list);

  const obj: ObjectType = {};

  list.forEach((dt) => (obj[getDeepValue(dt as ObjectType, key)] = dt));

  return Object.values(obj) as T[];
};
