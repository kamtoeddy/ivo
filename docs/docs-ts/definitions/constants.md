---
title: Constant Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Constant Fields

A constant field is a purely output field whose value is set once at creation and never changes.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
  const OrderModel = new Schema<any, { id: string; total: number }>((b) =>
  b
  .field(b.constant("id", () => "order-123"))
  .field(b.lax("total", 0)),
  ).getModel();

  const { data } = await OrderModel.create({ id: "ignored", total: 99 });
  console.log(data);

}

main();`}
/>

## Rules

- A constant must have either a static value or a resolver function.
- Constants are ignored when provided as input.
- Constants cannot be updated.
- They support `onDelete` and `onSuccess` listeners.
