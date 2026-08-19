---
title: "Extending Schemas"
sidebar_position: 5
---

# Extending Schemas

Schemas can inherit fields and options from a parent schema by calling the `extend` method.

## Basic example

```ts
const baseSchema = new Schema(
  {
    id: { constant: true, value: generateId },
    dob: { default: "" },
    name: { default: "" },
  },
  { timestamps: true },
);

const extendedSchema = baseSchema.extend(
  {
    name: { default: "default-name" },
  },
  { timestamps: { createdAt: "cAt" }, remove: "dob" },
);
```

## Rules

- To overwrite a property, redefine it in the extension definitions.
- The parent schema's options are inherited by default.
- `postValidate`, `shouldUpdate`, and lifecycle options are **not** inherited.
- Use `useParentOptions: false` to prevent option inheritance.
- Use the `remove` option to drop parent properties.
