# Dependent Properties

If set to **`true`**, any external attempt to modify the value of the said property will be ignored; making it's value solely modifiable via the life cycle listeners and side effects.

One such property `must` have the following rules:

- **default**: This is a [value or function](../../../v3.0.0/schema/definition/defaults.md#default-values) that will be used as (or used to generate a) default value for the said property
- **dependsOn**: Atleast one other property or side effect of your model the said property should depend on. It could be a string or an array of properties.
- **resolver**: A function (sync or async) that would be invoked to generate the said property's new value when any of it's dependencies changes

It could aslo be used in combination with other rules like [**readonly**](../../../v1.4.10/schema/definition/readonly.md#readonly-properties), [**life cycle handlers**](./life-cycles.md#life-cycle-handlers), etc. but **`cannot be required`**.

> Out of the box, dependent is assumed to be **`false`** for every property

Example:

```ts
import { Schema, type Summary } from 'clean-schema';

type Input = {
  firstName: string;
  lastName: string;
};
type Output = {
  firstName: string;
  fullName: string;
  lastName: string;
};

type ISummary = Summary<Output, Input>;

const userSchema = new Schema<Output, Input>({
  firstName: { required: true, validator: validateName },
  fullName: {
    default: '',
    dependent: true,
    dependsOn: ['firstName', 'lastName'],
    resolver: getFullName
  },
  lastName: { required: true, validator: validateName }
});

function getFullName({ context }: ISummary) {
  const { firstName, lastName } = context;

  return `${firstName} ${lastName}`;
}
```
