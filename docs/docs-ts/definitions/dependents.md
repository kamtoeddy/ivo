---
title: Dependent Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Dependent Fields

A dependent field is a purely output field whose value changes whenever at least one field it
depends on changes -- see [when the resolver runs](#when-the-resolver-runs).

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

The default is used until the resolver runs (see [when the resolver runs](#when-the-resolver-runs)).
The resolver receives the operation context.

## When the resolver runs

- **On create**, a dependent resolves when at least one of its parents:
  - is a required or virtual field that was provided and accepted;
  - is a lax field that was provided and accepted with a value other than its static default, or
    whose default comes from a resolver (a function), provided or not;
  - is a dependent field that resolved to a value other than its default, or whose default comes
    from a resolver.
- **On update**, a dependent resolves when at least one of its parents:
  - is a virtual field that was provided and accepted (always, since it has no previous value);
  - is a required or lax field that was provided and accepted with a value other than its
    previous one;
  - is a dependent field that resolved to a value other than its previous one.

A dependent that resolves to the value it already had is not a change, so the dependents below it
don't resolve. Dependents resolve in dependency order, each one at most once per operation, after
all its parents: a dependent with several parents never resolves twice.

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
| `default(resolver).resolve(r)` | Computed default; resolver runs when a dependency changes.               |
| `readonly()`                   | Locks after divergence; only with a static default.                      |
| `onDelete(handler)`            | Listener invoked by `model.delete`.                                      |
| `onSuccess(handler)`           | Listener invoked after a successful create/update.                       |
