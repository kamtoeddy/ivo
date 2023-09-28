import { beforeAll, describe, it, expect } from 'vitest'

import { expectFailure, expectNoFailure, validator } from '../_utils'

export const Test_ReadonlyProperties = ({ Schema, fx }: any) => {
  describe('readonly', () => {
    describe('valid', () => {
      it('should allow readonly(true) + dependent + default', () => {
        const toPass = fx({
          dependentProp: {
            default: 'value',
            dependent: true,
            dependsOn: 'prop',
            resolver: () => 1,
            readonly: true
          },
          prop: { default: '' }
        })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow readonly(true) + requiredBy', () => {
        const toPass = fx({
          propertyName: {
            default: '',
            readonly: true,
            required: () => true,
            validator
          }
        })

        expectNoFailure(toPass)

        toPass()
      })

      describe('behaviour', () => {
        let Model: any

        beforeAll(() => {
          Model = new Schema({
            age: { readonly: true, default: null },
            name: {
              default: 'Default Name'
            }
          }).getModel()
        })

        it('should not modify readonly props that have changed via life cycle listeners at creation', async () => {
          const { data } = await Model.create({ age: 25 })

          expect(data).toMatchObject({ age: 25, name: 'Default Name' })
        })

        it('should not modify readonly props that have changed via life cycle listeners during updates', async () => {
          const { data } = await Model.update(
            { age: null, name: 'Default Name' },
            { age: 25, name: 'YoYo' }
          )

          expect(data).toMatchObject({ age: 25, name: 'YoYo' })
        })
      })
    })

    describe('invalid', () => {
      it("should reject !readonly(true | 'lax')", () => {
        const values = [1, '', null, undefined, false]

        for (const readonly of values) {
          const toFail = fx({ propertyName: { readonly } })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Readonly properties are either true | 'lax'"
                ])
              })
            )
          }
        }
      })

      it('should reject readonly & required', () => {
        const values = [true, false]

        for (const required of values) {
          const toFail = fx({ propertyName: { readonly: true, required } })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  'Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule'
                ])
              })
            )
          }
        }
      })

      it('should reject readonly(true) + dependent & no default', () => {
        const toFail = fx({
          dependentProp: {
            dependent: true,
            dependsOn: 'prop',
            resolver: () => 1
          },
          prop: { default: '' }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties must have a default value'
              ])
            })
          )
        }
      })

      it('should reject readonly(lax) + dependent', () => {
        const toFail = fx({
          dependentProp: {
            dependent: true,
            default: '',
            dependsOn: 'prop',
            resolver: () => 1,
            readonly: 'lax'
          },
          prop: { default: '' }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Readonly(lax) properties cannot be dependent'
              ])
            })
          )
        }
      })

      it('should reject readonly(lax) & no default', () => {
        const toFail = fx({ propertyName: { readonly: 'lax' } })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect.objectContaining({
            propertyName: expect.arrayContaining([
              'Readonly properties must have a default value or a default setter'
            ])
          })
        }
      })

      it('should reject readonly(lax) & !shouldInit(undefined)', () => {
        const values = [false, true]

        for (const shouldInit of values) {
          const toFail = fx({
            propertyName: { default: '', readonly: 'lax', shouldInit }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  'Lax properties cannot have initialization blocked'
                ])
              })
            )
          }
        }
      })
    })
  })
}
