# Dependent Properties

If set to **`true`**, any external attempt to modify the of the said property will be ignored; making it's value solely modifiable via the life cycle listeners and side effects.

One such property `must` have the following rules:

- **default**: This is a [value or function](../../../v1.4.10/schema/definition/defaults.md#default-values) that will be used as (or used to generate a) default value for the said property
- **dependsOn**: Atleast one other property or side effect of your model the said property should depend on. It could be a string or an array of properties.
- **resolver**: A function (sync or async) that would be invoked to generate the said property's new value when any of it's dependencies changes

It could aslo be used in combination with other rules like [**readonly**](../../../v1.4.10/schema/definition/readonly.md#readonly-properties), [**life cycle listeners**](./schema/definition/life-cycles.md#life-cycle-listeners), etc. but **`cannot be required`**.

> Out of the box, dependent is assumed to be **`false`** for every property

Example:

```ts
import { Schema, type GetSummary } from "clean-schema";

type Input = { firstName: string; lastName: string };
type Output = { firstName: string; fullName: string; lastName: string };

type Summary = GetSummary<Input, Output>;

const userSchema = new Schema<Input, Output>({
  firstName: { required: true, validator: validateName },
  fullName: {
    default: "",
    dependent: true,
    dependsOn: ["firstName", "lastName"],
    resolver: generateFullName,
  },
  lastName: { required: true, validator: validateName },
});

function generateFullName({ context }: Summary) {
  const { firstName, lastName } = context;

  return `${firstName} ${lastName}`;
}
```