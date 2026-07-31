import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { createFieldBuilder } from '../../src/schema/fields';

/**
 * End-to-end prototype of the Rust-style typestate builder for "required"
 * fields (see src/schema/fields/required.ts). Unlike lax fields, validation
 * is mandatory here - `[BUILD]` (and therefore dropping the chain into a
 * `Definitions` object literal) only becomes available once either
 * `.allow()` or `.validate()` has been called, still mutually exclusive with
 * one another. `.readonly()`/`.ignoreUpdate()` share a single flag - like
 * Rust, calling either consumes both, since a required property is either
 * readonly or conditionally updatable, never both.
 */

type Input = {
  email: string;
  role: string;
  plan: string;
  score: number;
};
type Output = Input;

const field = createFieldBuilder<Input, Output>();

describe('field builder prototype: required()', () => {
  it('supports validate() as the primary validator', async () => {
    const schema = new Schema<Input, Output>({
      email: field
        .required('email')
        .validate((value) =>
          typeof value === 'string' && value.includes('@')
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid email' },
        ),
      role: field.required('role').allow(['admin', 'member']),
      plan: field.required('plan').allow(['free', 'pro']),
      score: field
        .required('score')
        .validate((value) =>
          typeof value === 'number'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid score' },
        ),
    });

    const Model = schema.getModel();

    const missing = await Model.create({}, {});
    expect(missing.error).toMatchObject({
      email: expect.objectContaining({ reason: "'email' is required" }),
    });

    const rejected = await Model.create(
      { email: 'not-an-email', role: 'admin', plan: 'free', score: 1 },
      {},
    );
    expect(rejected.error).toMatchObject({
      email: expect.objectContaining({ reason: 'invalid email' }),
    });

    const accepted = await Model.create(
      { email: 'ada@ivo.dev', role: 'admin', plan: 'free', score: 1 },
      {},
    );
    expect(accepted.error).toBeNull();
    expect(accepted.data?.email).toBe('ada@ivo.dev');
  });

  it('supports allow() as the primary validator, rejecting values outside the list', async () => {
    const schema = new Schema<Input, Output>({
      email: field
        .required('email')
        .validate((value) =>
          typeof value === 'string'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid email' },
        ),
      role: field.required('role').allow(['admin', 'member']),
      plan: field.required('plan').allow(['free', 'pro']),
      score: field
        .required('score')
        .validate((value) =>
          typeof value === 'number'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid score' },
        ),
    });

    const Model = schema.getModel();

    const rejected = await Model.create(
      { email: 'ada@ivo.dev', role: 'owner', plan: 'free', score: 1 },
      {},
    );
    expect(rejected.error).toMatchObject({
      role: expect.objectContaining({ reason: 'value not allowed' }),
    });

    const accepted = await Model.create(
      { email: 'ada@ivo.dev', role: 'member', plan: 'free', score: 1 },
      {},
    );
    expect(accepted.error).toBeNull();
    expect(accepted.data?.role).toBe('member');
  });

  it('supports allow().allowError() to customize the rejection message', async () => {
    const schema = new Schema<Input, Output>({
      email: field
        .required('email')
        .validate((value) =>
          typeof value === 'string'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid email' },
        ),
      role: field
        .required('role')
        .allow(['admin', 'member'])
        .allowError('role must be admin or member'),
      plan: field.required('plan').allow(['free', 'pro']),
      score: field
        .required('score')
        .validate((value) =>
          typeof value === 'number'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid score' },
        ),
    });

    const { data, error } = await schema
      .getModel()
      .create(
        { email: 'ada@ivo.dev', role: 'owner', plan: 'free', score: 1 },
        {},
      );

    expect(data).toBeNull();
    expect(error).toMatchObject({
      role: expect.objectContaining({ reason: 'role must be admin or member' }),
    });
  });

  it('supports validate().reValidate() and allow().reValidate()', async () => {
    const schema = new Schema<Input, Output>({
      email: field
        .required('email')
        .validate((value) =>
          typeof value === 'string'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid email' },
        ),
      role: field.required('role').allow(['admin', 'member']),
      plan: field
        .required('plan')
        .allow(['free', 'pro', 'enterprise'])
        .reValidate((value) =>
          value !== 'enterprise'
            ? { valid: true, validated: value }
            : {
                valid: false,
                reason: 'enterprise plan requires sales approval',
              },
        ),
      score: field
        .required('score')
        .validate((value) =>
          typeof value === 'number'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid score' },
        )
        .reValidate((value) =>
          value >= 0
            ? { valid: true, validated: value }
            : { valid: false, reason: 'score must be non-negative' },
        ),
    });

    const Model = schema.getModel();

    const rejectedByAllowSecondary = await Model.create(
      { email: 'ada@ivo.dev', role: 'admin', plan: 'enterprise', score: 1 },
      {},
    );
    expect(rejectedByAllowSecondary.error).toMatchObject({
      plan: expect.objectContaining({
        reason: 'enterprise plan requires sales approval',
      }),
    });

    const rejectedByValidateSecondary = await Model.create(
      { email: 'ada@ivo.dev', role: 'admin', plan: 'free', score: -1 },
      {},
    );
    expect(rejectedByValidateSecondary.error).toMatchObject({
      score: expect.objectContaining({ reason: 'score must be non-negative' }),
    });

    const accepted = await Model.create(
      { email: 'ada@ivo.dev', role: 'admin', plan: 'free', score: 5 },
      {},
    );
    expect(accepted.error).toBeNull();
  });

  it('supports readonly()/ignoreUpdate() (mutually exclusive) and onDelete()/onFailure()/onSuccess()', async () => {
    let deleted = false;
    let succeeded = false;

    const schema = new Schema<Input, Output>({
      email: field
        .required('email')
        .validate((value) =>
          typeof value === 'string'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid email' },
        )
        .readonly()
        .onDelete(() => {
          deleted = true;
        })
        .onSuccess(() => {
          succeeded = true;
        }),
      role: field
        .required('role')
        .allow(['admin', 'member'])
        // .readonly()
        .ignoreUpdate(() => false),
      plan: field.required('plan').allow(['free', 'pro']),
      score: field
        .required('score')
        .validate((value) =>
          typeof value === 'number'
            ? { valid: true, validated: value }
            : { valid: false, reason: 'invalid score' },
        ),
    });

    const Model = schema.getModel();

    const { data, handleSuccess } = await Model.create(
      { email: 'ada@ivo.dev', role: 'admin', plan: 'free', score: 1 },
      {},
    );
    if (!data) throw new Error('expected data to be present');

    await handleSuccess();
    await Model.delete(data, {});

    expect(succeeded).toBe(true);
    expect(deleted).toBe(true);

    const updated = await Model.update(data, { email: 'other@ivo.dev' }, {});
    expect(updated.data).toBeNull();
  });

  describe('invalid usage (compile-time only - nothing here is meant to run)', () => {
    it('rejects calling [BUILD] before allow() or validate()', () => {
      const builder = field.required('role');

      // @ts-expect-error - build() doesn't exist until either allow() or validate() has been called
      builder.build?.();
    });

    it('makes allow() and validate() mutually exclusive', () => {
      const withAllow = field.required('role').allow(['admin', 'member']);
      // @ts-expect-error - validate() isn't available once allow() has been chosen as the primary validator
      withAllow.validate?.(() => true);

      const withValidator = field.required('email').validate(() => true);
      // @ts-expect-error - allow() isn't available once validate() has been chosen as the primary validator
      withValidator.allow?.(['a', 'b']);
    });

    it('rejects allowError() before allow()', () => {
      const withValidator = field.required('email').validate(() => true);

      // @ts-expect-error - allowError() only becomes available once allow() has been called
      withValidator.allowError?.('nope');
    });

    it('never exposes a callable .build(), at any stage', () => {
      const validated = field.required('email').validate(() => true);

      // @ts-expect-error - build() doesn't exist; it's resolved internally by Schema only
      validated.build?.();
    });

    it('makes readonly() and ignoreUpdate() share a single flag', () => {
      const decorated = field
        .required('role')
        .allow(['admin', 'member'])
        .readonly();

      // @ts-expect-error - readonly() was already consumed
      decorated.readonly?.();
      // @ts-expect-error - ignoreUpdate() shares readonly()'s flag - also consumed
      decorated.ignoreUpdate?.();
    });

    it('rejects a second call to allowError()/reValidate()', () => {
      const decorated = field
        .required('role')
        .allow(['admin', 'member'])
        .allowError('nope')
        .reValidate(() => true);

      // @ts-expect-error - allowError() was already consumed
      decorated.allowError?.('nope again');
      // @ts-expect-error - reValidate() was already consumed
      decorated.reValidate?.(() => true);
    });
  });
});
