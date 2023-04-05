import {
  expectFailure,
  expectNoFailure,
  expectPromiseFailure,
  getValidSchema,
} from "../_utils";

export const Test_SchemaOnSuccess = ({ Schema, fx }: any) => {
  describe("Schema.options.onSuccess", () => {
    describe("valid", () => {
      it("should allow 'onSuccess' as (() => any) | ((() => any)[])", () => {
        const values = [() => {}, [() => {}]];

        for (const onSuccess of values) {
          const toPass = fx(getValidSchema(), { onSuccess });

          expectNoFailure(toPass);

          toPass();
        }
      });
    });

    describe("invalid", () => {
      it("should reject 'onSuccess' other than (() => any) | ((() => any)[])", () => {
        const invalidValues = [
          1,
          0,
          -14,
          true,
          false,
          "invalid",
          "",
          null,
          undefined,
        ];

        for (const onSuccess of invalidValues) {
          const toFail = fx(getValidSchema(), { onSuccess });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: "Invalid Schema",
              payload: {
                onSuccess: expect.arrayContaining([
                  "The success listener @[0] is not a function",
                ]),
              },
              statusCode: 500,
            });
          }
        }
      });
    });
  });
};
