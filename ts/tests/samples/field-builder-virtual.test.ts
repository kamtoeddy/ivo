import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { newFieldMaker } from '../../src/schema/fields';

/**
 * End-to-end prototype of the Rust-style typestate builder for "virtual"
 * fields (see src/schema/fields/virtual.ts). A virtual field's `validator`
 * is mandatory at runtime, so - unlike lax - every rule except `.alias()`
 * only unlocks once either `.allow()` or `.validate()` has been called
 * (mutually exclusive, same rule as lax/required); `.allow()`-only still
 * needs a real `validator` to exist, so the builder synthesizes a
 * passthrough primary validator in that case, same as lax/required.
 *
 * `dependsOn()` can only reference `keyof Input | keyof Output` (a pre-
 * existing, unrelated limitation - see the `dependent` builder), so these
 * tests keep every virtual's own definition key equal to a real `keyof
 * Input` key, and cover `.alias()` in isolation instead of wiring it to a
 * dependent.
 */

type Input = {
  rawGrade: unknown;
  rawStatus: unknown;
};
type Output = {
  grade: number;
  status: string;
};

const field = newFieldMaker<Input, Output>();
const dependent = newFieldMaker<Input, Output>();

const gradeDependent = () =>
  dependent
    .dependent('grade')
    .default(0)
    .dependsOn('rawGrade')
    .resolve(({ input }) => (input.rawGrade as number) ?? 0);

const statusDependent = () =>
  dependent
    .dependent('status')
    .default('unknown')
    .dependsOn('rawStatus')
    .resolve(({ input }) => (input.rawStatus as string) ?? 'unknown');

