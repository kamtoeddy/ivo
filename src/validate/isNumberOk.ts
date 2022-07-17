import { belongsTo } from "../utils/functions";

type numRangeType = {
  bounds: number[];
  inclusiveBottom?: boolean;
  inclusiveTop?: boolean;
};

type rangeType = undefined | numRangeType;

const bools = [false, true];

function isInRange(value: number, range: numRangeType) {
  const { bounds, inclusiveBottom, inclusiveTop } = range;
  const [min, max] = bounds;

  if ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min)) {
    return { valid: false, reasons: ["too small"] };
  }

  if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max)) {
    return { valid: false, reasons: ["too large"] };
  }

  return { valid: true, reasons: [] };
}

function makeRage(range: rangeType): rangeType {
  if (!range?.bounds) return undefined;

  const { inclusiveBottom, inclusiveTop } = range;

  if (!belongsTo(inclusiveBottom, bools)) range.inclusiveBottom = true;
  if (!belongsTo(inclusiveTop, bools)) range.inclusiveTop = true;

  return range;
}

export function isNumberOK(num: any, { range }: { range?: rangeType } = {}) {
  let valid = true,
    reasons: string[] = [];

  if (isNaN(num) || num === "") {
    return { valid: false, reasons: ["Expected a number"] };
  }

  num = Number(num);

  range = makeRage(range);

  if (range) {
    const _isInRange = isInRange(num, range);

    if (!_isInRange.valid) return _isInRange;
  }

  return { reasons, valid, validated: valid ? num : undefined };
}
