## Extending Schemas

For any schema that inherits from another, call the extend method on the parent schema like in the example below

> N.B:
>
> - To overwrite a property, you just do it in the property definitions.
> - Options are not inherited

```ts
const userSchema = new Schema(
  {
    firstName: { required: true, validator: validateName },
    fullName: {
      default: "",
      dependent: true,
      dependsOn: ["firstName", "lastName"],
      resolver: getFullName,
    },
    isBlocked: { default: false, validator: validateBoolean },
    id: { readonly: true, validator: validateId },
    lastName: { required: true, validator: validateName },
    lastSeen: { default: "", shouldInit: false },
    password: { required: true, validator: validatePassword },
    role: role: { constant: true, value: "User" },
  },
  { timestamps: true }
);

const adminSchema = userSchema.extend(
  {
    role: { constant: true, value: "Admin" },
  },
  { timestamps: true, remove: "dob" }
);
```
