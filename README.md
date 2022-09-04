# Foreword

Clean-schema's purpose is to help you ensure that the data going to your database is always consistent. It provides an interface for you to clearly define the behaviour of your entities at creation and during updates together with the flexibility of using the database of your choice.

> N.B: Do not forget to handle errors that might be thrown by the create, clone and update methods. [See the structure of the error](./docs/v1.4.6/api-error.md#structure-of-apierror).

# Installation

First install [Node.js](http://nodejs.org/) Then:

```bash
$ npm i clean-schema
```

# Importing

```js
// Using Nodejs `require`
const { Schema } = require("clean-schema");

// Using ES6 imports
import { Schema } from "clean-schema";
```

# Defining a model

```js
const { Schema, validate } = require("clean-schema");

const userSchema = new Schema(
  {
    firstName: {
      required: true,
      onChange: onNameChange,
      validator: validateName,
    },
    fullName: {
      default: "",
      dependent: true,
    },
    isBlocked: {
      default: false,
      validator: validateBoolean,
    },
    id: {
      readonly: true,
      validator: validateId,
    },
    lastName: {
      required: true,
      onChange: [onNameChange],
      validator: validateName,
    },
    lastSeen: {
      default: "",
      shouldInit: false,
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

  return { fullName: `${firstName} ${lastName}` };
}

const UserModel = userSchema.getModel();
```

# Creating an instance

```js
const user = await UserModel({
  firstName: "James",
  fullName: "Mr. James",
  id: 1,
  lastName: "Spader",
  lastSeen: new Date(),
  name: "John Doe",
  password: "au_34ibUv^T-adjInFjj",
  role: "app-user",
}).create();

console.log(user);
//  {
//   createdAt: new Date(),
//   firstName: "James",
//   fullName: "James Spader",
//   id: 1,
//   isBlocked: false,
//   lastName: "Spader",
//   lastSeen: "",
//   password: "au_34ibUv^T-adjInFjj",
//   role: "app-user",
//   updatedAt: new Date(),
// };

const db = require("db-of-choice"); // use db of your choice

await db.insert(user);
```

# Updating instances

```js
const user = await db.query({ id: 1 });

if (!user) throw new Error("User not found");

const userUpdate = await UserModel(user).update({
  lastSeen: new Date(),
  id: 2,
  age: 34,
  fullName: "Raymond Reddington",
});

// age is ignored because it is not a valid property
// fullName is ignored because it is dependent
// id is ignored because it is readonly
console.log(userUpdate); // { lastSeen: new Date(), updatedAt: new Date() }

await db.update({ id: 1 }, userUpdate);
```

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                        |
| -------- | -------- | ---------------------------------- |
| clone    | function | Async method to copy an instance   |
| create   | function | Async method to create an instance |
| update   | function | Async method to update an instance |

## Docs

- Schema
  - [Defining Properties](./docs/v1.4.6/schema/definition.md#defining-a-schema)
  - [Inheritance](./docs/v1.4.6/schema/inheritance.md)
  - [The Operation Context](./docs/v1.4.6/schema/definition.md#the-operation-context)
  - [Life Cycle listeners](./docs/v1.4.6/schema/definition.md#life-cycle-listeners)
  - [Options](./docs/v1.4.6/schema/definition.md#options)
- [Validators](./docs/v1.4.6/validate/index.md#validators)
  - [isArrayOk](./docs/v1.4.6/validate/isArrayOk.md)
  - [isBooleanOk](./docs/v1.4.6/validate/isBooleanOk.md)
  - [isCreditCardOk](./docs/v1.4.6/validate/isCreditCardOk.md)
  - [isEmailOk](./docs/v1.4.6/validate/isEmailOk.md)
  - [isNumberOk](./docs/v1.4.6/validate/isNumberOk.md)
  - [isStringOk](./docs/v1.4.6/validate/isStringOk.md)
- [ApiError](./docs/v1.4.6/api-error.md)
- [Changelog](./docs/v1.4.6/CHANGELOG)

## Happy coding! 😎
