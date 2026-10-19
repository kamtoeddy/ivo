---
title: Life Cycles
---

import TsPlayground from '@site/src/components/TsPlayground';

# Life Cycles

`ivo` exposes hooks at different stages of an operation.

## Operation context

Validators, resolvers and listeners receive a context object (`ctx`) with the operation's state:

```ts
{
  input: Partial<Input>; // sanitized input values
  rawInput: Partial<Input>; // original input values
  values: Output; // current values (on update: previous values merged with the changes)
  isUpdate: boolean;
  changes: Partial<Output> | null; // on update, the changes; null on create
  previousValues: Output | null; // on update, the values before the update; null on create
  options: CtxOptions; // a fresh copy on every read
  updateOptions: (options: Partial<CtxOptions>) => void; // not given to listeners
}
```

Constant, default and `ignoreUpdate` resolvers receive a smaller context (`input`, `rawInput`,
`options` and `updateOptions`, plus `values` for constants and `previousValues` for `ignoreUpdate`).

The context is frozen, and its properties are getters. Read them directly or destructure them:

```ts
.validate((value, ctx) => {
  const { isUpdate, values } = ctx; // ✅
  // ...
});
```

:::caution[Since v2.1.0]
The getters are defined on the context's prototype, not on the object itself, so `{ ...ctx }`,
`Object.keys(ctx)` and `JSON.stringify(ctx)` no longer list them. Copy the properties you need
instead, e.g. `{ values: ctx.values, isUpdate: ctx.isUpdate }`.
:::

## Global listeners

Set them on schema options:

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
  const UserModel = new Schema(
  (b) => b.field(b.lax("name", "Anonymous")),
  {
  onSuccess: (summary) => console.log("success:", summary.values),
  onDelete: (data) => console.log("deleted:", data),
  },
  ).getModel();

  const { data } = await UserModel.create({ name: "Ada" });
  await UserModel.delete(data!);

}

main();`}
/>

## Field listeners

Field builders support `onSuccess`, `onFailure`, and `onDelete`:

```ts
b.required("username")
  .validate(validateUsername)
  .onSuccess((summary) => console.log("username validated", summary))
  .onFailure((summary) => console.log("username failed", summary));
```

## Post-validation

Use `postValidate` to run cross-field validation after individual field validators:

```ts
new Schema((b) => ..., {
  postValidate: {
    fields: ['email', 'phoneNumber'],
    validator: ({ input }) => [
      !input.email && !input.phoneNumber,
      'Provide email or phone number',
    ],
  },
});
```

## Custom context options

Pass extra data through every operation:

```ts
const UserSchema = new Schema<Input, Output, { db: Database }>((b) => ..., {
  // schema definition
});

const UserModel = UserSchema.getModel();

const { data, error } = await UserModel.create(input, { db: usersDb });
```

Inside validators and resolvers, access the options via the context:

```ts
b.constant("id", ({ options }) => options.db.nextId());
```
