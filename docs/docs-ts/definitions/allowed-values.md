---
title: Allowed Values
---

import TsPlayground from '@site/src/components/TsPlayground';

# Allowed Values

Restrict a field to a fixed set of allowed values instead of writing a custom
validator.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

const UserModel = new Schema(
(b) =>
b
.field(b.required("role").allow(["admin", "editor", "viewer"]))
.field(b.lax("status", "draft").allow(["draft", "published", "archived"])),
).getModel();

const { data, error } = await UserModel.create({ role: "admin" });
console.log("created:", data);

const { error: invalid } = await UserModel.create({ role: "superuser" });
console.log("invalid reason:", invalid?.payload?.role?.reason);
console.log("invalid metadata:", invalid?.payload?.role?.metadata);
`}
/>

## Availability

| Field type  | `.allow()` | Notes                                           |
| ----------- | ---------- | ----------------------------------------------- |
| `required`  | Yes        | Can replace `.validate()` entirely.             |
| `lax`       | Yes        | Can replace `.validate()` entirely.             |
| `virtual`   | Yes        | Must call `.validate()` first, then `.allow()`. |
| `dependent` | No         | Dependents are resolved, not input-restricted.  |
| `constant`  | No         | Constants are set by the schema.                |

## Rules

- The array must contain at least **two** values.
- The static default of a `lax` field must be one of the allowed values.
- Values are compared using the configured `equalityDepth`.
- Use `.allowError()` to customize the error message.

```ts
b.required("role")
  .allow(["admin", "editor"])
  .allowError((value, allowed) => `"${value}" is not a valid role`);
```
