import { isEqual } from "../utils/isEqual";

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
