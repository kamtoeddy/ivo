# Dependent Properties

Any external attempt to modify the value of the a dependent property will be ignored; making it's value solely modifiable via their resolver functions.

One such property `must` have the following rules:

- **default**: This is a [value or function](./defaults.md#default-values) that will be used as (or used to generate a) default value for the said property
- **dependsOn**: At least one other property or side effect of your model the said property should depend on. It could be a string or an array of properties.
- **resolver**: A function (sync or async) that would be invoked to generate the said property's new value when any of it's dependencies changes

It could aslo be used in combination with other rules like [**readonly**](./readonly.md#readonly-properties), [**life cycle handlers**](../life-cycles.md#life-cycle-handlers), etc. but **`cannot be required`**.

Example:

```ts
import { Schema, type Summary } from 'ivo';

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
  fullName: {
    default: '',
    dependsOn: ['firstName', 'lastName'],
    resolver: getFullName
  },
  lastName: { required: true, validator: validateName }
});

function getFullName({ context }: Summary<Input, Output>) {
  const { firstName, lastName } = context;

  return `${firstName} ${lastName}`;
}
```
