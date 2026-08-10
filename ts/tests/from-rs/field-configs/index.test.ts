import { describe, expect, it } from 'bun:test';
import { expectFailure, makeFx, validator } from '../../_utils';

describe('field configs', () => {
  it('should reject duplicate field names', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 'value').validate(validator))
        .field(b.lax('lax', true).validate(validator)),
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.lax.includes(
          '"lax" occurs more than once, please remove duplicates',
        ),
      ).toBeTrue();
    }
  });

  it('should reject if field name is the same as createdAt with default name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('createdAt', 'value').validate(validator)),
      { timestamps: { createdAt: true } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.createdAt.includes(
          '"createdAt" is not a valid field name. It is the creation timestamp',
        ),
      ).toBeTrue();
    }
  });

  it('should reject if field name is the same as createdAt with custom name', () => {
    const toFail = makeFx(
      (b) => b.field(b.lax('customCreatedAt', 'value').validate(validator)),
      { timestamps: { createdAt: 'customCreatedAt' } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.customCreatedAt.includes(
          '"customCreatedAt" is not a valid field name. It is the creation timestamp',
        ),
      ).toBeTrue();
    }
  });

  it('should reject if field name is the same as updatedAt with default name', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('updatedAt', 'value').validate(validator)),
      { timestamps: { updatedAt: true } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.updatedAt.includes(
          '"updatedAt" is not a valid field name. It is the update timestamp',
        ),
      ).toBeTrue();
    }
  });

  it('should reject if field name is the same as updatedAt with custom name', () => {
    const toFail = makeFx(
      (b) => b.field(b.lax('customUpdatedAt', 'value').validate(validator)),
      { timestamps: { updatedAt: { key: 'customUpdatedAt' } } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.customUpdatedAt.includes(
          '"customUpdatedAt" is not a valid field name. It is the update timestamp',
        ),
      ).toBeTrue();
    }
  });
});
