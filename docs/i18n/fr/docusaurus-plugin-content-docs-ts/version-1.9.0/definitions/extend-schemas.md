---
title: "Extension des schémas"
---

## Extension des schémas

Pour tout schéma qui hérite d'un autre, appelez la méthode extend sur le schéma parent comme dans l'exemple ci-dessous.

> N.B.:
>
> - Pour écraser une propriété, il suffit de le faire dans les définitions de propriétés.
> - [postValidate](../index.md#postvalidate), [shouldUpdate](../index.md#shouldupdate-défaut--true) et les cycles de vie sont les seules options qui ne sont pas héritées.

Exemple:

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
