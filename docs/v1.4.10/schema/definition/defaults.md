## Default Values

Setting the default value of a given property can be done by:

- Populating the default field of the property's definition like `favoriteColor`
- Providing a synchronous function to provide a value at runtime
  > **`undefined`** is used as default value for all properties out of the box.

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

const UserModel = userSchema.getModel();

function getDefaultUserName(ctx) {
  return { userName: `${ctx.firstName}-${generateRandom()}` };
}
```
