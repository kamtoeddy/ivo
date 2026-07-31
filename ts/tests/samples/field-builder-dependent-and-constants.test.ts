import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { newFieldMaker } from "../../src/schema/fields";

type Input = { price: number; qty: number };
type Output = { id: number; price: number; qty: number; total: number };

const field = newFieldMaker<Input, Output>();

const totalField = field
  .dependent("total")
  .default(0)
  .dependsOn(["price", "qty"])
  .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0));

const schema = new Schema<Input, Output>((b) =>
  b
    .field(field.constant("id").value(0))
    .field(field.lax("price").default(0))
    .field(field.lax("qty").default(0))
    .field(totalField),
);

const Model = schema.getModel();

describe("field builder prototype: dependent()", () => {
  it("produces a field definition the runtime accepts and resolves correctly", async () => {
    const { data, error } = await Model.create({ price: 10, qty: 3 }, {});

    expect(error).toBeNull();
    expect(data?.total).toBe(30);
  });

  it("should allow calling default()/dependsOn() in either order", async () => {
    const swappedOrderField = field
      .dependent("total")
      .dependsOn(["price", "qty"])
      .default(0)
      .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0));

    const swappedSchema = new Schema<Input, Output>((b) =>
      b
        .field(field.lax("price").default(0))
        .field(field.lax("qty").default(0))
        .field(swappedOrderField),
    );

    const { data, error } = await swappedSchema.getModel().create(
      {
        price: 4,
        qty: 5,
      },
      {},
    );

    expect(error).toBeNull();
    expect(data?.total).toBe(20);
  });

  it("should allow the optional readonly()/onDelete()/onSuccess() calls once buildable", async () => {
    let deleted = false;
    let succeeded = false;

    const decoratedField = field
      .dependent("total")
      .default(0)
      .dependsOn(["price", "qty"])
      .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0))
      .readonly()
      .onDelete(() => {
        deleted = true;
      })
      .onSuccess(() => {
        succeeded = true;
      });

    const decoratedSchema = new Schema<Input, Output>((b) =>
      b
        .field(field.lax("price").default(0))
        .field(field.lax("qty").default(0))
        .field(decoratedField),
    );

    const decoratedModel = decoratedSchema.getModel();
    const { data, handleSuccess } = await decoratedModel.create(
      {
        price: 2,
        qty: 6,
      },
      {},
    );

    if (!data) throw new Error("expected data to be present");

    await handleSuccess();
    await decoratedModel.delete(data, {});

    expect(succeeded).toBe(true);
    expect(deleted).toBe(true);
  });

  it("should accept an array of handlers in a single onDelete()/onSuccess()", async () => {
    const deletedBy: string[] = [];
    const succeededBy: string[] = [];

    const decoratedField = field
      .dependent("total")
      .default(0)
      .dependsOn(["price", "qty"])
      .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0))
      .onDelete([
        () => {
          deletedBy.push("first");
        },
        () => {
          deletedBy.push("second");
        },
      ])
      .onSuccess([
        () => {
          succeededBy.push("first");
        },
        () => {
          succeededBy.push("second");
        },
      ]);

    const decoratedSchema = new Schema<Input, Output>((b) =>
      b
        .field(field.lax("price").default(0))
        .field(field.lax("qty").default(0))
        .field(decoratedField),
    );

    const decoratedModel = decoratedSchema.getModel();
    const { data, handleSuccess } = await decoratedModel.create(
      {
        price: 3,
        qty: 4,
      },
      {},
    );

    if (!data) throw new Error("expected data to be present");

    await handleSuccess();
    await decoratedModel.delete(data, {});

    expect(succeededBy).toEqual(["first", "second"]);
    expect(deletedBy).toEqual(["first", "second"]);
  });

  describe("invalid usage (compile-time only - nothing here is meant to run)", () => {
    it("should never expose a callable .build()", () => {
      const builder = field.dependent("total");

      // @ts-expect-error - build() doesn't exist before resolve() has run
      builder.build?.();

      const finished = builder
        .default(0)
        .dependsOn(["price", "qty"])
        .resolve(() => 0);

      // @ts-expect-error - build() doesn't exist even on the finished builder; it's resolved internally by Schema only
      finished.build?.();
    });

    it("should reject calling resolve() before its preconditions are met", () => {
      const builder = field.dependent("total");

      // @ts-expect-error - resolve() isn't available until default() and dependsOn() have both been set
      builder.resolve(() => 0);

      const withDefaultOnly = builder.default(0);
      // @ts-expect-error - resolve() still isn't available; dependsOn() hasn't been set yet
      withDefaultOnly.resolve(() => 0);

      const readyToResolve = withDefaultOnly.dependsOn(["price", "qty"]);
      // @ts-expect-error - default() was already provided transitioning into readyToResolve's state; it's not offered again
      readyToResolve.default(0);
    });

    it("should reject a second call to readonly()/onDelete()/onSuccess() - each is single-call", () => {
      const finished = field
        .dependent("total")
        .default(0)
        .dependsOn(["price", "qty"])
        .resolve(() => 0);

      const decorated = finished
        .readonly()
        .onDelete(() => {})
        .onSuccess(() => {});

      // @ts-expect-error - readonly() was already provided
      decorated.readonly();

      // @ts-expect-error - onDelete() was already provided - pass an array instead of calling it again
      decorated.onDelete(() => {});

      // @ts-expect-error - onSuccess() was already provided - pass an array instead of calling it again
      decorated.onSuccess(() => {});
    });
  });
});
