interface getUniquePropType {
  data: object[];
  key: number | string;
}

export function getUnique(prop: getUniquePropType) {
  // const { data = [], key = "id" }=prop
  const { data, key } = prop;

  const obj = {};

  data.forEach((dt) => (obj[dt[key]] = dt));

  return Object.values(obj);
}
