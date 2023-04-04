export const Test_Validators = ({ Schema }: any) => {
  describe("Model.validate", () => {
    describe("should respect the validator provided", () => {
      const Model = new Schema({
        prop: {
          default: "",
          validator(value: any) {
            return value == "valid"
              ? { valid: true }
              : { valid: false, reason: "Invalid prop" };
          },
        },
      }).getModel();

      it("should return the correct invalid message on validation failure", async () => {
        const res = await Model.validate("prop", "yoo");

        expect(res).toEqual({ valid: false, reasons: ["Invalid prop"] });
      });

      it("should respect the validator provided", async () => {
        const res = await Model.validate("prop", "valid");

        expect(res).toEqual({ valid: true, validated: "valid" });
      });
    });

    describe("should respect the validators that return booleans", () => {
      const Model = new Schema({
        prop: {
          default: "",
          validator: (value: any) => value == "valid",
        },
      }).getModel();

      it("should return the correct invalid message on validation failure", async () => {
        const res = await Model.validate("prop", "yoo");

        expect(res).toEqual({ valid: false, reasons: ["validation failed"] });
      });

      it("should respect the validator provided", async () => {
        const res = await Model.validate("prop", "valid");

        expect(res).toEqual({ valid: true, validated: "valid" });
      });
    });
  });
};
