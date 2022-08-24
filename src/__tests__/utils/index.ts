interface CommonUtilsProps {
  [key: string]: Function;
}

export const commonUtilTests = ({
  belongsTo,
  getDeepValue,
  getUnique,
  getUniqueBy,
  isEqual,
  serialize,
}: CommonUtilsProps) => {
  describe("belongsTo", () => {
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

  describe("isEqual", () => {
    it("should return true if a and b are equal else false", () => {
      // truthy
      expect(isEqual(1, 1)).toEqual(true);
      expect(isEqual({}, {})).toEqual(true);
      expect(isEqual([], [])).toEqual(true);
      expect(isEqual(undefined, undefined)).toEqual(true);
      expect(isEqual([1, "true", [], null], [1, "true", [], null])).toEqual(
        true
      );
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

  describe("getDeepValue", () => {
    let person: any;

    beforeAll(() => {
      person = {
        name: "James",
        age: 20,
        bio: {
          joinDate: "today",
          facebook: { link: "/facebook/james", likes: 1700 },
        },
      };
    });

    it("should give value with simple keys", () => {
      const truthy: [string, any][] = [
        ["name", "James"],
        ["age", 20],
      ];

      for (const [key, value] of truthy) {
        expect(getDeepValue(person, key)).toBe(value);
      }
    });

    it("should give value with nested keys", () => {
      const truthy: [string, any][] = [
        ["bio.joinDate", "today"],
        ["bio.facebook.link", "/facebook/james"],
        ["bio.facebook.likes", 1700],
      ];

      for (const [key, value] of truthy) {
        expect(getDeepValue(person, key)).toBe(value);
      }
    });

    it("should give undefined if simple key is not set", () => {
      expect(getDeepValue(person, "dob")).toBe(undefined);
    });

    it("should give undefined if nested key is not set", () => {
      expect(getDeepValue(person, "address.streetName")).toBe(undefined);
    });
  });

  describe("getUnique", () => {
    it("should return an array of unique values", () => {
      const values = [
        11,
        1,
        { name: "James" },
        { name: "Mary" },
        2,
        { name: "James" },
        1,
      ];

      expect(getUnique([]).length).toBe(0);
      expect(getUnique(values).length).toBe(5);
    });
  });

  describe("getUniqueBy", () => {
    it("should return an array of unique values without a key", () => {
      const values = [
        11,
        1,
        { name: "James" },
        { name: "Mary" },
        2,
        { name: "James" },
        1,
      ];

      expect(getUniqueBy([]).length).toBe(0);
      expect(getUniqueBy(values).length).toBe(5);
    });

    it("should return an array of unique values with a key", () => {
      const values = [{ name: "James" }, { name: "Mary" }, { name: "James" }];

      expect(getUniqueBy(values, "name").length).toBe(2);
      expect(getUniqueBy(values, "age").length).toBe(1);
    });
  });

  describe("serialize", () => {
    it("should convert values to json strings", () => {
      const truthy = [
        ["1", '"1"'],
        [1, "1"],
        [{}, "{}"],
        [[], "[]"],
      ];

      for (const [key, value] of truthy) {
        expect(serialize(key)).toBe(value);
      }
    });

    it("should convert json values their original types", () => {
      const truthy = [
        ['"1"', "1"],
        ["1", 1],
      ];

      for (const [key, value] of truthy) {
        expect(serialize(key, true)).toBe(value);
      }
    });

    it("should not serialize nor deserialise undefined", () => {
      expect(serialize(undefined)).toBe(serialize(undefined, true));
    });
  });
};
