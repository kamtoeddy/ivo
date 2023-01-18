# Dependent Properties

If set to **`true`**, any external attempt to modify the of the said property will be ignored; making it's value solely modifiable via the life cycle listeners and side effects.

One such property `must` have a [default value or setter](./defaults.md#default-values) and can be used in combination with other rules like [**readonly**](./readonly.md#readonly-properties), [**life cycle listeners**](../life-cycles.md#life-cycle-listeners), etc. but **`cannot be required`**.

> Out of the box, dependent is assumed to be **`false`** for every property

Example:

```js
const { Schema } = require("clean-schema");

const userSchema = new Schema({
  firstName: {
    required: true,
    onChange: onNameChange,
    validator: validateName,
  },
  fullName: {
    default: "",
    dependent: true,
    validator: validateFullName,
  },
  lastName: {
    required: true,
    onChange: onNameChange,
    validator: validateName,
  },
});

function onNameChange(context) {
  const { firstName, lastName } = context;

  return { fullName: `${firstName} ${lastName}` };
}
```
