import { isAcceptable } from "../utils/functions";

type numRangeType = {
  bounds: number[];
  inclusiveBottom?: boolean;
  inclusiveTop?: boolean;
};

type rangeType = undefined | numRangeType;

function isInRange(value: number, range: numRangeType) {
  const { bounds, inclusiveBottom, inclusiveTop } = range;
  const [min, max] = bounds;

  if ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min)) {
    return { valid: false, reason: "too small" };
  }

  if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max)) {
    return { valid: false, reason: "too large" };
  }

  return { valid: true, reason: "" };
}

function makeRage(range: rangeType): rangeType {
  if (!range?.bounds) return undefined;

  const { inclusiveBottom, inclusiveTop } = range;

  if (!isAcceptable(inclusiveBottom)) range.inclusiveBottom = true;
  if (!isAcceptable(inclusiveTop)) range.inclusiveTop = true;

  return range;
}

export default function isNumberOK(
  value: any,
  { range }: { range?: rangeType } = {}
) {
  let valid = true,
    reason = "";

  if (isNaN(value) || value === "") {
    return { valid: false, reason: "Invalid data type" };
  }

  value = Number(value);

  console.log(isAcceptable("undefined", { toAccept: ["yes"] }));

  range = makeRage(range);

  if (range) {
    const _isInRange = isInRange(value, range);

    if (!_isInRange.valid) return _isInRange;
  }

  return { reason, valid, validated: value };
}
