import { describe, it } from 'bun:test';
import {
  expectFailure,
  expectNoFailure,
  makeFx,
  validator,
} from '../../_utils';

describe('field configs.virtual', () => {
  it('should reject if virtual field does not have any dependency', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', 'lax')
            .default(1)
            .resolve(() => 2),
        )
        .field(b.virtual('virtualField').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject with same alias name', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(
          b
            .dependent('dependent', 'virtualField')
            .default(1)
            .resolve(() => 2),
        )
        .field(
          b.virtual('virtualField').alias('virtualField').validate(validator),
        ),
    );

    expectFailure(toFail);
  });

  it('should reject with alias as non-dependent field', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', 'virtualField')
            .default(1)
            .resolve(() => 2),
        )
        .field(b.virtual('virtualField').alias('lax').validate(validator)),
    );

    expectFailure(toFail);
  });

  it('should reject with alias as unrelated dependent field', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent1', 'lax')
            .default(1)
            .resolve(() => 2),
        )
        .field(
          b
            .dependent('dependent', 'virtualField')
            .default(1)
            .resolve(() => 2),
        )
        .field(
          b.virtual('virtualField').alias('dependent1').validate(validator),
        ),
    );

    expectFailure(toFail);
  });

  it('should reject if alias is same createdAt if enabled with default name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(
            b.virtual('virtualField').alias('createdAt').validate(validator),
          ),
      { timestamps: { createdAt: true } },
    );

    expectFailure(toFail);
  });

  it('should reject if alias is same createdAt if enabled with custom name', () => {
    const toFail = makeFx(
      (b) =>
        b.field(
          b
            .virtual('virtualField')
            .alias('customCreatedAt')
            .validate(validator),
        ),
      { timestamps: { createdAt: 'customCreatedAt' } },
    );

    expectFailure(toFail);
  });

  it('should reject if alias is same updatedAt if enabled with default name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(
            b.virtual('virtualField').alias('updatedAt').validate(validator),
          ),
      { timestamps: { updatedAt: { nullable: true } } },
    );

    expectFailure(toFail);
  });

  it('should reject if alias is same updatedAt if enabled with custom name', () => {
    const toFail = makeFx(
      (b) =>
        b.field(
          b
            .virtual('virtualField')
            .alias('customUpdatedAt')
            .validate(validator),
        ),
      { timestamps: { updatedAt: { key: 'customUpdatedAt', nullable: true } } },
    );

    expectFailure(toFail);
  });

  it('should reject if alias already used', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', ['lax', 'virtualField', 'virtualField1'])
            .default(1)
            .resolve(() => 2),
        )
        .field(
          b.virtual('virtualField1').alias('dependent').validate(validator),
        )
        .field(
          b.virtual('virtualField').alias('dependent').validate(validator),
        ),
    );

    expectFailure(toFail);
  });

  it('should allow virtuals with alias as direct dependent field', () => {
    const toPass = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 1))
        .field(
          b
            .dependent('dependent', ['lax', 'virtualField', 'virtualField1'])
            .default(1)
            .resolve(() => 2),
        )
        .field(b.virtual('virtualField').alias('dependent').validate(validator))
        .field(b.virtual('virtualField1').validate(validator)),
    );

    expectNoFailure(toPass);
    toPass();
  });

  it('should allow virtuals with alias as non field name', () => {
    const toPass = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1))
          .field(
            b
              .dependent('dependent', ['lax', 'virtualField'])
              .default(1)
              .resolve(() => 2),
          )
          .field(
            b.virtual('virtualField').alias('aliasName').validate(validator),
          ),
      { timestamps: { createdAt: true, updatedAt: true } },
    );

    expectNoFailure(toPass);
    toPass();
  });
});
