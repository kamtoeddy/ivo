interface getUniqueProperties {
  data: any[];
  key: any;
}

const getUnique = (props: getUniqueProperties) => {
  // const { data = [], key = "id" }=props
  const { data, key } = props;

  const obj = {};

  data.forEach((dt) => (obj[dt[key]] = dt));

  return Object.values(obj);
};

module.exports = { getUnique };
