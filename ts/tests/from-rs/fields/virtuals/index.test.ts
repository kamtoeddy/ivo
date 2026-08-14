import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';
import { expectFailure, makeFx } from '../../../_utils';

/**
 * The Rust suite triplicates almost every test in this file across three
 * virtual-field naming schemes: the field's own name, an alias, and an
 * alias that happens to collide with the name of the dependent field that
 * consumes it. Looping over the same three schemes here covers identical
 * ground without tripling this already-large file.
 */
const NAMING_SCHEMES: { alias?: string }[] = [
  {},
  { alias: 'virtualAlias' },
  { alias: 'dependent' },
];

function buildVirtual(b: any, alias?: string) {
  const virtual = b.virtual('virtualField');
  return alias ? virtual.alias(alias) : virtual;
}

describe('fields.virtual', () => {
  describe('nothing to update', () => {
    it('should reject updates if no value has changed', async () => {
      for (const { alias } of NAMING_SCHEMES) {
        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(1)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(buildVirtual(b, alias).validate(() => true)),
        ).getModel();

        const value = 24;
        const { error } = await Model.update(
          { dependent: value },
          { [alias ?? 'virtualField']: value },
          {},
        );

        expect(error).toEqual({ isNothingToUpdate: true, payload: null });
      }
    });
  });

  describe('required', () => {
    it('should respect the required rule', async () => {
      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;
        const defaultLaxValue = 'default_lax_value';

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.values.dependent + 1),
            )
            .field(b.lax('lax', defaultLaxValue))
            .field(
              buildVirtual(b, alias)
                .validate(() => ({ valid: true }))
                .required((ctx: any) => {
                  if (ctx.isUpdate) {
                    if (
                      ctx.previousValues.lax ===
                      'require_virtual_field_for_update'
                    )
                      return [
                        true,
                        'virtual_field is required for this update',
                      ];
                    return false;
                  }

                  if (ctx.input.lax === 'required_virtual_field_for_init')
                    return [
                      true,
                      'virtual_field is required to create at this time',
                    ];

                  return false;
                }),
            ),
        ).getModel();

        const { error } = await Model.create(
          { lax: 'required_virtual_field_for_init' },
          {},
        );

        expect(error?.[alias ?? 'virtualField']?.reason).toBe(
          'virtual_field is required to create at this time',
        );

        const lax = 'require_virtual_field_for_update';
        const { data } = await Model.create({ lax }, {});

        expect(data).toEqual({ dependent: defaultDependentValue, lax });

        const { error: updateError } = await Model.update(
          data,
          { lax: 'some update' },
          {},
        );

        expect(updateError?.payload?.[alias ?? 'virtualField']?.reason).toBe(
          'virtual_field is required for this update',
        );
      }
    });
  });

  describe('grouped required', () => {
    it('should properly handle grouped required errors', async () => {
      const IGNORE_WITH_DIFFERENT_ERRORS = 'IGNORE_WITH_DIFFERENT_ERRORS';
      const IGNORE_WITH_SAME_ERROR = 'IGNORE_WITH_SAME_ERROR';
      const EXPECTED_VIRTUAL_OR_LAX_1 = 'EXPECTED_VIRTUAL_OR_LAX_1';
      const VIRTUAL_IS_MISSING = 'VIRTUAL_IS_MISSING';
      const LAX_1_IS_MISSING = 'LAX_1_IS_MISSING';

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 'default_dependent_value';
        const defaultLax1Value = 'default_lax_1_value';
        const defaultLax2Value = 'default_lax_2_value';

        const Model = new Schema<any, any>(
          (b) =>
            b
              .field(
                b
                  .dependent('dependent', 'virtualField')
                  .default(defaultDependentValue)
                  .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
              )
              .field(buildVirtual(b, alias).validate(() => ({ valid: true })))
              .field(b.lax('lax_1', defaultLax1Value))
              .field(b.lax('lax_2', defaultLax2Value)),
          {
            required: {
              fields: ['virtualField', 'lax_1'] as never,
              handler: async (ctx: any) => {
                const lax2 = ctx.input.lax_2;
                if (lax2 == null) return undefined;
                if (lax2 === IGNORE_WITH_SAME_ERROR)
                  return {
                    [alias ?? 'virtualField']: EXPECTED_VIRTUAL_OR_LAX_1,
                    lax_1: EXPECTED_VIRTUAL_OR_LAX_1,
                  };
                return {
                  [alias ?? 'virtualField']: VIRTUAL_IS_MISSING,
                  lax_1: LAX_1_IS_MISSING,
                };
              },
            },
          },
        ).getModel();

        let { error } = await Model.create(
          { lax_2: IGNORE_WITH_SAME_ERROR },
          {},
        );

        expect(error?.lax_2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(
          EXPECTED_VIRTUAL_OR_LAX_1,
        );
        expect(error?.lax_1?.reason).toBe(EXPECTED_VIRTUAL_OR_LAX_1);

        ({ error } = await Model.create(
          { lax_2: IGNORE_WITH_DIFFERENT_ERRORS },
          {},
        ));

        expect(error?.lax_2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(
          VIRTUAL_IS_MISSING,
        );
        expect(error?.lax_1?.reason).toBe(LAX_1_IS_MISSING);

        const data = {
          dependent: defaultDependentValue,
          lax_1: defaultLax1Value,
          lax_2: defaultLax2Value,
        };

        let { error: updateError } = await Model.update(
          data,
          { lax_2: IGNORE_WITH_SAME_ERROR },
          {},
        );

        expect(updateError?.payload?.lax_2).toBeUndefined();
        expect(updateError?.payload?.[alias ?? 'virtualField']?.reason).toBe(
          EXPECTED_VIRTUAL_OR_LAX_1,
        );
        expect(updateError?.payload?.lax_1?.reason).toBe(
          EXPECTED_VIRTUAL_OR_LAX_1,
        );

        ({ error: updateError } = await Model.update(
          data,
          { lax_2: IGNORE_WITH_DIFFERENT_ERRORS },
          {},
        ));

        expect(updateError?.payload?.lax_2).toBeUndefined();
        expect(updateError?.payload?.[alias ?? 'virtualField']?.reason).toBe(
          VIRTUAL_IS_MISSING,
        );
        expect(updateError?.payload?.lax_1?.reason).toBe(LAX_1_IS_MISSING);
      }
    });
  });

  describe('validators', () => {
    it('should not create if primary validation fails', async () => {
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.values.dependent + 1),
            )
            .field(
              buildVirtual(b, alias).validate((v: unknown) => {
                const validated = String(v).trim();
                if (validated.length < 2)
                  return { valid: false, reason: MIN_LENGTH_ERROR };
                return { valid: true, validated };
              }),
            ),
        ).getModel();

        for (const value of [' ', ' 1', '1', ' 1   ']) {
          const { error } = await Model.create(
            { [alias ?? 'virtualField']: value },
            {},
          );
          expect(error?.[alias ?? 'virtualField']?.reason).toBe(
            MIN_LENGTH_ERROR,
          );
        }

        for (const value of ['11', '111']) {
          const { data } = await Model.create(
            { [alias ?? 'virtualField']: value },
            {},
          );
          expect(data.dependent).toBe(defaultDependentValue + 1);
        }
      }
    });

    it('should not update if primary validation fails', async () => {
      const OUT_OF_RANGE_ERROR =
        'virtual_field must be between 1 & 5 inclussive';
      const range = [1, 2, 3, 4, 5];

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(
              buildVirtual(b, alias).validate((v: unknown) => {
                if (!range.includes(v as number))
                  return { valid: false, reason: OUT_OF_RANGE_ERROR };
                return { valid: true };
              }),
            ),
        ).getModel();

        const data = { dependent: defaultDependentValue };

        for (const value of [-1, 0, 6]) {
          const { error } = await Model.update(
            data,
            { [alias ?? 'virtualField']: value },
            {},
          );
          expect(error?.payload?.[alias ?? 'virtualField']?.reason).toBe(
            OUT_OF_RANGE_ERROR,
          );
        }

        for (const updatedValue of range) {
          if (updatedValue === data.dependent) continue;

          const { data: updates } = await Model.update(
            data,
            { [alias ?? 'virtualField']: updatedValue },
            {},
          );

          expect(updates).toEqual({ dependent: updatedValue });
        }
      }
    });

    it('should properly use input values as output values if validator does not return a validated value', async () => {
      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(
              buildVirtual(b, alias).validate(() => ({
                valid: true,
              })),
            ),
        ).getModel();

        const value = 1;
        const { data } = await Model.create(
          { [alias ?? 'virtualField']: value },
          {},
        );

        expect(data).toEqual({ dependent: value });

        const value2 = 2;
        const { data: updates } = await Model.update(
          { dependent: value2 - 1 },
          { [alias ?? 'virtualField']: value2 },
          {},
        );

        expect(updates).toEqual({ dependent: value2 });
      }
    });
  });

  describe('re-validators', () => {
    it('should not create if re-validation fails', async () => {
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';
      const MIN_REVALIDATION_LENGTH_ERROR =
        'expected required to be at least 4 characters long';

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.values.dependent + 1),
            )
            .field(
              buildVirtual(b, alias)
                .validate((v: unknown) => {
                  const validated = String(v).trim();
                  if (validated.length < 2)
                    return { valid: false, reason: MIN_LENGTH_ERROR };
                  return { valid: true, validated };
                })
                .reValidate((v: string) => {
                  if (v.length < 4)
                    return {
                      valid: false,
                      reason: MIN_REVALIDATION_LENGTH_ERROR,
                    };
                  return { valid: true };
                }),
            ),
        ).getModel();

        for (const value of [' 111', ' 11 ', '11', ' 112   ']) {
          const { error } = await Model.create(
            { [alias ?? 'virtualField']: value },
            {},
          );
          expect(error?.[alias ?? 'virtualField']?.reason).toBe(
            MIN_REVALIDATION_LENGTH_ERROR,
          );
        }

        for (const value of ['1111', '11111']) {
          const { data } = await Model.create(
            { [alias ?? 'virtualField']: value },
            {},
          );
          expect(data.dependent).toBe(defaultDependentValue + 1);
        }
      }
    });

    it('should not update if re-validation fails', async () => {
      const OUT_OF_RANGE_ERROR = 'required must be between 1 & 50 inclussive';
      const revalidatedRange = { min: 10, max: 35 };
      const REVALIDATED_OUT_OF_RANGE_ERROR =
        'revalidated required must be between 10 & 35 inclussive';

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(
              buildVirtual(b, alias)
                .validate((v: unknown) => {
                  if ((v as number) < 1 || (v as number) > 50)
                    return { valid: false, reason: OUT_OF_RANGE_ERROR };
                  return { valid: true };
                })
                .reValidate((v: number) => {
                  if (v < revalidatedRange.min || v > revalidatedRange.max)
                    return {
                      valid: false,
                      reason: REVALIDATED_OUT_OF_RANGE_ERROR,
                    };
                  return { valid: true };
                }),
            ),
        ).getModel();

        const data = { dependent: defaultDependentValue };

        for (const value of [
          revalidatedRange.min - 1,
          revalidatedRange.max + 1,
        ]) {
          const { error } = await Model.update(
            data,
            { [alias ?? 'virtualField']: value },
            {},
          );
          expect(error?.payload?.[alias ?? 'virtualField']?.reason).toBe(
            REVALIDATED_OUT_OF_RANGE_ERROR,
          );
        }

        for (
          let updatedValue = revalidatedRange.min;
          updatedValue <= revalidatedRange.max;
          updatedValue++
        ) {
          if (updatedValue === data.dependent) continue;

          const { data: updates } = await Model.update(
            data,
            { [alias ?? 'virtualField']: updatedValue },
            {},
          );

          expect(updates).toEqual({ dependent: updatedValue });
        }
      }
    });

    it('should properly use re-validated values', async () => {
      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(
              buildVirtual(b, alias)
                .validate(() => ({ valid: true }))
                .reValidate((v: number) => ({ valid: true, validated: v + 1 })),
            ),
        ).getModel();

        const value = 1;
        const { data } = await Model.create(
          { [alias ?? 'virtualField']: value },
          {},
        );

        expect(data).toEqual({ dependent: value + 1 });

        const value2 = 2;
        const { data: updates } = await Model.update(
          { dependent: value2 - 1 },
          { [alias ?? 'virtualField']: value2 },
          {},
        );

        expect(updates).toEqual({ dependent: value2 + 1 });
      }
    });

    it('should properly use input values as output values if re-validator does not return a validated value', async () => {
      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(
              buildVirtual(b, alias)
                .validate((v: unknown) => ({
                  valid: true,
                  validated: (v as number) + 1,
                }))
                .reValidate(() => ({ valid: true })),
            ),
        ).getModel();

        const value = 1;
        const { data } = await Model.create(
          { [alias ?? 'virtualField']: value },
          {},
        );

        expect(data).toEqual({ dependent: value + 1 });

        const value2 = 2;
        const { data: updates } = await Model.update(
          { dependent: value2 - 1 },
          { [alias ?? 'virtualField']: value2 },
          {},
        );

        expect(updates).toEqual({ dependent: value2 + 1 });
      }
    });
  });

  describe('post-validation', () => {
    it('should respect post validation config', async () => {
      const VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS =
        'virtual_field failed pre-validation with unrelated errors';
      const VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS =
        'virtual_field failed post-validation with unrelated errors';
      const VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL =
        'required 1 failed pre-validation';
      const BOTH_PRE_VALIDATION_FAIL = 'both failed pre-validation';
      const VIRTUAL_FIELD_VALIDATION_FAIL =
        'virtual_field failed post-validatrion';
      const BOTH_VALIDATION_FAIL = 'both failed post-validatrion';

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>(
          (b) =>
            b
              .field(
                b
                  .dependent('dependent', [
                    'virtualField',
                    'virtualField1',
                    'virtualField2',
                  ])
                  .default(defaultDependentValue)
                  .resolve((ctx: any) => ctx.values.dependent + 1),
              )
              .field(buildVirtual(b, alias).validate(() => ({ valid: true })))
              .field(
                b.virtual('virtualField1').validate(() => ({ valid: true })),
              )
              .field(
                b.virtual('virtualField2').validate(() => ({ valid: true })),
              ),
          {
            postValidate: {
              fields: ['virtualField', 'virtualField1'],
              validator: [
                (ctx: any) => {
                  const virtualField = ctx.input[alias ?? 'virtualField'];
                  if (
                    virtualField ===
                    VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                  )
                    return {
                      [alias ?? 'virtualField']:
                        VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                      virtualField2:
                        VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                    };
                  if (virtualField === BOTH_PRE_VALIDATION_FAIL)
                    return {
                      [alias ?? 'virtualField']: BOTH_PRE_VALIDATION_FAIL,
                      virtualField1: BOTH_PRE_VALIDATION_FAIL,
                    };
                  if (
                    ctx.input.virtualField1 ===
                    VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL
                  )
                    return {
                      virtualField1: VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL,
                    };
                  return undefined;
                },
                (ctx: any) => {
                  const virtualField = ctx.input[alias ?? 'virtualField'];
                  if (
                    virtualField ===
                    VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                  )
                    return {
                      [alias ?? 'virtualField']:
                        VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                      virtualField2:
                        VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                    };
                  if (virtualField === VIRTUAL_FIELD_VALIDATION_FAIL)
                    return {
                      [alias ?? 'virtualField']: VIRTUAL_FIELD_VALIDATION_FAIL,
                    };
                  if (virtualField === BOTH_VALIDATION_FAIL)
                    return {
                      [alias ?? 'virtualField']: BOTH_VALIDATION_FAIL,
                      virtualField1: BOTH_VALIDATION_FAIL,
                    };
                  return undefined;
                },
              ],
            },
          },
        ).getModel();

        const someValue = 'some value';

        let virtualValue: string =
          VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
        let { error } = await Model.create(
          {
            [alias ?? 'virtualField']: virtualValue,
            virtualField1: someValue,
            virtualField2: someValue,
          },
          {},
        );

        expect(error?.virtualField1).toBeUndefined();
        expect(error?.virtualField2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(virtualValue);

        virtualValue = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
        ({ error } = await Model.create(
          {
            [alias ?? 'virtualField']: virtualValue,
            virtualField1: someValue,
            virtualField2: someValue,
          },
          {},
        ));

        expect(error?.virtualField1).toBeUndefined();
        expect(error?.virtualField2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(virtualValue);

        const virtualField1Value = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL;
        ({ error } = await Model.create(
          {
            [alias ?? 'virtualField']: someValue,
            virtualField1: virtualField1Value,
            virtualField2: someValue,
          },
          {},
        ));

        expect(error?.[alias ?? 'virtualField']).toBeUndefined();
        expect(error?.virtualField2).toBeUndefined();
        expect(error?.virtualField1?.reason).toBe(virtualField1Value);

        virtualValue = BOTH_PRE_VALIDATION_FAIL;
        ({ error } = await Model.create(
          {
            [alias ?? 'virtualField']: virtualValue,
            virtualField1: someValue,
            virtualField2: someValue,
          },
          {},
        ));

        expect(error?.virtualField2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(virtualValue);
        expect(error?.virtualField1?.reason).toBe(virtualValue);

        virtualValue = VIRTUAL_FIELD_VALIDATION_FAIL;
        ({ error } = await Model.create(
          {
            [alias ?? 'virtualField']: virtualValue,
            virtualField1: someValue,
            virtualField2: someValue,
          },
          {},
        ));

        expect(error?.virtualField1).toBeUndefined();
        expect(error?.virtualField2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(virtualValue);

        virtualValue = BOTH_VALIDATION_FAIL;
        ({ error } = await Model.create(
          {
            [alias ?? 'virtualField']: virtualValue,
            virtualField1: someValue,
            virtualField2: someValue,
          },
          {},
        ));

        expect(error?.virtualField2).toBeUndefined();
        expect(error?.[alias ?? 'virtualField']?.reason).toBe(virtualValue);
        expect(error?.virtualField1?.reason).toBe(virtualValue);

        // updates

        const data = { dependent: defaultDependentValue };

        let { error: updateError } = await Model.update(
          data,
          { virtualField1: VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL },
          {},
        );

        expect(updateError?.payload?.[alias ?? 'virtualField']).toBeUndefined();
        expect(updateError?.payload?.virtualField2).toBeUndefined();
        expect(updateError?.payload?.virtualField1?.reason).toBe(
          VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL,
        );

        virtualValue = BOTH_PRE_VALIDATION_FAIL;
        ({ error: updateError } = await Model.update(
          data,
          { [alias ?? 'virtualField']: virtualValue },
          {},
        ));

        expect(updateError?.payload?.virtualField2).toBeUndefined();
        expect(updateError?.payload?.[alias ?? 'virtualField']?.reason).toBe(
          virtualValue,
        );
        expect(updateError?.payload?.virtualField1?.reason).toBe(virtualValue);

        virtualValue = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
        ({ error: updateError } = await Model.update(
          data,
          { [alias ?? 'virtualField']: virtualValue },
          {},
        ));

        expect(updateError?.payload?.virtualField1).toBeUndefined();
        expect(updateError?.payload?.virtualField2).toBeUndefined();
        expect(updateError?.payload?.[alias ?? 'virtualField']?.reason).toBe(
          virtualValue,
        );

        virtualValue = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS;
        ({ error: updateError } = await Model.update(
          data,
          { [alias ?? 'virtualField']: virtualValue },
          {},
        ));

        expect(updateError?.payload?.virtualField1).toBeUndefined();
        expect(updateError?.payload?.virtualField2).toBeUndefined();
        expect(updateError?.payload?.[alias ?? 'virtualField']?.reason).toBe(
          virtualValue,
        );
      }
    });

    it('should respect updated values returned from pre-validator in post-validation config', async () => {
      const VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES =
        'VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES';
      const VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES =
        'VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES';
      const UPDATED_VALUE_FROM_PRE_VALIDATOR =
        'UPDATED_VALUE_FROM_PRE_VALIDATOR';
      const UPDATED_VALUE_FROM_POST_VALIDATOR =
        'UPDATED_VALUE_FROM_POST_VALIDATOR';

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 'default_dependent_value';

        const Model = new Schema<any, any>(
          (b) =>
            b
              .field(
                b
                  .dependent('dependent', ['virtualField', 'virtualField1'])
                  .default(defaultDependentValue)
                  .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
              )
              .field(buildVirtual(b, alias).validate(() => ({ valid: true })))
              .field(
                b.virtual('virtualField1').validate(() => ({ valid: true })),
              ),
          {
            postValidate: {
              fields: ['virtualField', 'virtualField1'],
              validator: [
                (ctx: any) => {
                  if (
                    ctx.input[alias ?? 'virtualField'] ===
                    VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES
                  )
                    return {
                      [alias ?? 'virtualField']: {
                        validated: UPDATED_VALUE_FROM_PRE_VALIDATOR,
                      },
                    };
                  return undefined;
                },
                (ctx: any) => {
                  if (
                    ctx.input[alias ?? 'virtualField'] ===
                    VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES
                  )
                    return {
                      [alias ?? 'virtualField']: {
                        validated: UPDATED_VALUE_FROM_POST_VALIDATOR,
                      },
                    };
                  return undefined;
                },
              ],
            },
          },
        ).getModel();

        let virtualValue: string =
          VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES;
        let { data } = await Model.create(
          { [alias ?? 'virtualField']: virtualValue },
          {},
        );

        expect(data).toEqual({ dependent: UPDATED_VALUE_FROM_PRE_VALIDATOR });

        virtualValue = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES;
        ({ data } = await Model.create(
          { [alias ?? 'virtualField']: virtualValue },
          {},
        ));

        expect(data).toEqual({ dependent: UPDATED_VALUE_FROM_POST_VALIDATOR });

        // updates

        const previous = { dependent: defaultDependentValue };

        virtualValue = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES;
        let { data: updates } = await Model.update(
          previous,
          { [alias ?? 'virtualField']: virtualValue },
          {},
        );

        expect(updates).toEqual({
          dependent: UPDATED_VALUE_FROM_PRE_VALIDATOR,
        });

        virtualValue = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES;
        ({ data: updates } = await Model.update(
          previous,
          { [alias ?? 'virtualField']: virtualValue },
          {},
        ));

        expect(updates).toEqual({
          dependent: UPDATED_VALUE_FROM_POST_VALIDATOR,
        });
      }
    });
  });

  describe('sanitizer', () => {
    it('should reject invalid sanitizer', () => {
      const values = [-1, 1, true, false, undefined, null, [], {}];

      for (const sanitizer of values) {
        const toFail = makeFx((b) =>
          b
            .field(
              b
                .dependent('dependentField', 'fieldName')
                .default('')
                .resolve(() => ''),
            )
            .field(
              b
                .virtual('fieldName')
                .validate(() => true)
                .sanitize(sanitizer as never),
            ),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              fieldName: expect.arrayContaining([
                "'sanitizer' must be a function",
              ]),
            }),
          );
        }
      }
    });

    it('should respect sanitizers if provided', async () => {
      function sanitize(value: string) {
        return `sanitized-${value}`;
      }

      for (const { alias } of NAMING_SCHEMES) {
        const defaultDependentValue = 'default_dependent_value';

        const Model = new Schema<any, any>((b) =>
          b
            .field(
              b
                .dependent('dependent', 'virtualField')
                .default(defaultDependentValue)
                .resolve((ctx: any) => ctx.input[alias ?? 'virtualField']),
            )
            .field(
              buildVirtual(b, alias)
                .validate(() => ({ valid: true }))
                .sanitize((ctx: any) =>
                  sanitize(ctx.input[alias ?? 'virtualField']),
                ),
            ),
        ).getModel();

        const virtualValue = 'virtual_value';
        const { data } = await Model.create(
          { [alias ?? 'virtualField']: virtualValue },
          {},
        );

        expect(data).toEqual({ dependent: sanitize(virtualValue) });
        expect(data).not.toEqual({ dependent: virtualValue });

        // updates

        const previous = { dependent: defaultDependentValue };
        const updatedVirtualValue = 'updated_virtual_value';

        const { data: updates } = await Model.update(
          previous,
          { [alias ?? 'virtualField']: updatedVirtualValue },
          {},
        );

        expect(updates).toEqual({ dependent: sanitize(updatedVirtualValue) });
        expect(updates).not.toEqual({ dependent: updatedVirtualValue });
      }
    });
  });
});
