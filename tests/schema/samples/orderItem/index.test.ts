import { beforeAll, describe, expect, it } from 'bun:test';

import { OrderItem } from '.';
import type { IOrderItem } from './types';

describe('Testing schema of Orderitem (inherited schema)', () => {
  let orderItem: IOrderItem;

  beforeAll(async () => {
    orderItem = (
      await OrderItem.create({
        id: '1',
        name: 'beer',
        costPrice: 5,
        price: 5,
        measureUnit: 'bottle',
        otherMeasureUnits: [
          { coefficient: 24, name: 'crate24' },
          { coefficient: 5, name: 'tray' },
          { coefficient: 12, name: 'crate' },
        ],
        _quantity: 100,
      })
    ).data!;
  });

  it('should have been created properly', () => {
    expect(orderItem).toMatchObject({
      id: '1',
      name: 'beer',
      costPrice: 5,
      measureUnit: 'bottle',
      otherMeasureUnits: [
        { coefficient: 12, name: 'crate' },
        { coefficient: 24, name: 'crate24' },
        { coefficient: 5, name: 'tray' },
      ],
      price: 5,
      quantity: 100,
    });
  });

  it('should not have properties of parent removed during inheritance', () => {
    const removedProps = [
      '_readOnlyNoInit',
      '_dependentReadOnly',
      '_sideEffectForDependentReadOnly',
    ];
    const props = Object.keys(orderItem);

    for (const prop of props) expect(removedProps.includes(prop)).toBe(false);
  });
});
