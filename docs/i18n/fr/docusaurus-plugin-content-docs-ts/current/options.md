---
title: Options de schéma
sidebar_position: 3
---

import TsPlayground from '@site/src/components/TsPlayground';

# Schema Options

The second argument to `new Schema((b) => ..., { ... })` configures schema-wide behavior. These options apply to every `create`, `update`, and `delete` operation.

:::note
Schema-level checks for `postValidate` run **after** individual field validators. Cross-field `required` constraints run during the required-field evaluation phase, before per-field validation. See [Life cycles](./life-cycles.md) and [Validators](./validators.md) for more on execution order.
:::

## `equalityDepth`

Nesting depth used when comparing values for equality during updates. Default: `1`.

A higher depth lets `ivo` detect changes inside nested objects and arrays, while `0` compares by reference and `1` compares one level deep.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
type Settings = { theme: string; notifications: boolean };
type User = { id: string; settings: Settings };

const UserModel = new Schema<{ settings: Settings }, User>(
(b) =>
b
.field(b.constant("id", () => "user-1"))
.field(
b
.lax("settings", { theme: "light", notifications: true })
.validate((value) =>
value && typeof value === "object"
? true
: { valid: false, reason: "Invalid settings" },
),
),
{ equalityDepth: 2 },
).getModel();

const user = {
id: "user-1",
settings: { theme: "dark", notifications: true },
};

const { data, error } = await UserModel.update(user, {
settings: { theme: "dark", notifications: false },
});

console.log({ data, error: error?.payload });
}

main();`}
/>

## `sanitizeError`

Transform the error payload before it is returned from `create` or `update`.

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
: { valid: false, reason: "Username is required" },
),
),
{
sanitizeError: (payload) => ({
message: "Validation failed",
errors: Object.entries(payload).map(([field, err]) => ({
field,
reason: err?.reason,
})),
}),
},
).getModel();

const { error } = await UserModel.create({});
console.log(error);
}

main();`}
/>

## `onDelete`

Global listener(s) invoked by `model.delete`. Receives the full entity and context options.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) => b.field(b.lax("name", "Anonymous")),
{
onDelete: (data) => console.log("deleted:", data),
},
).getModel();

const { data } = await UserModel.create({ name: "Ada" });
await UserModel.delete(data!);
}

main();`}
/>

## `onSuccess`

Global listener(s) invoked after a successful `create` or `update`. Can be a function or a grouped config that only runs when one of the listed fields is involved.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b
.field(b.lax("name", "Anonymous"))
.field(b.lax("email", null)),
{
onSuccess: [
(summary) => console.log("success:", summary.values),
{
fields: ["email"],
handler: (summary) =>
console.log("email changed:", summary.values.email),
},
],
},
).getModel();

await UserModel.create({ name: "Ada", email: "ada@example.com" });
await UserModel.update(
{ name: "Ada", email: "ada@example.com" },
{ email: "ada@new.com" },
);
}

main();`}
/>

## `postValidate`

Run cross-field validation after individual field validators have finished. Each config needs at least two fields and a validator.

The validator receives the operation context and may return `undefined`/`true`/`void` for success, or an object mapping field names to errors. It can also return sanitized values under the `validated` key for each field.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b
.field(b.lax("password", ""))
.field(b.lax("confirmPassword", "")),
{
postValidate: {
fields: ["password", "confirmPassword"],
validator: ({ input }) => {
if (input.password !== input.confirmPassword) {
return { confirmPassword: "Passwords do not match" };
}
},
},
},
).getModel();

const { error } = await UserModel.create({
password: "secret",
confirmPassword: "wrong",
});

console.log(error?.payload);
}

main();`}
/>

## `ignore`

Ignore input fields when the handler returns `true`. Only `lax` and `virtual` fields can be ignored at the schema level.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b
.field(b.lax("role", "guest"))
.field(b.lax("secret", null)),
{
ignore: {
fields: ["secret"],
handler: ({ input }) => input.role !== "admin",
},
},
).getModel();

const { data } = await UserModel.create({ role: "guest", secret: "xyz" });
console.log(data);
}

main();`}
/>

## `ignoreUpdate`

Ignore update values for the listed fields when the handler returns `true`. Works with `lax`, `required`, and `virtual` fields.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b
.field(b.lax("email", ""))
.field(b.lax("verified", false)),
{
ignoreUpdate: {
fields: ["email"],
handler: ({ previousValues }) => !previousValues.verified,
},
},
).getModel();

const user = { email: "old@example.com", verified: false };
const { data, error } = await UserModel.update(user, {
email: "new@example.com",
});

console.log({ data, error: error?.payload });
}

main();`}
/>

## `required`

Cross-field required constraint for `lax` and `virtual` fields. Use it when a field is only required depending on the value of another field.

The handler receives the operation context and returns an object mapping field names to errors, or `undefined` when no field is required.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
const UserModel = new Schema(
(b) =>
b
.field(b.lax("email", null))
.field(b.lax("phoneNumber", null)),
{
required: {
fields: ["email", "phoneNumber"],
handler({ input }) {
if (!input.email && !input.phoneNumber) {
return {
email: "Provide either an email or a phone number",
};
}
},
},
},
).getModel();

const { error } = await UserModel.create({});
console.log(error?.payload);
}

main();`}
/>

## `timestamps`

Enable `createdAt` and `updatedAt` automatically.

- `true` — uses `createdAt` and `updatedAt` keys.
- `{ createdAt?: boolean | string, updatedAt?: boolean | string | { key?: string, nullable?: boolean } }` — customize key names or disable one of them.

`updatedAt` is nullable by default on create.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
type User = {
id: string;
name: string;
createdAt: Date;
modifiedAt: Date;
};

const UserModel = new Schema<{ name: string }, User>(
(b) =>
b
.field(b.constant("id", () => "user-1"))
.field(
b
.required("name")
.validate((value) =>
typeof value === "string" && value.length >= 1
? true
: { valid: false, reason: "Name is required" },
),
),
{
timestamps: {
createdAt: "createdAt",
updatedAt: { key: "modifiedAt", nullable: false },
},
},
).getModel();

const { data } = await UserModel.create({ name: "Ada" });
console.log(data);
}

main();`}
/>

## Summary

```ts
new Schema((b) => ..., {
  equalityDepth: 1,
  sanitizeError: (payload, ctxOptions) => payload,
  onDelete: [listener],
  onSuccess: [listener],
  postValidate: { fields: ["a", "b"], validator: ... },
  ignore: { fields: ["secret"], handler: () => true },
  ignoreUpdate: { fields: ["email"], handler: () => true },
  required: { fields: ["email", "phone"], handler: ... },
  timestamps: true,
});
```

| Option          | Description                                                                 |
| --------------- | --------------------------------------------------------------------------- |
| `equalityDepth` | Nesting depth for value comparisons during updates.                         |
| `sanitizeError` | Transform the error payload before returning it.                            |
| `onDelete`      | Global listener(s) for `model.delete`.                                      |
| `onSuccess`     | Global listener(s) after a successful create/update.                        |
| `postValidate`  | Cross-field validation after per-field validators.                          |
| `ignore`        | Ignore input fields when the handler returns `true`.                        |
| `ignoreUpdate`  | Ignore update values for the listed fields when the handler returns `true`. |
| `required`      | Cross-field required constraint for lax/virtual fields.                     |
| `timestamps`    | Enable `createdAt`/`updatedAt` automatically.                               |
