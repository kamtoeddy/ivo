import { ObjectType } from "./interfaces";
import { isEqual } from "./isEqual";

export const getDeepValue = (data: ObjectType, key: string): any => {
  return key.split(".").reduce((prev, next) => prev?.[next], data);
};

export const getSubObject = (obj: ObjectType, sampleSub: ObjectType) => {
  const _obj: ObjectType = {},
    keys = Object.keys(sampleSub);

  keys.forEach((key) => (_obj[key] = getDeepValue(obj, key)));

  return _obj;
};

type FindByOptions = { fromBack?: boolean };

type IFindBy = <T>(
  list: T[],
  determinant: any,
  options?: FindByOptions
) => T | undefined;

export const findBy: IFindBy = <T>(
  list: T[] = [],
  determinant: any,
  options: FindByOptions = { fromBack: false }
) => {
  const { fromBack } = options;
  const detType = typeof determinant;

  if (detType === "function") return list.find(determinant);

  if (fromBack) list = list.reverse();

  if (Array.isArray(determinant))
    return list.find((dt) => {
      const [key, value] = determinant;
      const dt_val = getDeepValue(dt as ObjectType, key);

      return isEqual(dt_val, value);
    });

  if (detType === "object")
    return list.find((dt) => {
      const sub = getSubObject(dt as ObjectType, determinant);

      return isEqual(determinant, sub);
    });

  return list.find((dt) => getDeepValue(dt as ObjectType, determinant));
};

type GetUniqueByOptions = { backwards?: boolean };

const getUnique = <T>(list: T[]) => {
  let _list = list.map((dt) => {
    try {
      return JSON.stringify(dt);
    } catch (err) {
      return dt;
    }
  });

  _list = Array.from(new Set(_list));

  return _list.map((dt) => {
    try {
      return JSON.parse(dt as string) as T;
    } catch (err) {
      return dt as T;
    }
  });
};

export const getUniqueBy = <T>(
  list: T[],
  key?: string,
  { backwards }: GetUniqueByOptions = { backwards: false }
) => {
  if (backwards) list = list.reverse();

  if (!key) return getUnique(list);

  let obj: ObjectType = {};

  list.forEach((dt) => (obj[getDeepValue(dt as ObjectType, key)] = dt));

  return Object.values(obj) as T[];
};
