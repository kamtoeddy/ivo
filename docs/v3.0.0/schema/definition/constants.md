## Constant Properties

This type of property is set at creation or cloning and never changes.

- It requires 2 rules:

  - **`constant`** which must be **`true`** and
  - **`value`** which is either a **`fixed value`** or a setter (sync/async function which returns generated value)

- The `onDelete` & `onSuccess` are the only life cycle handlers supported on constant properties.
  These handlers would run once for when creating an instance ('onSuccess') and one more time when 'onDelete'

Example:

```ts
import { Schema } from "clean-schema";

const userSchema = new Schema({
  dateJoined: { constant: true, value: () => new Date() },
  id: {
    constant: true,
    value: (ctx) => `${ctx.userName}-${Date.now}`,
  },
  role: { constant: true, value: "user" },
  userName: { required: true, validator: validateUserName },
});
```
