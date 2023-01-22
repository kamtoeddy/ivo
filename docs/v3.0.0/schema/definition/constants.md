## Constant Properties

This type of property is set at creation or cloning and never changes.

- It requires 2 rules:

  - **`constant`** which must be **`true`** and
  - **`value`** which is either a **`fixed value`** or a setter (sync/async function which returns generated value)

- The `onDelete` & `onSuccess` are the only life cycle handlers supported on constant properties.
  These handlers would run once for when creating an instance ('onSuccess') and one more time when 'onDelete'

Example:

```js
const { Schema } = require("clean-schema");

const userSchema = new Schema({
  dateJoined: { constant: true, value: () => new Date() },
  id: {
    constant: true,
    value: (ctx) => `${ctx.userName}-${Date.now}`,
  },
  userName: { required: true, validator: validateUserName },
});
```
