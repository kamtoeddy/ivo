# Required Properties

A property with `required === true` is one that must be initialised at creation and cloning. It must have a validator and cannot have a default value (nor setter)

> Out of the box, required is **`false`**

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

## Required Error
