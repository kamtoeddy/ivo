import { ResponseInput } from "../schema/interfaces";
import { makeResponse } from "../schema/utils";
import { NumberRangeType } from "../utils/interfaces";

export type RangeType = undefined | NumberRangeType;

function isInRange(
  value: number,
  range: NumberRangeType
): ResponseInput<number> {
  const { bounds, inclusiveBottom, inclusiveTop } = range;
  const [min, max] = bounds;

  if ((inclusiveBottom && value < min) || (!inclusiveBottom && value <= min))
    return { reason: "Too small", valid: false };

  if ((inclusiveTop && value > max) || (!inclusiveTop && value >= max))
    return { reason: "Too large", valid: false };

  return { valid: true, validated: value };
}

function makeRage(range: RangeType): RangeType {
  if (!range?.bounds) return undefined;

  const { inclusiveBottom, inclusiveTop } = range;

  if (typeof inclusiveBottom !== "boolean") range.inclusiveBottom = true;
  if (typeof inclusiveTop !== "boolean") range.inclusiveTop = true;

  return range;
}

export function isNumberOk(num: any, { range }: { range?: RangeType } = {}) {
  let valid = true;

  if (!["number", "string"].includes(typeof num) || isNaN(num))
    return makeResponse({ reason: "Expected a number", valid: false });

  num = Number(num);

  range = makeRage(range);

  if (range) {
    const _isInRange = isInRange(num, range);

    if (!_isInRange.valid) return makeResponse(_isInRange);
  }

  return makeResponse<number>({ valid, validated: num });
}
