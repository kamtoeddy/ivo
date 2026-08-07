import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { expectFailure, expectNoFailure, makeFx, validator } from '../_utils';

describe('Schema.options.required', () => {
  describe('signature', () => {
    it('should allow a config object or array of config objects', () => {
      const values = [
        { fields: ['a', 'b'], handler: () => undefined },
        {
          fields: ['a', 'b'],
          handler: [() => undefined, () => undefined],
        },
        [{ fields: ['a', 'b'], handler: () => undefined }],
      ];

      for (const required of values) {
        const toPass = makeFx(
          (b) =>
            b
              .field(b.lax('a', 1).validate(validator))
              .field(b.lax('b', 2).validate(validator)),
          { required },
        );

        expectNoFailure(toPass);
        toPass();
      }
    });

    it('should allow requiredBy (conditionally-required) and virtual fields in a group', () => {
      const toPass = makeFx(
        (b) =>
          b
            .field(b.lax('a', 1).validate(validator))
            .field(
              b
                .dependent('dependentField', 'virtualField')
                .default(0)
                .resolve(() => 1),
            )
            .field(b.virtual('virtualField').validate(validator)),
        {
          required: { fields: ['a', 'virtualField'], handler: () => undefined },
        },
      );

      expectNoFailure(toPass);
      toPass();
    });
  });

  describe('invalid', () => {
    it('should reject invalid config shapes', () => {
      const invalidValues = [
        123,
        'invalid_string',
        {},
        { fields: ['a', 'b'] },
        { handler: () => undefined },
        { fields: 'a,b', handler: () => undefined },
      ];

      for (const required of invalidValues) {
        const toFail = makeFx(
          (b) =>
            b
              .field(b.lax('a', 1).validate(validator))
              .field(b.lax('b', 2).validate(validator)),
          { required },
        );

        expectFailure(toFail);
      }
    });

    it('should reject fewer than 2 fields', () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax('a', 1).validate(validator))
            .field(b.lax('b', 2).validate(validator)),
        { required: { fields: ['a'], handler: () => undefined } },
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            required: expect.arrayContaining([
              'grouped required expects at least 2 fields',
            ]),
          }),
        );
      }
    });

    it('should reject duplicate fields', () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax('a', 1).validate(validator))
            .field(b.lax('b', 2).validate(validator)),
        { required: { fields: ['a', 'a'], handler: () => undefined } },
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            required: expect.arrayContaining([
              "remove duplicates of 'a' in your grouped required config",
            ]),
          }),
        );
      }
    });

    it('should reject fields that do not exist on the schema', () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax('a', 1).validate(validator))
            .field(b.lax('b', 2).validate(validator)),
        { required: { fields: ['a', 'z'], handler: () => undefined } },
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            required: expect.arrayContaining([
              "'z' does not exist on your schema",
            ]),
          }),
        );
      }
    });

    it('should reject constant, dependent, and strictly-required fields', () => {
      const schemas = [
        makeFx(
          (b) =>
            b
              .field(b.constant('a', 1))
              .field(b.lax('b', 2).validate(validator)),
          {
            required: { fields: ['a', 'b'], handler: () => undefined },
          },
        ),

        makeFx(
          (b) =>
            b
              .field(
                b
                  .dependent('a', 'b')
                  .default(0)
                  .resolve(() => 1),
              )
              .field(b.lax('b', 2).validate(validator)),
          {
            required: { fields: ['a', 'b'], handler: () => undefined },
          },
        ),
        makeFx(
          (b) =>
            b
              .field(b.required('a').validate(validator))
              .field(b.lax('b', 2).validate(validator)),
          {
            required: { fields: ['a', 'b'], handler: () => undefined },
          },
        ),
      ];

      for (const invalidSchemas of schemas) {
        expectFailure(invalidSchemas);

        try {
          invalidSchemas();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              required: expect.arrayContaining([
                "only lax and virtual fields can belong to grouped required configs; remove 'a'",
              ]),
            }),
          );
        }
      }
    });

    it('should reject timestamp fields', () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax('a', 1).validate(validator))
            .field(b.lax('b', 2).validate(validator)),
        {
          timestamps: true,
          required: {
            fields: ['a', 'createdAt'],
            handler: () => undefined,
          },
        },
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            required: expect.arrayContaining([
              "only lax and virtual fields can belong to grouped required configs; remove 'createdAt'",
            ]),
          }),
        );
      }
    });

    it('should reject an alias; the virtual field name must be used instead', () => {
      const toFail = makeFx(
        (b) =>
          b
            .field(b.lax('b', 2).validate(validator))
            .field(
              b
                .dependent('dependentField', 'virtualField')
                .default(0)
                .resolve(() => 1),
            )
            .field(
              b.virtual('virtualField').alias('vAlias').validate(validator),
            ),
        {
          required: { fields: ['vAlias', 'b'], handler: () => undefined },
        },
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            required: expect.arrayContaining([
              "'vAlias' is an alias; use 'virtualField' instead",
            ]),
          }),
        );
      }
    });
  });

  describe('behaviour', () => {
    const Model = new Schema<{ email: string; phone: string }>(
      (b) => b.field(b.lax('email', '')).field(b.lax('phone', '')),
      {
        required: {
          fields: ['email', 'phone'],
          handler: () => ({
            email: 'either email or phone is required',
            phone: 'either email or phone is required',
          }),
        },
      },
    ).getModel();

    it('should not run the handler (and not fail creation) when at least one field in the group is provided', async () => {
      const withEmail = await Model.create({ email: 'a@b.com' }, {});
      expect(withEmail.error).toBeNull();
      expect(withEmail.data).toEqual({ email: 'a@b.com', phone: '' });

      const withPhone = await Model.create({ phone: '123' }, {});
      expect(withPhone.error).toBeNull();
      expect(withPhone.data).toEqual({ email: '', phone: '123' });
    });

    it('should run the handler and reject creation when none of the fields in the group were provided', async () => {
      const { data, error } = await Model.create({}, {});

      expect(data).toBeNull();
      expect(error).toEqual({
        email: {
          reason: 'either email or phone is required',
          metadata: null,
        },
        phone: {
          reason: 'either email or phone is required',
          metadata: null,
        },
      });
    });

    it('should drop errors returned for fields outside the group', async () => {
      const Model = new Schema<any>(
        (b) =>
          b
            .field(b.lax('a', undefined))
            .field(b.lax('b', undefined))
            .field(b.lax('c', 2)),
        {
          required: {
            fields: ['a', 'b'],
            handler: () => ({
              a: 'a is required',
              b: 'b is required',
              c: 'should be dropped, c is not in this group',
            }),
          },
        },
      ).getModel();

      const { data, error } = await Model.create({}, {});

      expect(data).toBeNull();
      expect(error).toEqual({
        a: { reason: 'a is required', metadata: null },
        b: { reason: 'b is required', metadata: null },
      });
    });

    it('should keep errors by alias for aliased virtual fields', async () => {
      const Model = new Schema<
        { name: string; y2: number },
        { name: string; y: number }
      >(
        (b) =>
          b
            .field(b.lax('name', ''))
            .field(
              b
                .dependent('y', 'setX')
                .default(0)
                .resolve((ctx) => ctx.input.y2 ?? 0),
            )
            .field(b.virtual('setX').alias('y2').validate(validator)),
        {
          required: {
            fields: ['name', 'setX'],
            handler: () =>
              ({ y2: 'setX (aliased as y2) is required' }) as never,
          },
        },
      ).getModel();

      const { data, error } = await Model.create({}, {});

      expect(data).toBeNull();
      expect(error).toEqual({
        y2: { reason: 'setX (aliased as y2) is required', metadata: null },
      });

      const { data: data2, error: error2 } = await Model.create({ y2: 5 }, {});

      expect(error2).toBeNull();
      expect(data2).toEqual({ name: '', y: 5 });
    });

    it('should evaluate on both creation and updates', async () => {
      const Model = new Schema<{ email: string; name: string; phone: string }>(
        (b) =>
          b
            .field(b.lax('email', ''))
            .field(b.lax('phone', ''))
            .field(b.lax('name', '')),
        {
          required: {
            fields: ['email', 'phone'],
            handler: () => ({
              email: 'either email or phone is required',
              phone: 'either email or phone is required',
            }),
          },
        },
      ).getModel();

      // a genuine update (changing `name`) that leaves the `email`/`phone`
      // group untouched — the group's handler should still run and reject.
      const { data, error } = await Model.update(
        { name: '', email: '', phone: '' },
        { name: 'bob' },
        {},
      );

      expect(data).toBeNull();
      expect(error?.payload).toEqual({
        email: {
          reason: 'either email or phone is required',
          metadata: null,
        },
        phone: {
          reason: 'either email or phone is required',
          metadata: null,
        },
      });
    });

    it('should run every handler in an array and merge their results', async () => {
      const Model = new Schema<{ a: any; b: any }>(
        (b) => b.field(b.lax('a', undefined)).field(b.lax('b', undefined)),
        {
          // @ts-expect-error ikr
          required: {
            fields: ['a', 'b'],
            handler: [() => ({ a: 'a missing' }), () => ({ b: 'b missing' })],
          },
        },
      ).getModel();

      const { data, error } = await Model.create({}, {});

      expect(data).toBeNull();
      expect(error).toEqual({
        // @ts-expect-error ikr
        a: { reason: 'a missing', metadata: null },
        b: { reason: 'b missing', metadata: null },
      });
    });

    it('should treat a thrown handler as satisfied (no error), matching the swallow-errors convention', async () => {
      const Model = new Schema<any>(
        (b) => b.field(b.lax('a', undefined)).field(b.lax('b', undefined)),
        {
          required: {
            fields: ['a', 'b'],
            handler: () => {
              throw new Error('boom');
            },
          },
        },
      ).getModel();

      const { data, error } = await Model.create({}, {});

      expect(error).toBeNull();
      expect(data).toEqual({});
    });
  });
});
