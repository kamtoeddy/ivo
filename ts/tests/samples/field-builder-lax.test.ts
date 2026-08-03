import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { newFieldMaker } from "../../src/schema/fields";

type Input = {
  name: string;
  status: string;
  grade: number;
  age: number;
};
type Output = Input;

const field = newFieldMaker<Input, Output>();

describe("field builder prototype: lax()", () => {
  it("should allow a bare default() with no validation", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b.field(field.lax("name").default("anonymous")),
    );

    const { data, error } = await schema.getModel().create({}, {});

    expect(error).toBeNull();
    expect(data?.name).toBe("anonymous");
  });

  it("should allow validate() as the primary validator", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b.field(
        field
          .lax("name")
          .default("anonymous")
          .validate((value) =>
            typeof value === "string" && value.length > 0
              ? { valid: true, validated: value }
              : { valid: false, reason: "name must be a non-empty string" },
          ),
      ),
    );

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

  it("should allow allow() as the primary validator, rejecting values outside the list", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b.field(
        field.lax("status").default("active").allow(["active", "inactive"]),
      ),
    );

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

  it("should allow allow().allowError() to customize the rejection message", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b.field(
        field
          .lax("status")
          .default("active")
          .allow(["active", "inactive"])
          .allowError("status must be active or inactive"),
      ),
    );

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

  it("should allow validate().reValidate() - the secondary validator runs after the primary", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b.field(
        field
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
      ),
    );

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

  it("should allow allow().reValidate() - the secondary validator runs even though allow(), not validate(), is primary", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b.field(
        field
          .lax("age")
          .default(18)
          .allow([18, 21, 30, 40, 50])
          .reValidate((value) =>
            value >= 21
              ? { valid: true, validated: value }
              : { valid: false, reason: "must be at least 21" },
          ),
      ),
    );

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

  it("should allow required()/ignore()/ignoreInit()/ignoreUpdate()/readonly()/onDelete()/onFailure()/onSuccess()", async () => {
    let deleted = false;
    let succeeded = false;
    let failed = false;

    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
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
        )
        .field(field.lax("status").default("active").readonly())
        .field(field.lax("grade").default(0).ignoreUpdate())
        .field(
          field
            .lax("age")
            .default(0)
            .ignore(() => false),
        ),
    );

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

  it("should accept an array of handlers in a single onDelete()/onSuccess()/onFailure() call", async () => {
    const deletedBy: string[] = [];
    const failedBy: string[] = [];
    const succeededBy: string[] = [];
    const defaultName = "anonymous";

    const Model = new Schema<Input, Output>((b, m) =>
      b.field(
        m
          .lax("name")
          .default(defaultName)
          .validate((v) => v === "valid")
          .onDelete([
            () => deletedBy.push("first"),
            () => deletedBy.push("second"),
          ])
          .onFailure([
            () => failedBy.push("first"),
            () => failedBy.push("second"),
          ])
          .onSuccess([
            () => succeededBy.push("first"),
            () => succeededBy.push("second"),
          ]),
      ),
    ).getModel();

    const { data, handleSuccess } = await Model.create({}, {});

    // @ts-expect-error - data should be non-null with default value
    expect(data).toEqual({ name: defaultName });

    await handleSuccess?.();

    expect(deletedBy).toEqual([]);
    expect(failedBy).toEqual([]);
    expect(succeededBy).toEqual(["first", "second"]);

    const { data: data2, handleFailure } = await Model.create(
      {
        name: "invalid",
      },
      {},
    );
    expect(data2).toEqual(null);

    await handleFailure?.();

    expect(deletedBy).toEqual([]);
    expect(failedBy).toEqual(["first", "second"]);
    expect(succeededBy).toEqual(["first", "second"]);

    // @ts-expect-error - data should be non-null with default value
    await Model.delete(data);

    expect(deletedBy).toEqual(["first", "second"]);
    expect(failedBy).toEqual(["first", "second"]);
    expect(succeededBy).toEqual(["first", "second"]);
  });

  describe("invalid usage (compile-time only - nothing here is meant to run)", () => {
    it("should never expose a callable .build()", () => {
      const builder = field.lax("name").default("anonymous");

      // @ts-expect-error - build() doesn't exist; it's resolved internally by Schema only
      builder.build?.();

      const allow = builder.allow(["anonymous", "user", "root"]);

      // @ts-expect-error - build() doesn't exist even on the buildable stage
      allow.build?.();

      const validated = builder.validate(() => true);

      // @ts-expect-error - build() doesn't exist even on the buildable stage
      validated.build?.();
    });

    it("should reject calling anything before default()", () => {
      const builder = field.lax("name");

      // @ts-expect-error - allow()/validate()/etc. aren't available until default() has been set
      builder.allow?.(["a", "b"]);

      // @ts-expect-error - allow()/validate()/etc. aren't available until default() has been set
      builder.validate?.(() => true);
    });

    it("should reject allow() + validate() or validate() + allow()", () => {
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

    it("should reject allowError() before allow()", () => {
      const withValidator = field.lax("name").default("anonymous");

      // @ts-expect-error - allowError() only becomes available once allow() has been called
      withValidator.allowError?.("nope");
    });

    it("should reject reValidate() before allow() or validate()", () => {
      const builder = field.lax("name").default("anonymous");

      // @ts-expect-error - reValidate() isn't available until allow() or validate() has been called
      builder.reValidate?.(() => true);
    });

    it("should reject a second call to allowError()/reValidate()/required()/readonly()", () => {
      const decorated = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .allowError("nope")
        .reValidate(() => true)
        .required(() => true)
        .readonly();

      // @ts-expect-error - allowError() was already provided
      decorated.allowError?.("nope again");
      // @ts-expect-error - reValidate() was already provided
      decorated.reValidate?.(() => true);
      // @ts-expect-error - required() was already provided
      decorated.required?.(() => true);
      // @ts-expect-error - readonly() was already provided
      decorated.readonly?.();
    });

    it("should reject a second call to readonly()", () => {
      const decorated = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .readonly();

      // @ts-expect-error - readonly was already provided
      decorated.readonly?.();
    });

    it("should reject a second call to ignoreUpdate()", () => {
      const decorated = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => false);
    });

    it("should reject a second call to ignoreUpdate(() => boolean)", () => {
      const decorated = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => false);
    });

    it("should reject allow() + ignoreInit() + ignoreUpdate()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit();

      // @ts-expect-error - ignoreInit() and ignoreUpdate() should not be provided together
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate and ignoreInit() should not be provided together
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);
    });

    it("should reject validate() + ignoreInit() + ignoreUpdate()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit();

      // @ts-expect-error - ignoreInit() and ignoreUpdate() should not be provided together
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate and ignoreInit() should not be provided together
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);
    });

    it("should accept allow() + ignoreInit() + ignoreUpdate(() => boolean)", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit()
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate(() => false)
        .ignoreInit();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => false);
    });

    it("should accept validate() + ignoreInit() + ignoreUpdate(() => boolean)", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit()
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreUpdate(() => false)
        .ignoreInit();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => false);
    });

    it("should accept allow() + ignoreInit(() => boolean) + ignoreUpdate()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit(() => false)
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate()
        .ignoreInit(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => false);
    });

    it("should accept validate() + ignoreInit(() => boolean) + ignoreUpdate()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit(() => false)
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreUpdate()
        .ignoreInit(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => false);
    });

    it("should accept allow() + ignoreInit(() => boolean) + ignoreUpdate(() => boolean)", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit(() => false)
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate(() => false)
        .ignoreInit(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => false);
    });

    it("should accept validate() + ignoreInit(() => boolean) + ignoreUpdate(() => boolean)", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit(() => false)
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreUpdate(() => false)
        .ignoreInit(() => false);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      ignoreUpdateFirst.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreUpdateFirst.ignoreInit?.(() => false);
    });

    it("should accept allow() + ignoreInit() + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit()
        .readonly();

      // @ts-expect-error - readonly was already provided
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .readonly()
        .ignoreInit();

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => false);
    });

    it("should accept validate() + ignoreInit() + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit()
        .readonly();

      // @ts-expect-error - readonly was already provided
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .readonly()
        .ignoreInit();

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => false);
    });

    it("should accept allow() + ignoreInit(() => boolean) + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit(() => true)
        .readonly();

      // @ts-expect-error - readonly was already provided
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .readonly()
        .ignoreInit(() => true);

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => false);
    });

    it("should accept validate() + ignoreInit(() => boolean) + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit(() => true)
        .readonly();

      // @ts-expect-error - readonly was already provided
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .readonly()
        .ignoreInit(() => true);

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => false);
    });

    it("should reject allow() + ignoreUpdate() + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate() + readonly() should not be allowed
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreUpdate?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreUpdate?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .readonly();

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - readonly() + ignoreUpdate() should not be allowed
      readonlyFirst.ignoreUpdate?.();
    });

    it("should reject validate() + ignoreUpdate() + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit();

      // @ts-expect-error - ignoreUpdate() + readonly() should not be allowed
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .readonly();

      // @ts-expect-error - readonly() + ignoreUpdate() should not be allowed
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();
    });

    it("should reject allow() + ignore(() => boolean) + ignoreInit()/ignoreInit(() => boolean) or ignoreUpdate()/ignoreUpdate(() => boolean)", () => {
      let ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit();

      // @ts-expect-error - ignoreInit() + ignore() should not be allowed
      ignoreInitFirst.ignore(() => false);

      ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit(() => false);

      // @ts-expect-error - ignoreInit(() => boolean) + ignore() should not be allowed
      ignoreInitFirst.ignore(() => false);

      const ignoreFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignore(() => false);

      // @ts-expect-error - ignore() + ignoreInit() should not be allowed
      ignoreFirst.ignoreInit();

      // @ts-expect-error - ignore() + ignoreInit(() => boolean) should not be allowed
      ignoreFirst.ignoreInit(() => true);

      // @ts-expect-error - ignore() + ignoreInit(() => boolean) should not be allowed
      ignoreFirst.ignoreInit(() => false);

      let ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate() + ignore() should not be allowed
      ignoreUpdateFirst.ignore(() => false);

      ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate(() => boolean) + ignore() should not be allowed
      ignoreUpdateFirst.ignore(() => false);

      const ignoreBeforeIgnoreUpdate = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignore(() => false);

      // @ts-expect-error - ignore() + ignoreUpdate() should not be allowed
      ignoreBeforeIgnoreUpdate.ignoreUpdate();

      // @ts-expect-error - ignore() + ignoreUpdate(() => boolean) should not be allowed
      ignoreBeforeIgnoreUpdate.ignoreUpdate(() => true);

      // @ts-expect-error - ignore() + ignoreUpdate(() => boolean) should not be allowed
      ignoreBeforeIgnoreUpdate.ignoreUpdate(() => false);
    });

    it("should reject validate() + ignore(() => boolean) + ignoreInit()/ignoreInit(() => boolean) or ignoreUpdate()/ignoreUpdate(() => boolean)", () => {
      let ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit();

      // @ts-expect-error - ignoreInit() + ignore() should not be allowed
      ignoreInitFirst.ignore(() => false);

      ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit(() => false);

      // @ts-expect-error - ignoreInit(() => boolean) + ignore() should not be allowed
      ignoreInitFirst.ignore(() => false);

      const ignoreFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignore(() => false);

      // @ts-expect-error - ignore() + ignoreInit() should not be allowed
      ignoreFirst.ignoreInit();

      // @ts-expect-error - ignore() + ignoreInit(() => boolean) should not be allowed
      ignoreFirst.ignoreInit(() => true);

      // @ts-expect-error - ignore() + ignoreInit(() => boolean) should not be allowed
      ignoreFirst.ignoreInit(() => false);

      let ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreUpdate();

      // @ts-expect-error - ignoreUpdate() + ignore() should not be allowed
      ignoreUpdateFirst.ignore(() => false);

      ignoreUpdateFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreUpdate(() => false);

      // @ts-expect-error - ignoreUpdate(() => boolean) + ignore() should not be allowed
      ignoreUpdateFirst.ignore(() => false);

      const ignoreBeforeIgnoreUpdate = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignore(() => false);

      // @ts-expect-error - ignore() + ignoreUpdate() should not be allowed
      ignoreBeforeIgnoreUpdate.ignoreUpdate();

      // @ts-expect-error - ignore() + ignoreUpdate(() => boolean) should not be allowed
      ignoreBeforeIgnoreUpdate.ignoreUpdate(() => true);

      // @ts-expect-error - ignore() + ignoreUpdate(() => boolean) should not be allowed
      ignoreBeforeIgnoreUpdate.ignoreUpdate(() => false);
    });

    it("should allow allow() + ignore(() => boolean) + readonly()", () => {
      const ignoreFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignore(() => false)
        .readonly();

      // @ts-expect-error - ignore was already provided
      ignoreFirst.rignore(() => false);

      // @ts-expect-error - readonly was already provided
      ignoreFirst.readonly();

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .readonly()
        .ignore(() => false);

      // @ts-expect-error - ignore was already provided
      readonlyFirst.rignore(() => false);

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();
    });

    it("should allow validate() + ignore(() => boolean) + readonly()", () => {
      const ignoreFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignore(() => false)
        .readonly();

      // @ts-expect-error - ignore was already provided
      ignoreFirst.rignore(() => false);

      // @ts-expect-error - readonly was already provided
      ignoreFirst.readonly();

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .readonly()
        .ignore(() => false);

      // @ts-expect-error - ignore was already provided
      readonlyFirst.rignore(() => false);

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();
    });

    it("should accept allow() + ignoreUpdate(() => boolean) + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .ignoreInit(() => true)
        .readonly();

      // @ts-expect-error - readonly was already provided
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .allow(["active", "inactive"])
        .readonly()
        .ignoreInit(() => true);

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => false);
    });

    it("should accept validate() + ignoreUpdate(() => boolean) + readonly()", () => {
      const ignoreInitFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .ignoreInit(() => true)
        .readonly();

      // @ts-expect-error - readonly was already provided
      ignoreInitFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      ignoreInitFirst.ignoreInit?.(() => false);

      const readonlyFirst = field
        .lax("status")
        .default("active")
        .validate(() => false)
        .readonly()
        .ignoreInit(() => true);

      // @ts-expect-error - readonly was already provided
      readonlyFirst.readonly();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.();

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => true);

      // @ts-expect-error - ignoreInit was already provided
      readonlyFirst.ignoreInit?.(() => false);
    });
  });
});
