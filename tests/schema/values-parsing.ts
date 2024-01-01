import { beforeAll, describe, it, expect } from 'vitest';

import { ERRORS } from '../../dist';

export const valuesParsing_Tests = ({ Schema }: any) => {
  const expectPromiseFailure = (
    fx: Function,
    message: string = ERRORS.INVALID_SCHEMA
  ) => {
    expect(fx).rejects.toThrow(message);
  };

  const expectNoFailure = (fx: Function) => expect(fx).not.toThrow();

  describe('Values Parsing', () => {
    const errorMessage = { message: ERRORS.VALIDATION_ERROR, payload: {} };
    const validData = { age: 15, name: 'Frank' };
    const invalidData = [1, -10, 0, false, true, '', 'true', null];

    describe('with silent errors', () => {
      let User: any;

      beforeAll(async () => {
        User = new Schema({
          age: { default: 10 },
          id: { constant: true, value: 1 },
          name: { default: '' }
        }).getModel();
      });

      describe('valid data', () => {
        it('should allow for create method of model to be empty', async () => {
          const { data, error } = await User.create();

          expect(error).toBe(null);

          expect(data).toEqual({ age: 10, id: 1, name: '' });
        });

        it('should set values properly at creation', async () => {
          const { data, error } = await User.create(validData);

          expect(error).toBe(null);

          expect(data).toEqual({ ...validData, id: 1 });
        });

        it('should set values properly during deletion', async () => {
          expectNoFailure(
            async () => await User.delete({ ...validData, id: 1 })
          );
        });

        it('should set values properly during updates', async () => {
          const user = { ...validData, id: 1 };
          const name = 'Mike';

          const { data, error } = await User.update(user, { name });

          expect(error).toBe(null);

          expect(data).toEqual({ name });
        });
      });

      describe('invalid data', () => {
        it('should ignore invalid data at creation', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.create(val);

            expectNoFailure(operation);

            const { data, error } = await operation();

            expect(error).toBe(null);

            expect(data).toEqual({ age: 10, id: 1, name: '' });
          }
        });

        it('should reject invalid data during deletion', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.delete(val);

            expectPromiseFailure(operation, ERRORS.VALIDATION_ERROR);

            try {
              await operation();
            } catch (err: any) {
              expect(err).toMatchObject(errorMessage);
            }
          }
        });

        it('should reject invalid data during updates', async () => {
          for (const val of invalidData) {
            const operation = async () =>
              await User.update(val, { name: 'yoo' });

            expectNoFailure(operation);

            const { data, error } = await operation();

            expect(data).toBe(null);

            expect(error).toEqual({
              message: ERRORS.NOTHING_TO_UPDATE,
              payload: {}
            });
          }
        });
      });
    });

    describe('with thrown errors', () => {
      let User: any;

      beforeAll(async () => {
        User = new Schema(
          {
            age: { default: 10 },
            id: { constant: true, value: 1 },
            name: { default: '' }
          },
          { errors: 'throw' }
        ).getModel();
      });

      describe('valid data', () => {
        it('should set values properly at creation', async () => {
          const { data, error } = await User.create(validData);

          expect(error).toBe(null);

          expect(data).toEqual({ ...validData, id: 1 });
        });

        it('should set values properly during deletion', async () => {
          expectNoFailure(
            async () => await User.delete({ ...validData, id: 1 })
          );
        });

        it('should set values properly during updates', async () => {
          const user = { ...validData, id: 1 };
          const name = 'Mike';

          const { data, error } = await User.update(user, { name });

          expect(error).toBe(null);

          expect(data).toEqual({ name });
        });
      });

      describe('invalid data', () => {
        it('should reject invalid data at creation', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.create(val);

            expectNoFailure(operation);

            const { data, error } = await operation();

            expect(error).toBe(null);
            expect(data).toEqual({ age: 10, id: 1, name: '' });
          }
        });

        it('should reject invalid data during deletion', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.delete(val);

            expectPromiseFailure(operation, ERRORS.VALIDATION_ERROR);

            try {
              await operation();
            } catch (err: any) {
              expect(err).toMatchObject(errorMessage);
            }
          }
        });

        it('should reject invalid data during updates', async () => {
          for (const val of invalidData) {
            const operation = async () =>
              await User.update(val, { name: 'yoo' });

            expectPromiseFailure(operation, ERRORS.NOTHING_TO_UPDATE);

            try {
              await operation();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.NOTHING_TO_UPDATE,
                payload: {}
              });
            }
          }
        });
      });
    });
  });
};
