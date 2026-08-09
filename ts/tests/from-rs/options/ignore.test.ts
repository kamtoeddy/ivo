import { describe, it } from 'bun:test';
import { expectFailure, makeFx, validator } from '../../_utils';

describe('options.ignore', () => {
  it('should reject if fields array is empty', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignore: { fields: [], resolver: () => false } },
    );

    expectFailure(toFail, 'grouped ignore expects at least 2 fields');
  });

  it('should reject if fields array has just one field', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignore: { fields: ['lax'], resolver: () => false } },
    );

    expectFailure(toFail, 'grouped ignore expects at least 2 fields');
  });

  it('should reject if the fields array contains any duplicates', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignore: { fields: ['lax', 'lax'], resolver: () => false } },
    );

    expectFailure(
      toFail,
      'remove duplicates of "lax" in your grouped ignore config',
    );
  });

  it('should reject if the fields array contains any string that is not a field on schema', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        ignore: {
          fields: ['lax', 'invalid_field'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail, '"invalid_field" does not exist on your schema');
  });

  it('should reject if a constant is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.constant('id', 1234))
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { ignore: { fields: ['lax', 'id'], resolver: () => false } },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "id"',
    );
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
        ignore: {
          fields: ['lax', 'lax_1', 'dependent'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "dependent"',
    );
  });

  it('should reject if a required field is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator))
          .field(b.required('required').validate(validator)),
      {
        ignore: {
          fields: ['lax', 'required', 'lax_1'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "required"',
    );
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
        ignore: {
          fields: ['lax', 'lax_1', 'dependent'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      '"dependent" is an alias; use "virtualField" instead',
    );
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
        ignore: {
          fields: ['lax', 'lax_1', 'alias'],
          resolver: () => false,
        },
      },
    );

    expectFailure(toFail, '"alias" is an alias; use "virtualField" instead');
  });

  it('should reject if created_at timestamp with default name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { createdAt: true },
        ignore: {
          fields: ['lax', 'lax_1', 'createdAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "createdAt"',
    );
  });

  it('should reject if created_at timestamp with custom name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { createdAt: 'customCreatedAt' },
        ignore: {
          fields: ['lax', 'lax_1', 'customCreatedAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "customCreatedAt"',
    );
  });

  it('should reject if updated_at timestamp with default name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { updatedAt: true },
        ignore: {
          fields: ['lax', 'lax_1', 'updatedAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "updatedAt"',
    );
  });

  it('should reject if updated_at timestamp with custom name is provided to the fields array', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        timestamps: { updatedAt: { key: 'customUpdatedAt' } },
        ignore: {
          fields: ['lax', 'lax_1', 'customUpdatedAt'],
          resolver: () => false,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped ignore configs; remove "customUpdatedAt"',
    );
  });
});
