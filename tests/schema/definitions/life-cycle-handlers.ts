import { beforeEach, describe, it, expect, test } from 'bun:test';

import { expectFailure, expectNoFailure, validator } from '../_utils';

export const Test_LifeCycleHandlers = ({ Schema, fx }: any) => {
  describe('life cycle handlers', () => {
    const rules = ['onDelete', 'onFailure', 'onSuccess'];

    describe('valid', () => {
      test('valid', () => {
        const values = [
          () => {},
          () => ({}),
          [() => {}],
          [() => {}, () => ({})],
        ];

        for (const rule of rules)
          for (const value of values) {
            const toPass = fx({
              propertyName: {
                default: '',
                [rule]: value,
                validator,
              },
            });

            expectNoFailure(toPass);

            toPass();
          }
      });
    });

    describe('invalid', () => {
      test('invalid', () => {
        const values = [1, '', 0, false, true, null, {}];

        for (const rule of rules)
          for (const value of values) {
            const toFail = fx({
              propertyName: {
                default: '',
                [rule]: value,
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
                    `The '${rule}' handler at index: 0 is not a function`,
                  ]),
                }),
              );
            }
          }
      });
    });

    describe('life cycle readonly ctx', () => {
      const rules = ['onDelete', 'onFailure', 'onSuccess'];

      let propChangeMap: any = {},
        ctxHasUpdateMethod: any = {};

      const validData = { constant: 1, prop1: '1', prop2: '2', prop3: '3' };
      const allProps = ['constant', 'prop1', 'prop2', 'prop3'],
        props = ['prop1', 'prop2', 'prop3'];

      const handle =
        (rule = '', prop = '') =>
        ({ context }: any) => {
          ctxHasUpdateMethod[rule] = !!context?.__updateOptions__;

          try {
            context[prop] = 1;
          } catch (err) {
            if (!propChangeMap[rule]) propChangeMap[rule] = {};

            propChangeMap[rule][prop] = true;
          }
        };
      const validator = (value: any) => ({ valid: !!value });

      const Model = new Schema({
        constant: {
          constant: true,
          value: 'constant',
          onDelete: handle('onDelete', 'constant'),
          onSuccess: handle('onSuccess', 'constant'),
        },
        prop1: {
          required: true,
          onDelete: handle('onDelete', 'prop1'),
          onFailure: handle('onFailure', 'prop1'),
          onSuccess: handle('onSuccess', 'prop1'),
          validator,
        },
        prop2: {
          required: true,
          onDelete: handle('onDelete', 'prop2'),
          onFailure: handle('onFailure', 'prop2'),
          onSuccess: handle('onSuccess', 'prop2'),
          validator,
        },
        prop3: {
          required: true,
          onDelete: handle('onDelete', 'prop3'),
          onFailure: handle('onFailure', 'prop3'),
          onSuccess: handle('onSuccess', 'prop3'),
          validator,
        },
      }).getModel();

      beforeEach(() => {
        propChangeMap = {};
        ctxHasUpdateMethod = {};
      });

      it('should reject handlers that try to mutate the onSuccess ctx', async () => {
        const { handleFailure, handleSuccess } = await Model.create(validData);

        expect(handleFailure).toBeNull();

        await handleSuccess();

        expect(propChangeMap.onSuccess.constant).toBe(true);
        expect(ctxHasUpdateMethod).toEqual({ onSuccess: false });
      });

      it('should reject handlers that try to mutate the onDelete ctx', async () => {
        await Model.delete(validData);

        for (const prop of allProps)
          expect(propChangeMap.onDelete[prop]).toBe(true);

        expect(ctxHasUpdateMethod).toEqual({ onDelete: false });
      });

      it('should reject handlers that try to mutate the onFailure(create) ctx', async () => {
        const { handleFailure } = await Model.create({
          prop1: '',
          prop2: '',
          prop3: '',
        });

        await handleFailure();

        for (const prop of props)
          for (const rule of rules) {
            const result = rule == 'onFailure' ? true : undefined;

            expect(propChangeMap?.[rule]?.[prop]).toBe(result);
          }

        expect(ctxHasUpdateMethod).toEqual({ onFailure: false });
      });

      it('should reject handlers that try to mutate the onFailure(update) ctx', async () => {
        const { handleFailure } = await Model.update(validData, {
          prop1: '',
          prop2: '',
          prop3: '',
        });

        await handleFailure();

        for (const prop of props)
          for (const rule of rules) {
            const result = rule == 'onFailure' ? true : undefined;

            expect(propChangeMap?.[rule]?.[prop]).toBe(result);
          }

        expect(ctxHasUpdateMethod).toEqual({ onFailure: false });
      });
    });

    describe('onDelete', () => {
      const contextOptions = { lang: 'en' };

      let cxtOptions: any = {},
        propChangeMap: any = {};

      const onDelete = (prop = '') => {
        return ({ __getOptions__ }) => {
          cxtOptions[prop] = __getOptions__();
          propChangeMap[prop] = true;
        };
      };
      const validator = () => ({ valid: false });

      const Model = new Schema({
        constant: {
          constant: true,
          value: 'constant',
          onDelete: onDelete('constant'),
        },
        prop1: { required: true, onDelete: onDelete('prop1'), validator },
        prop2: { required: true, onDelete: onDelete('prop2'), validator },
        prop3: { required: true, onDelete: onDelete('prop3'), validator },
      }).getModel();

      beforeEach(() => {
        cxtOptions = {};
        propChangeMap = {};
      });

      it('should trigger all onDelete handlers but for virtuals', async () => {
        await Model.delete(
          {
            constant: true,
            prop1: true,
            prop2: true,
            prop3: true,
            prop4: true,
          },
          contextOptions,
        );

        expect(cxtOptions).toEqual({
          constant: contextOptions,
          prop1: contextOptions,
          prop2: contextOptions,
          prop3: contextOptions,
        });
        expect(propChangeMap).toEqual({
          constant: true,
          prop1: true,
          prop2: true,
          prop3: true,
        });
      });
    });

    describe('onFailure', () => {
      it('should reject onFailure & no validator', () => {
        const toFail = fx({ prop: { default: '', onFailure: () => {} } });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              prop: expect.arrayContaining([
                "'onFailure' can only be used with properties that support and have validators",
              ]),
            }),
          );
        }
      });

      describe('behaviour', () => {
        const contextOptions = { lang: 'en' };

        let cxtOptions: any = {},
          onFailureCount: any = {};

        function incrementOnFailureCountOf(prop: string) {
          return ({ __getOptions__ }) => {
            cxtOptions[prop] = __getOptions__();
            onFailureCount[prop] = (onFailureCount[prop] ?? 0) + 1;
          };
        }
        const validator = () => ({ valid: false });

        const Model = new Schema({
          prop1: {
            default: true,
            onFailure: incrementOnFailureCountOf('prop1'),
            validator,
          },
          prop2: {
            required: true,
            onFailure: [
              incrementOnFailureCountOf('prop2'),
              incrementOnFailureCountOf('prop2'),
            ],
            validator,
          },
          dependentProp: {
            default: '',
            dependsOn: 'virtualProp',
            resolver: () => '',
          },
          virtualProp: {
            virtual: true,
            onFailure: [
              incrementOnFailureCountOf('virtualProp'),
              incrementOnFailureCountOf('virtualProp'),
              incrementOnFailureCountOf('virtualProp'),
            ],
            validator,
          },
        }).getModel();

        beforeEach(() => {
          cxtOptions = {};
          onFailureCount = {};
        });

        describe('creation', () => {
          it('should properly trigger onFailure handlers at creation', async () => {
            const { error, handleFailure } = await Model.create(
              { prop1: false },
              contextOptions,
            );

            await handleFailure();

            expect(error).toBeDefined();
            expect(cxtOptions).toEqual({
              prop1: contextOptions,
              prop2: contextOptions,
            });
            expect(onFailureCount).toEqual({ prop1: 1, prop2: 2 });
          });

          it('should properly trigger onFailure handlers at creation with virtuals', async () => {
            const { error, handleFailure } = await Model.create(
              {
                prop1: false,
                virtualProp: 'Yes',
              },
              contextOptions,
            );

            await handleFailure();

            expect(error).toBeDefined();
            expect(error).toBeDefined();
            expect(cxtOptions).toEqual({
              prop1: contextOptions,
              prop2: contextOptions,
              virtualProp: contextOptions,
            });
            expect(onFailureCount).toEqual({
              prop1: 1,
              prop2: 2,
              virtualProp: 3,
            });
          });
        });

        describe('updates', () => {
          it('should properly trigger onFailure handlers during updates', async () => {
            const { error, handleFailure } = await Model.update(
              {},
              { prop1: '' },
              contextOptions,
            );

            await handleFailure();

            expect(error).toBeDefined();
            expect(cxtOptions).toEqual({ prop1: contextOptions });
            expect(onFailureCount).toEqual({ prop1: 1 });
          });

          it('should properly trigger onFailure handlers during updates with virtuals', async () => {
            const data = [
              [
                { virtualProp: '' },
                { virtualProp: 3 },
                { virtualProp: contextOptions },
              ],
              [
                { prop1: '', virtualProp: '' },
                { prop1: 1, virtualProp: 3 },
                { prop1: contextOptions, virtualProp: contextOptions },
              ],
            ];

            for (const [changes, results, ctxOpts] of data) {
              onFailureCount = {};

              const { error, handleFailure } = await Model.update(
                {},
                changes,
                contextOptions,
              );

              await handleFailure();

              expect(error).toBeDefined();
              expect(cxtOptions).toEqual(ctxOpts);
              expect(onFailureCount).toEqual(results);
            }
          });

          it('should properly trigger onFailure handlers during updates & nothing to update', async () => {
            const { error, handleFailure } = await Model.update(
              { prop1: 2 },
              { prop1: 35 },
              contextOptions,
            );

            await handleFailure();

            expect(error).toBeDefined();
            expect(cxtOptions).toEqual({ prop1: contextOptions });
            expect(onFailureCount).toEqual({ prop1: 1 });
          });
        });
      });
    });

    describe('onSuccess', () => {
      const contextOptions = { lang: 'en' };

      let cxtOptions: any = {},
        initialData = {
          dependent: false,
          lax: 'changed',
          readonly: 'changed',
          readonlyLax: '',
          required: 'changed',
        },
        onSuccessValues: any = {},
        propChangeMap: any = {};

      const onSuccess =
        (prop = '') =>
        (summary: any) => {
          cxtOptions[prop] = summary.context.__getOptions__();
          onSuccessValues[prop] = summary;
          onSuccessValues.__ctx = summary.context;
          propChangeMap[prop] = true;
        };

      const validator = () => ({ valid: true });

      const Model = new Schema({
        dependent: {
          default: false,
          dependsOn: 'readonlyLax',
          onSuccess: onSuccess('dependent'),
          resolver: () => true,
        },
        lax: { default: '', onSuccess: onSuccess('lax'), validator },
        readonly: {
          readonly: true,
          onSuccess: onSuccess('readonly'),
          validator,
        },
        readonlyLax: {
          default: '',
          readonly: 'lax',
          onSuccess: onSuccess('readonlyLax'),
          validator,
        },
        required: {
          required: true,
          onSuccess: onSuccess('required'),
          validator,
        },
      }).getModel();

      beforeEach(() => {
        cxtOptions = {};
        onSuccessValues = {};
        propChangeMap = {};
      });

      // creation
      it('should call onSuccess handlers at creation', async () => {
        const { data, error, handleSuccess } = await Model.create(
          {
            required: true,
            readonly: true,
          },
          contextOptions,
        );

        await handleSuccess();

        expect(error).toBeNull();

        expect(cxtOptions).toEqual({
          dependent: contextOptions,
          lax: contextOptions,
          readonly: contextOptions,
          readonlyLax: contextOptions,
          required: contextOptions,
        });

        expect(propChangeMap).toEqual({
          dependent: true,
          lax: true,
          readonly: true,
          readonlyLax: true,
          required: true,
        });

        const changes = null,
          context = onSuccessValues.__ctx,
          isUpdate = false,
          previousValues = null,
          values = data,
          summary = { changes, context, isUpdate, previousValues, values };

        expect(onSuccessValues).toMatchObject({
          dependent: summary,
          lax: summary,
          readonly: summary,
          readonlyLax: summary,
          required: summary,
        });
      });

      // updates
      it('should call onSuccess handlers during updates with lax props', async () => {
        const { data, error, handleSuccess } = await Model.update(
          initialData,
          {
            lax: true,
          },
          contextOptions,
        );

        await handleSuccess();

        expect(error).toBeNull();
        expect(cxtOptions).toEqual({
          lax: contextOptions,
        });
        expect(propChangeMap).toEqual({ lax: true });

        expect(onSuccessValues).toMatchObject({
          lax: expect.objectContaining({
            changes: data,
            context: onSuccessValues.__ctx,
            isUpdate: true,
            previousValues: initialData,
            values: { ...initialData, lax: true },
          }),
        });
      });

      it('should call onSuccess handlers during updates with readonlyLax & dependent', async () => {
        const { data, error, handleSuccess } = await Model.update(
          initialData,
          {
            readonlyLax: true,
          },
          contextOptions,
        );

        await handleSuccess();

        expect(error).toBeNull();
        expect(cxtOptions).toEqual({
          dependent: contextOptions,
          readonlyLax: contextOptions,
        });
        expect(propChangeMap).toEqual({ dependent: true, readonlyLax: true });

        const changes = data,
          context = onSuccessValues.__ctx,
          isUpdate = true,
          previousValues = initialData,
          values = { ...initialData, ...data },
          summary = { changes, context, isUpdate, previousValues, values };

        expect(onSuccessValues).toMatchObject({
          dependent: summary,
          readonlyLax: summary,
        });
      });
    });
  });
};
