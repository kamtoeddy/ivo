import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator,
} from "../_utils";

export const Test_SchemaOnDelete = ({ Schema, fx }: any) => {
  describe("Schema.options.onDelete", () => {
    // describe("behaviour", () => {
    //   let successValues: any = {};

    //   function onDelete_(prop = "") {
    //     return (summary: any) => (successValues[prop] = summary);
    //   }

    //   beforeEach(() => {
    //     successValues = {};
    //   });

    //   describe("behaviour with other success listeners", () => {
    //     const Book = new Schema(
    //       {
    //         id: { constant: true, value: 1, onDelete: onDelete_("id") },
    //         name: { required: true, validator, onDelete: onDelete_("name") },
    //         price: {
    //           default: null,
    //           dependent: true,
    //           dependsOn: "_setPrice",
    //           resolver: ({ context }: any) => context._setPrice,
    //           onDelete: onDelete_("price"),
    //         },
    //         _setPrice: {
    //           virtual: true,
    //           validator,
    //           onDelete: onDelete_("_setPrice"),
    //         },
    //       },
    //       { onDelete: onDelete_("global") }
    //     ).getModel();

    //     it("should trigger all 'success' listeners at creation", async () => {
    //       const { data, handleSuccess } = await Book.create({
    //         name: "Book name",
    //         _setPrice: 100,
    //       });

    //       await handleSuccess();

    //       const values = { id: 1, name: "Book name", price: 100 };
    //       const summary = {
    //         context: { ...values, _setPrice: 100 },
    //         operation: "creation",
    //         previousValues: undefined,
    //         values: values,
    //       };

    //       expect(data).toEqual(values);
    //       expect(successValues).toEqual({
    //         id: summary,
    //         name: summary,
    //         price: summary,
    //         _setPrice: summary,
    //         global: summary,
    //       });
    //     });

    //     it("should trigger all 'success' listeners during cloning ", async () => {
    //       const book = { id: 1, name: "Book name", price: 100 };

    //       const { data, handleSuccess } = await Book.clone({
    //         ...book,
    //         _setPrice: 100,
    //       });

    //       await handleSuccess();

    //       const summary = {
    //         context: { ...book, _setPrice: 100 },
    //         operation: "creation",
    //         previousValues: undefined,
    //         values: book,
    //       };

    //       expect(data).toEqual(book);
    //       expect(successValues).toEqual({
    //         id: summary,
    //         name: summary,
    //         price: summary,
    //         _setPrice: summary,
    //         global: summary,
    //       });
    //     });

    //     it("should trigger all 'success' listeners during updates ", async () => {
    //       const book = { id: 1, name: "Book name", price: 100 };

    //       const { data, handleSuccess } = await Book.update(book, {
    //         _setPrice: 200,
    //       });

    //       await handleSuccess();

    //       const values = { ...book, price: 200 };

    //       const summary = {
    //         context: { ...values, _setPrice: 200 },
    //         operation: "update",
    //         previousValues: book,
    //         values: values,
    //       };

    //       expect(data).toEqual({ price: 200 });
    //       expect(successValues).toEqual({
    //         price: summary,
    //         _setPrice: summary,
    //         global: summary,
    //       });
    //     });
    //   });

    //   describe("behaviour without other success listeners", () => {
    //     const Book = new Schema(
    //       {
    //         id: { constant: true, value: 1 },
    //         name: { required: true, validator },
    //         price: {
    //           default: null,
    //           dependent: true,
    //           dependsOn: "_setPrice",
    //           resolver: ({ context }: any) => context._setPrice,
    //         },
    //         _setPrice: {
    //           virtual: true,
    //           validator,
    //         },
    //       },
    //       { onDelete: [onDelete_("global"), onDelete_("global-1")] }
    //     ).getModel();

    //     it("should trigger all 'success' listeners at creation", async () => {
    //       const { data, handleSuccess } = await Book.create({
    //         name: "Book name",
    //         _setPrice: 100,
    //       });

    //       await handleSuccess();

    //       const values = { id: 1, name: "Book name", price: 100 };
    //       const summary = {
    //         context: { ...values, _setPrice: 100 },
    //         operation: "creation",
    //         previousValues: undefined,
    //         values: values,
    //       };

    //       expect(data).toEqual(values);
    //       expect(successValues).toEqual({
    //         global: summary,
    //         "global-1": summary,
    //       });
    //     });

    //     it("should trigger all 'success' listeners during cloning ", async () => {
    //       const book = { id: 1, name: "Book name", price: 100 };

    //       const { data, handleSuccess } = await Book.clone({
    //         ...book,
    //         _setPrice: 100,
    //       });

    //       await handleSuccess();

    //       const summary = {
    //         context: { ...book, _setPrice: 100 },
    //         operation: "creation",
    //         previousValues: undefined,
    //         values: book,
    //       };

    //       expect(data).toEqual(book);
    //       expect(successValues).toEqual({
    //         global: summary,
    //         "global-1": summary,
    //       });
    //     });

    //     it("should trigger all 'success' listeners during updates ", async () => {
    //       const book = { id: 1, name: "Book name", price: 100 };

    //       const { data, handleSuccess } = await Book.update(book, {
    //         _setPrice: 200,
    //       });

    //       await handleSuccess();

    //       const values = { ...book, price: 200 };

    //       const summary = {
    //         context: { ...values, _setPrice: 200 },
    //         operation: "update",
    //         previousValues: book,
    //         values: values,
    //       };

    //       expect(data).toEqual({ price: 200 });
    //       expect(successValues).toEqual({
    //         global: summary,
    //         "global-1": summary,
    //       });
    //     });
    //   });
    // });

    describe("valid", () => {
      it("should allow 'onDelete' as (() => any) | ((() => any)[])", () => {
        const values = [() => {}, [() => {}]];

        for (const onDelete of values) {
          const toPass = fx(getValidSchema(), { onDelete });

          expectNoFailure(toPass);

          toPass();
        }
      });
    });

    describe("invalid", () => {
      it("should reject 'onDelete' other than (() => any) | ((() => any)[])", () => {
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

        for (const onDelete of invalidValues) {
          const toFail = fx(getValidSchema(), { onDelete });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: "Invalid Schema",
              payload: {
                onDelete: expect.arrayContaining([
                  "The 'onDelete' handler @[0] is not a function",
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
