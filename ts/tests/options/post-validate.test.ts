import { afterEach, beforeEach, describe, expect, it, test } from 'bun:test';

import { type IvoContext, type ReadonlyIvoContext, Schema } from '../../src';

import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  makeFx,
  validator,
} from '../_utils';

describe('Schema.options.postValidate', () => {
  describe('signature', () => {
    describe('single config', () => {
      describe('valid', () => {
        it("should allow 'postValidate' as valid config", () => {
          const validators = [
            () => {},
            [() => {}, () => {}],
            [() => {}, [() => {}, () => {}]],
            [() => {}, [() => {}, () => {}], () => {}],
          ];

          for (const validator of validators) {
            const toPass = makeFx(getValidSchema(), {
              postValidate: {
                fields: ['fieldName1', 'fieldName2'],
                validator,
              },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow 'postValidate' if some or all the fields to post validate are virtuals", () => {
          const values = [
            { fields: ['virtual', 'fieldName2'], validator() {} },
            [{ fields: ['virtual', 'virtual2'], validator() {} }],
          ];

          for (const postValidate of values) {
            const toPass = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', ['virtual', 'virtual2'])
                      .default('')
                      .resolve(() => {}),
                  )
                  .field(b.virtual('virtual').validate(() => false))
                  .field(b.virtual('virtual2').validate(() => false)),
              { postValidate },
            );

            expectNoFailure(toPass);

            toPass();
          }
        });
      });

      describe('invalid', () => {
        describe("should reject if 'fields' is not an array or does not contain valid input keys of schema", () => {
          const values = [
            [
              ['fieldName1', 'fieldName1'],
              'remove duplicates of "fieldName1" in your grouped post-validation config',
            ],
            [
              ['fieldName1', 'fieldName2', 'lol'],

              'only lax, required and virtual fields can be post-validated remove "lol"',
            ],
            [
              ['fieldName1', 'dependent'],

              'only lax, required and virtual fields can be post-validated remove "dependent"',
            ],
          ] as const;

          test.each(values)('', (fields, error) => {
            const toFail = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', ['fieldName1'])
                      .default('')
                      .resolve(() => {}),
                  ),
              { postValidate: { fields, validator() {} } },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: 'INVALID_SCHEMA',
                payload: {
                  'options.postValidate': expect.arrayContaining([error]),
                },
              });
            }
          });
        });

        it("should reject if 'validator' is not a function or array", () => {
          const values = [
            -1,
            0,
            1,
            null,
            undefined,
            true,
            false,
            '',
            'invalid',
            {},
          ];

          for (const validator of values) {
            const toFail = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', 'fieldName1')
                      .default('')
                      .resolve(() => {}),
                  ),
              {
                postValidate: {
                  fields: ['fieldName1', 'fieldName2'],
                  validator,
                },
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: 'INVALID_SCHEMA',
                payload: {
                  'options.postValidate': expect.arrayContaining([
                    'validator must be a function or array of functions',
                  ]),
                },
              });
            }
          }
        });

        describe('validator array', () => {
          it('should reject if array is empty', () => {
            const toFail = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', 'fieldName1')
                      .default('')
                      .resolve(() => {}),
                  ),
              {
                postValidate: {
                  fields: ['fieldName1', 'fieldName2'],
                  validator: [],
                },
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: 'INVALID_SCHEMA',
                payload: {
                  'options.postValidate': expect.arrayContaining([
                    'validator must be a function or array of functions',
                  ]),
                },
              });
            }
          });

          it('should reject if any of the validators is not a function', () => {
            const values = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              // [],
            ];

            const toFail = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', 'fieldName1')
                      .default('')
                      .resolve(() => {}),
                  ),
              {
                postValidate: {
                  fields: ['fieldName1', 'fieldName2'],
                  validator: values,
                },
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: 'INVALID_SCHEMA',
                payload: {
                  'options.postValidate': expect.arrayContaining(
                    values.map(
                      () =>
                        'validator must be a function or array of functions',
                    ),
                  ),
                },
              });
            }
          });
        });
      });
    });

    describe('multiple configs', () => {
      describe('valid', () => {
        it("should allow 'postValidate' as an array of subset and non-subset valid configs", () => {
          const configs = [
            ['fieldName1', 'p1', 'p3'],
            ['fieldName1', 'p1'],
            ['fieldName1', 'p3'],
            ['p1', 'p3'],
            ['fieldName1', 'p1', 'p2'],
            ['p1', 'p2', 'p3'],
            ['v1', 'v2'],
            ['p1', 'v2'],
            ['v1', 'p2'],
          ];

          const toPass = makeFx(
            (b) =>
              b
                .field(b.lax('fieldName1', ''))
                .field(b.lax('fieldName2', ''))
                .field(
                  b
                    .dependent('dependent', ['v1', 'v2'])
                    .default('')
                    .resolve(() => {}),
                )
                .field(b.lax('p1', ''))
                .field(b.lax('p2', ''))
                .field(b.lax('p3', ''))
                .field(b.virtual('v1').validate(() => true))
                .field(b.virtual('v2').validate(() => true)),
            {
              postValidate: configs.map((fields, i) => ({
                fields,
                validator: i % 2 === 0 ? validator : [validator, validator],
              })),
            },
          );

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe('invalid', () => {
        describe('validator array', () => {
          it('should reject if array is empty', () => {
            const toFail = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', 'fieldName1')
                      .default('')
                      .resolve(() => {}),
                  )
                  .field(b.lax('lax', '')),
              {
                postValidate: [
                  {
                    fields: ['fieldName1', 'fieldName2'],
                    validator: [validator],
                  },
                  {
                    fields: ['fieldName1', 'lax'],
                    validator: [],
                  },
                ],
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: 'INVALID_SCHEMA',
                payload: {
                  'options.postValidate': expect.arrayContaining([
                    'validator must be a function or array of functions',
                  ]),
                },
              });
            }
          });

          it('should reject if any of the validators is not a function', () => {
            const values = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
            ];

            const toFail = makeFx(
              (b) =>
                b
                  .field(b.lax('fieldName1', ''))
                  .field(b.lax('fieldName2', ''))
                  .field(
                    b
                      .dependent('dependent', 'fieldName1')
                      .default('')
                      .resolve(() => {}),
                  ),
              {
                postValidate: [
                  {
                    fields: ['fieldName1', 'fieldName2'],
                    validator,
                  },
                  {
                    fields: ['fieldName1', 'fieldName2'],
                    validator: values,
                  },
                ],
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: 'INVALID_SCHEMA',
                payload: {
                  'options.postValidate': expect.arrayContaining(
                    values.map(
                      () =>
                        'validator must be a function or array of functions',
                    ),
                  ),
                },
              });
            }
          });
        });
      });
    });
  });

  describe('behaviour', () => {
    describe('should properly trigger post-validators', () => {
      let providedfieldsStats: Record<string, number> = {};
      let ctxStats: Record<string, unknown> = {};

      function handlePostValidate(field: string, ctx: any) {
        ctxStats[field] = ctx;

        if (field in ctx.input)
          providedfieldsStats[field] = (providedfieldsStats[field] ?? 0) + 1;
      }

      function makePostValidationConfig(fields: string[]) {
        return {
          fields,
          validator(ctx: any) {
            for (const field of fields) handlePostValidate(field, ctx);
          },
        };
      }

      afterEach(() => {
        ctxStats = {};
        providedfieldsStats = {};
      });

      describe('behaviour with single post-validators', () => {
        const Model = new Schema<any>(
          (b) =>
            b
              .field(b.lax('fieldName1', ''))
              .field(b.lax('fieldName2', ''))
              .field(
                b
                  .dependent('dependent', ['virtual', 'virtual2'])
                  .default('')
                  .resolve(validator),
              )
              .field(b.lax('lax', ''))
              .field(b.required('requiredReadonly').validate(() => true))
              .field(b.lax('readonlyLax', '').readonly())
              .field(b.required('required').validate(() => true))
              .field(b.virtual('virtual').validate(() => true))
              .field(b.virtual('virtual2').validate(() => true)),
          {
            // @ts-expect-error ikr
            postValidate: makePostValidationConfig([
              'lax',
              'required',
              'requiredReadonly',
              'readonlyLax',
              'virtual',
              'virtual2',
            ]),
          },
        ).getModel();

        it('should trigger all post-validators at creation', async () => {
          const { error } = await Model.create(
            {
              required: 'req',
              lax: '',
              requiredReadonly: 'reqReadonly',
              readonlyLax: '',
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({
            lax: 1,
            required: 1,
            requiredReadonly: 1,
            readonlyLax: 1,
          });
        });

        it('should not trigger post-validators of virtuals not provided at creation', async () => {
          const { error } = await Model.create(
            {
              required: 'req',
              lax: '',
              requiredReadonly: 'reqReadonly',
              readonlyLax: '',
              virtual2: true,
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({
            lax: 1,
            required: 1,
            requiredReadonly: 1,
            readonlyLax: 1,
            virtual2: 1,
          });
        });

        it('should only trigger post-validators of props that change during updates', async () => {
          const { error } = await Model.update(
            // @ts-expect-error ikr
            { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: 1 },
            {
              lax: true,
              required: 1,
              requiredReadonly: true,
              readonlyLax: true,
              virtual: true,
              virtual2: true,
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({
            lax: 1,
            requiredReadonly: 1,
            virtual: 1,
            virtual2: 1,
          });
        });

        it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
          const { error } = await Model.update(
            // @ts-expect-error ikr
            { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: '' },
            {
              requiredReadonly: 1,
              readonlyLax: true,
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({ readonlyLax: 1 });
        });

        describe('behaviour with post-validators that have validator arrays', () => {
          const fields = [
            'lax',
            'required',
            'requiredReadonly',
            'readonlyLax',
            'virtual',
            'virtual2',
          ];

          const Model = new Schema<any>(
            (b) =>
              b
                .field(
                  b
                    .dependent('dependent', ['virtual', 'virtual2'])
                    .default('')
                    .resolve(validator),
                )
                .field(b.lax('lax', ''))
                .field(
                  b
                    .required('requiredReadonly')
                    .validate(() => true)
                    .readonly(),
                )
                .field(b.lax('readonlyLax', '').readonly())
                .field(b.required('required').validate(() => true))
                .field(b.virtual('virtual').validate(() => true))
                .field(b.virtual('virtual2').validate(() => true)),
            {
              // @ts-expect-error ikr
              postValidate: {
                fields,
                validator: [makePostValidator(), makePostValidator()],
              },
            },
          ).getModel();

          function makePostValidator() {
            return (ctx: any) => {
              for (const field of fields) handlePostValidate(field, ctx);
            };
          }

          it('should trigger all post-validators at creation', async () => {
            const { error } = await Model.create(
              {
                required: 'req',
                lax: '',
                requiredReadonly: 'reqReadonly',
                readonlyLax: '',
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({
              lax: 2,
              required: 2,
              requiredReadonly: 2,
              readonlyLax: 2,
            });
          });

          it('should not trigger post-validators of virtuals not provided at creation', async () => {
            const { error } = await Model.create(
              {
                required: 'req',
                lax: '',
                requiredReadonly: 'reqReadonly',
                readonlyLax: '',
                virtual2: true,
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({
              lax: 2,
              required: 2,
              requiredReadonly: 2,
              readonlyLax: 2,
              virtual2: 2,
            });
          });

          it('should only trigger post-validators of props that change during updates', async () => {
            const { error } = await Model.update(
              // @ts-expect-error ikr
              { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: 1 },
              {
                lax: true,
                required: 1,
                requiredReadonly: true,
                readonlyLax: true,
                virtual: true,
                virtual2: true,
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({
              lax: 2,
              virtual: 2,
              virtual2: 2,
            });
          });

          it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              // @ts-expect-error ikr
              { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: '' },
              { requiredReadonly: true, readonlyLax: true },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({ readonlyLax: 2 });
          });
        });
      });

      describe('behaviour with multiple post-validators', () => {
        const Model = new Schema<any>(
          (b) =>
            b
              .field(
                b
                  .dependent('dependent', ['virtual', 'virtual2'])
                  .default('')
                  .resolve(validator),
              )
              .field(b.lax('lax', ''))
              .field(
                b
                  .required('requiredReadonly')
                  .validate(() => true)
                  .readonly(),
              )
              .field(b.lax('readonlyLax', '').readonly())
              .field(b.required('required').validate(() => true))
              .field(b.virtual('virtual').validate(() => true))
              .field(b.virtual('virtual2').validate(() => true)),
          {
            postValidate: [
              // @ts-expect-error ikr
              makePostValidationConfig([
                'lax',
                'required',
                'requiredReadonly',
                'readonlyLax',
              ]),
              // @ts-expect-error ikr
              makePostValidationConfig(['lax', 'virtual']),
              // @ts-expect-error ikr
              makePostValidationConfig(['virtual', 'virtual2']),
            ],
          },
        ).getModel();

        afterEach(() => {
          ctxStats = {};
          providedfieldsStats = {};
        });

        it('should trigger all post-validators at creation', async () => {
          const { error } = await Model.create(
            {
              required: 'req',
              lax: '',
              requiredReadonly: 'reqReadonly',
              readonlyLax: '',
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({
            lax: 2,
            required: 1,
            requiredReadonly: 1,
            readonlyLax: 1,
          });
        });

        it('should not trigger post-validators of virtuals not provided at creation', async () => {
          const { error } = await Model.create(
            {
              required: 'req',
              lax: '',
              requiredReadonly: 'reqReadonly',
              readonlyLax: '',
              virtual2: true,
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({
            lax: 2,
            required: 1,
            requiredReadonly: 1,
            readonlyLax: 1,
            virtual2: 1,
          });
        });

        it('should only trigger post-validators of props provided during updates', async () => {
          const { error } = await Model.update(
            // @ts-expect-error ikr
            { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: 1 },
            {
              lax: true,
              required: true,
              requiredReadonly: true,
              readonlyLax: true,
              virtual: true,
              virtual2: true,
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({
            lax: 2,
            required: 1,
            virtual: 2,
            virtual2: 1,
          });
        });

        it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
          const { error } = await Model.update(
            // @ts-expect-error ikr
            { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: '' },
            {
              requiredReadonly: true,
              readonlyLax: true,
            },
            {},
          );

          expect(error).toBeNull();
          expect(providedfieldsStats).toEqual({ readonlyLax: 1 });
        });

        describe('behaviour with post-validators that have validator arrays', () => {
          const fields1 = [
              'lax',
              'required',
              'requiredReadonly',
              'readonlyLax',
            ],
            fields2 = ['virtual', 'virtual2'];

          const Model = new Schema<any>(
            (b) =>
              b
                .field(
                  b
                    .dependent('dependent', ['virtual', 'virtual2'])
                    .default('')
                    .resolve(validator),
                )
                .field(b.lax('lax', ''))
                .field(
                  b
                    .required('requiredReadonly')
                    .validate(() => true)
                    .readonly(),
                )
                .field(b.lax('readonlyLax', '').readonly())
                .field(b.required('required').validate(() => true))
                .field(b.virtual('virtual').validate(() => true))
                .field(b.virtual('virtual2').validate(() => true)),
            {
              postValidate: [
                {
                  // @ts-expect-error ikr
                  fields: fields1,
                  validator: [
                    makePostValidator(fields1),
                    makePostValidator(fields1),
                  ],
                },
                // @ts-expect-error ikr
                makePostValidationConfig(['lax', 'virtual']),
                {
                  // @ts-expect-error ikr
                  fields: fields2,
                  validator: [
                    makePostValidator(fields2),
                    makePostValidator(fields2),
                    makePostValidator(fields2),
                  ],
                },
              ],
            },
          ).getModel();

          function makePostValidator(fields: string[]) {
            return (ctx: any) => {
              for (const field of fields) handlePostValidate(field, ctx);
            };
          }

          it('should trigger all post-validators at creation', async () => {
            const { error } = await Model.create(
              {
                required: 'req',
                lax: '',
                requiredReadonly: 'reqReadonly',
                readonlyLax: '',
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({
              lax: 3,
              required: 2,
              requiredReadonly: 2,
              readonlyLax: 2,
            });
          });

          it('should not trigger post-validators of virtuals not provided at creation', async () => {
            const { error } = await Model.create(
              {
                required: 'req',
                lax: '',
                requiredReadonly: 'reqReadonly',
                readonlyLax: '',
                virtual2: true,
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({
              lax: 3,
              required: 2,
              requiredReadonly: 2,
              readonlyLax: 2,
              virtual2: 3,
            });
          });

          it('should only trigger post-validators of props provided during updates', async () => {
            const { error } = await Model.update(
              // @ts-expect-error ikr
              { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: 1 },
              {
                lax: true,
                required: true,
                requiredReadonly: true,
                readonlyLax: true,
                virtual: true,
                virtual2: true,
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({
              lax: 3,
              required: 2,
              virtual: 4,
              virtual2: 3,
            });
          });

          it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              // @ts-expect-error ikr
              { lax: 2, required: 1, requiredReadonly: 1, readonlyLax: '' },
              {
                requiredReadonly: true,
                readonlyLax: true,
              },
              {},
            );

            expect(error).toBeNull();
            expect(providedfieldsStats).toEqual({ readonlyLax: 2 });
          });
        });
      });
    });

    describe('values returned from post-validators should be handled accordingly', () => {
      it('should ignore non-object-like values', async () => {
        const values = [-1, 0, 1, '', 'lol', undefined, null, () => {}, []];

        for (const value of values) {
          const Model = new Schema<{ p1: string; p2: string }>(
            (b) => b.field(b.lax('p1', '')).field(b.lax('p2', '')),
            {
              // @ts-expect-error ikr
              postValidate: {
                fields: ['p1', 'p2'],
                validator: () => value,
              },
            },
          ).getModel();

          const { data, error } = await Model.create({}, {});

          expect(error).toBeNull();
          // @ts-expect-error ikr
          expect(data).toEqual({ p1: '', p2: '' });

          const updates = { p1: 'updated', p2: 'updated' };

          const { data: updated, error: error2 } = await Model.update(
            // @ts-expect-error ikr
            data,
            updates,
            {},
          );

          expect(error2).toBeNull();
          // @ts-expect-error ikr
          expect(updated).toEqual(updates);
        }
      });

      it('should respect errors returned in post-validators(sync & async)', async () => {
        type Input = {
          d1?: string;
          p1?: string;
          p2?: string;
          p3?: string;
          p4?: string;
        };
        type Output = {
          p1: string;
          p2: string;
          p3: string;
          p4: string;
          d1: string;
          d2: string;
        };

        const resolver = (ctx: IvoContext<Input, Output>) =>
          ctx.rawInput.d1 ?? ctx.input.d1 ?? ctx.values?.d1;

        const Model = new Schema<Input, Output>(
          (b) =>
            b
              .field(b.lax('p1', ''))
              .field(b.lax('p2', ''))
              .field(b.lax('p3', ''))
              .field(b.lax('p4', ''))
              .field(
                // @ts-expect-error ikr
                b.dependent('d1', 'v').default('').resolve(resolver),
              )
              .field(
                // @ts-expect-error ikr
                b.dependent('d2', 'v').default('').resolve(resolver),
              )
              .field(
                b
                  .virtual('v')
                  .alias('d1')
                  .validate(() => true),
              ),
          {
            postValidate: [
              {
                // @ts-expect-error ikr
                fields: ['p1', 'v'],
                // @ts-expect-error ikr
                validator(ctx) {
                  const v = ctx.rawInput.d1 ?? ctx.input.d1;
                  const isUpdate = ctx.isUpdate;
                  if (v === 'allow') return;

                  if (v === 'throw') throw new Error('lol');

                  return isUpdate
                    ? { d1: 'lolz' }
                    : {
                        p2: 'p2',
                        p3: 'error1',
                        p4: null,
                        v: { reason: 'error', metadata: { lol: true } },
                      };
                },
              },
              {
                fields: ['p1', 'p2'],
                // @ts-expect-error ikr
                validator: (ctx) => {
                  const v = ctx.rawInput.d1 ?? ctx.input.d1;
                  if (v === 'throw') throw new Error('lol');

                  return Promise.resolve(
                    v === 'allow' ? false : { p1: 'failed to validate' },
                  );
                },
              },
            ],
          },
        ).getModel();

        const res = await Model.create({ p2: 'x' }, {});

        expect(res.data).toBeNull();
        expect(res.error).toMatchObject({
          p1: expect.objectContaining({
            reason: 'failed to validate',
          }),
          // p2: expect.objectContaining({
          //   reason: "p2",
          // }),
          // p3: expect.objectContaining({
          //   reason: "error1",
          // }),
          // p4: expect.objectContaining({
          //   reason: "validation failed",
          // }),
          // v: expect.objectContaining({
          //   reason: "error",
          //   metadata: { lol: true },
          // }),
        });

        // @ts-expect-error ikr
        const res2 = await Model.update({}, { p2: 'updated', d1: 'updated' });
        expect(res2.data).toBeNull();
        expect(res2.error?.payload).toMatchObject({
          p1: expect.objectContaining({ reason: 'failed to validate' }),
          d1: expect.objectContaining({ reason: 'lolz' }),
        });

        const res3 = await Model.create({ d1: 'allow' }, {});
        expect(res3.error).toBeNull();
        expect(res3.data).toEqual({
          p1: '',
          p2: '',
          p3: '',
          p4: '',
          d1: 'allow',
          d2: 'allow',
        });

        // @ts-expect-error ikr
        const res4 = await Model.update(res3.data, {
          p1: 'data',
          d1: 'allow',
        });
        expect(res4.error).toBeNull();
        expect(res4.data).toEqual({ p1: 'data' });

        const res5 = await Model.create(
          {
            p1: 'provided',
            p2: 'provided',
            p4: 'provided',
            d1: 'throw',
          },
          {},
        );
        expect(res5.data).toBeNull();
        // @ts-expect-error ikr
        expect(Object.keys(res5.error).length).toBe(3);
        expect(res5.error).toMatchObject({
          p1: expect.objectContaining({ reason: 'validation failed' }),
          p2: expect.objectContaining({ reason: 'validation failed' }),
          d1: expect.objectContaining({ reason: 'validation failed' }),
        });

        const res6 = await Model.update(
          // @ts-expect-error ikr
          {},
          { p1: 'updated', p2: 'updated', d1: 'throw' },
          {},
        );
        expect(res6.data).toBeNull();
        // @ts-expect-error ikr
        expect(Object.keys(res6.error?.payload).length).toBe(3);
        expect(res6.error?.payload).toEqual({
          // @ts-expect-error ikr
          d1: expect.objectContaining({
            reason: 'validation failed',
          }),
          p1: expect.objectContaining({
            reason: 'validation failed',
          }),
          p2: expect.objectContaining({
            reason: 'validation failed',
          }),
        });

        const res7 = await Model.create({ d1: 'throw' }, {});
        expect(res7.data).toBeNull();
        // @ts-expect-error ikr
        expect(Object.keys(res7.error).length).toBe(1);
        expect(res7.error).toMatchObject({
          d1: expect.objectContaining({ reason: 'validation failed' }),
        });

        // @ts-expect-error ikr
        const res8 = await Model.update({}, { d1: 'throw' });
        expect(res8.data).toBeNull();
        // @ts-expect-error ikr
        expect(Object.keys(res8.error?.payload).length).toBe(1);
        expect(res8.error?.payload).toEqual({
          // @ts-expect-error ikr
          d1: expect.objectContaining({ reason: 'validation failed' }),
        });
      });

      it('should properly update revalidated values returned from post-validators', async () => {
        const Model = new Schema<{ p1: string; p2: string }>(
          (b) => b.field(b.lax('p1', '')).field(b.lax('p2', '')),
          {
            // @ts-expect-error ikr
            postValidate: {
              fields: ['p1', 'p2'],
              validator: ({ isUpdate }) =>
                isUpdate
                  ? {
                      p1: { validated: 're updated' },
                      p2: { validated: 'also re updated' },
                    }
                  : {
                      p1: { validated: true },
                      p2: { validated: 'also revalidated' },
                    },
            },
          },
        ).getModel();

        const { data, error } = await Model.create({ p1: '', p2: '' }, {});

        expect(error).toBeNull();
        // @ts-expect-error ikr
        expect(data).toEqual({ p1: true, p2: 'also revalidated' });

        const updates = { p1: 'updated', p2: 'updated' };

        const { data: updated, error: error2 } = await Model.update(
          // @ts-expect-error ikr
          data,
          updates,
          {},
        );

        expect(error2).toBeNull();
        expect(updated).toEqual({
          // @ts-expect-error ikr
          p1: 're updated',
          p2: 'also re updated',
        });
      });

      it('should properly update revalidated values returned from post-validators with virtuals', async () => {
        const Model = new Schema<
          { p1: string; p2: string; v: string; v1: string },
          { dependent: string; p1: string; p2: string }
        >(
          (b) =>
            b
              .field(b.lax('p1', ''))
              .field(b.lax('p2', ''))
              .field(
                b
                  .dependent('dependent', ['v', 'v1'])
                  .default('')
                  .resolve((ctx) => {
                    const v = ctx.input.v ?? ctx.rawInput.v ?? '';
                    const v1 = ctx.input.v1 ?? ctx.rawInput.v1 ?? '';
                    return `${v} ${v1}`.trim();
                  }),
              )
              .field(b.virtual('v').validate(() => true))
              .field(b.virtual('v1').validate(() => true)),

          {
            // @ts-expect-error ikr
            postValidate: {
              fields: ['v', 'v1'],
              validator: ({ isUpdate }) =>
                isUpdate
                  ? {
                      v: { validated: 're updated' },
                      v1: { validated: 'also re updated' },
                    }
                  : {
                      v: { validated: true },
                      v1: { validated: 'also revalidated' },
                    },
            },
          },
        ).getModel();

        // @ts-expect-error ikr
        const { data, error } = await Model.create({ v1: false });

        expect(error).toBeNull();
        expect(data).toMatchObject({ dependent: 'true also revalidated' });

        const updates = { v: true };

        const { data: updated, error: error2 } = await Model.update(
          // @ts-expect-error ikr
          data,
          updates,
          {},
        );

        expect(error2).toBeNull();
        expect(updated).toEqual({
          dependent: 're updated also re updated',
        });
      });

      it('should not revalidate props not related to validator', async () => {
        type Input = { p1?: string; p2: string; v?: string; v1?: string };
        type Output = { p1: string; p2: string; dependent: string };

        const Model = new Schema<Input, Output>(
          (b) =>
            b
              .field(b.lax('p1', ''))
              .field(b.lax('p2', ''))
              .field(
                b
                  .dependent('dependent', ['v', 'v1'])
                  .default('')
                  .resolve((ctx) => {
                    const v = ctx.input.v ?? ctx.rawInput.v ?? '';
                    const v1 = ctx.input.v1 ?? ctx.rawInput.v1 ?? '';
                    return `${v} ${v1}`.trim();
                  }),
              )
              .field(b.virtual('v').validate(() => true))
              .field(b.virtual('v1').validate(() => true)),
          {
            postValidate: [
              {
                fields: ['v', 'v1'],
                // @ts-expect-error ikr
                validator: ({ isUpdate }) =>
                  isUpdate
                    ? {
                        v: { validated: 're updated' },
                        v1: { validated: 'also re updated' },
                      }
                    : {
                        v: { validated: true },
                        v1: { validated: 'also revalidated' },
                      },
              },
              {
                fields: ['p1', 'p2', 'v'],
                // @ts-expect-error ikr
                validator: ({ isUpdate }) =>
                  isUpdate
                    ? {
                        p2: { validated: 're updated' },
                        v: { validated: 'successfully re updated' },
                        v1: { validated: 'also re updated' },
                      }
                    : {
                        p1: { validated: true },
                        p2: { validated: 'also revalidated' },
                        v1: { validated: 'also revalidated' },
                      },
              },
            ],
          },
        ).getModel();

        const { data, error } = await Model.create({ p1: 'provided' }, {});

        expect(error).toBeNull();
        // @ts-expect-error ikr
        expect(data).toEqual({
          dependent: '',
          p1: true,
          p2: 'also revalidated',
        });

        const updates = { p1: false };

        const { data: updated, error: error2 } = await Model.update(
          // @ts-expect-error ikr
          data,
          updates,
          {},
        );

        expect(error2).toBeNull();
        // @ts-expect-error ikr
        expect(updated).toEqual({
          p1: false,
          p2: 're updated',
          dependent: 'successfully re updated',
        });
      });

      describe('behaviour with validator array', () => {
        it('should ignore non-object-like values', async () => {
          const values = [-1, 0, 1, '', 'lol', undefined, null, () => {}, []];

          for (const value of values) {
            const Model = new Schema<any>(
              (b) => b.field(b.lax('p1', '')).field(b.lax('p2', '')),
              {
                // @ts-expect-error ikr
                postValidate: {
                  fields: ['p1', 'p2'],
                  validator: [() => value, () => value],
                },
              },
            ).getModel();

            const { data, error } = await Model.create({}, {});

            expect(error).toBeNull();
            // @ts-expect-error ikr
            expect(data).toEqual({ p1: '', p2: '' });

            const updates = { p1: 'updated', p2: 'updated' };

            const { data: updated, error: error2 } = await Model.update(
              // @ts-expect-error ikr
              data,
              updates,
              {},
            );

            expect(error2).toBeNull();
            // @ts-expect-error ikr
            expect(updated).toEqual(updates);
          }
        });

        it('should process errors of first validator to return errors and stop validating', async () => {
          type PVInput = {
            d1?: any;
            p1?: string;
            p2?: string;
            p3?: string;
            p4?: string;
          };
          type PVOutput = {
            d1: string;
            d2: string;
            p1: string;
            p2: string;
            p3: string;
            p4: string;
          };

          const resolver = (ctx: IvoContext<PVInput, PVOutput>) =>
            ctx.input.d1 ?? ctx.rawInput.d1;

          let validatorRunCount: Record<string, number> = {};

          function incrementValidatorCount(key: string) {
            validatorRunCount[key] = (validatorRunCount?.[key] ?? 0) + 1;
          }

          const Model = new Schema<PVInput, PVOutput>(
            (b) =>
              b
                .field(b.lax('p1', ''))
                .field(b.lax('p2', ''))
                .field(b.lax('p3', ''))
                .field(b.lax('p4', ''))
                .field(b.dependent('d1', 'v').default('').resolve(resolver))
                .field(b.dependent('d2', 'v').default('').resolve(resolver))
                .field(
                  b
                    .virtual('v')
                    .alias('d1')
                    .validate(() => true),
                ),
            {
              postValidate: [
                {
                  // @ts-expect-error ikr
                  fields: ['p1', 'v'],
                  validator: [
                    () => {
                      incrementValidatorCount('p1-v');
                    },
                    (ctx) => {
                      const v = ctx.input.d1 ?? ctx.rawInput.d1;
                      if (v === 'return error') return { d1: 'error returned' };
                    },
                    (ctx) => {
                      const v = ctx.input.d1 ?? ctx.rawInput.d1;
                      incrementValidatorCount('p1-v');
                      if (v === 'throw') throw new Error('lol');
                    },
                    // @ts-expect-error ikr
                    (ctx) => {
                      const isUpdate = ctx.isUpdate;
                      incrementValidatorCount('p1-v');

                      return isUpdate
                        ? { d1: 'lolz' }
                        : {
                            p1: 'p1',
                            p2: 'p2',
                            p3: 'error1',
                            p4: null,
                            d1: { reason: 'error', metadata: { lol: true } },
                          };
                    },
                  ],
                },
                {
                  fields: ['p1', 'p2'],
                  validator: [
                    (ctx) => {
                      const v = ctx.input.d1 ?? ctx.rawInput.d1;
                      incrementValidatorCount('p1-p2');
                      if (v === 'throw') throw new Error('lol');
                    },
                    // @ts-expect-error ikr
                    (ctx) => {
                      const v = ctx.input.d1 ?? ctx.rawInput.d1;
                      return Promise.resolve(
                        v === 'allow' ? false : { p1: 'failed to validate' },
                      );
                    },
                  ],
                },
              ],
            },
          ).getModel();

          // const createRes = await Model.create({},{});

          // expect(createRes.data).toBeNull();
          // expect(createRes.error?.payload).toMatchObject({
          //   // p1: expect.objectContaining({ reason: "failed to validate" }),
          //   // // p2: expect.objectContaining({ reason: "p2" }),
          //   // // p3: expect.objectContaining({ reason: "error1" }),
          //   // // p4: expect.objectContaining({ reason: "validation failed" }),
          //   // d1: expect.objectContaining({
          //   //   reason: "error",
          //   //   metadata: { lol: true },
          //   // }),
          // });

          // expect(validatorRunCount).toMatchObject({
          //   "p1-v": 3,
          //   "p1-p2": 2,
          // });

          // validatorRunCount = {};

          // @ts-expect-error ikr
          const createRes1 = await Model.create({ d1: 'throw', p2: true }, {});

          expect(createRes1.data).toBeNull();
          expect(createRes1.error).toEqual({
            // @ts-expect-error ikr
            p2: expect.objectContaining({ reason: 'validation failed' }),
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 2,
            'p1-p2': 1,
          });

          validatorRunCount = {};

          const createRes11 = await Model.create({ d1: 'return error' }, {});

          expect(createRes11.data).toBeNull();
          expect(createRes11.error).toMatchObject({
            d1: expect.objectContaining({ reason: 'error returned' }),
          });

          expect(validatorRunCount).toEqual({ 'p1-v': 1 });

          validatorRunCount = {};

          const updateRes = await Model.update(
            // @ts-expect-error ikr
            {},
            { p2: 'updated', d1: 'updated' },
            {},
          );
          expect(updateRes.data).toBeNull();
          expect(updateRes.error?.payload).toMatchObject({
            p1: expect.objectContaining({ reason: 'failed to validate' }),
            d1: expect.objectContaining({ reason: 'lolz' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 3,
            'p1-p2': 1,
          });

          validatorRunCount = {};

          const updateRes1 = await Model.update(
            // @ts-expect-error ikr
            {},
            { p2: 'updated', d1: 'throw' },
            {},
          );

          expect(updateRes1.data).toBeNull();
          expect(updateRes1.error?.payload).toMatchObject({
            p2: expect.objectContaining({ reason: 'validation failed' }),
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 2,
            'p1-p2': 1,
          });

          validatorRunCount = {};

          // @ts-expect-error ikr
          const updateRes2 = await Model.update({}, { d1: 'return error' });

          expect(updateRes2.data).toBeNull();
          expect(updateRes2.error?.payload).toMatchObject({
            d1: expect.objectContaining({ reason: 'error returned' }),
          });

          expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });
        });

        it('should process parallel and sequential validators accordingly', async () => {
          type PVInput2 = {
            d1?: any;
            p1?: string;
            p2?: string;
            p3?: string;
            p4?: string;
          };
          type PVOutput2 = {
            d1: string;
            d2: string;
            p1: string;
            p2: string;
            p3: string;
            p4: string;
          };
          type PVCtx2 = ReadonlyIvoContext<PVInput2, PVOutput2>;

          const resolver = (ctx: PVCtx2) => ctx.input.d1 ?? ctx.rawInput.d1;

          let validatorRunCount: Record<string, number> = {};

          function incrementValidatorCount(key: string) {
            validatorRunCount[key] = (validatorRunCount?.[key] ?? 0) + 1;
          }

          const Model = new Schema<PVInput2, PVOutput2>(
            (b) =>
              b
                .field(b.dependent('d1', 'v').default('').resolve(resolver))
                .field(b.dependent('d2', 'v').default('').resolve(resolver))
                .field(b.lax('p1', ''))
                .field(b.lax('p2', ''))
                .field(b.lax('p3', ''))
                .field(b.lax('p4', ''))
                .field(b.virtual('v').alias('d1').validate(validator)),
            {
              postValidate: [
                {
                  // @ts-expect-error ikr
                  fields: ['p1', 'v'],
                  validator: [
                    (ctx) => {
                      const v = ctx.input.d1 ?? ctx.rawInput.d1;
                      incrementValidatorCount('p1-v');

                      if (v === 'return error') return { d1: 'error returned' };

                      if (v === 'throw') throw new Error('lol');
                    },
                    [
                      (ctx) => {
                        const v = ctx.input.d1 ?? ctx.rawInput.d1;
                        incrementValidatorCount('p1-v-parallel');

                        if (v === 'throw-parallel') throw new Error('lol');
                      },
                      (ctx) => {
                        const v = ctx.input.d1 ?? ctx.rawInput.d1;
                        incrementValidatorCount('p1-v-parallel');

                        if (v === 'return error-parallel')
                          return { d1: 'error returned' };
                      },
                    ],
                    () => {
                      incrementValidatorCount('p1-v');
                    },
                    [
                      () => {
                        incrementValidatorCount('p1-v-parallel-1');
                      },
                      () => {
                        incrementValidatorCount('p1-v-parallel-1');
                      },
                    ],
                  ],
                },
              ],
            },
          ).getModel();

          const createRes = await Model.create({}, {});

          expect(createRes.error).toBeNull();
          expect(validatorRunCount).toMatchObject({});

          const createRes0 = await Model.create({ d1: '' }, {});

          expect(createRes0.error).toBeNull();
          expect(validatorRunCount).toMatchObject({
            'p1-v': 2,
            'p1-v-parallel': 2,
            'p1-v-parallel-1': 2,
          });

          validatorRunCount = {};

          // @ts-expect-error ikr
          const createRes1 = await Model.create({ d1: 'throw', p2: true }, {});

          expect(createRes1.data).toBeNull();
          expect(createRes1.error).toMatchObject({
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

          validatorRunCount = {};

          const createRes2 = await Model.create({ d1: 'return error' }, {});

          expect(createRes2.data).toBeNull();
          expect(createRes2.error).toMatchObject({
            d1: expect.objectContaining({ reason: 'error returned' }),
          });

          expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

          validatorRunCount = {};

          const createRes3 = await Model.create(
            {
              d1: 'throw-parallel',
              // @ts-expect-error ikr
              p2: true,
            },
            {},
          );

          expect(createRes3.data).toBeNull();
          expect(createRes3.error).toMatchObject({
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 1,
            'p1-v-parallel': 2,
          });

          validatorRunCount = {};

          const createRes4 = await Model.create(
            {
              d1: 'return error-parallel',
              // @ts-expect-error ikr
              p2: true,
            },
            {},
          );

          expect(createRes4.data).toBeNull();
          expect(createRes4.error).toMatchObject({
            d1: expect.objectContaining({ reason: 'error returned' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 1,
            'p1-v-parallel': 2,
          });

          validatorRunCount = {};

          // @ts-expect-error ikr
          const updateRes = await Model.update({}, { d1: 'valid' });
          expect(updateRes.error).toBeNull();

          expect(validatorRunCount).toMatchObject({
            'p1-v': 2,
            'p1-v-parallel': 2,
            'p1-v-parallel-1': 2,
          });

          validatorRunCount = {};

          // @ts-expect-error ikr
          const updateRes1 = await Model.update({}, { d1: 'throw' });
          expect(updateRes1.data).toBeNull();
          expect(updateRes1.error?.payload).toMatchObject({
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

          validatorRunCount = {};

          // @ts-expect-error ikr
          const updateRes2 = await Model.update({}, { d1: 'return error' }, {});

          expect(updateRes2.data).toBeNull();
          expect(updateRes2.error?.payload).toMatchObject({
            d1: expect.objectContaining({ reason: 'error returned' }),
          });

          expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

          validatorRunCount = {};

          const updateRes3 = await Model.update(
            // @ts-expect-error ikr
            {},
            { d1: 'throw-parallel', p2: true },
            {},
          );

          expect(updateRes3.data).toBeNull();
          expect(updateRes3.error?.payload).toMatchObject({
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 1,
            'p1-v-parallel': 2,
          });

          validatorRunCount = {};

          const updateRes4 = await Model.update(
            // @ts-expect-error ikr
            {},
            { d1: 'return error-parallel', p2: true },
            {},
          );

          expect(updateRes4.data).toBeNull();
          expect(updateRes4.error?.payload).toMatchObject({
            d1: expect.objectContaining({ reason: 'error returned' }),
          });

          expect(validatorRunCount).toMatchObject({
            'p1-v': 1,
            'p1-v-parallel': 2,
          });
        });
      });
    });

    describe('behaviour when updating ctxOptions from within post-validators', () => {
      let ctxValue: Record<string, unknown> = {};

      beforeEach(() => {
        ctxValue = {};
      });

      type Data = { p1: string; p2: string };

      const Model = new Schema<Data>(
        (b) => b.field(b.lax('p1', '')).field(b.lax('p2', '')),
        {
          postValidate: {
            fields: ['p1', 'p2'],
            validator: ({ updateOptions }) => {
              updateOptions({ updated: true });

              return true;
            },
          },
          onSuccess({ options }: ReadonlyIvoContext<Data>) {
            ctxValue = options;
          },
        },
      ).getModel();

      it('should respect ctx updates at creation', async () => {
        expect(ctxValue).toEqual({});

        const { handleSuccess } = await Model.create({}, {});

        await handleSuccess?.();

        expect(ctxValue).toEqual({});

        {
          const { handleSuccess } = await Model.create({ p1: '1' }, {});

          await handleSuccess?.();

          expect(ctxValue).toEqual({ updated: true });
        }
      });

      it('should respect ctx updates during updates', async () => {
        expect(ctxValue).toEqual({});

        const { handleSuccess } = await Model.update(
          { p1: '', p2: '3' },
          // @ts-expect-error ikr
          { p1: true },
          { initial: true },
        );

        await handleSuccess?.();

        expect(ctxValue).toEqual({ updated: true, initial: true });
      });

      describe('behaviour with validator array', () => {
        let ctxValue: Record<string, unknown> = {};

        beforeEach(() => {
          ctxValue = {};
        });
        type Data = { p1: string; p2: string };
        const Model = new Schema<Data, Data, Record<string, unknown>>(
          (b) => b.field(b.lax('p1', '')).field(b.lax('p2', '')),
          {
            postValidate: {
              fields: ['p1', 'p2'],
              validator: [
                ({ updateOptions }) => {
                  updateOptions({ updated: true });

                  return true;
                },
                ({ updateOptions, options: { updated } }) => {
                  updateOptions({ v2: { updated } });

                  return true;
                },
              ],
            },
            onSuccess({ options }: ReadonlyIvoContext<Data>) {
              ctxValue = options;
            },
          },
        ).getModel();

        it('should respect ctx updates at creation', async () => {
          expect(ctxValue).toEqual({});

          const { handleSuccess } = await Model.create({}, {});

          await handleSuccess?.();

          expect(ctxValue).toEqual({});

          {
            const { handleSuccess } = await Model.create({ p1: '1' }, {});

            await handleSuccess?.();
            expect(ctxValue).toEqual({
              updated: true,
              v2: { updated: true },
            });
          }
        });

        it('should respect ctx updates during updates', async () => {
          expect(ctxValue).toEqual({});

          const { handleSuccess } = await Model.update(
            { p1: '', p2: '3' },
            // @ts-expect-error ikr
            { p1: true },
            { initial: true },
          );

          await handleSuccess?.();

          expect(ctxValue).toEqual({
            updated: true,
            initial: true,
            v2: { updated: true },
          });
        });
      });
    });
  });
});
