---
title: "Lax"
sidebar_position: 3
---

# Lax

Lax properties are optional fields that carry a default value. They may be provided during creation or updates, and they support validation, allowed values, and lifecycle hooks.

## Defining a lax field

A lax field is defined with a `default` value. It is not strictly required and does not use `constant`, `dependsOn`, or `virtual`.

```ts
import { Schema, type SetterFnData } from "ivo";

type Input = {
  favoriteColor: string;
  userName: string;
};

type Output = {
  favoriteColor: string;
  userName: string;
};

type SetterData = SetterFnData<Input, Output, CtxOptions>;

const userSchema = new Schema({
  favoriteColor: { default: "indigo", validator: validateColor },
  userName: {
    default: ({ ctx }: SetterData) => "",
    validator: validateUserName,
  },
});
```

> **Note:** If the `default` function throws, the property's value becomes `null`.

## Default values

The `default` rule provides a fallback value. It can be a literal value or a function that returns a value at runtime.

```ts
const schema = new Schema({
  status: { default: "pending", validator: validateStatus },
});
```

## Allowed values

Use `allow` to restrict the values accepted by a lax field. If a validator is also provided, the value is first checked against the allowed list before being passed to the validator.

```ts
const schema = new Schema({
  role: { default: "user", allow: ["admin", "moderator", "user"] },
});
```

For a custom error message, use the object form:

```ts
const schema = new Schema({
  role: {
    default: "user",
    allow: {
      values: ["admin", "moderator", "user"],
      error: "Invalid role provided",
    },
  },
});
```

The error can be a string, an `InputFieldError`, or a function that receives the provided value and allowed values:

```ts
type NotAllowedError =
  | string
  | InputFieldError
  | ((valueProvided: any, allowedValues: any[]) => string | InputFieldError);
```

> **Note:** If the `NotAllowedError` function throws, the default error message is used.

## Readonly

Lax fields support two readonly modes via the `readonly` rule.

### `readonly: 'lax'`

The field is not required at creation or during updates. Once its value differs from the default, it no longer accepts updates.

```ts
const schema = new Schema({
  receiptNumber: {
    default: null,
    readonly: "lax",
    validator: validateReceipt,
  },
});
```

### `readonly: true`

The field is required at creation and never allows updates. Combine it with `shouldInit: false` to allow a single update later.

```ts
const schema = new Schema({
  isComplete: {
    default: false,
    readonly: true,
    shouldInit: false,
    validator: validateBoolean,
  },
});
```

> **Note:** Readonly lax fields can be conditionally required, but they cannot be strictly required (`required: true`).

## Validation

Lax fields can have one primary `validator` and one secondary `reValidator`. See the [Validators](../validators) page for details.

```ts
const schema = new Schema({
  email: {
    default: null,
    validator: validateEmail,
    reValidator: makeSureEmailIsUnique,
  },
});
```

## Ignore rules

Control when input is ignored with `ignore`, `ignoreInit`, or `ignoreUpdate`:

```ts
const schema = new Schema({
  promoCode: {
    default: null,
    validator: validatePromoCode,
    ignore({ ctx }) {
      return ctx.isGuest;
    },
  },
});
```

## Lifecycle hooks

Lax fields support `onDelete`, `onFailure`, and `onSuccess` handlers.

## API summary

| Option       | Type                     | Required | Description                                                         |
| ------------ | ------------------------ | -------- | ------------------------------------------------------------------- |
| default      | `any \| function`        | Yes      | Default value or resolver for the property.                         |
| allow        | `any[] \| object`        | No       | Allowed values and optional custom error.                           |
| readonly     | `true \| 'lax'`          | No       | Makes the field immutable; `'lax'` locks once changed from default. |
| validator    | `function`               | No       | Primary validator.                                                  |
| reValidator  | `function`               | No       | Secondary validator that runs after the primary validator.          |
| required     | `function`               | No       | Conditionally requires the field.                                   |
| ignore       | `function`               | No       | Determines whether input should be ignored.                         |
| ignoreInit   | `true`                   | No       | Ignores the field during creation.                                  |
| ignoreUpdate | `true`                   | No       | Ignores the field during updates.                                   |
| onDelete     | `function \| function[]` | No       | Handler(s) invoked when the model instance is deleted.              |
| onFailure    | `function \| function[]` | No       | Handler(s) invoked after a failed create or update operation.       |
| onSuccess    | `function \| function[]` | No       | Handler(s) invoked after a successful create or update operation.   |
