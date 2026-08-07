import { afterEach, describe, expect, it, mock } from 'bun:test';

import { Schema } from '../../src';
import type { IvoSuccessContext } from '../../src/utils/types';
import { expectFailure, expectNoFailure, makeFx, validator } from '../_utils';

describe('ignore', () => {
  describe('valid', () => {
    it('should accept ignore + default', () => {
      const fxn = makeFx((b) =>
        b.field(b.lax('fieldName', true).ignore(() => false)),
      );

      expectNoFailure(fxn);

      fxn();
    });

    it('should accept ignore + virtual', () => {
      const fxn = makeFx((b) =>
        b
          .field(
            b
              .dependent('dependent', 'fieldName')
              .default(true)
              .resolve(validator as never),
          )
          .field(
            b
              .virtual('fieldName')
              .validate(validator)
              .ignore(() => false),
          ),
      );

      expectNoFailure(fxn);

      fxn();
    });

    describe('behaviour', () => {
      it('should ignore accordingly', async () => {
        const Model = new Schema<{
          env: string;
          isBlocked: boolean | string;
          laxField: number;
        }>((b) =>
          b
            .field(
              b
                .lax('isBlocked', false)
                .ignore(({ input: { env } }) => env === 'dev'),
            )
            .field(b.lax('env', 'dev'))
            .field(b.lax('laxField', 0)),
        ).getModel();

        const { data } = await Model.create(
          { env: 'dev', isBlocked: true },
          {},
        );

        expect(data).toMatchObject({
          env: 'dev',
          isBlocked: false,
          laxField: 0,
        });

        {
          const { data } = await Model.create(
            {
              env: 'Lol',
              isBlocked: true,
            },
            {},
          );

          expect(data).toMatchObject({
            env: 'Lol',
            isBlocked: true,
            laxField: 0,
          });
        }

        {
          const { data } = await Model.update(
            {
              env: 'Lol',
              isBlocked: true,
              laxField: 0,
            },
            { env: 'dev', isBlocked: 'updated' },
            {},
          );
          expect(data).toEqual({ env: 'dev' });
        }

        {
          const { data } = await Model.update(
            {
              env: 'dev',
              isBlocked: true,
              laxField: 0,
            },
            { env: 'Lol', isBlocked: 'updated' },
            {},
          );

          expect(data).toEqual({ env: 'Lol', isBlocked: 'updated' });
        }
      });

      it('should not trigger validators of ignored fields', async () => {
        const validator = () => true;

        const mockedValidator = mock(validator);

        const Model = new Schema<any>((b) =>
          b
            .field(
              b
                .lax('isBlocked', false)
                .validate(mockedValidator)
                .ignore(({ input: { env } }) => env === 'dev'),
            )
            .field(b.lax('env', 'dev'))
            .field(b.lax('laxField', 0)),
        ).getModel();

        const { data } = await Model.create(
          { env: 'dev', isBlocked: true },
          {},
        );

        expect(mockedValidator).toBeCalledTimes(0);

        expect(data).toMatchObject({
          env: 'dev',
          isBlocked: false,
          laxField: 0,
        });

        {
          const { data } = await Model.create(
            { env: 'Lol', isBlocked: true },
            {},
          );

          expect(mockedValidator).toBeCalledTimes(1);

          expect(data).toMatchObject({
            env: 'Lol',
            isBlocked: true,
            laxField: 0,
          });
        }

        {
          const { data } = await Model.update(
            { env: 'Lol', isBlocked: true, laxField: 0 },
            { env: 'dev', isBlocked: 'updated' },
            {},
          );
          expect(mockedValidator).toBeCalledTimes(1);
          expect(data).toEqual({ env: 'dev' });
        }

        {
          const { data } = await Model.update(
            { env: 'dev', isBlocked: true, laxField: 0 },
            { env: 'Lol', isBlocked: 'updated' },
            {},
          );

          expect(mockedValidator).toBeCalledTimes(2);
          expect(data).toEqual({ env: 'Lol', isBlocked: 'updated' });
        }
      });

      it('should properly handle ignored properties even when not provided', async () => {
        const validator = () => true;

        const mockedValidator = mock(validator);

        const Model = new Schema<any>((b) =>
          b
            .field(
              b
                .lax('isBlocked', false)
                .validate(mockedValidator as never)
                .ignore(({ input: { env } }: any) => env === 'dev'),
            )
            .field(b.lax('env', 'dev'))
            .field(b.lax('laxField', 0)),
        ).getModel();

        const { data } = await Model.create({ env: 'dev' }, {});

        expect(mockedValidator).toBeCalledTimes(0);

        expect(data).toMatchObject({
          env: 'dev',
          isBlocked: false,
          laxField: 0,
        });

        {
          const { data } = await Model.create(
            { env: 'Lol', isBlocked: true },
            {},
          );

          expect(mockedValidator).toBeCalledTimes(1);

          expect(data).toMatchObject({
            env: 'Lol',
            isBlocked: true,
            laxField: 0,
          });
        }

        {
          const { data } = await Model.update(
            { env: 'Lol', isBlocked: true, laxField: 0 },
            { env: 'dev', isBlocked: 'updated' },
            {},
          );
          expect(mockedValidator).toBeCalledTimes(1);
          expect(data).toEqual({ env: 'dev' });
        }

        {
          const { data } = await Model.update(
            { env: 'dev', isBlocked: true, laxField: 0 },
            { env: 'Lol', isBlocked: 'updated' },
            {},
          );

          expect(mockedValidator).toBeCalledTimes(2);
          expect(data).toEqual({ env: 'Lol', isBlocked: 'updated' });
        }
      });
    });
  });

  // "should reject ignore & no default" discarded: `.ignore()` isn't
  // available on `LaxBuilder` until `.default()` has been called, so a
  // field with `ignore` but no default is structurally unrepresentable.
  describe('invalid', () => {
    it('should reject ingnore !(() => boolean)', () => {
      const values = [
        undefined,
        1,
        {},
        null,
        [],
        true,
        false,
        'yes',
        'false',
        'true',
      ];

      for (const ignore of values) {
        const fxn = makeFx((b) =>
          b.field(b.lax('fieldName', true).ignore(ignore as never)),
        );

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              fieldName: expect.arrayContaining([
                '"ignore" must be a function that returns a boolean',
              ]),
            }),
          );
        }
      }
    });
  });
});

