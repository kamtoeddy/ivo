import { beforeAll, describe, expect, it } from 'bun:test';

import { ERRORS } from '../../dist';
import { expectNoFailure } from './_utils';

export const valuesParsing_Tests = ({ Schema }: any) => {
  describe('Values Parsing', () => {
    const validData = { age: 15, name: 'Frank' };
    const invalidData = [1, -10, 0, false, true, '', 'true', null];

    let User: any;

    beforeAll(async () => {
      User = new Schema({
        age: { default: 10 },
        id: { constant: true, value: 1 },
        name: { default: '' },
      }).getModel();
    });

    describe('valid data', () => {
      it('should allow for create method of model to be empty', async () => {
        const { data, error } = await User.create();

        expect(error).toBeNull();

        expect(data).toEqual({ age: 10, id: 1, name: '' });
      });

      it('should set values properly at creation', async () => {
        const { data, error } = await User.create(validData);

        expect(error).toBeNull();

        expect(data).toEqual({ ...validData, id: 1 });
      });

      it('should set values properly during deletion', async () => {
        expectNoFailure(async () => await User.delete({ ...validData, id: 1 }));
      });

      it('should set values properly during updates', async () => {
        const user = { ...validData, id: 1 };
        const name = 'Mike';

        const { data, error } = await User.update(user, { name });

        expect(error).toBeNull();

        expect(data).toEqual({ name });
      });
    });

    describe('invalid data', () => {
      it('should ignore invalid data at creation', async () => {
        for (const val of invalidData) {
          const operation = async () => await User.create(val);

          expectNoFailure(operation);

          const { data, error } = await operation();

          expect(error).toBeNull();

          expect(data).toEqual({ age: 10, id: 1, name: '' });
        }
      });

      it('should reject invalid data during updates', async () => {
        for (const val of invalidData) {
          const operation = async () => await User.update(val, { name: 'yoo' });

          expectNoFailure(operation);

          const { data, error } = await operation();

          expect(data).toBeNull();

          expect(error).toEqual({
            message: ERRORS.NOTHING_TO_UPDATE,
            payload: {},
          });
        }
      });
    });
  });
};
