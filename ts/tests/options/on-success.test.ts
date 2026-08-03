import { beforeEach, describe, expect, it } from "bun:test";

import { type ReadonlyIvoContext, Schema } from "../../src";
import {
  getInvalidConfigMessageForRepeatedFields,
  getInvalidOnSuccessConfigMessage,
} from "../../src/schema/schema-core";
import {
  ERRORS,
  expectFailure,
  expectNoFailure,
  getValidSchema,
  makeFx,
  validator,
} from "../_utils";

describe("Schema.options.onSuccess", () => {
  describe("signature", () => {
    describe("valid", () => {
      it("should allow valid 'onSuccess' config", () => {
        const values = [
          () => {},
          [() => {}],
          [() => {}, () => {}],
          {
            fields: ["fieldName1", "fieldName2"],
            handler: () => {},
          },
          {
            fields: ["fieldName1", "fieldName2"],
            handler: [() => {}, () => {}],
          },
          {
            fields: [
              "constant",
              "laxField",
              "fieldName2",
              "dependent",
              "virtual",
            ],
            handler: [() => {}, () => {}],
          },
          {
            fields: [
              "constant",
              "laxField",
              "fieldName2",
              "dependent",
              "virtual",
            ],
            handler: () => {},
          },
          [
            () => {},
            {
              fields: ["fieldName1", "constant"],
              handler: [() => {}, () => {}],
            },
            {
              fields: ["laxField", "fieldName2", "dependent", "virtual"],
              handler: () => {},
            },
          ],
          [
            () => {},
            {
              fields: ["fieldName1", "fieldName1", "constant"],
              handler: [() => {}, () => {}],
            },
            {
              fields: ["laxField", "fieldName2", "dependent", "virtual"],
              handler: () => {},
            },
          ],
        ];

        for (const onSuccess of values) {
          const toPass = makeFx(
            (b, m) =>
              b
                .field(m.constant("constant", ""))
                .field(m.lax("fieldName1").default(""))
                .field(m.lax("fieldName2").default(""))
                .field(m.lax("laxField").default(""))
                .field(
                  m
                    .dependent("dependent", ["laxField", "virtual"])
                    .default("")
                    .resolve(() => {}),
                )
                .field(
                  m
                    .required("readonly")
                    .validate(() => false)
                    .readonly(),
                )
                .field(m.virtual("virtual").validate(() => false)),
            {
              onSuccess,
            },
          );

          expectNoFailure(toPass);

          toPass();
        }
      });

      it("should allow 'onSuccess' if a property or virtual is provided in more than 1 config or subsets if the configs don't have the same fields", () => {
        const toPass = makeFx(
          (b, m) =>
            b
              .field(m.constant("constant", ""))
              .field(m.lax("laxField").default(""))
              .field(m.lax("fieldName1").default(""))
              .field(m.lax("fieldName2").default(""))
              .field(
                m
                  .dependent("dependent", ["laxField", "virtual"])
                  .default("")
                  .resolve(() => {}),
              )
              .field(
                m
                  .required("readonly")
                  .validate(() => false)
                  .readonly(),
              )
              .field(m.virtual("virtual").validate(() => false)),
          {
            onSuccess: [
              {
                fields: ["fieldName1", "laxField", "dependent"],
                handler: () => {},
              },
              {
                fields: ["virtual", "laxField"],
                handler: () => {},
              },
              {
                fields: ["dependent", "fieldName1"],
                handler: () => {},
              },
            ],
          },
        );

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe("invalid", () => {
      it("should reject 'onSuccess' if it's not a function, object or array", () => {
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

        invalidValues.forEach((onSuccess) => {
          const toFail = makeFx(getValidSchema(), { onSuccess });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining([
                  getInvalidOnSuccessConfigMessage(),
                ]),
              },
            });
          }
        });
      });

      it("should reject 'onSuccess' if invalid fields or handlers are passed in config object", () => {
        const invalidFielderties = [
          1,
          0,
          -14,
          true,
          false,
          "invalid",
          "",
          null,
          undefined,
          [],
        ];

        invalidFielderties.forEach((fields) => {
          const toFail = makeFx(getValidSchema(), {
            onSuccess: { fields, handler: () => {} },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining([
                  '"fields" must be an array of at least 2 fields or virtuals of your schema',
                ]),
              },
            });
          }
        });

        const invalidHandlers = [
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
        invalidHandlers.forEach((handler) => {
          const toFail = makeFx(getValidSchema(), {
            onSuccess: {
              fields: ["fieldName1", "fieldName2"],
              handler,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining([
                  '"handler" must be a function or array of functions',
                ]),
              },
            });
          }
        });

        const invalidNestedHandlers = [
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
        const schemaWithInvalidHandlers = makeFx(getValidSchema(), {
          onSuccess: {
            fields: ["fieldName1", "fieldName2"],
            handler: invalidNestedHandlers,
          },
        });

        expectFailure(schemaWithInvalidHandlers);

        try {
          schemaWithInvalidHandlers();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              onSuccess: expect.arrayContaining(
                invalidNestedHandlers.map((_, i) =>
                  getInvalidOnSuccessConfigMessage(
                    undefined,
                    "handler-must-be-function",
                    i,
                  ),
                ),
              ),
            },
          });
        }
      });

      it("should reject if any of the fields passed in config object are not valid fields or virtuals", () => {
        const invalidFielderties = [
          1,
          0,
          -14,
          true,
          false,
          "invalid",
          "",
          null,
          undefined,
          [],
        ];

        const schemaWithInvalidFielderties = makeFx(getValidSchema(), {
          onSuccess: { fields: invalidFielderties, handler: () => {} },
        });

        expectFailure(schemaWithInvalidFielderties);

        try {
          schemaWithInvalidFielderties();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              onSuccess: expect.arrayContaining(
                invalidFielderties.map(
                  (field) =>
                    `"${field}" is not a property or virtual on your schema`,
                ),
              ),
            },
          });
        }

        const schemaWithNestedInvalidFielderties = makeFx(getValidSchema(), {
          onSuccess: [{ fields: invalidFielderties, handler: () => {} }],
        });

        expectFailure(schemaWithNestedInvalidFielderties);

        try {
          schemaWithNestedInvalidFielderties();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              onSuccess: expect.arrayContaining(
                invalidFielderties.map(
                  (field) =>
                    `Config at index 0: "${field}" is not a property or virtual on your schema`,
                ),
              ),
            },
          });
        }
      });

      it("should reject if some configs have the same fields in any order", () => {
        const validConfigs = [
          { fields: ["fieldName1", "fieldName2"], handler() {} },
          { fields: ["virtual", "virtual2"], handler() {} },
          { fields: ["fieldName1", "virtual2"], handler() {} },
          { fields: ["virtual", "fieldName2"], handler() {} },
          {
            fields: ["fieldName1", "fieldName2", "virtual"],
            handler() {},
          },
          {
            fields: ["fieldName1", "fieldName2", "virtual", "virtual2"],
            handler() {},
          },
        ];

        const configs = [
          // valid
          ...validConfigs.map((c, i) => [c, i]),

          // invalid because they're repeated
          ...validConfigs.map((c, i) => [c, i]),

          // invalid because they're re-arranged
          [
            {
              fields: ["fieldName2", "fieldName1"],
              handler() {},
            },
            0,
          ],
          [{ fields: ["virtual2", "virtual"], handler() {} }, 1],
          [{ fields: ["virtual2", "fieldName1"], handler() {} }, 2],
          [
            {
              fields: ["fieldName2", "fieldName1", "virtual"],
              handler() {},
            },
            4,
          ],
          [
            {
              fields: ["fieldName1", "virtual", "fieldName2"],
              handler() {},
            },
            4,
          ],
        ] as [any, number][];

        const length = validConfigs.length;

        const reasons = configs
          .slice(length)
          .map((ci, i) =>
            getInvalidConfigMessageForRepeatedFields(i + length, ci[1]),
          );

        const toFail = makeFx(
          (b, m) =>
            b
              .field(
                m
                  .dependent("dependent", ["virtual", "virtual2"])
                  .default("")
                  .resolve(() => {}),
              )
              .field(m.lax("fieldName1").default(""))
              .field(m.lax("fieldName2").default(""))
              .field(m.virtual("virtual").validate(() => false))
              .field(m.virtual("virtual2").validate(() => false)),
          { onSuccess: configs.map((ci) => ci[0]) },
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              onSuccess: expect.arrayContaining(reasons),
            },
          });
        }
      });
    });
  });

  describe("behaviour", () => {
    let successValues: Record<string, unknown> = {};

    type BookInput = { _setPrice?: number; name?: string };
    type BookOutput = { id: number; name: string; price: number | null };

    function onSuccess_(field = "") {
      return (summary: ReadonlyIvoContext<BookInput, BookOutput>) => {
        successValues[field] = summary;
      };
    }

    beforeEach(() => {
      successValues = {};
    });

    describe("behaviour with other success listeners", () => {
      const Book = new Schema<BookInput, BookOutput>(
        (b, m) =>
          b
            .field(m.constant("id", 1).onSuccess(onSuccess_("id")))
            .field(
              m
                .required("name")
                .validate(validator)
                .onSuccess(onSuccess_("name")),
            )
            .field(
              m
                .dependent("price", "_setPrice")
                .default(null)
                .resolve((ctx) => ctx.input._setPrice!)
                .onSuccess(onSuccess_("price")),
            )
            .field(
              m
                .virtual("_setPrice")
                .validate(validator)
                .onSuccess(onSuccess_("_setPrice")),
            ),
        { onSuccess: onSuccess_("global") },
      ).getModel();

      it("should trigger all 'success' listeners at creation", async () => {
        const { data, handleSuccess } = await Book.create(
          {
            name: "Book name",
            _setPrice: 100,
          },
          {},
        );

        await handleSuccess?.();

        const values = { id: 1, name: "Book name", price: 100 };
        const summary = {
          changes: null,
          input: { name: "Book name", _setPrice: 100 },
          isUpdate: false,
          previousValues: null,
          values: values,
        };

        expect(data).toEqual(values);
        expect(successValues).toMatchObject({
          id: summary,
          name: summary,
          price: summary,
          _setPrice: summary,
          global: summary,
        });
      });

      it("should trigger all 'success' listeners during updates ", async () => {
        const book = { id: 1, name: "Book name", price: 100 };

        const { data, handleSuccess } = await Book.update(
          book,
          {
            _setPrice: 200,
          },
          {},
        );

        await handleSuccess?.();

        const values = { ...book, price: 200 };

        const summary = {
          changes: data,
          input: { _setPrice: 200 },
          isUpdate: true,
          previousValues: book,
          values: values,
        };

        expect(data).toEqual({ price: 200 });
        expect(successValues).toMatchObject({
          price: summary,
          _setPrice: summary,
          global: summary,
        });
      });
    });

    describe("behaviour without other success listeners", () => {
      const Book = new Schema<BookInput, BookOutput>(
        (b, m) =>
          b
            .field(m.constant("id", 1))
            .field(m.required("name").validate(validator))
            .field(
              m
                .dependent("price", "_setPrice")
                .default(null)
                .resolve((ctx) => ctx.input._setPrice!),
            )
            .field(m.virtual("_setPrice").validate(validator)),
        { onSuccess: [onSuccess_("global"), onSuccess_("global-1")] },
      ).getModel();

      it("should trigger all 'success' listeners at creation", async () => {
        const { data, handleSuccess } = await Book.create(
          {
            name: "Book name",
            _setPrice: 100,
          },
          {},
        );

        await handleSuccess?.();

        const values = { id: 1, name: "Book name", price: 100 };
        const summary = {
          changes: null,
          input: { name: "Book name", _setPrice: 100 },
          isUpdate: false,
          previousValues: null,
          values: values,
        };

        expect(data).toEqual(values);
        expect(successValues).toMatchObject({
          global: summary,
          "global-1": summary,
        });
      });

      it("should trigger all 'success' listeners during updates ", async () => {
        const book = { id: 1, name: "Book name", price: 100 };

        const { data, handleSuccess } = await Book.update(
          book,
          {
            _setPrice: 200,
          },
          {},
        );

        await handleSuccess?.();

        const values = { ...book, price: 200 };

        const summary = {
          changes: data,
          input: { _setPrice: 200 },
          isUpdate: true,
          previousValues: book,
          values: values,
        };

        expect(data).toEqual({ price: 200 });
        expect(successValues).toMatchObject({
          global: summary,
          "global-1": summary,
        });
      });
    });

    describe("behaviour onSuccess config object", () => {
      let successValuesFromOptions: Record<string, number> = {};

      beforeEach(() => {
        successValuesFromOptions = {};
      });

      function onOptionSuccess(props: string[]) {
        return () => {
          props.forEach((field) => {
            successValuesFromOptions[field] =
              (successValuesFromOptions[field] ?? 0) + 1;
          });
        };
      }

      describe("constant fields", () => {
        const Model = new Schema<any>(
          (b, m) =>
            b
              .field(m.constant("const1", 1))
              .field(m.constant("const2", 2))
              .field(m.lax("lax").default(true)),
          {
            onSuccess: {
              fields: ["const1", "const2"],
              // @ts-expect-error failed to properly infer
              handler: onOptionSuccess(["const1", "const2"]),
            },
          },
        ).getModel();

        it("should trigger all 'success' listeners of constant props at creation", async () => {
          const { data, handleSuccess } = await Model.create({}, {});

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const1: 1,
            const2: 1,
          });
        });

        it("should not trigger 'success' listeners of constant props during updates", async () => {
          const initialData = { const1: 400, const2: 400, lax: 100 };

          // @ts-expect-error ikr
          const { data, handleSuccess } = await Model.update(initialData, {
            const1: 200,
            const2: 200,
            lax: 200,
          });

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({});
        });
      });

      describe("non-constant fields", () => {
        const Model = new Schema<any>(
          (b, m) =>
            b
              .field(m.constant("const", 1))
              .field(m.lax("lax").default(true))
              .field(m.lax("lax2").default(true))
              .field(m.required("required").validate(validator))
              .field(m.required("required2").validate(validator))
              .field(
                m
                  .dependent("dependent", ["lax2", "virtual1", "virtual2"])
                  .default(null)
                  .resolve(validator as never)
                  .onSuccess(onSuccess_("dependent")),
              )
              .field(m.virtual("virtual1").validate(validator))
              .field(m.virtual("virtual2").validate(validator)),
          {
            onSuccess: [
              onOptionSuccess(["dependent"]),
              {
                fields: ["lax", "lax2"],
                // @ts-expect-error failed to properly infer
                handler: [
                  onOptionSuccess(["lax", "lax2"]),
                  onOptionSuccess(["lax2"]),
                ],
              },
              {
                fields: ["virtual1", "virtual2"],
                // @ts-expect-error failed to properly infer
                handler: onOptionSuccess(["virtual1", "virtual2"]),
              },
              {
                fields: ["required", "const"],
                // @ts-expect-error failed to properly infer
                handler: onOptionSuccess(["required", "const"]),
              },
              {
                fields: ["required2", "dependent"],
                // @ts-expect-error failed to properly infer
                handler: onOptionSuccess(["required2", "dependent"]),
              },
            ],
          },
        ).getModel();

        it("should trigger all related 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Model.create(
            {
              required: 100,
              required2: 100,
            },
            {},
          );

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const: 1,
            dependent: 2,
            lax: 1,
            lax2: 2,
            required: 1,
            required2: 1,
          });
        });

        it("should trigger 'success' listeners of virtual at creation if they are provided", async () => {
          const { data, handleSuccess } = await Model.create(
            {
              required: 100,
              required2: 100,
              virtual1: 4,
            },
            {},
          );

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const: 1,
            dependent: 2,
            lax: 1,
            lax2: 2,
            required: 1,
            required2: 1,
            virtual1: 1,
            virtual2: 1,
          });
        });

        it("should trigger 'success' listeners of props provided during updates", async () => {
          const initialData = {
            const: 1,
            dependent: 2,
            lax: 1,
            lax2: 1,
            required: 1,
            required2: 1,
          };

          // @ts-expect-error ikr
          const { data, handleSuccess } = await Model.update(initialData, {
            const1: 200,
            const2: 200,
            required: 200,
          });

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const: 1,
            dependent: 1,
            required: 1,
          });

          successValuesFromOptions = {};

          {
            // @ts-expect-error ikr
            const { data, handleSuccess } = await Model.update(initialData, {
              virtual1: 200,
            });

            await handleSuccess?.();

            expect(data).not.toBeNull();
            expect(successValuesFromOptions).toEqual({
              dependent: 2,
              required2: 1,
              virtual1: 1,
              virtual2: 1,
            });
          }
        });
      });
    });
  });
});
