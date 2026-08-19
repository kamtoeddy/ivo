import { describe, expect, it } from 'bun:test';
import { expectFailure, makeFx } from '../../_utils';

describe('field configs.constant', () => {
  it('should reject if field name is already set', () => {
    const toFail = makeFx((b) =>
      b.field(b.constant('id', 1234)).field(b.constant('id', 1234)),
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (e) {
      expect(
        // @ts-expect-error ikr
        e.payload.id.includes(
          '"id" occurs more than once, please remove duplicates',
        ),
      ).toBeTrue();
    }
  });

  it('should reject if field name is the same as createdAt with default name', () => {
    const toFail = makeFx((b) => b.field(b.constant('createdAt', 1234)), {
      timestamps: { createdAt: true },
    });

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
    const toFail = makeFx((b) => b.field(b.constant('customCreatedAt', 1234)), {
      timestamps: { createdAt: 'customCreatedAt' },
    });

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
    const toFail = makeFx((b) => b.field(b.constant('updatedAt', 1234)), {
      timestamps: { updatedAt: { nullable: true } },
    });

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
    const toFail = makeFx((b) => b.field(b.constant('customUpdatedAt', 1234)), {
      timestamps: { updatedAt: { key: 'customUpdatedAt', nullable: true } },
    });

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
