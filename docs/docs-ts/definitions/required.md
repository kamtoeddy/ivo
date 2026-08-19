---
title: Required Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Required Fields

A required field is both an input and output field whose value must be provided at creation.
You must call `.validate()` or `.allow()` before the field can be built.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema<any, { username: string }>((b) =>
b.field(
b
.required("username")
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

## Allowed values

`.allow()` is a concise alternative to a validator and satisfies the required-field rule.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) => b.field(b.required("role").allow(["admin", "editor", "viewer"])),
).getModel();

const { data, error } = await UserModel.create({ role: "admin" });
console.log("created:", data);

const { error: invalid } = await UserModel.create({ role: "superuser" });
console.log("invalid reason:", invalid?.payload?.role?.reason);
}

main();`}
/>

Use `.allowError()` to customize the error message:

```ts
b.required("role")
  .allow(["admin", "editor"])
  .allowError((value, allowed) => `"${value}" is not a valid role`);
```

## Validation

Use `.validate()` for create-time validation and `.reValidate()` for update-time validation. If
`reValidate` is omitted, the create validator is reused.

```ts
b.required("username")
  .validate(validateUsername)
  .reValidate(validateUsernameUpdate);
```

## Readonly and ignore updates

- `.readonly()` — permanently ignore the field during updates.
- `.ignoreUpdate(resolver)` — ignore updates when the resolver returns `true`.

These are mutually exclusive.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b.field(
b
.required("username")
.validate((value) =>
typeof value === "string" && value.length >= 3
? true
: { valid: false, reason: "Username too short" },
)
.ignoreUpdate(({ previousValues }) => previousValues.verified),
)
.field(b.lax("verified", false)),
).getModel();

const user = { username: "ada", verified: false };
const { data } = await UserModel.update(user, { username: "bob" });
console.log(data);
}

main();`}
/>

## Required error

Use `.requiredError()` to customize the message when the field is missing at creation.

```ts
b.required("email").requiredError("Email is required").validate(validateEmail);
```

## Hooks

Required fields support `onDelete`, `onSuccess`, and `onFailure` listeners.

## API summary

| Method                   | Description                                           |
| ------------------------ | ----------------------------------------------------- |
| `required(name)`         | Create a required field.                              |
| `allow(values)`          | Restrict to at least two allowed values.              |
| `allowError(error)`      | Customize the not-allowed error.                      |
| `validate(validator)`    | Validator for create (and update if no `reValidate`). |
| `reValidate(validator)`  | Validator used during updates.                        |
| `readonly()`             | Permanently ignore updates.                           |
| `ignoreUpdate(resolver)` | Ignore updates when resolver returns `true`.          |
| `requiredError(error)`   | Customize the missing-field error.                    |
| `onDelete(handler)`      | Listener invoked by `model.delete`.                   |
| `onFailure(handler)`     | Listener invoked after validation failure.            |
| `onSuccess(handler)`     | Listener invoked after a successful create/update.    |
