import { beforeEach, describe, expect, it } from "bun:test";

import { type ReadonlyIvoContext, Schema } from "../../src";
import { expectFailure, expectNoFailure, makeFx, validator } from "../_utils";
import { IvoSuccessContext } from "../../src/utils/types";

describe("virtual", () => {
  describe("valid", () => {
    describe("alias", () => {
      it("should allow alias", () => {
        const toPass = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", "fieldName")
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual("fieldName")
                .alias("alias")
                .validate(validator)
                .sanitize(() => ""),
            ),
        );

        expectNoFailure(toPass);

        toPass();
      });

      it("should allow alias if it is the same as a related dependency of the virtual", () => {
        const dependentField = "dependentField";
        const virtualField = "virtualField";

        const toPass = makeFx((b, m) =>
          b
            .field(
              m
                .dependent(dependentField, virtualField)
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual(virtualField)
                .alias(dependentField)
                .validate(validator)
                .sanitize(() => ""),
            ),
        );

        expectNoFailure(toPass);

        toPass();
      });

      describe("behaviour", () => {
        let contextRecord = {} as Record<string, number | undefined>;

        type QuantityInput = { qty?: number };
        type QuantityOutput = { id: number; quantity: number };

        function resolver({
          input: { qty },
        }: ReadonlyIvoContext<QuantityInput, QuantityOutput>) {
          if (qty !== undefined) contextRecord.qty = qty;

          return qty;
        }

        function validator(v: any) {
          const _type = typeof v;
          return _type === "number"
            ? { valid: true, validated: v }
            : { valid: false, reason: "Invalid quantity" };
        }

        const Model = new Schema<any>((b, m) =>
          b
            .field(m.constant("id", 1).onDelete(resolver as never))
            .field(
              m
                .dependent("quantity", "setQuantity")
                .default(0.0)
                .resolve(resolver as never),
            )
            .field(m.virtual("setQuantity").alias("qty").validate(validator)),
        ).getModel();

        beforeEach(() => {
          contextRecord = {};
        });

        describe("creation", () => {
          it("should respect alias if provided at creation", async () => {
            const qty = 12;
            const { data } = await Model.create({ qty }, {});

            expect(data).toMatchObject({ id: 1, quantity: qty });
            expect(contextRecord).toEqual({ qty });
          });

          it("should use default values of dependent props to be set if an alias with that field's name exists on the same schema but initialization is blocked", async () => {
            const Model = new Schema<any>((b, m) =>
              b
                .field(m.constant("id", 1).onDelete(resolver as never))
                .field(
                  m
                    .dependent("quantity", "setQuantity")
                    .default(0.0)
                    .resolve(resolver as never),
                )
                .field(
                  m
                    .virtual("setQuantity")
                    .alias("quantity")
                    .validate(validator)
                    .ignoreInit(),
                ),
            ).getModel();

            const { data } = await Model.create({ quantity: 12 }, {});

            expect(data).toMatchObject({ id: 1, quantity: 0 });
            expect(contextRecord).toEqual({});
          });

          it("should return alias errors with alias name in error payload at creation", async () => {
            const { error } = await Model.create({ qty: "12" }, {});

            expect(error).toMatchObject({
              qty: {
                reason: "Invalid quantity",
                metadata: null,
              },
            });
            expect(contextRecord).toEqual({});
          });
        });

        describe("delete", () => {
          it("aliases should not be available in context during deletion", async () => {
            await Model.delete({ id: 1, quantity: 12, qty: 1000 }, {});

            expect(contextRecord).toEqual({});
          });
        });

        describe("update", () => {
          it("should respect alias if provided during updates", async () => {
            const qty = 5;
            const { data } = await Model.update(
              { id: 1, quantity: 12 },
              { qty },
              {},
            );

            expect(data).toMatchObject({ quantity: qty });
            expect(contextRecord).toEqual({ qty });
          });

          it("should return alias errors with alias name in error payload during updates", async () => {
            const { error } = await Model.update(
              { id: 1, quantity: 12 },
              { qty: "2" },
              {},
            );

            expect(error).toMatchObject({
              qty: { reason: "Invalid quantity", metadata: null },
            });
            expect(contextRecord).toEqual({});
          });
        });

        describe("availability of virtuals in ctx of 'required' method of virtual", () => {
          const Model = new Schema<any>((b, m) =>
            b
              .field(m.constant("id", 1))
              .field(m.lax("note", ""))
              .field(
                m
                  .dependent("quantity", "setQuantity")
                  .default(0.0)
                  .resolve(resolver as never),
              )
              .field(
                m
                  .virtual("setQuantity")
                  .alias("qty")
                  .validate(validator)
                  .required(({ input: { setQuantity } }: any) => {
                    contextRecord.setQuantity = setQuantity;

                    return true;
                  }),
              ),
          ).getModel();

          it("should make ctx.input available (keyed by the virtual's config name, not its alias) inside 'required' at creation", async () => {
            const operation = await Model.create({ id: 1 }, {});

            expect(contextRecord).toEqual({ setQuantity: undefined });
            expect(operation.data).toBe(null);
            expect(operation.error).toEqual({
              qty: { reason: "'qty' is required", metadata: null },
            });
          });

          it("should make ctx.input available (keyed by the virtual's config name, not its alias) inside 'required' during updates", async () => {
            const entity = { id: 1, note: "", quantity: 100 };
            // a genuine, unrelated change so the update isn't a no-op —
            // `qty` itself stays unprovided, so `required` still fires for it.
            const operation = await Model.update(entity, { note: "hey" }, {});

            expect(contextRecord).toEqual({ setQuantity: undefined });
            expect(operation.data).toBe(null);
            expect(operation.error?.payload).toEqual({
              qty: { reason: "'qty' is required", metadata: null },
            });
          });
        });

        describe("availability of virtuals in ctx of ignoreInit & ignoreUpdate methods of the virtual when it's alias is provided", () => {
          const Model = new Schema<any>((b, m) =>
            b
              .field(m.constant("id", 1).onDelete(resolver as never))
              .field(
                m
                  .dependent("quantity", "setQuantity")
                  .default(0.0)
                  .resolve(resolver as never),
              )
              .field(
                m
                  .virtual("setQuantity")
                  .alias("qty")
                  .validate(validator)
                  .ignore(({ input: { qty } }) => {
                    contextRecord.setQuantity = qty;

                    return (qty ?? 0) <= 0;
                  }),
                // .ignoreUpdate((input: any, previousValues: any) => {
                //   const qty = input.qty;
                //   const quantity = previousValues.quantity;
                //   contextRecord.setQuantity = qty;

                //   return (qty ?? 0) <= quantity;
                // }),
              ),
          ).getModel();

          it("should respect 'ignoreInit' rule of virtual property even when alias is provided at creation", async () => {
            let qty = -75;
            const operation1 = await Model.create({ id: 1, qty }, {});

            expect(contextRecord).toEqual({ setQuantity: -75 });
            expect(operation1.error).toBe(null);
            expect(operation1.data).toEqual({ id: 1, quantity: 0 });

            qty = 75;

            const operation2 = await Model.create({ id: 1, qty }, {});

            expect(contextRecord).toEqual({ qty, setQuantity: qty });
            expect(operation2.error).toBe(null);
            expect(operation2.data).toEqual({ id: 1, quantity: qty });
          });

          it("should respect 'ignoreUpdate' rule of virtual property even when alias is provided during updates", async () => {
            let qty = 12;
            const operation1 = await Model.update(
              { id: 1, quantity: 75 },
              { qty },
              {},
            );

            expect(contextRecord).toEqual({ setQuantity: qty });
            expect(operation1.error).toBeNull();
            expect(operation1.data).toBe(null);

            qty = 100;

            const operation2 = await Model.update(
              { id: 1, quantity: 75 },
              { qty },
              {},
            );

            expect(contextRecord).toEqual({ qty, setQuantity: qty });
            expect(operation2.error).toBe(null);
            expect(operation2.data).toMatchObject({ quantity: qty });
          });
        });
      });

      describe("behaviour with validation & required errors and alias with different name", () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .dependent("dependent", "_virtual")
                .default(0.0)
                .resolve(() => 1),
            )
            .field(
              m
                .virtual("_virtual")
                .alias("virtual")
                .validate((v: any) => v === "valid")
                .required(() => true),
            ),
        ).getModel();

        describe("creation", () => {
          it("should return alias name as error key if provided and validation fails at creation", async () => {
            const { error } = await Model.create({ virtual: "5" }, {});

            expect(error).toMatchObject({
              virtual: {
                reason: "validation failed",
                metadata: null,
              },
            });
            expect(error?._virtual).toBeUndefined();
          });

          it("should return alias name as error key in case of required error at creation", async () => {
            const { error } = await Model.create({}, {});

            expect(error).toMatchObject({
              virtual: {
                reason: "'virtual' is required",
                metadata: null,
              },
            });
            expect(error?._virtual).toBeUndefined();
          });
        });

        describe("updates", () => {
          const validData = { dependent: 20 };

          it("should return alias name as error key if provided and validation fails during updates", async () => {
            const { error } = await Model.update(
              validData,
              { virtual: "5" },
              {},
            );

            expect(error?.payload).toMatchObject({
              virtual: {
                reason: "validation failed",
                metadata: null,
              },
            });
            expect(error?.payload?._virtual).toBeUndefined();
          });
        });
      });

      describe("behaviour with validation & required errors and alias with name of dependent field", () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .dependent("dependent", "_virtual")
                .default(0.0)
                .resolve(() => 1),
            )
            .field(
              m
                .virtual("_virtual")
                .alias("dependent")
                .validate((v: any) => v === "valid")
                .required(() => true),
            ),
        ).getModel();

        describe("creation", () => {
          it("should return alias name as error key if provided and validation fails at creation", async () => {
            const { error } = await Model.create({ dependent: "5" }, {});

            expect(error).toMatchObject({
              dependent: {
                reason: "validation failed",
                metadata: null,
              },
            });
            expect(error?._virtual).toBeUndefined();
          });

          it("should return alias name as error key in case of required error at creation", async () => {
            const { error } = await Model.create({}, {});

            expect(error).toMatchObject({
              dependent: {
                reason: "'dependent' is required",
                metadata: null,
              },
            });
            expect(error?._virtual).toBeUndefined();
          });
        });

        describe("updates", () => {
          const validData = { dependent: 20 };

          it("should return alias name as error key if provided and validation fails during updates", async () => {
            const { error } = await Model.update(
              validData,
              { dependent: "5" },
              {},
            );

            expect(error).toMatchObject({
              dependent: { reason: "validation failed", metadata: null },
            });
            expect(error?.payload?._virtual).toBeUndefined();
          });
        });
      });
    });

    it("should allow sanitizer", () => {
      const toPass = makeFx((b, m) =>
        b
          .field(
            m
              .dependent("dependentField", "fieldName")
              .default("")
              .resolve(() => ""),
          )
          .field(
            m
              .virtual("fieldName")
              .validate(validator)
              .sanitize(() => ""),
          ),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should allow onFailure", () => {
      const toPass = makeFx((b, m) =>
        b
          .field(
            m
              .dependent("dependentField", "fieldName")
              .default("")
              .resolve(() => ""),
          )
          .field(
            m
              .virtual("fieldName")
              .validate(validator)
              .onFailure(validator as never),
          ),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should allow requiredBy", () => {
      const toPass = makeFx((b, m) =>
        b
          .field(
            m
              .dependent("dependentField", "fieldName")
              .default("")
              .resolve(() => ""),
          )
          .field(
            m
              .virtual("fieldName")
              .validate(validator)
              .required(() => true),
          ),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should allow onSuccess + validator", () => {
      const values = [[], () => ({})];

      for (const onSuccess of values) {
        const toPass = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", "fieldName")
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual("fieldName")
                .validate(validator)
                .onSuccess(onSuccess as never),
            ),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    describe("behaviour", () => {
      const onSuccessValues: Record<string, unknown> = {};
      const onSuccessStats: Record<string, number> = {};
      const sanitizedValues: Record<string, unknown> = {};

      const User = new Schema<any>((b, m) =>
        b
          .field(
            m
              .dependent("dependentSideInit", [
                "virtualInit",
                "virtualWithSanitizer",
              ])
              .default("")
              .resolve(
                ({ input: { virtualInit, virtualWithSanitizer } }: any) =>
                  virtualInit && virtualWithSanitizer ? "both" : "one",
              )
              // @ts-expect-error ikr
              .onSuccess(onSuccess("dependentSideInit")),
          )
          .field(
            m
              .dependent("dependentSideNoInit", [
                "virtualNoInit",
                "virtualWithSanitizerNoInit",
              ])
              .default("")
              .resolve(() => "changed")
              // @ts-expect-error ikr
              .onSuccess(onSuccess("dependentSideNoInit")),
          )
          .field(m.lax("name", ""))
          .field(
            m
              .virtual("virtualInit")
              .validate(validateBoolean)
              // @ts-expect-error ikr
              .onSuccess(onSuccess("virtualInit")),
          )
          .field(
            m
              .virtual("virtualNoInit")
              .validate(validateBoolean as never)
              .ignoreInit()
              .onSuccess([
                // @ts-expect-error ikr
                onSuccess("virtualNoInit"),
                incrementOnSuccessStats("virtualNoInit"),
              ]),
          )
          .field(
            m
              .virtual("virtualWithSanitizer")
              .validate(validateBoolean as never)
              .sanitize(sanitizerOf("virtualWithSanitizer", "sanitized"))
              .onSuccess([
                // @ts-expect-error ikr
                onSuccess("virtualWithSanitizer"),
                incrementOnSuccessStats("virtualWithSanitizer"),
                incrementOnSuccessStats("virtualWithSanitizer"),
              ]),
          )
          .field(
            m
              .virtual("virtualWithSanitizerNoInit")
              .validate(validateBoolean as never)
              .ignoreInit()
              .sanitize(
                sanitizerOf("virtualWithSanitizerNoInit", "sanitized no init"),
              )
              .onSuccess([
                // @ts-expect-error ikr
                onSuccess("virtualWithSanitizerNoInit"),
                incrementOnSuccessStats("virtualWithSanitizerNoInit"),
              ]),
          ),
      ).getModel();

      function sanitizerOf(field: string, value: any) {
        return () => {
          sanitizedValues[field] = value;

          return value;
        };
      }

      function incrementOnSuccessStats(field: string) {
        return () => {
          onSuccessStats[field] = (onSuccessStats[field] ?? 0) + 1;
        };
      }

      type UserInput = {
        virtualInit?: boolean;
        virtualNoInit?: boolean;
        virtualWithSanitizer?: boolean;
        virtualWithSanitizerNoInit?: boolean;
      };
      type UserOutput = {
        name: string;
        dependentSideInit: string;
        dependentSideNoInit: string;
      };

      function onSuccess(field: string) {
        return (context: IvoSuccessContext<UserInput, UserOutput>) => {
          onSuccessValues[field] =
            (context.values as Record<string, unknown>)?.[field] ??
            (context.input as Record<string, unknown>)?.[field];
          incrementOnSuccessStats(field)();
        };
      }

      function validateBoolean(value: any) {
        if ([false, true].includes(value)) return true;

        return { valid: false, reason: `${value} is not a boolean` };
      }

      beforeEach(() => {
        for (const key of Object.keys(onSuccessStats))
          delete onSuccessStats[key];
        for (const key of Object.keys(onSuccessValues))
          delete onSuccessValues[key];
        for (const key of Object.keys(sanitizedValues))
          delete sanitizedValues[key];
      });

      describe("creation", () => {
        it("should not sanitize virtuals nor resolve their dependencies if not provided", async () => {
          const { data } = await User.create({ name: "Peter" }, {});

          expect(data).toEqual({
            dependentSideInit: "",
            dependentSideNoInit: "",
            name: "Peter",
          });

          expect(sanitizedValues).toEqual({});
        });

        it("should respect sanitizer at creation", async () => {
          const { data } = await User.create(
            {
              name: "Peter",
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true,
            },
            {},
          );

          expect(data).toEqual({
            dependentSideInit: "one",
            dependentSideNoInit: "",
            name: "Peter",
          });

          expect(sanitizedValues).toEqual({
            virtualWithSanitizer: "sanitized",
          });
        });

        it("should respect virtualInits & virtualNoInit at creation", async () => {
          const { data: user, handleSuccess } = await User.create(
            {
              dependentSideNoInit: "",
              dependentSideInit: true,
              name: "Peter",
              virtualInit: true,
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true,
            },
            {},
          );

          await handleSuccess?.();

          expect(user).toEqual({
            dependentSideInit: "both",
            dependentSideNoInit: "",
            name: "Peter",
          });

          expect(onSuccessStats).toEqual({
            dependentSideInit: 1,
            dependentSideNoInit: 1,
            virtualInit: 1,
            virtualWithSanitizer: 3,
          });

          expect(onSuccessValues).toEqual({
            dependentSideInit: "both",
            dependentSideNoInit: "",
            virtualInit: true,
            virtualWithSanitizer: "sanitized",
          });

          expect(sanitizedValues).toEqual({
            virtualWithSanitizer: "sanitized",
          });
        });
      });

      describe("updating", () => {
        it("should respect sanitizer of all virtuals provided during updates", async () => {
          const { data, handleSuccess } = await User.update(
            { name: "Peter" },
            {
              name: "John",
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true,
            },
            {},
          );

          await handleSuccess?.();

          expect(data).toEqual({
            name: "John",
            dependentSideInit: "one",
            dependentSideNoInit: "changed",
          });

          expect(onSuccessStats).toEqual({
            dependentSideInit: 1,
            dependentSideNoInit: 1,
            virtualWithSanitizer: 3,
            virtualWithSanitizerNoInit: 2,
          });

          expect(onSuccessValues).toEqual({
            dependentSideInit: "one",
            dependentSideNoInit: "changed",
            virtualWithSanitizer: "sanitized",
            virtualWithSanitizerNoInit: "sanitized no init",
          });

          expect(sanitizedValues).toEqual({
            virtualWithSanitizer: "sanitized",
            virtualWithSanitizerNoInit: "sanitized no init",
          });
        });
      });

      describe("behaviour with errors thrown in the sanitizer", () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .dependent("dependent", "virtual")
                .default("")
                .resolve(
                  (context: any) =>
                    context.input?.virtual ?? context.rawInput?.virtual,
                ),
            )
            .field(
              m
                .virtual("virtual")
                .validate(() => true)
                .sanitize(() => {
                  throw new Error("lolol");
                }),
            ),
        ).getModel();

        const values = [null, "", 1, 0, -1, true, false, [], {}];

        it("should use the validated value at creation", async () => {
          for (const virtual of values) {
            const { data, error } = await Model.create({ virtual }, {});

            expect(error).toBeNull();
            expect(data).toMatchObject({ dependent: virtual });
          }
        });

        it("should use the validated value during updates", async () => {
          for (const virtual of values) {
            const { data, error } = await Model.update(
              { dependent: "lolol" },
              { virtual },
              {},
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ dependent: virtual });
          }
        });
      });
    });
  });

  describe("invalid", () => {
    // "should reject alias if definition does not have the virtual
    // keyword" discarded: `.alias()` only exists on `VirtualBuilder` -
    // there's no way to call it on a required/readonly/lax/dependent
    // builder chain at all, so a non-virtualm having an alias is
    // structurally unrepresentable.
    describe("alias", () => {
      it("should reject alias if non-empty string is provided", () => {
        const values = [-1, 1, true, false, undefined, "", null, [], {}];

        for (const alias of values) {
          const toFail = makeFx((b, m) =>
            b
              .field(
                m
                  .dependent("dependentField", "fieldName")
                  .default("")
                  .resolve(() => ""),
              )
              .field(
                m
                  .virtual("fieldName")
                  .alias(alias as never)
                  .validate(validator),
              ),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                fieldName: expect.arrayContaining([
                  "An alias must be a string with at least 1 character",
                ]),
              }),
            );
          }
        }
      });

      it("should reject alias if it's same as the virtual property", () => {
        const virtualField = "virtualField";

        const toFail = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", virtualField)
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual(virtualField)
                .alias(virtualField as never)
                .validate(validator),
            ),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              [virtualField]: expect.arrayContaining([
                "An alias cannot be the same as the virtual property",
              ]),
            }),
          );
        }
      });

      it("should reject alias if already used by another virtual", () => {
        const alias = "alias";
        const virtualField = "virtualField";

        const toFail = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", [virtualField, "virtualField1"])
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual(virtualField)
                .alias(alias as never)
                .validate(validator),
            )
            .field(
              m
                .virtual("virtualField1")
                .alias(alias as never)
                .validate(validator),
            ),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              virtualField1: expect.arrayContaining([
                `Sorry, alias provided '${alias}' already belongs to property '${virtualField}'`,
              ]),
            }),
          );
        }
      });

      it("should reject alias if it is the same as the name of existing virtual", () => {
        const alias = "virtualField1";
        const virtualField = "virtualField";

        const toFail = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", [virtualField, "virtualField1"])
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual(virtualField)
                .alias(alias as never)
                .validate(validator),
            )
            .field(m.virtual("virtualField1").validate(validator)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              [virtualField]: expect.arrayContaining([
                `'${alias}' cannot be used as the alias of '${virtualField}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
              ]),
            }),
          );
        }
      });

      it("should reject alias if it is the same as the name of existing property", () => {
        const laxField = "laxField";
        const virtualField = "virtualField";

        const toFail = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", virtualField)
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual(virtualField)
                .alias(laxField as never)
                .validate(validator),
            )
            .field(m.lax(laxField, true)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              [virtualField]: expect.arrayContaining([
                `'${laxField}' cannot be used as the alias of '${virtualField}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
              ]),
            }),
          );
        }
      });

      it("should reject alias if it is the same as an unrelated dependent property", () => {
        const dependentField = "dependentField";
        const virtualField = "virtualField";

        const toFail = makeFx((b, m) =>
          b
            .field(
              m
                .dependent(dependentField, virtualField)
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual(virtualField)
                .alias("dependentField1" as never)
                .validate(validator),
            )
            .field(
              m
                .dependent("dependentField1", "virtualField1")
                .default("")
                .resolve(() => ""),
            )
            .field(m.virtual("virtualField1").validate(validator)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              [virtualField]: expect.arrayContaining([
                `'dependentField1' cannot be used as the alias of '${virtualField}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
              ]),
            }),
          );
        }
      });
    });

    describe("sanitizers", () => {
      it("should reject invalid sanitizer", () => {
        const values = [-1, 1, true, false, undefined, null, [], {}];

        for (const sanitizer of values) {
          const toFail = makeFx((b, m) =>
            b
              .field(
                m
                  .dependent("dependentField", "fieldName")
                  .default("")
                  .resolve(() => ""),
              )
              .field(
                m
                  .virtual("fieldName")
                  .validate(validator)
                  .sanitize(sanitizer as never),
              ),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                fieldName: expect.arrayContaining([
                  "'sanitizer' must be a function",
                ]),
              }),
            );
          }
        }
      });
    });

    it("should reject virtual & no dependent property ", () => {
      const toFail = makeFx((b, m) =>
        b.field(m.virtual("fieldName").validate(validator)),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            fieldName: [
              "A virtual property must have at least one property that depends on it",
            ],
          }),
        );
      }
    });
  });
});
