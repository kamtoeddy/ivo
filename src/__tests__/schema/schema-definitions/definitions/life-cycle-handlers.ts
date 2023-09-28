import { beforeEach, describe, it, expect, test } from 'vitest'

import { expectFailure, expectNoFailure, validator } from '../_utils'

export const Test_LifeCycleHandlers = ({ Schema, fx }: any) => {
  describe('life cycle handlers', () => {
    const rules = ['onDelete', 'onFailure', 'onSuccess']

    describe('valid', () => {
      test('valid', () => {
        const values = [
          () => {},
          () => ({}),
          [() => {}],
          [() => {}, () => ({})]
        ]

        for (const rule of rules)
          for (const value of values) {
            const toPass = fx({
              propertyName: {
                default: '',
                [rule]: value,
                validator
              }
            })

            expectNoFailure(toPass)

            toPass()
          }
      })
    })

    describe('invalid', () => {
      test('invalid', () => {
        const values = [1, '', 0, false, true, null, {}]

        for (const rule of rules)
          for (const value of values) {
            const toFail = fx({
              propertyName: {
                default: '',
                [rule]: value,
                validator
              }
            })

            expectFailure(toFail)

            try {
              toFail()
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    `The '${rule}' handler @[0] is not a function`
                  ])
                })
              )
            }
          }
      })
    })

    describe('life cycle readonly ctx', () => {
      const rules = ['onDelete', 'onFailure', 'onSuccess']

      let propChangeMap: any = {}

      const validData = { constant: 1, prop1: '1', prop2: '2', prop3: '3' }
      const allProps = ['constant', 'prop1', 'prop2', 'prop3'],
        props = ['prop1', 'prop2', 'prop3']

      const handle =
        (rule = '', prop = '') =>
        ({ context }: any) => {
          try {
            context[prop] = 1
          } catch (err) {
            if (!propChangeMap[rule]) propChangeMap[rule] = {}

            propChangeMap[rule][prop] = true
          }
        }
      const validator = (value: any) => ({ valid: !!value })

      const Model = new Schema({
        constant: {
          constant: true,
          value: 'constant',
          onDelete: handle('onDelete', 'constant'),
          onSuccess: handle('onSuccess', 'constant')
        },
        prop1: {
          required: true,
          onDelete: handle('onDelete', 'prop1'),
          onFailure: handle('onFailure', 'prop1'),
          onSuccess: handle('onSuccess', 'prop1'),
          validator
        },
        prop2: {
          required: true,
          onDelete: handle('onDelete', 'prop2'),
          onFailure: handle('onFailure', 'prop2'),
          onSuccess: handle('onSuccess', 'prop2'),
          validator
        },
        prop3: {
          required: true,
          onDelete: handle('onDelete', 'prop3'),
          onFailure: handle('onFailure', 'prop3'),
          onSuccess: handle('onSuccess', 'prop3'),
          validator
        }
      }).getModel()

      beforeEach(() => {
        propChangeMap = {}
      })

      it('should reject handlers that try to mutate the onChange, onCreate & onSuccess ctx', async () => {
        const { handleSuccess } = await Model.create(validData)

        await handleSuccess?.()

        expect(propChangeMap.onSuccess.constant).toBe(true)
      })

      it('should reject handlers that try to mutate the onDelete ctx', async () => {
        await Model.delete(validData)

        for (const prop of allProps)
          expect(propChangeMap.onDelete[prop]).toBe(true)
      })

      it('should reject handlers that try to mutate the onFailure(clone) ctx', async () => {
        await Model.clone({ prop1: '', prop2: '', prop3: '' })

        for (const prop of props)
          for (const rule of rules) {
            const result = rule == 'onFailure' ? true : undefined

            expect(propChangeMap?.[rule]?.[prop]).toBe(result)
          }
      })

      it('should reject handlers that try to mutate the onFailure(create) ctx', async () => {
        await Model.create({ prop1: '', prop2: '', prop3: '' })

        for (const prop of props)
          for (const rule of rules) {
            const result = rule == 'onFailure' ? true : undefined

            expect(propChangeMap?.[rule]?.[prop]).toBe(result)
          }
      })

      it('should reject handlers that try to mutate the onFailure(update) ctx', async () => {
        await Model.update(validData, { prop1: '', prop2: '', prop3: '' })

        for (const prop of props)
          for (const rule of rules) {
            const result = rule == 'onFailure' ? true : undefined

            expect(propChangeMap?.[rule]?.[prop]).toBe(result)
          }
      })
    })

    describe('onDelete', () => {
      let propChangeMap: any = {}

      const onDelete =
        (prop = '') =>
        () =>
          (propChangeMap[prop] = true)
      const validator = () => ({ valid: false })

      const Model = new Schema({
        constant: {
          constant: true,
          value: 'constant',
          onDelete: onDelete('constant')
        },
        prop1: { required: true, onDelete: onDelete('prop1'), validator },
        prop2: { required: true, onDelete: onDelete('prop2'), validator },
        prop3: { required: true, onDelete: onDelete('prop3'), validator }
      }).getModel()

      beforeEach(() => {
        propChangeMap = {}
      })

      it('should trigger all onDelete handlers but for virtuals', async () => {
        await Model.delete({
          constant: true,
          prop1: true,
          prop2: true,
          prop3: true,
          prop4: true
        })

        expect(propChangeMap).toEqual({
          constant: true,
          prop1: true,
          prop2: true,
          prop3: true
        })
      })
    })

    describe('onFailure', () => {
      it('should reject onFailure & no validator', () => {
        const toFail = fx({ prop: { default: '', onFailure: () => {} } })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              prop: expect.arrayContaining([
                "'onFailure' can only be used with properties that support and have validators"
              ])
            })
          )
        }
      })

      describe('behaviour', () => {
        let onFailureCount: any = {}

        function incrementOnFailureCountOf(prop: string) {
          return () => {
            onFailureCount[prop] = (onFailureCount[prop] ?? 0) + 1
          }
        }
        const validator = () => ({ valid: false })

        const Model = new Schema({
          prop1: {
            default: true,
            onFailure: incrementOnFailureCountOf('prop1'),
            validator
          },
          prop2: {
            required: true,
            onFailure: [
              incrementOnFailureCountOf('prop2'),
              incrementOnFailureCountOf('prop2')
            ],
            validator
          },
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'virtualProp',
            resolver: () => ''
          },
          virtualProp: {
            virtual: true,
            onFailure: [
              incrementOnFailureCountOf('virtualProp'),
              incrementOnFailureCountOf('virtualProp'),
              incrementOnFailureCountOf('virtualProp')
            ],
            validator
          }
        }).getModel()

        beforeEach(() => {
          onFailureCount = {}
        })

        describe('creation', () => {
          it('should call onFailure handlers at creation', async () => {
            const { error } = await Model.create({ prop1: false })

            expect(error).toBeDefined()
            expect(onFailureCount).toEqual({ prop1: 1, prop2: 2 })
          })

          it('should call onFailure handlers at creation with virtuals', async () => {
            const { error } = await Model.create({
              prop1: false,
              virtualProp: 'Yes'
            })

            expect(error).toBeDefined()
            expect(onFailureCount).toEqual({
              prop1: 1,
              prop2: 2,
              virtualProp: 3
            })
          })

          it('should call onFailure handlers during cloning', async () => {
            const { error } = await Model.clone({ prop1: '' })

            expect(error).toBeDefined()
            expect(onFailureCount).toEqual({ prop1: 1, prop2: 2 })
          })

          it('should call onFailure handlers during cloning with virtuals', async () => {
            const { error } = await Model.clone({
              prop1: '',
              virtualProp: 'Yes'
            })

            expect(error).toBeDefined()
            expect(onFailureCount).toEqual({
              prop1: 1,
              prop2: 2,
              virtualProp: 3
            })
          })
        })

        describe('updates', () => {
          it('should call onFailure handlers during updates', async () => {
            const { error } = await Model.update({}, { prop1: '' })

            expect(error).toBeDefined()
            expect(onFailureCount).toEqual({ prop1: 1 })
          })

          it('should call onFailure handlers during updates with virtuals', async () => {
            const data = [
              [{ virtualProp: '' }, { virtualProp: 3 }],
              [
                { prop1: '', virtualProp: '' },
                { prop1: 1, virtualProp: 3 }
              ]
            ]

            for (const [changes, results] of data) {
              onFailureCount = {}

              const { error } = await Model.update({}, changes)

              expect(error).toBeDefined()
              expect(onFailureCount).toEqual(results)
            }
          })

          it('should call onFailure handlers during updates & nothing to update', async () => {
            const { error } = await Model.update({ prop1: 2 }, { prop1: 35 })

            expect(error).toBeDefined()
            expect(onFailureCount).toEqual({ prop1: 1 })
          })
        })
      })
    })

    describe('onSuccess', () => {
      let initialData = {
          dependent: false,
          lax: 'changed',
          readonly: 'changed',
          readonlyLax: '',
          required: 'changed'
        },
        propChangeMap: any = {},
        onSuccessValues: any = {}

      const onSuccess =
        (prop = '') =>
        (summary: any) => {
          propChangeMap[prop] = true
          onSuccessValues[prop] = summary
          onSuccessValues.__ctx = summary.context
        }

      const validator = () => ({ valid: true })

      const Model = new Schema({
        dependent: {
          default: false,
          dependent: true,
          dependsOn: 'readonlyLax',
          onSuccess: onSuccess('dependent'),
          resolver: () => true
        },
        lax: { default: '', onSuccess: onSuccess('lax'), validator },
        readonly: {
          readonly: true,
          onSuccess: onSuccess('readonly'),
          validator
        },
        readonlyLax: {
          default: '',
          readonly: 'lax',
          onSuccess: onSuccess('readonlyLax'),
          validator
        },
        required: {
          required: true,
          onSuccess: onSuccess('required'),
          validator
        }
      }).getModel()

      beforeEach(() => {
        propChangeMap = {}
        onSuccessValues = {}
      })

      // creation
      it('should call onSuccess handlers at creation', async () => {
        const { data, error, handleSuccess } = await Model.create({
          required: true,
          readonly: true
        })

        await handleSuccess()

        expect(error).toBe(null)
        expect(propChangeMap).toEqual({
          dependent: true,
          lax: true,
          readonly: true,
          readonlyLax: true,
          required: true
        })

        const changes = null,
          context = onSuccessValues.__ctx,
          operation = 'creation',
          previousValues = null,
          values = data,
          summary = { changes, context, operation, previousValues, values }

        expect(onSuccessValues).toMatchObject({
          dependent: summary,
          lax: summary,
          readonly: summary,
          readonlyLax: summary,
          required: summary
        })
      })

      // cloning
      it('should call onSuccess handlers during cloning', async () => {
        const initialData = {
          dependent: false,
          lax: '',
          readonly: true,
          readonlyLax: '',
          required: true
        }
        const { data, error, handleSuccess } = await Model.clone(initialData)

        await handleSuccess()

        expect(error).toBe(null)
        expect(propChangeMap).toEqual({
          dependent: true,
          lax: true,
          readonly: true,
          readonlyLax: true,
          required: true
        })

        const changes = null,
          context = onSuccessValues.__ctx,
          operation = 'creation',
          previousValues = null,
          values = { ...initialData, ...data },
          summary = { changes, context, operation, previousValues, values }

        expect(onSuccessValues).toMatchObject({
          dependent: summary,
          lax: summary,
          readonly: summary,
          readonlyLax: summary,
          required: summary
        })
      })

      // updates
      it('should call onSuccess handlers during updates with lax props', async () => {
        const { data, error, handleSuccess } = await Model.update(initialData, {
          lax: true
        })

        await handleSuccess()

        expect(error).toBe(null)
        expect(propChangeMap).toEqual({ lax: true })

        const operation = 'update'

        expect(onSuccessValues).toMatchObject({
          lax: expect.objectContaining({
            changes: data,
            context: onSuccessValues.__ctx,
            operation,
            previousValues: initialData,
            values: { ...initialData, lax: true }
          })
        })
      })

      it('should call onSuccess handlers during updates with readonlyLax & dependent', async () => {
        const { data, error, handleSuccess } = await Model.update(initialData, {
          readonlyLax: true
        })

        await handleSuccess()

        expect(error).toBe(null)
        expect(propChangeMap).toEqual({ dependent: true, readonlyLax: true })

        const changes = data,
          context = onSuccessValues.__ctx,
          operation = 'update',
          previousValues = initialData,
          values = { ...initialData, ...data },
          summary = { changes, context, operation, previousValues, values }

        expect(onSuccessValues).toMatchObject({
          dependent: summary,
          readonlyLax: summary
        })
      })
    })
  })
}
