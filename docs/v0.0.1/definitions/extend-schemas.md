## Extending Schemas

For any schema that inherits from another, call the extend method on the parent schema like in the example below

> N.B:
>
> - To overwrite a property, you just do it in the property definitions.
> - From v4, all options are inherited except lifecycles and [shouldUpdate](../index.md#shouldupdate) options

Example:

```ts
const baseSchema = new Schema(
  {
    id: { constant: true, value: generateId },
    dob: { default: '' },
    name: { default: '' }
  },
  { timestamps: true }
)

const childSchema = baseSchema.extend(
  {
    name: { default: 'default-name' }
  },
  { timestamps: { createdAt: 'cAt' }, remove: 'dob' }
)
```
