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

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                          |
| -------- | -------- | ------------------------------------ |
| clone    | function | Async function to copy an instance   |
| create   | function | Async function to create an instance |
| update   | function | Async function to update an instance |

## Docs

- Schema
  - [Defining Properties](./docs/schema/definition.md#defining-a-schema)
  - [Definitions](./docs/schema/definition.md#definitions)
  - [The validation context](./docs/schema/definition.md#the-validation-context)
  - [onCreate & onUpdate handlers](./docs/schema/definition.md#oncreate--onupdate-handlers)
  - [Options](./docs/schema/definition.md#options)
- [Helper Validators](./docs/validate/index.md#built-in-validation-helpers)
  - [isArrayOk](./docs/validate/isArrayOk.md)
  - [isBooleanOk](./docs/validate/isBooleanOk.md)
  - [isEmailOk](./docs/validate/isEmailOk.md)
  - [isNumberOk](./docs/validate/isNumberOk.md)
  - [isStringOk](./docs/validate/isStringOk.md)
- [ApiError](./docs/api-error.md#structure-of-apierror)

## Happy coding! 😎
