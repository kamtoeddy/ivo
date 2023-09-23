import { ERRORS } from '../../../..'

export const commonTestData = {
  id: '1',
  name: 'beer',
  price: 5,
  measureUnit: 'bottle',
  _dependentReadOnly: 100,
  _readOnlyLax1: 'lax1 set',
  _readOnlyLaxNoInit: [],
  _readOnlyNoInit: [],
  otherMeasureUnits: [
    { coefficient: 24, name: 'crate24' },
    { coefficient: 5, name: 'tray' },
    { coefficient: 12, name: 'crate' }
  ],
  _quantity: 100
}

export const CommonInheritanceTest = (
  schemaName = '',
  Model: any,
  testData = commonTestData
) => {
  describe(`behaviour shared via inheritance for '${schemaName}'`, () => {
    let item: any

    beforeAll(async () => (item = (await Model.create(testData)).data))

    describe('create', () => {
      it('should create properly with right values', () => {
        expect(item).toMatchObject({
          id: '1',
          name: 'beer',
          price: 5,
          measureUnit: 'bottle',
          otherMeasureUnits: [
            { coefficient: 12, name: 'crate' },
            { coefficient: 24, name: 'crate24' },
            { coefficient: 5, name: 'tray' }
          ],
          quantity: 100
        })
      })

      it('should reject missing readonly field', async () => {
        const { id, ...testData1 } = testData

        const createWithoutReadonly = async () => await Model.create(testData1)

        await expect(createWithoutReadonly()).rejects.toThrow(
          ERRORS.VALIDATION_ERROR
        )
      })

      it('should reject missing required field', async () => {
        const { name, ...testData1 } = testData

        const createWithoutReadonly = async () => await Model.create(testData1)

        await expect(createWithoutReadonly()).rejects.toThrow(
          ERRORS.VALIDATION_ERROR
        )
      })

      it('should reject dependent properties', () => {
        expect(item).toMatchObject({
          _dependentReadOnly: 0
        })
      })

      it('should reject readonly(true) + shouldInit(false)', () => {
        expect(item).toMatchObject({ _readOnlyNoInit: '' })
      })

      it('should accept provided lax readonly properties', () => {
        expect(item).toMatchObject({
          _readOnlyLax1: 'lax1 set',
          _readOnlyLax2: '',
          _readOnlyNoInit: ''
        })
      })
    })

    describe('clone', () => {
      it('should clone properly', async () => {
        const { data: clonedItem } = await Model.clone(item)

        expect(clonedItem).toMatchObject({
          id: '1',
          name: 'beer',
          price: 5,
          measureUnit: 'bottle',
          otherMeasureUnits: [
            { coefficient: 12, name: 'crate' },
            { coefficient: 24, name: 'crate24' },
            { coefficient: 5, name: 'tray' }
          ],
          quantity: 100
        })
      })

      it('should clone properly with virtuals', async () => {
        const { data: clonedItem } = await Model.clone({
          ...item,
          quantities: [
            { quantity: 1, name: 'crate24' },
            { quantity: 1, name: 'tray' }
          ]
        })

        expect(clonedItem).toMatchObject({
          id: '1',
          name: 'beer',
          price: 5,
          measureUnit: 'bottle',
          otherMeasureUnits: [
            { coefficient: 12, name: 'crate' },
            { coefficient: 24, name: 'crate24' },
            { coefficient: 5, name: 'tray' }
          ],
          quantity: 129
        })
      })

      it('should respect clone reset option for property with default value', async () => {
        const { data: clone1 } = await Model.clone(item, { reset: 'quantity' })
        const { data: clone2 } = await Model.clone(item, {
          reset: ['quantity']
        })

        const expectedResult = {
          id: '1',
          name: 'beer',
          price: 5,
          measureUnit: 'bottle',
          otherMeasureUnits: [
            { coefficient: 12, name: 'crate' },
            { coefficient: 24, name: 'crate24' },
            { coefficient: 5, name: 'tray' }
          ],
          quantity: 0
        }

        for (let clonedItem of [clone1, clone2])
          expect(clonedItem).toMatchObject(expectedResult)
      })

      it('should respect clone reset option for property without default value', async () => {
        const { data: clone1 } = await Model.clone(item, {
          reset: 'measureUnit'
        })
        const { data: clone2 } = await Model.clone(item, {
          reset: ['measureUnit']
        })

        const expectedResult = {
          id: '1',
          name: 'beer',
          price: 5,
          measureUnit: 'bottle',
          otherMeasureUnits: [
            { coefficient: 12, name: 'crate' },
            { coefficient: 24, name: 'crate24' },
            { coefficient: 5, name: 'tray' }
          ],
          quantity: 100
        }

        for (let clonedItem of [clone1, clone2])
          expect(clonedItem).toMatchObject(expectedResult)
      })
    })

    describe('update', () => {
      it('should update the relevant properties', async () => {
        const update = await Model.update(item, {
          name: 'Castel',
          _quantity: 10
        })

        expect(update.data).toMatchObject({
          name: 'Castel',
          quantityChangeCounter: 2,
          quantity: 10
        })
      })

      it('should ignore properties that have not changed', () => {
        const toFail = () =>
          Model.update(item, {
            name: 'beer',
            price: 5,
            measureUnit: 'bottle',
            quantity: 100
          })

        expect(toFail).rejects.toThrow(ERRORS.NOTHING_TO_UPDATE)
      })

      it('should update on virtuals', async () => {
        const update = await Model.update(item, {
          quantities: [
            { quantity: 1, name: 'crate24' },
            { name: 'crate', quantity: 2 },
            { name: 'tray', quantity: 5 }
          ]
        })

        expect(update.data).toMatchObject({
          quantityChangeCounter: 2,
          quantity: 173
        })
      })

      it('should update the relevant properties & on virtuals', async () => {
        const update = await Model.update(item, {
          name: 'Castel',
          _quantity: 10,
          quantities: [
            { quantity: 1, name: 'crate24' },
            { name: 'crate', quantity: 2 },
            { name: 'tray', quantity: 5 }
          ]
        })

        expect(update.data).toMatchObject({
          name: 'Castel',
          quantityChangeCounter: 2,
          quantity: 83
        })
      })

      it('should update lax properties not initialized at creation', async () => {
        const { data: update } = await Model.update(item, {
          _readOnlyLax2: 'haha'
        })

        expect(update).toMatchObject({
          _readOnlyLax2: 'haha'
        })

        const updateReadOnlyProperty = async () =>
          await Model.update(
            { ...item, ...update },
            {
              _readOnlyLax2: 'lax1 set again'
            }
          )

        await expect(updateReadOnlyProperty()).rejects.toThrow(
          ERRORS.NOTHING_TO_UPDATE
        )
      })

      it('should not update dependent properties', async () => {
        const updateReadOnlyProperty = async () =>
          await Model.update(item, { quantityChangeCounter: 0 })

        await expect(updateReadOnlyProperty()).rejects.toThrow(
          ERRORS.NOTHING_TO_UPDATE
        )
      })

      it('should update dependent properties on virtuals', async () => {
        const { data: update } = await Model.update(item, {
          _virtualForDependentReadOnly: 'haha'
        })

        expect(update).toMatchObject({
          _dependentReadOnly: 1
        })
      })

      it('should not update readonly dependent properties that have changed', async () => {
        const { data: update } = await Model.update(item, {
          _virtualForDependentReadOnly: 'haha'
        })

        const updateToFail = async () => {
          await Model.update(
            { ...item, ...update },
            {
              _virtualForDependentReadOnly: 'haha'
            }
          )
        }

        await expect(updateToFail()).rejects.toThrow(ERRORS.NOTHING_TO_UPDATE)
      })

      it('should not update readonly properties that have changed', async () => {
        const updateReadOnlyProperty = async () =>
          await Model.update(item, {
            id: '2',
            _readOnlyLax1: 'lax1 set again'
          })

        await expect(updateReadOnlyProperty()).rejects.toThrow(
          ERRORS.NOTHING_TO_UPDATE
        )
      })
    })
  })

  describe(`initialization with virtual props for '${schemaName}'`, () => {
    let item: any

    beforeAll(async () => {
      item = (
        await Model.create({
          ...testData,
          quantities: [
            { name: 'crate24', quantity: 1 },
            { name: 'tray', quantity: 1 }
          ]
        })
      ).data
    })

    // creation
    it('should have been created properly', () => {
      expect(item).toMatchObject({
        id: '1',
        name: 'beer',
        price: 5,
        measureUnit: 'bottle',
        otherMeasureUnits: [
          { coefficient: 12, name: 'crate' },
          { coefficient: 24, name: 'crate24' },
          { coefficient: 5, name: 'tray' }
        ],
        quantity: 129
      })
    })
  })

  describe(`user defined validation errors for '${schemaName}'`, () => {
    it('should respect user defined error messages at creation', () => {
      const failToCreate = async () => {
        try {
          await Model.create({ ...commonTestData, name: '', _laxProp: [] })
        } catch (err: any) {
          expect(err.message).toBe(ERRORS.VALIDATION_ERROR)
          expect(err.payload).toMatchObject({
            _laxProp: {
              errors: expect.arrayContaining(['Invalid lax prop', 'Too short'])
            },
            name: { errors: expect.arrayContaining(['validation failed']) }
          })
        }
      }

      failToCreate()
    })

    it('should respect user defined error messages during cloning', () => {
      const failToClone = async () => {
        try {
          await Model.clone(testData, { reset: ['name', '_laxProp'] })
        } catch (err: any) {
          expect(err.message).toBe(ERRORS.VALIDATION_ERROR)
          expect(err.payload).toMatchObject({
            _laxProp: {
              errors: expect.arrayContaining([
                'Invalid lax prop',
                'Unacceptable value'
              ])
            },
            name: { errors: expect.arrayContaining(['validation failed']) }
          })
        }
      }

      failToClone()
    })

    it('should respect user defined error messages during updates', () => {
      const failToUpdate = async () => {
        try {
          await Model.update(commonTestData, { name: '', _laxProp: [] })
        } catch (err: any) {
          expect(err.message).toBe(ERRORS.VALIDATION_ERROR)
          expect(err.payload).toMatchObject({
            _laxProp: {
              errors: expect.arrayContaining(['Invalid lax prop', 'Too short'])
            },
            name: { errors: expect.arrayContaining(['validation failed']) }
          })
        }
      }

      failToUpdate()
    })
  })
}
