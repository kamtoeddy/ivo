import { belongsTo } from "../utils/functions";
import { isEqual } from "../utils/isEqual";

describe("Belongs to", () => {
  it("should return true if value passed is in array supplied else false", () => {
    const values = [1, "hey", null, undefined, false];

    // truthy tests
    expect(belongsTo(1, values)).toBe(true);
    expect(belongsTo("hey", values)).toBe(true);
    expect(belongsTo(null, values)).toBe(true);
    expect(belongsTo(undefined, values)).toBe(true);
    expect(belongsTo(false, values)).toBe(true);

    // falsy tests
    expect(belongsTo("1", values)).toBe(false);
    expect(belongsTo("Hey", values)).toBe(false);
    expect(belongsTo("null", values)).toBe(false);
    expect(belongsTo("undefined", values)).toBe(false);
    expect(belongsTo("false", values)).toBe(false);
    expect(belongsTo(2, values)).toBe(false);
    expect(belongsTo(true, values)).toBe(false);
  });
});

describe("Is Equal", () => {
  it("should return true if a and b are equal else false", () => {
    // truthy
    expect(isEqual(1, 1)).toEqual(true);
    expect(isEqual({}, {})).toEqual(true);
    expect(isEqual([], [])).toEqual(true);
    expect(isEqual([1, "true", [], null], [1, "true", [], null])).toEqual(true);
    expect(isEqual({ name: "James" }, { name: "James" })).toEqual(true);

    // falsy
    expect(isEqual(1, "1")).toEqual(false);
    expect(isEqual({}, "1")).toEqual(false);
    expect(isEqual([1, "true", []], [1, "true", "[]"])).toEqual(false);
    expect(isEqual([1, "true", [], null], [1, "true", null, []])).toEqual(
      false
    );
    expect(isEqual({ name: "James" }, { name: "JameS" })).toEqual(false);
    expect(isEqual({ name: "James" }, { name: "James", age: 17 })).toEqual(
      false
    );
  });
});
