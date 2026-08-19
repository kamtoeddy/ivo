---
title: Life Cycles
---

import TsPlayground from '@site/src/components/TsPlayground';

# Life Cycles

`ivo` exposes hooks at different stages of an operation.

## Operation context

Handlers receive a context object with useful operation state:

```ts
{
  input: Input; // sanitized input values
  rawInput: Input; // original input values
  values: Output; // current output values (including defaults and resolved dependents)
  isUpdate: boolean;
  summary: IvoSummary<Input, Output>;
}
```

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
