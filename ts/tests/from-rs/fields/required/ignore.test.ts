import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.required.ignore', () => {
  it('should respect the ignoreUpdate rule', async () => {
    const IGNORE_REQUIRED_FOR_UPDATE = 'ignore_required_for_update';

    const Model = new Schema<{ lax: string; required: number }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(() => ({ valid: true }))
            .ignoreUpdate(
              (ctx) => ctx.previousValues.lax === IGNORE_REQUIRED_FOR_UPDATE,
            ),
        )
        .field(b.lax('lax', 'default_lax_value')),
    ).getModel();

    const lax = IGNORE_REQUIRED_FOR_UPDATE;
    const required = 1;

    const { data } = await Model.create({ lax, required }, {});

    expect(data).toEqual({ lax, required });

    const required2 = required + 2;
    const { error } = await Model.update(data!, { required: required2 }, {});

    expect(error).toEqual({ isNothingToUpdate: true, payload: null });

    const previous = { ...data!, lax: 'normal_lax_value' };

    const { data: updates } = await Model.update(
      previous,
      { required: required2 },
      {},
    );

    expect(updates).toEqual({ required: required2 });
  });

  it('should respect the readonly rule', async () => {
    const IGNORE_REQUIRED_FOR_UPDATE = 'ignore_required_for_update';

    const Model = new Schema<{ lax: string; required: number }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(() => ({ valid: true }))
            .readonly(),
        )
        .field(b.lax('lax', 'default_lax_value')),
    ).getModel();

    const lax = IGNORE_REQUIRED_FOR_UPDATE;
    const required = 1;

    const { data } = await Model.create({ lax, required }, {});

    expect(data).toEqual({ lax, required });

    const required2 = required + 2;
    let { error } = await Model.update(data!, { required: required2 }, {});

    expect(error).toEqual({ isNothingToUpdate: true, payload: null });

    const previous = { ...data!, lax: 'normal_lax_value' };

    ({ error } = await Model.update(previous, { required: required2 }, {}));

    expect(error).toEqual({ isNothingToUpdate: true, payload: null });
  });

  describe('grouped ignoreUpdate', () => {
    it('should properly handle grouped ignoreUpdate rule', async () => {
      const IGNORE = 'IGNORE';
      const defaultLaxValue = 'default_lax_value';
      const defaultLax1Value = 'default_lax_1_value';

      const Model = new Schema<{
        lax: string;
        lax_1: string;
        required: string;
      }>(
        (b) =>
          b
            .field(b.lax('lax', defaultLaxValue))
            .field(b.lax('lax_1', defaultLax1Value))
            .field(b.required('required').validate(() => ({ valid: true }))),
        {
          ignoreUpdate: {
            fields: ['lax', 'required'],
            resolver: (ctx) => ctx.rawInput.lax === IGNORE,
          },
        },
      ).getModel();

      let lax = IGNORE;
      let lax1 = 'lax_1';
      let required = 'some value';

      let { data } = await Model.create({ lax, lax_1: lax1, required }, {});

      expect(data).toEqual({ lax, lax_1: lax1, required });

      lax = 'some lax value';
      lax1 = 'lax_1';
      required = 'some value';

      ({ data } = await Model.create({ lax, lax_1: lax1, required }, {}));

      expect(data).toEqual({ lax, lax_1: lax1, required });

      const previous = {
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        required: 'some value',
      };

      lax1 = 'lax_1';
      let updatedRequired = 'updated value';

      let { data: updates } = await Model.update(
        previous,
        { lax: IGNORE, lax_1: lax1, required: updatedRequired },
        {},
      );

      expect(updates).toEqual({ lax_1: lax1 });

      lax = 'some lax value';
      lax1 = 'lax_1';
      updatedRequired = 'updated value';

      ({ data: updates } = await Model.update(
        previous,
        { lax, lax_1: lax1, required: updatedRequired },
        {},
      ));

      expect(updates).toEqual({ lax, lax_1: lax1, required: updatedRequired });
    });
  });
});
