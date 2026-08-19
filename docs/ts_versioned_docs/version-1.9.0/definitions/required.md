---
title: "Required"
sidebar_position: 4
---

# Required

Required properties must be provided during creation. They must have a validator and cannot have a default value when they are strictly required.

## Defining a required field

```ts
import { Schema } from "ivo";

const userSchema = new Schema({
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
});
```

## Validation

Required fields must have a `validator`. They may also have a `reValidator` for secondary validation.

```ts
const schema = new Schema({
  email: {
    required: true,
    validator: validateEmail,
    reValidator: makeSureEmailIsUnique,
  },
});
```

## Allowed values

Use `allow` to restrict the accepted values. The value is checked against the allowed list before being passed to the validator.

```ts
const schema = new Schema({
  role: { required: true, allow: ["admin", "user"], validator: validateRole },
});
```

Custom errors work the same as for lax fields:

```ts
const schema = new Schema({
  role: {
    required: true,
    allow: {
      values: ["admin", "user"],
      error: "Invalid role provided",
    },
    validator: validateRole,
  },
});
```

## Conditionally required

Instead of `required: true`, you can provide a function that decides whether the field is required for the current operation.

```ts
type RequiredError = string | { reason?: string; metadata?: object | null };
```

The function may return:

- `boolean`
- `[boolean, RequiredError]`
- `Promise<boolean | [boolean, RequiredError]>`

```ts
import { Schema, type IvoSummary } from "ivo";

type Book = {
  bookId: string;
  isPublished: boolean;
  price: number | null;
};

const bookSchema = new Schema<Book>({
  bookId: { required: true, validator: validateBookId },
  isPublished: { default: false, validator: validateBoolean },
  price: {
    default: null,
    required({ ctx: { isPublished, price } }: IvoSummary<Book>) {
      const isRequired = price == null && isPublished;
      return [isRequired, "A price is required to publish a book!"];
    },
    validator: validatePrice,
  },
});
```

> **Notes:**
>
> - If no required error is provided, `[propertyName] is required!` is used.
> - If the required function returns nothing, the operation proceeds with `required: false`.
> - If the required function throws, the operation proceeds with `required: false`.

Conditionally required fields may have a `default` value.

## Readonly

Required fields cannot be strictly readonly (`readonly: true` with `required: true`). However, a conditionally required field may be readonly.

## Ignore rules

Required fields support `ignore`, `ignoreInit`, and `ignoreUpdate`.

## Lifecycle hooks

Required fields support `onDelete`, `onFailure`, and `onSuccess` handlers.

## API summary

| Option        | Type                     | Required | Description                                                       |
| ------------- | ------------------------ | -------- | ----------------------------------------------------------------- |
| required      | `true \| function`       | Yes      | Marks the field as required or makes it conditionally required.   |
| validator     | `function`               | Yes\*    | Primary validator. Required when `required: true`.                |
| reValidator   | `function`               | No       | Secondary validator.                                              |
| allow         | `any[] \| object`        | No       | Allowed values and optional custom error.                         |
| requiredError | `string \| function`     | No       | Custom error for conditional requirement.                         |
| readonly      | `true \| 'lax'`          | No       | Only allowed with conditional requirement.                        |
| ignore        | `function`               | No       | Determines whether input should be ignored.                       |
| ignoreInit    | `true`                   | No       | Ignores the field during creation.                                |
| ignoreUpdate  | `true`                   | No       | Ignores the field during updates.                                 |
| onDelete      | `function \| function[]` | No       | Handler(s) invoked when the model instance is deleted.            |
| onFailure     | `function \| function[]` | No       | Handler(s) invoked after a failed create or update operation.     |
| onSuccess     | `function \| function[]` | No       | Handler(s) invoked after a successful create or update operation. |
