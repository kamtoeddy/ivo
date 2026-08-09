import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

const NAMING_SCHEMES: { publicKey: string; alias?: string }[] = [
  { publicKey: 'virtualField' },
  { publicKey: 'virtualAlias', alias: 'virtualAlias' },
  { publicKey: 'dependent', alias: 'dependent' },
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
    for (const scheme of NAMING_SCHEMES) {
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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.input[scheme.publicKey];
              }),
          ),
      ).getModel();

      const { error, handleFailure } = await Model.create(
        { [scheme.publicKey]: 'fail_validation' },
        {},
      );

      expect(error?.[scheme.publicKey]?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('fail_validation');
    }
  });

  it('should trigger onFailure handlers during updates', async () => {
    for (const scheme of NAMING_SCHEMES) {
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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.input[scheme.publicKey];
              }),
          ),
      ).getModel();

      const { error, handleFailure } = await Model.update(
        { dependent: defaultDependentValue },
        { [scheme.publicKey]: 'fail_validation' },
        {},
      );

      expect(error?.payload?.[scheme.publicKey]?.reason).toBe(
        'validation failed',
      );

      await handleFailure?.();

      expect(triggeredWith).toBe('fail_validation');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignore fn at creation', async () => {
    for (const scheme of NAMING_SCHEMES) {
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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignore(() => true)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[scheme.publicKey];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.create(
        {
          [scheme.publicKey]: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.[scheme.publicKey]).toBeUndefined();
      expect(error?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignore fn during updates', async () => {
    for (const scheme of NAMING_SCHEMES) {
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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignore(() => true)
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[scheme.publicKey];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.update(
        { dependent: defaultDependentValue },
        {
          [scheme.publicKey]: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.payload?.[scheme.publicKey]).toBeUndefined();
      expect(error?.payload?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignoreInit fn', async () => {
    for (const scheme of NAMING_SCHEMES) {
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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignoreInit()
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[scheme.publicKey];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.create(
        {
          [scheme.publicKey]: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.[scheme.publicKey]).toBeUndefined();
      expect(error?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });

  it('should trigger onFailure handlers even if provided and ignored by ignoreUpdate fn', async () => {
    for (const scheme of NAMING_SCHEMES) {
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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignoreUpdate()
              .onFailure((ctx: any) => {
                triggeredWith = ctx.rawInput[scheme.publicKey];
              }),
          )
          .field(b.virtual('virtualField2').validate(virtualValidator)),
      ).getModel();

      const { error, handleFailure } = await Model.update(
        { dependent: defaultDependentValue },
        {
          [scheme.publicKey]: 'update to be ignored',
          virtualField2: 'fail_validation',
        },
        {},
      );

      expect(error?.payload?.[scheme.publicKey]).toBeUndefined();
      expect(error?.payload?.virtualField2?.reason).toBe('validation failed');

      await handleFailure?.();

      expect(triggeredWith).toBe('update to be ignored');
    }
  });
});
