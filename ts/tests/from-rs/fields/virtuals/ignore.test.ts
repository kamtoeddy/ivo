import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

/**
 * As in `index.test.ts`, the Rust suite triplicates each test here across
 * three virtual-field naming schemes. Looping over them covers the same
 * ground without tripling the file.
 */
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

describe('fields.virtual.ignore', () => {
  it('should respect the ignore rule', async () => {
    for (const scheme of NAMING_SCHEMES) {
      const defaultDependentValue = 1;
      const defaultLaxValue = 10;

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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignore(() => true),
          ),
      ).getModel();

      let { data } = await Model.create(
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(data).toEqual({
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
      });

      const lax = defaultLaxValue + 10;
      ({ data } = await Model.create(
        { lax, [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(data).toEqual({ dependent: defaultDependentValue, lax });

      const lax2 = data.lax + 10;
      const { data: updates } = await Model.update(
        data,
        { lax: lax2, [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(updates).toEqual({ lax: lax2 });

      const { error } = await Model.update(
        data,
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    }
  });

  it('should respect the ignoreInit rule', async () => {
    for (const scheme of NAMING_SCHEMES) {
      const defaultDependentValue = 1;
      const defaultLaxValue = 10;

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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignoreInit(),
          ),
      ).getModel();

      let { data } = await Model.create(
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(data).toEqual({
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
      });

      const lax = defaultLaxValue + 10;
      ({ data } = await Model.create(
        { lax, [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(data).toEqual({ dependent: defaultDependentValue, lax });

      const lax2 = data.lax + 10;
      let { data: updates } = await Model.update(
        data,
        { lax: lax2, [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(updates).toEqual({ dependent: data.dependent + 1, lax: lax2 });

      ({ data: updates } = await Model.update(
        data,
        { [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(updates).toEqual({ dependent: data.dependent + 1 });
    }
  });

  it('should respect the ignoreUpdate rule', async () => {
    for (const scheme of NAMING_SCHEMES) {
      const defaultDependentValue = 1;
      const defaultLaxValue = 10;

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
            buildVirtual(b, scheme.alias)
              .validate(virtualValidator)
              .ignoreUpdate(),
          ),
      ).getModel();

      const lax = defaultLaxValue + 10;
      let { data } = await Model.create(
        { lax, [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(data).toEqual({ dependent: defaultDependentValue + 1, lax });

      ({ data } = await Model.create(
        { [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(data).toEqual({
        dependent: defaultDependentValue + 1,
        lax: defaultLaxValue,
      });

      const lax2 = defaultLaxValue + 10;
      ({ data } = await Model.create(
        { lax: lax2, [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(data).toEqual({ dependent: defaultDependentValue + 1, lax: lax2 });

      const lax3 = data.lax + 10;
      const { data: updates } = await Model.update(
        data,
        { lax: lax3, [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(updates).toEqual({ lax: lax3 });

      const { error } = await Model.update(
        data,
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(error).toEqual({ isNothingToUpdate: true, payload: null });
    }
  });

  describe('grouped ignore', () => {
    it('should properly handle grouped ignore rule', async () => {
      const IGNORE = 'IGNORE';

      for (const scheme of NAMING_SCHEMES) {
        const defaultLaxValue = 'default_lax_value';
        const defaultLax1Value = 'default_lax_1_value';
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>(
          (b) =>
            b
              .field(b.lax('lax', defaultLaxValue))
              .field(b.lax('lax_1', defaultLax1Value))
              .field(
                b
                  .dependent('dependent', 'virtualField')
                  .default(defaultDependentValue)
                  .resolve((ctx: any) => ctx.values.dependent + 1),
              )
              .field(
                buildVirtual(b, scheme.alias).validate(() => ({ valid: true })),
              ),
          {
            ignore: {
              fields: [scheme.publicKey, 'lax'],
              resolver: (ctx: any) => ctx.input.lax === IGNORE,
            },
          },
        ).getModel();

        let lax1 = 'lax_1';
        let virtualValue = 'virtual_value';

        let { data } = await Model.create(
          { lax: IGNORE, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        );

        expect(data).toEqual({
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
          lax_1: lax1,
        });

        let lax = 'some lax value';
        lax1 = 'lax_1';

        ({ data } = await Model.create(
          { lax, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        ));

        expect(data).toEqual({
          dependent: defaultDependentValue + 1,
          lax,
          lax_1: lax1,
        });

        const previous = {
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
          lax_1: defaultLax1Value,
        };

        lax1 = 'lax_1';
        virtualValue = 'virtual_value';

        let { data: updates } = await Model.update(
          previous,
          { lax: IGNORE, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        );

        expect(updates).toEqual({ lax_1: lax1 });

        lax = 'some lax value';
        lax1 = 'lax_1';

        ({ data: updates } = await Model.update(
          previous,
          { lax, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        ));

        expect(updates).toEqual({
          dependent: previous.dependent + 1,
          lax,
          lax_1: lax1,
        });
      }
    });
  });

  describe('grouped ignoreUpdate', () => {
    it('should properly handle grouped ignoreUpdate rule', async () => {
      const IGNORE = 'IGNORE';

      for (const scheme of NAMING_SCHEMES) {
        const defaultLaxValue = 'default_lax_value';
        const defaultLax1Value = 'default_lax_1_value';
        const defaultDependentValue = 1;

        const Model = new Schema<any, any>(
          (b) =>
            b
              .field(
                b
                  .dependent('dependent', 'virtualField')
                  .default(defaultDependentValue)
                  .resolve((ctx: any) => ctx.values.dependent + 1),
              )
              .field(b.lax('lax', defaultLaxValue))
              .field(b.lax('lax_1', defaultLax1Value))
              .field(
                buildVirtual(b, scheme.alias).validate(() => ({ valid: true })),
              ),
          {
            ignoreUpdate: {
              fields: ['lax', scheme.publicKey],
              resolver: (ctx: any) => ctx.rawInput.lax === IGNORE,
            },
          },
        ).getModel();

        let lax = IGNORE;
        let lax1 = 'lax_1';
        let virtualValue = 'some value';

        let { data } = await Model.create(
          { lax, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        );

        expect(data).toEqual({
          dependent: defaultDependentValue + 1,
          lax,
          lax_1: lax1,
        });

        lax = 'some lax value';
        lax1 = 'lax_1';

        ({ data } = await Model.create(
          { lax, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        ));

        expect(data).toEqual({
          dependent: defaultDependentValue + 1,
          lax,
          lax_1: lax1,
        });

        const previous = {
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
          lax_1: defaultLax1Value,
        };

        lax1 = 'lax_1';
        virtualValue = 'updated value';

        let { data: updates } = await Model.update(
          previous,
          { lax: IGNORE, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        );

        expect(updates).toEqual({ lax_1: lax1 });

        lax = 'some lax value';
        lax1 = 'lax_1';

        ({ data: updates } = await Model.update(
          previous,
          { lax, lax_1: lax1, [scheme.publicKey]: virtualValue },
          {},
        ));

        expect(updates).toEqual({
          dependent: previous.dependent + 1,
          lax,
          lax_1: lax1,
        });
      }
    });
  });
});
