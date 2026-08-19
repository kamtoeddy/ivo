---
title: "Constants"
sidebar_position: 1
---

# Constants

Constant properties are set once during creation and never change afterward. Any value provided for them during creation or updates is ignored.

## Defining a constant

A constant field requires two rules:

- `constant: true`
- `value`: a fixed value or a sync/async function that generates the value

```ts
import { Schema, type SetterFnData } from "ivo";

type Input = {
  userName: string;
};

type Output = {
  dateJoined: Date;
  id: string;
  role: string;
};

type SetterData = SetterFnData<Input, Output, CtxOptions>;

const userSchema = new Schema<Input, Output>({
  dateJoined: { constant: true, value: () => new Date() },
  id: {
    constant: true,
    value: ({ ctx }: SetterData) => `${ctx.userName}-${Date.now()}`,
  },
  role: { constant: true, value: "user" },
  userName: { required: true, validator: validateUserName },
});
```

> **Note:** If the `value` function throws, the constant's value becomes `null`.

## Lifecycle hooks

Constants support only `onDelete` and `onSuccess` lifecycle handlers:

- `onSuccess` runs once after a successful creation.
- `onDelete` runs once when the model's `delete` method is invoked.

```ts
const schema = new Schema({
  id: {
    constant: true,
    value: generateId,
    onSuccess: ({ ctx }) => console.log("created", ctx.id),
    onDelete: (data) => console.log("deleted", data.id),
  },
});
```

## API summary

| Option    | Type                     | Required | Description                                               |
| --------- | ------------------------ | -------- | --------------------------------------------------------- |
| constant  | `true`                   | Yes      | Marks the property as constant.                           |
| value     | `any \| function`        | Yes      | Fixed value or resolver that produces the constant value. |
| onDelete  | `function \| function[]` | No       | Handler(s) invoked when the model instance is deleted.    |
| onSuccess | `function \| function[]` | No       | Handler(s) invoked after a successful create operation.   |
