import { describe, expect, it } from 'bun:test';
import { expectFailure, makeFx, validator } from '../../_utils';

describe('field configs.required', () => {
  it('should reject if field name is already set', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.required('required').validate(validator))
        .field(b.required('required').validate(validator)),
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.required.includes(
          '"required" occurs more than once, please remove duplicates',
        ),
      ).toBeTrue();
    }
  });

  it('should reject if field name is the same as createdAt with default name', () => {
    const toFail = makeFx(
      (b) => b.field(b.required('createdAt').validate(validator)),
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
      (b) => b.field(b.required('customCreatedAt').validate(validator)),
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
      (b) => b.field(b.required('updatedAt').validate(validator)),
      { timestamps: { updatedAt: { nullable: true } } },
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
      (b) => b.field(b.required('customUpdatedAt').validate(validator)),
      { timestamps: { updatedAt: { key: 'customUpdatedAt', nullable: true } } },
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
