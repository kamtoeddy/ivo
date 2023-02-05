## Virtual Properties

These properties are used to manipulate dependent properties at the level of your model but won't appear on instances, hence don't go to you database.

Side effects must have:

- `virtual: true`
- A validator and
- Atleast one property that depends on it

They can have (**`shouldInit === false`**), cannot be dependent, defaulted, strictly required nor readonly

> Out of the box, virtual is **`false`**

Example:

```js
import { Schema } from "clean-schema";

// definition
const User = new Schema({
  blockUser: { virtual: true, validator: validateBoolean },
  isBlocked: {
    default: false,
    dependent: true,
    dependsOn: "blockUser",
    resolver: ({ blockUser: isBlocked }) => isBlocked,
  },
}).getModel();

function validateBoolean(value) {
  if (![false, true].includes(value))
    return { valid: false, reason: `${value} is not a boolean` };
  return { valid: true };
}

// creating
const user = await User.create({ blockUser: true, name: "Peter" });

console.log(user); // { isBlocked: true }
```

The results of the above operation is an object with a single property `isBlocked`. `name` is missing because it does not belong to our schema but `blockUser` is missing because it is virtual and because it was provided, the value of isBlocked is true instead of the default(false).

The same concept applies to `clone` and `update` operations.
