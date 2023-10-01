import { beforeAll, beforeEach, describe, it, expect } from 'vitest'

import { expectFailure, expectNoFailure } from '../_utils'

export const Test_ShouldInitRule = ({ Schema, fx }: any) => {
  describe('shouldInit', () => {
    describe('valid', () => {
      describe('behaviour', () => {
        const Model = new Schema({
          isBlocked: {
            default: false,
            shouldInit: (ctx: any) => ctx.env == 'test'
          },
          env: { default: 'dev' },
          laxProp: { default: 0 }
        }).getModel()

        it('should respect default rules', async () => {
          const { data } = await Model.create({ isBlocked: true })

          expect(data).toMatchObject({
            env: 'dev',
            isBlocked: false,
            laxProp: 0
          })
        })

        it('should respect callable should init when condition passes during cloning', async () => {
          const { data } = await Model.clone({
            env: 'test',
            isBlocked: 'yes'
          })

          expect(data).toMatchObject({
            env: 'test',
            isBlocked: 'yes',
            laxProp: 0
          })
        })

        it('should respect callable should init when condition passes at creation', async () => {
          const { data } = await Model.create({
            env: 'test',
            isBlocked: 'yes'
          })

          expect(data).toMatchObject({
            env: 'test',
            isBlocked: 'yes',
            laxProp: 0
          })
        })

        describe('behaviour when shouldInit method returns nothing', () => {
          const Model = new Schema({
            isBlocked: { default: false, shouldInit: () => {} },
            laxProp: { default: 0 }
          }).getModel()

          it('should assume initialization as falsy if shouldInit method returns nothing at creation', async () => {
            const { data } = await Model.create({ isBlocked: 'yes' })

            expect(data).toMatchObject({ isBlocked: false, laxProp: 0 })
          })

          it('should assume initialization as falsy if shouldInit method returns nothing during cloning', async () => {
            const { data } = await Model.clone({ isBlocked: true, laxProp: 0 })

            expect(data).toMatchObject({ isBlocked: false, laxProp: 0 })
          })
        })
      })

      describe('behaviour of callable shouldInit', () => {
        let onSuccessValues: any = {}

        let onSuccessStats: any = {}

        let sanitizedValues: any = {}

        let Model: any

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
          ).getModel()

          function sanitizerOf(prop: string, value: any) {
            return () => {
              // to make sure sanitizer is invoked
              sanitizedValues[prop] = value

              return value
            }
          }

          function incrementOnSuccessStats(prop: string) {
            return () => {
              onSuccessStats[prop] = (onSuccessStats[prop] ?? 0) + 1
            }
          }

          function onSuccess(prop: string) {
            return ({ context }: any) => {
              onSuccessValues[prop] = context[prop]
              incrementOnSuccessStats(prop)()
            }
          }

          function validateBoolean(value: any) {
            if (![false, true].includes(value))
              return { valid: false, reason: `${value} is not a boolean` }
            return { valid: true }
          }
        })

        beforeEach(() => {
          onSuccessStats = {}
          onSuccessValues = {}
          sanitizedValues = {}
        })

        it("should ignore virtuals at creation when their shouldInit handler returns 'false'", async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: 'Peter',
            virtual: true
          })

          await handleSuccess()

          expect(data).toEqual({ dependent: '', laxProp: 'Peter' })

          expect(onSuccessStats).toEqual({ dependent: 1 })

          expect(onSuccessValues).toEqual({ dependent: '' })

          expect(sanitizedValues).toEqual({})
        })

        it("should ignore virtuals at creation(cloning) when their shouldInit handler returns 'false'", async () => {
          const { data, handleSuccess } = await Model.clone({
            dependent: '',
            laxProp: 'Peter',
            virtual: true
          })

          await handleSuccess()

          expect(data).toEqual({ dependent: '', laxProp: 'Peter' })

          expect(onSuccessStats).toEqual({ dependent: 1 })

          expect(onSuccessValues).toEqual({ dependent: '' })

          expect(sanitizedValues).toEqual({})
        })

        it("should respect virtuals at creation when their shouldInit handler returns 'true'", async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: 'allow virtual',
            virtual: true
          })

          await handleSuccess()

          expect(data).toEqual({
            dependent: 'changed',
            laxProp: 'allow virtual'
          })

          expect(onSuccessStats).toEqual({ dependent: 1, virtual: 3 })

          expect(onSuccessValues).toEqual({
            dependent: 'changed',
            virtual: 'sanitized'
          })

          expect(sanitizedValues).toEqual({ virtual: 'sanitized' })
        })

        it("should respect virtuals at creation(cloning) when their shouldInit handler returns 'true'", async () => {
          const { data, handleSuccess } = await Model.clone({
            dependent: '',
            laxProp: 'allow virtual',
            virtual: true
          })

          await handleSuccess()

          expect(data).toEqual({
            dependent: 'changed',
            laxProp: 'allow virtual'
          })

          expect(onSuccessStats).toEqual({ dependent: 1, virtual: 3 })

          expect(onSuccessValues).toEqual({
            dependent: 'changed',
            virtual: 'sanitized'
          })

          expect(sanitizedValues).toEqual({ virtual: 'sanitized' })
        })
      })

      it('should accept shouldInit(false) + default', () => {
        const fxn = fx({
          propertyName: { shouldInit: false, default: true }
        })

        expectNoFailure(fxn)

        fxn()
      })

      it('should accept shouldInit: () => boolean + default', () => {
        const values = [() => true, () => false]

        for (const shouldInit of values) {
          const fxn = fx({
            propertyName: { shouldInit, default: true }
          })

          expectNoFailure(fxn)

          fxn()
        }
      })
    })

    describe('invalid', () => {
      it('should reject shouldInit(false) & no default', () => {
        const fxn = fx({ propertyName: { shouldInit: false } })

        expectFailure(fxn)

        try {
          fxn()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'A property with initialization blocked must have a default value'
              ])
            })
          )
        }
      })

      it('should reject shouldInit !(boolean | () => boolean)', () => {
        const values = [undefined, 1, {}, null, [], 'yes', 'false', 'true']

        for (const shouldInit of values) {
          const fxn = fx({ propertyName: { shouldInit, default: true } })

          expectFailure(fxn)

          try {
            fxn()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean"
                ])
              })
            )
          }
        }
      })
    })
  })
}
