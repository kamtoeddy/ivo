---
title: Lax Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Lax Fields

A lax field is both an input and output field whose value may or may not be provided at creation.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

const UserModel = new Schema<any, { role: string }>((b) =>
b.field(b.lax("role", "user")),
).getModel();

const { data } = await UserModel.create({});
console.log(data);
`}
/>

## Rules

- A lax field must have either a static default value or a resolver function.
- It may have a `validator` and/or `reValidate`.
- It may be conditionally required via `required(handler)`.
- It supports `ignore`, `ignoreInit`, and `ignoreUpdate` rules.
- It supports `readonly()` when the default is static.
- It may have `onDelete`, `onSuccess`, and `onFailure` listeners.

## Default resolvers

The default value can be a static value or a function that receives the operation context:

```ts
b.lax("timezone", "UTC");
b.lax("locale", ({ ctx }) => ctx.locale ?? "en");
```
