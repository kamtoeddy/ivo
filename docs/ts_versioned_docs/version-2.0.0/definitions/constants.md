---
title: Constant Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Constant Fields

A constant field is a purely output field whose value is set once at creation and never changes.
Input values and updates are ignored.

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

## Default values

The constant value can be static or a resolver function. Resolvers receive the creation context
and are evaluated once, when the entity is created.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const Model = new Schema<any, { id: string; code: number }>((b) =>
b
.field(b.constant("id", ({ options }) => "user-" + options.userId))
.field(b.constant("code", 42)),
).getModel();

const { data } = await Model.create({}, { userId: 7 });
console.log(data);
}

main();`}
/>

If a resolver throws, the field value becomes `null`.

## Hooks

Constant fields support `onDelete` and `onSuccess` listeners.

```ts
b.constant("id", () => "user-123")
  .onDelete((entity) => console.log("deleted", entity.id))
  .onSuccess((summary) => console.log("created", summary.values.id));
```

## API summary

| Method                  | Description                                              |
| ----------------------- | -------------------------------------------------------- |
| `constant(name, value)` | Create a constant field with a static value or resolver. |
| `onDelete(handler)`     | Listener invoked by `model.delete`.                      |
| `onSuccess(handler)`    | Listener invoked after a successful create/update.       |
