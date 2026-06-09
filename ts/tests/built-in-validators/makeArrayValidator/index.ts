import { describe, expect, it } from 'bun:test';
import { expectFailure } from '../../schema/_utils';

const people = [
  { name: 'James', bio: { displayName: 'james' }, age: 31 },
  { name: 'Mary', bio: { displayName: 'mary' }, age: 20 },
  { name: 'Paul', bio: { displayName: 'paul' }, age: 20 },
  { name: 'James', bio: { displayName: 'james_1' }, age: 20 },
];

export const makeArrayValidatorTest = ({
  makeArrayValidator,
}: {
  makeArrayValidator: Function;
}) => {
  describe('makeArrayValidator', () => {
    it('should throw an error if a filter function is not provided', async () => {
      expectFailure(
        () => makeArrayValidator({})([1, {}, '', false, '[]']),
        'Array validator must have a filter function',
      );
    });

    it('should reject non-array values', async () => {
      const values = [1, {}, '', false, '[]'];

      for (const value of values) {
        const result = await makeArrayValidator({ filter: () => false })(value);

        expect(result.validated).toBeUndefined();
        expect(result).toMatchObject({
          reason: 'Expected an array',
          valid: false,
        });
      }
    });

    it('not should reject an empty array', async () => {
      const res = await makeArrayValidator({ filter: () => false })([]);

      expect(res).toMatchObject({ valid: true, validated: [] });
    });

    it('should allow items within min max limits', async () => {
      const values = [[1], [1, 2], [1, 2, 3]];

      for (const data of values) {
        const res = await makeArrayValidator({
          filter: () => true,
          min: 1,
          max: 3,
        })(data);

        expect(res).toMatchObject({ valid: true, validated: data });
      }
    });

    it('should if items are more than max length', async () => {
      const res = await makeArrayValidator({ filter: () => true, max: 1 })([
        1, 2,
      ]);

      expect(res).toMatchObject({
        valid: false,
        reason: 'Max limit reached',
        metadata: { max: 1 },
      });
    });

    it('should respect custom max errors', async () => {
      const res = await makeArrayValidator({
        filter: () => true,
        max: { value: 1, error: 'lol' },
      })([1, 2]);

      expect(res).toMatchObject({
        valid: false,
        reason: 'lol',
        metadata: { max: 1 },
      });
    });

    it('should if items are less than min length', async () => {
      const res = await makeArrayValidator({ filter: () => true, min: 1 })([]);

      expect(res).toMatchObject({
        valid: false,
        reason: 'Expected a non-empty array',
        metadata: { min: 1 },
      });
    });

    it('should respect custom min errors', async () => {
      const res = await makeArrayValidator({
        filter: () => true,
        min: { value: 1, error: 'min not reached' },
      })([]);

      expect(res).toMatchObject({
        valid: false,
        reason: 'min not reached',
        metadata: { min: 1 },
      });
    });

    it('should sort resulting array if option is specified', async () => {
      const values = [1, 7, 0, 18, -20];
      const res = await makeArrayValidator({ filter: () => true, sort: true })(
        values,
      );

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({
        valid: true,
        validated: [-20, 0, 1, 7, 18],
      });

      const res1 = await makeArrayValidator({
        filter: () => true,
        sort: true,
        sortOrder: 'desc',
      })(values);

      expect(res1.reason).toBeUndefined();
      expect(res1).toMatchObject({
        valid: true,
        validated: res.validated.reverse(),
      });
    });

    it("should sort 'asc' if invalid sortOrder is provided", async () => {
      const values = [1, 0, 18, 7, -20];
      const sorted = [-20, 0, 1, 7, 18];
      const res = await makeArrayValidator({
        filter: () => true,
        sort: true,
        sortedOrder: 'yoo',
      })(values);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: sorted });
    });

    it('should respect filter', async () => {
      const filter = (v: any) => !isNaN(v);

      const values = [1, 7, 18, -20];

      const res = await makeArrayValidator({ filter })(values);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: values });

      const res1 = await makeArrayValidator({
        filter,
        min: 1,
      })(['1-1', '.', undefined]);

      expect(res1.validated).toBeUndefined();
      expect(res1).toMatchObject({
        reason: 'Expected a non-empty array',
        valid: false,
      });
    });

    it('should respect modifier', async () => {
      const modifier = Number;

      const res = await makeArrayValidator({ filter: () => true, modifier })([
        1,
        7,
        '18',
        -20,
        null,
      ]);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: [1, 7, 18, -20, 0] });
    });

    it('should respect unique option', async () => {
      const res = await makeArrayValidator({
        filter: () => true,
        unique: true,
      })([1, 7, '18', '18', -20, null, 1]);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({
        valid: true,
        validated: [1, 7, '18', -20, null],
      });
    });

    it('should respect unique option with unique key', async () => {
      const res = await makeArrayValidator({
        filter: () => true,
        unique: true,
        uniqueKey: 'name',
      })(people);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true });
      expect(res.validated.length).toBe(3);
    });

    it('should respect unique option with unique key (nested)', async () => {
      const res = await makeArrayValidator({
        filter: () => true,
        unique: true,
        uniqueKey: 'bio.displayName',
      })(people);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true });
      expect(res.validated.length).toBe(4);
    });

    it('should respect filter, modifier & sorter', async () => {
      const res = await makeArrayValidator({
        filter: (v: any) => !isNaN(v),
        modifier: Number,
        sort: (a: number, b: number) => (a < b ? -1 : 1),
      })([1, 7, '18', -20, null]);

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: [-20, 0, 1, 7, 18] });
    });
  });
};
