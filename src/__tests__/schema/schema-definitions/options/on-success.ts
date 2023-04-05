import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator,
} from "../_utils";

export const Test_SchemaOnSuccess = ({ Schema, fx }: any) => {
  describe("Schema.options.onSuccess", () => {
    describe("behaviour", () => {
      let successValues: any = {};

      function onSuccess_(prop = "") {
        return (summary: any) => (successValues[prop] = summary);
      }

      beforeEach(() => {
        successValues = {};
      });

      describe("behaviour with other success listeners", () => {
        const Book = new Schema(
          {
            id: { constant: true, value: 1, onSuccess: onSuccess_("id") },
            name: { required: true, validator, onSuccess: onSuccess_("name") },
            price: {
              default: null,
              dependent: true,
              dependsOn: "_setPrice",
              resolver: ({ _setPrice }: any) => _setPrice,
              onSuccess: onSuccess_("price"),
            },
            _setPrice: {
              virtual: true,
              validator,
              onSuccess: onSuccess_("_setPrice"),
            },
          },
          { onSuccess: onSuccess_("global") }
        ).getModel();

        it("should trigger all 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Book.create({
            name: "Book name",
            _setPrice: 100,
          });

          await handleSuccess();

          const values = { id: 1, name: "Book name", price: 100 };
          const summary = {
            context: { ...values, _setPrice: 100 },
            operation: "creation",
            previousValues: undefined,
            values: values,
          };

          expect(data).toEqual(values);
          expect(successValues).toEqual({
            id: summary,
            name: summary,
            price: summary,
            _setPrice: summary,
            global: summary,
          });
        });
      });
    });

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
