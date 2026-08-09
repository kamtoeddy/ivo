import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.required', () => {
  describe('nothing to update', () => {
    it('should reject updates if no value has changed', async () => {
      const Model = new Schema<{ required: number }>((b) =>
        b.field(b.required('required').validate(() => ({ valid: true }))),
      ).getModel();

      const { error } = await Model.update(
        { required: 24 },
        { required: 24 },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });

    it('should reject updates if no value has changed after validation', async () => {
      const DEFAULT_VALUE = 1;

      const Model = new Schema<{ required: number }>((b) =>
        b.field(
          b
            .lax('required', DEFAULT_VALUE)
            .validate(() => ({ valid: true, validated: DEFAULT_VALUE })),
        ),
      ).getModel();

      const { error } = await Model.update(
        { required: DEFAULT_VALUE },
        { required: 24 },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });

    it('should reject updates if no value has changed after re-validation', async () => {
      const DEFAULT_VALUE = 1;

      const Model = new Schema<{ required: number }>((b) =>
        b.field(
          b
            .lax('required', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .reValidate(() => ({ valid: true, validated: DEFAULT_VALUE })),
        ),
      ).getModel();

      const { error } = await Model.update(
        { required: DEFAULT_VALUE },
        { required: 24 },
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

      const Model = new Schema<{ required: string; lax_1: string }>(
        (b) =>
          b
            .field(
              b
                .required('required')
                .validate(() => ({ valid: true }))
                .reValidate(() => ({ valid: true, validated: DEFAULT_VALUE })),
            )
            .field(b.lax('lax_1', DEFAULT_VALUE)),
        {
          postValidate: {
            fields: ['required', 'lax_1'],
            validator: [
              (ctx) => {
                if (!ctx.isUpdate) return undefined;
                if (ctx.input.required === RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR)
                  return {
                    required: { validated: ctx.previousValues.required },
                  };
                return undefined;
              },
              (ctx) => {
                if (!ctx.isUpdate) return undefined;
                if (
                  ctx.input.required === RESET_TO_PREV_VALUE_IN_POST_VALIDATOR
                )
                  return {
                    required: { validated: ctx.previousValues.required },
                  };
                return undefined;
              },
            ],
          },
        },
      ).getModel();

      let { error } = await Model.update(
        { required: DEFAULT_VALUE, lax_1: DEFAULT_VALUE },
        { required: RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });

      ({ error } = await Model.update(
        { required: DEFAULT_VALUE, lax_1: DEFAULT_VALUE },
        { required: RESET_TO_PREV_VALUE_IN_POST_VALIDATOR },
        {},
      ));

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    });
  });

  describe('requiredError', () => {
    it('should respect the default required error if field is missing', async () => {
      const Model = new Schema<{ required: number }>((b) =>
        b.field(b.required('required').validate(() => ({ valid: true }))),
      ).getModel();

      const { error } = await Model.create({}, {});

      expect(error?.required?.reason).toBeTruthy();

      const required = 2;
      const { data } = await Model.update(
        { required: required - 1 },
        { required },
        {},
      );

      expect(data).toEqual({ required });
    });

    it('should respect custom static required error if field is missing', async () => {
      const requiredError = 'Yooo! you did not provide: "required"';

      const Model = new Schema<{ required: number }>((b) =>
        b.field(
          b
            .required('required')
            .requiredError(requiredError)
            .validate(() => ({ valid: true })),
        ),
      ).getModel();

      const { error } = await Model.create({}, {});

      expect(error?.required?.reason).toBe(requiredError);

      const required = 2;
      const { data } = await Model.update(
        { required: required - 1 },
        { required },
        {},
      );

      expect(data).toEqual({ required });
    });

    it('should respect custom dynamic required error if field is missing', async () => {
      const REQUIRED_ERROR = 'Yooo! you did not provide: "required"';

      const Model = new Schema<{ required: number }>((b) =>
        b.field(
          b
            .required('required')
            .requiredError(() => REQUIRED_ERROR)
            .validate(() => ({ valid: true })),
        ),
      ).getModel();

      const { error } = await Model.create({}, {});

      expect(error?.required?.reason).toBe(REQUIRED_ERROR);

      const required = 2;
      const { data } = await Model.update(
        { required: required - 1 },
        { required },
        {},
      );

      expect(data).toEqual({ required });
    });
  });

  describe('validators', () => {
    it('should not create if primary validation fails', async () => {
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';

      const Model = new Schema<{ required: string }>((b) =>
        b.field(
          b.required('required').validate((v) => {
            const validated = String(v).trim();
            if (validated.length < 2)
              return { valid: false, reason: MIN_LENGTH_ERROR };
            return { valid: true, validated };
          }),
        ),
      ).getModel();

      for (const requiredValue of [' ', ' 1', '1', ' 1   ']) {
        const { error } = await Model.create({ required: requiredValue }, {});
        expect(error?.required?.reason).toBe(MIN_LENGTH_ERROR);
      }

      for (const requiredValue of ['11', '111']) {
        const { data } = await Model.create({ required: requiredValue }, {});
        expect(data?.required).toBe(requiredValue);
      }
    });

    it('should not update if primary validation fails', async () => {
      const OUT_OF_RANGE_ERROR = 'required must be between 1 & 5 inclussive';
      const range = [1, 2, 3, 4, 5];

      const Model = new Schema<
        { required: number },
        { id: number; required: number }
      >((b) =>
        b.field(b.constant('id', () => 1)).field(
          b.required('required').validate((v) => {
            if (!range.includes(v as number))
              return { valid: false, reason: OUT_OF_RANGE_ERROR };
            return { valid: true };
          }),
        ),
      ).getModel();

      const data = { id: 1, required: 2 };

      for (const requiredValue of [-1, 0, 6]) {
        const { error } = await Model.update(
          data,
          { required: requiredValue },
          {},
        );
        expect(error?.payload?.required?.reason).toBe(OUT_OF_RANGE_ERROR);
      }

      for (const updatedValue of range) {
        if (updatedValue === data.required) continue;

        const { data: updates } = await Model.update(
          data,
          { required: updatedValue },
          {},
        );

        expect(updates).toEqual({ required: updatedValue });
      }
    });

    it('should properly use input values as output values if validator does not return a validated value', async () => {
      const Model = new Schema<{ required: number }>((b) =>
        b.field(b.required('required').validate(() => ({ valid: true }))),
      ).getModel();

      const required = 1;
      const { data } = await Model.create({ required }, {});

      expect(data).toEqual({ required });

      const required2 = 2;
      const { data: updates } = await Model.update(
        { required: required2 - 1 },
        { required: required2 },
        {},
      );

      expect(updates).toEqual({ required: required2 });
    });
  });

  describe('re-validators', () => {
    it('should not create if re-validation fails', async () => {
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';
      const MIN_REVALIDATION_LENGTH_ERROR =
        'expected required to be at least 4 characters long';

      const Model = new Schema<{ required: string }>((b) =>
        b.field(
          b
            .required('required')
            .validate((v) => {
              const validated = String(v).trim();
              if (validated.length < 2)
                return { valid: false, reason: MIN_LENGTH_ERROR };
              return { valid: true, validated };
            })
            .reValidate((v) => {
              if (v.length < 4)
                return {
                  valid: false,
                  reason: MIN_REVALIDATION_LENGTH_ERROR,
                };
              return { valid: true };
            }),
        ),
      ).getModel();

      for (const requiredValue of [' 111', ' 11 ', '11', ' 112   ']) {
        const { error } = await Model.create({ required: requiredValue }, {});
        expect(error?.required?.reason).toBe(MIN_REVALIDATION_LENGTH_ERROR);
      }

      for (const requiredValue of ['1111', '11111']) {
        const { data } = await Model.create({ required: requiredValue }, {});
        expect(data?.required).toBe(requiredValue);
      }
    });

    it('should not update if re-validation fails', async () => {
      const OUT_OF_RANGE_ERROR = 'required must be between 1 & 50 inclussive';
      const revalidatedRange = { min: 10, max: 35 };
      const REVALIDATED_OUT_OF_RANGE_ERROR =
        'revalidated required must be between 10 & 35 inclussive';

      const Model = new Schema<
        { required: number },
        { id: number; required: number }
      >((b) =>
        b.field(b.constant('id', () => 1)).field(
          b
            .required('required')
            .validate((v) => {
              if ((v as number) < 1 || (v as number) > 50)
                return { valid: false, reason: OUT_OF_RANGE_ERROR };
              return { valid: true };
            })
            .reValidate((v) => {
              if (v < revalidatedRange.min || v > revalidatedRange.max)
                return {
                  valid: false,
                  reason: REVALIDATED_OUT_OF_RANGE_ERROR,
                };
              return { valid: true };
            }),
        ),
      ).getModel();

      const data = { id: 1, required: 20 };

      for (const requiredValue of [
        revalidatedRange.min - 1,
        revalidatedRange.max + 1,
      ]) {
        const { error } = await Model.update(
          data,
          { required: requiredValue },
          {},
        );
        expect(error?.payload?.required?.reason).toBe(
          REVALIDATED_OUT_OF_RANGE_ERROR,
        );
      }

      for (
        let updatedValue = revalidatedRange.min;
        updatedValue <= revalidatedRange.max;
        updatedValue++
      ) {
        if (updatedValue === data.required) continue;

        const { data: updates } = await Model.update(
          data,
          { required: updatedValue },
          {},
        );

        expect(updates).toEqual({ required: updatedValue });
      }
    });

    it('should properly use re-validated values', async () => {
      const Model = new Schema<{ required: number }>((b) =>
        b.field(
          b
            .required('required')
            .validate(() => ({ valid: true }))
            .reValidate((v) => ({ valid: true, validated: v + 1 })),
        ),
      ).getModel();

      const value = 1;
      const { data } = await Model.create({ required: value }, {});

      expect(data).toEqual({ required: value + 1 });

      const value2 = 2;
      const { data: updates } = await Model.update(
        { required: value2 - 1 },
        { required: value2 },
        {},
      );

      expect(updates).toEqual({ required: value2 + 1 });
    });

    it('should properly use input values as output values if re-validator does not return a validated value', async () => {
      const Model = new Schema<{ required: number }>((b) =>
        b.field(
          b
            .required('required')
            .validate((v) => ({ valid: true, validated: (v as number) + 1 }))
            .reValidate(() => ({ valid: true })),
        ),
      ).getModel();

      const value = 1;
      const { data } = await Model.create({ required: value }, {});

      expect(data).toEqual({ required: value + 1 });

      const value2 = 2;
      const { data: updates } = await Model.update(
        { required: value2 - 1 },
        { required: value2 },
        {},
      );

      expect(updates).toEqual({ required: value2 + 1 });
    });
  });

  describe('post-validation', () => {
    it('should respect post validation config', async () => {
      const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS =
        'required failed pre-validation with unrelated errors';
      const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS =
        'required failed post-validation with unrelated errors';
      const REQUIRED_1_PRE_VALIDATION_FAIL = 'required 1 failed pre-validation';
      const BOTH_PRE_VALIDATION_FAIL = 'both failed pre-validation';
      const REQUIRED_VALIDATION_FAIL = 'required failed post-validatrion';
      const BOTH_VALIDATION_FAIL = 'both failed post-validatrion';

      const Model = new Schema<{
        required: string;
        required_1: string;
        required_2: string;
      }>(
        (b) =>
          b
            .field(b.required('required').validate(() => ({ valid: true })))
            .field(b.required('required_1').validate(() => ({ valid: true })))
            .field(b.required('required_2').validate(() => ({ valid: true }))),
        {
          postValidate: {
            fields: ['required', 'required_1'],
            validator: [
              (ctx) => {
                const required = ctx.input.required;
                if (
                  required ===
                  REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                )
                  return {
                    required:
                      REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                    required_2:
                      REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                  };
                if (required === BOTH_PRE_VALIDATION_FAIL)
                  return {
                    required: BOTH_PRE_VALIDATION_FAIL,
                    required_1: BOTH_PRE_VALIDATION_FAIL,
                  };
                if (ctx.values.required_1 === REQUIRED_1_PRE_VALIDATION_FAIL)
                  return { required_1: REQUIRED_1_PRE_VALIDATION_FAIL };
                return undefined;
              },
              (ctx) => {
                const required = ctx.input.required;
                if (
                  required ===
                  REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                )
                  return {
                    required:
                      REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                    required_2:
                      REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                  };
                if (required === REQUIRED_VALIDATION_FAIL)
                  return { required: REQUIRED_VALIDATION_FAIL };
                if (required === BOTH_VALIDATION_FAIL)
                  return {
                    required: BOTH_VALIDATION_FAIL,
                    required_1: BOTH_VALIDATION_FAIL,
                  };
                return undefined;
              },
            ],
          },
        },
      ).getModel();

      const value = 'some value';

      let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
      let { error } = await Model.create(
        { required, required_1: value, required_2: value },
        {},
      );

      expect(error?.required_1).toBeUndefined();
      expect(error?.required_2).toBeUndefined();
      expect(error?.required?.reason).toBe(required);

      required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
      ({ error } = await Model.create(
        { required, required_1: value, required_2: value },
        {},
      ));

      expect(error?.required_1).toBeUndefined();
      expect(error?.required_2).toBeUndefined();
      expect(error?.required?.reason).toBe(required);

      const required1 = REQUIRED_1_PRE_VALIDATION_FAIL;
      ({ error } = await Model.create(
        { required: value, required_1: required1, required_2: value },
        {},
      ));

      expect(error?.required).toBeUndefined();
      expect(error?.required_2).toBeUndefined();
      expect(error?.required_1?.reason).toBe(required1);

      required = BOTH_PRE_VALIDATION_FAIL;
      ({ error } = await Model.create(
        { required, required_1: value, required_2: value },
        {},
      ));

      expect(error?.required_2).toBeUndefined();
      expect(error?.required?.reason).toBe(required);
      expect(error?.required_1?.reason).toBe(required);

      required = REQUIRED_VALIDATION_FAIL;
      ({ error } = await Model.create(
        { required, required_1: value, required_2: value },
        {},
      ));

      expect(error?.required_1).toBeUndefined();
      expect(error?.required_2).toBeUndefined();
      expect(error?.required?.reason).toBe(required);

      required = BOTH_VALIDATION_FAIL;
      ({ error } = await Model.create(
        { required, required_1: value, required_2: value },
        {},
      ));

      expect(error?.required_2).toBeUndefined();
      expect(error?.required?.reason).toBe(required);
      expect(error?.required_1?.reason).toBe(required);

      // updates

      const data1 = {
        required: value,
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL,
        required_2: value,
      };

      let { error: updateError } = await Model.update(
        data1,
        { required: 'lol' },
        {},
      );

      expect(updateError?.payload?.required).toBeUndefined();
      expect(updateError?.payload?.required_2).toBeUndefined();
      expect(updateError?.payload?.required_1?.reason).toBe(
        REQUIRED_1_PRE_VALIDATION_FAIL,
      );

      required = BOTH_PRE_VALIDATION_FAIL;
      ({ error: updateError } = await Model.update(data1, { required }, {}));

      expect(updateError?.payload?.required_2).toBeUndefined();
      expect(updateError?.payload?.required?.reason).toBe(required);
      expect(updateError?.payload?.required_1?.reason).toBe(required);

      required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
      ({ error: updateError } = await Model.update(data1, { required }, {}));

      expect(updateError?.payload?.required_1).toBeUndefined();
      expect(updateError?.payload?.required_2).toBeUndefined();
      expect(updateError?.payload?.required?.reason).toBe(required);

      const data2 = { ...data1, required_1: value };
      required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;

      ({ error: updateError } = await Model.update(data2, { required }, {}));

      expect(updateError?.payload?.required_1).toBeUndefined();
      expect(updateError?.payload?.required_2).toBeUndefined();
      expect(updateError?.payload?.required?.reason).toBe(required);
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

      const Model = new Schema<{
        required: string;
        required_1: string;
        required_2: string;
      }>(
        (b) =>
          b
            .field(b.required('required').validate(() => ({ valid: true })))
            .field(b.required('required_1').validate(() => ({ valid: true })))
            .field(b.required('required_2').validate(() => ({ valid: true }))),
        {
          postValidate: {
            fields: ['required', 'required_1'],
            validator: [
              (ctx) => {
                if (
                  ctx.input.required === LAX_PRE_VALIDATED_WITH_UPDATED_VALUES
                )
                  return {
                    required: { validated: UPDATED_VALUE_FROM_PRE_VALIDATOR },
                    required_1: {
                      validated: UPDATED_VALUE_FROM_PRE_VALIDATOR,
                    },
                  };
                return undefined;
              },
              (ctx) => {
                if (
                  ctx.input.required === LAX_POST_VALIDATED_WITH_UPDATED_VALUES
                )
                  return {
                    required: {
                      validated: UPDATED_VALUE_FROM_POST_VALIDATOR,
                    },
                    required_1: {
                      validated: UPDATED_VALUE_FROM_POST_VALIDATOR,
                    },
                  };
                return undefined;
              },
            ],
          },
        },
      ).getModel();

      const value = 'some random value';

      let { data } = await Model.create(
        {
          required: LAX_PRE_VALIDATED_WITH_UPDATED_VALUES,
          required_1: value,
          required_2: value,
        },
        {},
      );

      expect(data).toEqual({
        required: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        required_1: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        required_2: value,
      });

      ({ data } = await Model.create(
        {
          required: LAX_POST_VALIDATED_WITH_UPDATED_VALUES,
          required_1: value,
          required_2: value,
        },
        {},
      ));

      expect(data).toEqual({
        required: UPDATED_VALUE_FROM_POST_VALIDATOR,
        required_1: UPDATED_VALUE_FROM_POST_VALIDATOR,
        required_2: value,
      });

      const previous = {
        required: value,
        required_1: value,
        required_2: value,
      };

      let { data: updates } = await Model.update(
        previous,
        { required: LAX_PRE_VALIDATED_WITH_UPDATED_VALUES },
        {},
      );

      expect(updates).toEqual({
        required: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        required_1: UPDATED_VALUE_FROM_PRE_VALIDATOR,
      });

      ({ data: updates } = await Model.update(
        previous,
        { required: LAX_POST_VALIDATED_WITH_UPDATED_VALUES },
        {},
      ));

      expect(updates).toEqual({
        required: UPDATED_VALUE_FROM_POST_VALIDATOR,
        required_1: UPDATED_VALUE_FROM_POST_VALIDATOR,
      });
    });
  });
});
