---
title: "Dependents"
sidebar_position: 2
---

# Dependents

Dependent properties are resolved automatically from other properties. Any external attempt to change their value is ignored; their value is solely modifiable through their resolver function.

## Defining a dependent

A dependent field requires three rules:

- `default`: a value or function used as (or to generate) the default value
- `dependsOn`: at least one other property or virtual the field depends on
- `resolver`: a sync/async function that produces the new value when a dependency changes

```ts
import { Schema, type IvoSummary } from "ivo";

type Input = {
  firstName: string;
  lastName: string;
};

type Output = {
  firstName: string;
  fullName: string;
  lastName: string;
};

const userSchema = new Schema<Input, Output>({
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
  fullName: {
    default: "",
    dependsOn: ["firstName", "lastName"],
    resolver({ ctx: { firstName, lastName } }) {
      return `${firstName} ${lastName}`;
    },
  },
});
```

> **Note:** The resolver runs after post-validation and virtual sanitizers. If the resolver throws during creation, the value becomes `null`; during an update, the property is ignored.

## Default values

Dependent properties must have a `default` value. This value is used when the resolver has not yet run or when no dependencies are provided.

```ts
const schema = new Schema({
  total: {
    default: 0,
    dependsOn: ["price", "quantity"],
    resolver({ ctx }) {
      return ctx.price * ctx.quantity;
    },
  },
});
```

## Readonly

Dependent properties can be made readonly with `readonly: true`. Once resolved, their value cannot be changed externally.

```ts
const schema = new Schema({
  completedAt: {
    default: "",
    readonly: true,
    dependsOn: "isComplete",
    resolver({ ctx }) {
      return ctx.isComplete ? new Date() : "";
    },
  },
});
```

## Limitations

- Dependent properties **cannot be required**.
- They cannot have their own `validator`; validation should happen on the properties they depend on.

## API summary

| Option    | Type                     | Required | Description                                                       |
| --------- | ------------------------ | -------- | ----------------------------------------------------------------- |
| default   | `any \| function`        | Yes      | Default value or resolver for the property.                       |
| dependsOn | `string \| string[]`     | Yes      | Property or properties the field depends on.                      |
| resolver  | `function`               | Yes      | Function that computes the value when dependencies change.        |
| readonly  | `true`                   | No       | Prevents external updates after the value is resolved.            |
| onDelete  | `function \| function[]` | No       | Handler(s) invoked when the model instance is deleted.            |
| onSuccess | `function \| function[]` | No       | Handler(s) invoked after a successful create or update operation. |