describe('field builder prototype: virtual()', () => {
  it('supports validate() as the primary validator, feeding a dependent field', async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .virtual('rawGrade')
            .validate((value) =>
              typeof value === 'number'
                ? { valid: true, validated: value }
                : { valid: false, reason: 'grade must be a number' },
            ),
        )
        .field(field.virtual('rawStatus').allow(['ok', 'fail']))
        .field(gradeDependent())
        .field(statusDependent()),
    );

    const Model = schema.getModel();

    const rejected = await Model.create(
      { rawGrade: 'lol', rawStatus: 'ok' },
      {},
    );
    expect(rejected.error).toMatchObject({
      rawGrade: expect.objectContaining({ reason: 'grade must be a number' }),
    });

    const accepted = await Model.create({ rawGrade: 87, rawStatus: 'ok' }, {});
    expect(accepted.error).toBeNull();
    expect(accepted.data?.grade).toBe(87);
  });

  it('supports allow() as the primary validator, rejecting values outside the list', async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .virtual('rawGrade')
            .validate((value) =>
              typeof value === 'number'
                ? { valid: true, validated: value }
                : { valid: false, reason: 'grade must be a number' },
            ),
        )
        .field(field.virtual('rawStatus').allow(['ok', 'fail']))
        .field(gradeDependent())
        .field(statusDependent()),
    );

    const { data, error } = await schema
      .getModel()
      .create({ rawGrade: 87, rawStatus: 'unknown' }, {});

    expect(data).toBeNull();
    expect(error).toMatchObject({
      rawStatus: expect.objectContaining({ reason: 'value not allowed' }),
    });
  });

  it('supports allow().allowError() to customize the rejection message', async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(field.virtual('rawGrade').validate(() => true))
        .field(
          field
            .virtual('rawStatus')
            .allow(['ok', 'fail'])
            .allowError('status must be ok or fail'),
        )
        .field(gradeDependent())
        .field(statusDependent()),
    );

    const { data, error } = await schema
      .getModel()
      .create({ rawGrade: 1, rawStatus: 'unknown' }, {});

    expect(data).toBeNull();
    expect(error).toMatchObject({
      rawStatus: expect.objectContaining({
        reason: 'status must be ok or fail',
      }),
    });
  });

  it('supports validate().reValidate() and allow().reValidate()', async () => {
    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .virtual('rawGrade')
            .validate((value) =>
              typeof value === 'number'
                ? { valid: true, validated: value }
                : { valid: false, reason: 'grade must be a number' },
            )
            .reValidate((value) =>
              (value as number) <= 100
                ? { valid: true, validated: value }
                : { valid: false, reason: 'grade must be at most 100' },
            ),
        )
        .field(
          field
            .virtual('rawStatus')
            .allow(['ok', 'fail', 'unknown'])
            .reValidate((value) =>
              value !== 'unknown'
                ? { valid: true, validated: value }
                : { valid: false, reason: 'status cannot be unknown' },
            ),
        )
        .field(gradeDependent())
        .field(statusDependent()),
    );

    const Model = schema.getModel();

    const rejectedByValidateSecondary = await Model.create(
      { rawGrade: 150, rawStatus: 'ok' },
      {},
    );
    expect(rejectedByValidateSecondary.error).toMatchObject({
      rawGrade: expect.objectContaining({
        reason: 'grade must be at most 100',
      }),
    });

    const rejectedByAllowSecondary = await Model.create(
      { rawGrade: 87, rawStatus: 'unknown' },
      {},
    );
    expect(rejectedByAllowSecondary.error).toMatchObject({
      rawStatus: expect.objectContaining({
        reason: 'status cannot be unknown',
      }),
    });

    const accepted = await Model.create({ rawGrade: 87, rawStatus: 'ok' }, {});
    expect(accepted.error).toBeNull();
  });

  it('supports required()/sanitize()/ignoreInit()/ignoreUpdate()/onFailure()/onSuccess()', async () => {
    let failed = false;
    let succeeded = false;

    const schema = new Schema<Input, Output>((b) =>
      b
        .field(
          field
            .virtual('rawGrade')
            .validate((value) =>
              typeof value === 'number'
                ? { valid: true, validated: value }
                : { valid: false, reason: 'grade must be a number' },
            )
            .required(() => true)
            .sanitize(({ values }) => Math.round(values.grade ?? 0))
            .onFailure(() => {
              failed = true;
            })
            .onSuccess(() => {
              succeeded = true;
            }),
        )
        .field(field.virtual('rawStatus').allow(['ok', 'fail']).ignoreUpdate())
        .field(gradeDependent())
        .field(statusDependent()),
    );

    const Model = schema.getModel();

    const missing = await Model.create({ rawStatus: 'ok' }, {});
    expect(missing.error).toMatchObject({
      rawGrade: expect.objectContaining({ reason: "'rawGrade' is required" }),
    });

    // onFailure only cleans up fields actually present in the raw input, and
    // must be invoked explicitly by the caller, so trigger it with an
    // invalid (rather than missing) value.
    const invalid = await Model.create(
      { rawGrade: 'lol', rawStatus: 'ok' },
      {},
    );
    expect(invalid.error).toMatchObject({
      rawGrade: expect.objectContaining({ reason: 'grade must be a number' }),
    });
    if (!invalid.handleFailure)
      throw new Error('expected handleFailure to be present');
    await invalid.handleFailure();
    expect(failed).toBe(true);

    const { error, handleSuccess } = await Model.create(
      { rawGrade: 87, rawStatus: 'ok' },
      {},
    );
    expect(error).toBeNull();
    if (!handleSuccess) throw new Error('expected handleSuccess to be present');
    await handleSuccess();
    expect(succeeded).toBe(true);
  });

  it('supports alias() to accept input under a different public name', async () => {
    type AliasInput = { grade: number };
    type AliasOutput = { result: number };

    const aliasField = newFieldMaker<AliasInput, AliasOutput>();
    const aliasDependent = newFieldMaker<AliasInput, AliasOutput>();

    const schema = new Schema<AliasInput, AliasOutput>((b) =>
      b
        .field(
          aliasField
            .virtual('g')
            .alias('grade')
            .validate((value) =>
              typeof value === 'number'
                ? { valid: true, validated: value }
                : { valid: false, reason: 'grade must be a number' },
            ),
        )
        // `dependsOn()` can only reference `keyof Input | keyof Output` (see
        // the top-of-file note) - 'g' is the virtual's own arbitrary
        // definition key, not a real input/output key, so referencing it
        // here needs a cast, same as the pre-existing alias/dependsOn
        // limitation.
        .field(
          aliasDependent
            .dependent('result')
            .default(0)
            .dependsOn('g' as never)
            .resolve(({ input }) => input.grade ?? 0),
        ),
    );

    const Model = schema.getModel();

    // @ts-expect-error ikr
    const rejected = await Model.create({ grade: 'lol' }, {});
    expect(rejected.error).toMatchObject({
      grade: expect.objectContaining({ reason: 'grade must be a number' }),
    });

    const accepted = await Model.create({ grade: 87 }, {});
    expect(accepted.error).toBeNull();
    expect(accepted.data?.result).toBe(87);
  });

  describe('invalid usage (compile-time only - nothing here is meant to run)', () => {
    it('rejects calling [BUILD] and validation-gated rules before allow() or validate()', () => {
      const builder = field.virtual('rawGrade');

      // @ts-expect-error - build() doesn't exist until either allow() or validate() has been called
      builder.build?.();
      // @ts-expect-error - required() isn't available before allow()/validate()
      builder.required?.(() => true);
      // @ts-expect-error - sanitize() isn't available before allow()/validate()
      builder.sanitize?.(() => 0);
    });

    it('makes allow() and validate() mutually exclusive', () => {
      const withAllow = field.virtual('rawStatus').allow(['ok', 'fail']);
      // @ts-expect-error - validate() isn't available once allow() has been chosen as the primary validator
      withAllow.validate?.(() => true);

      const withValidator = field.virtual('rawGrade').validate(() => true);
      // @ts-expect-error - allow() isn't available once validate() has been chosen as the primary validator
      withValidator.allow?.(['a', 'b']);
    });

    it('rejects allowError() before allow()', () => {
      const withValidator = field.virtual('rawGrade').validate(() => true);

      // @ts-expect-error - allowError() only becomes available once allow() has been called
      withValidator.allowError?.('nope');
    });

    it('never exposes a callable .build(), at any stage', () => {
      const validated = field.virtual('rawGrade').validate(() => true);

      // @ts-expect-error - build() doesn't exist; it's resolved internally by Schema only
      validated.build?.();
    });

    it('rejects a second call to allowError()/reValidate()/required()/sanitize()', () => {
      const decorated = field
        .virtual('rawStatus')
        .allow(['ok', 'fail'])
        .allowError('nope')
        .reValidate(() => true)
        .required(() => true)
        .sanitize(() => 0);

      // @ts-expect-error - allowError() was already consumed
      decorated.allowError?.('nope again');
      // @ts-expect-error - reValidate() was already consumed
      decorated.reValidate?.(() => true);
      // @ts-expect-error - required() was already consumed
      decorated.required?.(() => true);
      // @ts-expect-error - sanitize() was already consumed
      decorated.sanitize?.(() => 0);
    });
  });
});
