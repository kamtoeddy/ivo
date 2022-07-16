# Foreword

Clean-schema's purpose is to help you define and validate your data at creation and during updates. Hence, clean-schema gives you the flexibility of using the database of your choice.

> N.B: Do not forget to handle errors that might be thrown by the create and update methods.See the structure of the error [**ApiError**](./docs/api-error.md#structure-of-apierror) below.

# Installation

First install [Node.js](http://nodejs.org/) Then:

```bash
$ npm i clean-schema
```

# Importing

```javascript
// Using Nodejs `require`
const { makeModel, Schema } = require("clean-schema");

// Using ES6 imports
import { makeModel, Schema } from "clean-schema";
```

# Defining a model

```javascript
const userSchema = new Schema({
  id: {
    readonly: true,
    validator: validateId,
  },
  isBlocked: {
    default: false,
    validator: validateboolean,
  },
  name: {
    required: true,
    validator: validateName,
  },
  password: {
    required: true,
    validator: validatePassword,
  },
  role: {
    required: true,
    validator: validateRole,
  },
});

const UserModel = makeModel(userSchema);
```

# Creating an instance

```javascript
const user = await UserModel({
  id: 1,
  name: "James Spader",
  password: "AbsdivinnnBbnkl-adjfbjj",
  role: "app-user",
}).create();

console.log(user); // { id: 1, name: "James Spader", password: "AbsdivinnnBbnkl-adjfbjj", role: "app-user"}

const db = require("db-of-choice"); // use db of your choice

await db.insert(user);
```

# Updating instances

```javascript
const user = await db.query({ id: 1 });

if (!user) return null;

const userUpdate = await UserModel(user).update({
  id: 2,
  name: "Raymond Reddington",
});

console.log(userUpdate); // { name: "Raymond Reddington"}

await db.update({ id: 1 }, userUpdate);
```

## More on the onCreate & onUpdate properties

These are arrays of sync/async functions which will get called at creation or update of the property they're defined on respectively. Each properly defined method has access to its context ( an object composed of all the properties and values of the instance ) and is expected to return an object which will be attached to the instance at creation or the updated values during an update. See the example below:

```javascript
const { makeModel, Schema } = require("clean-schema");

const userSchema = new Schema({
  firstName: {
    required: true,
    onCreate: [onNameChange],
    onUpdate: [onNameChange],
    validator: validateName,
  },
  lastName: {
    required: true,
    onCreate: [onNameChange],
    onUpdate: [onNameChange],
    validator: validateName,
  },
  fullName: {
    default: "",
    validator: validateName,
  },
});

const UserModel = makeModel(userSchema);

function onNameChange(context) {
  const { firstName, lastName } = context;

  const fullName = `${firstName} ${lastName}`;

  return { fullName };
}
```

> N.B All functions(async / sync), passed to these arrays must return an object with valid properties of the model.

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                          |
| -------- | -------- | ------------------------------------ |
| clone    | function | Async function to copy an instance   |
| create   | function | Async function to create an instance |
| update   | function | Async function to update an instance |

## Happy coding! 😎
