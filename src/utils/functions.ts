export const belongsTo = (value: any, values: any[]) => values.includes(value);

export const toArray = (value: any) => (Array.isArray(value) ? value : [value]);
