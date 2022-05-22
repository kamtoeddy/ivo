interface numRangeType {
  bounds: number[];
  inclusiveBottom?: boolean;
  inclusiveTop?: boolean;
}

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

export default function isNumberOK(
  value: any,
  { range = undefined }: { range?: numRangeType } = {}
) {
  let valid = true,
    reason = "";

  if (isNaN(value) || value === "") {
    return { valid: false, reason: "Invalid data type" };
  }

  value = Number(value);

  if (range) {
    const _isInRange = isInRange(value, range);

    if (!_isInRange.valid) return _isInRange;
  }

  return { valid, reason };
}
