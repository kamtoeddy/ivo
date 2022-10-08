## Default Values

Setting the default value of a given property can be done by:

- Populating the default field of the property's definition like `favoriteColor`
- Providing a function (sync or async) to provide a value at runtime

  > **`undefined`** is used as default value for all properties out of the box.

Example:

```js
const { Schema } = require("clean-schema");

const userSchema = new Schema({
  favoriteColor: {
    default: "indigo",
    validator: validateColor,
  },
  userName: {
    default: getDefaultUserName,
    validator: validateUserName,
  },
  firstName: {
    required: true,
    onChange: onNameChange,
    validator: validateName,
  },
  lastName: {
    required: true,
    onChange: onNameChange,
    validator: validateName,
  },
});

function getDefaultUserName({ firstName }) {
  return { userName: `${firstName}-${generateRandom()}` };
}
```
