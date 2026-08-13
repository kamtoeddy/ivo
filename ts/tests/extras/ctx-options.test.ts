import { describe, expect, it } from 'bun:test';
import { type IvoContext, Schema } from '../../src';

describe('ctx options threading', () => {
  type Input = { name?: string };
  type Output = { name: string; upper: string };
  type CtxOpts = { log: string[] };

  function push(entry: string) {
    return (ctx: IvoContext<Input, Output, CtxOpts>) =>
      ctx.updateOptions({ log: [...ctx.options.log, entry] });
  }

  const Model = new Schema<Input, Output, CtxOpts>(
    (b) =>
      b
        .field(
          b.lax('name', '').validate((v, ctx) => {
            push('validator')(ctx);
            ctx.options.log = ['lol'];
            return { valid: true, validated: v as string };
          }),
        )
        .field(
          b
            .dependent('upper', 'name')
            .default('')
            .resolve((ctx) => {
              push('resolver')(ctx);
              return String(ctx.values.name ?? '').toUpperCase();
            }),
        ),
    {
      onSuccess(ctx) {
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

    await Model.update(
      { name: 'sue', upper: 'SUE' },
      { name: 'sue-update' },
      original,
    );

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

  it('calling ctx.updateOptions inside a "required" handler throws and is swallowed as not-required', async () => {
    type Input = { a: number };
    const Model = new Schema<Input, Input, CtxOpts>((b) =>
      b.field(
        b.lax('a', 0).required((ctx) => {
          ctx.updateOptions({ log: [...ctx.options.log, 'should-not-run'] });
          return false;
        }),
      ),
    ).getModel();

    const original = { log: [] };

    const { data, error, options } = await Model.create({}, original);

    // the thrown error inside the handler is swallowed; field is treated as not required
    expect(error).toBeNull();
    expect(data).toEqual({ a: 0 });
    expect(original).toEqual({ log: [] });
    expect(options.log).toEqual(['should-not-run']);
  });

  describe('ctx options as plain object with methods', () => {
    type Input = { a: number };
    type CtxOpts = {
      logs: string[];
      add(msg: string): void;
      setExternalMsg(msg: string): void;
      getExternalMsg(): string;
    };

    const messageFromCtxOptionSetExternalMsg =
      'set from ctx option.setExternalMsg';
    const messageIvoUpdateOptionsAPI = 'set from ivo updateOptions API';

    it('ctx options as plain object with methods', async () => {
      let externalMsg = '';

      const Model = new Schema<Input, Input, CtxOpts>((b) =>
        b.field(
          b.lax('a', 0).required((ctx) => {
            const options = ctx.options;
            options.add('should not be set on ctx options');
            options.setExternalMsg(messageFromCtxOptionSetExternalMsg);
            expect(options.logs).toEqual([
              formatLogEntry('should not be set on ctx options'),
              formatLogEntry(
                `set externalMsg to ${messageFromCtxOptionSetExternalMsg}`,
              ),
            ]);
            ctx.updateOptions({
              logs: [...ctx.options.logs, messageIvoUpdateOptionsAPI],
            });

            return false;
          }),
        ),
      ).getModel();

      function formatLogEntry(entry: string) {
        return `[log]: ${entry}`;
      }

      const o1: CtxOpts = {
        logs: [],
        add(entry) {
          this.logs.push(formatLogEntry(entry));
        },
        setExternalMsg(msg: string) {
          externalMsg = msg;
          this.logs.push(formatLogEntry(`set externalMsg to ${externalMsg}`));
        },
        getExternalMsg() {
          return externalMsg;
        },
      };

      let newExternalMsg = 'hello, tony';
      o1.setExternalMsg(newExternalMsg);
      expect(externalMsg).toBe(newExternalMsg);
      expect(o1.logs).toEqual([
        formatLogEntry(`set externalMsg to ${newExternalMsg}`),
      ]);

      const msg = 'added via add method';
      o1.add(msg);
      expect(o1.logs).toEqual([
        formatLogEntry(`set externalMsg to ${newExternalMsg}`),
        formatLogEntry(msg),
      ]);

      externalMsg = '';

      const original: CtxOpts = {
        logs: [],
        add(entry) {
          this.logs.push(formatLogEntry(entry));
        },
        setExternalMsg(msg: string) {
          externalMsg = msg;
          this.logs.push(formatLogEntry(`set externalMsg to ${externalMsg}`));
        },
        getExternalMsg() {
          return externalMsg;
        },
      };

      const { data, error, options } = await Model.create({}, original);

      expect(externalMsg).toBe(messageFromCtxOptionSetExternalMsg);

      expect(options.getExternalMsg()).toEqual(
        messageFromCtxOptionSetExternalMsg,
      );

      expect(error).toBeNull();
      expect(data).toEqual({ a: 0 });
      expect(original.logs).toEqual([]);
      expect(options.logs).toEqual([messageIvoUpdateOptionsAPI]);

      externalMsg = '';

      newExternalMsg = 'hello, tony';
      options.setExternalMsg(newExternalMsg);
      expect(externalMsg).toBe(newExternalMsg);
      console.log(options);
      expect(
        options.logs.includes(
          formatLogEntry(`set externalMsg to ${newExternalMsg}`),
        ),
      ).toBe(true);
      expect(options.logs).toEqual([
        messageIvoUpdateOptionsAPI,
        formatLogEntry(`set externalMsg to ${newExternalMsg}`),
      ]);
    });
  });
});
