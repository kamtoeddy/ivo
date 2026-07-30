import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { createFieldBuilder } from '../../src/schema/fields';

/**
 * End-to-end prototype of the Rust-style typestate builder for "dependent"
 * fields (see src/schema/field-builder.ts). Proves three things:
 *  1. runtime: a builder chain's result is a real, working field definition
 *     once dropped into a Definitions object literal - no `.build()` call
 *     required or even possible; `Schema` resolves it internally.
 *  2. compile-time: skipping/reordering required steps is a type error, not
 *     a runtime footgun - see the "invalid usage" block below.
 *  3. there is no user-reachable `.build()`, at any stage - not before
 *     `resolve()`, and not even after it.
 *  4. `readonly()`/`onDelete()`/`onSuccess()` are each callable exactly
 *     once (unlike Rust, where attaching several `on_delete`/`on_success`
 *     handlers means calling the method repeatedly) - pass an array to
 *     attach several handlers in that one call instead.
 */

type Input = { price: number; qty: number };
type Output = { id: number; price: number; qty: number; total: number };

const field = createFieldBuilder<Input, Output>();

const totalField = field
  .dependent('total')
  .default(0)
  .dependsOn(['price', 'qty'])
  .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0));

const schema = new Schema<Input, Output>({
  id: field.constant('id').value(0),
  price: { default: 0 },
  qty: { default: 0 },
  total: totalField,
});

const Model = schema.getModel();

describe('field builder prototype: dependent()', () => {
  it('produces a field definition the runtime accepts and resolves correctly', async () => {
    const { data, error } = await Model.create({ price: 10, qty: 3 }, {});

    expect(error).toBeNull();
    expect(data?.total).toBe(30);
  });

  it('supports calling default()/dependsOn() in either order', async () => {
    const swappedOrderField = field
      .dependent('total')
      .dependsOn(['price', 'qty'])
      .default(0)
      .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0));

    const swappedSchema = new Schema<Input, Output>({
      price: { default: 0 },
      qty: { default: 0 },
      total: swappedOrderField,
    });

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

  it('supports the optional readonly()/onDelete()/onSuccess() calls once buildable', async () => {
    let deleted = false;
    let succeeded = false;

    const decoratedField = field
      .dependent('total')
      .default(0)
      .dependsOn(['price', 'qty'])
      .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0))
      .readonly()
      .onDelete(() => {
        deleted = true;
      })
      .onSuccess(() => {
        succeeded = true;
      });

    const decoratedSchema = new Schema<Input, Output>({
      price: { default: 0 },
      qty: { default: 0 },
      total: decoratedField,
    });

    const decoratedModel = decoratedSchema.getModel();
    const { data, handleSuccess } = await decoratedModel.create(
      {
        price: 2,
        qty: 6,
      },
      {},
    );

    if (!data) throw new Error('expected data to be present');

    await handleSuccess();
    await decoratedModel.delete(data, {});

    expect(succeeded).toBe(true);
    expect(deleted).toBe(true);
  });

  it("accepts an array of handlers in a single onDelete()/onSuccess() call (unlike Rust's repeated calls)", async () => {
    const deletedBy: string[] = [];
    const succeededBy: string[] = [];

    const decoratedField = field
      .dependent('total')
      .default(0)
      .dependsOn(['price', 'qty'])
      .resolve(({ values }) => (values.price ?? 0) * (values.qty ?? 0))
      .onDelete([
        () => {
          deletedBy.push('first');
        },
        () => {
          deletedBy.push('second');
        },
      ])
      .onSuccess([
        () => {
          succeededBy.push('first');
        },
        () => {
          succeededBy.push('second');
        },
      ]);

    const decoratedSchema = new Schema<Input, Output>({
      price: { default: 0 },
      qty: { default: 0 },
      total: decoratedField,
    });

    const decoratedModel = decoratedSchema.getModel();
    const { data, handleSuccess } = await decoratedModel.create(
      {
        price: 3,
        qty: 4,
      },
      {},
    );

    if (!data) throw new Error('expected data to be present');

    await handleSuccess();
    await decoratedModel.delete(data, {});

    expect(succeededBy).toEqual(['first', 'second']);
    expect(deletedBy).toEqual(['first', 'second']);
  });

  describe('invalid usage (compile-time only - nothing here is meant to run)', () => {
    it('rejects calling resolve() before its preconditions are met', () => {
      const builder = field.dependent('total');

      // @ts-expect-error - resolve() isn't available until default() and dependsOn() have both been set
      builder.resolve(() => 0);

      const withDefaultOnly = builder.default(0);
      // @ts-expect-error - resolve() still isn't available; dependsOn() hasn't been set yet
      withDefaultOnly.resolve(() => 0);

      const readyToResolve = withDefaultOnly.dependsOn(['price', 'qty']);
      // @ts-expect-error - default() was already consumed transitioning into readyToResolve's state; it's not offered again
      readyToResolve.default(0);
    });

    it('never exposes a callable .build(), at any stage', () => {
      const builder = field.dependent('total');

      // @ts-expect-error - build() doesn't exist before resolve() has run
      builder.build?.();

      const finished = builder
        .default(0)
        .dependsOn(['price', 'qty'])
        .resolve(() => 0);

      // @ts-expect-error - build() doesn't exist even on the finished builder; it's resolved internally by Schema only
      finished.build?.();
    });

    it("rejects a second call to readonly()/onDelete()/onSuccess() - each is single-call, unlike Rust's repeatable calls", () => {
      const finished = field
        .dependent('total')
        .default(0)
        .dependsOn(['price', 'qty'])
        .resolve(() => 0);

      const decorated = finished
        .readonly()
        .onDelete(() => {})
        .onSuccess(() => {});

      // @ts-expect-error - readonly() was already consumed
      decorated.readonly();
      // @ts-expect-error - onDelete() was already consumed - pass an array instead of calling it again
      decorated.onDelete(() => {});
      // @ts-expect-error - onSuccess() was already consumed - pass an array instead of calling it again
      decorated.onSuccess(() => {});
    });
  });
});
