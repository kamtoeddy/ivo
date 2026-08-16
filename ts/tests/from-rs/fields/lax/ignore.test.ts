import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.lax.ignore', () => {
  it('should respect the ignore rule', async () => {
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; other: string }>((b) =>
      b
        .field(
          b
            .lax('other', 'default_other_value')
            .validate(() => ({ valid: true })),
        )
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(() => ({ valid: true }))
            .ignore((ctx) => {
              if (ctx.isUpdate)
                return ctx.previousValues.other === 'ignore_lax_for_update';
              return ctx.input.other === 'ignore_lax_for_init';
            }),
        ),
    ).getModel();

    const otherValue = 'ignore_lax_for_init';
    const { data } = await Model.create(
      { lax: 'value to be ignored', other: otherValue },
      {},
    );

    expect(data).toEqual({ lax: defaultLaxValue, other: otherValue });

    const updatedLaxValue = 'updated_lax_value';
    const otherValue1 = 'ignore_lax_for_update';

    const { data: updates } = await Model.update(
      data!,
      { lax: updatedLaxValue, other: otherValue1 },
      {},
    );

    expect(updates).toEqual({ lax: updatedLaxValue, other: otherValue1 });

    const previous = Object.assign({}, data, updates);
    const otherValue2 = 'some other update';

    const { data: updates2 } = await Model.update(
      previous,
      { lax: 'some lax update', other: otherValue2 },
      {},
    );

    expect(updates2).toEqual({ other: otherValue2 });
  });

  it('should respect the ignoreInit rule', async () => {
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; other: string }>((b) =>
      b
        .field(
          b
            .lax('other', 'default_other_value')
            .validate(() => ({ valid: true })),
        )
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(() => ({ valid: true }))
            .ignoreInit(),
        ),
    ).getModel();

    const otherValue = 'some other value';
    const { data } = await Model.create(
      { lax: 'value to be ignored', other: otherValue },
      {},
    );

    expect(data).toEqual({ lax: defaultLaxValue, other: otherValue });

    const updatedLaxValue = 'updated_lax_value';
    const otherValue2 = 'updated_other_value';

    const { data: updates } = await Model.update(
      data!,
      { lax: updatedLaxValue, other: otherValue2 },
      {},
    );

    expect(updates).toEqual({ lax: updatedLaxValue, other: otherValue2 });
  });

  it('should respect the ignoreUpdate rule', async () => {
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; other: string }>((b) =>
      b
        .field(
          b
            .lax('other', 'default_other_value')
            .validate(() => ({ valid: true })),
        )
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(() => ({ valid: true }))
            .ignoreUpdate(),
        ),
    ).getModel();

    const laxValue = 'lax value';
    const otherValue = 'other value';
    const { data } = await Model.create(
      { lax: laxValue, other: otherValue },
      {},
    );

    expect(data).toEqual({ lax: laxValue, other: otherValue });

    const updatedLaxValue = 'lax value to be ignored';
    const otherValue2 = 'updated other value';

    const { data: updates } = await Model.update(
      data!,
      { lax: updatedLaxValue, other: otherValue2 },
      {},
    );

    expect(updates).toEqual({ other: otherValue2 });
  });

  describe('grouped ignore', () => {
    it('should properly handle grouped ignore rule', async () => {
      const IGNORE = 'IGNORE';
      const defaultLaxValue = 'default_lax_value';
      const defaultLax1Value = 'default_lax_1_value';
      const defaultLax2Value = 'default_lax_2_value';

      const Model = new Schema<{ lax: string; lax_1: string; lax_2: string }>(
        (b) =>
          b
            .field(b.lax('lax', defaultLaxValue))
            .field(b.lax('lax_1', defaultLax1Value))
            .field(b.lax('lax_2', defaultLax2Value)),
        {
          ignore: {
            fields: ['lax', 'lax_1'],
            handler: (ctx) => ctx.input.lax === IGNORE,
          },
        },
      ).getModel();

      let lax1 = 'lax_1';
      let lax2 = 'lax_2';

      let { data } = await Model.create(
        { lax: IGNORE, lax_1: lax1, lax_2: lax2 },
        {},
      );

      expect(data).toEqual({
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        lax_2: lax2,
      });

      let lax = 'some lax value';
      lax1 = 'lax_1';
      lax2 = 'lax_2';

      ({ data } = await Model.create({ lax, lax_1: lax1, lax_2: lax2 }, {}));

      expect(data).toEqual({ lax, lax_1: lax1, lax_2: lax2 });

      const previous = {
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        lax_2: defaultLax2Value,
      };

      lax1 = 'lax_1';
      lax2 = 'lax_2';

      let { data: updates } = await Model.update(
        previous,
        { lax: IGNORE, lax_1: lax1, lax_2: lax2 },
        {},
      );

      expect(updates).toEqual({ lax_2: lax2 });

      lax = 'some lax value';
      lax1 = 'lax_1';
      lax2 = 'lax_2';

      ({ data: updates } = await Model.update(
        previous,
        { lax, lax_1: lax1, lax_2: lax2 },
        {},
      ));

      expect(updates).toEqual({ lax, lax_1: lax1, lax_2: lax2 });
    });
  });

  describe('grouped ignoreUpdate', () => {
    it('should properly handle grouped ignoreUpdate rule', async () => {
      const IGNORE = 'IGNORE';
      const defaultLaxValue = 'default_lax_value';
      const defaultLax1Value = 'default_lax_1_value';
      const defaultLax2Value = 'default_lax_2_value';

      const Model = new Schema<{ lax: string; lax_1: string; lax_2: string }>(
        (b) =>
          b
            .field(b.lax('lax', defaultLaxValue))
            .field(b.lax('lax_1', defaultLax1Value))
            .field(b.lax('lax_2', defaultLax2Value)),
        {
          ignoreUpdate: {
            fields: ['lax', 'lax_1'],
            handler: (ctx) => ctx.rawInput.lax === IGNORE,
          },
        },
      ).getModel();

      let lax1 = 'lax_1';
      let lax2 = 'lax_2';

      let { data } = await Model.create(
        { lax: IGNORE, lax_1: lax1, lax_2: lax2 },
        {},
      );

      expect(data).toEqual({ lax: IGNORE, lax_1: lax1, lax_2: lax2 });

      let lax = 'some lax value';
      lax1 = 'lax_1';
      lax2 = 'lax_2';

      ({ data } = await Model.create({ lax, lax_1: lax1, lax_2: lax2 }, {}));

      expect(data).toEqual({ lax, lax_1: lax1, lax_2: lax2 });

      const previous = {
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        lax_2: defaultLax2Value,
      };

      lax1 = 'lax_1';
      lax2 = 'lax_2';

      let { data: updates } = await Model.update(
        previous,
        { lax: IGNORE, lax_1: lax1, lax_2: lax2 },
        {},
      );

      expect(updates).toEqual({ lax_2: lax2 });

      lax = 'some lax value';
      lax1 = 'lax_1';
      lax2 = 'lax_2';

      ({ data: updates } = await Model.update(
        previous,
        { lax, lax_1: lax1, lax_2: lax2 },
        {},
      ));

      expect(updates).toEqual({ lax, lax_1: lax1, lax_2: lax2 });
    });
  });

  describe('readonly', () => {
    it('should ignore updates on readonly fields if values are different from default after creation', async () => {
      const defaultValue = 1;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', defaultValue).readonly()),
      ).getModel();

      const laxValue = 40;
      const { data } = await Model.create({ lax: laxValue }, {});

      expect(data).toEqual({ lax: laxValue });

      const { error } = await Model.update(data!, { lax: 2 }, {});

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });

    it('should ignore updates on readonly fields if values are different from default after updates', async () => {
      const DEFAULT_VALUE = 1;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', DEFAULT_VALUE).readonly()),
      ).getModel();

      const { data } = await Model.create({}, {});

      expect(data).toEqual({ lax: DEFAULT_VALUE });

      const updatedValue = 2;
      const { data: updates } = await Model.update(
        data!,
        { lax: updatedValue },
        {},
      );

      expect(updates).toEqual({ lax: updatedValue });

      const updated = Object.assign({}, data, updates);

      expect(updated).toEqual({ lax: updatedValue });

      const { error } = await Model.update(updated, { lax: 3 }, {});

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });
  });
});
