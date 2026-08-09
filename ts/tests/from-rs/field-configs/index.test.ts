import { describe, expect, it } from 'bun:test';
import { expectFailure, makeFx, validator } from '../../_utils';

describe('field configs', () => {
  it('should reject if field name is already set', () => {
    const toFail = makeFx((b) =>
      b
        .field(b.constant('id', 1234))
        .field(b.lax('lax', 'value').validate(validator))
        .field(b.lax('lax', true).validate(validator)),
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          lax: expect.arrayContaining([
            'occurs more than once, please remove duplicates',
          ]),
        }),
      );
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
  });

  it('should reject if field name is the same as createdAt with custom name', () => {
    const toFail = makeFx(
      (b) => b.field(b.lax('customCreatedAt', 'value').validate(validator)),
      { timestamps: { createdAt: 'customCreatedAt' } },
    );

    expectFailure(toFail);
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
  });

  it('should reject if field name is the same as updatedAt with custom name', () => {
    const toFail = makeFx(
      (b) => b.field(b.lax('customUpdatedAt', 'value').validate(validator)),
      { timestamps: { updatedAt: { key: 'customUpdatedAt' } } },
    );

    expectFailure(toFail);
  });
});
