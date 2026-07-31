import { beforeAll, describe, expect, it } from 'bun:test';

import { type ReadonlyIvoContext, Schema } from '../../src';

import { expectFailure, expectNoFailure, makeFx } from '../_utils';

describe('constant', () => {
  describe('valid', () => {
    let User: any, user: any;

    beforeAll(async () => {
      User = new Schema<any>((b, m) =>
        b
          .field(m.constant('asyncConstant').value(asyncSetter))
          .field(
            m.constant('id').value(
              (
                ctx: ReadonlyIvoContext<
                  { id?: string | number },
                  {
                    asyncConstant: number;
                    id: string;
                    parentId: string;
                    laxField: number;
                  }
                >,
              ) => (ctx?.input?.id === 'id' ? 'id-2' : 'id'),
            ),
          )
          .field(m.constant('parentId').value('parent id'))
          .field(m.lax('laxField').default(0)),
      ).getModel();

      function asyncSetter() {
        return Promise.resolve(20);
      }

      user = (await User.create({ id: 2, parentId: [], laxField: 2 })).data;
    });

    it('should set constants at creation', () => {
      expect(user).toEqual({
        asyncConstant: 20,
        id: 'id',
        parentId: 'parent id',
        laxField: 2,
      });
    });

    it('should not set constants via listeners', async () => {
      const { data: update } = await User.update(user, {
        laxField: 'update id',
      });

      expect(update).toEqual({ laxField: 'update id' });
    });

    it('should ignore constants during updates', async () => {
      const { data, error } = await User.update(user, { id: 25 });

      expect(data).toBeNull();
      expect(error).toBeNull();
    });

    it('should accept constant(true) & value(any | ()=>any)', () => {
      const values = ['', 'value', 1, null, false, true, {}, [], () => 1];

      for (const value of values) {
        const toPass = makeFx((b, m) =>
          b.field(m.constant('fieldName').value(value)),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should accept constant & value + onDelete(function | function[])', () => {
      const values = [() => ({}), [() => ({})], [() => ({}), () => ({})]];

      for (const onDelete of values) {
        const toPass = makeFx((b, m) =>
          b.field(
            m
              .constant('fieldName')
              .value('')
              .onDelete(onDelete as never),
          ),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should accept constant & value + onSuccess(function | function[])', () => {
      const values = [() => ({}), [() => ({})], [() => ({}), () => ({})]];

      for (const onSuccess of values) {
        const toPass = makeFx((b, m) =>
          b.field(
            m
              .constant('fieldName')
              .value('')
              .onSuccess(onSuccess as never),
          ),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });
  });

  // "should reject constant(!true)", "should reject constant & no value",
  // "should reject 'value' on non-constants", and "should reject (constant
  // & value) + any other non constant rule" discarded: `ConstantBuilder`
  // always sets `type: "constant"` and only exposes `.value()`/
  // `.onDelete()`/`.onSuccess()` - there's no way to produce a Buildable
  // without calling `.value()`, no way to mis-set the `constant` marker,
  // and no way to attach any of the other rules (default, dependsOn,
  // onFailure, readonly, resolver, required, sanitizer, ignoreInit,
  // validator, virtual) to a constant field builder chain at all.
  describe('invalid', () => {
    it('should reject constant & value(undefined)', () => {
      const toFail = makeFx((b, m) =>
        b.field(m.constant('fieldName').value(undefined as never)),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            fieldName: expect.arrayContaining([
              "Constant fields cannot have 'undefined' as value",
            ]),
          }),
        );
      }
    });
  });

  describe('behaviour with errors thrown in the value generator', () => {
    const Model = new Schema<any>((b, m) =>
      b.field(
        m.constant('constant').value(() => {
          throw new Error('lolol');
        }),
      ),
    ).getModel();

    it('should set value of constant to null if value could not be generated properly', async () => {
      const { data, error } = await Model.create({});

      expect(error).toBeNull();
      expect(data).toMatchObject({ constant: null });
    });
  });
});
