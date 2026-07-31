import { afterEach, describe, expect, it } from 'bun:test';
import type { ReadonlyIvoContext } from '../../src';
import { createFieldBuilder } from '../../src/schema/fields';
import { expectFailure, expectNoFailure, validator } from '../_utils';

const field = createFieldBuilder<any, any>();

export const Test_DependentProperties = ({ Schema, fx }: any) => {
  describe('dependent', () => {
    const resolver = () => 1;

    describe('behaviour', () => {
      let onDeleteStats = {} as Record<string, number | undefined>;
      let onSuccessStats = {} as Record<string, number | undefined>;
      let resolversCalledStats = {} as Record<string, number | undefined>;

      const successCountOfDependentProps = {
        dependentProp: 4,
        dependentProp_1: 1,
        dependentProp_2: 3,
        dependentProp_3: 1,
      };

      type SampleInput = { laxProp?: string; laxProp_1?: string };
      type SampleOutput = {
        laxProp: string;
        laxProp_1: string;
        dependentProp: number;
        dependentProp_1: number;
      };

      const Model = new Schema({
        laxProp: field.lax('laxProp').default(''),
        laxProp_1: field.lax('laxProp_1').default(''),
        laxProp_2: field
          .lax('laxProp_2')
          .default('')
          .onDelete(incrementOnDeleteCountOf('laxProp_2')),
        dependentProp: field
          .dependent('dependentProp')
          .default(0)
          .dependsOn(['laxProp', 'laxProp_1'])
          .resolve(resolverOfDependentProp as never)
          .onDelete([
            incrementOnDeleteCountOf('dependentProp'),
            incrementOnDeleteCountOf('dependentProp'),
          ])
          .onSuccess([
            incrementOnSuccessCountOf('dependentProp'),
            incrementOnSuccessCountOf('dependentProp'),
            incrementOnSuccessCountOf('dependentProp'),
            incrementOnSuccessCountOf('dependentProp'),
          ]),
        dependentProp_1: field
          .dependent('dependentProp_1')
          .default(0)
          .dependsOn('dependentProp')
          .resolve(resolverOfDependentProp_1 as never)
          .onDelete(incrementOnDeleteCountOf('dependentProp_1'))
          .onSuccess(incrementOnSuccessCountOf('dependentProp_1')),
        dependentProp_2: field
          .dependent('dependentProp_2')
          .default(0)
          .dependsOn('dependentProp')
          .resolve(asyncResolver('dependentProp_2') as never)
          .readonly()
          .onDelete([
            incrementOnDeleteCountOf('dependentProp_2'),
            incrementOnDeleteCountOf('dependentProp_2'),
          ])
          .onSuccess([
            incrementOnSuccessCountOf('dependentProp_2'),
            incrementOnSuccessCountOf('dependentProp_2'),
            incrementOnSuccessCountOf('dependentProp_2'),
          ]),
        dependentProp_3: field
          .dependent('dependentProp_3')
          .default(0)
          .dependsOn('laxProp_2')
          .resolve(asyncResolver('dependentProp_3') as never)
          .onDelete([
            incrementOnDeleteCountOf('dependentProp_3'),
            incrementOnDeleteCountOf('dependentProp_3'),
          ])
          .onSuccess([incrementOnSuccessCountOf('dependentProp_3')]),
      }).getModel();

      function incrementOnDeleteCountOf(prop: string) {
        return () => {
          const previousCount = onDeleteStats[prop] ?? 0;

          onDeleteStats[prop] = previousCount + 1;
        };
      }

      function incrementOnSuccessCountOf(prop: string) {
        return () => {
          const previousCount = onSuccessStats[prop] ?? 0;

          onSuccessStats[prop] = previousCount + 1;
        };
      }

      function incrementResolveCountOf(prop: string) {
        const previousCount = resolversCalledStats[prop] ?? 0;

        resolversCalledStats[prop] = previousCount + 1;
      }

      function resolverOfDependentProp(
        ctx: ReadonlyIvoContext<SampleInput, SampleOutput, {}>,
      ) {
        incrementResolveCountOf('dependentProp');
        const laxProp =
          ctx.rawInput.laxProp ?? ctx.input.laxProp ?? ctx.values.laxProp;
        const laxProp_1 =
          ctx.rawInput.laxProp_1 ?? ctx.input.laxProp_1 ?? ctx.values.laxProp_1;

        return laxProp.length + laxProp_1.length;
      }

      function resolverOfDependentProp_1(
        ctx: ReadonlyIvoContext<SampleInput, SampleOutput, {}>,
      ) {
        incrementResolveCountOf('dependentProp_1');

        return ctx.values.dependentProp + 1;
      }

      function asyncResolver(prop: string) {
        return (ctx: ReadonlyIvoContext<SampleInput, SampleOutput, {}>) => {
          incrementResolveCountOf(prop);

          return Promise.resolve(ctx.values.dependentProp + 2);
        };
      }

      afterEach(() => {
        onDeleteStats = {};
        onSuccessStats = {};
        resolversCalledStats = {};
      });

      describe('creation', () => {
        it('should resolve dependent properties correctly at creation', async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp_2: 'value based pricing',
            dependentProp: 25,
            dependentProp_1: 34,
            dependentProp_2: 17,
            dependentProp_3: 1,
          });

          await handleSuccess?.();

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2,
          });

          expect(resolversCalledStats).toEqual({ dependentProp_3: 1 });
          expect(onSuccessStats).toEqual(successCountOfDependentProps);
        });

        it('should resolve dependencies of dependent properties if provided at creation', async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: '',
            laxProp_1: 'hello',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
          });

          await handleSuccess();

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: 'hello',
            laxProp_2: '',
            dependentProp: 5,
            dependentProp_1: 6,
            dependentProp_2: 7,
            dependentProp_3: 0,
          });

          expect(resolversCalledStats).toEqual({
            dependentProp: 1,
            dependentProp_1: 1,
            dependentProp_2: 1,
          });

          expect(onSuccessStats).toEqual(successCountOfDependentProps);
        });
      });

      describe('updates', () => {
        it('should have all correct properties and values after updates', async () => {
          const { data: updates, handleSuccess } = await Model.update(
            {
              laxProp: '',
              laxProp_1: '',
              laxProp_2: 'value based pricing',
              dependentProp: 0,
              dependentProp_1: 0,
              dependentProp_2: 0,
              dependentProp_3: 2,
            },
            {
              laxProp_2: 'hey',
              dependentProp: 74,
              dependentProp_1: 235,
              dependentProp_2: 72,
              dependentProp_3: 702,
            },
          );

          await handleSuccess();

          expect(updates).toMatchObject({ laxProp_2: 'hey' });

          expect(resolversCalledStats).toEqual({ dependentProp_3: 1 });

          expect(onSuccessStats).toEqual({});
        });

        it('should resolve dependencies of dependent properties if provided during updates', async () => {
          const { data, handleSuccess } = await Model.update(
            {
              laxProp: '',
              laxProp_1: '',
              laxProp_2: '',
              dependentProp: 0,
              dependentProp_1: 0,
              dependentProp_2: 0,
              dependentProp_3: 0,
            },
            { laxProp: 'hello', laxProp_1: 'world' },
          );

          await handleSuccess();

          expect(data).toEqual({
            laxProp: 'hello',
            laxProp_1: 'world',
            dependentProp: 10,
            dependentProp_1: 11,
            dependentProp_2: 12,
          });

          const { dependentProp_3: _, ...stats } = successCountOfDependentProps;

          expect(resolversCalledStats).toEqual({
            dependentProp: 1,
            dependentProp_1: 1,
            dependentProp_2: 1,
          });

          expect(onSuccessStats).toEqual(stats);
        });

        it('should not resolve readonly dependent properties that have changed if provided during updates', async () => {
          const { data, handleSuccess } = await Model.update(
            {
              laxProp: 'hello',
              laxProp_1: 'world',
              dependentProp: 10,
              dependentProp_1: 11,
              dependentProp_2: 12,
              dependentProp_3: 0,
            },
            { laxProp: '', laxProp_1: 'hey' },
          );

          await handleSuccess();

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: 'hey',
            dependentProp: 3,
            dependentProp_1: 4,
          });

          const stats = {
            dependentProp: 1,
            dependentProp_1: 1,
          };

          expect(resolversCalledStats).toEqual(stats);

          expect(onSuccessStats).toEqual({
            dependentProp: 4,
            dependentProp_1: 1,
          });
        });

        it('should not consider new resolved values of dependent properties if they are not different from previous values during updates', async () => {
          const { data, handleSuccess } = await Model.update(
            {
              laxProp: 'hello',
              laxProp_1: 'world',
              dependentProp: 3,
              dependentProp_1: 4,
              dependentProp_2: 12,
              dependentProp_3: 0,
            },
            { laxProp: '', laxProp_1: 'hey' },
          );

          await handleSuccess();

          expect(data).toEqual({ laxProp: '', laxProp_1: 'hey' });

          expect(resolversCalledStats).toEqual({ dependentProp: 1 });

          expect(onSuccessStats).toEqual({});
        });
      });

      describe('deletion', () => {
        it('should have all correct properties and values at creation', async () => {
          await Model.delete({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2,
          });

          expect(onDeleteStats).toEqual({
            laxProp_2: 1,
            dependentProp: 2,
            dependentProp_1: 1,
            dependentProp_2: 2,
            dependentProp_3: 2,
          });
        });
      });

      describe('readonly resolver "unlock while == default / freeze once diverged" semantics', () => {
        // Mirrors rs/tests/fields/dependents.rs::
        // should_not_run_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value
        let resolverRunCount = 0;

        const Model = new Schema({
          parent: field.lax('parent').default(''),
          child: field
            .dependent('child')
            .default(0)
            .dependsOn('parent')
            .resolve(() => {
              resolverRunCount++;
              return 1;
            })
            .readonly(),
        }).getModel();

        it('does not run the resolver at creation if the parent was not provided (value stays at the raw static default)', async () => {
          resolverRunCount = 0;
          const { data } = await Model.create({});

          expect(data).toEqual({ parent: '', child: 0 });
          expect(resolverRunCount).toBe(0);
        });

        it('runs the resolver once creation is given the parent explicitly, moving the value off the default', async () => {
          resolverRunCount = 0;
          const { data } = await Model.create({ parent: 'x' });

          expect(data).toEqual({ parent: 'x', child: 1 });
          expect(resolverRunCount).toBe(1);
        });

        it('re-runs the resolver on the first update while the value still equals the default', async () => {
          resolverRunCount = 0;
          const { data } = await Model.update(
            { parent: '', child: 0 },
            { parent: 'x' },
          );

          expect(data).toEqual({ parent: 'x', child: 1 });
          expect(resolverRunCount).toBe(1);
        });

        it('freezes permanently once the value has diverged from the default, even if the parent changes again', async () => {
          resolverRunCount = 0;
          const { data } = await Model.update(
            { parent: 'x', child: 1 },
            { parent: 'y' },
          );

          expect(data).toEqual({ parent: 'y' });
          expect(resolverRunCount).toBe(0);
        });
      });

      describe('behaviour with errors thrown in the resolver', () => {
        const Model = new Schema({
          prop: field.lax('prop').default(''),
          dependent: field
            .dependent('dependent')
            .default('')
            .dependsOn('prop')
            .resolve(() => {
              throw new Error('lolol');
            }),
          dependent1: field
            .dependent('dependent1')
            .default('')
            .dependsOn('dependent')
            .resolve(() => {
              throw new Error('lolol');
            }),
          dependent2: field
            .dependent('dependent2')
            .default('')
            .dependsOn('dependent')
            .resolve(() => {
              throw new Error('lolol');
            }),
        }).getModel();

        it("should set dependent to null if error occurred resolving at creation'", async () => {
          const { data, error } = await Model.create({ prop: 'test' });

          expect(error).toBeNull();
          expect(data).toEqual({
            dependent: null,
            dependent1: null,
            dependent2: null,
            prop: 'test',
          });
        });

        it('should ignore dependent properties that error when resolving during updates', async () => {
          const { data, error } = await Model.update(
            { dependent: '', dependent1: '', dependent2: '', prop: '' },
            { prop: 'updated' },
          );

          expect(error).toBeNull();
          expect(data).toEqual({ prop: 'updated' });
        });
      });
    });

    describe('valid', () => {
      it('should accept dependent & default(any | function)', () => {
        const values = ['', 1, false, true, null, {}, []];

        for (const value of values) {
          const toPass = fx({
            dependentProp: field
              .dependent('dependentProp')
              .default(value)
              .dependsOn('prop')
              .resolve(resolver),
            prop: field.lax('prop').default(''),
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should allow dependsOn + resolver & no dependent', () => {
        const toPass = fx({
          dependentProp: field
            .dependent('dependentProp')
            .default('')
            .dependsOn('prop')
            .resolve(resolver),
          prop: field.lax('prop').default(''),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it("should accept life cycle listeners except 'onFailure'", () => {
        const lifeCycles = ['onDelete', 'onSuccess'] as const;
        const values = [() => {}, () => ({}), [() => {}, () => ({})]];

        for (const lifeCycle of lifeCycles) {
          for (const value of values) {
            const toPass = fx({
              dependentProp: field
                .dependent('dependentProp')
                .default(value)
                .dependsOn('prop')
                .resolve(resolver)
                [lifeCycle](value as never),
              prop: field.lax('prop').default(''),
            });

            expectNoFailure(toPass);

            toPass();
          }
        }
      });

      it('should accept dependsOn & resolver', () => {
        const values = [
          'prop',
          ['prop', 'prop1'],
          ['prop', 'prop1', 'prop2', 'prop3'],
        ];

        for (const dependsOn of values) {
          const toPass = fx({
            dependentProp: field
              .dependent('dependentProp')
              .default('')
              .dependsOn(dependsOn as never)
              .resolve(resolver),
            prop: field.lax('prop').default(''),
            prop1: field.lax('prop1').default(''),
            prop2: field.lax('prop2').default(''),
            prop3: field.lax('prop3').default(''),
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should allow a dependent prop to depend on another dependent prop (non-circular)', () => {
        const toPass = fx({
          dependentProp1: field
            .dependent('dependentProp1')
            .default('')
            .dependsOn('prop')
            .resolve(resolver),
          dependentProp2: field
            .dependent('dependentProp2')
            .default('')
            .dependsOn('dependentProp1')
            .resolve(resolver),
          prop: field.lax('prop').default(''),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow a dependency on virtuals', () => {
        const toPass = fx({
          dependentProp: field
            .dependent('dependentProp')
            .default('')
            .dependsOn('virtualProp')
            .resolve(resolver),
          virtualProp: field.virtual('virtualProp').validate(() => true),
        });

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe('invalid', () => {
      it('should reject dependency on non-properties', () => {
        const invalidProp = 'invalidProp';

        const toFail = fx({
          dependentProp: { default: '', dependsOn: invalidProp, resolver },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                `Cannot establish dependency with '${invalidProp}' as it is neither a property nor a virtual of your model`,
              ]),
            }),
          );
        }
      });

      it('should not allow property to depend on itself', () => {
        const toFail = fx({
          dependentProp: { default: '', dependsOn: 'dependentProp', resolver },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'A property cannot depend on itself',
              ]),
            }),
          );
        }
      });

      it('should not allow property to depend on a constant property', () => {
        const toFail = fx({
          constantProp: { constant: true, value: '' },
          dependentProp: { default: '', dependsOn: 'constantProp', resolver },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'A property cannot depend on a constant property',
              ]),
            }),
          );
        }
      });

      it('should identify circular dependencies and reject', () => {
        const toFail = fx({
          A: { default: '', dependsOn: ['B', 'C', 'D'], resolver },
          B: { default: '', dependsOn: ['A', 'C', 'E'], resolver },
          C: { default: '', dependsOn: ['A'], resolver },
          D: { default: '', dependsOn: 'E', resolver },
          E: { default: '', dependsOn: 'A', resolver },
          F: { default: '', dependsOn: 'prop', resolver },
          prop: { default: '' },
        });

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
        const toFail = fx({
          A: { default: '', dependsOn: 'prop', resolver },
          B: { default: '', dependsOn: ['A', 'prop'], resolver },
          C: { default: '', dependsOn: 'A', resolver },
          D: { default: '', dependsOn: ['prop', 'C'], resolver },
          prop: { default: '' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              B: expect.arrayContaining([
                "Dependency on 'prop' is redundant because of dependency on 'A'",
              ]),
              D: expect.arrayContaining([
                "Dependency on 'prop' is redundant because of dependency on 'C'",
              ]),
            }),
          );
        }
      });

      it('should reject dependent + missing dependsOn', () => {
        const toFail = fx({ propertyName: { default: '', resolver } });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'Dependent properties must depend on at least one property',
              ]),
            }),
          );
        }
      });

      it('should reject dependent + missing resolver', () => {
        const toFail = fx({
          dependentProp: { default: '', dependsOn: 'prop' },
          prop: { default: '' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties must have a resolver',
              ]),
            }),
          );
        }
      });

      it('should reject dependent & ignoreInit', () => {
        const values = [false, true, () => true];

        for (const ignoreInit of values) {
          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: 'prop',
              resolver,
              ignoreInit,
            },
            prop: { default: '' },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  'Dependent properties cannot have ignoreInit rule',
                ]),
              }),
            );
          }
        }
      });

      it('should reject dependent & virtual', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependsOn: 'prop',
            resolver,
            virtual: true,
          },
          prop: { default: '' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties cannot be virtual',
              ]),
            }),
          );
        }
      });

      it('should reject dependent & validator', () => {
        const values = [null, '', 1, true, false, validator];

        for (const validator of values) {
          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: 'prop',
              resolver,
              validator,
            },
            prop: { default: '' },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  'Dependent properties cannot be validated',
                ]),
              }),
            );
          }
        }
      });

      it('should reject dependent & required', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependsOn: 'prop',
            resolver,
            required: true,
          },
          prop: { default: '' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties cannot be required',
              ]),
            }),
          );
        }
      });

      it('should reject dependent + requiredBy', () => {
        const toFail = fx({
          dependentProp: {
            required() {
              return true;
            },
            default: '',
            dependsOn: 'prop',
            resolver: () => 1,
          },
          prop: { default: '' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties cannot be required',
              ]),
            }),
          );
        }
      });
    });
  });
};
