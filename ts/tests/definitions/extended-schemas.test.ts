import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';

describe('Extended Schema', () => {
  describe('Options', () => {
    describe('timestamps', () => {
      it('should respect "timestamps" option from baseSchema if enabled', async () => {
        const Model = new Schema<any>(
          (b, m) => b.field(m.constant('id').value(1)),
          { timestamps: { updatedAt: 'u_At' } },
        )
          .extend<any>((b, m) => b.field(m.lax('name').default('')))
          .getModel();

        const { data, error } = await Model.create({});

        expect(error).toBeNull();
        expect(data).toMatchObject({ id: 1, name: '' });

        expect(data.createdAt).toBeDefined();
        expect(data.u_At).toBeDefined();
      });

      it('should respect "timestamps" option from baseSchema if not enabled', async () => {
        const Model = new Schema<any>(
          (b, m) => b.field(m.constant('id').value(1)),
          { timestamps: false },
        )
          .extend<any>((b, m) => b.field(m.lax('name').default('')))
          .getModel();

        const { data, error } = await Model.create({});

        expect(error).toBeNull();
        expect(data).toMatchObject({ id: 1, name: '' });
        expect(data.createdAt).toBeUndefined();
        expect(data.updatedAt).toBeUndefined();
      });

      it('should respect overwritten "timestamps" option from baseSchema', async () => {
        const Model = new Schema<any>(
          (b, m) => b.field(m.constant('id').value(1)),
          { timestamps: { createdAt: 'c_at', updatedAt: 'uAt' } },
        )
          .extend<any>((b, m) => b.field(m.lax('name').default('')))
          .getModel();

        const { data, error } = await Model.create({});

        expect(error).toBeNull();
        expect(data).toMatchObject({ id: 1, name: '' });
        expect(data.c_at).toBeDefined();
        expect(data.uAt).toBeDefined();
      });
    });

    describe('useParentOptions', () => {
      it('should respect "useParentOptions" option if enabled', async () => {
        const options = [undefined, true];
        for (const useParentOptions of options) {
          const Model = new Schema<any>(
            (b, m) => b.field(m.constant('id').value(1)),
            { timestamps: { updatedAt: 'u_At' } },
          )
            .extend<any>((b, m) => b.field(m.lax('name').default('')), {
              useParentOptions,
            })
            .getModel();

          const { data, error } = await Model.create({});

          expect(error).toBeNull();
          expect(data).toMatchObject({ id: 1, name: '' });
          expect(data.createdAt).toBeDefined();
          expect(data.u_At).toBeDefined();
        }
      });

      it('should respect "useParentOptions" option if enabled', async () => {
        const Model = new Schema<any>(
          (b, m) => b.field(m.constant('id').value(1)),
          { timestamps: { updatedAt: 'u_At' } },
        )
          .extend<any>((b, m) => b.field(m.lax('name').default('')), {
            useParentOptions: false,
          })
          .getModel();

        const { data, error } = await Model.create({});

        expect(error).toBeNull();
        expect(data).toMatchObject({ id: 1, name: '' });
        expect(data.createdAt).toBeUndefined();
        expect(data.updatedAt).toBeUndefined();
        expect(data.u_At).toBeUndefined();
      });
    });
  });
});
