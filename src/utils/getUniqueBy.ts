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

type IFindBy = (list: any[], determinant: any, options?: FindByOptions) => any;

export const findBy: IFindBy = (
  list = [],
  determinant,
  options = { fromBack: false }
) => {
  const { fromBack } = options;
  const detType = typeof determinant;

  if (detType === "function") return list.find(determinant);

  if (fromBack) list = list.reverse();

  if (Array.isArray(determinant))
    return list.find((dt) => {
      const [key, value] = determinant;
      const dt_val = getDeepValue(dt, key);

      return isEqual(dt_val, value);
    });

  if (detType === "object")
    return list.find((dt) => {
      const sub = getSubObject(dt, determinant);

      return isEqual(determinant, sub);
    });

  return list.find((dt) => getDeepValue(dt, determinant));
};

type GetUniqueByOptions = { backwards?: boolean };

const getUnique = (list: any[]) => {
  list = list.map((dt) => {
    try {
      return JSON.stringify(dt);
    } catch (err) {
      return dt;
    }
  });

  list = Array.from(new Set(list));

  list = list.map((dt) => {
    try {
      return JSON.parse(dt);
    } catch (err) {
      return dt;
    }
  });

  return list;
};

export const getUniqueBy = (
  list: any[],
  key?: string,
  { backwards }: GetUniqueByOptions = { backwards: false }
) => {
  if (backwards) list = list.reverse();

  if (!key) return getUnique(list);

  let obj: ObjectType = {};

  list.forEach((dt) => (obj[getDeepValue(dt, key)] = dt));

  return Object.values(obj);
};
