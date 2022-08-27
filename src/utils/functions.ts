export function belongsTo(value: any, values: any[]): boolean {
  return values.includes(value);
}

export const toArray = (value: any) => (Array.isArray(value) ? value : [value]);
