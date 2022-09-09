## Side Effect Properties

Use if you have a usecase(s) for properties you want to manipulate at the level of your model but won't appear on instances.

It's validator is required and must have at least one [onChange listener](../../../v1.4.10/schema/life-cycles.md#onchange). Side effects only support onChange listeners as **`onChange = onCreate + onUpdate`** but initialization can be blocked (**`shouldInit === false`**)

Cannot be dependent, defaulted, required nor readonly

> Out of the box, sideEffect is **`false`**

Example:

```js
const { Schema } = require("clean-schema");

// definition
const User = new Schema({
  blockUser: {
    sideEffect: true,
    onChange: makeBlocked,
    validator: validateBoolean,
  },
  isBlocked: {
    default: false,
    dependent: true,
    validator: validateBoolean,
  },
}).getModel();

function makeBlocked({ blockUser }) {
  return { isBlocked: blockUser ? true : false };
}

function validateBoolean(value) {
  if (![false, true].includes(value))
    return { valid: false, reason: `${value} is not a boolean` };
  return { valid: true };
}

// creating
const user = await User.create({ blockUser: true, name: "Peter" });

console.log(user);
// {
//   isBlocked: true;
// }
```

The results of the above operation is an object with a single property `isBlocked`. `name` is missing because it does not belong to our schema but `blockUser` is missing because it is a side effect and because it was provided, the value of isBlocked is true instead of the default(false).

The same concept applies to `clone` and `update` operations.
