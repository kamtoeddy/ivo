# Foreword

Clean-schema's purpose is to help you define and validate your data at creation and during updates. Hence, clean-schema gives you the flexibility of using the database of your choice.

> N.B: Do not forget to handle errors that might be thrown by the create and update methods. [See the structure of the error **ApiError**](./docs/api-error.md#structure-of-apierror).

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
const { makeModel, Schema, validate } = require("clean-schema");

const userSchema = new Schema(
  {
    firstName: {
      required: true,
      onCreate: [onNameChange],
      onUpdate: [onNameChange],
      validator: validateName,
    },
    isBlocked: {
      default: false,
      validator: validateboolean,
    },
    id: {
      readonly: true,
      validator: validateId,
    },
    lastName: {
      required: true,
      onCreate: [onNameChange],
      onUpdate: [onNameChange],
      validator: validateName,
    },
    lastSeen: {
      default: "",
      shouldInit: false,
    },
    name: {
      default: "",
      dependent: true,
    },
    password: {
      required: true,
      validator: validatePassword,
    },
    role: {
      required: true,
      validator: (value) =>
        validate.isStringOk(value, { enums: ["admin", "app-user"] }),
    },
  },
  { timestamps: true }
);

function onNameChange(context) {
  const { firstName, lastName } = context;

  const name = `${firstName} ${lastName}`;

  return { name };
}

const UserModel = makeModel(userSchema);
```

# Creating an instance

```javascript
const user = await UserModel({
  firstName: "James",
  id: 1,
  lastName: "Spader",
  lastSeen: new Date(),
  name: "John Doe",
  password: "au_34ibUv^T-adjInFjj",
  role: "app-user",
}).create();

console.log(user);
// { createdAt: new Date(), firstName: "James", isBlocked: false, id: 1, lastName: "Spader", lastSeen: "", name: "James Spader", password: "au_34ibUv^T-adjInFjj", role: "app-user", updatedAt: new Date(),
// };

const db = require("db-of-choice"); // use db of your choice

await db.insert(user);
```

# Updating instances

```javascript
const user = await db.query({ id: 1 });

if (!user) return null;

const userUpdate = await UserModel(user).update({
  lastSeen: new Date(),
  id: 2,
  name: "Raymond Reddington",
});

// id is ignored because it is readonly
// name is ignored because it is dependent
console.log(userUpdate); // { lastSeen: new Date(), updatedAt: new Date() }

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
  - [Inheritance](./docs/schema/definition.md#inheritance)
  - [The Validation Context](./docs/schema/definition.md#the-validation-context)
  - [onCreate & onUpdate handlers](./docs/schema/definition.md#oncreate--onupdate-handlers)
  - [Options](./docs/schema/definition.md#options)
- [Helper Validators](./docs/validate/index.md#built-in-validation-helpers)
  - [isArrayOk](./docs/validate/isArrayOk.md)
  - [isBooleanOk](./docs/validate/isBooleanOk.md)
  - [isCreditCardOk](./docs/validate/isCreditCardOk.md)
  - [isEmailOk](./docs/validate/isEmailOk.md)
  - [isNumberOk](./docs/validate/isNumberOk.md)
  - [isStringOk](./docs/validate/isStringOk.md)
- [ApiError](./docs/api-error.md#structure-of-apierror)
- [Changelog](./docs/CHANGELOG.md)

## Happy coding! 😎
