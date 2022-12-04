export const valuesParsing_Tests = ({ Schema }: any) => {
  const fx =
    (definition: any = undefined, options: any = { timestamps: false }) =>
    () =>
      new Schema(definition, options);

  const expectFailure = (fx: Function, message = "Invalid Schema") => {
    expect(fx).toThrow(message);
  };

  const expectPromiseFailure = (fx: Function, message = "Invalid Schema") => {
    expect(fx).rejects.toThrow(message);
  };

  const expectNoFailure = (fx: Function) => {
    expect(fx).not.toThrow();
  };

  const validator = () => ({ valid: true });

  describe("Values Parsing", () => {
    const validData = { age: 15, name: "Frank" };
    const invalidValues = [1, -10, 0, false, true, "", "true", undefined, null];

    describe("with silent errors", () => {
      let User: any;

      beforeAll(async () => {
        User = new Schema({
          age: { default: 10 },
          id: { constant: true, value: 1 },
          name: { default: "" },
        }).getModel();
      });

      describe("valid", () => {
        it("should set values properly at creation", async () => {
          const { data, error } = await User.create(validData);

          expect(error).toBeUndefined();

          expect(data).toEqual({ ...validData, id: 1 });
        });
      });

      // describe("invalid", () => {

      // });
    });

    // describe("with thrown errors", () => {

    // });
  });
};
