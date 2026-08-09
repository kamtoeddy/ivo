import { describe, expect, it } from "bun:test";
import { Schema } from "../../../src";
import {
  expectFailure,
  expectNoFailure,
  makeFx,
  validator,
} from "../../_utils";

describe("options", () => {
  describe("ignoreUpdate", () => {
    it("should respect option to ignore updates with empty fields array", async () => {
      const defaultValue = "default_lax_value";
      const IGNORE_VALUE = "ignore_value";

      const Model = new Schema<{ lax: string }>(
        (b) => b.field(b.lax("lax", defaultValue)),
        {
          ignoreUpdate: {
            fields: [] as never,
            resolver: (ctx) => ctx.rawInput.lax === IGNORE_VALUE,
          },
        },
      ).getModel();

      const data = { lax: "lax_value" };

      const { error } = await Model.update(data, { lax: IGNORE_VALUE }, {});

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });

      const laxUpdate = "should_not_ignore";
      const { data: updates } = await Model.update(
        data,
        { lax: laxUpdate },
        {},
      );

      expect(updates).toEqual({ lax: laxUpdate });
    });
  });

  describe("onDelete", () => {
    it("should properly trigger onDelete handlers", async () => {
      let triggered = false;

      const Model = new Schema<{ lax: number; lax_1: number }>(
        (b) => b.field(b.lax("lax", 1234)).field(b.lax("lax_1", 5678)),
        {
          onDelete: () => {
            triggered = true;
          },
        },
      ).getModel();

      await Model.delete({ lax: 2, lax_1: 3 }, {});

      expect(triggered).toBe(true);
    });

    it("should properly trigger all onDelete handlers", async () => {
      let firstTriggered = false;
      let secondTriggered = false;

      const Model = new Schema<{ lax: number; lax_1: number }>(
        (b) => b.field(b.lax("lax", 1234)).field(b.lax("lax_1", 5678)),
        {
          onDelete: [
            () => {
              firstTriggered = true;
            },
            () => {
              secondTriggered = true;
            },
          ],
        },
      ).getModel();

      await Model.delete({ lax: 2, lax_1: 3 }, {});

      expect(firstTriggered).toBe(true);
      expect(secondTriggered).toBe(true);
    });
  });

  describe("onSuccess", () => {
    it("should reject if the fields array contains any duplicates", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator)),
        { onSuccess: { fields: ["lax", "lax"], resolver: () => undefined } },
      );

      expectFailure(
        toFail,
        'remove duplicates of "lax" in grouped onSuccess config',
      );
    });

    it("should reject if the fields array contains any string that is not a field on schema", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator)),
        {
          onSuccess: {
            fields: ["lax", "invalid_field"],
            resolver: () => undefined,
          },
        },
      );

      expectFailure(toFail, '"invalid_field" does not exist on your schema');
    });

    it("should reject if an alias with foreign name is provided to the fields array", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(
              b
                .dependent("dependent", ["lax", "virtualField"])
                .default(1)
                .resolve(() => 2),
            )
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator))
            .field(
              b.virtual("virtualField").alias("alias").validate(validator),
            ),
        {
          onSuccess: {
            fields: ["lax", "lax_1", "alias"],
            resolver: () => undefined,
          },
        },
      );

      expectFailure(toFail, '"alias" is an alias; use "virtualField" instead');
    });

    it("should reject createdAt timestamp with default name in the fields array", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator)),
        {
          timestamps: { createdAt: true },
          onSuccess: {
            fields: ["lax", "lax_1", "createdAt"],
            resolver: () => undefined,
          },
        },
      );

      expectFailure(
        toFail,
        'timestamps are not allowed in onSuccess. remove "createdAt"',
      );
    });

    it("should reject createdAt timestamp with custom name in the fields array", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator)),
        {
          timestamps: { createdAt: "customCreatedAt" },
          onSuccess: {
            fields: ["lax", "lax_1", "customCreatedAt"],
            resolver: () => undefined,
          },
        },
      );

      expectFailure(
        toFail,
        'timestamps are not allowed in onSuccess. remove "customCreatedAt"',
      );
    });

    it("should reject updatedAt timestamp with default name in the fields array", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator)),
        {
          timestamps: { updatedAt: true },
          onSuccess: {
            fields: ["lax", "lax_1", "updatedAt"],
            resolver: () => undefined,
          },
        },
      );

      expectFailure(
        toFail,
        'timestamps are not allowed in onSuccess. remove "updatedAt"',
      );
    });

    it("should reject updatedAt timestamp with custom name in the fields array", () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax("lax", 1234).validate(validator))
            .field(b.lax("lax_1", 5678).validate(validator)),
        {
          timestamps: { updatedAt: { key: "customUpdatedAt" } },
          onSuccess: {
            fields: ["lax", "lax_1", "customUpdatedAt"],
            resolver: () => undefined,
          },
        },
      );

      expectFailure(
        toFail,
        'timestamps are not allowed in onSuccess. remove "customUpdatedAt"',
      );
    });

    it("should allow constants and dependents in the fields array", () => {
      const toPass = makeFx(
        (b) =>
          b
            .field(b.constant("id", 1234))
            .field(
              b
                .dependent("dependent", "lax")
                .default(1)
                .resolve((ctx: any) => ctx.values.dependent + 1),
            )
            .field(b.lax("lax", 5678)),
        {
          onSuccess: { fields: ["id", "dependent"], resolver: () => undefined },
        },
      );

      expectNoFailure(toPass);
      toPass();
    });
  });
});
