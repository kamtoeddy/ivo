import { afterEach, beforeAll, beforeEach, describe, it, expect } from 'vitest';

import { ERRORS } from '../../../../dist';
import { expectFailure, expectNoFailure, validator } from '../_utils';

export const Test_ShouldInitAndUpdateRules = ({ Schema, fx }: any) => {
  describe('Both', () => {
    describe('valid', () => {
      it('should accept shouldInit & shouldUpdate for lax props', () => {
        // [shouldInit, shouldUpdate]
        const values = [
          [false, () => {}],
          [() => {}, false],
          [() => {}, () => {}]
        ];

        for (const [shouldInit, shouldUpdate] of values) {
          const toPass = fx({
            propertyName: { default: '', shouldInit, shouldUpdate }
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should accept shouldInit(() => boolean) + shouldUpdate(false | () => boolean) + readonly(true)', () => {
        // [shouldInit, shouldUpdate]
        const readonlyTrue = [
          [false, () => {}],
          [() => {}, false],
          [() => {}, () => {}]
        ];

        for (const [shouldInit, shouldUpdate] of readonlyTrue) {
          const toPass = fx({
            dependentProp: {
              default: '',
              readonly: true,
              shouldInit,
              shouldUpdate,
              validator
            }
          });

          expectNoFailure(toPass);

          toPass();
        }

        const toPass = fx({
          dependentProp: {
            default: '',
            readonly: 'lax',
            shouldUpdate: () => {},
            validator
          }
        });

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe('invalid', () => {
      it('should reject shouldUpdate == false & shouldInit == false', () => {
        const toFail = fx({
          propertyName: {
            default: '',
            shouldInit: false,
            shouldUpdate: false
          }
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Both 'shouldInit' & 'shouldUpdate' cannot be 'false'"
              ])
            })
          );
        }
      });

      describe('Readonly lax', () => {
        it("should reject readonly('lax') + shouldInit", () => {
          for (const shouldInit of [false, () => {}]) {
            const toFail = fx({
              propertyName: {
                default: '',
                readonly: 'lax',
                shouldInit,
                validator
              }
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    'Lax properties cannot have initialization blocked'
                  ])
                })
              );
            }
          }
        });

        it("should reject readonly('lax') + shouldUpdate(false)", () => {
          const toFail = fx({
            propertyName: {
              default: '',
              readonly: 'lax',
              shouldUpdate: false,
              validator
            }
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  'Readonly(lax) properties cannot have updates strictly blocked'
                ])
              })
            );
          }
        });
      });
    });
  });

  describe('shouldInit', () => {
    describe('valid', () => {
      it('should accept shouldInit(false) + default', () => {
        const fxn = fx({
          propertyName: { shouldInit: false, default: true }
        });

        expectNoFailure(fxn);

        fxn();
      });

      it('should accept shouldInit: () => boolean + default', () => {
        const values = [() => true, () => false];

        for (const shouldInit of values) {
          const fxn = fx({
            propertyName: { shouldInit, default: true }
          });

          expectNoFailure(fxn);

          fxn();
        }
      });

      describe('behaviour', () => {
        const Model = new Schema({
          isBlocked: {
            default: false,
            shouldInit: (ctx: any) => ctx.env == 'test'
          },
          env: { default: 'dev' },
          laxProp: { default: 0 }
        }).getModel();

        it('should respect default rules', async () => {
          const { data } = await Model.create({ isBlocked: true });

          expect(data).toMatchObject({
            env: 'dev',
            isBlocked: false,
            laxProp: 0
          });
        });

        it('should respect callable should init when condition passes during cloning', async () => {
          const { data } = await Model.clone({
            env: 'test',
            isBlocked: 'yes'
          });

          expect(data).toMatchObject({
            env: 'test',
            isBlocked: 'yes',
            laxProp: 0
          });
        });

        it('should respect callable should init when condition passes at creation', async () => {
          const { data } = await Model.create({
            env: 'test',
            isBlocked: 'yes'
          });

          expect(data).toMatchObject({
            env: 'test',
            isBlocked: 'yes',
            laxProp: 0
          });
        });

        describe('behaviour when shouldInit method returns nothing', () => {
          const Model = new Schema({
            isBlocked: { default: false, shouldInit: () => {} },
            laxProp: { default: 0 }
          }).getModel();

          it('should assume initialization as falsy if shouldInit method returns nothing at creation', async () => {
            const { data } = await Model.create({ isBlocked: 'yes' });

            expect(data).toMatchObject({ isBlocked: false, laxProp: 0 });
          });

          it('should assume initialization as falsy if shouldInit method returns nothing during cloning', async () => {
            const { data } = await Model.clone({ isBlocked: true, laxProp: 0 });

            expect(data).toMatchObject({ isBlocked: false, laxProp: 0 });
          });
        });
      });

      describe('behaviour of callable shouldInit', () => {
        let onSuccessValues: any = {};

        let onSuccessStats: any = {};

        let sanitizedValues: any = {};

        let Model: any;

        beforeAll(() => {
          Model = new Schema(
            {
              dependent: {
                default: '',
                dependent: true,
                dependsOn: 'virtual',
                resolver: () => 'changed',
                onSuccess: onSuccess('dependent')
              },
              laxProp: { default: '' },
              virtual: {
                virtual: true,
                shouldInit: ({ laxProp }: any) => laxProp === 'allow virtual',
                onSuccess: [
                  onSuccess('virtual'),
                  incrementOnSuccessStats('virtual'),
                  incrementOnSuccessStats('virtual')
                ],
                sanitizer: sanitizerOf('virtual', 'sanitized'),
                validator: validateBoolean
              }
            },
            { errors: 'throw' }
          ).getModel();

          function sanitizerOf(prop: string, value: any) {
            return () => {
              // to make sure sanitizer is invoked
              sanitizedValues[prop] = value;

              return value;
            };
          }

          function incrementOnSuccessStats(prop: string) {
            return () => {
              onSuccessStats[prop] = (onSuccessStats[prop] ?? 0) + 1;
            };
          }

          function onSuccess(prop: string) {
            return ({ context }: any) => {
              onSuccessValues[prop] = context[prop];
              incrementOnSuccessStats(prop)();
            };
          }

          function validateBoolean(value: any) {
            if (![false, true].includes(value))
              return { valid: false, reason: `${value} is not a boolean` };
            return { valid: true };
          }
        });

        beforeEach(() => {
          onSuccessStats = {};
          onSuccessValues = {};
          sanitizedValues = {};
        });

        it("should ignore virtuals at creation when their shouldInit handler returns 'false'", async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: 'Peter',
            virtual: true
          });

          await handleSuccess();

          expect(data).toEqual({ dependent: '', laxProp: 'Peter' });

          expect(onSuccessStats).toEqual({ dependent: 1 });

          expect(onSuccessValues).toEqual({ dependent: '' });

          expect(sanitizedValues).toEqual({});
        });

        it("should ignore virtuals at creation(cloning) when their shouldInit handler returns 'false'", async () => {
          const { data, handleSuccess } = await Model.clone({
            dependent: '',
            laxProp: 'Peter',
            virtual: true
          });

          await handleSuccess();

          expect(data).toEqual({ dependent: '', laxProp: 'Peter' });

          expect(onSuccessStats).toEqual({ dependent: 1 });

          expect(onSuccessValues).toEqual({ dependent: '' });

          expect(sanitizedValues).toEqual({});
        });

        it("should respect virtuals at creation when their shouldInit handler returns 'true'", async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: 'allow virtual',
            virtual: true
          });

          await handleSuccess();

          expect(data).toEqual({
            dependent: 'changed',
            laxProp: 'allow virtual'
          });

          expect(onSuccessStats).toEqual({ dependent: 1, virtual: 3 });

          expect(onSuccessValues).toEqual({
            dependent: 'changed',
            virtual: 'sanitized'
          });

          expect(sanitizedValues).toEqual({ virtual: 'sanitized' });
        });

        it("should respect virtuals at creation(cloning) when their shouldInit handler returns 'true'", async () => {
          const { data, handleSuccess } = await Model.clone({
            dependent: '',
            laxProp: 'allow virtual',
            virtual: true
          });

          await handleSuccess();

          expect(data).toEqual({
            dependent: 'changed',
            laxProp: 'allow virtual'
          });

          expect(onSuccessStats).toEqual({ dependent: 1, virtual: 3 });

          expect(onSuccessValues).toEqual({
            dependent: 'changed',
            virtual: 'sanitized'
          });

          expect(sanitizedValues).toEqual({ virtual: 'sanitized' });
        });
      });
    });

    describe('invalid', () => {
      it('should reject shouldInit(false) & no default', () => {
        const fxn = fx({ propertyName: { shouldInit: false } });

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'A property with initialization blocked must have a default value'
              ])
            })
          );
        }
      });

      it('should reject shouldInit !(boolean | () => boolean)', () => {
        const values = [undefined, 1, {}, null, [], 'yes', 'false', 'true'];

        for (const shouldInit of values) {
          const fxn = fx({ propertyName: { shouldInit, default: true } });

          expectFailure(fxn);

          try {
            fxn();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean"
                ])
              })
            );
          }
        }
      });
    });
  });

  describe('shouldUpdate', () => {
    describe('valid', () => {
      it('should accept shouldUpdate(() => boolean)', () => {
        const validValues = [() => false, () => true];

        for (const shouldUpdate of validValues) {
          const toPass = fx({ propertyName: { default: '', shouldUpdate } });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should accept shouldInit(() => boolean) & shouldUpdate(false) for virtuals', () => {
        const values = [() => true, () => false];

        for (const shouldInit of values) {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'virtual',
              resolver: () => ''
            },
            virtual: {
              virtual: true,
              shouldInit,
              shouldUpdate: false,
              validator
            }
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      describe('behaviour', () => {
        let onSuccessValues: any = {};
        let onSuccessStats: any = {};

        function incrementOnSuccessCountOf(prop: string) {
          return ({ context }: any) => {
            const previousCount = onSuccessStats[prop] ?? 0;

            onSuccessStats[prop] = previousCount + 1;
            onSuccessValues[prop] = context[prop];
          };
        }

        const Model = new Schema({
          dependentProp: {
            default: false,
            dependent: true,
            dependsOn: 'virtual',
            resolver: ({ context }: any) => context.virtual,
            onSuccess: incrementOnSuccessCountOf('dependentProp')
          },
          dependentProp_1: {
            default: false,
            dependent: true,
            dependsOn: 'virtual_1',
            resolver: ({ context }: any) => context.virtual_1,
            onSuccess: incrementOnSuccessCountOf('dependentProp_1')
          },
          laxProp: {
            default: '',
            readonly: 'lax',
            shouldUpdate: (ctx: any) => ctx.laxProp_1 == 'test',
            onSuccess: incrementOnSuccessCountOf('laxProp')
          },
          laxProp_1: { default: 'dev' },
          virtual: {
            virtual: true,
            shouldUpdate: false,
            validator: () => ({ valid: true }),
            onSuccess: incrementOnSuccessCountOf('virtual')
          },
          virtual_1: {
            virtual: true,
            shouldUpdate: (ctx: any) => ctx.laxProp_1 == 'test',
            validator: () => ({ valid: true }),
            onSuccess: incrementOnSuccessCountOf('virtual_1')
          }
        }).getModel();

        afterEach(() => {
          onSuccessValues = {};
          onSuccessStats = {};
        });

        it("should not update properties when 'shouldUpdate' resolved to 'false'", async () => {
          const { data, error } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: '',
              laxProp_1: ''
            },
            { laxProp: 'yoyo', virtual: true, virtual_1: true }
          );

          expect(data).toBe(null);
          expect(error.message).toBe(ERRORS.NOTHING_TO_UPDATE);
        });

        it("should update properties when 'shouldUpdate' resolved to 'true'", async () => {
          const { data, error, handleSuccess } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: '',
              laxProp_1: 'test'
            },
            { laxProp: 'yoyo', virtual: true, virtual_1: true }
          );

          await handleSuccess();

          expect(error).toBe(null);
          expect(data).toEqual({ dependentProp_1: true, laxProp: 'yoyo' });

          expect(onSuccessStats).toEqual({
            dependentProp_1: 1,
            laxProp: 1,
            virtual_1: 1
          });

          expect(onSuccessValues).toEqual({
            dependentProp_1: true,
            laxProp: 'yoyo',
            virtual_1: true
          });
        });

        it("should not update readonly properties that have changed even when 'shouldUpdate' resolved to 'true'", async () => {
          const { data, error } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: 'changed',
              laxProp_1: 'test'
            },
            { laxProp: 'yoyo' }
          );

          expect(data).toBe(null);
          expect(error.message).toBe(ERRORS.NOTHING_TO_UPDATE);
        });

        describe('behaviour when shouldUpdate method returns nothing', () => {
          const Model = new Schema({
            isBlocked: { default: false, shouldUpdate: () => {} },
            laxProp: { default: 0 }
          }).getModel();

          it('should assume updatability of a property as falsy if shouldInit method returns nothing', async () => {
            const { data, error } = await Model.update(
              { isBlocked: false, laxProp: 0 },
              { isBlocked: true }
            );

            expect(data).toBe(null);
            expect(error).toMatchObject({
              message: ERRORS.NOTHING_TO_UPDATE,
              payload: {}
            });
          });
        });
      });
    });

    describe('invalid', () => {
      it('should reject shouldUpdate !(false | () => boolean)', () => {
        const invalidValues = [
          true,
          1,
          0,
          -1,
          'true',
          'false',
          [],
          null,
          undefined,
          {}
        ];

        for (const shouldUpdate of invalidValues) {
          const toFail = fx({ propertyName: { default: '', shouldUpdate } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "'shouldUpdate' only accepts false or a function that returns a boolean"
                ])
              })
            );
          }
        }
      });

      it('should reject shouldUpdate & readonly(true) & no shouldInit', () => {
        const toFail = fx({
          propertyName: {
            default: '',
            readonly: true,
            shouldUpdate: () => {}
          }
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Cannot block the update of 'readonly' properties that do not have initialization('shouldInit') blocked. Either add 'shouldInit' or use readonly: 'lax'"
              ])
            })
          );
        }
      });
    });
  });
};
