import { beforeEach, describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { expectNoFailure, makeFx, validator } from '../_utils';

describe('required', () => {
  describe('valid', () => {
    it('should allow required + validator', () => {
      const toPass = makeFx((b) =>
        b.field(b.required('fieldName').validate(validator)),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow required: true + allow alone', () => {
      const toPass = makeFx((b) =>
        b.field(b.required('fieldName').allow([1, 2, 435, 45])),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow required(true) + readonly(true) (locks the m after creation)', () => {
      const toPass = makeFx((b) =>
        b.field(b.required('fieldName').validate(validator).readonly()),
      );

      expectNoFailure(toPass);

      toPass();
    });
  });

  // "invalid" discarded entirely: "required & no validator" (validate()/
  // allow() is mandatory before [BUILD] is offered), "required(true) +
  // default" (RequiredBuilder never exposes a `.default()` method), and
  // "required(true) + ignoreInit" (RequiredBuilder never exposes an
  // `.ignoreInit()` method) are all structurally unrepresentable through
  // the builder by design.
});

describe('required runtime enforcement (strictly required, i.e. required: true)', () => {
  // Mirrors rs/tests/fields/required/mod.rs::
  // should_respect_the_default_required_error_if_field_is_missing
  const Book = new Schema<any>((b) =>
    b
      .field(b.required('bookId').validate(validator))
      .field(b.lax('isPublished', false).validate(validator)),
  ).getModel();

  it('should reject creation if a strictly required m is missing, with the default message', async () => {
    const { data, error } = await Book.create({ isPublished: true }, {});

    expect(data).toBeNull();
    expect(error).toMatchObject({
      bookId: { reason: "'bookId' is required" },
    });
  });

  it('should create normally once the strictly required m is provided', async () => {
    const { data, error } = await Book.create({ bookId: 1 }, {});

    expect(error).toBeNull();
    expect(data).toEqual({ bookId: 1, isPublished: false });
  });

  it('updates are unaffected by required-ness (a strictly required m may simply be absent from an update)', async () => {
    const { data, error } = await Book.update(
      { bookId: 1, isPublished: false },
      { isPublished: true },
      {},
    );

    expect(error).toBeNull();
    expect(data).toEqual({ isPublished: true });
  });
});

describe('requiredBy', () => {
  describe('behaviour', () => {
    let callsPerField: Record<string, boolean> = {};

    const book = {
      bookId: 1,
      isPublished: false,
      price: null,
      priceReadonly: null,
      priceRequiredWithoutMessage: null,
    };

    function validatePrice(price: unknown) {
      const validated = Number(price),
        // @ts-expect-error isNaN takes a number
        valid = !isNaN(price) && validated;
      return { valid, validated };
    }

    function recordCalls(field: string) {
      callsPerField[field] = true;
    }

    const Book = new Schema<any>((b) =>
      b
        .field(b.required('bookId').validate(validator))
        .field(b.lax('isPublished', false).validate(validator))
        .field(
          b
            .lax('price', null)
            .validate(validatePrice as never)
            .required((ctx: any) => {
              const isPublished =
                ctx.rawInput.isPublished ??
                ctx.input.isPublished ??
                ctx.values.isPublished;
              const price =
                ctx.rawInput.price ?? ctx.input.price ?? ctx.values.price;
              const isRequired = isPublished && price == null;
              recordCalls('price');
              return [isRequired, 'A price is required to publish a book!'];
            }),
        )
        .field(
          b
            .lax('priceReadonly', null)
            .validate(validatePrice as never)
            .readonly()
            .required((ctx: any) => {
              const price =
                ctx.rawInput.price ?? ctx.input.price ?? ctx.values.price;
              const priceReadonly =
                ctx.rawInput.priceReadonly ??
                ctx.input.priceReadonly ??
                ctx.values.priceReadonly;
              const isRequired = price === 101 && priceReadonly == null;
              recordCalls('priceReadonly');
              return [
                isRequired,
                'A priceReadonly is required when price is 101!',
              ];
            }),
        )
        .field(
          b
            .lax('priceRequiredWithoutMessage', null)
            .validate(validatePrice as never)
            .readonly()
            .required((ctx: any) => {
              const price =
                ctx.rawInput.price ?? ctx.input.price ?? ctx.values.price;
              const priceReadonly =
                ctx.rawInput.priceReadonly ??
                ctx.input.priceReadonly ??
                ctx.values.priceReadonly;
              recordCalls('priceRequiredWithoutMessage');
              return price === 101 && priceReadonly == null;
            }),
        ),
    ).getModel();

    beforeEach(() => {
      callsPerField = {};
    });

    describe('creation', () => {
      it('should create normally', async () => {
        const toPass = () => Book.create({ bookId: 1 }, {});

        expectNoFailure(toPass);

        const { data } = await toPass();

        expect(data).toEqual(book);
        expect(callsPerField).toEqual({
          price: true,
          priceReadonly: true,
          priceRequiredWithoutMessage: true,
        });
      });

      it('should pass if condition is met at creation', async () => {
        const toPass = () =>
          Book.create({ bookId: 1, isPublished: true, price: 2000 }, {});

        expectNoFailure(toPass);

        const { data } = await toPass();

        expect(data).toEqual({
          bookId: 1,
          isPublished: true,
          price: 2000,
          priceReadonly: null,
          priceRequiredWithoutMessage: null,
        });
        // `price` was provided (relevant), so its `required` callback is
        // skipped entirely — only ms that weren't given a value get
        // evaluated for required-ness.
        expect(callsPerField).toEqual({
          priceReadonly: true,
          priceRequiredWithoutMessage: true,
        });
      });

      it('should reject if condition is not met at creation', async () => {
        const { data, error } = await Book.create(
          {
            bookId: 1,
            isPublished: true,
          },
          {},
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({
          price: {
            reason: 'A price is required to publish a book!',
            metadata: null,
          },
        });

        expect(callsPerField).toEqual({
          price: true,
          priceReadonly: true,
          priceRequiredWithoutMessage: true,
        });
      });
    });

    describe('updates', () => {
      it('should pass if condition is met during updates', async () => {
        const toPass = () =>
          Book.update(
            {
              bookId: 1,
              isPublished: false,
              price: null,
              priceReadonly: null,
              priceRequiredWithoutMessage: null,
            },
            { isPublished: true, price: 20 },
            {},
          );

        expectNoFailure(toPass);

        const { data } = await toPass();

        expect(data).toEqual({ isPublished: true, price: 20 });
        // `price` was provided (relevant), so skipped.
        expect(callsPerField).toEqual({
          priceReadonly: true,
          priceRequiredWithoutMessage: true,
        });
      });

      it('should pass if condition is met during updates of readonly', async () => {
        const toPass = () =>
          Book.update(book, { price: 101, priceReadonly: 201 }, {});

        expectNoFailure(toPass);

        const { data } = await toPass();

        expect(data).toEqual({ price: 101, priceReadonly: 201 });
        // `price` and `priceReadonly` were both provided, so skipped.
        expect(callsPerField).toEqual({
          priceRequiredWithoutMessage: true,
        });
      });

      it('should reject if condition is not met during updates', async () => {
        const { data, error } = await Book.update(
          {
            bookId: 1,
            isPublished: false,
            price: null,
            priceReadonly: null,
          },
          { isPublished: true },
          {},
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({
          price: {
            reason: 'A price is required to publish a book!',
            metadata: null,
          },
        });

        expect(callsPerField).toEqual({ price: true, priceReadonly: true });
      });

      it('should reject if condition is not met during updates of readonly', async () => {
        const { data, error } = await Book.update(book, { price: 101 }, {});

        expect(data).toBeNull();
        expect(error).toMatchObject({
          priceReadonly: {
            reason: 'A priceReadonly is required when price is 101!',
            metadata: null,
          },
          priceRequiredWithoutMessage: {
            reason: "'priceRequiredWithoutMessage' is required",
            metadata: null,
          },
        });

        // `price` was provided, so skipped; `priceReadonly` and
        // `priceRequiredWithoutMessage` weren't, so both get evaluated.
        expect(callsPerField).toEqual({
          priceReadonly: true,
          priceRequiredWithoutMessage: true,
        });
      });

      it('should not update callable readonly field that has changed', async () => {
        const { data, error } = await Book.update(
          {
            bookId: 1,
            isPublished: false,
            price: null,
            priceReadonly: 3000,
            priceRequiredWithoutMessage: null,
          },
          { priceReadonly: 101, priceRequiredWithoutMessage: 2000 },
          {},
        );

        expect(error).toBeNull();
        expect(data).toEqual({ priceRequiredWithoutMessage: 2000 });
        // `priceReadonly`'s previous value (3000) already diverged from its
        // default (null), so it's permanently locked — its `required`
        // callback isn't re-evaluated. `priceRequiredWithoutMessage` was
        // provided, so it's skipped too. Only `price` (untouched) is
        // evaluated.
        expect(callsPerField).toEqual({
          price: true,
        });
      });
    });

    describe('behaviour when nothing is returned from required function', () => {
      const Book = new Schema<any>((b) =>
        b
          .field(b.required('bookId').validate(validator))
          .field(b.lax('isPublished', false).validate(validator))
          .field(b.lax('name', '').validate(validator))
          .field(
            b
              .lax('price', null)
              .validate(validator)
              .required((() => {}) as never),
          ),
      ).getModel();

      it('should create normally', async () => {
        const { data } = await Book.create({ bookId: 1 }, {});

        expect(data).toEqual({
          bookId: 1,
          isPublished: false,
          name: '',
          price: null,
        });
      });

      it('should update normally', async () => {
        const book = {
          bookId: 1,
          isPublished: false,
          name: '',
          price: null,
        };
        const { data } = await Book.update(book, { name: 'yooo' }, {});

        expect(data).toEqual({ name: 'yooo' });
      });
    });

    describe('behaviour when a non-string value is returned as message from required function', () => {
      describe('should respect InputField', () => {
        const responses = [
          [{ reason: 'lol' }, { reason: 'lol' }],
          [
            { reason: 'lol', metadata: { shouldWork: true } },
            { reason: 'lol', metadata: { shouldWork: true } },
          ],
          [
            { reason: '', metadata: null },
            { reason: "'price' is required", metadata: null },
          ],
          [
            { metadata: null },
            { reason: "'price' is required", metadata: null },
          ],
          [{}, { reason: "'price' is required" }],
        ];

        for (const [provided, expected] of responses) {
          const Book = new Schema<any>((b) =>
            b
              .field(b.required('bookId').validate(validator))
              .field(b.lax('isPublished', false).validate(validator))
              .field(b.lax('name', '').validate(validator))
              .field(
                b
                  .lax('price', null)
                  .validate(validator)
                  .required((): never => [true, provided] as never),
              ),
          ).getModel();

          it('should reject with proper required error message at creation', async () => {
            const { data, error } = await Book.create({ bookId: 1 }, {});

            expect(data).toBeNull();

            expect(error).toMatchObject({
              price: expect.objectContaining(expected),
            });
          });

          it('should reject with proper required error message during updates', async () => {
            const book = {
              bookId: 1,
              isPublished: false,
              name: '',
              price: null,
            };
            const { data, error } = await Book.update(
              book,
              { name: 'yooo' },
              {},
            );

            expect(data).toBeNull();

            expect(error).toMatchObject({
              price: expect.objectContaining(expected),
            });
          });
        }
      });

      describe('should ignore unsupported types', () => {
        const invalidMessages = [null, undefined, [], {}, 1, 0, -12, () => {}];

        for (const message of invalidMessages) {
          const Book = new Schema<any>((b) =>
            b
              .field(b.required('bookId').validate(validator))
              .field(b.lax('isPublished', false).validate(validator))
              .field(b.lax('name', '').validate(validator))
              .field(
                b
                  .lax('price', null)
                  .validate(validator)
                  .required((): never => [true, message] as never),
              ),
          ).getModel();

          it('should reject with proper required error message at creation', async () => {
            const { data, error } = await Book.create({ bookId: 1 }, {});

            expect(data).toBeNull();

            expect(error).toMatchObject({
              price: {
                reason: "'price' is required",
                metadata: null,
              },
            });
          });

          it('should reject with proper required error message during updates', async () => {
            const book = {
              bookId: 1,
              isPublished: false,
              name: '',
              price: null,
            };
            const { data, error } = await Book.update(
              book,
              { name: 'yooo' },
              {},
            );

            expect(data).toBeNull();

            expect(error).toMatchObject({
              price: { reason: "'price' is required", metadata: null },
            });
          });
        }
      });
    });

    describe('behaviour when a value returned by required function is not boolean nor array', () => {
      const invalidResponses = [
        null,
        undefined,
        {},
        '',
        'not array',
        1,
        0,
        -12,
        () => {},
      ];

      for (const response of invalidResponses) {
        const Book = new Schema<any>((b) =>
          b
            .field(b.required('bookId').validate(validator))
            .field(b.lax('isPublished', false).validate(validator))
            .field(b.lax('name', '').validate(validator))
            .field(
              b
                .lax('price', null)
                .validate(validator)
                .required((): never => response as never),
            ),
        ).getModel();

        it('should create normally', async () => {
          const { data } = await Book.create({ bookId: 1 }, {});

          expect(data).toEqual({
            bookId: 1,
            isPublished: false,
            name: '',
            price: null,
          });
        });

        it('should update normally', async () => {
          const book = {
            bookId: 1,
            isPublished: false,
            name: '',
            price: null,
          };
          const { data } = await Book.update(book, { name: 'yooo' }, {});

          expect(data).toEqual({ name: 'yooo' });
        });
      }
    });

    describe('behaviour with virtual properties', () => {
      const book = { name: 'book name', price: 10 };

      describe('when value of virtual is not provided', () => {
        const Book = new Schema<any>((b) =>
          b
            .field(b.lax('name', ''))
            .field(
              b
                .dependent('price', '_price')
                .default(null)
                .resolve(
                  (ctx: any) =>
                    ctx.rawInput._price ?? ctx.input._price ?? ctx.values.price,
                ),
            )
            .field(
              b
                .virtual('_price')
                .validate(validator)
                .required((ctx: any) => {
                  const _price = ctx.rawInput._price ?? ctx.input._price;
                  return _price === undefined;
                }),
            ),
        ).getModel();

        it('should reject at creation', async () => {
          const { data, error } = await Book.create({}, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            _price: {
              reason: "'_price' is required",
              metadata: null,
            },
          });
        });

        it('should reject during updates', async () => {
          const { data, error } = await Book.update(
            book,
            {
              name: 'updated name',
            },
            {},
          );

          expect(data).toBeNull();
          expect(error).toMatchObject({
            _price: {
              reason: "'_price' is required",
              metadata: null,
            },
          });
        });
      });

      describe('when value of virtual is not provided and required at creation only', () => {
        const Book = new Schema<any>((b) =>
          b
            .field(b.lax('name', ''))
            .field(
              b
                .dependent('price', '_price')
                .default(null)
                .resolve(
                  (ctx: any) =>
                    ctx.rawInput._price ?? ctx.input._price ?? ctx.values.price,
                ),
            )
            .field(
              b
                .virtual('_price')
                .validate(validator)
                .required((ctx: any) => {
                  const _price = ctx.rawInput._price ?? ctx.input._price;
                  return _price === undefined && !ctx.isUpdate;
                }),
            ),
        ).getModel();

        it('should reject at creation', async () => {
          const { data, error } = await Book.create({}, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            _price: {
              reason: "'_price' is required",
              metadata: null,
            },
          });
        });

        it('should reject during updates', async () => {
          const name = 'updated book name';
          const { data, error } = await Book.update(
            book,
            {
              name,
            },
            {},
          );

          expect(error).toBeNull();
          expect(data).toEqual({ name });
        });
      });

      describe('when value of virtual is not provided and required at creation and update is blocked', () => {
        const Book = new Schema<any>((b) =>
          b
            .field(b.lax('name', ''))
            .field(
              b
                .dependent('price', '_price')
                .default(null)
                .resolve(
                  (ctx: any) =>
                    ctx.rawInput._price ?? ctx.input._price ?? ctx.values.price,
                ),
            )
            .field(
              b
                .virtual('_price')
                .validate(validator)
                .ignoreUpdate()
                .required((ctx: any) => {
                  const _price = ctx.rawInput._price ?? ctx.input._price;
                  return _price === undefined;
                }),
            ),
        ).getModel();

        it('should reject at creation', async () => {
          const { data, error } = await Book.create({}, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            _price: {
              reason: "'_price' is required",
              metadata: null,
            },
          });
        });

        it('should reject during updates', async () => {
          const name = 'updated book name';
          const { data, error } = await Book.update(
            book,
            {
              name,
            },
            {},
          );

          expect(error).toBeNull();
          expect(data).toEqual({ name });
        });
      });
    });

    describe('behaviour with asychronous required setters', () => {
      const book = { name: 'book name', price: 10 };
      const Book = new Schema<any>((b) =>
        b
          .field(b.lax('name', ''))
          .field(
            b
              .dependent('price', '_price')
              .default(null)
              .resolve(
                (ctx: any) =>
                  ctx.rawInput._price ?? ctx.input._price ?? ctx.values.price,
              ),
          )
          .field(
            b
              .virtual('_price')
              .validate(validator)
              .required((ctx: any) => {
                const _price = ctx.rawInput._price ?? ctx.input._price;
                return Promise.resolve(_price === undefined);
              }),
          ),
      ).getModel();

      describe('creation', () => {
        it('should reject when condition is not met', async () => {
          const { data, error } = await Book.create({}, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            _price: { reason: "'_price' is required", metadata: null },
          });
        });

        it('should allow when condition is met', async () => {
          const { data, error } = await Book.create({ _price: 20 }, {});

          expect(error).toBeNull();
          expect(data).toMatchObject({ name: '', price: 20 });
        });
      });

      describe('updates', () => {
        it('should reject when condition is not met', async () => {
          // A genuine, unrelated change (`name`) so this isn't a no-op
          // update — `_price` itself stays unprovided, so its `required`
          // callback still runs and rejects.
          const { data, error } = await Book.update(
            book,
            {
              name: 'updated name',
            },
            {},
          );

          expect(data).toBeNull();
          expect(error).toMatchObject({
            _price: {
              reason: "'_price' is required",
              metadata: null,
            },
          });
        });

        it('should allow when condition is met', async () => {
          const { data, error } = await Book.update(book, { _price: 20 }, {});

          expect(error).toBeNull();
          expect(data).toMatchObject({ price: 20 });
        });
      });
    });

    describe('behaviour with errors thrown in required setter', () => {
      const Model = new Schema<any>((b) =>
        b.field(b.lax('prop1', '')).field(
          b
            .lax('field', null)
            .validate(validator)
            .required(() => {
              throw new Error('lolol');
            }),
        ),
      ).getModel();

      it('should consider required:false if occurred at creation', async () => {
        const { data, error } = await Model.create({}, {});

        expect(error).toBeNull();
        expect(data).toEqual({ field: null, prop1: '' });
      });

      it('should consider required:false if occurred during updates', async () => {
        const { data, error } = await Model.update(
          { field: null, prop1: '' },
          { prop1: 'updated' },
          {},
        );

        expect(error).toBeNull();
        expect(data).toEqual({ prop1: 'updated' });
      });
    });
  });

  describe('valid', () => {
    it('should accept requiredBy + default(any | function)', () => {
      const values = ['', () => ''];

      for (const value of values) {
        const toPass = makeFx((b) =>
          b.field(
            b
              .lax('fieldName', value)
              .validate(validator)
              .required(() => true),
          ),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should accept requiredBy + readonly', () => {
      const toPass = makeFx((b) =>
        b.field(
          b
            .lax('fieldName', '')
            .validate(validator)
            .readonly()
            .required(() => true),
        ),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it('should accept requiredBy + ignoreInit', () => {
      const toPass = makeFx((b) =>
        b.field(
          b
            .lax('fieldName', '')
            .validate(validator)
            .readonly()
            .ignoreInit(() => true)
            .required(() => true),
        ),
      );

      expectNoFailure(toPass);

      toPass();
    });
  });

  // "invalid" discarded entirely: "requiredBy & no default" (`.required()`
  // only unlocks after `.default()` on the lax builder), "requiredBy +
  // default & dependent(true)" (`DependentBuilder` never exposes a
  // `.required()` method), and "requiredBy + allow" (combines allow()+
  // validator on the same m, already established unrepresentable in
  // allowed-values.ts, and also omits default()) are all structurally
  // unrepresentable through the builder by design.
});
