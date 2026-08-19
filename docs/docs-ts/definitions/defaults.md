---
title: Default Values
---

import TsPlayground from '@site/src/components/TsPlayground';

# Default Values

Default values fill missing input at creation time. They can be static or a
resolver function.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

const ItemModel = new Schema(
(b) =>
b
.field(b.constant("id", () => "item-1"))
.field(b.lax("name", "Anonymous"))
.field(b.lax("createdBy", ({ options }) => options.userId))
.field(
b
.dependent("slug", "name")
.default("")
.resolve(({ input }) => input.name!.toLowerCase().replace(/\\s+/g, "-")),
),
).getModel();

const { data } = await ItemModel.create({}, { userId: "u-123" });
console.log(data);
`}
/>

## Default value behavior

| Field type  | Default required? | Static / resolver | Notes                                                       |
| ----------- | ----------------- | ----------------- | ----------------------------------------------------------- |
| `lax`       | Yes               | Both              | Used when the field is missing from input at creation.      |
| `dependent` | Yes               | Both              | Static default is used when no dependency is triggered.     |
| `constant`  | Value required    | Both              | Set at creation; input/updates are ignored.                 |
| `required`  | No                | —                 | Must be provided by the caller.                             |
| `virtual`   | No                | —                 | Input-only; use a dependent field to materialize a default. |

## Resolver context

Default resolvers for `lax` and `dependent` fields receive the creation context:

```ts
{
  input: Partial<Input>;     // sanitized input values
  rawInput: Partial<Input>;  // original input values
  options: CtxOptions;       // operation context options
  updateOptions: (updates) => void;
}
```

If a resolver throws, the field value becomes `null`.
