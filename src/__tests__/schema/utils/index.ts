import { isEqual, ObjectType } from "../../../../lib";
import { getDeepValue } from "../../../../lib/utils/getUniqueBy";

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

export const getSubObject = (obj: ObjectType, sampleSub: ObjectType) => {
  const _obj: ObjectType = {},
    keys = Object.keys(sampleSub);

  keys.forEach((key) => (_obj[key] = getDeepValue(obj, key)));

  return _obj;
};
