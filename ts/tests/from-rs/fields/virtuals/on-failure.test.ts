import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

const NAMING_SCHEMES: { alias?: string }[] = [
  {},
  { alias: 'virtualAlias' },
  { alias: 'dependent' },
];

function buildVirtual(b: any, alias?: string) {
  const virtual = b.virtual('virtualField');
  return alias ? virtual.alias(alias) : virtual;
}

function virtualValidator(v: unknown) {
  if (v === 'fail_validation')
    return { valid: false, reason: 'validation failed' } as const;
  return { valid: true } as const;
}

describe('fields.virtual.onFailure', () => {
  it('should trigger onFailure handlers at creation', async () => {
    for (const { alias } of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;

      const Model = new Schema<any, any>((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(1)
              .resolve((ctx: any) => ctx.values.dependent + 1),
          )
          .field(
            buildVirtual(b, alias)
              .validate(virtualValidator)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.input[alias ?? 'virtualField'];
              }),
          ),
      ).getModel();

      const { error, handleFailure } = await Model.create(
        { [alias ?? 'virtualField']: 'fail_validation' },
        {},
      );

      expect(error?.[alias ?? 'virtualField']?.reason).toBe(
        'validation failed',
      );

      await handleFailure?.();

      expect(triggeredWith).toBe('fail_validation');
    }
  });

  it('should trigger onFailure handlers during updates', async () => {
    for (const { alias } of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
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
              .validate(virtualValidator)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.input[alias ?? 'virtualField'];
              }),
          ),
      ).getModel();

      const { error, handleFailure } = await Model.update(
        { dependent: defaultDependentValue },
        { [alias ?? 'virtualField']: 'fail_validation' },
        {},
      );

      expect(error?.payload?.[alias ?? 'virtualField']?.reason).toBe(
        'validation failed',
      );

      await handleFailure?.();

      expect(triggeredWith).toBe('fail_validation');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignore fn at creation', async () => {
    for (const { alias } of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
      const defaultDependentValue = 1;

      const Model = new Schema<any, any>((b) =>
        b
          .field(
            b
              .dependent('dependent', ['virtualField', 'virtualField2'])
              .default(defaultDependentValue)
              .resolve((ctx: any) => ctx.values.dependent + 1),
          )
          .field(
            buildVirtual(b, alias)
              .validate(virtualValidator)
              .ignore(() => true)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[alias ?? 'virtualField'];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.create(
        {
          [alias ?? 'virtualField']: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.[alias ?? 'virtualField']).toBeUndefined();
      expect(error?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignore fn during updates', async () => {
    for (const { alias } of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
      const defaultDependentValue = 1;

      const Model = new Schema<any, any>((b) =>
        b
          .field(
            b
              .dependent('dependent', ['virtualField', 'virtualField2'])
              .default(defaultDependentValue)
              .resolve((ctx: any) => ctx.values.dependent + 1),
          )
          .field(
            buildVirtual(b, alias)
              .validate(virtualValidator)
              .ignore(() => true)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[alias ?? 'virtualField'];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.update(
        { dependent: defaultDependentValue },
        {
          [alias ?? 'virtualField']: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.payload?.[alias ?? 'virtualField']).toBeUndefined();
      expect(error?.payload?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignoreInit fn', async () => {
    for (const { alias } of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
      const defaultDependentValue = 1;

      const Model = new Schema<any, any>((b) =>
        b
          .field(
            b
              .dependent('dependent', ['virtualField', 'virtualField2'])
              .default(defaultDependentValue)
              .resolve((ctx: any) => ctx.values.dependent + 1),
          )
          .field(
            buildVirtual(b, alias)
              .validate(virtualValidator)
              .ignoreInit()
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[alias ?? 'virtualField'];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.create(
        {
          [alias ?? 'virtualField']: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.[alias ?? 'virtualField']).toBeUndefined();
      expect(error?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignoreUpdate fn', async () => {
    for (const { alias } of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
      const defaultDependentValue = 1;

      const Model = new Schema<any, any>((b) =>
        b
          .field(
            b
              .dependent('dependent', ['virtualField', 'virtualField2'])
              .default(defaultDependentValue)
              .resolve((ctx: any) => ctx.values.dependent + 1),
          )
          .field(
            buildVirtual(b, alias)
              .validate(virtualValidator)
              .ignoreUpdate()
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[alias ?? 'virtualField'];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.update(
        { dependent: defaultDependentValue },
        {
          [alias ?? 'virtualField']: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.payload?.[alias ?? 'virtualField']).toBeUndefined();
      expect(error?.payload?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });
});
