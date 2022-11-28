# Foreword

Clean-schema's purpose is to help you ensure that the data going to your database is always consistent. It provides an interface for you to clearly define the behaviour of your entities at creation and during updates together with the flexibility of using the database of your choice.

# Installation

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

# Defining an entity's schema

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
const {
  data: user,
  error,
  handleSuccess,
} = await UserModel.create({
  firstName: "James",
  fullName: "Mr. James",
  id: 1,
  lastName: "Spader",
  lastSeen: new Date(),
  name: "John Doe",
  password: "au_34ibUv^T-adjInFjj",
  role: "app-user",
});

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

await handleSuccess?.();

const db = require("db-of-choice"); // use db of your choice

await db.insert(user);
```

# Updating instances

```js
const user = await db.query({ id: 1 });

if (!user) throw new Error("User not found");

const { data, error, handleSuccess } = await UserModel.update(user, {
  lastSeen: new Date(),
  id: 2,
  age: 34,
  fullName: "Raymond Reddington",
});

// age is ignored because it is not a valid property
// fullName is ignored because it is dependent
// id is ignored because it is readonly
console.log(data); // { lastSeen: new Date(), updatedAt: new Date() }

await handleSuccess?.();

await db.update({ id: 1 }, data);
```

## Docs

- [Defining a schema](./docs/v2.1.0/schema/definition/index.md#defining-a-schema)
  - [constant properties](./docs/v1.5.0/schema/definition/constants.md#constant-properties-v150)
  - [default values](./docs/v1.4.10/schema/definition/defaults.md#default-values)
  - [dependent properties](./docs/v1.4.10/schema/definition/dependents.md#dependent-properties)
  - [readonly properties](./docs/v1.4.10/schema/definition/readonly.md#readonly-properties)
  - [required properties](./docs/v1.5.0/schema/definition/required.md#required-properties)
  - [side effects](./docs/v2.1.0/schema/definition/side-effects.md#side-effect-properties)
  - [validators](./docs/v1.4.6/validate/index.md#validators)
    - [isArrayOk](./docs/v1.4.6/validate/isArrayOk.md)
    - [isBooleanOk](./docs/v1.4.6/validate/isBooleanOk.md)
    - [isCreditCardOk](./docs/v1.4.6/validate/isCreditCardOk.md)
    - [isEmailOk](./docs/v1.4.6/validate/isEmailOk.md)
    - [isNumberOk](./docs/v1.4.6/validate/isNumberOk.md)
    - [isStringOk](./docs/v1.4.6/validate/isStringOk.md)
- [Inheritance](./docs/v1.4.6/schema/inheritance.md#schema-inheritance)
- [The Operation Context](./docs/v2.5.0/schema/definition/life-cycles.md#the-operation-context)
- [Life Cycles & Listeners](./docs/v2.5.0/schema/definition/life-cycles.md#life-cycle-listeners)
  - [onChange](./docs/v2.5.0/schema/definition/life-cycles.md#onchange)
  - [onCreate](./docs/v2.5.0/schema/definition/life-cycles.md#oncreate)
  - [onDelete](./docs/v2.5.0/schema/definition/life-cycles.md#ondelete)
  - [onFailure](./docs/v2.5.0/schema/definition/life-cycles.md#onfailure)
  - [onSuccess](./docs/v2.5.0/schema/definition/life-cycles.md#onsuccess)
  - [onUpdate](./docs/v2.5.0/schema/definition/life-cycles.md#onupdate)
- [Options](./docs/v1.4.7/schema/definitions.md#options)

- [Changelog](./docs/v2.5.5/CHANGELOG.md#changelog)

## Happy coding! 😎
