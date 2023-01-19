## Side Effect Properties

These properties are used to manipulate dependent properties at the level of your model but won't appear on instances, hence don't go to you database.

Side effects must have `sideEffect: true`, a validator and atleast one property that depends on it. They can have (**`shouldInit === false`**)

Cannot be dependent, defaulted, strictly required nor readonly

> Out of the box, sideEffect is **`false`**

Example:

```js
const { Schema } = require("clean-schema");

// definition
const User = new Schema({
  blockUser: {
    sideEffect: true,
    validator: validateBoolean,
  },
  isBlocked: {
    default: false,
    dependent: true,
    dependsOn: "blockUser",
    resolver: makeBlocked,
  },
}).getModel();

function makeBlocked({ blockUser }) {
  return  isBlocked: blockUser ? true : false ;
}

function validateBoolean(value) {
  if (![false, true].includes(value))
    return { valid: false, reason: `${value} is not a boolean` };
  return { valid: true };
}

// creating
const user = await User.create({ blockUser: true, name: "Peter" });

console.log(user);
// { isBlocked: true }
```

The results of the above operation is an object with a single property `isBlocked`. `name` is missing because it does not belong to our schema but `blockUser` is missing because it is a side effect and because it was provided, the value of isBlocked is true instead of the default(false).

The same concept applies to `clone` and `update` operations.
