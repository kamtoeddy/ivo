import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { expectFailure, expectNoFailure, makeFx, validator } from '../_utils';

describe('allowed values', () => {
  describe('valid', () => {
    it('should not reject if allowed values provided are >= 2', () => {
      const values = [
        ['lol', 2],
        ['lol', 2, 3],
      ];

      for (const allow of values) {
        const toPass = makeFx((b) =>
          b.field(b.lax('field', allow[0]).allow(allow as never)),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should not reject if default value provided is an allowed value', () => {
      const toPass = makeFx((b) =>
        b.field(b.lax('field', null).allow([null, 'lolz', -1])),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow virtuals to have allowed values', () => {
      const toPass = makeFx((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtual')
              .default(true)
              .resolve(validator as never),
          )

          .field(b.virtual('virtual').allow([null, 'lolz', -1])),
      );

      expectNoFailure(toPass);

      toPass();
    });

    describe('allow as an object', () => {
      it('should not reject if "values" is the only key provided', () => {
        const toPass = makeFx((b) =>
          b.field(
            b
              .lax('dependent', null)
              .allow({ values: [null, 'lolz', -1] } as never),
          ),
        );

        expectNoFailure(toPass);

        toPass();
      });

      it('should not reject if "values" & "error" are both provided', () => {
        const toPass = makeFx((b) =>
          b.field(
            b.lax('dependent', null).allow({
              error: 'value not allowed',
              values: [null, 'lolz', -1],
            } as never),
          ),
        );

        expectNoFailure(toPass);

        toPass();
      });

      // "should not reject if validator is provided" and "should not
      // reject with errors valid formats" discarded: the builder makes
      // allow()+validate() together structurally unrepresentable - allow()
      // is the field's primary validator when provided (same reason as the
      // "behaviour" section below).
    });
  });

  describe('invalid', () => {
    it('should reject if non-array value is provided', () => {
      const values = [
        null,
        undefined,
        new Number(),
        new String(),
        Symbol(),
        2,
        -10,
        true,
        () => {},
        {},
      ];

      for (const allow of values) {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', null).allow(allow as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['Allowed values must be an array'],
          });
        }
      }
    });

    it('should reject if allowed values provided are not unique', () => {
      const values = [
        [1, 2, 2, 4, 5],
        ['lol', 59, 'lol', null],
        [true, false, true],
        [{}, {}],
        [{ id: 'lol' }, { id: 'lol' }],
      ];

      for (const allow of values) {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', null).allow(allow as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['Allowed values must be an array of unique values'],
          });
        }
      }
    });

    it('should reject if allowed values provided are less than 2', () => {
      const values = [[], ['lol']];

      for (const allow of values) {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', null).allow(allow as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['Allowed values must have at least 2 values'],
          });
        }
      }
    });

    it('should reject if default value provided is not an allowed value', () => {
      const values = [
        ['lol', [null, 'lolz', -1]],
        [null, [1, 4, 'lol', undefined]],
      ];

      for (const [_default, allow] of values) {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', _default).allow(allow as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['The default value must be an allowed value'],
          });
        }
      }
    });

    describe('allow as an object', () => {
      it('should reject if values array is not provided', () => {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', null).allow({} as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['Allowed values must be an array'],
          });
        }
      });

      it('should reject if non-array value is provided', () => {
        const invalidValues = [
          null,
          undefined,
          new Number(),
          new String(),
          Symbol(),
          2,
          -10,
          true,
          () => {},
          {},
        ];

        for (const values of invalidValues) {
          const toFail = makeFx((b) =>
            b.field(b.lax('field', null).allow({ values } as never)),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              field: ['Allowed values must be an array'],
            });
          }
        }
      });

      it('should reject if allowed values provided are not unique', () => {
        const invalidValues = [
          [1, 2, 2, 4, 5],
          ['lol', 59, 'lol', null],
          [true, false, true],
          [{}, {}],
          [{ id: 'lol' }, { id: 'lol' }],
        ];

        for (const values of invalidValues) {
          const toFail = makeFx((b) =>
            b.field(b.lax('field', null).allow({ values } as never)),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              field: ['Allowed values must be an array of unique values'],
            });
          }
        }
      });

      it('should reject if allowed values provided are less than 2', () => {
        const invalidValues = [[], ['lol']];

        for (const values of invalidValues) {
          const toFail = makeFx((b) =>
            b.field(b.lax('field', null).allow({ values } as never)),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              field: ['Allowed values must have at least 2 values'],
            });
          }
        }
      });

      it('should reject if default value provided is not an allowed value', () => {
        const data = [
          ['lol', [null, 'lolz', -1]],
          [null, [1, 4, 'lol', undefined]],
        ];

        for (const [_default, values] of data) {
          const toFail = makeFx((b) =>
            b.field(b.lax('field', _default).allow({ values } as never)),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              field: ['The default value must be an allowed value'],
            });
          }
        }
      });

      it('should reject if error is provided and is of invalid type', () => {
        const errors = [
          null,
          true,
          false,
          {},
          { key: 'value' },
          -1,
          0,
          1,
          [],
          [[], null, true, false, {}, { key: 'value' }, -1, 0, 1],
        ];

        for (const error of errors) {
          const toFail = makeFx((b) =>
            b.field(
              b
                .lax('field', null)
                .allow({ error, values: [null, 'lolz', -1] } as never),
            ),
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              field: [
                'The "error" field of the allow rule can only accept a string, InputFieldError or an function that returns any of the above mentioned',
              ],
            });
          }
        }
      });

      it('should reject if an invalid config key is passed', () => {
        const toFail = makeFx((b) =>
          b.field(
            b
              .lax('field', null)
              .allow({ key: 'value', values: [null, 'lolz', -1] } as never),
          ),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: [
              'The "allow" rule only accepts "error" & "values" as configuration. Please remove the extra keys',
            ],
          });
        }
      });
    });
  });

  describe('behaviour', () => {
    const metadata = { allowed: [null, 'allowed'] };

    describe('behaviour with lax props & no validators', () => {
      const Model = new Schema<any>((b) =>
        b.field(b.lax('field', null).allow(metadata.allowed as never)),
      ).getModel();

      describe('creation', () => {
        it('should allow if value provided is allowed', async () => {
          const { data, error } = await Model.create({ field: 'allowed' }, {});

          expect(error).toBeNull();
          expect(data).toMatchObject({ field: 'allowed' });
        });

        it('should reject if value provided is not allowed', async () => {
          const { data, error } = await Model.create({ field: true }, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });

      describe('updates', () => {
        it('should allow if value provided is allowed', async () => {
          const { data, error } = await Model.update(
            { field: 'allowed' },
            { field: null },
            {},
          );

          expect(error).toBeNull();
          expect(data).toMatchObject({ field: null });
        });

        it('should reject if value provided is not allowed', async () => {
          const { data, error } = await Model.update(
            { field: null },
            { field: true },
            {},
          );

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });
    });

    // "behaviour with lax props & validators" discarded: the builder makes
    // allow()+validate() together (as independent, both-applied checks)
    // structurally unrepresentable - allow() is the field's primary
    // validator when provided.

    describe('behaviour with required props & no validators', () => {
      const Model = new Schema<any>((b) =>
        b.field(b.required('field').allow(metadata.allowed as never)),
      ).getModel();

      describe('creation', () => {
        it('should accept allowed values if provided', async () => {
          const { data, error } = await Model.create({ field: null }, {});

          expect(error).toBeNull();
          expect(data).toEqual({ field: null });
        });

        it('should reject non-allowed values if provided', async () => {
          const { data, error } = await Model.create({ field: 'lolz' }, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });

        it('should reject if no value is provided', async () => {
          const { data, error } = await Model.create({}, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });

      describe('updates', () => {
        it('should accept allowed values if provided', async () => {
          const { data, error } = await Model.update(
            { field: 'allowed' },
            { field: null },
            {},
          );

          expect(error).toBeNull();
          expect(data).toEqual({ field: null });
        });

        it('should reject non-allowed values if provided', async () => {
          const { data, error } = await Model.update(
            { field: 'allowed' },
            { field: 'whatever' },
            {},
          );

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });
    });

    // "behaviour with required props & validators" discarded: same
    // allow()+validate() mutual-exclusion reason as above.

    // "behaviour with virtuals" and "behaviour with virtuals & alias"
    // discarded: same allow()+validate() mutual-exclusion reason as above
    // (a virtual's validator is the synthesized passthrough when only
    // allow() is used, so there's no separate custom validator to also
    // "respect").

    describe('allow as an object', () => {
      describe('behaviour with lax props & no validators', () => {
        const Model = new Schema<any>((b) =>
          b.field(b.lax('field', null).allow(metadata.allowed as never)),
        ).getModel();

        describe('creation', () => {
          it('should allow if value provided is allowed', async () => {
            const { data, error } = await Model.create(
              { field: 'allowed' },
              {},
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ field: 'allowed' });
          });

          it('should reject if value provided is not allowed', async () => {
            const { data, error } = await Model.create({ field: true }, {});

            expect(data).toBeNull();
            expect(error).toMatchObject({
              field: expect.objectContaining({
                reason: 'value not allowed',
                metadata,
              }),
            });
          });
        });

        describe('updates', () => {
          it('should allow if value provided is allowed', async () => {
            const { data, error } = await Model.update(
              { field: 'allowed' },
              { field: null },
              {},
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ field: null });
          });

          it('should reject if value provided is not allowed', async () => {
            const { data, error } = await Model.update(
              { field: null },
              { field: true },
              {},
            );

            expect(data).toBeNull();
            expect(error).toMatchObject({
              field: expect.objectContaining({
                reason: 'value not allowed',
                metadata,
              }),
            });
          });
        });
      });

      // "behaviour with lax props & validators", "behaviour with
      // virtuals", and "behaviour with virtuals & alias" (object-form
      // allow) discarded: same allow()+validate() mutual-exclusion reason
      // as above.

      describe('error', () => {
        describe('error as a string', () => {
          describe('if string is empty', () => {
            const Model = new Schema<any>((b) =>
              b.field(
                b
                  .lax('field', metadata.allowed[0])
                  .allow(metadata.allowed as never)
                  .allowError(''),
              ),
            ).getModel();

            it('should return default error message at creation', async () => {
              const { data, error } = await Model.create(
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining({
                  reason: 'value not allowed',
                }),
              });
            });

            it('should return default error message during updates', async () => {
              const { data, error } = await Model.update(
                { field: metadata.allowed[0] },
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining({
                  reason: 'value not allowed',
                }),
              });
            });
          });

          describe('if string is not empty', () => {
            const errorMessage = 'Value not allowed. lol';

            const Model = new Schema<any>((b) =>
              b.field(
                b
                  .lax('field', metadata.allowed[0])
                  .allow(metadata.allowed as never)
                  .allowError(errorMessage),
              ),
            ).getModel();

            it('should return default error message at creation', async () => {
              const { data, error } = await Model.create(
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining({ reason: errorMessage }),
              });
            });

            it('should return default error message during updates', async () => {
              const { data, error } = await Model.update(
                { field: metadata.allowed[0] },
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining({ reason: errorMessage }),
              });
            });
          });
        });

        describe('error as InputFieldError', () => {
          const errorMessages = [
            [{ reason: 'Invalid lol' }],
            [
              {
                reason: 'failed again',
                metadata: { allowed: metadata.allowed },
              },
            ],
          ];

          for (const [expected] of errorMessages) {
            const Model = new Schema<any>((b) =>
              b.field(
                b
                  .lax('field', metadata.allowed[0])
                  .allow(metadata.allowed as never)
                  .allowError(expected as never),
              ),
            ).getModel();

            it('should return default error message at creation', async () => {
              const { data, error } = await Model.create(
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining(expected),
              });
            });

            it('should return default error message during updates', async () => {
              const { data, error } = await Model.update(
                { field: metadata.allowed[0] },
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining(expected),
              });
            });
          }
        });

        describe('error as function', () => {
          const reason = 'value not allowed';
          const errorMessages = [
            [() => '', { reason }],
            [() => ({}), { reason }],
            [() => null, { reason }],
            [() => undefined, { reason }],
            [() => -1, { reason }],
            [() => 0, { reason }],
            [() => 1, { reason }],
            [() => true, { reason }],
            [() => false, { reason }],
            [() => 'Invalid lol', { reason: 'Invalid lol' }],
            [() => ['invalid as array', 'Invalid lol'], { reason }],
            [
              () => ({ metadata: { valid: false } }),
              { metadata: { valid: false } },
            ],
            [() => ({ reason: 'Invalid lol' }), { reason: 'Invalid lol' }],
            [
              () => ({
                reason: 'failed again',
                metadata: { allowed: metadata.allowed },
              }),
              {
                reason: 'failed again',
                metadata: { allowed: metadata.allowed },
              },
            ],
          ];

          for (const [error, expected] of errorMessages) {
            const Model = new Schema<any>((b) =>
              b.field(
                b
                  .lax('field', metadata.allowed[0])
                  .allow(metadata.allowed as never)
                  .allowError(error as never),
              ),
            ).getModel();

            it('should return default error message at creation', async () => {
              const { data, error } = await Model.create(
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining(expected),
              });
            });

            it('should return default error message during updates', async () => {
              const { data, error } = await Model.update(
                { field: metadata.allowed[0] },
                { field: 'Invalid' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining(expected),
              });
            });
          }

          describe('behaviour with errors thrown in the error setter', () => {
            const Model = new Schema<any>((b) =>
              b.field(
                b
                  .lax('field', 'lol')
                  .allow(['lol', 'lolol'])
                  .allowError(() => {
                    throw new Error('lolol');
                  }),
              ),
            ).getModel();

            it('should return proper errors at creation', async () => {
              const { data, error } = await Model.create({ field: '' }, {});

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining({
                  reason: 'value not allowed',
                }),
              });
            });

            it('should return proper errors during updates', async () => {
              const { data, error } = await Model.update(
                { field: 'lol' },
                { field: '' },
                {},
              );

              expect(data).toBeNull();
              expect(error).toMatchObject({
                field: expect.objectContaining({
                  reason: 'value not allowed',
                }),
              });
            });
          });
        });
      });
    });
  });
});
