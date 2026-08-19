---
title: Dependent Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Dependent Fields

A dependent field is a purely output field whose value changes whenever at least one field it
depends on is provided and accepted.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
type Input = { firstName?: string; lastName?: string };
type Output = { firstName: string; lastName: string; fullName: string };

const UserModel = new Schema<Input, Output>((b) =>
b
.field(b.lax("firstName", ""))
.field(b.lax("lastName", ""))
.field(
b
.dependent("fullName", ["firstName", "lastName"])
.default("")
.resolve(({ ctx }) => \`\${ctx.firstName} \${ctx.lastName}\`.trim()),
),
).getModel();

const { data } = await UserModel.create({ firstName: "Ada", lastName: "Lovelace" });
console.log(data);
}

main();`}
/>

## Default values

A dependent field must declare a default value. Use `.default(value)` for a static default or
`.default(resolver).resolve(...)` for a computed default.

```ts
b.dependent("slug", "name")
  .default("")
  .resolve(({ input }) => input.name!.toLowerCase().replace(/\s+/g, "-"));
```

The static default is used when none of the dependencies are provided or accepted. The resolver
receives the operation context and is invoked whenever a dependency is accepted.

## Readonly

`readonly()` is only available when the default is a static value. It locks the field once its
value has diverged from the default.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
type Input = { firstName?: string; lastName?: string };
type Output = { firstName: string; lastName: string; fullName: string };

const UserModel = new Schema<Input, Output>((b) =>
b
.field(b.lax("firstName", "Ada"))
.field(b.lax("lastName", "Lovelace"))
.field(
b
.dependent("fullName", ["firstName", "lastName"])
.default("")
.resolve(({ ctx }) => \`\${ctx.firstName} \${ctx.lastName}\`.trim())
.readonly(),
),
).getModel();

const { data: created } = await UserModel.create({});
console.log("created:", created);

const { data: updated } = await UserModel.update(created!, { firstName: "Grace" });
console.log("updated:", updated);
}

main();`}
/>

With a resolver default, `readonly()` is not offered; the resolver itself controls when the field
is recomputed.

## Hooks

Dependent fields support `onDelete` and `onSuccess` listeners.

## API summary

| Method                         | Description                                                              |
| ------------------------------ | ------------------------------------------------------------------------ |
| `dependent(name, dependsOn)`   | Create a dependent field. `dependsOn` can be a single field or an array. |
| `default(value)`               | Static default used when no dependency is triggered.                     |
| `default(resolver).resolve(r)` | Computed default; resolver runs when a dependency is accepted.           |
| `readonly()`                   | Locks after divergence; only with a static default.                      |
| `onDelete(handler)`            | Listener invoked by `model.delete`.                                      |
| `onSuccess(handler)`           | Listener invoked after a successful create/update.                       |
