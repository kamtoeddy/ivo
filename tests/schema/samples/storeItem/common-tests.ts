import { beforeAll, describe, it, expect } from 'bun:test';

import { ERRORS } from '../../../../dist';

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
    { coefficient: 12, name: 'crate' },
  ],
  _quantity: 100,
};

export const CommonInheritanceTest = (
  schemaName = '',
  Model: any,
  testData = commonTestData,
) => {
  describe(`behaviour shared via inheritance for '${schemaName}'`, () => {
    let item: any;

    beforeAll(async () => (item = (await Model.create(testData)).data));

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
            { coefficient: 5, name: 'tray' },
          ],
          quantity: 100,
        });
      });

      it('should reject missing readonly field', async () => {
        const { id, ...testData1 } = testData,
          { data, error } = await Model.create(testData1);

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.VALIDATION_ERROR });
      });

      it('should reject missing required field', async () => {
        const { name, ...testData1 } = testData,
          { data, error } = await Model.create(testData1);

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.VALIDATION_ERROR });
      });

      it('should reject dependent properties', () => {
        expect(item).toMatchObject({
          _dependentReadOnly: 0,
        });
      });

      it('should reject readonly(true) + shouldInit(false)', () => {
        expect(item).toMatchObject({ _readOnlyNoInit: '' });
      });

      it('should accept provided lax readonly properties', () => {
        expect(item).toMatchObject({
          _readOnlyLax1: 'lax1 set',
          _readOnlyLax2: '',
          _readOnlyNoInit: '',
        });
      });
    });

    describe('update', () => {
      it('should update the relevant properties', async () => {
        const update = await Model.update(item, {
          name: 'Castel',
          _quantity: 10,
        });

        expect(update.data).toMatchObject({
          name: 'Castel',
          quantityChangeCounter: 2,
          quantity: 10,
        });
      });

      it('should ignore properties that have not changed', async () => {
        const { data, error } = await Model.update(item, {
          name: 'beer',
          price: 5,
          measureUnit: 'bottle',
          quantity: 100,
        });

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.NOTHING_TO_UPDATE });
      });

      it('should update on virtuals', async () => {
        const update = await Model.update(item, {
          quantities: [
            { quantity: 1, name: 'crate24' },
            { name: 'crate', quantity: 2 },
            { name: 'tray', quantity: 5 },
          ],
        });

        expect(update.data).toMatchObject({
          quantityChangeCounter: 2,
          quantity: 173,
        });
      });

      it('should update the relevant properties & on virtuals', async () => {
        const update = await Model.update(item, {
          name: 'Castel',
          _quantity: 10,
          quantities: [
            { quantity: 1, name: 'crate24' },
            { name: 'crate', quantity: 2 },
            { name: 'tray', quantity: 5 },
          ],
        });

        expect(update.data).toMatchObject({
          name: 'Castel',
          quantityChangeCounter: 2,
          quantity: 83,
        });
      });

      it('should update lax properties not initialized at creation', async () => {
        const { data: update } = await Model.update(item, {
          _readOnlyLax2: 'haha',
        });

        expect(update).toMatchObject({ _readOnlyLax2: 'haha' });

        const { data, error } = await Model.update(
          { ...item, ...update },
          { _readOnlyLax2: 'lax1 set again' },
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.NOTHING_TO_UPDATE });
      });

      it('should not update dependent properties', async () => {
        const { data, error } = await Model.update(item, {
          quantityChangeCounter: 0,
        });

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.NOTHING_TO_UPDATE });
      });

      it('should update dependent properties on virtuals', async () => {
        const { data: update } = await Model.update(item, {
          _virtualForDependentReadOnly: 'haha',
        });

        expect(update).toMatchObject({ _dependentReadOnly: 1 });
      });

      it('should not update readonly dependent properties that have changed', async () => {
        const { data: update } = await Model.update(item, {
          _virtualForDependentReadOnly: 'haha',
        });

        const { data, error } = await Model.update(
          { ...item, ...update },
          {
            _virtualForDependentReadOnly: 'haha',
          },
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.NOTHING_TO_UPDATE });
      });

      it('should not update readonly properties that have changed', async () => {
        const { data, error } = await Model.update(item, {
          id: '2',
          _readOnlyLax1: 'lax1 set again',
        });

        expect(data).toBeNull();
        expect(error).toMatchObject({ message: ERRORS.NOTHING_TO_UPDATE });
      });
    });
  });

  describe(`initialization with virtual props for '${schemaName}'`, () => {
    let item: any;

    beforeAll(async () => {
      item = (
        await Model.create({
          ...testData,
          quantities: [
            { name: 'crate24', quantity: 1 },
            { name: 'tray', quantity: 1 },
          ],
        })
      ).data;
    });

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
          { coefficient: 5, name: 'tray' },
        ],
        quantity: 129,
      });
    });
  });

  describe(`user defined validation errors for '${schemaName}'`, () => {
    it('should respect user defined error messages at creation', async () => {
      const { data, error } = await Model.create({
        ...commonTestData,
        name: '',
        _laxProp: [],
      });

      expect(data).toBeNull();
      expect(error.message).toBe(ERRORS.VALIDATION_ERROR);
      expect(error.payload).toMatchObject({
        _laxProp: {
          reasons: expect.arrayContaining([
            'Invalid lax prop',
            'Expected a string',
          ]),
        },
        name: expect.objectContaining({
          reasons: expect.arrayContaining(['too_short']),
        }),
      });
    });

    it('should respect user defined error messages during updates', async () => {
      const { data, error } = await Model.update(commonTestData, {
        name: '',
        _laxProp: [],
      });

      expect(data).toBeNull();
      expect(error.message).toBe(ERRORS.VALIDATION_ERROR);
      expect(error.payload).toMatchObject({
        _laxProp: {
          reasons: expect.arrayContaining([
            'Invalid lax prop',
            'Expected a string',
          ]),
        },
        name: expect.objectContaining({
          reasons: expect.arrayContaining(['too_short']),
        }),
      });
    });
  });
};
