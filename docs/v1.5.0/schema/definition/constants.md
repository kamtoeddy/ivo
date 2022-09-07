## Constant Properties

> **`undefined`** is used as default value for all properties out of the box.

## Value

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
    value: () => "",
  },
  initialFollowersCount: {
    constant: true,
    value: 0,
  },
});
```
