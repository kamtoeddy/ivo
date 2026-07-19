# Dependent Properties

Any external attempt to modify the value of the a dependent property will be ignored; making it's value solely modifiable via their resolver functions.

One such property `must` have the following rules:

- **default**: This is a [value or function](./defaults.md#default-values) that will be used as (or used to generate a) default value for the said property
- **dependsOn**: At least one other property or [`virtual`](./virtuals.md#virtual-properties) of your model it should depend on. It could be a string or an array of properties.
- **resolver**: A function (sync or async) that would be invoked to generate the said property's new value when any of it's dependencies changes. This function is invoked after the last validation step (post-validaton) and [sanitizers](./virtuals.md#sanitizer) have been run.
  > N.B: if the resolver happens to throw an error, the value of the property will be `null` at creation but if this happens during an update, the property will be ignored

Dependent properties could also be used in combination with other rules like [**readonly**](./readonly.md#readonly-properties), [**life cycle handlers**](../life-cycles.md#life-cycle-handlers), etc. but **`cannot be required`**

Example:

```ts
import { Schema, type MutableSummary } from "ivo";

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
    default: "",
    dependsOn: ["firstName", "lastName"],
    resolver: resolveFullName,
  },
  lastName: { required: true, validator: validateName },
});

function resolveFullName({ context }: MutableSummary<Input, Output>) {
  const { firstName, lastName } = context;

  return `${firstName} ${lastName}`;
}
```
