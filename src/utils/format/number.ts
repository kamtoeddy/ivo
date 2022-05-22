export function toDecimalPoints({
  value,
  dps = 2,
}: {
  value: number;
  dps: number;
}) {
  return Number(value.toFixed(dps));
}
