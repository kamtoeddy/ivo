const ApiError = require("./ApiError");

const isJSON = (value: any) => {
  try {
    return JSON.parse(value) || !isNaN(value) ? true : false;
  } catch (err) {
    return false;
  }
};

const getDateString = (value: Date | string) =>
  new Date(value).toISOString().substring(0, 10);

const getPeriod = ({
  start = new Date(),
  stop,
  distance = 1,
  useDistance = false,
}) => {
  let otherTime;

  if (!start && useDistance) {
    otherTime = new Date(getDateString(stop || new Date()));
    start = new Date(otherTime.getTime() - distance * 24 * 60 * 60 * 1000);
  }

  if (!stop && useDistance) {
    otherTime = new Date(getDateString(start));
    stop = new Date(otherTime.getTime() + distance * 24 * 60 * 60 * 1000);
  }

  return {
    start: new Date(getDateString(start)),
    stop: new Date(getDateString(stop)),
  };
};

const getUnique = ({ data = [], key = "id" }) => {
  const obj = {};

  data.forEach((dt) => (obj[dt[key]] = dt));

  return Object.values(obj);
};

const array = (
  value = [],
  {
    empty = false,
    sorted = true,
    sorter,
    sortOrder = -1,
    filter = (data) => false,
    modifier,
    unique = true,
    uniqueKey = "measureUnit",
  }
) => {
  if (!Array.isArray(value))
    return { valid: false, reasons: ["Expected an array"] };

  let _array = value.filter(filter);

  if (!empty && !_array.length)
    return { valid: false, reasons: ["Expected a non-empty array"] };

  if (modifier) _array = _array.map(modifier);

  if (unique && _array.length) {
    _array =
      typeof _array[0] == "object"
        ? getUnique({ data: _array, key: uniqueKey })
        : [...new Set(_array)];
  }

  if (sorted) {
    if (!sorter) {
      if (![-1, 1].includes(sortOrder)) sortOrder = -1;
      sorter = (a, b) => (a < b ? sortOrder : -sortOrder);
    }
    _array = _array.sort(sorter);
  }

  return { valid: true, valdated: _array };
};

const emailRegex =
  /^[a-z\d]([a-z\d\.-_]*)([a-z\d])@([a-z\d-\.]+)\.([a-z]{2,})(\.[a-z]{2,})?$/;
const phoneRegex = /^(\+)?([2][3][7])?( )?6([5,6,7,8,9])([0-9]){7}$/;

const dbNameRegex = /^[a-zA-Z_\-\S]+$/;

const email = (value) => (emailRegex.test(value) ? value : false);
const dbName = (value) => (dbNameRegex.test(value) ? value : false);
const phone = (value) => (phoneRegex.test(value) ? value : false);

const boolean = (value) => {
  let valid = true,
    reason = "";

  if (![false, true].includes(value)) {
    valid = false;
    reason = "Expected a boolean";
  }

  return { valid, reason };
};

const _number = {
  isInRange(value, range) {
    if (!this.isRange(range)) return { valid: false, reason: "invalid range" };

    const { bounds, inclusiveBottom, inclusiveTop } = range;
    const [min, max] = bounds;

    if (
      (inclusiveBottom && value < min) ||
      (!inclusiveBottom && value <= min)
    ) {
      return { valid: false, reason: "too small" };
    }

    if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max)) {
      return { valid: false, reason: "too large" };
    }

    return { valid: true, reason: "" };
  },

  isRange(range) {
    if (!range || typeof range !== "object") return false;

    const [min, max] = range.bounds;

    if (min >= max) return false;

    return true;
  },

  getNumbersRange(range) {
    if (!this.isRange(range)) return null;

    if (!range.hasOwnProperty("inclusiveBottom")) {
      range.inclusiveBottom = true;
    }

    if (!range.hasOwnProperty("inclusiveTop")) {
      range.inclusiveTop = true;
    }

    return range;
  },
};

const number = (value, { range } = {}) => {
  let valid = true,
    reason = "";

  if (isNaN(value) || value === "") {
    return { valid: false, reason: "Invalid data type" };
  }

  value = Number(value);

  range = _number.getNumbersRange(range);

  if (range) {
    const _isInRange = _number.isInRange(value, range);

    if (!_isInRange.valid) return _isInRange;
  }

  return { valid, reason };
};

const string = (
  value,
  { maxLength = 150, minLength = 1, enums } = {
    maxLength: 150,
    minLength: 1,
  }
) => {
  let valid = true,
    reason = "";

  if (!["bigint", "boolean", "number", "string"].includes(typeof value)) {
    return { valid: false, reason: "Expected a string" };
  }

  value = String(value).trim();

  if (value.length < minLength) {
    valid = false;
    reason = "too short";
  }

  if (value.length > maxLength) {
    valid = false;
    reason = "too long";
  }

  if (enums && !enums.includes(value)) {
    valid = false;
    reason = "unaccepted value";
  }

  return { valid, reason };
};

module.exports = {
  array,
  boolean,
  dbName,
  email,
  getPeriod,
  isJSON,
  number,
  phone,
  string,
};
