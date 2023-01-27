# Foreword

Clean-schema is a user story focused event-driven schema validator whose purpose is to help you ensure that the data going to your database is always consistent.

It provides an interface for you to clearly define the behaviour of your entities and their properites at creation and during updates together with the flexibility of using the database of your choice.

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

# Defining a schema

```ts
import { Schema } from "clean-schema";
import hash from "hashing-module-of-choice";

const userSchema = new Schema(
  {
    firstName: {
      required: true,
      validator: validateString("invalid first name"),
    },
    fullName: {
      default: "",
      dependent: true,
      dependsOn: ["firstName", "lastName"],
      resolver: generateFullName,
    },
    isBlocked: { default: false, validator: validateBoolean },
    id: { constant: true, value: generateId },
    lastName: {
      required: true,
      validator: validateString("invalid last name"),
    },
    lastSeen: { default: "", shouldInit: false },
    password: {
      required: true,
      validator(value) {
        let validated = String(value).trim();

        const valid = validated.length >= 8;

        if (!valid) return { valid, reason: "minimum characters should be 8" };

        validated = hash(validated);

        return { valid, validated };
      },
    },
    role: {
      readonly: true,
      validator(value) {
        const validated = String(value).trim();

        const valid = ["admin", "user"].includes(validated);

        if (!valid) return { valid, reason: "invalid user role" };

        return { valid, validated };
      },
    },
  },
  { timestamps: true }
);

function generateFullName(context) {
  const { firstName, lastName } = context;

  return `${firstName} ${lastName}`;
}

// get the model
const UserModel = userSchema.getModel();
```

# Creating an entity

```ts
import userDb from "db-of-choice"; // use any db that supports the information you are modelling

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
  role: "user",
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
//   role: "user",
//   updatedAt: new Date(),
// };

await userDb.insert(user);

await handleSuccess?.();
```

# Updating an entity

```ts
const user = await userDb.query({ id: 1 });

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

await userDb.update({ id: 1 }, data);

await handleSuccess?.();
```

## Docs

- [Defining a schema](./docs/v3.0.0/schema/definition/index.md#defining-a-schema)
  - [constant properties](./docs/v3.0.0/schema/definition/constants.md#constant-properties)
  - [default values](./docs/v3.0.0/schema/definition/defaults.md#default-values)
  - [dependent properties](./docs/v3.0.0/schema/definition/dependents.md#dependent-properties)
  - [readonly properties](./docs/v3.0.0/schema/definition/readonly.md#readonly-properties)
  - [required properties](./docs/v3.0.0/schema/definition/required.md#required-properties)
  - [side effects](./docs/v3.0.0/schema/definition/side-effects.md#side-effect-properties)
  - [validators](./docs/v2.6.0/validate/index.md#validators)
    - [isArrayOk](./docs/v2.6.0/validate/isArrayOk.md)
    - [isBooleanOk](./docs/v2.6.0/validate/isBooleanOk.md)
    - [isCreditCardOk](./docs/v2.6.0/validate/isCreditCardOk.md)
    - [isEmailOk](./docs/v2.6.0/validate/isEmailOk.md)
    - [isNumberOk](./docs/v2.6.0/validate/isNumberOk.md)
    - [isStringOk](./docs/v2.6.0/validate/isStringOk.md)
- [Inheritance](./docs/v3.0.0/schema/definition/inheritance.md#schema-inheritance)
- [The Operation Context](./docs/v3.0.0/schema/definition/life-cycles.md#the-operation-context)
- [Life Cycles & Listeners](./docs/v3.0.0/schema/definition/life-cycles.md#life-cycle-listeners)

  - [onDelete](./docs/v3.0.0/schema/definition/life-cycles.md#ondelete)
  - [onFailure](./docs/v3.0.0/schema/definition/life-cycles.md#onfailure)
  - [onSuccess](./docs/v3.0.0/schema/definition/life-cycles.md#onsuccess)

- [Options](./docs/v3.0.0/schema/definition/index.md#options)

- [Changelog](./docs/CHANGELOG.md#changelog)

## Happy coding! 😎
