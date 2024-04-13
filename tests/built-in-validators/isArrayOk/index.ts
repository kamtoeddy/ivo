import { describe, it, expect } from 'bun:test';

const people = [
  { name: 'James', bio: { displayName: 'james' }, age: 31 },
  { name: 'Mary', bio: { displayName: 'mary' }, age: 20 },
  { name: 'Paul', bio: { displayName: 'paul' }, age: 20 },
  { name: 'James', bio: { displayName: 'james_1' }, age: 20 },
];

export const isArrayOkTest = ({ isArrayOk }: { isArrayOk: Function }) => {
  describe('isArrayOk', () => {
    it('should reject non-array values', async () => {
      const values = [1, {}, '', false, '[]'];

      for (const value of values) {
        const result = await isArrayOk(value);

        expect(result.validated).toBeUndefined();
        expect(result).toMatchObject({
          reason: ['Expected an array'],
          valid: false,
        });
      }
    });

    it('should reject an empty array', async () => {
      const res = await isArrayOk([]);

      expect(res.validated).toBeUndefined();
      expect(res).toMatchObject({
        reason: ['Expected a non-empty array'],
        valid: false,
      });
    });

    it('should accept an empty array if option is specified', async () => {
      const res = await isArrayOk([], { empty: true });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: [] });
    });

    it('should sort resulting array if option is specified', async () => {
      const values = [1, 7, 0, 18, -20];
      const res = await isArrayOk(values, { sorted: true });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({
        valid: true,
        validated: [-20, 0, 1, 7, 18],
      });

      const res1 = await isArrayOk(values, {
        sorted: true,
        sortOrder: 'desc',
      });

      expect(res1.reason).toBeUndefined();
      expect(res1).toMatchObject({
        valid: true,
        validated: res.validated.reverse(),
      });
    });

    it("should sort 'asc' if invalid sortOrder is provided", async () => {
      const values = [1, 0, 18, 7, -20];
      const sorted = [-20, 0, 1, 7, 18];
      const res = await isArrayOk(values, { sorted: true, sortedOrder: 'yoo' });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: sorted });
    });

    it('should respect filter', async () => {
      const filter = (v: any) => (isNaN(v) ? false : true);

      const res = await isArrayOk([1, 7, 18, -20], { filter });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: [1, 7, 18, -20] });

      const res1 = await isArrayOk(['1-1', '.', undefined], { filter });

      expect(res1.validated).toBeUndefined();
      expect(res1).toMatchObject({
        reason: ['Expected a non-empty array'],
        valid: false,
      });
    });

    it('should respect modifier', async () => {
      const modifier = Number;

      const res = await isArrayOk([1, 7, '18', -20, null], { modifier });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: [1, 7, 18, -20, 0] });
    });

    it('should respect unique option', async () => {
      const res = await isArrayOk([1, 7, '18', '18', -20, null, 1], {
        unique: true,
      });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({
        valid: true,
        validated: [1, 7, '18', -20, null],
      });
    });

    it('should respect unique option with unique key', async () => {
      const res = await isArrayOk(people, { unique: true, uniqueKey: 'name' });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true });
      expect(res.validated.length).toBe(3);
    });

    it('should respect unique option with unique key (nested)', async () => {
      const res = await isArrayOk(people, {
        unique: true,
        uniqueKey: 'bio.displayName',
      });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true });
      expect(res.validated.length).toBe(4);
    });

    it('should respect filter, modifier & sorter', async () => {
      const res = await isArrayOk([1, 7, '18', -20, null], {
        filter: (v: any) => (isNaN(v) ? false : true),
        modifier: Number,
        sorter: (a: number, b: number) => (a < b ? -1 : 1),
      });

      expect(res.reason).toBeUndefined();
      expect(res).toMatchObject({ valid: true, validated: [-20, 0, 1, 7, 18] });
    });
  });
};
