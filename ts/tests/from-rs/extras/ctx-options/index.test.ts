import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

type Supplier = {
  id: number;
  name: string;
  companyName: string;
  contactEmail: string;
  status: 'active' | 'phaseOut' | 'suspended';
};

const SUPPLIERS_DB: Map<number, Supplier> = new Map(
  Array.from({ length: 5 }, (_, i) => {
    const num = i + 1;
    const name = `supplier-${num}`;
    const companyName = `company_${num}`;

    return [
      num,
      {
        id: num,
        name,
        companyName,
        contactEmail: `${name}@${companyName}.com`,
        status: 'phaseOut' as const,
      },
    ];
  }),
);

describe('extras.ctxOptions', () => {
  it('should properly update ctx options', async () => {
    type ProductInput = {
      name: string;
      sku: string;
      price: number;
      supplier: number;
    };
    type Product = {
      id: number;
      name: string;
      sku: string;
      price: number;
      supplier: number;
    };
    type ProductCtxOptions = { warnings: string[] };

    const ProductModel = new Schema<ProductInput, Product, ProductCtxOptions>(
      (b) =>
        b
          .field(b.constant('id', () => 1))
          .field(b.required('name').validate(() => ({ valid: true })))
          .field(b.required('sku').validate(() => ({ valid: true })))
          .field(b.required('price').validate(() => ({ valid: true })))
          .field(
            b
              .required('supplier')
              .validate(() => ({ valid: true }))
              .reValidate((id, ctx) => {
                const supplier = SUPPLIERS_DB.get(id);

                if (!supplier)
                  return { valid: false, reason: 'Supplier not found' };

                if (
                  supplier.status === 'phaseOut' ||
                  supplier.status === 'suspended'
                )
                  ctx.updateOptions({
                    warnings: [
                      ...ctx.options.warnings,
                      `warning: supplier ${id} is not currently active!`,
                    ],
                  });

                return { valid: true };
              }),
          ),
      {},
    ).getModel();

    const supplierNum = 2;

    const { data, options: options1 } = await ProductModel.create(
      {
        name: 'product_name',
        price: 1_000,
        sku: 'product_sku',
        supplier: supplierNum,
      },
      { warnings: [] },
    );

    expect(options1.warnings[0]).toBe(
      `warning: supplier ${supplierNum} is not currently active!`,
    );

    const supplierNum2 = 3;

    const { options: options2 } = await ProductModel.update(
      data!,
      { supplier: supplierNum2 },
      { warnings: [] },
    );

    expect(options2.warnings[0]).toBe(
      `warning: supplier ${supplierNum2} is not currently active!`,
    );
  });
});
