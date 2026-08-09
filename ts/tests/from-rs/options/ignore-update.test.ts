import { describe, expect, it } from 'bun:test';
import {
  expectFailure,
  expectNoFailure,
  makeFx,
  validator,
} from '../../_utils';

describe('options.ignoreUpdate', () => {
  it('should allow if fields array is empty', () => {
    const toPass = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignoreUpdate: { fields: [], resolver: () => false } },
    );

    expectNoFailure(toPass);
    toPass();
  });

  it('should reject if fields array has just one field', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignoreUpdate: { fields: ['lax'], resolver: () => false } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'grouped ignore update expects either zero (0) fields or at least 2 fields',
          ]),
        }),
      );
    }
  });

  it('should reject if the fields array contains any duplicates', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignoreUpdate: { fields: ['lax', 'lax'], resolver: () => false } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'remove duplicates of "lax" in your grouped ignore update config',
          ]),
        }),
      );
    }
  });

  it('should reject if the fields array contains any string that is not a field on schema', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        ignoreUpdate: {
          fields: ['lax', 'invalid_field'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            '"invalid_field" does not exist on your schema',
          ]),
        }),
      );
    }
  });

  it('should reject if a constant is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignoreUpdate: { fields: ['lax', 'id'], resolver: () => false } },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'only lax, required and virtual fields can belong to grouped ignore update configs; remove "id"',
          ]),
        }),
      );
    }
  });

  it('should reject if a dependent field is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', ['lax', 'lax_1'])
              .default(1)
              .resolve(() => 2),
          )
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'dependent'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'only lax, required and virtual fields can belong to grouped ignore update configs; remove "dependent"',
          ]),
        }),
      );
    }
  });

  it('should reject if an alias similar to a dependent field is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', ['lax', 'virtualField'])
              .default(1)
              .resolve(() => 2),
          )
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator))
          .field(
            b.virtual('virtualField').alias('dependent').validate(validator),
          ),
      {
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'dependent'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            '"dependent" is an alias; use "virtualField" instead',
          ]),
        }),
      );
    }
  });

  it('should reject if an alias with foreign name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', ['lax', 'virtualField'])
              .default(1)
              .resolve(() => 2),
          )
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator))
          .field(b.virtual('virtualField').alias('alias').validate(validator)),
      {
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'alias'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            '"alias" is an alias; use "virtualField" instead',
          ]),
        }),
      );
    }
  });

  it('should reject if created_at timestamp with default name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { createdAt: true },
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'createdAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'only lax, required and virtual fields can belong to grouped ignore update configs; remove "createdAt"',
          ]),
        }),
      );
    }
  });

  it('should reject if created_at timestamp with custom name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { createdAt: 'customCreatedAt' },
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'customCreatedAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'only lax, required and virtual fields can belong to grouped ignore update configs; remove "customCreatedAt"',
          ]),
        }),
      );
    }
  });

  it('should reject if updated_at timestamp with default name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { updatedAt: true },
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'updatedAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'only lax, required and virtual fields can belong to grouped ignore update configs; remove "updatedAt"',
          ]),
        }),
      );
    }
  });

  it('should reject if updated_at timestamp with custom name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { updatedAt: { key: 'customUpdatedAt' } },
        ignoreUpdate: {
          fields: ['lax', 'lax_1', 'customUpdatedAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toEqual(
        expect.objectContaining({
          ignoreUpdate: expect.arrayContaining([
            'only lax, required and virtual fields can belong to grouped ignore update configs; remove "customUpdatedAt"',
          ]),
        }),
      );
    }
  });
});
