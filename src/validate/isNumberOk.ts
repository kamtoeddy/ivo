import { belongsTo } from "../utils/functions";

export interface NumberRangeType {
  bounds: number[];
  inclusiveBottom?: boolean;
  inclusiveTop?: boolean;
}

export type RangeType = undefined | NumberRangeType;

const bools = [false, true];

function isInRange(value: number, range: NumberRangeType) {
  const { bounds, inclusiveBottom, inclusiveTop } = range;
  const [min, max] = bounds;

  if ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min)) {
    return { valid: false, reasons: ["too small"], validated: undefined };
  }

  if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max)) {
    return { valid: false, reasons: ["too large"], validated: undefined };
  }

  return { valid: true, reasons: [] };
}

function makeRage(range: RangeType): RangeType {
  if (!range?.bounds) return undefined;

  const { inclusiveBottom, inclusiveTop } = range;

  if (!belongsTo(inclusiveBottom, bools)) range.inclusiveBottom = true;
  if (!belongsTo(inclusiveTop, bools)) range.inclusiveTop = true;

  return range;
}

export function isNumberOk(num: any, { range }: { range?: RangeType } = {}) {
  let valid = true,
    reasons: string[] = [];

  if (!["number", "string"].includes(typeof num) || isNaN(num)) {
    return {
      valid: false,
      reasons: ["Expected a number"],
      validated: undefined,
    };
  }

  num = Number(num);

  range = makeRage(range);

  if (range) {
    const _isInRange = isInRange(num, range);

    if (!_isInRange.valid) return _isInRange;
  }

  return { reasons, valid, validated: valid ? num : undefined };
}
