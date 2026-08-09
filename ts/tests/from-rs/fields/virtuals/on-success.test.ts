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

describe('fields.virtual.onSuccess', () => {
  it('should trigger onSuccess handlers if virtual is provided at creation', async () => {
    for (const scheme of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
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
              .onSuccess((ctx: any) => {
                triggeredWith = ctx.rawInput[scheme.publicKey];
              }),
          ),
      ).getModel();

      const { data, handleSuccess } = await Model.create(
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(data).toEqual({
        dependent: defaultDependentValue + 1,
        lax: defaultLaxValue,
      });

      await handleSuccess?.();

      expect(triggeredWith).toBe('virtual_value');
    }
  });

  it('should trigger onSuccess handlers if virtual is provided during updates', async () => {
    for (const scheme of NAMING_SCHEMES) {
      let triggeredWith: string | undefined;
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
              .onSuccess((ctx: any) => {
                triggeredWith = ctx.rawInput[scheme.publicKey];
              }),
          ),
      ).getModel();

      const previous = {
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
      };

      const { data: updates, handleSuccess } = await Model.update(
        previous,
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(updates).toEqual({ dependent: defaultDependentValue + 1 });

      await handleSuccess?.();

      expect(triggeredWith).toBe('virtual_value');
    }
  });

  it('should not trigger onSuccess handlers if virtual is not provided', async () => {
    for (const scheme of NAMING_SCHEMES) {
      let triggered = false;
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
              .onSuccess(() => {
                triggered = true;
              }),
          ),
      ).getModel();

      let { data, handleSuccess } = await Model.create({}, {});

      expect(data).toEqual({
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
      });

      await handleSuccess?.();

      const lax = defaultLaxValue + 10;
      ({ data, handleSuccess } = await Model.create({ lax }, {}));

      expect(data).toEqual({ dependent: defaultDependentValue, lax });

      await handleSuccess?.();

      const lax2 = data.lax + 10;
      const { data: updates, handleSuccess: handleUpdateSuccess } =
        await Model.update(data, { lax: lax2 }, {});

      expect(updates).toEqual({ lax: lax2 });

      await handleUpdateSuccess?.();

      expect(triggered).toBe(false);
    }
  });

  it('should not trigger onSuccess handlers if virtual is provided but ignored by ignore fn', async () => {
    for (const scheme of NAMING_SCHEMES) {
      let triggered = false;
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
              .ignore(() => true)
              .onSuccess(() => {
                triggered = true;
              }),
          ),
      ).getModel();

      let { data, handleSuccess } = await Model.create(
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(data).toEqual({
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
      });

      await handleSuccess?.();

      const lax = defaultLaxValue + 10;
      ({ data, handleSuccess } = await Model.create(
        { lax, [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(data).toEqual({ dependent: defaultDependentValue, lax });

      await handleSuccess?.();

      const lax2 = data.lax + 10;
      const { data: updates, handleSuccess: handleUpdateSuccess } =
        await Model.update(
          data,
          { lax: lax2, [scheme.publicKey]: 'virtual_value' },
          {},
        );

      expect(updates).toEqual({ lax: lax2 });

      await handleUpdateSuccess?.();

      expect(triggered).toBe(false);
    }
  });

  it('should not trigger onSuccess handlers if virtual is provided but ignored by ignoreInit', async () => {
    for (const scheme of NAMING_SCHEMES) {
      let triggered = false;
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
              .ignoreInit()
              .onSuccess(() => {
                triggered = true;
              }),
          ),
      ).getModel();

      let { data, handleSuccess } = await Model.create(
        { [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(data).toEqual({
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
      });

      await handleSuccess?.();

      const lax = defaultLaxValue + 10;
      ({ data, handleSuccess } = await Model.create(
        { lax, [scheme.publicKey]: 'virtual_value' },
        {},
      ));

      expect(data).toEqual({ dependent: defaultDependentValue, lax });

      await handleSuccess?.();

      expect(triggered).toBe(false);
    }
  });

  it('should not trigger onSuccess handlers if virtual is provided but ignored by ignoreUpdate', async () => {
    for (const scheme of NAMING_SCHEMES) {
      let triggered = false;
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
              .ignoreUpdate()
              .onSuccess(() => {
                triggered = true;
              }),
          ),
      ).getModel();

      const lax = defaultLaxValue + 10;
      const { data: updates, handleSuccess } = await Model.update(
        { dependent: defaultDependentValue, lax: defaultLaxValue },
        { lax, [scheme.publicKey]: 'virtual_value' },
        {},
      );

      expect(updates).toEqual({ lax });

      await handleSuccess?.();

      expect(triggered).toBe(false);
    }
  });

  describe('o.onSuccess', () => {
    it('should trigger grouped onSuccess handlers if virtual is provided at creation', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(buildVirtual(b, scheme.alias).validate(virtualValidator)),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        const { data, handleSuccess } = await Model.create(
          { [scheme.publicKey]: 'virtual_value' },
          {},
        );

        expect(data).toEqual({
          dependent: defaultDependentValue + 1,
          lax: defaultLaxValue,
        });

        await handleSuccess?.();

        expect(triggered).toBe(true);
      }
    });

    it('should not trigger grouped onSuccess handlers if virtual is not provided at creation', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(buildVirtual(b, scheme.alias).validate(virtualValidator)),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        let { data, handleSuccess } = await Model.create({}, {});

        expect(data).toEqual({
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
        });

        await handleSuccess?.();

        const lax = defaultLaxValue + 10;
        ({ data, handleSuccess } = await Model.create({ lax }, {}));

        expect(data).toEqual({ dependent: defaultDependentValue, lax });

        await handleSuccess?.();

        expect(triggered).toBe(false);
      }
    });

    it('should not trigger grouped onSuccess handlers if virtual is provided but ignored by ignore fn', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(
                buildVirtual(b, scheme.alias)
                  .validate(virtualValidator)
                  .ignore(() => true),
              ),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        const { data, handleSuccess } = await Model.create(
          { [scheme.publicKey]: 'virtual_value' },
          {},
        );

        expect(data).toEqual({
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
        });

        await handleSuccess?.();

        const lax = data.lax + 10;
        const { data: updates, handleSuccess: handleUpdateSuccess } =
          await Model.update(
            data,
            { lax, [scheme.publicKey]: 'virtual_value' },
            {},
          );

        expect(updates).toEqual({ lax });

        await handleUpdateSuccess?.();

        expect(triggered).toBe(false);
      }
    });

    it('should not trigger grouped onSuccess handlers if virtual is provided but ignored by ignoreInit fn at creation', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(
                buildVirtual(b, scheme.alias)
                  .validate(virtualValidator)
                  .ignoreInit(),
              ),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        const { data, handleSuccess } = await Model.create(
          { [scheme.publicKey]: 'virtual_value' },
          {},
        );

        expect(data).toEqual({
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
        });

        await handleSuccess?.();

        expect(triggered).toBe(false);
      }
    });

    it('should trigger grouped onSuccess handlers if virtual is provided during updates', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(buildVirtual(b, scheme.alias).validate(virtualValidator)),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        const previous = {
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
        };

        const { data: updates, handleSuccess } = await Model.update(
          previous,
          { [scheme.publicKey]: 'virtual_value' },
          {},
        );

        expect(updates).toEqual({ dependent: defaultDependentValue + 1 });

        await handleSuccess?.();

        expect(triggered).toBe(true);
      }
    });

    it('should not trigger grouped onSuccess handlers if virtual is not provided during updates', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(buildVirtual(b, scheme.alias).validate(virtualValidator)),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        const previous = {
          dependent: defaultDependentValue,
          lax: defaultLaxValue,
        };

        const lax = defaultLaxValue + 10;
        const { data: updates, handleSuccess } = await Model.update(
          previous,
          { lax },
          {},
        );

        expect(updates).toEqual({ lax });

        await handleSuccess?.();

        expect(triggered).toBe(false);
      }
    });

    it('should not trigger grouped onSuccess handlers if virtual is provided but ignored by ignoreUpdate fn', async () => {
      for (const scheme of NAMING_SCHEMES) {
        let triggered = false;
        const defaultDependentValue = 1;
        const defaultLaxValue = 10;

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
              .field(
                buildVirtual(b, scheme.alias)
                  .validate(virtualValidator)
                  .ignoreUpdate(),
              ),
          {
            onSuccess: {
              fields: [scheme.publicKey] as never,
              resolver: () => {
                triggered = true;
              },
            },
          },
        ).getModel();

        const lax = defaultLaxValue + 10;
        const { data: updates, handleSuccess } = await Model.update(
          { dependent: defaultDependentValue, lax: defaultLaxValue },
          { lax, [scheme.publicKey]: 'virtual_value' },
          {},
        );

        expect(updates).toEqual({ lax });

        await handleSuccess?.();

        expect(triggered).toBe(false);
      }
    });
  });
});
