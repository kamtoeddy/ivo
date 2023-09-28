import { afterEach, describe, it, expect } from 'vitest'

import { expectFailure, expectNoFailure, pauseFor, validator } from '../_utils'

export const Test_DependentProperties = ({ Schema, fx }: any) => {
  describe('dependent', () => {
    const resolver = () => 1

    describe('behaviour', () => {
      let onDeleteStats = {} as Record<string, number | undefined>
      let onSuccessStats = {} as Record<string, number | undefined>
      let resolversCalledStats = {} as Record<string, number | undefined>

      const successCountOfDependentProps = {
        dependentProp: 4,
        dependentProp_1: 1,
        dependentProp_2: 3,
        dependentProp_3: 1
      }

      const Model = new Schema({
        laxProp: { default: '' },
        laxProp_1: { default: '' },
        laxProp_2: {
          default: '',
          onDelete: incrementOnDeleteCountOf('laxProp_2')
        },
        dependentProp: {
          default: 0,
          dependent: true,
          dependsOn: ['laxProp', 'laxProp_1'],
          resolver: resolverOfDependentProp,
          onDelete: [
            incrementOnDeleteCountOf('dependentProp'),
            incrementOnDeleteCountOf('dependentProp')
          ],
          onSuccess: [
            incrementOnSuccessCountOf('dependentProp'),
            incrementOnSuccessCountOf('dependentProp'),
            incrementOnSuccessCountOf('dependentProp'),
            incrementOnSuccessCountOf('dependentProp')
          ]
        },
        dependentProp_1: {
          default: 0,
          dependent: true,
          dependsOn: 'dependentProp',
          resolver: resolverOfDependentProp_1,
          onDelete: incrementOnDeleteCountOf('dependentProp_1'),
          onSuccess: incrementOnSuccessCountOf('dependentProp_1')
        },
        dependentProp_2: {
          default: 0,
          dependent: true,
          dependsOn: 'dependentProp',
          readonly: true,
          resolver: asyncResolver('dependentProp_2'),
          onDelete: [
            incrementOnDeleteCountOf('dependentProp_2'),
            incrementOnDeleteCountOf('dependentProp_2')
          ],
          onSuccess: [
            incrementOnSuccessCountOf('dependentProp_2'),
            incrementOnSuccessCountOf('dependentProp_2'),
            incrementOnSuccessCountOf('dependentProp_2')
          ]
        },
        dependentProp_3: {
          default: 0,
          dependent: true,
          dependsOn: 'laxProp_2',
          resolver: asyncResolver('dependentProp_3'),
          onDelete: [
            incrementOnDeleteCountOf('dependentProp_3'),
            incrementOnDeleteCountOf('dependentProp_3')
          ],
          onSuccess: [incrementOnSuccessCountOf('dependentProp_3')]
        }
      }).getModel()

      function incrementOnDeleteCountOf(prop: string) {
        return () => {
          const previousCount = onDeleteStats[prop] ?? 0

          onDeleteStats[prop] = previousCount + 1
        }
      }

      function incrementOnSuccessCountOf(prop: string) {
        return () => {
          const previousCount = onSuccessStats[prop] ?? 0

          onSuccessStats[prop] = previousCount + 1
        }
      }

      function incrementResolveCountOf(prop: string) {
        const previousCount = resolversCalledStats[prop] ?? 0

        resolversCalledStats[prop] = previousCount + 1
      }

      function resolverOfDependentProp({
        context: { laxProp, laxProp_1 }
      }: any) {
        incrementResolveCountOf('dependentProp')

        return laxProp.length + laxProp_1.length
      }

      function resolverOfDependentProp_1({ context: { dependentProp } }: any) {
        incrementResolveCountOf('dependentProp_1')

        return dependentProp + 1
      }

      function asyncResolver(prop: string) {
        return async ({ context: { dependentProp } }: any) => {
          incrementResolveCountOf(prop)

          await pauseFor()

          return dependentProp + 2
        }
      }

      afterEach(() => {
        onDeleteStats = {}
        onSuccessStats = {}
        resolversCalledStats = {}
      })

      describe('creation', () => {
        it('should resolve dependent properties correctly at creation', async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp_2: 'value based pricing',
            dependentProp: 25,
            dependentProp_1: 34,
            dependentProp_2: 17,
            dependentProp_3: 1
          })

          await handleSuccess?.()

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2
          })

          expect(resolversCalledStats).toEqual({ dependentProp_3: 1 })
          expect(onSuccessStats).toEqual(successCountOfDependentProps)
        })

        it('should resolve dependencies of dependent properties if provided at creation', async () => {
          const { data, handleSuccess } = await Model.create({
            laxProp: '',
            laxProp_1: 'hello',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0
          })

          await handleSuccess()

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: 'hello',
            laxProp_2: '',
            dependentProp: 5,
            dependentProp_1: 6,
            dependentProp_2: 7,
            dependentProp_3: 0
          })

          expect(resolversCalledStats).toEqual({
            dependentProp: 1,
            dependentProp_1: 1,
            dependentProp_2: 1
          })

          expect(onSuccessStats).toEqual(successCountOfDependentProps)
        })
      })

      describe('cloning', () => {
        it("should have all correct properties and values at creation with 'clone' method", async () => {
          const { data: clone, handleSuccess } = await Model.clone({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2
          })

          await handleSuccess()

          expect(clone).toMatchObject({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2
          })

          expect(resolversCalledStats).toEqual({ dependentProp_3: 1 })

          expect(onSuccessStats).toEqual(successCountOfDependentProps)
        })

        it("should respect 'reset' option at creation with 'clone' method", async () => {
          const { data: clone, handleSuccess } = await Model.clone(
            {
              laxProp: '',
              laxProp_1: '',
              laxProp_2: 'value based pricing',
              dependentProp: 20,
              dependentProp_1: 1302,
              dependentProp_2: 10,
              dependentProp_3: 350
            },
            {
              reset: ['dependentProp', 'dependentProp_1', 'dependentProp_2']
            }
          )

          await handleSuccess()

          expect(clone).toMatchObject({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2
          })

          expect(resolversCalledStats).toEqual({ dependentProp_3: 1 })

          expect(onSuccessStats).toEqual(successCountOfDependentProps)
        })

        it('should resolve dependencies of dependent properties if provided at creation(cloning)', async () => {
          const { data, handleSuccess } = await Model.clone({
            laxProp: '',
            laxProp_1: 'hello',
            laxProp_2: '',
            dependentProp: 5,
            dependentProp_1: 6,
            dependentProp_2: 7,
            dependentProp_3: 0
          })

          await handleSuccess()

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: 'hello',
            laxProp_2: '',
            dependentProp: 5,
            dependentProp_1: 6,
            dependentProp_2: 7,
            dependentProp_3: 0
          })

          expect(resolversCalledStats).toEqual({
            dependentProp: 1,
            dependentProp_1: 2,
            dependentProp_2: 2
          })

          expect(onSuccessStats).toEqual(successCountOfDependentProps)
        })

        it("should resolve dependencies of dependent properties if provided at creation(cloning) and also respect the 'reset' option", async () => {
          const { data, handleSuccess } = await Model.clone(
            {
              laxProp: '',
              laxProp_1: 'hello',
              laxProp_2: '',
              dependentProp: 5,
              dependentProp_1: 6,
              dependentProp_2: 7,
              dependentProp_3: 0
            },
            { reset: 'dependentProp' }
          )

          await handleSuccess()

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: 'hello',
            laxProp_2: '',
            dependentProp: 5,
            dependentProp_1: 6,
            dependentProp_2: 7,
            dependentProp_3: 0
          })

          expect(resolversCalledStats).toEqual({
            dependentProp: 1,
            dependentProp_1: 1,
            dependentProp_2: 1
          })

          expect(onSuccessStats).toEqual(successCountOfDependentProps)
        })
      })

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
              dependentProp_3: 2
            },
            {
              laxProp_2: 'hey',
              dependentProp: 74,
              dependentProp_1: 235,
              dependentProp_2: 72,
              dependentProp_3: 702
            }
          )

          await handleSuccess()

          expect(updates).toMatchObject({ laxProp_2: 'hey' })

          expect(resolversCalledStats).toEqual({ dependentProp_3: 1 })

          expect(onSuccessStats).toEqual({})
        })

        it('should resolve dependencies of dependent properties if provided during updates', async () => {
          const { data, handleSuccess } = await Model.update(
            {
              laxProp: '',
              laxProp_1: '',
              laxProp_2: '',
              dependentProp: 0,
              dependentProp_1: 0,
              dependentProp_2: 0,
              dependentProp_3: 0
            },
            {
              laxProp: 'hello',
              laxProp_1: 'world'
            }
          )

          await handleSuccess()

          expect(data).toEqual({
            laxProp: 'hello',
            laxProp_1: 'world',
            dependentProp: 10,
            dependentProp_1: 11,
            dependentProp_2: 12
          })

          const { dependentProp_3, ...stats } = successCountOfDependentProps

          expect(resolversCalledStats).toEqual({
            dependentProp: 1,
            dependentProp_1: 1,
            dependentProp_2: 1
          })

          expect(onSuccessStats).toEqual(stats)
        })

        it('should not resolve readonly dependent properties that have changed if provided during updates', async () => {
          const { data, handleSuccess } = await Model.update(
            {
              laxProp: 'hello',
              laxProp_1: 'world',
              dependentProp: 10,
              dependentProp_1: 11,
              dependentProp_2: 12,
              dependentProp_3: 0
            },
            { laxProp: '', laxProp_1: 'hey' }
          )

          await handleSuccess()

          expect(data).toEqual({
            laxProp: '',
            laxProp_1: 'hey',
            dependentProp: 3,
            dependentProp_1: 4
          })

          const stats = {
            dependentProp: 1,
            dependentProp_1: 1
          }

          expect(resolversCalledStats).toEqual(stats)

          expect(onSuccessStats).toEqual({
            dependentProp: 4,
            dependentProp_1: 1
          })
        })

        it('should not consider new resolved values of dependent properties if they are not different from previous values during updates', async () => {
          const { data, handleSuccess } = await Model.update(
            {
              laxProp: 'hello',
              laxProp_1: 'world',
              dependentProp: 3,
              dependentProp_1: 4,
              dependentProp_2: 12,
              dependentProp_3: 0
            },
            { laxProp: '', laxProp_1: 'hey' }
          )

          await handleSuccess()

          expect(data).toEqual({ laxProp: '', laxProp_1: 'hey' })

          expect(resolversCalledStats).toEqual({ dependentProp: 1 })

          expect(onSuccessStats).toEqual({})
        })
      })

      describe('deletion', () => {
        it('should have all correct properties and values at creation', async () => {
          await Model.delete({
            laxProp: '',
            laxProp_1: '',
            laxProp_2: 'value based pricing',
            dependentProp: 0,
            dependentProp_1: 0,
            dependentProp_2: 0,
            dependentProp_3: 2
          })

          expect(onDeleteStats).toEqual({
            laxProp_2: 1,
            dependentProp: 2,
            dependentProp_1: 1,
            dependentProp_2: 2,
            dependentProp_3: 2
          })
        })
      })
    })

    describe('valid', () => {
      it('should accept dependent & default(any | function)', () => {
        const values = ['', 1, false, true, null, {}, []]

        for (const value of values) {
          const toPass = fx({
            dependentProp: {
              default: value,
              dependent: true,
              dependsOn: 'prop',
              resolver
            },
            prop: { default: '' }
          })

          expectNoFailure(toPass)

          toPass()
        }
      })

      it("should accept life cycle listeners except 'onFailure'", () => {
        const lifeCycles = ['onDelete', 'onSuccess']
        const values = [() => {}, () => ({}), [() => {}, () => ({})]]

        for (const lifeCycle of lifeCycles) {
          for (const value of values) {
            const toPass = fx({
              dependentProp: {
                default: value,
                dependent: true,
                dependsOn: 'prop',
                resolver,
                [lifeCycle]: value
              },
              prop: { default: '' }
            })

            expectNoFailure(toPass)

            toPass()
          }
        }
      })

      it('should accept dependsOn & resolver', () => {
        const values = [
          'prop',
          ['prop', 'prop1'],
          ['prop', 'prop1', 'prop2', 'prop3']
        ]

        for (const dependsOn of values) {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn,
              resolver
            },
            prop: { default: '' },
            prop1: { default: '' },
            prop2: { default: '' },
            prop3: { default: '' }
          })

          expectNoFailure(toPass)

          toPass()
        }
      })

      it('should allow a dependent prop to depend on another dependent prop (non-circular)', () => {
        const toPass = fx({
          dependentProp1: {
            default: '',
            dependent: true,
            dependsOn: 'prop',
            resolver
          },
          dependentProp2: {
            default: '',
            dependent: true,
            dependsOn: 'dependentProp1',
            resolver
          },
          prop: { default: '' }
        })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow a dependency on virtuals', () => {
        const toPass = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'virtualProp',
            resolver
          },
          virtualProp: { virtual: true, validator: resolver }
        })

        expectNoFailure(toPass)

        toPass()
      })
    })

    describe('invalid', () => {
      it('should reject dependency on non-properties', () => {
        const invalidProp = 'invalidProp'

        const toFail = fx({
          dependentProp: {
            dependent: true,
            default: '',
            dependsOn: invalidProp,
            resolver
          }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                `Cannot establish dependency with '${invalidProp}' as it is neither a property nor a virtual of your model`
              ])
            })
          )
        }
      })

      it('should reject no dependent + dependsOn or resolver', () => {
        const toFail = fx({
          dependentProp: { default: '', dependsOn: 'prop', resolver },
          prop: { default: '' }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'dependsOn & resolver rules can only belong to dependent properties'
              ])
            })
          )
        }
      })

      it('should not allow property to depend on itself', () => {
        const toFail = fx({
          dependentProp: {
            dependent: true,
            default: '',
            dependsOn: 'dependentProp',
            resolver
          }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'A property cannot depend on itself'
              ])
            })
          )
        }
      })

      it('should not allow property to depend on a constant property', () => {
        const toFail = fx({
          constantProp: {
            constant: true,
            value: ''
          },
          dependentProp: {
            dependent: true,
            default: '',
            dependsOn: 'constantProp',
            resolver
          }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'A property cannot depend on a constant property'
              ])
            })
          )
        }
      })

      it('should identify circular dependencies and reject', () => {
        const toFail = fx({
          A: {
            default: '',
            dependent: true,
            dependsOn: ['B', 'C', 'D'],
            resolver
          },
          B: {
            default: '',
            dependent: true,
            dependsOn: ['A', 'C', 'E'],
            resolver
          },
          C: {
            default: '',
            dependent: true,
            dependsOn: ['A'],
            resolver
          },
          D: {
            default: '',
            dependent: true,
            dependsOn: 'E',
            resolver
          },
          E: {
            default: '',
            dependent: true,
            dependsOn: 'A',
            resolver
          },
          F: {
            default: '',
            dependent: true,
            dependsOn: 'prop',
            resolver
          },
          prop: { default: '' }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              A: expect.arrayContaining([
                "Circular dependency identified with 'B'",
                "Circular dependency identified with 'C'",
                "Circular dependency identified with 'E'"
              ]),
              B: expect.arrayContaining([
                "Circular dependency identified with 'A'"
              ]),
              C: expect.arrayContaining([
                "Circular dependency identified with 'A'",
                "Circular dependency identified with 'B'"
              ]),
              D: expect.arrayContaining([
                "Circular dependency identified with 'A'"
              ]),
              E: expect.arrayContaining([
                "Circular dependency identified with 'B'",
                "Circular dependency identified with 'D'"
              ])
            })
          )
        }
      })

      it('should reject dependent + missing default', () => {
        const toFail = fx({ dependentProp: { dependent: true } })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties must have a default value'
              ])
            })
          )
        }
      })

      it('should reject dependent + missing dependsOn', () => {
        const toFail = fx({
          propertyName: { dependent: true, default: '', resolver }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'Dependent properties must depend on atleast one property'
              ])
            })
          )
        }
      })

      it('should reject dependent + missing resolver', () => {
        const toFail = fx({
          dependentProp: { dependent: true, default: '', dependsOn: 'prop' },
          prop: { default: '' }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties must have a resolver'
              ])
            })
          )
        }
      })

      it('should reject dependent & shouldInit', () => {
        const values = [false, true]

        for (const shouldInit of values) {
          const toFail = fx({
            dependentProp: {
              dependent: true,
              default: '',
              dependsOn: 'prop',
              resolver,
              shouldInit
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
                  'Dependent properties cannot have shouldInit rule'
                ])
              })
            )
          }
        }
      })

      it('should reject dependent & readonly(lax)', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'prop',
            resolver,
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
                "Dependent properties cannot be readonly 'lax'"
              ])
            })
          )
        }
      })

      it('should reject dependent & validator', () => {
        const values = [null, '', 1, true, false, validator]

        for (const validator of values) {
          const toFail = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'prop',
              resolver,
              validator
            },
            prop: { default: '' }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  'Dependent properties cannot be validated'
                ])
              })
            )
          }
        }
      })

      it('should reject dependent & required', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'prop',
            resolver,
            required: true
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
                'Dependent properties cannot be required'
              ])
            })
          )
        }
      })

      it('should reject dependent + requiredBy', () => {
        const toFail = fx({
          dependentProp: {
            required() {
              return true
            },
            requiredError: "'dependentProp' is required",
            dependent: true,
            default: '',
            dependsOn: 'prop',
            resolver: () => 1
          },
          prop: { default: '' }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties cannot be required'
              ])
            })
          )
        }
      })
    })
  })
}
