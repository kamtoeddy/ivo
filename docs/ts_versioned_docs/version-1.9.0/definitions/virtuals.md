---
title: "Virtuals"
sidebar_position: 5
---

# Virtuals

Virtual properties are helper properties used during operations but they do not appear on model instances. They are useful for transforming input before it reaches dependent properties.

## Defining a virtual

A virtual field requires:

- `virtual: true`
- a `validator`
- at least one dependent property that uses `dependsOn`

```ts
import { Schema } from "ivo";

type UserInput = {
  blockUser: boolean;
};

type User = {
  isBlocked: boolean;
};

const UserModel = new Schema<UserInput, User>({
  blockUser: { virtual: true, validator: validateBoolean },
  isBlocked: {
    default: false,
    dependsOn: "blockUser",
    resolver: ({ ctx }) => ctx.blockUser,
  },
}).getModel();

// creating
const user = await UserModel.create({ blockUser: true, name: "Peter" });

console.log(user); // { isBlocked: true }
```

The `name` property is ignored because it is not in the schema. `blockUser` is virtual, so it does not appear on the output, but its value still drives the dependent `isBlocked` property.

## Validation

Virtuals must have a `validator`. They may also have a `reValidator` and can be conditionally required.

```ts
const schema = new Schema({
  avatarFile: {
    virtual: true,
    validator: validateFile,
    reValidator: validateFileSize,
  },
});
```

## Allowed values

Virtuals support `allow` to restrict accepted input values.

```ts
const schema = new Schema({
  status: {
    virtual: true,
    allow: ["active", "inactive"],
    validator: validateStatus,
  },
});
```

## Aliases

An alias is an extra external name for a virtual property.

- Only virtuals can have aliases.
- An alias must be a string.
- It cannot be the name of another property or virtual unless it is the name of a dependent property of that virtual.

```ts
type Input = {
  quantity?: number;
  _virtualQuantity?: number;
};

type Output = {
  quantity: number;
};

type Aliases = {
  quantity: number;
};

const StoreItem = new Schema<Input, Output, Aliases>({
  quantity: {
    default: 0,
    dependsOn: "_virtualQuantity",
    resolver: ({ ctx }) => ctx._virtualQuantity,
  },
  _virtualQuantity: {
    alias: "quantity",
    virtual: true,
    validator: validateVirtualQuantity,
  },
}).getModel();

// these are equivalent
const { data: item1 } = await StoreItem.create({ _virtualQuantity: 100 });
const { data: item2 } = await StoreItem.create({ quantity: 100 });

console.log(item1, item2); // { quantity: 100 } { quantity: 100 }
```

If both the virtual and its alias are provided, the last value wins.

> **Note:** Do not access virtuals by their aliases in the operation context. Aliases only work when passed to `create` and `update`.

## Sanitizer

Use a sanitizer when a virtual may exist in more than one form. It runs after the last validation step and can transform the virtual value before dependent resolvers run.

```ts
import { Schema, type IvoSummary } from "ivo";

type FileMetadata = { size: number; url: string };

type Input = {
  file: File | FileMetadata;
  name: string;
};

type Output = {
  id: string;
  metadata: FileMetadata;
  name: string;
};

const FileModel = new Schema<Input, Output>({
  id: { constant: true, value: generateID },
  name: { required: true, validator: validateName },
  metadata: {
    default: { size: 0, url: "" },
    dependsOn: "file",
    resolver: ({ ctx }) => ctx.file as FileMetadata,
  },
  file: {
    virtual: true,
    sanitizer: sanitizeFile,
    validator: validateFile,
  },
}).getModel();

async function sanitizeFile({ ctx: { file } }: IvoSummary<Input, Output>) {
  const { size, url } = await uploadFile(file as File);
  return { size, url } as FileMetadata;
}
```

> **Note:** If the sanitizer throws, the value before sanitization is used.

## Ignore rules

Virtuals support `ignore`, `ignoreInit`, and `ignoreUpdate`.

## Lifecycle hooks

Virtuals support `onFailure` and `onSuccess` handlers.

## Limitations

- Virtuals cannot be dependent.
- Virtuals cannot have a `default` value.
- Virtuals cannot be strictly required, but they can be conditionally required.
- Virtuals cannot be readonly.

## API summary

| Option       | Type                     | Required | Description                                                       |
| ------------ | ------------------------ | -------- | ----------------------------------------------------------------- |
| virtual      | `true`                   | Yes      | Marks the property as virtual.                                    |
| validator    | `function`               | Yes      | Primary validator.                                                |
| reValidator  | `function`               | No       | Secondary validator.                                              |
| alias        | `string`                 | No       | External alias for the virtual.                                   |
| sanitizer    | `function`               | No       | Transforms the virtual after validation.                          |
| allow        | `any[] \| object`        | No       | Allowed values and optional custom error.                         |
| required     | `function`               | No       | Conditionally requires the virtual.                               |
| ignore       | `function`               | No       | Determines whether input should be ignored.                       |
| ignoreInit   | `true`                   | No       | Ignores the virtual during creation.                              |
| ignoreUpdate | `true`                   | No       | Ignores the virtual during updates.                               |
| onFailure    | `function \| function[]` | No       | Handler(s) invoked after a failed create or update operation.     |
| onSuccess    | `function \| function[]` | No       | Handler(s) invoked after a successful create or update operation. |
