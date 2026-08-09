import { describe, it } from 'bun:test';
import {
  expectFailure,
  expectNoFailure,
  makeFx,
  validator,
} from '../../_utils';

describe('field configs.dependent', () => {
  it('should reject if parent array is empty', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', [] as never)
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('required').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject dependency of createdAt field with default name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1))
          .field(
            b
              .dependent('dependent', ['lax', 'required', 'createdAt'])
              .default(2)
              .resolve(() => 4),
          )
          .field(b.required('required').validate(validator)),
      { timestamps: { createdAt: true } },
    );

    expectFailure(toFail);
  });

  it('should reject dependency of createdAt field with custom name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1))
          .field(
            b
              .dependent('dependent', ['lax', 'required', 'customCreatedAt'])
              .default(2)
              .resolve(() => 4),
          )
          .field(b.required('required').validate(validator)),
      { timestamps: { createdAt: 'customCreatedAt' } },
    );

    expectFailure(toFail);
  });

  it('should reject dependency of updatedAt field with default name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1))
          .field(
            b
              .dependent('dependent', ['lax', 'required', 'updatedAt'])
              .default(2)
              .resolve(() => 4),
          )
          .field(b.required('required').validate(validator)),
      { timestamps: { updatedAt: true } },
    );

    expectFailure(toFail);
  });

  it('should reject dependency of updatedAt field with custom name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1))
          .field(
            b
              .dependent('dependent', ['lax', 'required', 'customUpdatedAt'])
              .default(2)
              .resolve(() => 4),
          )
          .field(b.required('required').validate(validator)),
      { timestamps: { updatedAt: { key: 'customUpdatedAt' } } },
    );

    expectFailure(toFail);
  });

  it('should reject if any parent field provided does not belong on schema', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', ['lax', 'required', 'lol'])
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('required').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject if any parent field name is same as dependent field name', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', ['lax', 'required', 'dependent'])
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('required').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject if duplicate parent fields are provided', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', ['lax', 'required', 'lax'])
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('required').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject dependency of constant fields', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', ['lax', 'required', 'id'])
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('required').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject any redundant dependencies', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.lax('c', 1))
        .field(
          b
            .dependent('b', 'c')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('a', ['c', 'd', 'b'])
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('d').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject any deeply redundant dependencies', () => {
    const toFail = makeFx((b) =>
      b
        .field(
          b
            .dependent('c', 'd')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('b', 'c')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('a', ['b', 'd'])
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('d').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject any circular dependencies', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.lax('c', 1))
        .field(
          b
            .dependent('a', 'b')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('b', ['a', 'c'])
            .default(2)
            .resolve(() => 4),
        ),
    );

    expectFailure(toFail);
  });

  it('should reject any deeply circular dependencies', () => {
    const toFail = makeFx((b) =>
      b
        .field(
          b
            .dependent('a', 'b')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('c', ['a', 'd'])
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('b', 'c')
            .default(2)
            .resolve(() => 4),
        )
        .field(b.lax('d', 1)),
    );

    expectFailure(toFail);
  });

  it('should allow dependency on normal lax or required fields', () => {
    for (const dependsOn of ['lax', 'required', ['lax', 'required'] as const]) {
      const toPass = makeFx(
        (b) =>
          b
            .field(b.lax('lax', 1))
            .field(
              b
                .dependent('dependent', dependsOn as never)
                .default(2)
                .resolve(() => 4),
            )
            .field(b.required('required').validate(validator)),
        { timestamps: { updatedAt: { nullable: true } } },
      );

      expectNoFailure(toPass);
      toPass();
    }
  });

  it('should allow dependency on other dependent fields', () => {
    const toPass = makeFx((b) =>
      b
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent1', 'lax')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b
            .dependent('dependent', 'dependent1')
            .default(2)
            .resolve(() => 4),
        )
        .field(b.required('required').validate(validator)),
    );

    expectNoFailure(toPass);
    toPass();
  });

  it('should allow dependency on virtual fields', () => {
    const toPass = makeFx((b) =>
      b
        .field(
          b
            .dependent('dependent', 'virtualField')
            .default(2)
            .resolve(() => 4),
        )
        .field(b.virtual('virtualField').validate(validator)),
    );

    expectNoFailure(toPass);
    toPass();
  });

  it('should allow dependency on virtual fields with aliases', () => {
    const toPass = makeFx((b) =>
      b
        .field(
          b
            .dependent('dependent', 'virtualField')
            .default(2)
            .resolve(() => 4),
        )
        .field(
          b.virtual('virtualField').alias('virtualAlias').validate(validator),
        ),
    );

    expectNoFailure(toPass);
    toPass();
  });
});
