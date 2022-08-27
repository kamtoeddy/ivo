const failResponse = {
  reasons: ["Invalid card number"],
  valid: false,
  validated: undefined,
};

const isEven = (num: number) => num % 2 === 0;

const getSingleDigits = (value: number | string) => {
  return String(value)
    .split("")
    .filter((v) => !isNaN(parseInt(v)))
    .map(Number);
};

const getCheckSum = (values: number[]) => {
  const separated = getSingleDigits(values.map((v) => String(v)).join(""));

  return separated.map(Number).reduce((prev, next) => (prev += next));
};

const isCheckSumOk = (values: number[]) => {
  const controlNumber = values[15];
  const toCheck = values.slice(0, 15).map((v, i) => (isEven(i) ? 2 * v : v));

  return 10 - (getCheckSum(toCheck) % 10) === controlNumber;
};

export const isCreditCardOk = (value: any) => {
  const _value = String(value).trim();

  if (_value.length !== 16) return failResponse;

  const singleDigits = getSingleDigits(_value);

  if (singleDigits.length !== 16) return failResponse;

  if (!isCheckSumOk(singleDigits)) return failResponse;

  const validated = typeof value === "number" ? value : _value;

  return { reasons: [], valid: true, validated };
};
