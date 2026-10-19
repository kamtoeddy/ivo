---
title: Virtual Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Virtual Fields

A virtual field is a purely input field whose value may or may not be provided, used to trigger a
change in one or more dependent fields. It is not part of the output.

A virtual field must have at least one dependent field that depends on it, and it must declare
`.allow()` or `.validate()` before most other rules unlock.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
type Input = { rawEmail?: string };
type Output = { email: string };

const UserModel = new Schema<Input, Output>((b) =>
b
.field(
b
.virtual("rawEmail")
.validate((value) => ({ valid: value.includes("@") }))
.sanitize((value) => value.trim().toLowerCase()),
)
.field(
b
.dependent("email", "rawEmail")
.default("")
.resolve(({ input }) => input.rawEmail ?? ""),
),
).getModel();

const { data } = await UserModel.create({ rawEmail: "Ada@Example.COM" });
console.log(data);
}

main();`}
/>

## Allowed values

`.allow()` restricts the accepted input values. As with `.validate()`, it unlocks the rest of the
builder.

```ts
b.virtual("role").allow(["admin", "editor"]);
```

The array must contain at least two values. Use `.allowError()` to customize the error.

## Validation

`.validate()` is the most common way to unlock the virtual builder. The validator receives the raw
input value and returns a validation result.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
type Input = { code?: string };
type Output = { normalizedCode: string };

const Model = new Schema<Input, Output>((b) =>
b
.field(
b
.virtual("code")
.validate((value) =>
/^[A-Z]{3}$/.test(value)
? true
: { valid: false, reason: "Code must be 3 uppercase letters" },
),
)
.field(
b
.dependent("normalizedCode", "code")
.default("")
.resolve(({ input }) => input.code ?? ""),
),
).getModel();

const { error } = await Model.create({ code: "ab" });
console.log(error?.payload);
}

main();`}
/>

## Alias

Use `.alias(name)` to accept the virtual value under a different input key. `.alias()` can be set
before or after `.allow()` / `.validate()`, but only once.

```ts
b.virtual("rawEmail").validate(isEmail).alias("email");
```

## Sanitizer

Use `.sanitize()` to transform the raw input before validation and before dependents resolve.
Sanitizers run after `.allow()` / `.validate()` has been declared.

```ts
b.virtual("rawEmail")
  .validate(isEmail)
  .sanitize((value) => value.trim().toLowerCase());
```

## Conditional required

A virtual field can be made conditionally required with `.required(handler)`, just like a lax
field.

```ts
b.virtual("promoCode")
  .validate(isPromoCode)
  .required(({ input }) => [
    input.role === "affiliate" && !input.promoCode,
    "Promo code is required for affiliates",
  ]);
```

## Ignore rules

- `.ignore(resolver)` — ignore the field during create and update when the resolver returns `true`.
- `.ignoreInit()` — ignore the field only during create.
- `.ignoreUpdate()` — ignore the field only during update.

These unlock only after `.allow()` or `.validate()`.

## Hooks

Virtual fields support `onSuccess` and `onFailure` listeners.

## API summary

| Method                  | Description                                             |
| ----------------------- | ------------------------------------------------------- |
| `virtual(name)`         | Create a virtual input field.                           |
| `alias(name)`           | Accept the value under a different input key.           |
| `allow(values)`         | Restrict to at least two allowed values.                |
| `allowError(error)`     | Customize the not-allowed error.                        |
| `validate(validator)`   | Validator for the input value.                          |
| `reValidate(validator)` | Validator used during updates.                          |
| `required(handler)`     | Conditionally require the field.                        |
| `sanitize(sanitizer)`   | Transform the raw input before validation / dependents. |
| `ignore(resolver)`      | Ignore field when resolver returns `true`.              |
| `ignoreInit()`          | Ignore field at creation.                               |
| `ignoreUpdate()`        | Ignore field at update.                                 |
| `onFailure(handler)`    | Listener invoked after validation failure.              |
| `onSuccess(handler)`    | Listener invoked after a successful create/update.      |
