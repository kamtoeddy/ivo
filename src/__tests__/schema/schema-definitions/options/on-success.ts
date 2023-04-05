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
  });
};
