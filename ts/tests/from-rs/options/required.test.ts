import { describe, it } from 'bun:test';
import { expectFailure, makeFx, validator } from '../../_utils';

describe('options.required', () => {
  it('should reject if fields array is empty', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { required: { fields: [], handler: () => undefined } },
    );

    expectFailure(toFail, 'grouped required expects at least 2 fields');
  });

  it('should reject if fields array has just one field', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { required: { fields: ['lax'], handler: () => undefined } },
    );

    expectFailure(toFail, 'grouped required expects at least 2 fields');
  });

  it('should reject if the fields array contains any duplicates', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { required: { fields: ['lax', 'lax'], handler: () => undefined } },
    );

    expectFailure(
      toFail,
      'remove duplicates of "lax" in your grouped required config',
    );
  });

  it('should reject if the fields array contains any string that is not a field on schema', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        required: {
          fields: ['lax', 'invalid_field'],
          handler: () => undefined,
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
      { required: { fields: ['lax', 'id'], handler: () => undefined } },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "id"',
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
        required: {
          fields: ['lax', 'lax_1', 'dependent'],
          handler: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "dependent"',
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
        required: {
          fields: ['lax', 'required', 'lax_1'],
          handler: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "required"',
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
        required: {
          fields: ['lax', 'lax_1', 'dependent'],
          handler: () => undefined,
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
        required: {
          fields: ['lax', 'lax_1', 'alias'],
          handler: () => undefined,
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
        required: {
          fields: ['lax', 'lax_1', 'createdAt'],
          handler: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "createdAt"',
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
        required: {
          fields: ['lax', 'lax_1', 'customCreatedAt'],
          handler: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "customCreatedAt"',
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
        required: {
          fields: ['lax', 'lax_1', 'updatedAt'],
          handler: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "updatedAt"',
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
        required: {
          fields: ['lax', 'lax_1', 'customUpdatedAt'],
          handler: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax and virtual fields can belong to grouped required configs; remove "customUpdatedAt"',
    );
  });
});
