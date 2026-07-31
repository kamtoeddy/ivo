import { beforeEach, describe, expect, it } from 'bun:test';

import type { ReadonlyIvoContext } from '../../src';
import { createFieldBuilder } from '../../src/schema/fields';
import { DEFINITION_RULES, VIRTUAL_RULES } from '../../src/schema/types';

import { expectFailure, expectNoFailure, validator } from '../_utils';

const field = createFieldBuilder<any, any>();

export const Test_VirtualProperties = ({ Schema, fx }: any) => {
  describe('virtual', () => {
    describe('valid', () => {
      describe('alias', () => {
        it('should allow alias', () => {
          const toPass = fx({
            dependentProp: field
              .dependent('dependentProp')
              .default('')
              .dependsOn('propertyName')
              .resolve(() => ''),
            propertyName: field
              .virtual('propertyName')
              .alias('alias')
              .validate(validator)
              .sanitize(() => ''),
          });

          expectNoFailure(toPass);

          toPass();
        });

        it('should allow alias if it is the same as a related dependency of the virtual', () => {
          const dependentProp = 'dependentProp';
          const virtualProp = 'virtualProp';

          const toPass = fx({
            [dependentProp]: field
              .dependent(dependentProp)
              .default('')
              .dependsOn(virtualProp)
              .resolve(() => ''),
            [virtualProp]: field
              .virtual(virtualProp)
              .alias(dependentProp)
              .validate(validator)
              .sanitize(() => ''),
          });

          expectNoFailure(toPass);

          toPass();
        });

        describe('behaviour', () => {
          let contextRecord = {} as Record<string, number | undefined>;

          type QuantityInput = { qty?: number };
          type QuantityOutput = { id: number; quantity: number };

          function resolver({
            input: { qty },
          }: ReadonlyIvoContext<QuantityInput, QuantityOutput>) {
            if (qty !== undefined) contextRecord.qty = qty;

            return qty;
          }

          function validator(v: any) {
            const _type = typeof v;
            return _type === 'number'
              ? { valid: true, validated: v }
              : { valid: false, reason: 'Invalid quantity' };
          }

          const Model = new Schema({
            id: field
              .constant('id')
              .value(1)
              .onDelete(resolver as never),
            quantity: field
              .dependent('quantity')
              .default(0.0)
              .dependsOn('setQuantity')
              .resolve(resolver as never),
            setQuantity: field
              .virtual('setQuantity')
              .alias('qty')
              .validate(validator),
          }).getModel();

          beforeEach(() => {
            contextRecord = {};
          });

          describe('creation', () => {
            it('should respect alias if provided at creation', async () => {
              const qty = 12;
              const { data } = await Model.create({ qty });

              expect(data).toMatchObject({ id: 1, quantity: qty });
              expect(contextRecord).toEqual({ qty });
            });

            it("should use default values of dependent props to be set if an alias with that prop's name exists on the same schema but initialization is blocked", async () => {
              const Model = new Schema({
                id: field
                  .constant('id')
                  .value(1)
                  .onDelete(resolver as never),
                quantity: field
                  .dependent('quantity')
                  .default(0.0)
                  .dependsOn('setQuantity')
                  .resolve(resolver as never),
                setQuantity: field
                  .virtual('setQuantity')
                  .alias('quantity')
                  .validate(validator)
                  .ignoreInit(),
              }).getModel();

              const { data } = await Model.create({ quantity: 12 });

              expect(data).toMatchObject({ id: 1, quantity: 0 });
              expect(contextRecord).toEqual({});
            });

            it('should return alias errors with alias name in error payload at creation', async () => {
              const { error } = await Model.create({ qty: '12' });

              expect(error).toMatchObject({
                qty: {
                  reason: 'Invalid quantity',
                  metadata: null,
                },
              });
              expect(contextRecord).toEqual({});
            });
          });

          describe('delete', () => {
            it('aliases should not be available in context during deletion', async () => {
              await Model.delete({ id: 1, quantity: 12, qty: 1000 });

              expect(contextRecord).toEqual({});
            });
          });

          describe('update', () => {
            it('should respect alias if provided during updates', async () => {
              const qty = 5;
              const { data } = await Model.update(
                { id: 1, quantity: 12 },
                { qty },
              );

              expect(data).toMatchObject({ quantity: qty });
              expect(contextRecord).toEqual({ qty });
            });

            it('should return alias errors with alias name in error payload during updates', async () => {
              const { error } = await Model.update(
                { id: 1, quantity: 12 },
                { qty: '2' },
              );

              expect(error).toMatchObject({
                qty: { reason: 'Invalid quantity', metadata: null },
              });
              expect(contextRecord).toEqual({});
            });
          });

          describe("availability of virtuals in ctx of 'required' method of virtual", () => {
            const Model = new Schema({
              id: field.constant('id').value(1),
              note: field.lax('note').default(''),
              quantity: field
                .dependent('quantity')
                .default(0.0)
                .dependsOn('setQuantity')
                .resolve(resolver as never),
              setQuantity: field
                .virtual('setQuantity')
                .alias('qty')
                .validate(validator)
                .required(({ input: { setQuantity } }: any) => {
                  contextRecord.setQuantity = setQuantity;

                  return true;
                }),
            }).getModel();

            it("should make ctx.input available (keyed by the virtual's config name, not its alias) inside 'required' at creation", async () => {
              const operation = await Model.create({ id: 1 });

              expect(contextRecord).toEqual({ setQuantity: undefined });
              expect(operation.data).toBe(null);
              expect(operation.error).toEqual({
                qty: { reason: "'qty' is required", metadata: null },
              });
            });

            it("should make ctx.input available (keyed by the virtual's config name, not its alias) inside 'required' during updates", async () => {
              const entity = { id: 1, note: '', quantity: 100 };
              // a genuine, unrelated change so the update isn't a no-op —
              // `qty` itself stays unprovided, so `required` still fires for it.
              const operation = await Model.update(entity, { note: 'hey' });

              expect(contextRecord).toEqual({ setQuantity: undefined });
              expect(operation.data).toBe(null);
              expect(operation.error).toEqual({
                qty: { reason: "'qty' is required", metadata: null },
              });
            });
          });

          describe("availability of virtuals in ctx of ignoreInit & ignoreUpdate methods of the virtual when it's alias is provided", () => {
            const Model = new Schema({
              id: field
                .constant('id')
                .value(1)
                .onDelete(resolver as never),
              quantity: field
                .dependent('quantity')
                .default(0.0)
                .dependsOn('setQuantity')
                .resolve(resolver as never),
              setQuantity: field
                .virtual('setQuantity')
                .alias('qty')
                .validate(validator)
                .ignoreInit(({ input: { qty } }: any) => {
                  contextRecord.setQuantity = qty;

                  return (qty ?? 0) <= 0;
                })
                .ignoreUpdate(
                  ({ input: { qty }, values: { quantity } }: any) => {
                    contextRecord.setQuantity = qty;

                    return (qty ?? 0) <= quantity;
                  },
                ),
            }).getModel();

            it("should respect 'ignoreInit' rule of virtual property even when alias is provided at creation", async () => {
              const operation1 = await Model.create({ id: 1, qty: -75 });

              expect(contextRecord).toEqual({ setQuantity: -75 });
              expect(operation1.error).toBe(null);
              expect(operation1.data).toEqual({ id: 1, quantity: 0 });

              const qty = 75;

              const operation2 = await Model.create({ id: 1, qty });

              expect(contextRecord).toEqual({ qty, setQuantity: qty });
              expect(operation2.error).toBe(null);
              expect(operation2.data).toEqual({ id: 1, quantity: qty });
            });

            it("should respect 'ignoreUpdate' rule of virtual property even when alias is provided during updates", async () => {
              let qty = 12;
              const operation1 = await Model.update(
                { id: 1, quantity: 75 },
                { qty },
              );

              expect(contextRecord).toEqual({ setQuantity: qty });
              expect(operation1.error).toBeNull();
              expect(operation1.data).toBe(null);

              qty = 100;

              const operation2 = await Model.update(
                { id: 1, quantity: 75 },
                { qty },
              );

              expect(contextRecord).toEqual({ qty, setQuantity: qty });
              expect(operation2.error).toBe(null);
              expect(operation2.data).toMatchObject({ quantity: qty });
            });
          });
        });

        describe('behaviour with validation & required errors and alias with different name', () => {
          const Model = new Schema({
            dependent: field
              .dependent('dependent')
              .default(0.0)
              .dependsOn('_virtual')
              .resolve(() => 1),
            _virtual: field
              .virtual('_virtual')
              .alias('virtual')
              .validate((v: any) => v === 'valid')
              .required(() => true),
          }).getModel();

          describe('creation', () => {
            it('should return alias name as error key if provided and validation fails at creation', async () => {
              const { error } = await Model.create({ virtual: '5' });

              expect(error).toMatchObject({
                virtual: {
                  reason: 'validation failed',
                  metadata: null,
                },
              });
              expect(error._virtual).toBeUndefined();
            });

            it('should return alias name as error key in case of required error at creation', async () => {
              const { error } = await Model.create({});

              expect(error).toMatchObject({
                virtual: {
                  reason: "'virtual' is required",
                  metadata: null,
                },
              });
              expect(error._virtual).toBeUndefined();
            });
          });

          describe('updates', () => {
            const validData = { dependent: 20 };

            it('should return alias name as error key if provided and validation fails during updates', async () => {
              const { error } = await Model.update(validData, { virtual: '5' });

              expect(error).toMatchObject({
                virtual: {
                  reason: 'validation failed',
                  metadata: null,
                },
              });
              expect(error._virtual).toBeUndefined();
            });
          });
        });

        describe('behaviour with validation & required errors and alias with name of dependent prop', () => {
          const Model = new Schema({
            dependent: field
              .dependent('dependent')
              .default(0.0)
              .dependsOn('_virtual')
              .resolve(() => 1),
            _virtual: field
              .virtual('_virtual')
              .alias('dependent')
              .validate((v: any) => v === 'valid')
              .required(() => true),
          }).getModel();

          describe('creation', () => {
            it('should return alias name as error key if provided and validation fails at creation', async () => {
              const { error } = await Model.create({ dependent: '5' });

              expect(error).toMatchObject({
                dependent: {
                  reason: 'validation failed',
                  metadata: null,
                },
              });
              expect(error._virtual).toBeUndefined();
            });

            it('should return alias name as error key in case of required error at creation', async () => {
              const { error } = await Model.create({});

              expect(error).toMatchObject({
                dependent: {
                  reason: "'dependent' is required",
                  metadata: null,
                },
              });
              expect(error._virtual).toBeUndefined();
            });
          });

          describe('updates', () => {
            const validData = { dependent: 20 };

            it('should return alias name as error key if provided and validation fails during updates', async () => {
              const { error } = await Model.update(validData, {
                dependent: '5',
              });

              expect(error).toMatchObject({
                dependent: { reason: 'validation failed', metadata: null },
              });
              expect(error._virtual).toBeUndefined();
            });
          });
        });
      });

      it('should allow sanitizer', () => {
        const toPass = fx({
          dependentProp: field
            .dependent('dependentProp')
            .default('')
            .dependsOn('propertyName')
            .resolve(() => ''),
          propertyName: field
            .virtual('propertyName')
            .validate(validator)
            .sanitize(() => ''),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow onFailure', () => {
        const toPass = fx({
          dependentProp: field
            .dependent('dependentProp')
            .default('')
            .dependsOn('propertyName')
            .resolve(() => ''),
          propertyName: field
            .virtual('propertyName')
            .validate(validator)
            .onFailure(validator as never),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow requiredBy', () => {
        const toPass = fx({
          dependentProp: field
            .dependent('dependentProp')
            .default('')
            .dependsOn('propertyName')
            .resolve(() => ''),
          propertyName: field
            .virtual('propertyName')
            .validate(validator)
            .required(() => true),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow ignoreInit(true|()=>boolean) + validator', () => {
        const values = [true, () => false, () => true];

        for (const ignoreInit of values) {
          const toPass = fx({
            dependentProp: field
              .dependent('dependentProp')
              .default('')
              .dependsOn('propertyName')
              .resolve(() => ''),
            propertyName:
              ignoreInit === true
                ? field.virtual('propertyName').validate(validator).ignoreInit()
                : field
                    .virtual('propertyName')
                    .validate(validator)
                    .ignoreInit(ignoreInit as never),
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should allow onSuccess + validator', () => {
        const values = [[], () => ({})];

        for (const onSuccess of values) {
          const toPass = fx({
            dependentProp: field
              .dependent('dependentProp')
              .default('')
              .dependsOn('propertyName')
              .resolve(() => ''),
            propertyName: field
              .virtual('propertyName')
              .validate(validator)
              .onSuccess(onSuccess as never),
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      describe('behaviour', () => {
        const onSuccessValues: Record<string, unknown> = {};
        const onSuccessStats: Record<string, number> = {};
        const sanitizedValues: Record<string, unknown> = {};

        const User = new Schema({
          dependentSideInit: field
            .dependent('dependentSideInit')
            .default('')
            .dependsOn(['virtualInit', 'virtualWithSanitizer'])
            .resolve(({ input: { virtualInit, virtualWithSanitizer } }: any) =>
              virtualInit && virtualWithSanitizer ? 'both' : 'one',
            )
            .onSuccess(onSuccess('dependentSideInit')),
          dependentSideNoInit: field
            .dependent('dependentSideNoInit')
            .default('')
            .dependsOn(['virtualNoInit', 'virtualWithSanitizerNoInit'])
            .resolve(() => 'changed')
            .onSuccess(onSuccess('dependentSideNoInit')),
          name: field.lax('name').default(''),
          virtualInit: field
            .virtual('virtualInit')
            .validate(validateBoolean as never)
            .onSuccess(onSuccess('virtualInit')),
          virtualNoInit: field
            .virtual('virtualNoInit')
            .validate(validateBoolean as never)
            .ignoreInit()
            .onSuccess([
              onSuccess('virtualNoInit'),
              incrementOnSuccessStats('virtualNoInit'),
            ]),
          virtualWithSanitizer: field
            .virtual('virtualWithSanitizer')
            .validate(validateBoolean as never)
            .sanitize(sanitizerOf('virtualWithSanitizer', 'sanitized'))
            .onSuccess([
              onSuccess('virtualWithSanitizer'),
              incrementOnSuccessStats('virtualWithSanitizer'),
              incrementOnSuccessStats('virtualWithSanitizer'),
            ]),
          virtualWithSanitizerNoInit: field
            .virtual('virtualWithSanitizerNoInit')
            .validate(validateBoolean as never)
            .ignoreInit()
            .sanitize(
              sanitizerOf('virtualWithSanitizerNoInit', 'sanitized no init'),
            )
            .onSuccess([
              onSuccess('virtualWithSanitizerNoInit'),
              incrementOnSuccessStats('virtualWithSanitizerNoInit'),
            ]),
        }).getModel();

        function sanitizerOf(prop: string, value: any) {
          return () => {
            sanitizedValues[prop] = value;

            return value;
          };
        }

        function incrementOnSuccessStats(prop: string) {
          return () => {
            onSuccessStats[prop] = (onSuccessStats[prop] ?? 0) + 1;
          };
        }

        type UserInput = {
          virtualInit?: boolean;
          virtualNoInit?: boolean;
          virtualWithSanitizer?: boolean;
          virtualWithSanitizerNoInit?: boolean;
        };
        type UserOutput = {
          name: string;
          dependentSideInit: string;
          dependentSideNoInit: string;
        };

        function onSuccess(prop: string) {
          return (context: ReadonlyIvoContext<UserInput, UserOutput>) => {
            onSuccessValues[prop] =
              (context.values as Record<string, unknown>)?.[prop] ??
              (context.input as Record<string, unknown>)?.[prop];
            incrementOnSuccessStats(prop)();
          };
        }

        function validateBoolean(value: any) {
          if ([false, true].includes(value)) return true;

          return { valid: false, reason: `${value} is not a boolean` };
        }

        beforeEach(() => {
          for (const key of Object.keys(onSuccessStats))
            delete onSuccessStats[key];
          for (const key of Object.keys(onSuccessValues))
            delete onSuccessValues[key];
          for (const key of Object.keys(sanitizedValues))
            delete sanitizedValues[key];
        });

        describe('creation', () => {
          it('should not sanitize virtuals nor resolve their dependencies if not provided', async () => {
            const { data } = await User.create({ name: 'Peter' });

            expect(data).toEqual({
              dependentSideInit: '',
              dependentSideNoInit: '',
              name: 'Peter',
            });

            expect(sanitizedValues).toEqual({});
          });

          it('should respect sanitizer at creation', async () => {
            const { data } = await User.create({
              name: 'Peter',
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true,
            });

            expect(data).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: '',
              name: 'Peter',
            });

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized',
            });
          });

          it('should respect virtualInits & virtualNoInit at creation', async () => {
            const { data: user, handleSuccess } = await User.create({
              dependentSideNoInit: '',
              dependentSideInit: true,
              name: 'Peter',
              virtualInit: true,
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true,
            });

            await handleSuccess();

            expect(user).toEqual({
              dependentSideInit: 'both',
              dependentSideNoInit: '',
              name: 'Peter',
            });

            expect(onSuccessStats).toEqual({
              dependentSideInit: 1,
              dependentSideNoInit: 1,
              virtualInit: 1,
              virtualWithSanitizer: 3,
            });

            expect(onSuccessValues).toEqual({
              dependentSideInit: 'both',
              dependentSideNoInit: '',
              virtualInit: true,
              virtualWithSanitizer: 'sanitized',
            });

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized',
            });
          });
        });

        describe('updating', () => {
          it('should respect sanitizer of all virtuals provided during updates', async () => {
            const { data, handleSuccess } = await User.update(
              { name: 'Peter' },
              {
                name: 'John',
                virtualWithSanitizer: true,
                virtualWithSanitizerNoInit: true,
              },
            );

            await handleSuccess();

            expect(data).toEqual({
              name: 'John',
              dependentSideInit: 'one',
              dependentSideNoInit: 'changed',
            });

            expect(onSuccessStats).toEqual({
              dependentSideInit: 1,
              dependentSideNoInit: 1,
              virtualWithSanitizer: 3,
              virtualWithSanitizerNoInit: 2,
            });

            expect(onSuccessValues).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: 'changed',
              virtualWithSanitizer: 'sanitized',
              virtualWithSanitizerNoInit: 'sanitized no init',
            });

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized',
              virtualWithSanitizerNoInit: 'sanitized no init',
            });
          });
        });

        describe('behaviour with errors thrown in the sanitizer', () => {
          const Model = new Schema({
            dependent: field
              .dependent('dependent')
              .default('')
              .dependsOn('virtual')
              .resolve(
                (context: any) =>
                  context.input?.virtual ?? context.rawInput?.virtual,
              ),
            virtual: field
              .virtual('virtual')
              .validate(() => true)
              .sanitize(() => {
                throw new Error('lolol');
              }),
          }).getModel();

          const values = [null, '', 1, 0, -1, true, false, [], {}];

          it('should use the validated value at creation', async () => {
            for (const virtual of values) {
              const { data, error } = await Model.create({ virtual });

              expect(error).toBeNull();
              expect(data).toMatchObject({ dependent: virtual });
            }
          });

          it('should use the validated value during updates', async () => {
            for (const virtual of values) {
              const { data, error } = await Model.update(
                { dependent: 'lolol' },
                { virtual },
              );

              expect(error).toBeNull();
              expect(data).toMatchObject({ dependent: virtual });
            }
          });
        });
      });
    });

    describe('invalid', () => {
      describe('alias', () => {
        it('should reject alias if definition does not have the virtual keyword', () => {
          const virtualProp = 'virtualProp';

          const toFail = fx({
            required: { alias: 'a1', required: true, validator },
            readonly: { alias: 's2', readonly: true, validator },
            lax1: { alias: 'a3', default: '' },
            lax2: { alias: 'a5', default: '', validator },
            dependentProp: {
              alias: 'lol',
              default: '',
              dependsOn: virtualProp,
              resolver: () => '',
            },
            [virtualProp]: { virtual: true, validator },
          });

          expectFailure(toFail);

          const expectedError = expect.arrayContaining([
            'Only virtual properties can have aliases',
          ]);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                required: expectedError,
                readonly: expectedError,
                lax1: expectedError,
                lax2: expectedError,
                dependentProp: expectedError,
              }),
            );
          }
        });

        it('should reject alias if non-empty string is provided', () => {
          const values = [-1, 1, true, false, undefined, '', null, [], {}];

          for (const alias of values) {
            const toFail = fx({
              dependentProp: {
                default: '',
                dependsOn: 'propertyName',
                resolver: () => '',
              },
              propertyName: { alias, virtual: true, validator },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    'An alias must be a string with at least 1 character',
                  ]),
                }),
              );
            }
          }
        });

        it("should reject alias if it's same as the virtual property", () => {
          const virtualProp = 'virtualProp';

          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: virtualProp,
              resolver: () => '',
            },
            [virtualProp]: { alias: virtualProp, virtual: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  'An alias cannot be the same as the virtual property',
                ]),
              }),
            );
          }
        });

        it('should reject alias if already used by another virtual', () => {
          const alias = 'alias';
          const virtualProp = 'virtualProp';

          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: [virtualProp, 'virtualProp1'],
              resolver: () => '',
            },
            [virtualProp]: { alias, virtual: true, validator },
            virtualProp1: { alias, virtual: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                virtualProp1: expect.arrayContaining([
                  `Sorry, alias provided '${alias}' already belongs to property '${virtualProp}'`,
                ]),
              }),
            );
          }
        });

        it('should reject alias if it is the same as the name of existing virtual', () => {
          const alias = 'virtualProp1';
          const virtualProp = 'virtualProp';

          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: [virtualProp, 'virtualProp1'],
              resolver: () => '',
            },
            [virtualProp]: { alias, virtual: true, validator },
            virtualProp1: { virtual: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  `'${alias}' cannot be used as the alias of '${virtualProp}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
                ]),
              }),
            );
          }
        });

        it('should reject alias if it is the same as the name of existing property', () => {
          const laxProp = 'laxProp';
          const virtualProp = 'virtualProp';

          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: virtualProp,
              resolver: () => '',
            },
            [virtualProp]: { alias: laxProp, virtual: true, validator },
            [laxProp]: { default: true },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  `'${laxProp}' cannot be used as the alias of '${virtualProp}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
                ]),
              }),
            );
          }
        });

        it('should reject alias if it is the same as an unrelated dependent property', () => {
          const dependentProp = 'dependentProp';
          const virtualProp = 'virtualProp';

          const toFail = fx({
            [dependentProp]: {
              default: '',
              dependsOn: virtualProp,
              resolver: () => '',
            },
            [virtualProp]: {
              alias: 'dependentProp1',
              virtual: true,
              validator,
            },
            dependentProp1: {
              default: '',
              dependsOn: 'virtualProp1',
              resolver: () => '',
            },
            virtualProp1: { virtual: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  `'dependentProp1' cannot be used as the alias of '${virtualProp}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`,
                ]),
              }),
            );
          }
        });
      });

      describe('sanitizers', () => {
        it('should reject invalid sanitizer', () => {
          const values = [-1, 1, true, false, undefined, null, [], {}];

          for (const sanitizer of values) {
            const toFail = fx({
              propertyName: { virtual: true, sanitizer, validator },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "'sanitizer' must be a function",
                  ]),
                }),
              );
            }
          }
        });

        it("should reject 'sanitizer' rule on non-virtuals", () => {
          const values = [
            -1,
            1,
            true,
            false,
            undefined,
            null,
            [],
            {},
            () => {},
          ];

          for (const sanitizer of values) {
            const toFail = fx({ propertyName: { default: '', sanitizer } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "'sanitizer' is only valid on virtuals",
                  ]),
                }),
              );
            }
          }
        });
      });

      it('should reject virtual & no dependent property ', () => {
        const toFail = fx({ propertyName: { virtual: true, validator } });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: [
                'A virtual property must have at least one property that depends on it',
              ],
            }),
          );
        }
      });

      it('should reject virtual & no validator ', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependsOn: 'propertyName',
            resolver: () => '',
          },
          propertyName: { virtual: true },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining(['Invalid validator']),
            }),
          );
        }
      });

      it('should reject requiredBy + ignoreInit', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependsOn: 'propertyName',
            resolver: () => '',
          },
          propertyName: {
            virtual: true,
            ignoreInit: true,
            required: () => true,
            validator,
          },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'Required virtuals cannot have initialization blocked',
              ]),
            }),
          );
        }
      });

      it('should reject required(true)', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependsOn: 'propertyName',
            resolver: () => '',
          },
          propertyName: { virtual: true, required: true, validator },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'Callable required properties must have required as a function',
              ]),
            }),
          );
        }
      });

      it('should reject any non virtual rule', () => {
        const values = DEFINITION_RULES.filter(
          (rule) => !VIRTUAL_RULES.includes(rule),
        );

        for (const rule of values) {
          const toFail = fx({
            dependentProp: {
              default: '',
              dependsOn: 'propertyName',
              resolver: () => '',
            },
            propertyName: { virtual: true, [rule]: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              propertyName: expect.arrayContaining([
                `Virtual properties can only have (${VIRTUAL_RULES.join(
                  ', ',
                )}) as rules`,
              ]),
            });
          }
        }
      });
    });
  });
};
