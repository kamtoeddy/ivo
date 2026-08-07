import { afterEach, describe, expect, it } from 'bun:test';
import { type ReadonlyIvoContext, Schema } from '../../src';
import { expectFailure, expectNoFailure, makeFx } from '../_utils';

describe('dependent', () => {
  const resolver = () => 1;

  describe('behaviour', () => {
    let onDeleteStats = {} as Record<string, number | undefined>;
    let onSuccessStats = {} as Record<string, number | undefined>;
    let resolversCalledStats = {} as Record<string, number | undefined>;

    const successCountOfDependentFields = {
      dependentField: 4,
      dependentField_1: 1,
      dependentField_2: 3,
      dependentField_3: 1,
    };

    type SampleInput = { laxField?: string; laxField_1?: string };
    type SampleOutput = {
      laxField: string;
      laxField_1: string;
      dependentField: number;
      dependentField_1: number;
    };

    const Model = new Schema<SampleInput, SampleOutput>((b) =>
      b
        .field(b.lax('laxField', ''))
        .field(b.lax('laxField_1', ''))
        .field(
          b

            // @ts-expect-error ikr
            .lax('laxField_2', '')
            .onDelete(incrementOnDeleteCountOf('laxField_2')),
        )
        .field(
          b
            .dependent('dependentField', ['laxField', 'laxField_1'])
            .default(0)
            .resolve(resolverOfDependentField as never)
            .onDelete([
              incrementOnDeleteCountOf('dependentField'),
              incrementOnDeleteCountOf('dependentField'),
            ])
            .onSuccess([
              incrementOnSuccessCountOf('dependentField'),
              incrementOnSuccessCountOf('dependentField'),
              incrementOnSuccessCountOf('dependentField'),
              incrementOnSuccessCountOf('dependentField'),
            ]),
        )
        .field(
          b
            .dependent('dependentField_1', 'dependentField')
            .default(0)
            .resolve(resolverOfDependentField_1 as never)
            .onDelete(incrementOnDeleteCountOf('dependentField_1'))
            .onSuccess(incrementOnSuccessCountOf('dependentField_1')),
        )
        .field(
          b // @ts-expect-error ikr
            .dependent('dependentField_2', 'dependentField')
            .default(0)
            .resolve(asyncResolver('dependentField_2') as never)
            .readonly()
            .onDelete([
              incrementOnDeleteCountOf('dependentField_2'),
              incrementOnDeleteCountOf('dependentField_2'),
            ])
            .onSuccess([
              incrementOnSuccessCountOf('dependentField_2'),
              incrementOnSuccessCountOf('dependentField_2'),
              incrementOnSuccessCountOf('dependentField_2'),
            ]),
        )
        .field(
          b // @ts-expect-error ikr
            .dependent('dependentField_3', 'laxField_2')
            .default(0)
            .resolve(asyncResolver('dependentField_3') as never)
            .onDelete([
              incrementOnDeleteCountOf('dependentField_3'),
              incrementOnDeleteCountOf('dependentField_3'),
            ])
            .onSuccess([incrementOnSuccessCountOf('dependentField_3')]),
        ),
    ).getModel();

    function incrementOnDeleteCountOf(field: string) {
      return () => {
        const previousCount = onDeleteStats[field] ?? 0;

        onDeleteStats[field] = previousCount + 1;
      };
    }

    function incrementOnSuccessCountOf(field: string) {
      return () => {
        const previousCount = onSuccessStats[field] ?? 0;

        onSuccessStats[field] = previousCount + 1;
      };
    }

    function incrementResolveCountOf(field: string) {
      const previousCount = resolversCalledStats[field] ?? 0;

      resolversCalledStats[field] = previousCount + 1;
    }

    function resolverOfDependentField(
      ctx: ReadonlyIvoContext<SampleInput, SampleOutput, {}>,
    ) {
      incrementResolveCountOf('dependentField');
      const laxField =
        ctx.rawInput.laxField ?? ctx.input.laxField ?? ctx.values.laxField;
      const laxField_1 =
        ctx.rawInput.laxField_1 ??
        ctx.input.laxField_1 ??
        ctx.values.laxField_1;

      return (laxField?.length ?? 0) + (laxField_1?.length ?? 0);
    }

    function resolverOfDependentField_1(
      ctx: ReadonlyIvoContext<SampleInput, SampleOutput, {}>,
    ) {
      incrementResolveCountOf('dependentField_1');

      return ctx.values.dependentField! + 1;
    }

    function asyncResolver(field: string) {
      return (ctx: ReadonlyIvoContext<SampleInput, SampleOutput, {}>) => {
        incrementResolveCountOf(field);

        return Promise.resolve(ctx.values.dependentField! + 2);
      };
    }

    afterEach(() => {
      onDeleteStats = {};
      onSuccessStats = {};
      resolversCalledStats = {};
    });

    describe('creation', () => {
      it('should resolve dependent properties correctly at creation', async () => {
        const { data, handleSuccess } = await Model.create(
          {
            // @ts-expect-error ikr
            laxField_2: 'value based pricing',
            dependentField: 25,
            dependentField_1: 34,
            dependentField_2: 17,
            dependentField_3: 1,
          },
          {},
        );

        await handleSuccess?.();

        expect(data).toEqual({
          laxField: '',
          laxField_1: '',
          // @ts-expect-error ikr
          laxField_2: 'value based pricing',
          dependentField: 0,
          dependentField_1: 0,
          dependentField_2: 0,
          dependentField_3: 2,
        });

        expect(resolversCalledStats).toEqual({ dependentField_3: 1 });
        expect(onSuccessStats).toEqual(successCountOfDependentFields);
      });

      it('should resolve dependencies of dependent properties if provided at creation', async () => {
        const { data, handleSuccess } = await Model.create(
          {
            laxField: '',
            laxField_1: 'hello',
            // @ts-expect-error ikr
            dependentField: 0,
            dependentField_1: 0,
            dependentField_2: 0,
          },
          {},
        );

        await handleSuccess?.();

        expect(data).toEqual({
          laxField: '',
          laxField_1: 'hello',
          // @ts-expect-error ikr
          laxField_2: '',
          dependentField: 5,
          dependentField_1: 6,
          dependentField_2: 7,
          dependentField_3: 0,
        });

        expect(resolversCalledStats).toEqual({
          dependentField: 1,
          dependentField_1: 1,
          dependentField_2: 1,
        });

        expect(onSuccessStats).toEqual(successCountOfDependentFields);
      });
    });

    describe('updates', () => {
      it('should have all correct properties and values after updates', async () => {
        const { data: updates, handleSuccess } = await Model.update(
          {
            laxField: '',
            laxField_1: '',
            // @ts-expect-error ikr
            laxField_2: 'value based pricing',
            dependentField: 0,
            dependentField_1: 0,
            dependentField_2: 0,
            dependentField_3: 2,
          },
          {
            laxField_2: 'hey',
            dependentField: 74,
            dependentField_1: 235,
            dependentField_2: 72,
            dependentField_3: 702,
          },
          {},
        );

        await handleSuccess?.();

        expect(updates).toMatchObject({ laxField_2: 'hey' });

        expect(resolversCalledStats).toEqual({ dependentField_3: 1 });

        expect(onSuccessStats).toEqual({});
      });

      it('should resolve dependencies of dependent properties if provided during updates', async () => {
        const { data, handleSuccess } = await Model.update(
          {
            laxField: '',
            laxField_1: '',
            // @ts-expect-error ikr
            laxField_2: '',
            dependentField: 0,
            dependentField_1: 0,
            dependentField_2: 0,
            dependentField_3: 0,
          },
          { laxField: 'hello', laxField_1: 'world' },
          {},
        );

        await handleSuccess?.();

        expect(data).toEqual({
          laxField: 'hello',
          laxField_1: 'world',
          dependentField: 10,
          dependentField_1: 11,
          // @ts-expect-error ikr
          dependentField_2: 12,
        });

        const { dependentField_3: _, ...stats } = successCountOfDependentFields;

        expect(resolversCalledStats).toEqual({
          dependentField: 1,
          dependentField_1: 1,
          dependentField_2: 1,
        });

        expect(onSuccessStats).toEqual(stats);
      });

      it('should not resolve readonly dependent properties that have changed if provided during updates', async () => {
        const { data, handleSuccess } = await Model.update(
          {
            laxField: 'hello',
            laxField_1: 'world',
            dependentField: 10,
            dependentField_1: 11,
            // @ts-expect-error ikr
            dependentField_2: 12,
            dependentField_3: 0,
          },
          { laxField: '', laxField_1: 'hey' },
          {},
        );

        await handleSuccess?.();

        expect(data).toEqual({
          laxField: '',
          laxField_1: 'hey',
          dependentField: 3,
          dependentField_1: 4,
        });

        const stats = {
          dependentField: 1,
          dependentField_1: 1,
        };

        expect(resolversCalledStats).toEqual(stats);

        expect(onSuccessStats).toEqual({
          dependentField: 4,
          dependentField_1: 1,
        });
      });

      it('should not consider new resolved values of dependent properties if they are not different from previous values during updates', async () => {
        const { data, handleSuccess } = await Model.update(
          {
            laxField: 'hello',
            laxField_1: 'world',
            dependentField: 3,
            dependentField_1: 4,
            // @ts-expect-error ikr
            dependentField_2: 12,
            dependentField_3: 0,
          },
          { laxField: '', laxField_1: 'hey' },
          {},
        );

        await handleSuccess?.();

        expect(data).toEqual({ laxField: '', laxField_1: 'hey' });

        expect(resolversCalledStats).toEqual({ dependentField: 1 });

        expect(onSuccessStats).toEqual({});
      });
    });

    describe('deletion', () => {
      it('should have all correct properties and values at creation', async () => {
        await Model.delete(
          {
            laxField: '',
            laxField_1: '',
            // @ts-expect-error ikr
            laxField_2: 'value based pricing',
            dependentField: 0,
            dependentField_1: 0,
            dependentField_2: 0,
            dependentField_3: 2,
          },
          {},
        );

        expect(onDeleteStats).toEqual({
          laxField_2: 1,
          dependentField: 2,
          dependentField_1: 1,
          dependentField_2: 2,
          dependentField_3: 2,
        });
      });
    });

    describe('readonly resolver "unlock while == default / freeze once diverged" semantics', () => {
      // Mirrors rs/tests/fields/dependents.rs::
      // should_not_run_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value
      let resolverRunCount = 0;

      const Model = new Schema<any>((b) =>
        b.field(b.lax('parent', '')).field(
          b
            .dependent('child', 'parent')
            .default(0)
            .resolve(() => {
              resolverRunCount++;
              return 1;
            })
            .readonly(),
        ),
      ).getModel();

      it('does not run the resolver at creation if the parent was not provided (value stays at the raw static default)', async () => {
        resolverRunCount = 0;
        const { data } = await Model.create({}, {});

        expect(data).toEqual({ parent: '', child: 0 });
        expect(resolverRunCount).toBe(0);
      });

      it('runs the resolver once creation is given the parent explicitly, moving the value off the default', async () => {
        resolverRunCount = 0;
        const { data } = await Model.create({ parent: 'x' }, {});

        expect(data).toEqual({ parent: 'x', child: 1 });
        expect(resolverRunCount).toBe(1);
      });

      it('re-runs the resolver on the first update while the value still equals the default', async () => {
        resolverRunCount = 0;
        const { data } = await Model.update(
          { parent: '', child: 0 },
          { parent: 'x' },
          {},
        );

        expect(data).toEqual({ parent: 'x', child: 1 });
        expect(resolverRunCount).toBe(1);
      });

      it('freezes permanently once the value has diverged from the default, even if the parent changes again', async () => {
        resolverRunCount = 0;
        const { data } = await Model.update(
          { parent: 'x', child: 1 },
          { parent: 'y' },
          {},
        );

        expect(data).toEqual({ parent: 'y' });
        expect(resolverRunCount).toBe(0);
      });
    });

    describe('behaviour with errors thrown in the resolver', () => {
      const Model = new Schema<any>((b) =>
        b
          .field(b.lax('field', ''))
          .field(
            b
              .dependent('dependent', 'field')
              .default('')
              .resolve(() => {
                throw new Error('lolol');
              }),
          )
          .field(
            b
              .dependent('dependent1', 'dependent')
              .default('')
              .resolve(() => {
                throw new Error('lolol');
              }),
          )
          .field(
            b
              .dependent('dependent2', 'dependent')
              .default('')
              .resolve(() => {
                throw new Error('lolol');
              }),
          ),
      ).getModel();

      it("should set dependent to null if error occurred resolving at creation'", async () => {
        const { data, error } = await Model.create({ field: 'test' }, {});

        expect(error).toBeNull();
        expect(data).toEqual({
          dependent: null,
          dependent1: null,
          dependent2: null,
          field: 'test',
        });
      });

      it('should ignore dependent properties that error when resolving during updates', async () => {
        const { data, error } = await Model.update(
          { dependent: '', dependent1: '', dependent2: '', field: '' },
          { field: 'updated' },
          {},
        );

        expect(error).toBeNull();
        expect(data).toEqual({ field: 'updated' });
      });
    });
  });

  describe('valid', () => {
    it('should accept dependent & default(any | function)', () => {
      const values = ['', 1, false, true, null, {}, []];

      for (const value of values) {
        const toPass = makeFx((b) =>
          b
            .field(
              b
                .dependent('dependentField', 'field')
                .default(value)
                .resolve(resolver),
            )
            .field(b.lax('field', '')),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should allow dependsOn + resolver & no dependent', () => {
      const toPass = makeFx((b) =>
        b
          .field(
            b
              .dependent('dependentField', 'field')
              .default('')
              .resolve(resolver),
          )
          .field(b.lax('field', '')),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should accept life cycle listeners except 'onFailure'", () => {
      const lifeCycles = ['onDelete', 'onSuccess'] as const;
      const values = [() => {}, () => ({}), [() => {}, () => ({})]];

      for (const lifeCycle of lifeCycles) {
        for (const value of values) {
          const toPass = makeFx((b) =>
            b
              .field(
                b
                  .dependent('dependentField', 'field')
                  .default(value)
                  .resolve(resolver)
                  [lifeCycle](value as never),
              )
              .field(b.lax('field', '')),
          );

          expectNoFailure(toPass);

          toPass();
        }
      }
    });

    it('should accept dependsOn & resolver', () => {
      const values = [
        'field',
        ['field', 'prop1'],
        ['field', 'prop1', 'prop2', 'prop3'],
      ] as const;

      for (const dependsOn of values) {
        const toPass = makeFx((b) =>
          b
            .field(
              b
                .dependent('dependentField', dependsOn)
                .default('')
                .resolve(resolver),
            )
            .field(b.lax('field', ''))
            .field(b.lax('prop1', ''))
            .field(b.lax('prop2', ''))
            .field(b.lax('prop3', '')),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should allow a dependent field to depend on another dependent field (non-circular)', () => {
      const toPass = makeFx((b) =>
        b
          .field(
            b
              .dependent('dependentField1', 'field')
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('dependentField2', 'dependentField1')
              .default('')
              .resolve(resolver),
          )
          .field(b.lax('field', '')),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow a dependency on virtuals', () => {
      const toPass = makeFx((b) =>
        b
          .field(
            b
              .dependent('dependentField', 'virtualField')
              .default('')
              .resolve(resolver),
          )
          .field(b.virtual('virtualField').validate(() => true)),
      );

      expectNoFailure(toPass);

      toPass();
    });
  });

  describe('invalid', () => {
    it('should reject dependency on non-properties', () => {
      const invalidField = 'invalidField';

      const toFail = makeFx((b) =>
        b.field(
          b
            .dependent('dependentField', invalidField as never)
            .default('')
            .resolve(resolver),
        ),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject(
          expect.objectContaining({
            dependentField: expect.arrayContaining([
              `Cannot establish dependency with '${invalidField}' as it is neither a property nor a virtual of your model`,
            ]),
          }),
        );
      }
    });

    it('should not allow property to depend on itself', () => {
      const toFail = makeFx((b) =>
        b.field(
          b
            .dependent('dependentField', 'dependentField' as never)
            .default('')
            .resolve(resolver),
        ),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject(
          expect.objectContaining({
            dependentField: expect.arrayContaining([
              'A property cannot depend on itself',
            ]),
          }),
        );
      }
    });

    it('should not allow property to depend on a constant property', () => {
      const toFail = makeFx((b) =>
        b.field(b.constant('constantField', '')).field(
          b
            .dependent('dependentField', 'constantField' as never)
            .default('')
            .resolve(resolver),
        ),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject(
          expect.objectContaining({
            dependentField: expect.arrayContaining([
              'A property cannot depend on a constant property',
            ]),
          }),
        );
      }
    });

    it('should identify circular dependencies and reject', () => {
      const toFail = makeFx((b) =>
        b
          .field(
            b
              .dependent('A', ['B', 'C', 'D'] as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('B', ['A', 'C', 'E'] as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('C', ['A'] as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('D', 'E' as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('E', 'A' as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('F', 'field' as never)
              .default('')
              .resolve(resolver),
          )
          .field(b.lax('field', '')),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject(
          expect.objectContaining({
            A: expect.arrayContaining([
              "Circular dependency identified with 'B'",
              "Circular dependency identified with 'C'",
              "Circular dependency identified with 'E'",
            ]),
            B: expect.arrayContaining([
              "Circular dependency identified with 'A'",
            ]),
            C: expect.arrayContaining([
              "Circular dependency identified with 'A'",
              "Circular dependency identified with 'B'",
            ]),
            D: expect.arrayContaining([
              "Circular dependency identified with 'A'",
            ]),
            E: expect.arrayContaining([
              "Circular dependency identified with 'B'",
              "Circular dependency identified with 'D'",
            ]),
          }),
        );
      }
    });

    it('should identify redundant dependencies and reject', () => {
      const toFail = makeFx((b) =>
        b
          .field(
            b
              .dependent('A', 'field' as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('B', ['A', 'field'] as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('C', 'A' as never)
              .default('')
              .resolve(resolver),
          )
          .field(
            b
              .dependent('D', ['field', 'C'] as never)
              .default('')
              .resolve(resolver),
          )
          .field(b.lax('field', '')),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject(
          expect.objectContaining({
            B: expect.arrayContaining([
              "Dependency on 'field' is redundant because of dependency on 'A'",
            ]),
            D: expect.arrayContaining([
              "Dependency on 'field' is redundant because of dependency on 'C'",
            ]),
          }),
        );
      }
    });
  });
});
