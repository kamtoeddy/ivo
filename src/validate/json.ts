export function isJSON(value: any) {
  try {
    return JSON.parse(value) || !isNaN(value) ? true : false;
  } catch (err) {
    return false;
  }
}
