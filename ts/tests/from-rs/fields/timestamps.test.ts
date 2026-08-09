import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../src';

describe('fields.timestamps', () => {
  it('should respect createdAt timestamp with default name', async () => {
    const Model = new Schema<{ lax: number }, { createdAt: Date; lax: number }>(
      (b) => b.field(b.lax('lax', 1234)),
      {
        timestamps: { createdAt: true, updatedAt: false },
      },
    ).getModel();

    const lax = 400;
    const { data } = await Model.create({ lax }, {});

    expect(data?.lax).toBe(lax);
    expect(data?.createdAt).toBeInstanceOf(Date);

    const laxUpdate = 200;
    const { data: updates } = await Model.update(data!, { lax: laxUpdate }, {});

    expect(updates).toEqual({ lax: laxUpdate });
    expect((updates as { createdAt?: Date })?.createdAt).toBeUndefined();
  });

  it('should respect createdAt timestamp with custom name', async () => {
    const Model = new Schema<
      { lax: number },
      { customCreatedAt: Date; lax: number }
    >((b) => b.field(b.lax('lax', 1234)), {
      timestamps: { createdAt: 'customCreatedAt', updatedAt: false },
    }).getModel();

    const lax = 400;
    const { data } = await Model.create({ lax }, {});

    expect(data?.lax).toBe(lax);
    expect(data?.customCreatedAt).toBeInstanceOf(Date);

    const laxUpdate = 200;
    const { data: updates } = await Model.update(data!, { lax: laxUpdate }, {});

    expect(updates).toEqual({ lax: laxUpdate });
    expect(
      (updates as { customCreatedAt?: Date })?.customCreatedAt,
    ).toBeUndefined();
  });

  it('should respect updatedAt timestamp with default name', async () => {
    const Model = new Schema<{ lax: number }, { lax: number; updatedAt: Date }>(
      (b) => b.field(b.lax('lax', 1234)),
      { timestamps: { createdAt: false, updatedAt: { nullable: false } } },
    ).getModel();

    const lax = 400;
    const { data } = await Model.create({ lax }, {});

    expect(data?.lax).toBe(lax);
    expect(data?.updatedAt).toBeInstanceOf(Date);

    const laxUpdate = 200;
    const { data: updates } = await Model.update(data!, { lax: laxUpdate }, {});

    expect(updates?.lax).toBe(laxUpdate);
    expect(updates?.updatedAt).toBeDefined();
  });

  it('should respect updatedAt timestamp with custom name', async () => {
    const Model = new Schema<
      { lax: number },
      { customUpdatedAt: Date; lax: number }
    >((b) => b.field(b.lax('lax', 1234)), {
      timestamps: {
        createdAt: false,
        updatedAt: { key: 'customUpdatedAt', nullable: false },
      },
    }).getModel();

    const lax = 400;
    const { data } = await Model.create({ lax }, {});

    expect(data?.lax).toBe(lax);
    expect(data?.customUpdatedAt).toBeInstanceOf(Date);

    const laxUpdate = 200;
    const { data: updates } = await Model.update(data!, { lax: laxUpdate }, {});

    expect(updates?.lax).toBe(laxUpdate);
    expect(updates?.customUpdatedAt).toBeDefined();
  });

  it('should respect optional updatedAt timestamp with default name', async () => {
    const Model = new Schema<
      { lax: number },
      { lax: number; updatedAt: Date | null }
    >((b) => b.field(b.lax('lax', 1234)), {
      timestamps: { createdAt: false, updatedAt: { nullable: true } },
    }).getModel();

    const lax = 400;
    const { data } = await Model.create({ lax }, {});

    expect(data?.lax).toBe(lax);
    expect(data?.updatedAt).toBeNull();

    const laxUpdate = 200;
    const { data: updates } = await Model.update(data!, { lax: laxUpdate }, {});

    expect(updates?.lax).toBe(laxUpdate);
    expect(updates?.updatedAt).toBeInstanceOf(Date);

    const updated = Object.assign({}, data, updates);

    expect(updated.lax).toBe(updates!.lax!);
    expect(updated.updatedAt).toBe(updates!.updatedAt!);
  });

  it('should respect optional updatedAt timestamp with custom name', async () => {
    const Model = new Schema<
      { lax: number },
      { customUpdatedAt: Date | null; lax: number }
    >((b) => b.field(b.lax('lax', 1234)), {
      timestamps: {
        createdAt: false,
        updatedAt: { key: 'customUpdatedAt', nullable: true },
      },
    }).getModel();

    const lax = 400;
    const { data } = await Model.create({ lax }, {});

    expect(data?.lax).toBe(lax);
    expect(data?.customUpdatedAt).toBeNull();

    const laxUpdate = 200;
    const { data: updates } = await Model.update(data!, { lax: laxUpdate }, {});

    expect(updates?.lax).toBe(laxUpdate);
    expect(updates?.customUpdatedAt).toBeInstanceOf(Date);

    const updated = Object.assign({}, data, updates);

    expect(updated.lax).toBe(updates!.lax!);
    expect(updated.customUpdatedAt).toBe(updates!.customUpdatedAt!);
  });
});
