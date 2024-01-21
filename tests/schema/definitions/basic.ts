import { describe, it, expect } from 'vitest';

import { ERRORS } from '../../../dist';
import { expectFailure } from '../_utils';

export const Test_BasicDefinitions = ({ fx, Schema }: any) => {
  describe('Schema definitions', () => {
    it('should reject if property definitions is not an object', () => {
      const values = [
        null,
        undefined,
        new Number(),
        new String(),
        Symbol(),
        2,
        -10,
        true,
        []
      ];

      for (const value of values) expectFailure(fx(value));
    });

    it('should reject if property definitions has no property', () => {
      const toFail = fx({});

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          'schema properties': ['Insufficient Schema properties']
        });
      }
    });

    it("should reject if a property's definition is an empty object", () => {
      const toFail = fx({ emptyProp: {} });
      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err).toEqual(
          expect.objectContaining({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              emptyProp: [
                'A property should at least be readonly, required, or have a default value'
              ]
            }
          })
        );
      }
    });

    it("should reject if a property's definition is not an object", () => {
      const invalidDefinitions = [true, false, [], 1, -1, '', 'invalid'];

      for (const definition of invalidDefinitions) {
        const toFail = fx({ invalidProp0000: definition });
        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err).toEqual(
            expect.objectContaining({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                invalidProp0000: [
                  `Invalid property definition. Expected an object '{}' but received '${typeof definition}'`
                ]
              }
            })
          );
        }
      }
    });

    it("should reject if a property's definition has an invalid rule", () => {
      const toFail = fx({ emptyProp: { default: '', yoo: true } });
      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err).toEqual(
          expect.objectContaining({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              emptyProp: expect.arrayContaining(["'yoo' is not a valid rule"])
            }
          })
        );
      }
    });

    it('should allow access to reservedKeys of valid schemas', () => {
      const schema = new Schema(
        {
          id: { constant: true, value: 1 },
          dependent: { default: '', dependsOn: 'virtual', resolver: () => '' },
          lax: { default: true },
          virtual: { virtual: true, validator: () => true }
        },
        { timestamps: { createdAt: 'c_At' } }
      );

      expect(schema.reservedKeys).toEqual(
        expect.arrayContaining([
          'c_At',
          'dependent',
          'id',
          'lax',
          'updatedAt',
          'virtual'
        ])
      );
    });
  });

  describe('behaviour of schema when errors thrown in setter of default values', () => {
    const Model = new Schema(
      {
        prop: {
          default() {
            throw new Error('lolol');
          }
        },
        prop1: { default: '' }
      },
      { setMissingDefaultsOnUpdate: true }
    ).getModel();

    it('should set value as default on error generating default value at creation', async () => {
      const { data, error } = await Model.create();

      expect(error).toBeNull();
      expect(data).toMatchObject({ prop: null, prop1: '' });
    });

    it("should set value as default on error generating default value during updates'", async () => {
      const { data, error } = await Model.update(
        { prop1: '' },
        { prop1: 'updated' },
        { debug: true }
      );

      expect(error).toBeNull();
      expect(data).toMatchObject({ prop: null, prop1: 'updated' });
    });
  });
};
