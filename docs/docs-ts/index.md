---
slug: /
title: Getting Started
---

import TsPlayground from '@site/src/components/TsPlayground';

# Getting Started

`ivo` for TypeScript lets you define a schema with a fluent field builder, then derive a model with
`create`, `update`, and `delete` methods.

## Installation

```bash
npm i ivo
```

## Defining a schema

A schema is created with `new Schema((b) => ...)` where `b` is a `FieldBuilder`. Fields are declared
with `b.required(...)`, `b.lax(...)`, `b.constant(...)`, `b.dependent(...)`, or `b.virtual(...)`,
then passed to `b.field(...)`.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

type UserInput = {
email: string | null;
phoneNumber: string | null;
username: string;
};

type User = {
id: string;
createdAt: Date;
email: string | null;
phoneNumber: string | null;
updatedAt: Date | null;
username: string;
usernameLastUpdatedAt: Date | null;
};

const isEmailOrPhoneRequired = ({ input }: any) => [
!input.email && !input.phoneNumber,
"Provide either an email or phone number",
];

const validateEmail = (value: string | null) =>
value && value.includes("@")
? true
: { valid: false, reason: "Invalid email" };

const validatePhoneNumber = (value: string | null) =>
value && value.length >= 5
? true
: { valid: false, reason: "Invalid phone number" };

const validateUsername = (value: string) =>
value.length >= 3
? true
: { valid: false, reason: "Username must be at least 3 characters" };

const userSchema = new Schema<UserInput, User>(
(b) =>
b
.field(b.constant("id", () => "user-123"))
.field(
b
.lax("email", null)
.required(isEmailOrPhoneRequired)
.validate(validateEmail),
)
.field(
b
.lax("phoneNumber", null)
.required(isEmailOrPhoneRequired)
.validate(validatePhoneNumber),
)
.field(
b
.required("username")
.validate(validateUsername)
.ignoreUpdate(({ previousValues }) => {
const last = previousValues.usernameLastUpdatedAt;
if (!last) return false;

            const thirtyDays = 2_592_000_000;
            return new Date().getTime() - last.getTime() < thirtyDays;
          }),
      )
      .field(
        b
          .dependent("usernameLastUpdatedAt", "username")
          .default(null)
          .resolve(({ isUpdate }) => (isUpdate ? new Date() : null)),
      ),

{ timestamps: true },
);

const UserModel = userSchema.getModel();

const { data, error } = await UserModel.create({
email: "john.doe@mail.com",
username: "john_doe",
});
console.log("created:", data);

const user = { ...data!, updatedAt: new Date() };
const { data: updated } = await UserModel.update(user, { username: "johndoe" });
console.log("updated:", updated);
`}
/>

## Model methods

The model returned by `schema.getModel()` exposes async methods:

| Method   | Description                                                          |
| -------- | -------------------------------------------------------------------- |
| `create` | Creates a new instance from a partial input.                         |
| `update` | Applies a partial update to an existing instance.                    |
| `delete` | Triggers all registered `onDelete` listeners on the provided entity. |

## Creating an entity

Unknown properties and output-only properties (`constant`, `dependent`, `timestamps`) are ignored
automatically.

```ts
const { data, error } = await UserModel.create({
  email: "john.doe@mail.com",
  id: 5, // ignored because 'id' is constant
  name: "John Doe", // ignored because it is not on the schema
  username: "john_doe",
  updatedAt: new Date(), // ignored because it is a timestamp
  usernameLastUpdatedAt: new Date(), // ignored because it is dependent
});

if (error) return handleError(error);

console.log(data);
// {
//   id: '...',
//   createdAt: Date,
//   email: 'john.doe@mail.com',
//   phoneNumber: null,
//   updatedAt: null,
//   username: 'john_doe',
//   usernameLastUpdatedAt: null
// }
```

## Updating an entity

```ts
const user = await usersDb.findByID(id);
if (!user) return handleError({ message: "User not found" });

const { data, error } = await UserModel.update(user, {
  usernameLastUpdatedAt: new Date(), // dependent -> ignored
  id: 75, // constant -> ignored
  age: 34, // not on schema -> ignored
  username: "johndoe",
});

if (error) return handleError(error);

console.log(data);
// {
//   updatedAt: Date,
//   username: 'johndoe',
//   usernameLastUpdatedAt: Date
// }
```

## Field categories

- [Allowed values](./definitions/allowed-values.md)
- [Constant fields](./definitions/constants.md)
- [Default values](./definitions/defaults.md)
- [Dependent fields](./definitions/dependents.md)
- [Extending schemas](./definitions/extend-schemas.md)
- [Lax fields](./definitions/lax.md)
- [Readonly fields](./definitions/readonly.md)
- [Required fields](./definitions/required.md)
- [Virtual fields](./definitions/virtuals.md)

## Schema options

The second argument to `new Schema` accepts options:

```ts
new Schema((b) => ..., {
  equalityDepth: 1,
  sanitizeError: (payload, ctxOptions) => payload,
  onDelete: [listener],
  onSuccess: [listener],
  postValidate: { fields: ['email', 'phoneNumber'], validator: ... },
  ignore: { fields: ['secret'], handler: () => true },
  ignoreUpdate: { fields: ['email'], handler: () => true },
  required: { fields: ['email', 'phoneNumber'], handler: ... },
  timestamps: true,
});
```

| Option          | Description                                                                 |
| --------------- | --------------------------------------------------------------------------- |
| `equalityDepth` | Nesting depth used to compare values during updates (default: `1`).         |
| `sanitizeError` | Transform the error payload before it is returned.                          |
| `onDelete`      | Global listener(s) invoked by `model.delete`.                               |
| `onSuccess`     | Global listener(s) invoked after a successful create/update.                |
| `postValidate`  | Cross-field validation configuration (`fields` + `validator`).              |
| `ignore`        | Ignore input fields when the handler returns `true`.                        |
| `ignoreUpdate`  | Ignore update values for the listed fields when the handler returns `true`. |
| `required`      | Cross-field required constraint (`fields` + `handler`).                     |
| `timestamps`    | Enable `createdAt`/`updatedAt` (boolean or `{ createdAt?, updatedAt? }`).   |

See [Life cycles](./life-cycles.md) and [Validators](./validators.md) for more.

## Extending a schema

Use `.extend()` to create a new schema that inherits fields and options from the parent:

```ts
const AdminSchema = userSchema.extend<AdminInput, AdminOutput>(
  (b) => b.field(b.required("role").validate(validateRole)),
  { useParentOptions: true },
);
```

Set `useParentOptions: false` to drop parent options and start from the provided options only.
Fields can be removed with the `remove` option.
