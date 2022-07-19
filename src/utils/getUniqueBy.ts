import { ILooseObject } from "./interfaces";

export const getDeepValue = (
  data: ILooseObject,
  { key }: { key: string }
): any => {
  return key.split(".").reduce((prev, next) => prev?.[next], data);
};

type options = { backwards?: boolean };

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
  { backwards }: options = { backwards: false }
) => {
  if (backwards) list = list.reverse();

  if (!key) return getUnique(list);

  let obj: ILooseObject = {};

  list.forEach((dt) => (obj[getDeepValue(dt, { key })] = dt));

  return Object.values(obj);
};
