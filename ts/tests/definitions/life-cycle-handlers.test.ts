import { beforeEach, describe, expect, it, test } from "bun:test";
import { type ReadonlyIvoContext, Schema } from "../../src";
import { expectFailure, expectNoFailure, makeFx, validator } from "../_utils";
import { IvoSuccessContext } from "../../src/utils/types";

describe("life cycle handlers", () => {
  const rules = ["onDelete", "onFailure", "onSuccess"] as const;

  describe("valid", () => {
    test("valid", () => {
      const values = [() => {}, () => ({}), [() => {}], [() => {}, () => ({})]];

      for (const rule of rules)
        for (const value of values) {
          const toPass = makeFx((b, m) =>
            b.field(
              m
                .lax("fieldName", "")
                .validate(validator)
                [rule](value as never),
            ),
          );

          expectNoFailure(toPass);

          toPass();
        }
    });
  });

  describe("invalid", () => {
    test("invalid", () => {
      const values = [1, "", 0, false, true, null, {}];

      for (const rule of rules)
        for (const value of values) {
          const toFail = makeFx((b, m) =>
            b.field(
              m
                .lax("fieldName", "")
                .validate(validator)
                [rule](value as never),
            ),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                fieldName: expect.arrayContaining([
                  `The '${rule}' handler at index: 0 is not a function`,
                ]),
              }),
            );
          }
        }
    });
  });

  describe("life cycle readonly ctx", () => {
    const rules = ["onDelete", "onFailure", "onSuccess"];

    let propChangeMap: Record<string, Record<string, boolean>> = {},
      ctxHasUpdateMethod: Record<string, boolean> = {};

    const validData = { constant: 1, prop1: "1", prop2: "2", prop3: "3" };
    const allFields = ["constant", "prop1", "prop2", "prop3"],
      props = ["prop1", "prop2", "prop3"];

    const handle =
      (rule = "", field = "") =>
      (context: IvoSuccessContext<any>) => {
        ctxHasUpdateMethod[rule] = !!(context as any)?.updateOptions;

        try {
          (context as any)[field] = 1;
        } catch {
          if (!propChangeMap[rule]) propChangeMap[rule] = {};

          propChangeMap[rule][field] = true;
        }
      };
    const validator = (value: unknown) => ({ valid: !!value });

    const Model = new Schema<any>((b, m) =>
      b
        .field(
          m
            .constant("constant", "constant")
            .onDelete(handle("onDelete", "constant"))
            // @ts-expect-error ikr
            .onSuccess(handle("onSuccess", "constant")),
        )
        .field(
          m
            .required("prop1")
            .validate(validator)
            .onDelete(handle("onDelete", "prop1"))
            .onFailure(handle("onFailure", "prop1"))
            .onSuccess(handle("onSuccess", "prop1")),
        )
        .field(
          m
            .required("prop2")
            .validate(validator)
            .onDelete(handle("onDelete", "prop2"))
            .onFailure(handle("onFailure", "prop2"))
            .onSuccess(handle("onSuccess", "prop2")),
        )
        .field(
          m
            .required("prop3")
            .validate(validator)
            .onDelete(handle("onDelete", "prop3"))
            .onFailure(handle("onFailure", "prop3"))
            .onSuccess(handle("onSuccess", "prop3")),
        ),
    ).getModel();

    beforeEach(() => {
      propChangeMap = {};
      ctxHasUpdateMethod = {};
    });

    it("should reject handlers that try to mutate the onSuccess ctx", async () => {
      const { handleFailure, handleSuccess } = await Model.create(
        validData,
        {},
      );

      expect(handleFailure).toBeNull();

      await handleSuccess?.();

      expect(propChangeMap.onSuccess.constant).toBe(true);
      expect(ctxHasUpdateMethod).toEqual({ onSuccess: false });
    });

    it("should reject handlers that try to mutate the onDelete ctx", async () => {
      await Model.delete(validData, {});

      for (const field of allFields)
        expect(propChangeMap.onDelete[field]).toBe(true);

      expect(ctxHasUpdateMethod).toEqual({ onDelete: false });
    });

    it("should reject handlers that try to mutate the onFailure(create) ctx", async () => {
      const { handleFailure } = await Model.create(
        { prop1: "", prop2: "", prop3: "" },
        {},
      );

      await handleFailure?.();

      for (const field of props)
        for (const rule of rules) {
          const result = rule === "onFailure" ? true : undefined;

          // @ts-expect-error we are testing that the context is readonly
          expect(propChangeMap?.[rule]?.[field]).toBe(result);
        }

      expect(ctxHasUpdateMethod).toEqual({ onFailure: false });
    });

    it("should reject handlers that try to mutate the onFailure(update) ctx", async () => {
      const { handleFailure } = await Model.update(
        validData,
        { prop1: "", prop2: "", prop3: "" },
        {},
      );

      await handleFailure?.();

      for (const field of props)
        for (const rule of rules) {
          const result = rule === "onFailure" ? true : undefined;

          // @ts-expect-error we are testing that the context is readonly
          expect(propChangeMap?.[rule]?.[field]).toBe(result);
        }

      expect(ctxHasUpdateMethod).toEqual({ onFailure: false });
    });
  });

  describe("onDelete", () => {
    const contextOptions = { lang: "en" };

    let cxtOptions: Record<string, unknown> = {},
      propChangeMap: Record<string, boolean> = {};

    const onDelete = (field = "") => {
      return (_: unknown, options: Record<string, unknown>) => {
        cxtOptions[field] = options;
        propChangeMap[field] = true;
      };
    };
    const validator = () => ({ valid: false });

    const Model = new Schema<any>((b, m) =>
      b
        .field(
          m.constant("constant", "constant").onDelete(onDelete("constant")),
        )
        .field(
          m.required("prop1").validate(validator).onDelete(onDelete("prop1")),
        )
        .field(
          m.required("prop2").validate(validator).onDelete(onDelete("prop2")),
        )
        .field(
          m.required("prop3").validate(validator).onDelete(onDelete("prop3")),
        ),
    ).getModel();

    beforeEach(() => {
      cxtOptions = {};
      propChangeMap = {};
    });

    it("should trigger all onDelete handlers but for virtuals", async () => {
      await Model.delete(
        { constant: true, prop1: true, prop2: true, prop3: true, prop4: true },
        contextOptions,
      );

      expect(cxtOptions).toEqual({
        constant: contextOptions,
        prop1: contextOptions,
        prop2: contextOptions,
        prop3: contextOptions,
      });
      expect(propChangeMap).toEqual({
        constant: true,
        prop1: true,
        prop2: true,
        prop3: true,
      });
    });

    it("should not trigger any handlers if values are invalid", async () => {
      const invalidData = [1, -10, 0, false, true, "", "true", null];

      for (const val of invalidData) {
        await Model.delete(val, contextOptions);

        expect(cxtOptions).toEqual({});
        expect(propChangeMap).toEqual({});
      }
    });
  });

  describe("onFailure", () => {
    it("should reject onFailure & no validator", () => {
      const toFail = makeFx((b, m) =>
        b.field(m.lax("field", "").onFailure(() => {})),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject(
          expect.objectContaining({
            field: expect.arrayContaining([
              "'onFailure' can only be used with properties that support and have validators",
            ]),
          }),
        );
      }
    });

    describe("behaviour", () => {
      const contextOptions = { lang: "en" };

      let cxtOptions: Record<string, unknown> = {},
        onFailureCount: Record<string, number> = {};

      function incrementOnFailureCountOf(field: string) {
        return ({ options }: ReadonlyIvoContext<any>) => {
          cxtOptions[field] = options;
          onFailureCount[field] = (onFailureCount[field] ?? 0) + 1;
        };
      }
      const validator = () => ({ valid: false });

      const Model = new Schema<any>((b, m) =>
        b
          .field(
            m
              .lax("prop1", true)
              .validate(validator)
              .onFailure(incrementOnFailureCountOf("prop1")),
          )
          .field(
            m
              .required("prop2")
              .validate(validator)
              .onFailure([
                incrementOnFailureCountOf("prop2"),
                incrementOnFailureCountOf("prop2"),
              ]),
          )
          .field(
            m
              .dependent("dependentField", "virtualField")
              .default("")
              .resolve(() => ""),
          )
          .field(
            m
              .virtual("virtualField")
              .validate(validator)
              .onFailure([
                incrementOnFailureCountOf("virtualField"),
                incrementOnFailureCountOf("virtualField"),
                incrementOnFailureCountOf("virtualField"),
              ]),
          ),
      ).getModel();

      beforeEach(() => {
        cxtOptions = {};
        onFailureCount = {};
      });

      describe("creation", () => {
        it("should properly trigger onFailure handlers at creation", async () => {
          const { error, handleFailure } = await Model.create(
            { prop1: false },
            contextOptions,
          );

          await handleFailure?.();

          expect(error).toBeDefined();
          expect(cxtOptions).toEqual({
            prop1: contextOptions,
          });
          expect(onFailureCount).toEqual({ prop1: 1 });
        });

        it("should properly trigger onFailure handlers at creation with virtuals", async () => {
          const { error, handleFailure } = await Model.create(
            {
              prop1: false,
              virtualField: "Yes",
            },
            contextOptions,
          );

          await handleFailure?.();

          expect(error).toBeDefined();
          expect(error).toBeDefined();
          expect(cxtOptions).toEqual({
            prop1: contextOptions,
            virtualField: contextOptions,
          });
          expect(onFailureCount).toEqual({
            prop1: 1,
            virtualField: 3,
          });
        });
      });

      describe("updates", () => {
        it("should properly trigger onFailure handlers during updates", async () => {
          const { error, handleFailure } = await Model.update(
            {},
            { prop1: "" },
            contextOptions,
          );

          await handleFailure?.();

          expect(error).toBeDefined();
          expect(cxtOptions).toEqual({ prop1: contextOptions });
          expect(onFailureCount).toEqual({ prop1: 1 });
        });

        it("should properly trigger onFailure handlers during updates with virtuals", async () => {
          const data = [
            [
              { virtualField: "" },
              { virtualField: 3 },
              { virtualField: contextOptions },
            ],
            [
              { prop1: "", virtualField: "" },
              { prop1: 1, virtualField: 3 },
              { prop1: contextOptions, virtualField: contextOptions },
            ],
          ];

          for (const [changes, results, ctxOpts] of data) {
            onFailureCount = {};

            const { error, handleFailure } = await Model.update(
              {},
              changes,
              contextOptions,
            );

            await handleFailure?.();

            expect(error).toBeDefined();
            expect(cxtOptions).toEqual(ctxOpts);
            // @ts-expect-error we are testing that the context is readonly
            expect(onFailureCount).toEqual(results);
          }
        });

        it("should properly trigger onFailure handlers during updates & nothing to update", async () => {
          const { error, handleFailure } = await Model.update(
            { prop1: 2 },
            { prop1: 35 },
            contextOptions,
          );

          await handleFailure?.();

          expect(error).toBeDefined();
          expect(cxtOptions).toEqual({ prop1: contextOptions });
          expect(onFailureCount).toEqual({ prop1: 1 });
        });
      });
    });
  });

  describe("onSuccess", () => {
    const contextOptions = { lang: "en" };

    let cxtOptions: any = {},
      initialData = {
        dependent: false,
        lax: "changed",
        readonly: "changed",
        readonlyLax: "",
        required: "changed",
      },
      onSuccessValues: Record<string, unknown> = {},
      propChangeMap: Record<string, boolean> = {};

    const onSuccess =
      (field = "") =>
      (ctx: ReadonlyIvoContext<any>) => {
        cxtOptions[field] = ctx.options;
        onSuccessValues[field] = ctx;
        onSuccessValues.input = ctx.input;
        propChangeMap[field] = true;
      };

    const validator = () => ({ valid: true });

    const Model = new Schema<any>((b, m) =>
      b
        .field(
          m
            .dependent("dependent", "readonlyLax")
            .default(false)
            .resolve(() => true)
            .onSuccess(onSuccess("dependent")),
        )
        .field(m.lax("lax", "").validate(validator).onSuccess(onSuccess("lax")))
        .field(
          m
            .required("requiredReadonly")
            .validate(validator)
            .readonly()
            .onSuccess(onSuccess("requiredReadonly")),
        )
        .field(
          m
            .lax("readonlyLax", "")
            .validate(validator)
            .readonly()
            .onSuccess(onSuccess("readonlyLax")),
        )
        .field(
          m
            .required("required")
            .validate(validator)
            .onSuccess(onSuccess("required")),
        ),
    ).getModel();

    beforeEach(() => {
      cxtOptions = {};
      onSuccessValues = {};
      propChangeMap = {};
    });

    // creation
    it("should call onSuccess handlers at creation", async () => {
      const { data, error, handleSuccess } = await Model.create(
        { required: true, requiredReadonly: true },
        contextOptions,
      );

      await handleSuccess?.();

      expect(error).toBeNull();

      expect(cxtOptions).toEqual({
        dependent: contextOptions,
        lax: contextOptions,
        requiredReadonly: contextOptions,
        readonlyLax: contextOptions,
        required: contextOptions,
      });

      expect(propChangeMap).toEqual({
        dependent: true,
        lax: true,
        requiredReadonly: true,
        readonlyLax: true,
        required: true,
      });

      const changes = null,
        input = onSuccessValues.input,
        isUpdate = false,
        previousValues = null,
        values = data,
        ctx = { changes, input, isUpdate, previousValues, values };

      expect(onSuccessValues).toMatchObject({
        dependent: ctx,
        lax: ctx,
        requiredReadonly: ctx,
        readonlyLax: ctx,
        required: ctx,
      });
    });

    // updates
    it("should call onSuccess handlers during updates with lax props", async () => {
      const { data, error, handleSuccess } = await Model.update(
        initialData,
        { lax: true },
        contextOptions,
      );

      await handleSuccess?.();

      expect(error).toBeNull();
      expect(cxtOptions).toEqual({ lax: contextOptions });
      expect(propChangeMap).toEqual({ lax: true });

      expect(onSuccessValues).toMatchObject({
        lax: expect.objectContaining({
          changes: data,
          isUpdate: true,
          previousValues: initialData,
          values: { ...initialData, lax: true },
        }),
      });
    });

    it("should call onSuccess handlers during updates with readonlyLax & dependent", async () => {
      const { data, error, handleSuccess } = await Model.update(
        initialData,
        { readonlyLax: true },
        contextOptions,
      );

      await handleSuccess?.();

      expect(error).toBeNull();
      expect(cxtOptions).toEqual({
        dependent: contextOptions,
        readonlyLax: contextOptions,
      });
      expect(propChangeMap).toEqual({ dependent: true, readonlyLax: true });

      const changes = data,
        isUpdate = true,
        previousValues = initialData,
        values = { ...initialData, ...data },
        summary = { changes, isUpdate, previousValues, values };

      expect(onSuccessValues).toMatchObject({
        dependent: expect.objectContaining(summary),
        readonlyLax: expect.objectContaining(summary),
      });
    });
  });
});
