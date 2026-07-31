import { afterEach, describe, expect, it } from "bun:test";

import { type IvoContext, Schema } from "../../src";
import { createFieldBuilder } from "../../src/schema/fields";

describe("Context options", () => {
  describe("RequiredBy", () => {
    const contextOptions = { lang: "en" };
    const validator = () => true;

    type Input = { lax: string; name: string; price: number };
    type Output = { lax: string; name: string; price: number };
    type CtxOptions = typeof contextOptions;

    function handleRequired(prop: string) {
      return ({ options }: IvoContext<Input, Output, CtxOptions>) => {
        ctxOptions[prop] = options;

        return false;
      };
    }

    const field = createFieldBuilder<Input, Output, CtxOptions>();

    const Model = new Schema<Input, Output, CtxOptions>((b) =>
      b
        .field(field.lax("lax").default(""))
        .field(
          field
            .lax("name")
            .default("")
            .required(handleRequired("name"))
            .validate(validator),
        )
        .field(field.lax("price").default(0).required(handleRequired("price"))),
    ).getModel();

    let ctxOptions: any = {};

    afterEach(() => {
      ctxOptions = {};
    });

    it('provided "contextOptions" should be accessible in requiredBy methods at creation', async () => {
      await Model.create({ lax: "lax" }, contextOptions);

      expect(ctxOptions).toEqual({
        name: contextOptions,
        price: contextOptions,
      });
    });

    it('provided "contextOptions" should be accessible in requiredBy methods during updates', async () => {
      const { options } = await Model.update(
        { lax: "lax", name: "test", price: 4 },
        { lax: "update" },
        contextOptions,
      );

      expect(options).toEqual(contextOptions);
      expect(ctxOptions).toEqual({
        name: contextOptions,
        price: contextOptions,
      });
    });
  });

  describe("should accept functions properly", () => {
    let called = false;

    function ctxHandler() {
      called = true;
    }

    type Input = { name: string };
    type Output = { name: string };
    type CtxOptions = { ctxHandler: () => any };

    const field = createFieldBuilder<Input, Output, CtxOptions>();

    const Model = new Schema<Input, Output, CtxOptions>((b) =>
      b.field(
        field
          .lax("name")
          .default("")
          .validate((_, { options }) => {
            options.ctxHandler();
            return true;
          })
          .onDelete((_, options) => options.ctxHandler())
          .onSuccess(({ options }) => options.ctxHandler()),
      ),
    ).getModel();

    afterEach(() => {
      called = false;
    });

    it("should allow provided function in ctx options at creation", async () => {
      expect(called).toBe(false);

      await Model.create({ name: "lol" }, { ctxHandler });

      expect(called).toBe(true);
    });

    it("should allow provided function in ctx options onSuccess after creation", async () => {
      expect(called).toBe(false);

      const { handleSuccess } = await Model.create({}, { ctxHandler });

      expect(called).toBe(false);

      handleSuccess?.();

      expect(called).toBe(true);
    });

    it("should allow provided function in ctx options during updates", async () => {
      expect(called).toBe(false);

      await Model.update({ name: "lol" }, { name: "updated" }, { ctxHandler });

      expect(called).toBe(true);
    });

    it("should allow provided function in ctx options on deletion", async () => {
      expect(called).toBe(false);

      await Model.delete({ name: "lol" }, { ctxHandler });

      expect(called).toBe(true);
    });
  });
});
