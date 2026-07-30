import { describe, expect, it } from 'bun:test';
import { expectFailure, expectNoFailure, validator } from '../_utils';

/**
 * Mirrors `rs/tests/options/required.rs` (construction-time validation) and
 * the grouped-required runtime behaviour exercised throughout Rust's
 * `evaluate_missing_required_fields`: a config's handler only runs when NONE
 * of its `properties` are relevant/provided; the handler then returns a
 * per-field error map (or `undefined` when satisfied), and only entries for
 * fields declared in the group are kept.
 */
export const Test_SchemaOptionRequired = ({ Schema, fx }: any) => {
  describe('Schema.options.required', () => {
    describe('signature', () => {
      it('should allow a config object or array of config objects', () => {
        const values = [
          { properties: ['a', 'b'], handler: () => undefined },
          {
            properties: ['a', 'b'],
            handler: [() => undefined, () => undefined],
          },
          [{ properties: ['a', 'b'], handler: () => undefined }],
        ];

        for (const required of values) {
          const toPass = fx(
            { a: { default: 1, validator }, b: { default: 2, validator } },
            { required },
          );

          expectNoFailure(toPass);
          toPass();
        }
      });

      it('should allow requiredBy (conditionally-required) and virtual fields in a group', () => {
        const toPass = fx(
          {
            a: { default: 1, required: () => false, validator },
            virtualProp: { virtual: true, validator },
            dependentProp: {
              default: 0,
              dependsOn: 'virtualProp',
              resolver: () => 1,
            },
          },
          {
            required: {
              properties: ['a', 'virtualProp'],
              handler: () => undefined,
            },
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
          { properties: ['a', 'b'] },
          { handler: () => undefined },
          { properties: 'a,b', handler: () => undefined },
        ];

        for (const required of invalidValues) {
          const toFail = fx(
            { a: { default: 1, validator }, b: { default: 2, validator } },
            { required },
          );

          expectFailure(toFail);
        }
      });

      it('should reject fewer than 2 properties', () => {
        const toFail = fx(
          { a: { default: 1, validator }, b: { default: 2, validator } },
          { required: { properties: ['a'], handler: () => undefined } },
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

      it('should reject duplicate properties', () => {
        const toFail = fx(
          { a: { default: 1, validator }, b: { default: 2, validator } },
          { required: { properties: ['a', 'a'], handler: () => undefined } },
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

      it('should reject properties that do not exist on the schema', () => {
        const toFail = fx(
          { a: { default: 1, validator }, b: { default: 2, validator } },
          { required: { properties: ['a', 'z'], handler: () => undefined } },
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
        const cases = [
          {
            a: { constant: true, value: 1 },
            b: { default: 2, validator },
          },
          {
            a: {
              default: 0,
              dependsOn: 'b',
              resolver: () => 1,
            },
            b: { default: 2, validator },
          },
          {
            a: { required: true, validator },
            b: { default: 2, validator },
          },
        ];

        for (const definitions of cases) {
          const toFail = fx(definitions, {
            required: { properties: ['a', 'b'], handler: () => undefined },
          });

          expectFailure(toFail);

          try {
            toFail();
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
        const toFail = fx(
          { a: { default: 1, validator }, b: { default: 2, validator } },
          {
            timestamps: true,
            required: {
              properties: ['a', 'createdAt'],
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
        const toFail = fx(
          {
            virtualProp: { virtual: true, alias: 'vAlias', validator },
            dependentProp: {
              default: 0,
              dependsOn: 'virtualProp',
              resolver: () => 1,
            },
            b: { default: 2, validator },
          },
          {
            required: { properties: ['vAlias', 'b'], handler: () => undefined },
          },
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              required: expect.arrayContaining([
                "'vAlias' is an alias; use 'virtualProp' instead",
              ]),
            }),
          );
        }
      });
    });

    describe('behaviour', () => {
      it('should not run the handler (and not fail creation) when at least one field in the group is provided', async () => {
        const Model = new Schema(
          { email: { default: '' }, phone: { default: '' } },
          {
            required: {
              properties: ['email', 'phone'],
              handler: () => ({
                email: 'either email or phone is required',
                phone: 'either email or phone is required',
              }),
            },
          },
        ).getModel();

        const withEmail = await Model.create({ email: 'a@b.com' });
        expect(withEmail.error).toBeNull();
        expect(withEmail.data).toEqual({ email: 'a@b.com', phone: '' });

        const withPhone = await Model.create({ phone: '123' });
        expect(withPhone.error).toBeNull();
        expect(withPhone.data).toEqual({ email: '', phone: '123' });
      });

      it('should run the handler and reject creation when none of the fields in the group were provided', async () => {
        const Model = new Schema(
          { email: { default: '' }, phone: { default: '' } },
          {
            required: {
              properties: ['email', 'phone'],
              handler: () => ({
                email: 'either email or phone is required',
                phone: 'either email or phone is required',
              }),
            },
          },
        ).getModel();

        const { data, error } = await Model.create({});

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
        const Model = new Schema(
          {
            a: { default: undefined as never },
            b: { default: undefined as never },
            c: { default: 0 },
          },
          {
            required: {
              properties: ['a', 'b'],
              handler: () => ({
                a: 'a is required',
                b: 'b is required',
                c: 'should be dropped, c is not in this group',
              }),
            },
          },
        ).getModel();

        const { data, error } = await Model.create({});

        expect(data).toBeNull();
        expect(error).toEqual({
          a: { reason: 'a is required', metadata: null },
          b: { reason: 'b is required', metadata: null },
        });
      });

      it('should key errors by alias for aliased virtual fields', async () => {
        const Model = new Schema(
          {
            name: { default: '' },
            y: {
              default: 0,
              dependsOn: 'setX',
              resolver: (ctx: any) => ctx.input.setX ?? 0,
            },
            setX: { virtual: true, alias: 'y2', validator: () => true },
          },
          {
            required: {
              properties: ['name', 'setX'],
              handler: () =>
                ({ y2: 'setX (aliased as y2) is required' }) as never,
            },
          },
        ).getModel();

        const { data, error } = await Model.create({});

        expect(data).toBeNull();
        expect(error).toEqual({
          y2: { reason: 'setX (aliased as y2) is required', metadata: null },
        });

        const { data: data2, error: error2 } = await Model.create({
          y2: 5,
        } as never);

        expect(error2).toBeNull();
        expect(data2).toEqual({ name: '', y: 5 });
      });

      it('should evaluate on both creation and updates', async () => {
        const Model = new Schema(
          {
            name: { default: '' },
            email: { default: '' },
            phone: { default: '' },
          },
          {
            required: {
              properties: ['email', 'phone'],
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
        );

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

      it('should run every handler in an array and merge their results', async () => {
        const Model = new Schema(
          {
            a: { default: undefined as never },
            b: { default: undefined as never },
          },
          {
            required: {
              properties: ['a', 'b'],
              handler: [() => ({ a: 'a missing' }), () => ({ b: 'b missing' })],
            },
          },
        ).getModel();

        const { data, error } = await Model.create({});

        expect(data).toBeNull();
        expect(error).toEqual({
          a: { reason: 'a missing', metadata: null },
          b: { reason: 'b missing', metadata: null },
        });
      });

      it('should treat a thrown handler as satisfied (no error), matching the swallow-errors convention', async () => {
        const Model = new Schema(
          {
            a: { default: undefined as never },
            b: { default: undefined as never },
          },
          {
            required: {
              properties: ['a', 'b'],
              handler: () => {
                throw new Error('boom');
              },
            },
          },
        ).getModel();

        const { data, error } = await Model.create({});

        expect(error).toBeNull();
        expect(data).toEqual({});
      });
    });
  });
};
