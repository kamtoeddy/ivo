export const trim = (value: string) => String(value).trim();

export function getSub(
  value: string,
  { start = 0, stop = 150 }: { start: number; stop: number }
) {
  return String(value).substring(start, stop);
}
