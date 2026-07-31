import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { createFieldBuilder } from "../../src/schema/fields";

type Input = {
  name: string;
  status: string;
  grade: number;
  age: number;
};
type Output = Input;

const field = createFieldBuilder<Input, Output>();

describe("field builder prototype: lax()", () => {
  it("supports a bare default() with no validation", async () => {
    const schema = new Schema<Input, Output>({
      name: field.lax("name").default("anonymous"),
      // status: { default: 'active' },
      // grade: { default: 0 },
      // age: { default: 0 },
    });

    const { data, error } = await schema.getModel().create({}, {});

    expect(error).toBeNull();
    expect(data?.name).toBe("anonymous");
  });

  it("supports validate() as the primary validator", async () => {
    const schema = new Schema<Input, Output>({
      name: field
        .lax("name")
        .default("anonymous")
        .validate((value) =>
          typeof value === "string" && value.length > 0
            ? { valid: true, validated: value }
            : { valid: false, reason: "name must be a non-empty string" },
        ),
      // status: { default: "active" },
      // grade: { default: 0 },
      // age: { default: 0 },
    });

    const Model = schema.getModel();

    const rejected = await Model.create({ name: "" }, {});
    expect(rejected.data).toBeNull();
    expect(rejected.error).toMatchObject({
      name: expect.objectContaining({
        reason: "name must be a non-empty string",
      }),
    });

    const accepted = await Model.create({ name: "Ada" }, {});
    expect(accepted.error).toBeNull();
    expect(accepted.data?.name).toBe("Ada");
  });

  it("supports allow() as the primary validator, rejecting values outside the list", async () => {
    const schema = new Schema<Input, Output>({
      // name: { default: "anonymous" },
      status: field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"]),
      // grade: { default: 0 },
      // age: { default: 0 },
    });

    const Model = schema.getModel();

    const rejected = await Model.create({ status: "unknown" }, {});
    expect(rejected.data).toBeNull();
    expect(rejected.error).toMatchObject({
      status: expect.objectContaining({ reason: "value not allowed" }),
    });

    const accepted = await Model.create({ status: "inactive" }, {});
    expect(accepted.error).toBeNull();
    expect(accepted.data?.status).toBe("inactive");
  });

  it("supports allow().allowError() to customize the rejection message", async () => {
    const schema = new Schema<Input, Output>({
      // name: { default: "anonymous" },
      status: field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .allowError("status must be active or inactive"),
      // grade: { default: 0 },
      // age: { default: 0 },
    });

    const { data, error } = await schema
      .getModel()
      .create({ status: "unknown" }, {});

    expect(data).toBeNull();
    expect(error).toMatchObject({
      status: expect.objectContaining({
        reason: "status must be active or inactive",
      }),
    });
  });

  it("supports validate().reValidate() - the secondary validator runs after the primary", async () => {
    const schema = new Schema<Input, Output>({
      // name: { default: "anonymous" },
      // status: { default: "active" },
      grade: field
        .lax("grade")
        .default(0)
        .validate((value) =>
          typeof value === "number"
            ? { valid: true, validated: value }
            : { valid: false, reason: "grade must be a number" },
        )
        .reValidate((value) =>
          value >= 0 && value <= 100
            ? { valid: true, validated: value }
            : { valid: false, reason: "grade must be between 0 and 100" },
        ),
      // age: { default: 0 },
    });

    const Model = schema.getModel();

    const rejectedByPrimary = await Model.create({ grade: "lol" as never }, {});
    expect(rejectedByPrimary.error).toMatchObject({
      grade: expect.objectContaining({ reason: "grade must be a number" }),
    });

    const rejectedBySecondary = await Model.create({ grade: 150 }, {});
    expect(rejectedBySecondary.error).toMatchObject({
      grade: expect.objectContaining({
        reason: "grade must be between 0 and 100",
      }),
    });

    const accepted = await Model.create({ grade: 87 }, {});
    expect(accepted.error).toBeNull();
    expect(accepted.data?.grade).toBe(87);
  });

  it("supports allow().reValidate() - the secondary validator runs even though allow(), not validate(), is primary", async () => {
    const schema = new Schema<Input, Output>({
      // name: { default: "anonymous" },
      // status: { default: "active" },
      // grade: { default: 0 },
      age: field
        .lax("age")
        .default(18)
        .allow([18, 21, 30, 40, 50])
        .reValidate((value) =>
          value >= 21
            ? { valid: true, validated: value }
            : { valid: false, reason: "must be at least 21" },
        ),
    });

    const Model = schema.getModel();

    const rejectedByAllow = await Model.create({ age: 19 }, {});
    expect(rejectedByAllow.error).toMatchObject({
      age: expect.objectContaining({ reason: "value not allowed" }),
    });

    const rejectedBySecondary = await Model.create({ age: 18 }, {});
    expect(rejectedBySecondary.error).toMatchObject({
      age: expect.objectContaining({ reason: "must be at least 21" }),
    });

    const accepted = await Model.create({ age: 30 }, {});
    expect(accepted.error).toBeNull();
    expect(accepted.data?.age).toBe(30);
  });

  it("supports required()/ignore()/ignoreInit()/ignoreUpdate()/readonly()/onDelete()/onFailure()/onSuccess()", async () => {
    let deleted = false;
    let succeeded = false;
    let failed = false;

    const schema = new Schema<Input, Output>({
      name: field
        .lax("name")
        .default("anonymous")
        .validate((value) =>
          typeof value === "string"
            ? { valid: true, validated: value }
            : { valid: false, reason: "invalid name" },
        )
        .required(() => true)
        .onDelete(() => {
          deleted = true;
        })
        .onSuccess(() => {
          succeeded = true;
        })
        .onFailure(() => {
          failed = true;
        }),
      status: field.lax("status").default("active").readonly(),
      grade: field.lax("grade").default(0).ignoreUpdate(),
      age: field
        .lax("age")
        .default(0)
        .ignore(() => false),
    });

    const Model = schema.getModel();

    const missing = await Model.create({}, {});
    expect(missing.error).toMatchObject({
      name: expect.objectContaining({ reason: "'name' is required" }),
    });

    // onFailure only cleans up fields actually present in the raw input, and
    // must be invoked explicitly by the caller, so trigger it with an
    // invalid (rather than missing) value.
    const invalid = await Model.create({ name: 123 as never }, {});
    expect(invalid.error).toMatchObject({
      name: expect.objectContaining({ reason: "invalid name" }),
    });
    if (!invalid.handleFailure)
      throw new Error("expected handleFailure to be present");
    await invalid.handleFailure();
    expect(failed).toBe(true);

    const { data, handleSuccess } = await Model.create({ name: "Ada" }, {});
    if (!data) throw new Error("expected data to be present");

    await handleSuccess();
    await Model.delete(data, {});

    expect(succeeded).toBe(true);
    expect(deleted).toBe(true);

    const updated = await Model.update(data, { status: "inactive" }, {});
    expect(updated.data).toBeNull();
  });

  it("accepts an array of handlers in a single onDelete()/onSuccess()/onFailure() call", async () => {
    const succeededBy: string[] = [];

    const schema = new Schema<Input, Output>({
      name: field
        .lax("name")
        .default("anonymous")
        .onSuccess([
          () => {
            succeededBy.push("first");
          },
          () => {
            succeededBy.push("second");
          },
        ]),
      // status: { default: "active" },
      // grade: { default: 0 },
      // age: { default: 0 },
    });

    const Model = schema.getModel();
    const { data, handleSuccess } = await Model.create({}, {});

    if (!data) throw new Error("expected data to be present");

    await handleSuccess();

    expect(succeededBy).toEqual(["first", "second"]);
  });

  describe("invalid usage (compile-time only - nothing here is meant to run)", () => {
    it("rejects calling anything before default()", () => {
      const builder = field.lax("name");

      // @ts-expect-error - allow()/validate()/etc. aren't available until default() has been set
      builder.allow?.(["a", "b"]);
    });

    it("makes allow() and validate() mutually exclusive", () => {
      const withAllow = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"]);
      // @ts-expect-error - validate() isn't available once allow() has been chosen as the primary validator
      withAllow.validate?.(() => true);

      const withValidator = field
        .lax("name")
        .default("anonymous")
        .validate(() => true);
      // @ts-expect-error - allow() isn't available once validate() has been chosen as the primary validator
      withValidator.allow?.(["a", "b"]);
    });

    it("rejects allowError() before allow()", () => {
      const withValidator = field
        .lax("name")
        .default("anonymous")
        .validate(() => true);

      // @ts-expect-error - allowError() only becomes available once allow() has been called
      withValidator.allowError?.("nope");
    });

    it("rejects reValidate() before allow() or validate()", () => {
      const builder = field.lax("name").default("anonymous");

      // @ts-expect-error - reValidate() isn't available until allow() or validate() has been called
      builder.reValidate?.(() => true);
    });

    it("never exposes a callable .build(), at any stage", () => {
      const builder = field.lax("name").default("anonymous");

      // @ts-expect-error - build() doesn't exist; it's resolved internally by Schema only
      builder.build?.();

      const validated = builder.validate(() => true);

      // @ts-expect-error - build() doesn't exist even on the buildable stage
      validated.build?.();
    });

    it("rejects a second call to allowError()/reValidate()/required()/readonly()", () => {
      const decorated = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .allowError("nope")
        .reValidate(() => true)
        .required(() => true)
        .readonly();

      // @ts-expect-error - allowError() was already consumed
      decorated.allowError?.("nope again");
      // @ts-expect-error - reValidate() was already consumed
      decorated.reValidate?.(() => true);
      // @ts-expect-error - required() was already consumed
      decorated.required?.(() => true);
      // @ts-expect-error - readonly() was already consumed
      decorated.readonly?.();
    });
  });
});
