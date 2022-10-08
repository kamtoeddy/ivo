## Constant Properties (v1.5.0)

This type of property is set at creation or cloning and never changes. It requires # rules: **`constant`** & [**`value`**](#value) and [**`onCreate`**](../../../v1.4.10/schema/life-cycles.md#oncreate) which is optional.

constant must be true and value is either an fixed value or a setter (sync or async function returns generated value)

Example:

```js
const { Schema } = require("clean-schema");

const userSchema = new Schema({
  dateJoined: {
    constant: true,
    value: () => new Date(),
  },
  id: {
    constant: true,
    value: (ctx) => `${ctx.userName}-${Date.now}`,
  },
  userName: {
    required: true,
    validator: validateUserName,
  },
});
```
