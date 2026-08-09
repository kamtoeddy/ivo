import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.lax', () => {
  describe('nothing to update', () => {
    it('should reject updates if no value has changed', async () => {
      const defaultValue = 1;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', defaultValue)),
      ).getModel();

      const { error } = await Model.update({ lax: 24 }, { lax: 24 }, {});

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });

    it('should reject updates if no value has changed after validation', async () => {
      const DEFAULT_VALUE = 1;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true, validated: DEFAULT_VALUE })),
        ),
      ).getModel();

      const { error } = await Model.update(
        { lax: DEFAULT_VALUE },
        { lax: 24 },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });

    it('should reject updates if no value has changed after re-validation', async () => {
      const DEFAULT_VALUE = 1;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .reValidate(() => ({ valid: true, validated: DEFAULT_VALUE })),
        ),
      ).getModel();

      const { error } = await Model.update(
        { lax: DEFAULT_VALUE },
        { lax: 24 },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });

    it('should reject updates if no value has changed after post validation', async () => {
      const DEFAULT_VALUE = 'default_value';
      const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR =
        'RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR';
      const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR =
        'RESET_TO_PREV_VALUE_IN_POST_VALIDATOR';

      const Model = new Schema<{ lax: string; lax_1: string }>(
        (b) =>
          b
            .field(
              b
                .lax('lax', DEFAULT_VALUE)
                .validate(() => ({ valid: true }))
                .reValidate(() => ({ valid: true, validated: DEFAULT_VALUE })),
            )
            .field(b.lax('lax_1', DEFAULT_VALUE)),
        {
          postValidate: {
            fields: ['lax', 'lax_1'],
            validator: [
              (ctx) => {
                if (!ctx.isUpdate) return undefined;
                if (ctx.input.lax === RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR)
                  return { lax: { validated: ctx.previousValues.lax } };
                return undefined;
              },
              (ctx) => {
                if (!ctx.isUpdate) return undefined;
                if (ctx.input.lax === RESET_TO_PREV_VALUE_IN_POST_VALIDATOR)
                  return { lax: { validated: ctx.previousValues.lax } };
                return undefined;
              },
            ],
          },
        },
      ).getModel();

      let { error } = await Model.update(
        { lax: DEFAULT_VALUE, lax_1: DEFAULT_VALUE },
        { lax: RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });

      ({ error } = await Model.update(
        { lax: DEFAULT_VALUE, lax_1: DEFAULT_VALUE },
        { lax: RESET_TO_PREV_VALUE_IN_POST_VALIDATOR },
        {},
      ));

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });
  });

  describe('default values & fns', () => {
    it('should properly use default value of missing fields at creation', async () => {
      const defaultValue = 1;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', defaultValue)),
      ).getModel();

      const { data } = await Model.create({}, {});

      expect(data).toEqual({ lax: defaultValue });
    });

    it('should properly resolve default values of missing fields at creation', async () => {
      const DEFAULT_VALUE = 1_000;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', () => DEFAULT_VALUE)),
      ).getModel();

      const { data } = await Model.create({}, {});

      expect(data).toEqual({ lax: DEFAULT_VALUE });
    });

    it('should properly use lax input values as output values if no validator is provided', async () => {
      const DEFAULT_VALUE = 1_000;

      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', () => DEFAULT_VALUE)),
      ).getModel();

      const lax = 34;
      const { data } = await Model.create({ lax }, {});

      expect(data).toEqual({ lax });

      const laxUpdate = 30;
      const { data: updates } = await Model.update(
        data!,
        { lax: laxUpdate },
        {},
      );

      expect(updates).toEqual({ lax: laxUpdate });
    });
  });

  describe('required', () => {
    it('should respect the required rule', async () => {
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
              .required((ctx) => {
                if (ctx.isUpdate) {
                  if (ctx.previousValues.other === 'require_lax_for_update')
                    return [true, 'lax is required for this update'];
                  return false;
                }

                if (ctx.input.other === 'required_lax_for_init')
                  return [true, 'lax is required to create at this time'];

                return false;
              }),
          ),
      ).getModel();

      const { error } = await Model.create(
        { other: 'required_lax_for_init' },
        {},
      );

      expect(error?.lax?.reason).toBe('lax is required to create at this time');

      const otherValue = 'require_lax_for_update';
      const { data } = await Model.create({ other: otherValue }, {});

      expect(data).toEqual({ lax: defaultLaxValue, other: otherValue });

      const { error: updateError } = await Model.update(
        data!,
        { other: 'some update' },
        {},
      );

      expect(updateError?.payload?.lax?.reason).toBe(
        'lax is required for this update',
      );
    });
  });

  describe('grouped required', () => {
    it('should properly handle grouped required errors', async () => {
      const IGNORE_WITH_DIFFERENT_ERRORS = 'IGNORE_WITH_DIFFERENT_ERRORS';
      const IGNORE_WITH_SAME_ERROR = 'IGNORE_WITH_SAME_ERROR';
      const EXPECTED_LAX_OR_LAX_1 = 'EXPECTED_LAX_OR_LAX_1';
      const LAX_IS_MISSING = 'LAX_IS_MISSING';
      const LAX_1_IS_MISSING = 'LAX_1_IS_MISSING';

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
          required: {
            fields: ['lax', 'lax_1'],
            handler: async (ctx) => {
              const lax2 = ctx.input.lax_2;
              if (lax2 == null) return undefined;
              if (lax2 === IGNORE_WITH_SAME_ERROR)
                return {
                  lax: EXPECTED_LAX_OR_LAX_1,
                  lax_1: EXPECTED_LAX_OR_LAX_1,
                };
              return { lax: LAX_IS_MISSING, lax_1: LAX_1_IS_MISSING };
            },
          },
        },
      ).getModel();

      let { error } = await Model.create({ lax_2: IGNORE_WITH_SAME_ERROR }, {});

      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(EXPECTED_LAX_OR_LAX_1);
      expect(error?.lax_1?.reason).toBe(EXPECTED_LAX_OR_LAX_1);

      ({ error } = await Model.create(
        { lax_2: IGNORE_WITH_DIFFERENT_ERRORS },
        {},
      ));

      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(LAX_IS_MISSING);
      expect(error?.lax_1?.reason).toBe(LAX_1_IS_MISSING);

      const data = {
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        lax_2: defaultLax2Value,
      };

      let { error: updateError } = await Model.update(
        data,
        { lax_2: IGNORE_WITH_SAME_ERROR },
        {},
      );

      expect(updateError?.payload?.lax_2).toBeUndefined();
      expect(updateError?.payload?.lax?.reason).toBe(EXPECTED_LAX_OR_LAX_1);
      expect(updateError?.payload?.lax_1?.reason).toBe(EXPECTED_LAX_OR_LAX_1);

      ({ error: updateError } = await Model.update(
        data,
        { lax_2: IGNORE_WITH_DIFFERENT_ERRORS },
        {},
      ));

      expect(updateError?.payload?.lax_2).toBeUndefined();
      expect(updateError?.payload?.lax?.reason).toBe(LAX_IS_MISSING);
      expect(updateError?.payload?.lax_1?.reason).toBe(LAX_1_IS_MISSING);
    });
  });

  describe('validators', () => {
    it('should not create if primary validation fails', async () => {
      const MIN_LENGTH_ERROR = 'expected lax to be at least 2 characters long';

      const Model = new Schema<{ lax: string }>((b) =>
        b.field(
          b.lax('lax', 'default_value').validate((v) => {
            const validated = String(v).trim();
            if (validated.length < 2)
              return { valid: false, reason: MIN_LENGTH_ERROR };
            return { valid: true, validated };
          }),
        ),
      ).getModel();

      for (const laxValue of [' ', ' 1', '1', ' 1   ']) {
        const { error } = await Model.create({ lax: laxValue }, {});
        expect(error?.lax?.reason).toBe(MIN_LENGTH_ERROR);
      }

      for (const laxValue of ['11', '111']) {
        const { data } = await Model.create({ lax: laxValue }, {});
        expect(data?.lax).toBe(laxValue);
      }
    });

    it('should not update if primary validation fails', async () => {
      const LAX_OUT_OF_RANGE_ERROR = 'lax must be between 1 & 5 inclussive';
      const laxRange = [1, 2, 3, 4, 5];

      const Model = new Schema<{ lax: number }, { id: number; lax: number }>(
        (b) =>
          b.field(b.constant('id', () => 1)).field(
            b.lax('lax', 1).validate((v) => {
              if (!laxRange.includes(v as number))
                return { valid: false, reason: LAX_OUT_OF_RANGE_ERROR };
              return { valid: true };
            }),
          ),
      ).getModel();

      const data = { id: 1, lax: 2 };

      for (const laxValue of [-1, 0, 6]) {
        const { error } = await Model.update(data, { lax: laxValue }, {});
        expect(error?.payload?.lax?.reason).toBe(LAX_OUT_OF_RANGE_ERROR);
      }

      for (const updatedValue of laxRange) {
        if (updatedValue === data.lax) continue;

        const { data: updates } = await Model.update(
          data,
          { lax: updatedValue },
          {},
        );

        expect(updates).toEqual({ lax: updatedValue });
      }
    });

    it('should properly use input values as output values if validator does not return a validated value', async () => {
      const Model = new Schema<{ lax: number }>((b) =>
        b.field(b.lax('lax', 1).validate(() => ({ valid: true }))),
      ).getModel();

      const value = 1;
      const { data } = await Model.create({ lax: value }, {});

      expect(data).toEqual({ lax: value });

      const value2 = 2;
      const { data: updates } = await Model.update(
        { lax: value2 - 1 },
        { lax: value2 },
        {},
      );

      expect(updates).toEqual({ lax: value2 });
    });
  });

  describe('re-validators', () => {
    it('should not create if re-validation fails', async () => {
      const MIN_LENGTH_ERROR = 'expected lax to be at least 2 characters long';
      const MIN_REVALIDATION_LENGTH_ERROR =
        'expected lax to be at least 4 characters long';

      const Model = new Schema<{ lax: string }>((b) =>
        b.field(
          b
            .lax('lax', 'default_value')
            .validate((v) => {
              const validated = String(v).trim();
              if (validated.length < 2)
                return { valid: false, reason: MIN_LENGTH_ERROR };
              return { valid: true, validated };
            })
            .reValidate((v) => {
              const validated = v.trim();
              if (validated.length < 4)
                return {
                  valid: false,
                  reason: MIN_REVALIDATION_LENGTH_ERROR,
                };
              return { valid: true, validated };
            }),
        ),
      ).getModel();

      for (const laxValue of [' 111', ' 11 ', '11', ' 112   ']) {
        const { error } = await Model.create({ lax: laxValue }, {});
        expect(error?.lax?.reason).toBe(MIN_REVALIDATION_LENGTH_ERROR);
      }

      for (const laxValue of ['1111', '11111']) {
        const { data } = await Model.create({ lax: laxValue }, {});
        expect(data?.lax).toBe(laxValue);
      }
    });

    it('should not update if re-validation fails', async () => {
      const LAX_OUT_OF_RANGE_ERROR = 'lax must be between 1 & 50 inclussive';
      const REVALIDATED_LAX_OUT_OF_RANGE_ERROR =
        'revalidated lax must be between 10 & 35 inclussive';
      const revalidatedRange = { min: 10, max: 35 };

      const Model = new Schema<{ lax: number }, { id: number; lax: number }>(
        (b) =>
          b.field(b.constant('id', () => 1)).field(
            b
              .lax('lax', 1)
              .validate((v) => {
                if ((v as number) < 1 || (v as number) > 50)
                  return { valid: false, reason: LAX_OUT_OF_RANGE_ERROR };
                return { valid: true };
              })
              .reValidate((v) => {
                if (v < revalidatedRange.min || v > revalidatedRange.max)
                  return {
                    valid: false,
                    reason: REVALIDATED_LAX_OUT_OF_RANGE_ERROR,
                  };
                return { valid: true };
              }),
          ),
      ).getModel();

      const data = { id: 1, lax: 20 };

      for (const laxValue of [
        revalidatedRange.min - 1,
        revalidatedRange.max + 1,
      ]) {
        const { error } = await Model.update(data, { lax: laxValue }, {});
        expect(error?.payload?.lax?.reason).toBe(
          REVALIDATED_LAX_OUT_OF_RANGE_ERROR,
        );
      }

      for (
        let updatedValue = revalidatedRange.min;
        updatedValue <= revalidatedRange.max;
        updatedValue++
      ) {
        if (updatedValue === data.lax) continue;

        const { data: updates } = await Model.update(
          data,
          { lax: updatedValue },
          {},
        );

        expect(updates).toEqual({ lax: updatedValue });
      }
    });

    it('should properly use re-validated values', async () => {
      const Model = new Schema<{ lax: number }>((b) =>
        b.field(
          b
            .lax('lax', 0)
            .validate(() => ({ valid: true }))
            .reValidate((v) => ({ valid: true, validated: v + 1 })),
        ),
      ).getModel();

      const value = 1;
      const { data } = await Model.create({ lax: value }, {});

      expect(data).toEqual({ lax: value + 1 });

      const value2 = 2;
      const { data: updates } = await Model.update(
        { lax: value2 - 1 },
        { lax: value2 },
        {},
      );

      expect(updates).toEqual({ lax: value2 + 1 });
    });

    it('should properly use input values as output values if re-validator does not return a validated value', async () => {
      const Model = new Schema<{ lax: number }>((b) =>
        b.field(
          b
            .lax('lax', 1)
            .validate((v) => ({ valid: true, validated: (v as number) + 1 }))
            .reValidate(() => ({ valid: true })),
        ),
      ).getModel();

      const value = 1;
      const { data } = await Model.create({ lax: value }, {});

      expect(data).toEqual({ lax: value + 1 });

      const value2 = 2;
      const { data: updates } = await Model.update(
        { lax: value2 - 1 },
        { lax: value2 },
        {},
      );

      expect(updates).toEqual({ lax: value2 + 1 });
    });
  });

  describe('post-validation', () => {
    it('should respect post validation config', async () => {
      const LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS =
        'lax failed pre-validation with unrelated errors';
      const LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS =
        'lax failed post-validation with unrelated errors';
      const LAX_1_PRE_VALIDATION_FAIL = 'lax 1 failed pre-validation';
      const BOTH_PRE_VALIDATION_FAIL = 'both failed pre-validation';
      const LAX_VALIDATION_FAIL = 'lax failed post-validatrion';
      const BOTH_VALIDATION_FAIL = 'both failed post-validatrion';

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
          postValidate: {
            fields: ['lax', 'lax_1'],
            validator: [
              (ctx) => {
                const lax = ctx.input.lax;
                if (lax === LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS)
                  return {
                    lax: LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                    lax_2: LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                  };
                if (lax === BOTH_PRE_VALIDATION_FAIL)
                  return {
                    lax: BOTH_PRE_VALIDATION_FAIL,
                    lax_1: BOTH_PRE_VALIDATION_FAIL,
                  };
                if (ctx.values.lax_1 === LAX_1_PRE_VALIDATION_FAIL)
                  return { lax_1: LAX_1_PRE_VALIDATION_FAIL };
                return undefined;
              },
              (ctx) => {
                const lax = ctx.input.lax;
                if (lax === LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS)
                  return {
                    lax: LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                    lax_2: LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                  };
                if (lax === LAX_VALIDATION_FAIL)
                  return { lax: LAX_VALIDATION_FAIL };
                if (lax === BOTH_VALIDATION_FAIL)
                  return {
                    lax: BOTH_VALIDATION_FAIL,
                    lax_1: BOTH_VALIDATION_FAIL,
                  };
                return undefined;
              },
            ],
          },
        },
      ).getModel();

      const lax2 = 'lax_2_provided';
      const { data } = await Model.create({ lax_2: lax2 }, {});

      expect(data).toEqual({
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        lax_2: lax2,
      });

      let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
      let { error } = await Model.create({ lax }, {});

      expect(error?.lax_1).toBeUndefined();
      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(lax);

      lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
      ({ error } = await Model.create({ lax }, {}));

      expect(error?.lax_1).toBeUndefined();
      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(lax);

      const lax1 = LAX_1_PRE_VALIDATION_FAIL;
      ({ error } = await Model.create({ lax_1: lax1 }, {}));

      expect(error?.lax).toBeUndefined();
      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax_1?.reason).toBe(lax1);

      lax = BOTH_PRE_VALIDATION_FAIL;
      ({ error } = await Model.create({ lax }, {}));

      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(lax);
      expect(error?.lax_1?.reason).toBe(lax);

      lax = LAX_VALIDATION_FAIL;
      ({ error } = await Model.create({ lax, lax_2: 'lax_2_provided' }, {}));

      expect(error?.lax_1).toBeUndefined();
      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(lax);

      lax = BOTH_VALIDATION_FAIL;
      ({ error } = await Model.create({ lax, lax_2: 'lax_2_provided' }, {}));

      expect(error?.lax_2).toBeUndefined();
      expect(error?.lax?.reason).toBe(lax);
      expect(error?.lax_1?.reason).toBe(lax);

      // updates

      const data1 = {
        lax: defaultLaxValue,
        lax_1: LAX_1_PRE_VALIDATION_FAIL,
        lax_2: defaultLax2Value,
      };

      let { error: updateError } = await Model.update(
        data1,
        { lax: 'lol' },
        {},
      );

      expect(updateError?.payload?.lax).toBeUndefined();
      expect(updateError?.payload?.lax_2).toBeUndefined();
      expect(updateError?.payload?.lax_1?.reason).toBe(
        LAX_1_PRE_VALIDATION_FAIL,
      );

      lax = BOTH_PRE_VALIDATION_FAIL;
      ({ error: updateError } = await Model.update(data1, { lax }, {}));

      expect(updateError?.payload?.lax_2).toBeUndefined();
      expect(updateError?.payload?.lax?.reason).toBe(lax);
      expect(updateError?.payload?.lax_1?.reason).toBe(lax);

      lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
      ({ error: updateError } = await Model.update(data1, { lax }, {}));

      expect(updateError?.payload?.lax_1).toBeUndefined();
      expect(updateError?.payload?.lax_2).toBeUndefined();
      expect(updateError?.payload?.lax?.reason).toBe(lax);

      const data2 = { ...data1, lax_1: defaultLax1Value };
      lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;

      ({ error: updateError } = await Model.update(data2, { lax }, {}));

      expect(updateError?.payload?.lax_1).toBeUndefined();
      expect(updateError?.payload?.lax_2).toBeUndefined();
      expect(updateError?.payload?.lax?.reason).toBe(lax);
    });

    it('should respect updated values returned from pre-validator in post-validation config', async () => {
      const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES =
        'LAX_PRE_VALIDATED_WITH_UPDATED_VALUES';
      const LAX_POST_VALIDATED_WITH_UPDATED_VALUES =
        'LAX_POST_VALIDATED_WITH_UPDATED_VALUES';
      const UPDATED_VALUE_FROM_PRE_VALIDATOR =
        'UPDATED_VALUE_FROM_PRE_VALIDATOR';
      const UPDATED_VALUE_FROM_POST_VALIDATOR =
        'UPDATED_VALUE_FROM_POST_VALIDATOR';

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
          postValidate: {
            fields: ['lax', 'lax_1'],
            validator: [
              (ctx) => {
                if (ctx.input.lax === LAX_PRE_VALIDATED_WITH_UPDATED_VALUES)
                  return {
                    lax: { validated: UPDATED_VALUE_FROM_PRE_VALIDATOR },
                    lax_1: { validated: UPDATED_VALUE_FROM_PRE_VALIDATOR },
                  };
                return undefined;
              },
              (ctx) => {
                if (ctx.input.lax === LAX_POST_VALIDATED_WITH_UPDATED_VALUES)
                  return {
                    lax: { validated: UPDATED_VALUE_FROM_POST_VALIDATOR },
                    lax_1: { validated: UPDATED_VALUE_FROM_POST_VALIDATOR },
                  };
                return undefined;
              },
            ],
          },
        },
      ).getModel();

      let { data } = await Model.create(
        { lax: LAX_PRE_VALIDATED_WITH_UPDATED_VALUES },
        {},
      );

      expect(data).toEqual({
        lax: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        lax_1: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        lax_2: defaultLax2Value,
      });

      ({ data } = await Model.create(
        { lax: LAX_POST_VALIDATED_WITH_UPDATED_VALUES },
        {},
      ));

      expect(data).toEqual({
        lax: UPDATED_VALUE_FROM_POST_VALIDATOR,
        lax_1: UPDATED_VALUE_FROM_POST_VALIDATOR,
        lax_2: defaultLax2Value,
      });

      const previous = {
        lax: defaultLaxValue,
        lax_1: defaultLax1Value,
        lax_2: defaultLax2Value,
      };

      let { data: updates } = await Model.update(
        previous,
        { lax: LAX_PRE_VALIDATED_WITH_UPDATED_VALUES },
        {},
      );

      expect(updates).toEqual({
        lax: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        lax_1: UPDATED_VALUE_FROM_PRE_VALIDATOR,
      });

      ({ data: updates } = await Model.update(
        previous,
        { lax: LAX_POST_VALIDATED_WITH_UPDATED_VALUES },
        {},
      ));

      expect(updates).toEqual({
        lax: UPDATED_VALUE_FROM_POST_VALIDATOR,
        lax_1: UPDATED_VALUE_FROM_POST_VALIDATOR,
      });
    });
  });
});
