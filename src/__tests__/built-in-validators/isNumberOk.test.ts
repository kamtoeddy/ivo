import { isNumberOk } from "../../validate";
import { RangeType } from "../../validate/isNumberOk";

describe("Testing isNumberOk", () => {
  it("should whether input is a number or not", () => {
    // truthy values
    const truthyValues = [-45, 0, 0.23, 1, 100];

    for (let value of truthyValues) {
      expect(isNumberOk(value)).toMatchObject({
        reasons: [],
        valid: true,
        validated: value,
      });
    }

    // fasly values that will be parsed
    const falsyButParsed = ["-1", "", "1"];

    const parsedByValueMap: Record<string, number> = {
      "-1": -1,
      "": 0,
      "1": 1,
    };

    for (let value of falsyButParsed) {
      expect(isNumberOk(value)).toMatchObject({
        reasons: [],
        valid: true,
        validated: parsedByValueMap[value],
      });
    }

    // falsy values

    const falsyValues = [false, true, "hey", NaN, null, undefined, [], {}];

    for (let value of falsyValues) {
      expect(isNumberOk(value)).toMatchObject({
        reasons: ["Expected a number"],
        valid: false,
        validated: undefined,
      });
    }
  });

  it("should tell whether input is in range specified", () => {
    // truthy values
    const zero_Twelve: RangeType = { bounds: [0, 12] };
    const zero_Twelve_Ex: RangeType = {
      bounds: [0, 12],
      inclusiveBottom: false,
      inclusiveTop: false,
    };
    const zero_Twelve_ExBottom: RangeType = {
      bounds: [0, 12],
      inclusiveBottom: false,
    };
    const zero_Twelve_ExTop: RangeType = {
      bounds: [0, 12],
      inclusiveTop: false,
    };

    const pairsToPass: [number, RangeType][] = [
      [0, zero_Twelve],
      [5, zero_Twelve],
      [10.75, zero_Twelve],
      [12, zero_Twelve],

      // exclusive top & bottom
      [0.0001, zero_Twelve_Ex],
      [5, zero_Twelve_Ex],
      [10.75, zero_Twelve_Ex],
      [11.99, zero_Twelve_Ex],

      // exclusive bottom
      [0.0001, zero_Twelve_ExBottom],
      [5, zero_Twelve_ExBottom],
      [10.75, zero_Twelve_ExBottom],
      [12, zero_Twelve_ExBottom],

      // exclusive top
      [0, zero_Twelve_ExTop],
      [5, zero_Twelve_ExTop],
      [10.75, zero_Twelve_ExTop],
      [11.99, zero_Twelve_ExTop],
    ];

    pairsToPass.forEach(([num, range]) => {
      expect(isNumberOk(num, { range })).toMatchObject({
        valid: true,
      });
    });

    // falsy values
    const pairsToFail: [number, RangeType][] = [
      [-1, zero_Twelve],
      [12.01, zero_Twelve],

      // exclusive top & bottom
      [0, zero_Twelve_Ex],
      [12, zero_Twelve_Ex],

      // exclusive bottom
      [-1, zero_Twelve_ExBottom],
      [0, zero_Twelve_ExBottom],

      // exclusive top
      [12, zero_Twelve_ExTop],
      [12.01, zero_Twelve_ExTop],
    ];

    pairsToFail.forEach(([num, range]) => {
      expect(isNumberOk(num, { range })).toMatchObject({
        valid: false,
      });
    });
  });
});
