import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { newFieldMaker } from "../../src/schema/fields";

type Input = {
  email: string;
  role: string;
  plan: string;
  score: number;
};
type Output = Input;

const field = newFieldMaker<Input, Output>();

describe("field builder: required()", () => {
  it("should allow validate() as the primary validator", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .required("email")
            .validate((value) =>
              typeof value === "string" && value.includes("@")
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid email" },
            ),
        )
        .field(field.required("role").allow(["admin", "member"]))
        .field(field.required("plan").allow(["free", "pro"]))
        .field(
          field
            .required("score")
            .validate((value) =>
              typeof value === "number"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid score" },
            ),
        ),
    );

    const Model = schema.getModel();

    const missing = await Model.create({}, {});
    expect(missing.error).toMatchObject({
      email: expect.objectContaining({ reason: "'email' is required" }),
    });

    const rejected = await Model.create(
      { email: "not-an-email", role: "admin", plan: "free", score: 1 },
      {},
    );
    expect(rejected.error).toMatchObject({
      email: expect.objectContaining({ reason: "invalid email" }),
    });

    const accepted = await Model.create(
      { email: "ada@ivo.dev", role: "admin", plan: "free", score: 1 },
      {},
    );
    expect(accepted.error).toBeNull();
    expect(accepted.data?.email).toBe("ada@ivo.dev");
  });

  it("should allow allow() as the primary validator, rejecting values outside the list", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .required("email")
            .validate((value) =>
              typeof value === "string"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid email" },
            ),
        )
        .field(field.required("role").allow(["admin", "member"]))
        .field(field.required("plan").allow(["free", "pro"]))
        .field(
          field
            .required("score")
            .validate((value) =>
              typeof value === "number"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid score" },
            ),
        ),
    );

    const Model = schema.getModel();

    const rejected = await Model.create(
      { email: "ada@ivo.dev", role: "owner", plan: "free", score: 1 },
      {},
    );
    expect(rejected.error).toMatchObject({
      role: expect.objectContaining({ reason: "value not allowed" }),
    });

    const accepted = await Model.create(
      { email: "ada@ivo.dev", role: "member", plan: "free", score: 1 },
      {},
    );
    expect(accepted.error).toBeNull();
    expect(accepted.data?.role).toBe("member");
  });

  it("should allow allow().allowError() to customize the rejection message", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .required("email")
            .validate((value) =>
              typeof value === "string"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid email" },
            ),
        )
        .field(
          field
            .required("role")
            .allow(["admin", "member"])
            .allowError("role must be admin or member"),
        )
        .field(field.required("plan").allow(["free", "pro"]))
        .field(
          field
            .required("score")
            .validate((value) =>
              typeof value === "number"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid score" },
            ),
        ),
    );

    const { data, error } = await schema
      .getModel()
      .create(
        { email: "ada@ivo.dev", role: "owner", plan: "free", score: 1 },
        {},
      );

    expect(data).toBeNull();
    expect(error).toMatchObject({
      role: expect.objectContaining({ reason: "role must be admin or member" }),
    });
  });

  it("should allow validate().reValidate() and allow().reValidate()", async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .required("email")
            .validate((value) =>
              typeof value === "string"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid email" },
            ),
        )
        .field(field.required("role").allow(["admin", "member"]))
        .field(
          field
            .required("plan")
            .allow(["free", "pro", "enterprise"])
            .reValidate((value) =>
              value !== "enterprise"
                ? { valid: true, validated: value }
                : {
                    valid: false,
                    reason: "enterprise plan requires sales approval",
                  },
            ),
        )
        .field(
          field
            .required("score")
            .validate((value) =>
              typeof value === "number"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid score" },
            )
            .reValidate((value) =>
              value >= 0
                ? { valid: true, validated: value }
                : { valid: false, reason: "score must be non-negative" },
            ),
        ),
    );

    const Model = schema.getModel();

    const rejectedByAllowSecondary = await Model.create(
      { email: "ada@ivo.dev", role: "admin", plan: "enterprise", score: 1 },
      {},
    );
    expect(rejectedByAllowSecondary.error).toMatchObject({
      plan: expect.objectContaining({
        reason: "enterprise plan requires sales approval",
      }),
    });

    const rejectedByValidateSecondary = await Model.create(
      { email: "ada@ivo.dev", role: "admin", plan: "free", score: -1 },
      {},
    );
    expect(rejectedByValidateSecondary.error).toMatchObject({
      score: expect.objectContaining({ reason: "score must be non-negative" }),
    });

    const accepted = await Model.create(
      { email: "ada@ivo.dev", role: "admin", plan: "free", score: 5 },
      {},
    );
    expect(accepted.error).toBeNull();
  });

  it("should allow readonly()/ignoreUpdate(resolver?) and onDelete()/onFailure()/onSuccess()", async () => {
    let deleted = false;
    let succeeded = false;

    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .required("email")
            .validate((value) =>
              typeof value === "string"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid email" },
            )
            .readonly()
            .onDelete(() => {
              deleted = true;
            })
            .onSuccess(() => {
              succeeded = true;
            }),
        )
        .field(
          field
            .required("role")
            .allow(["admin", "member"])
            .ignoreUpdate(() => false),
        )
        .field(field.required("plan").allow(["free", "pro"]))
        .field(
          field
            .required("score")
            .validate((value) =>
              typeof value === "number"
                ? { valid: true, validated: value }
                : { valid: false, reason: "invalid score" },
            ),
        ),
    );

    const Model = schema.getModel();

    const { data, handleSuccess } = await Model.create(
      { email: "ada@ivo.dev", role: "admin", plan: "free", score: 1 },
      {},
    );
    if (!data) throw new Error("expected data to be present");

    await handleSuccess();
    await Model.delete(data, {});

    expect(succeeded).toBe(true);
    expect(deleted).toBe(true);

    const updated = await Model.update(data, { email: "other@ivo.dev" }, {});
    expect(updated.data).toBeNull();
  });

  describe("invalid usage (compile-time only - nothing here is meant to run)", () => {
    it("should never expose a callable .build()", () => {
      const allow = field.required("role").allow(["anonymous", "user", "root"]);

      // @ts-expect-error - build() doesn't exist even on the buildable stage
      allow.build?.();

      const validated = field.required("role").validate(() => true);

      // @ts-expect-error - build() doesn't exist even on the buildable stage
      validated.build?.();
    });

    it("makes allow() and validate() mutually exclusive", () => {
      const withAllow = field.required("role").allow(["admin", "member"]);
      // @ts-expect-error - validate() isn't available once allow() has been chosen as the primary validator
      withAllow.validate?.(() => true);

      const withValidator = field.required("email").validate(() => true);
      // @ts-expect-error - allow() isn't available once validate() has been chosen as the primary validator
      withValidator.allow?.(["a", "b"]);
    });

    it("should reject allowError() before allow()", () => {
      const withValidator = field.required("email").validate(() => true);

      // @ts-expect-error - allowError() only becomes available once allow() has been called
      withValidator.allowError?.("nope");
    });

    it("should reject a second call to allowError()/reValidate()", () => {
      const decorated = field
        .required("role")
        .allow(["admin", "member"])
        .allowError("nope")
        .reValidate(() => true);

      // @ts-expect-error - allowError() was already provided
      decorated.allowError?.("nope again");
      // @ts-expect-error - reValidate() was already provided
      decorated.reValidate?.(() => true);
    });

    it("should reject a second call to readonly()", () => {
      const decorated = field
        .required("role")
        .allow(["admin", "member"])
        .readonly();

      // @ts-expect-error - readonly was already provided
      decorated.readonly?.();
    });

    it("should reject a second call to ignoreUpdate()", () => {
      const decorated = field
        .required("role")
        .allow(["admin", "member"])
        .ignoreUpdate(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => false);
    });

    it("should reject allow() + (ignoreUpdate() + readonly())", () => {
      const decorated = field
        .required("role")
        .allow(["admin", "member"])
        .ignoreUpdate(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreUpdate() + readonly() should not be allowed
      decorated.readonly();
    });

    it("should reject allow() + (readonly() + ignoreUpdate())", () => {
      const decorated = field
        .required("role")
        .allow(["admin", "member"])
        .readonly();

      // @ts-expect-error - readonly was already provided
      decorated.readonly();

      // @ts-expect-error - readonly() + ignoreUpdate() should not be allowed
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - readonly() + ignoreUpdate() should not be allowed
      decorated.ignoreUpdate?.(() => false);
    });

    it("should reject validate() + (ignoreUpdate() + readonly())", () => {
      const decorated = field
        .required("role")
        .validate(() => true)
        .ignoreUpdate(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - ignoreUpdate was already provided
      decorated.ignoreUpdate?.(() => false);

      // @ts-expect-error - ignoreUpdate() + readonly() should not be allowed
      decorated.readonly();
    });

    it("should reject validate() + (readonly() + ignoreUpdate())", () => {
      const decorated = field
        .required("role")
        .validate(() => true)
        .readonly();

      // @ts-expect-error - readonly was already provided
      decorated.readonly();

      // @ts-expect-error - readonly() + ignoreUpdate() should not be allowed
      decorated.ignoreUpdate?.(() => true);

      // @ts-expect-error - readonly() + ignoreUpdate() should not be allowed
      decorated.ignoreUpdate?.(() => false);
    });
  });
});
