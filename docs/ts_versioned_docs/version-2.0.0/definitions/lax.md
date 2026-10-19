---
title: Lax Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Lax Fields

A lax field is both an input and output field whose value may or may not be provided at creation.
When missing, its default value is used.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema<any, { role: string }>((b) =>
b.field(b.lax("role", "user")),
).getModel();

const { data } = await UserModel.create({});
console.log(data);
}

main();`}
/>

## Allowed values

Restrict a lax field to a fixed set of values with `.allow()`. The array must contain at least two
values, and the static default must be one of them.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) => b.field(b.lax("status", "draft").allow(["draft", "published", "archived"])),
).getModel();

const { data, error } = await UserModel.create({ status: "published" });
console.log("created:", data);

const { error: invalid } = await UserModel.create({ status: "deleted" });
console.log("invalid reason:", invalid?.payload?.status?.reason);
}

main();`}
/>

Use `.allowError()` to customize the error message:

```ts
b.lax("status", "draft")
  .allow(["draft", "published"])
  .allowError((value, allowed) => `"${value}" is not a valid status`);
```

## Default values

The default value can be static or a resolver function. Resolvers receive the creation context:

```ts
b.lax("timezone", "UTC");
b.lax("locale", ({ options }) => options.locale ?? "en");
```

If a resolver throws, the field value becomes `null`.

## Validation

Use `.validate()` to run validators at creation and `.reValidate()` to run different validators on
update. If `reValidate` is omitted, the create validator is reused.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema<any, { username: string }>((b) =>
b.field(
b
.lax("username", "")
.validate((value) =>
value.length >= 3
? true
: { valid: false, reason: "Username must be at least 3 characters" },
),
),
).getModel();

const { data, error } = await UserModel.create({ username: "ab" });
console.log({ data, error: error?.payload });
}

main();`}
/>

## Conditional required

A lax field can be made conditionally required with `.required(handler)`. The handler receives the
operation context and returns a tuple `[isRequired, errorMessage]`.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b
.field(b.lax("email", null))
.field(
b
.lax("phoneNumber", null)
.required(({ input }) => [
!input.email,
"Provide a phone number when email is missing",
]),
),
).getModel();

const { error } = await UserModel.create({});
console.log(error?.payload);
}

main();`}
/>

`.required()` is mutually exclusive with `.readonly()`, `.ignoreInit()`, and `.ignoreUpdate()`.

## Readonly

`readonly()` is only available when the default is a static value. It locks the field once its
current value has diverged from the default.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const CodeModel = new Schema<{ code: string }, { code: string }>(
(b) => b.field(b.lax("code", "PENDING").readonly()),
).getModel();

const { data: created } = await CodeModel.create({ code: "ABC" });
console.log("created:", created);

const { data: updated } = await CodeModel.update(created, { code: "DEF" });
console.log("updated:", updated);

const { data } = await CodeModel.update(
{ ...created, ...updated },
{ code: "GHI" },
);
console.log("second:", data);
}

main();`}
/>

`.readonly()` is mutually exclusive with `.required()`, `.ignoreInit()`, and `.ignoreUpdate()`.

## Ignore rules

- `.ignore(resolver)` — ignore the field during create and update when the resolver returns `true`.
- `.ignoreInit()` — ignore the field only during create.
- `.ignoreUpdate()` — ignore the field only during update.

These are mutually exclusive with `.readonly()` and `.required()`.

```ts
b.lax("role", "guest").ignore(({ input }) => input.role === "admin");
```

## Hooks

Lax fields support `onDelete`, `onSuccess`, and `onFailure` listeners.

## API summary

| Method                    | Description                                           |
| ------------------------- | ----------------------------------------------------- |
| `lax(name, defaultValue)` | Create a lax field with a static default or resolver. |
| `allow(values)`           | Restrict to at least two allowed values.              |
| `allowError(error)`       | Customize the not-allowed error.                      |
| `validate(validator)`     | Validator for create (and update if no `reValidate`). |
| `reValidate(validator)`   | Validator used during updates.                        |
| `required(handler)`       | Conditionally require the field.                      |
| `readonly()`              | Lock after divergence; only with a static default.    |
| `ignore(resolver)`        | Ignore field when resolver returns `true`.            |
| `ignoreInit()`            | Ignore field at creation.                             |
| `ignoreUpdate()`          | Ignore field at update.                               |
| `onDelete(handler)`       | Listener invoked by `model.delete`.                   |
| `onFailure(handler)`      | Listener invoked after validation failure.            |
| `onSuccess(handler)`      | Listener invoked after a successful create/update.    |
