import { NumberRangeType } from "../utils/interfaces";

export type RangeType = undefined | NumberRangeType;

function isInRange(value: number, range: NumberRangeType) {
  const { bounds, inclusiveBottom, inclusiveTop } = range;
  const [min, max] = bounds;

  if ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min))
    return { reasons: ["Too small"], valid: false, validated: undefined };

  if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max))
    return { reasons: ["Too large"], valid: false, validated: undefined };

  return { reasons: [], valid: true, validated: value };
}

function makeRage(range: RangeType): RangeType {
  if (!range?.bounds) return undefined;

  const { inclusiveBottom, inclusiveTop } = range;

  if (typeof inclusiveBottom !== "boolean") range.inclusiveBottom = true;
  if (typeof inclusiveTop !== "boolean") range.inclusiveTop = true;

  return range;
}

export function isNumberOk(num: any, { range }: { range?: RangeType } = {}) {
  let valid = true,
    reasons: string[] = [];

  if (!["number", "string"].includes(typeof num) || isNaN(num)) {
    return {
      reasons: ["Expected a number"],
      valid: false,
      validated: undefined,
    };
  }

  num = Number(num);

  range = makeRage(range);

  if (range) {
    const _isInRange = isInRange(num, range);

    if (!_isInRange.valid) return _isInRange;
  }

  return { reasons, valid, validated: num as number };
}
