import { DEFINITION_RULES, VIRTUAL_RULES } from '../../../../schema/types'
import { ERRORS } from '../../../..'

import { expectFailure, expectNoFailure, validator } from '../_utils'

export const Test_VirtualProperties = ({ Schema, fx }: any) => {
  describe('virtual', () => {
    describe('valid', () => {
      describe('alias', () => {
        it('should allow alias', () => {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'propertyName',
              resolver: () => ''
            },
            propertyName: {
              alias: 'alias',
              virtual: true,
              sanitizer: () => '',
              validator
            }
          })

          expectNoFailure(toPass)

          toPass()
        })

        it('should allow alias if it is the same as a related dependency of the virtual', () => {
          const dependentProp = 'dependentProp'
          const virtualProp = 'virtualProp'

          const toPass = fx({
            [dependentProp]: {
              default: '',
              dependent: true,
              dependsOn: virtualProp,
              resolver: () => ''
            },
            [virtualProp]: {
              alias: dependentProp,
              virtual: true,
              sanitizer: () => '',
              validator
            }
          })

          expectNoFailure(toPass)

          toPass()
        })

        describe('Model.validate', () => {
          function validator(v: any) {
            return v === true
              ? { valid: true, validated: 'validated' }
              : { valid: false }
          }

          it('should call the validator of corresponding virtual when alias is passed to Model.validate method', async () => {
            const Model = new Schema({
              dependentProp: {
                default: '',
                dependent: true,
                dependsOn: 'virtualProp',
                resolver: () => {}
              },
              virtualProp: {
                alias: 'alias',
                virtual: true,
                validator
              }
            }).getModel()

            const res = await Model.validate('alias', true)

            expect(res.valid).toBe(true)
            expect(res.validated).toBe('validated')

            const res1 = await Model.validate('alias', false)

            expect(res1.valid).toBe(false)
            expect(res1?.validated).toBeUndefined()
          })

          it("should not reject with 'invalid property error' if Model.validate is passed an alias with same name as a dependent property", async () => {
            const Model = new Schema({
              dependentProp: {
                default: '',
                dependent: true,
                dependsOn: 'virtualProp',
                resolver: () => {}
              },
              virtualProp: {
                alias: 'dependentProp',
                virtual: true,
                validator
              }
            }).getModel()

            const res = await Model.validate('dependentProp', true)

            expect(res.valid).toBe(true)
            expect(res.validated).toBe('validated')

            const res1 = await Model.validate('alias', false)

            expect(res1.valid).toBe(false)
            expect(res1?.validated).toBeUndefined()
          })
        })

        describe('behaviour', () => {
          let contextRecord = {} as Record<string, number | undefined>

          function resolver({ context: { setQuantity, qty } }: any) {
            if (qty !== undefined) contextRecord.qty = qty

            return setQuantity
          }

          function validator(v: any) {
            const _type = typeof v
            return _type === 'number'
              ? { valid: true, validated: v }
              : { valid: false, reason: 'Invalid quantity' }
          }

          const Model = new Schema({
            id: { constant: true, value: 1, onDelete: resolver },
            quantity: {
              default: 0.0,
              dependent: true,
              dependsOn: 'setQuantity',
              resolver
            },
            setQuantity: { alias: 'qty', virtual: true, validator }
          }).getModel()

          beforeEach(() => {
            contextRecord = {}
          })

          describe('cloning', () => {
            it('should respect alias if provided during cloning', async () => {
              const { data } = await Model.clone({
                id: 1,
                quantity: 2000,
                qty: 5
              })

              expect(data).toMatchObject({ id: 1, quantity: 5 })
              expect(contextRecord).toEqual({})
            })

            it('should return alias errors with alias name in error payload during cloning', async () => {
              const { error } = await Model.create({
                id: 1,
                quantity: 2000,
                qty: '5'
              })

              expect(error.payload).toMatchObject({
                qty: {
                  reasons: expect.arrayContaining(['Invalid quantity']),
                  metadata: null
                }
              })
              expect(contextRecord).toEqual({})
            })

            it('should respect precedence of virtual property and alias if both are provided during cloning', async () => {
              const operation1 = await Model.clone({
                id: 1,
                qty: 12,
                setQuantity: 50
              })

              expect(operation1.data).toMatchObject({ id: 1, quantity: 50 })

              const operation2 = await Model.clone({
                id: 1,
                setQuantity: 20,
                qty: 1
              })

              expect(operation2.data).toMatchObject({ id: 1, quantity: 1 })
            })
          })

          describe('creation', () => {
            it('should respect alias if provided at creation', async () => {
              const { data } = await Model.create({ qty: 12 })

              expect(data).toMatchObject({ id: 1, quantity: 12 })
              expect(contextRecord).toEqual({})
            })

            it("should use default values of dependent props to be set if an alias with that prop's name exists on the same schema at creation", async () => {
              const Model = new Schema({
                id: { constant: true, value: 1, onDelete: resolver },
                quantity: {
                  default: 0.0,
                  dependent: true,
                  dependsOn: 'setQuantity',
                  resolver
                },
                setQuantity: {
                  alias: 'quantity',
                  virtual: true,
                  shouldInit: false,
                  validator
                }
              }).getModel()

              const { data } = await Model.create({ quantity: 12 })

              expect(data).toMatchObject({ id: 1, quantity: 0 })
              expect(contextRecord).toEqual({})
            })

            it('should return alias errors with alias name in error payload at creation', async () => {
              const { error } = await Model.create({ qty: '12' })

              expect(error.payload).toMatchObject({
                qty: {
                  reasons: expect.arrayContaining(['Invalid quantity']),
                  metadata: null
                }
              })
              expect(contextRecord).toEqual({})
            })

            it('should respect precedence of virtual property and alias if both are provided at creation', async () => {
              const operation1 = await Model.create({
                qty: 12,
                setQuantity: 50
              })

              expect(operation1.data).toMatchObject({ id: 1, quantity: 50 })

              const operation2 = await Model.create({
                setQuantity: 20,
                qty: 1
              })

              expect(operation2.data).toMatchObject({ id: 1, quantity: 1 })
            })
          })

          describe('delete', () => {
            it('aliases should not be available in context during deletion', async () => {
              await Model.delete({ id: 1, quantity: 12, qty: 1000 })

              expect(contextRecord).toEqual({})
            })
          })

          describe('update', () => {
            it('should respect alias if provided during updates', async () => {
              const { data } = await Model.update(
                { id: 1, quantity: 12 },
                { qty: 5 }
              )

              expect(data).toMatchObject({ quantity: 5 })
              expect(contextRecord).toEqual({})
            })

            it('should return alias errors with alias name in error payload during updates', async () => {
              const { error } = await Model.update(
                { id: 1, quantity: 12 },
                { qty: '2' }
              )

              expect(error.payload).toMatchObject({
                qty: {
                  reasons: expect.arrayContaining(['Invalid quantity']),
                  metadata: null
                }
              })
              expect(contextRecord).toEqual({})
            })

            it('should respect precedence of virtual property and alias if both are provided during updates', async () => {
              const operation1 = await Model.update(
                { id: 1, quantity: 75 },
                {
                  qty: 12,
                  setQuantity: 50
                }
              )

              expect(operation1.data).toMatchObject({ quantity: 50 })

              const operation2 = await Model.update(
                { id: 1, quantity: 75 },
                {
                  setQuantity: 20,
                  qty: 1
                }
              )

              expect(operation2.data).toMatchObject({ quantity: 1 })
            })
          })

          describe("availability of virtuals in ctx of 'required' method of virtual", () => {
            const Model = new Schema({
              id: { constant: true, value: 1 },
              quantity: {
                default: 0.0,
                dependent: true,
                dependsOn: 'setQuantity',
                resolver
              },
              setQuantity: {
                alias: 'qty',
                virtual: true,
                required({ context: { setQuantity } }: any) {
                  contextRecord.setQuantity = setQuantity

                  if (setQuantity == -100) return true

                  return setQuantity == -1000
                    ? [true, 'invalid quantity']
                    : false
                },
                validator
              }
            }).getModel()

            it("should respect 'required' rule of virtual property even when alias is provided at creation", async () => {
              let qty = -100
              const operation1 = await Model.create({ id: 1, qty })

              expect(contextRecord).toEqual({ setQuantity: qty })
              expect(operation1.data).toBe(null)
              expect(operation1.error.payload).toEqual({
                qty: {
                  reasons: expect.arrayContaining(["'qty' is required!"]),
                  metadata: null
                },
                setQuantity: {
                  reasons: ["'setQuantity' is required!"],
                  metadata: null
                }
              })

              qty = -1000
              const operation2 = await Model.create({ id: 1, qty })

              expect(contextRecord).toEqual({ setQuantity: qty })
              expect(operation2.data).toBe(null)
              expect(operation2.error.payload).toEqual({
                qty: {
                  reasons: expect.arrayContaining(['invalid quantity']),
                  metadata: null
                },
                setQuantity: {
                  reasons: expect.arrayContaining(['invalid quantity']),
                  metadata: null
                }
              })
            })

            it("should respect 'required' rule of virtual property even when alias is provided during cloning", async () => {
              let qty = -100
              const entity = { id: 1, quantity: 100 }
              const operation1 = await Model.clone({ ...entity, qty })

              expect(contextRecord).toEqual({ setQuantity: qty })
              expect(operation1.data).toBe(null)
              expect(operation1.error.payload).toEqual({
                qty: {
                  reasons: expect.arrayContaining(["'qty' is required!"]),
                  metadata: null
                },
                setQuantity: {
                  reasons: ["'setQuantity' is required!"],
                  metadata: null
                }
              })

              qty = -1000
              const operation2 = await Model.clone({ ...entity, qty })

              expect(contextRecord).toEqual({ setQuantity: qty })
              expect(operation2.data).toBe(null)
              expect(operation2.error.payload).toEqual({
                qty: {
                  reasons: expect.arrayContaining(['invalid quantity']),
                  metadata: null
                },
                setQuantity: {
                  reasons: expect.arrayContaining(['invalid quantity']),
                  metadata: null
                }
              })
            })

            it("should respect 'required' rule of virtual property even when alias is provided during updates", async () => {
              let qty = -100
              const entity = { id: 1, quantity: 100 }
              const operation1 = await Model.update(entity, { qty })

              expect(contextRecord).toEqual({ setQuantity: qty })
              expect(operation1.data).toBe(null)
              expect(operation1.error.payload).toEqual({
                qty: {
                  reasons: expect.arrayContaining(["'qty' is required!"]),
                  metadata: null
                },
                setQuantity: {
                  reasons: expect.arrayContaining([
                    "'setQuantity' is required!"
                  ]),
                  metadata: null
                }
              })

              qty = -1000
              const operation2 = await Model.update(entity, { qty })

              expect(contextRecord).toEqual({ setQuantity: qty })
              expect(operation2.data).toBe(null)
              expect(operation2.error.payload).toEqual({
                qty: {
                  reasons: expect.arrayContaining(['invalid quantity']),
                  metadata: null
                },
                setQuantity: {
                  reasons: expect.arrayContaining(['invalid quantity']),
                  metadata: null
                }
              })
            })
          })

          describe("availability of virtuals in ctx of shouldInit & shouldUpdate methods of the virtual when it's alias is provided", () => {
            const Model = new Schema({
              id: { constant: true, value: 1, onDelete: resolver },
              quantity: {
                default: 0.0,
                dependent: true,
                dependsOn: 'setQuantity',
                resolver
              },
              setQuantity: {
                alias: 'qty',
                virtual: true,
                shouldInit({ setQuantity }: any) {
                  contextRecord.setQuantity = setQuantity

                  return setQuantity > 0
                },
                shouldUpdate({ quantity, setQuantity }: any) {
                  contextRecord.setQuantity = setQuantity

                  return setQuantity > quantity
                },
                validator
              }
            }).getModel()

            it("should respect 'shouldInit' rule of virtual property even when alias is provided at creation", async () => {
              const operation1 = await Model.create({ id: 1, qty: -75 })

              expect(contextRecord).toEqual({ setQuantity: -75 })
              expect(operation1.error).toBe(null)
              expect(operation1.data).toEqual({ id: 1, quantity: 0 })

              const operation2 = await Model.create({ id: 1, qty: 75 })

              expect(contextRecord).toEqual({ setQuantity: 75 })
              expect(operation2.error).toBe(null)
              expect(operation2.data).toEqual({ id: 1, quantity: 75 })
            })

            it("should respect 'shouldInit' rule of virtual property even when alias is provided during cloning", async () => {
              const operation1 = await Model.clone({
                id: 1,
                quantity: 0,
                qty: -75
              })

              expect(contextRecord).toEqual({ setQuantity: -75 })
              expect(operation1.error).toBe(null)
              expect(operation1.data).toEqual({ id: 1, quantity: 0 })

              const operation2 = await Model.clone({
                id: 1,
                quantity: 0,
                qty: 75
              })

              expect(contextRecord).toEqual({ setQuantity: 75 })
              expect(operation2.error).toBe(null)
              expect(operation2.data).toEqual({ id: 1, quantity: 75 })
            })

            it("should respect 'shouldUpdate' rule of virtual property even when alias is provided during updates", async () => {
              const operation1 = await Model.update(
                { id: 1, quantity: 75 },
                {
                  qty: 12
                }
              )

              expect(contextRecord).toEqual({ setQuantity: 12 })
              expect(operation1.error).toEqual({
                message: ERRORS.NOTHING_TO_UPDATE,
                payload: {}
              })
              expect(operation1.data).toBe(null)

              const operation2 = await Model.update(
                { id: 1, quantity: 75 },
                {
                  qty: 100
                }
              )

              expect(contextRecord).toEqual({ setQuantity: 100 })
              expect(operation2.error).toBe(null)
              expect(operation2.data).toMatchObject({ quantity: 100 })
            })
          })
        })
      })

      it('should allow sanitizer', () => {
        const toPass = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'propertyName',
            resolver: () => ''
          },
          propertyName: { virtual: true, sanitizer: () => '', validator }
        })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow onFailure', () => {
        const toPass = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'propertyName',
            resolver: () => ''
          },
          propertyName: {
            virtual: true,
            onFailure: validator,
            validator
          }
        })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow requiredBy + requiredError', () => {
        const toPass = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'propertyName',
            resolver: () => ''
          },
          propertyName: {
            virtual: true,
            required: () => true,
            validator
          }
        })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow shouldInit(false|()=>boolean) + validator', () => {
        const values = [false, () => false, () => true]

        for (const shouldInit of values) {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'propertyName',
              resolver: () => ''
            },
            propertyName: { virtual: true, shouldInit, validator }
          })

          expectNoFailure(toPass)

          toPass()
        }
      })

      it('should allow onSuccess + validator', () => {
        const values = [[], () => ({})]

        for (const onSuccess of values) {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'propertyName',
              resolver: () => ''
            },
            propertyName: { virtual: true, onSuccess, validator }
          })

          expectNoFailure(toPass)

          toPass()
        }
      })

      describe('behaviour', () => {
        let onSuccessValues: any = {}

        let onSuccessStats: any = {}

        let sanitizedValues: any = {}

        const User = new Schema(
          {
            dependentSideInit: {
              default: '',
              dependent: true,
              dependsOn: ['virtualInit', 'virtualWithSanitizer'],
              resolver({
                context: { virtualInit, virtualWithSanitizer }
              }: any) {
                return virtualInit && virtualWithSanitizer ? 'both' : 'one'
              },
              onSuccess: onSuccess('dependentSideInit')
            },
            dependentSideNoInit: {
              default: '',
              dependent: true,
              dependsOn: ['virtualNoInit', 'virtualWithSanitizerNoInit'],
              resolver: () => 'changed',
              onSuccess: onSuccess('dependentSideNoInit')
            },
            name: { default: '' },
            virtualInit: {
              virtual: true,
              onSuccess: onSuccess('virtualInit'),
              validator: validateBoolean
            },
            virtualNoInit: {
              virtual: true,
              shouldInit: false,
              onSuccess: [
                onSuccess('virtualNoInit'),
                incrementOnSuccessStats('virtualNoInit')
              ],
              validator: validateBoolean
            },
            virtualWithSanitizer: {
              virtual: true,
              onSuccess: [
                onSuccess('virtualWithSanitizer'),
                incrementOnSuccessStats('virtualWithSanitizer'),
                incrementOnSuccessStats('virtualWithSanitizer')
              ],
              sanitizer: sanitizerOf('virtualWithSanitizer', 'sanitized'),
              validator: validateBoolean
            },
            virtualWithSanitizerNoInit: {
              virtual: true,
              shouldInit: false,
              onSuccess: [
                onSuccess('virtualWithSanitizerNoInit'),
                incrementOnSuccessStats('virtualWithSanitizerNoInit')
              ],
              sanitizer: sanitizerOf(
                'virtualWithSanitizerNoInit',
                'sanitized no init'
              ),
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

        beforeEach(() => {
          onSuccessStats = {}
          onSuccessValues = {}
          sanitizedValues = {}
        })

        describe('creation', () => {
          it('should not sanitize virtuals nor resolve their dependencies if not provided', async () => {
            const { data } = await User.create({
              name: 'Peter'
            })

            expect(data).toEqual({
              dependentSideInit: '',
              dependentSideNoInit: '',
              name: 'Peter'
            })

            expect(sanitizedValues).toEqual({})
          })

          it('should respect sanitizer at creation', async () => {
            const { data } = await User.create({
              name: 'Peter',
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true
            })

            expect(data).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: '',
              name: 'Peter'
            })

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized'
            })
          })

          it('should respect sanitizer at creation(cloning)', async () => {
            const { data } = await User.clone({
              dependentSideNoInit: '',
              dependentSideInit: true,
              name: 'Peter',
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true
            })

            expect(data).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: '',
              name: 'Peter'
            })

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized'
            })
          })

          it('should respect virtualInits & virtualNoInit at creation', async () => {
            const { data: user, handleSuccess } = await User.create({
              dependentSideNoInit: '',
              dependentSideInit: true,
              name: 'Peter',

              virtualInit: true,
              virtualWithSanitizer: true,
              virtualWithSanitizerNoInit: true
            })

            await handleSuccess()

            expect(user).toEqual({
              dependentSideInit: 'both',
              dependentSideNoInit: '',
              name: 'Peter'
            })

            expect(onSuccessStats).toEqual({
              dependentSideInit: 1,
              dependentSideNoInit: 1,
              virtualInit: 1,
              virtualWithSanitizer: 3
            })

            expect(onSuccessValues).toEqual({
              dependentSideInit: 'both',
              dependentSideNoInit: '',
              virtualInit: true,
              virtualWithSanitizer: 'sanitized'
            })

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized'
            })
          })

          it('should respect virtualInits & virtualNoInit at creation(cloning)', async () => {
            const { data: user, handleSuccess } = await User.clone(
              {
                dependentSideNoInit: 'bignw ',
                dependentSideInit: 'iehvhgwop',
                name: 'Peter',
                virtualInit: true,
                virtualNoInit: true
              },
              { reset: ['dependentSideInit', 'dependentSideNoInit'] }
            )

            await handleSuccess()

            expect(user).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: '',
              name: 'Peter'
            })

            expect(onSuccessStats).toEqual({
              dependentSideInit: 1,
              dependentSideNoInit: 1,
              virtualInit: 1
            })

            expect(onSuccessValues).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: '',
              virtualInit: true
            })

            expect(sanitizedValues).toEqual({})
          })
        })

        describe('updating', () => {
          it('should respect sanitizer of all virtuals provided during updates', async () => {
            const { data, handleSuccess } = await User.update(
              { name: 'Peter' },
              {
                name: 'John',
                virtualWithSanitizer: true,
                virtualWithSanitizerNoInit: true
              }
            )

            await handleSuccess()

            expect(data).toEqual({
              name: 'John',
              dependentSideInit: 'one',
              dependentSideNoInit: 'changed'
            })

            expect(onSuccessStats).toEqual({
              dependentSideInit: 1,
              dependentSideNoInit: 1,
              virtualWithSanitizer: 3,
              virtualWithSanitizerNoInit: 2
            })

            expect(onSuccessValues).toEqual({
              dependentSideInit: 'one',
              dependentSideNoInit: 'changed',
              virtualWithSanitizer: 'sanitized',
              virtualWithSanitizerNoInit: 'sanitized no init'
            })

            expect(sanitizedValues).toEqual({
              virtualWithSanitizer: 'sanitized',
              virtualWithSanitizerNoInit: 'sanitized no init'
            })
          })
        })
      })
    })

    describe('invalid', () => {
      describe('alias', () => {
        it('should reject alias if definition does not have the virtual keyword', () => {
          const virtualProp = 'virtualProp'

          const toFail = fx({
            required: { alias: 'a1', required: true, validator },
            readonly: { alias: 's2', readonly: true, validator },
            lax1: { alias: 'a3', default: '' },
            lax2: { alias: 'a5', default: '', validator },
            dependentProp: {
              alias: 'lol',
              default: '',
              dependent: true,
              dependsOn: virtualProp,
              resolver: () => ''
            },
            [virtualProp]: { virtual: true, validator }
          })

          expectFailure(toFail)

          const expectedError = expect.arrayContaining([
            'Only virtual properties can have aliases'
          ])

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                required: expectedError,
                readonly: expectedError,
                lax1: expectedError,
                lax2: expectedError,
                dependentProp: expectedError
              })
            )
          }
        })

        it('should reject alias if non-empty string is provided', () => {
          const values = [-1, 1, true, false, undefined, '', null, [], {}]

          for (const alias of values) {
            const toFail = fx({
              dependentProp: {
                default: '',
                dependent: true,
                dependsOn: 'propertyName',
                resolver: () => ''
              },
              propertyName: { alias, virtual: true, validator }
            })

            expectFailure(toFail)

            try {
              toFail()
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    'An alias must be a string with atleast 1 character'
                  ])
                })
              )
            }
          }
        })

        it("should reject alias if it's same as the virtual property", () => {
          const virtualProp = 'virtualProp'

          const toFail = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: virtualProp,
              resolver: () => ''
            },
            [virtualProp]: {
              alias: virtualProp,
              virtual: true,
              validator
            }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  'An alias cannot be the same as the virtual property'
                ])
              })
            )
          }
        })

        it('should reject alias if already used by another virtual', () => {
          const alias = 'alias'
          const virtualProp = 'virtualProp'

          const toFail = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: [virtualProp, 'virtualProp1'],
              resolver: () => ''
            },
            [virtualProp]: {
              alias,
              virtual: true,
              validator
            },
            virtualProp1: {
              alias,
              virtual: true,
              validator
            }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                virtualProp1: expect.arrayContaining([
                  `Sorry, alias provided '${alias}' already belongs to property '${virtualProp}'`
                ])
              })
            )
          }
        })

        it('should reject alias if it is the same as the name of existing virtual', () => {
          const alias = 'virtualProp1'
          const virtualProp = 'virtualProp'

          const toFail = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: [virtualProp, 'virtualProp1'],
              resolver: () => ''
            },
            [virtualProp]: {
              alias,
              virtual: true,
              validator
            },
            virtualProp1: {
              virtual: true,
              validator
            }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  `'${alias}' cannot be used as the alias of '${virtualProp}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`
                ])
              })
            )
          }
        })

        it('should reject alias if it is the same as the name of existing property', () => {
          const laxProp = 'laxProp'
          const virtualProp = 'virtualProp'

          const toFail = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: virtualProp,
              resolver: () => ''
            },
            [virtualProp]: {
              alias: laxProp,
              virtual: true,
              validator
            },
            [laxProp]: { default: true }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  `'${laxProp}' cannot be used as the alias of '${virtualProp}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`
                ])
              })
            )
          }
        })

        it('should reject alias if it is the same as an unrelated dependent property', () => {
          const dependentProp = 'dependentProp'
          const virtualProp = 'virtualProp'

          const toFail = fx({
            [dependentProp]: {
              default: '',
              dependent: true,
              dependsOn: virtualProp,
              resolver: () => ''
            },
            [virtualProp]: {
              alias: 'dependentProp1',
              virtual: true,
              validator
            },
            dependentProp1: {
              default: '',
              dependent: true,
              dependsOn: 'virtualProp1',
              resolver: () => ''
            },
            virtualProp1: {
              virtual: true,
              validator
            }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                [virtualProp]: expect.arrayContaining([
                  `'dependentProp1' cannot be used as the alias of '${virtualProp}' because it is the name of an existing property on your schema. To use an alias that matches another property on your schema, this property must be dependent on the said virtual property`
                ])
              })
            )
          }
        })
      })

      describe('sanitizers', () => {
        it('should reject invalid sanitizer', () => {
          const values = [-1, 1, true, false, undefined, null, [], {}]

          for (const sanitizer of values) {
            const toFail = fx({
              propertyName: { virtual: true, sanitizer, validator }
            })

            expectFailure(toFail)

            try {
              toFail()
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "'sanitizer' must be a function"
                  ])
                })
              )
            }
          }
        })

        it("should reject 'sanitizer' rule on non-virtuals", () => {
          const values = [-1, 1, true, false, undefined, null, [], {}, () => {}]

          for (const sanitizer of values) {
            const toFail = fx({ propertyName: { default: '', sanitizer } })

            expectFailure(toFail)

            try {
              toFail()
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "'sanitizer' is only valid on virtuals"
                  ])
                })
              )
            }
          }
        })
      })

      it('should reject virtual & no dependent property ', () => {
        const toFail = fx({
          propertyName: { virtual: true, validator }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: [
                'A virtual property must have atleast one property that depends on it'
              ]
            })
          )
        }
      })

      it('should reject virtual & no validator ', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'propertyName',
            resolver: () => ''
          },
          propertyName: { virtual: true }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining(['Invalid validator'])
            })
          )
        }
      })

      it('should reject requiredBy + shouldInit', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'propertyName',
            resolver: () => ''
          },
          propertyName: {
            virtual: true,
            shouldInit: false,
            required: () => true,
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
                'Required virtuals cannot have initialization blocked'
              ])
            })
          )
        }
      })

      it('should reject required(true)', () => {
        const toFail = fx({
          dependentProp: {
            default: '',
            dependent: true,
            dependsOn: 'propertyName',
            resolver: () => ''
          },
          propertyName: {
            virtual: true,
            required: true,
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
                'Callable required properties must have required as a function'
              ])
            })
          )
        }
      })

      it('should reject any non virtual rule', () => {
        const values = DEFINITION_RULES.filter(
          (rule) => !VIRTUAL_RULES.includes(rule)
        )

        for (const rule of values) {
          const toFail = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'propertyName',
              resolver: () => ''
            },
            propertyName: {
              virtual: true,
              [rule]: true,
              validator
            }
          })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              propertyName: expect.arrayContaining([
                `Virtual properties can only have (${VIRTUAL_RULES.join(
                  ', '
                )}) as rules`
              ])
            })
          }
        }
      })
    })
  })
}
