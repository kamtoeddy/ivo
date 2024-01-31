## Extending Schemas

For any schema that inherits from another, call the extend method on the parent schema like in the example below

> N.B:
>
> - To overwrite a property, you just do it in the property definitions.
> - [postValidate](../index.md#postvalidate), [shouldUpdate](../index.md#shouldupdate) and lifecycles are the only options that are not inherited

Example:

```ts
const baseSchema = new Schema(
  {
    id: { constant: true, value: generateId },
    dob: { default: '' },
    name: { default: '' }
  },
  { timestamps: true }
);

const childSchema = baseSchema.extend(
  {
    name: { default: 'default-name' }
  },
  { timestamps: { createdAt: 'cAt' }, remove: 'dob' }
);
```
