import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  it,
  mock,
} from 'bun:test';

import type { IvoContext, ReadonlyIvoContext } from '../../../src';
import { expectFailure, expectNoFailure, validator } from '../_utils';

export const Test_ShouldInitAndUpdateRules = ({ Schema, fx }: any) => {
  describe('ignore', () => {
    describe('valid', () => {
      it('should accept ignore + default', () => {
        const fxn = fx({
          propertyName: { ignore: () => false, default: true },
        });

        expectNoFailure(fxn);

        fxn();
      });

      it('should accept ignore + virtual', () => {
        const fxn = fx({
          dependent: {
            default: true,
            dependsOn: 'propertyName',
            resolver: validator,
          },
          propertyName: { ignore: () => false, virtual: true, validator },
        });

        expectNoFailure(fxn);

        fxn();
      });

      describe('behaviour', () => {
        it('should ignore accordingly', async () => {
          const Model = new Schema({
            isBlocked: {
              default: false,
              ignore: ({
                input: { env },
              }: IvoContext<
                { env: string; isBlocked?: boolean },
                { env: string; isBlocked: boolean; laxProp: number }
              >) => env === 'dev',
            },
            env: { default: 'dev' },
            laxProp: { default: 0 },
          }).getModel();

          const { data } = await Model.create({ env: 'dev', isBlocked: true });

          expect(data).toMatchObject({
            env: 'dev',
            isBlocked: false,
            laxProp: 0,
          });

          {
            const { data } = await Model.create({
              env: 'Lol',
              isBlocked: true,
            });

            expect(data).toMatchObject({
              env: 'Lol',
              isBlocked: true,
              laxProp: 0,
            });
          }

          {
            const { data } = await Model.update(
              {
                env: 'Lol',
                isBlocked: true,
                laxProp: 0,
              },
              { env: 'dev', isBlocked: 'updated' },
            );
            expect(data).toEqual({ env: 'dev' });
          }

          {
            const { data } = await Model.update(
              {
                env: 'dev',
                isBlocked: true,
                laxProp: 0,
              },
              { env: 'Lol', isBlocked: 'updated' },
            );

            expect(data).toEqual({ env: 'Lol', isBlocked: 'updated' });
          }
        });

        it('should not trigger validators of ignored properties', async () => {
          const validator = () => true;

          const mockedValidator = mock(validator);

          const Model = new Schema({
            isBlocked: {
              default: false,
              ignore: ({
                input: { env },
              }: IvoContext<
                { env: string; isBlocked?: boolean },
                { env: string; isBlocked: boolean; laxProp: number }
              >) => env === 'dev',
              validator: mockedValidator,
            },
            env: { default: 'dev' },
            laxProp: { default: 0 },
          }).getModel();

          const { data } = await Model.create({ env: 'dev', isBlocked: true });

          expect(mockedValidator).toBeCalledTimes(0);

          expect(data).toMatchObject({
            env: 'dev',
            isBlocked: false,
            laxProp: 0,
          });

          {
            const { data } = await Model.create({
              env: 'Lol',
              isBlocked: true,
            });

            expect(mockedValidator).toBeCalledTimes(1);

            expect(data).toMatchObject({
              env: 'Lol',
              isBlocked: true,
              laxProp: 0,
            });
          }

          {
            const { data } = await Model.update(
              {
                env: 'Lol',
                isBlocked: true,
                laxProp: 0,
              },
              { env: 'dev', isBlocked: 'updated' },
            );
            expect(mockedValidator).toBeCalledTimes(1);
            expect(data).toEqual({ env: 'dev' });
          }

          {
            const { data } = await Model.update(
              {
                env: 'dev',
                isBlocked: true,
                laxProp: 0,
              },
              { env: 'Lol', isBlocked: 'updated' },
            );

            expect(mockedValidator).toBeCalledTimes(2);
            expect(data).toEqual({ env: 'Lol', isBlocked: 'updated' });
          }
        });

        it('should properly handle ignored properties even when not provided', async () => {
          const validator = () => true;

          const mockedValidator = mock(validator);

          const Model = new Schema({
            isBlocked: {
              default: false,
              ignore: ({
                input: { env },
              }: IvoContext<
                { env: string; isBlocked?: boolean },
                { env: string; isBlocked: boolean; laxProp: number }
              >) => env === 'dev',
              validator: mockedValidator,
            },
            env: { default: 'dev' },
            laxProp: { default: 0 },
          }).getModel();

          const { data } = await Model.create({ env: 'dev' });

          expect(mockedValidator).toBeCalledTimes(0);

          expect(data).toMatchObject({
            env: 'dev',
            isBlocked: false,
            laxProp: 0,
          });

          {
            const { data } = await Model.create({
              env: 'Lol',
              isBlocked: true,
            });

            expect(mockedValidator).toBeCalledTimes(1);

            expect(data).toMatchObject({
              env: 'Lol',
              isBlocked: true,
              laxProp: 0,
            });
          }

          {
            const { data } = await Model.update(
              {
                env: 'Lol',
                isBlocked: true,
                laxProp: 0,
              },
              { env: 'dev', isBlocked: 'updated' },
            );
            expect(mockedValidator).toBeCalledTimes(1);
            expect(data).toEqual({ env: 'dev' });
          }

          {
            const { data } = await Model.update(
              {
                env: 'dev',
                isBlocked: true,
                laxProp: 0,
              },
              { env: 'Lol', isBlocked: 'updated' },
            );

            expect(mockedValidator).toBeCalledTimes(2);
            expect(data).toEqual({ env: 'Lol', isBlocked: 'updated' });
          }
        });
      });
    });

    describe('invalid', () => {
      it('should reject ignore & no default', () => {
        const fxn = fx({ propertyName: { ignore: () => false } });

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'For a property to be ignored, it must have a default value or be virtual',
              ]),
            }),
          );
        }
      });

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
          const fxn = fx({ propertyName: { ignore, default: true } });

          expectFailure(fxn);

          try {
            fxn();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  '"ignore" must be a function that returns a boolean',
                ]),
              }),
            );
          }
        }
      });

      it('should reject ignore + (ignoreInit | ignoreUpdate)', () => {
        const values = [
          { ignoreInit: true },
          { ignoreInit: false },
          { ignoreUpdate: false },
          { ignoreUpdate: true },
          { ignoreInit: false, ignoreUpdate: () => true },
          { ignoreInit: true, ignoreUpdate: () => true },
          { ignoreInit: () => true, ignoreUpdate: true },
          { ignoreInit: () => true, ignoreUpdate: false },
          { ignoreInit: () => true, ignoreUpdate: () => true },
        ];

        for (const config of values) {
          const fxn = fx({
            propertyName: { ignore: () => true, default: true, ...config },
          });

          expectFailure(fxn);

          try {
            fxn();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  '"ignore" cannot be used with "ignoreInit" or "ignoreUpdate"',
                ]),
              }),
            );
          }
        }
      });
    });
  });

  describe('ignoreInit', () => {
    describe('valid', () => {
      it('should accept ignoreInit(false) + default', () => {
        const fxn = fx({
          propertyName: { ignoreInit: false, default: true },
        });

        expectNoFailure(fxn);

        fxn();
      });

      it('should accept ignoreInit: () => boolean + default', () => {
        const values = [() => true, () => false];

        for (const ignoreInit of values) {
          const fxn = fx({
            propertyName: { ignoreInit, default: true },
          });

          expectNoFailure(fxn);

          fxn();
        }
      });

      describe('behaviour', () => {
        const Model = new Schema({
          isBlocked: {
            default: false,
            ignoreInit: ({
              input,
            }: IvoContext<{
              isBlocked: boolean;
              env: string;
              laxProp: number;
            }>) => input?.env === 'test',
          },
          env: { default: 'dev' },
          laxProp: { default: 0 },
        }).getModel();

        it('should respect default rules', async () => {
          const { data } = await Model.create({ isBlocked: true });

          expect(data).toMatchObject({
            env: 'dev',
            isBlocked: true,
            laxProp: 0,
          });
        });

        it('should respect callable should init when condition passes at creation', async () => {
          const { data } = await Model.create({
            env: 'test',
            isBlocked: true,
          });

          expect(data).toEqual({
            env: 'test',
            laxProp: 0,
          });
        });

        describe('behaviour when ignoreInit method returns nothing', () => {
          const Model = new Schema({
            isBlocked: { default: false, ignoreInit: () => {} },
            laxProp: { default: 0 },
          }).getModel();

          it('should assume initialization as falsy if ignoreInit method returns nothing at creation', async () => {
            const { data } = await Model.create({ isBlocked: 'yes' });

            expect(data).toMatchObject({ isBlocked: 'yes', laxProp: 0 });
          });
        });
      });

      describe('behaviour of callable ignoreInit', () => {
        const onSuccessValues: Record<string, unknown> = {};
        const onSuccessStats: Record<string, number> = {};
        const sanitizedValues: Record<string, unknown> = {};

        let Model: any;

        beforeAll(() => {
          Model = new Schema({
            dependent: {
              default: '',
              dependsOn: 'virtual',
              resolver: () => 'changed',
              onSuccess: onSuccess('dependent'),
            },
            laxProp: { default: '' },
            virtual: {
              virtual: true,
              ignoreInit: ({ input }: IvoContext<{ laxProp: string }>) =>
                input?.laxProp === 'allow virtual',
              onSuccess: [
                onSuccess('virtual'),
                incrementOnSuccessStats('virtual'),
                incrementOnSuccessStats('virtual'),
              ],
              sanitizer: sanitizerOf('virtual', 'sanitized'),
              validator: validateBoolean,
            },
          }).getModel();

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

          type IgnoreInitInput = { virtual?: boolean; laxProp?: string };
          type IgnoreInitOutput = { dependent: string; laxProp: string };

          function onSuccess(prop: string) {
            return ({
              input,
              values,
            }: ReadonlyIvoContext<IgnoreInitInput, IgnoreInitOutput>) => {
              onSuccessValues[prop] =
                (values as Record<string, unknown>)?.[prop] ??
                (input as Record<string, unknown>)?.[prop];
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
          for (const key of Object.keys(onSuccessStats))
            delete onSuccessStats[key];
          for (const key of Object.keys(onSuccessValues))
            delete onSuccessValues[key];
          for (const key of Object.keys(sanitizedValues))
            delete sanitizedValues[key];
        });

        it("should respect virtuals at creation when their ignoreInit handler returns 'false'", async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: 'Peter',
            virtual: true,
          });

          await handleSuccess();

          expect(data).toEqual({ dependent: 'changed', laxProp: 'Peter' });

          expect(onSuccessStats).toEqual({
            dependent: 1,
            virtual: 3,
          });

          expect(onSuccessValues).toEqual({
            dependent: 'changed',
            virtual: 'sanitized',
          });

          expect(sanitizedValues).toEqual({ virtual: 'sanitized' });
        });

        it("should ignore virtuals at creation when their ignoreInit handler returns 'true'", async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: 'allow virtual',
            virtual: true,
          });

          await handleSuccess();

          expect(data).toEqual({
            dependent: '',
            laxProp: 'allow virtual',
          });

          expect(onSuccessStats).toEqual({ dependent: 1 });

          expect(onSuccessValues).toEqual({ dependent: '' });

          expect(sanitizedValues).toEqual({});
        });
      });
    });

    describe('invalid', () => {
      it('should reject ignoreInit(false) & no default', () => {
        const fxn = fx({ propertyName: { ignoreInit: false } });

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'A property with initialization blocked must have a default value',
              ]),
            }),
          );
        }
      });

      it('should reject ignoreInit !(boolean | () => boolean)', () => {
        const values = [undefined, 1, {}, null, [], 'yes', 'false', 'true'];

        for (const ignoreInit of values) {
          const fxn = fx({ propertyName: { ignoreInit, default: true } });

          expectFailure(fxn);

          try {
            fxn();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "The initialization of a property can only be blocked if the 'ignoreinit' rule is set to 'false' or a function that returns a boolean",
                ]),
              }),
            );
          }
        }
      });
    });
  });

  describe('ignoreUpdate', () => {
    describe('valid', () => {
      it('should accept ignoreUpdate(() => boolean)', () => {
        const validValues = [() => false, () => true];

        for (const ignoreUpdate of validValues) {
          const toPass = fx({ propertyName: { default: '', ignoreUpdate } });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should accept ignoreInit(() => boolean) & ignoreUpdate(false) for virtuals', () => {
        const values = [() => true, () => false];

        for (const ignoreInit of values) {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependsOn: 'virtual',
              resolver: () => '',
            },
            virtual: {
              virtual: true,
              ignoreInit,
              ignoreUpdate: false,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      describe('behaviour', () => {
        let onSuccessValues: Record<string, unknown> = {};
        let onSuccessStats: Record<string, number> = {};

        type IgnoreUpdateOutput = {
          dependentProp: boolean;
          dependentProp_1: boolean;
          laxProp: string;
          laxProp_1: string;
        };

        function incrementOnSuccessCountOf(prop: string) {
          return ({
            values,
          }: ReadonlyIvoContext<
            {
              virtual: boolean;
              virtual_1: boolean;
              laxProp: string;
              laxProp_1: string;
            },
            IgnoreUpdateOutput
          >) => {
            const previousCount = onSuccessStats[prop] ?? 0;

            onSuccessStats[prop] = previousCount + 1;
            onSuccessValues[prop] = (values as Record<string, unknown>)[prop];
          };
        }

        const Model = new Schema({
          dependentProp: {
            default: false,
            dependsOn: 'virtual',
            resolver: ({
              input,
            }: IvoContext<{
              virtual: boolean;
              virtual_1: boolean;
              laxProp: string;
              laxProp_1: string;
            }>) => input.virtual,
            onSuccess: incrementOnSuccessCountOf('dependentProp'),
          },
          dependentProp_1: {
            default: false,
            dependsOn: 'virtual_1',
            resolver: ({
              input,
            }: IvoContext<{
              virtual: boolean;
              virtual_1: boolean;
              laxProp: string;
              laxProp_1: string;
            }>) => input.virtual_1,
            onSuccess: incrementOnSuccessCountOf('dependentProp_1'),
          },
          laxProp: {
            default: '',
            readonly: 'lax',
            ignoreUpdate: ({
              input,
            }: IvoContext<{
              virtual: boolean;
              virtual_1: boolean;
              laxProp: string;
              laxProp_1: string;
            }>) => input?.laxProp_1 === 'test',
            onSuccess: incrementOnSuccessCountOf('laxProp'),
          },
          laxProp_1: { default: 'dev' },
          virtual: {
            virtual: true,
            ignoreUpdate: false,
            validator: () => ({ valid: true }),
            onSuccess: incrementOnSuccessCountOf('virtual'),
          },
          virtual_1: {
            virtual: true,
            ignoreUpdate: ({
              input,
            }: IvoContext<{
              virtual: boolean;
              virtual_1: boolean;
              laxProp: string;
              laxProp_1: string;
            }>) => input?.laxProp_1 === 'test',
            validator: () => ({ valid: true }),
            onSuccess: incrementOnSuccessCountOf('virtual_1'),
          },
        }).getModel();

        afterEach(() => {
          onSuccessValues = {};
          onSuccessStats = {};
        });

        it("should not update properties when 'ignoreUpdate' resolved to 'false'", async () => {
          const { data, error } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: '',
              laxProp_1: '',
            },
            { laxProp: 'yoyo', virtual: true, virtual_1: true },
          );

          expect(data).toBeNull();
          expect(error).toBeNull();
        });

        it("should update properties when 'ignoreUpdate' resolved to 'true'", async () => {
          const { data, error, handleSuccess } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: '',
              laxProp_1: 'test',
            },
            { laxProp: 'yoyo', virtual: true, virtual_1: true },
          );

          await handleSuccess();

          expect(error).toBeNull();
          expect(data).toEqual({ dependentProp_1: true, laxProp: 'yoyo' });

          expect(onSuccessStats).toEqual({
            dependentProp_1: 1,
            laxProp: 1,
            virtual_1: 1,
          });

          expect(onSuccessValues).toEqual({
            dependentProp_1: true,
            laxProp: 'yoyo',
            virtual_1: true,
          });
        });

        it("should not update readonly properties that have changed even when 'ignoreUpdate' resolved to 'true'", async () => {
          const { data, error } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: 'changed',
              laxProp_1: 'test',
            },
            { laxProp: 'yoyo' },
          );

          expect(data).toBeNull();
          expect(error).toBeNull();
        });

        describe('behaviour when ignoreUpdate method returns nothing', () => {
          const Model = new Schema({
            isBlocked: { default: false, ignoreUpdate: () => {} },
            laxProp: { default: 0 },
          }).getModel();

          it('should assume updatability of a property as falsy if ignoreInit method returns nothing', async () => {
            const { data, error } = await Model.update(
              { isBlocked: false, laxProp: 0 },
              { isBlocked: true },
            );

            expect(data).toBeNull();
            expect(error).toBeNull();
          });
        });
      });
    });

    describe('invalid', () => {
      it('should reject ignoreUpdate !(false | () => boolean)', () => {
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
          {},
        ];

        for (const ignoreUpdate of invalidValues) {
          const toFail = fx({ propertyName: { default: '', ignoreUpdate } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "'ignoreUpdate' only accepts false or a function that returns a boolean",
                ]),
              }),
            );
          }
        }
      });

      it('should reject ignoreUpdate & readonly(true) & no ignoreInit', () => {
        const toFail = fx({
          propertyName: {
            default: '',
            readonly: true,
            ignoreUpdate: () => {},
          },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Cannot block the update of 'readonly' properties that do not have initialization('ignoreInit') blocked. Either add 'ignoreInit' or use readonly: 'lax'",
              ]),
            }),
          );
        }
      });
    });
  });

  describe('ignoreInit & ignoreUpdate', () => {
    describe('valid', () => {
      it('should accept ignoreInit & ignoreUpdate for lax props', () => {
        // [ignoreInit, ignoreUpdate]
        const values = [
          [false, () => {}],
          [() => {}, false],
          [() => {}, () => {}],
        ];

        for (const [ignoreInit, ignoreUpdate] of values) {
          const toPass = fx({
            propertyName: { default: '', ignoreInit, ignoreUpdate },
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should accept ignoreInit(() => boolean) + ignoreUpdate(false | () => boolean) + readonly(true)', () => {
        // [ignoreInit, ignoreUpdate]
        const readonlyTrue = [
          [false, () => {}],
          [() => {}, false],
          [() => {}, () => {}],
        ];

        for (const [ignoreInit, ignoreUpdate] of readonlyTrue) {
          const toPass = fx({
            dependentProp: {
              default: '',
              readonly: true,
              ignoreInit,
              ignoreUpdate,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        }

        const toPass = fx({
          dependentProp: {
            default: '',
            readonly: 'lax',
            ignoreUpdate: () => {},
            validator,
          },
        });

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe('invalid', () => {
      it('should reject ignoreUpdate == false & ignoreInit == false', () => {
        const toFail = fx({
          propertyName: {
            default: '',
            ignoreInit: false,
            ignoreUpdate: false,
          },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Both 'ignoreInit' & 'ignoreUpdate' cannot be 'false'",
              ]),
            }),
          );
        }
      });

      describe('Readonly lax', () => {
        it("should reject readonly('lax') + ignoreInit", () => {
          for (const ignoreInit of [false, () => {}]) {
            const toFail = fx({
              propertyName: {
                default: '',
                readonly: 'lax',
                ignoreInit,
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
                    'Lax properties cannot have initialization blocked',
                  ]),
                }),
              );
            }
          }
        });

        it("should reject readonly('lax') + ignoreUpdate(false)", () => {
          const toFail = fx({
            propertyName: {
              default: '',
              readonly: 'lax',
              ignoreUpdate: false,
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
                  'Readonly(lax) properties cannot have updates strictly blocked',
                ]),
              }),
            );
          }
        });
      });
    });
  });
};
