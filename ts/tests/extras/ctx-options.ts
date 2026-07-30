import { describe, expect, it } from 'bun:test';
import type { IvoContext, ReadonlyIvoContext, Schema } from '../../src';

/**
 * Mirrors `rs/tests/extras/ctx_options/*.rs`: a user-defined, mutable
 * `CtxOptions` object is threaded through the whole create/update pipeline
 * via `ctx.updateOptions(partial)`, and later hooks (resolvers, validators,
 * onSuccess) observe the accumulated state through `ctx.options`.
 *
 * Rust achieves this with an `RwLock`-guarded `IvoCtxOptions`; the TS
 * equivalent is the explicit `updateOptions` function present on the
 * mutable `IvoContext` (validators, resolvers, sanitizers, ignore/ignoreUpdate,
 * post-validators). Note: field-level `required` handlers are typed to
 * receive a full `IvoContext` (with `updateOptions`), but at runtime receive
 * a read-only context without it — calling `ctx.updateOptions` inside a
 * `required` handler throws, which (per the model's general "errors inside
 * handlers are swallowed") silently makes that evaluation resolve to
 * "not required" rather than propagating. That gap is exercised explicitly
 * below rather than assumed.
 */
export const Test_CtxOptions = ({
  Schema: SchemaClass,
}: {
  Schema: typeof Schema;
}) => {
  describe('ctx options threading', () => {
    type Input = { name?: string };
    type Output = { name: string; upper: string };
    type CtxOpts = { log: string[] };

    function push(entry: string) {
      return (ctx: IvoContext<Input, Output, CtxOpts>) =>
        ctx.updateOptions({ log: [...ctx.options.log, entry] });
    }

    const Model = new SchemaClass<Input, Output, CtxOpts>(
      {
        name: {
          default: '',
          validator: (v: unknown, ctx: IvoContext<Input, Output, CtxOpts>) => {
            push('validator')(ctx);
            return { valid: true, validated: v as string };
          },
        },
        upper: {
          default: '',
          dependsOn: 'name',
          resolver: (ctx: IvoContext<Input, Output, CtxOpts>) => {
            push('resolver')(ctx);
            return String(ctx.values.name ?? '').toUpperCase();
          },
        },
      },
      {
        onSuccess: (ctx: ReadonlyIvoContext<Input, Output, CtxOpts>) => {
          seenLogAtSuccess = ctx.options.log;
        },
      },
    ).getModel();

    let seenLogAtSuccess: string[] = [];

    it('accumulates writes from validator then resolver, visible to onSuccess', async () => {
      seenLogAtSuccess = [];

      const { data, handleSuccess } = await Model.create(
        { name: 'bob' },
        { log: [] },
      );

      expect(data).toEqual({ name: 'bob', upper: 'BOB' });

      await handleSuccess?.();

      expect(seenLogAtSuccess).toEqual(['validator', 'resolver']);
    });

    it('does not mutate the ctxOptions object instance the caller originally passed in', async () => {
      const original: CtxOpts = { log: [] };

      await Model.create({ name: 'sue' }, original);

      // the model works off an internal copy; the caller's own reference is untouched
      expect(original.log).toEqual([]);
    });

    it('re-threads a fresh accumulation per call (no leakage across create() calls)', async () => {
      seenLogAtSuccess = [];
      const first = await Model.create({ name: 'a' }, { log: ['seed'] });
      await first.handleSuccess?.();
      expect(seenLogAtSuccess).toEqual(['seed', 'validator', 'resolver']);

      seenLogAtSuccess = [];
      const second = await Model.create({ name: 'b' }, { log: [] });
      await second.handleSuccess?.();
      expect(seenLogAtSuccess).toEqual(['validator', 'resolver']);
    });

    it('threads ctx options through updates too', async () => {
      seenLogAtSuccess = [];

      const { data, handleSuccess } = await Model.update(
        { name: 'bob', upper: 'BOB' },
        { name: 'rob' },
        { log: [] },
      );

      expect(data).toEqual({ name: 'rob', upper: 'ROB' });

      await handleSuccess?.();

      expect(seenLogAtSuccess).toEqual(['validator', 'resolver']);
    });

    it('calling ctx.updateOptions inside a "required" handler throws and is swallowed as not-required', async () => {
      type ReqInput = { a?: number };
      type ReqOutput = { a: number };

      const ReqModel = new SchemaClass<ReqInput, ReqOutput, CtxOpts>({
        a: {
          default: 0,
          required(ctx: IvoContext<ReqInput, ReqOutput, CtxOpts>) {
            ctx.updateOptions({ log: [...ctx.options.log, 'should-not-run'] });
            return false;
          },
        },
      }).getModel();

      const { data, error, options } = await ReqModel.create({}, { log: [] });

      // the thrown error inside the handler is swallowed; field is treated as not required
      expect(error).toBeNull();
      expect(data).toEqual({ a: 0 });
      expect(options.log).toEqual(['should-not-run']);
    });
  });
};