describe('ignoreInit', () => {
  const Model = new Schema<any>((b) =>
    b
      .field(b.lax('isBlocked', false).ignoreInit())
      .field(b.lax('env', 'dev'))
      .field(b.lax('laxField', 0)),
  ).getModel();

  it('should respect default rules', async () => {
    const { data } = await Model.create({ isBlocked: true }, {});

    expect(data).toMatchObject({
      env: 'dev',
      isBlocked: false,
      laxField: 0,
    });

    const { data: updatedData } = await Model.update(
      data,
      { isBlocked: true },
      {},
    );

    expect(updatedData).toMatchObject({ isBlocked: true });
  });
});

describe('ignoreUpdate', () => {
  let onSuccessValues: Record<string, unknown> = {};
  let onSuccessStats: Record<string, number> = {};

  type Input = {
    virtual: boolean;
    virtual_1: boolean;
    laxField: string;
    laxField_1: string;
  };
  type Output = {
    dependentField: boolean;
    dependentField_1: boolean;
    laxField: string;
    laxField_1: string;
  };

  function incrementOnSuccessCountOf(field: string) {
    return ({ input, values }: IvoSuccessContext<Input, Output>) => {
      const previousCount = onSuccessStats[field] ?? 0;

      onSuccessStats[field] = previousCount + 1;
      onSuccessValues[field] =
        (values as Record<string, unknown>)[field] ??
        (input as Record<string, unknown>)?.[field];
    };
  }

  const Model = new Schema<Input, Output>((b) =>
    b
      .field(
        b
          .dependent('dependentField', 'virtual')
          .default(false)
          .resolve(({ input }: any) => input.virtual)
          .onSuccess(incrementOnSuccessCountOf('dependentField')),
      )
      .field(
        b
          .dependent('dependentField_1', 'virtual_1')
          .default(false)
          .resolve(({ input }: any) => input.virtual_1)
          .onSuccess(incrementOnSuccessCountOf('dependentField_1')),
      )
      .field(
        b
          .lax('laxField', '')
          .readonly()
          .ignore(({ previousValues }) => previousValues?.laxField_1 === 'test')
          .onSuccess(incrementOnSuccessCountOf('laxField')),
      )
      .field(b.lax('laxField_1', 'dev'))
      .field(
        b
          .virtual('virtual')
          .validate(() => ({ valid: true }))
          .ignoreUpdate()
          .onSuccess(incrementOnSuccessCountOf('virtual')),
      )
      .field(
        b
          .virtual('virtual_1')
          .validate(() => ({ valid: true }))
          .ignore(({ previousValues }) => previousValues?.laxField_1 === 'test')
          .onSuccess(incrementOnSuccessCountOf('virtual_1')),
      ),
  ).getModel();

  afterEach(() => {
    onSuccessValues = {};
    onSuccessStats = {};
  });

  it("should update properties when 'ignoreUpdate' resolved to 'false'", async () => {
    const { data, error, handleSuccess } = await Model.update(
      {
        // @ts-expect-error ikr
        dependentField: 'dev',
        // @ts-expect-error ikr
        dependentField_1: 'dev',
        laxField: '',
        laxField_1: '',
      },
      { laxField: 'yoyo', virtual: true, virtual_1: true },
      {},
    );

    await handleSuccess?.();

    expect(error).toBeNull();
    expect(data).toEqual({ dependentField_1: true, laxField: 'yoyo' });

    expect(onSuccessStats).toEqual({
      dependentField_1: 1,
      laxField: 1,
      virtual_1: 1,
    });

    expect(onSuccessValues).toEqual({
      dependentField_1: true,
      laxField: 'yoyo',
      virtual_1: true,
    });
  });

  it("should not update properties when 'ignoreUpdate' resolved to 'true'", async () => {
    const { data, error } = await Model.update(
      {
        // @ts-expect-error ikr
        dependentField: 'dev',
        // @ts-expect-error ikr
        dependentField_1: 'dev',
        laxField: '',
        laxField_1: 'test',
      },
      { laxField: 'yoyo', virtual: true, virtual_1: true },
      {},
    );

    expect(data).toBeNull();
    expect(error).toBeNull();
  });

  it("should not update readonly properties that have changed even when 'ignoreUpdate' resolved to 'false'", async () => {
    const { data, error } = await Model.update(
      {
        // @ts-expect-error ikr
        dependentField: 'dev',
        // @ts-expect-error ikr
        dependentField_1: 'dev',
        laxField: 'changed',
        laxField_1: 'test',
      },
      { laxField: 'yoyo' },
      {},
    );

    expect(data).toBeNull();
    expect(error).toBeNull();
  });

  describe('behaviour when ignoreUpdate method returns nothing', () => {
    const Model = new Schema<any>((b) =>
      b
        .field(b.lax('isBlocked', false).ignoreUpdate())
        .field(b.lax('laxField', 0)),
    ).getModel();

    it('should update fields if ignoreUpdate method returns nothing', async () => {
      const { data, error } = await Model.update(
        { isBlocked: false, laxField: 0 },
        { isBlocked: true },
        {},
      );

      expect(error).toBeNull();
      expect(data).toEqual({ isBlocked: true });
    });
  });
});
