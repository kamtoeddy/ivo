---
title: Virtual Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Virtual Fields

A virtual field is a purely input field whose value may or may not be provided, used to trigger a
change in one or more fields that depend on it.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

type Input = { rawEmail?: string };
type Output = { email: string };

const UserModel = new Schema<Input, Output>((b) =>
b
.field(
b
.virtual("rawEmail")
.sanitize((value) => value.trim().toLowerCase())
.validate((value) => ({ valid: value.includes("@") }))
.alias("email"),
)
.field(
b
.dependent("email", "rawEmail")
.default("")
.resolve(({ input }) => input.rawEmail ?? ""),
),
).getModel();

const { data } = await UserModel.create({ email: "Ada@Example.COM" });
console.log(data);
`}
/>

## Rules

- A virtual field must have one or more dependent fields depending on it.
- It must have a `validate` or `allow` rule.
- It may have a `reValidate` rule.
- It may have a `sanitizer` to transform the raw input before dependents resolve.
- It may have an `alias` used as the input field name.
- It supports `ignore`, `ignoreInit`, and `ignoreUpdate` rules.
- It may have `onSuccess` and `onFailure` listeners.
