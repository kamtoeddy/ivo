import { describe, it } from 'bun:test';
import { expectFailure, makeFx, validator } from '../../_utils';

describe('options.postValidate', () => {
  it('should reject if fields array is empty', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { postValidate: { fields: [], validator: () => undefined } },
    );

    expectFailure(toFail, 'post-validation expects at least 2 fields');
  });

  it('should reject if fields array has just one field', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      { postValidate: { fields: ['lax'], validator: () => undefined } },
    );

    expectFailure(toFail, 'post-validation expects at least 2 fields');
  });

  it('should reject if the fields array contains any duplicates', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        postValidate: { fields: ['lax', 'lax'], validator: () => undefined },
      },
    );

    expectFailure(
      toFail,
      'remove duplicates of "lax" in your post-validation config',
    );
  });

  it('should reject if the fields array contains any string that is not a field on schema', () => {
    const toFail = makeFx(
      (b) =>
        b
          .field(b.lax('lax', 1234).validate(validator))
          .field(b.lax('lax_1', 5678).validate(validator)),
      {
        postValidate: {
          fields: ['lax', 'invalid_field'],
          validator: () => undefined,
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
      { postValidate: { fields: ['lax', 'id'], validator: () => undefined } },
    );

    expectFailure(
      toFail,
      'only lax, required and virtual fields can be post-validated; remove "id"',
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
        postValidate: {
          fields: ['lax', 'lax_1', 'dependent'],
          validator: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax, required and virtual fields can be post-validated; remove "dependent"',
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
        postValidate: {
          fields: ['lax', 'lax_1', 'dependent'],
          validator: () => undefined,
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
        postValidate: {
          fields: ['lax', 'lax_1', 'alias'],
          validator: () => undefined,
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
        postValidate: {
          fields: ['lax', 'lax_1', 'createdAt'],
          validator: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax, required and virtual fields can be post-validated; remove "createdAt"',
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
        postValidate: {
          fields: ['lax', 'lax_1', 'customCreatedAt'],
          validator: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax, required and virtual fields can be post-validated; remove "customCreatedAt"',
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
        postValidate: {
          fields: ['lax', 'lax_1', 'updatedAt'],
          validator: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax, required and virtual fields can be post-validated; remove "updatedAt"',
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
        postValidate: {
          fields: ['lax', 'lax_1', 'customUpdatedAt'],
          validator: () => undefined,
        },
      },
    );

    expectFailure(
      toFail,
      'only lax, required and virtual fields can be post-validated; remove "customUpdatedAt"',
    );
  });
});
