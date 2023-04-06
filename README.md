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
import type { GetContext, GetSummary } from "clean-schema";

type UserRole = "admin" | "user";

type Input = {
  firstName: string;
  lastName: string;
  password: string;
  role?: UserRole;
};

type Output = {
  createdAt: Date;
  firstName: string;
  fullName: string;
  id: number;
  lastName: string;
  password: string;
  role: UserRole;
  updatedAt: Date;
};

type Context = GetContext<Input, Output>;
type Summary = GetSummary<Input, Output>;

const userSchema = new Schema<Input, Output>(
  {
    firstName: {
      required: true,
      validator: validateString("invalid first name"),
    },
    fullName: {
      default: "",
      dependent: true,
      dependsOn: ["firstName", "lastName"],
      resolver: getFullName,
    },
    id: { constant: true, value: generateUserId },
    lastName: {
      required: true,
      validator: validateString("invalid last name"),
    },
    password: { required: true, validator: validatePassword },
    role: { default: "user", shouldInit: false, validator: validateRole },
  },
  { timestamps: true }
);

// resolvers
function getFullName({ context: { firstName, lastName } }: Summary) {
  return `${firstName} ${lastName}`;
}

function generateUserId() {
  return Math.random() * 1e18;
}

function hashPassword(value) {
  return Math.random().toString(value.length);
}

// validators
function validatePassword(value) {
  const isValidString = validateString()(value);

  if (!isValidString.valid) return isValidString;

  let validated = isValidString.validated;

  const valid = validated.length >= 8;

  if (!valid) return { valid, reason: "minimum characters should be 8" };

  return { valid, validated: hashPassword(validated) };
}

function validateRole(value) {
  const isValidString = validateString()(value);

  if (!isValidString.valid) return isValidString;

  const valid = ["admin", "user"].includes(validated);

  if (!valid) return { valid, reason: "invalid user role" };

  return { valid, validated: isValidString.validated };
}

function validateString(message = "") {
  return (value) => {
    const typeProvided = typeof value;

    if (typeProvided != "string") {
      const reason = message || `expected a string but got '${typeProvided}'`;

      return { valid: false, reason };
    }

    return { valid: true, validated: value.trim() };
  };
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
  firstName: "John",
  fullName: "Mr. James",
  id: 1,
  lastName: "Doe",
  lastSeen: new Date(),
  name: "John Doe",
  password: "au_34ibUv^T-adjInFjj",
  role: "admin",
});

console.log(user);
//  {
//   createdAt: new Date(),
//   firstName: "John",
//   fullName: "John Doe",
//   id: '18927934748659724',
//   lastName: "Doe",
//   password: "**************",
//   role: "user",
//   updatedAt: new Date(),
// };

await userDb.insert(user);

await handleSuccess();
```

# Updating an entity

```ts
const user = await userDb.query({ id: 1 });

if (!user) throw new Error("User not found");

const { data, error, handleSuccess } = await UserModel.update(user, {
  firstName: "Peter",
  id: 2,
  age: 34,
  fullName: "Tony Stark",
});

// age is ignored because it is not a valid property
// fullName is ignored because it is dependent
// id is ignored because it is a constant
console.log(data); // { firstName: "Peter", fullName: "Peter Doe", updatedAt: new Date() }

await userDb.update({ id: 1 }, data);

await handleSuccess();
```

## Docs

- [Defining a schema](./docs/v3.2.0/schema/definition/index.md#defining-a-schema)
  - [constant properties](./docs/v3.0.0/schema/definition/constants.md#constant-properties)
  - [default values](./docs/v3.0.0/schema/definition/defaults.md#default-values)
  - [dependent properties](./docs/v3.2.0/schema/definition/dependents.md#dependent-properties)
  - [readonly properties](./docs/v3.0.0/schema/definition/readonly.md#readonly-properties)
  - [required properties](./docs/v3.2.0/schema/definition/required.md#required-properties)
  - [virtuals](./docs/v3.2.0/schema/definition/virtuals.md#virtual-properties)
  - [validators](./docs/v3.2.0/validate/index.md#validators)
    - [isArrayOk](./docs/v2.6.0/validate/isArrayOk.md)
    - [isBooleanOk](./docs/v2.6.0/validate/isBooleanOk.md)
    - [isEmailOk](./docs/v2.6.0/validate/isEmailOk.md)
    - [isNumberOk](./docs/v2.6.0/validate/isNumberOk.md)
    - [isStringOk](./docs/v3.2.0/validate/isStringOk.md)
- [Extending Schemas](./docs/v3.2.0/schema/definition/extend-schemas.md#extending-schemas)
- [The Operation Context](./docs/v3.2.0/schema/definition/life-cycles.md#the-operation-context)
- [The Operation Summary](./docs/v3.2.0/schema/definition/life-cycles.md#the-operation-summary)
- [Life Cycles & Listeners](./docs/v3.2.0/schema/definition/life-cycles.md#life-cycle-listeners)

  - [onDelete](./docs/v3.2.0/schema/definition/life-cycles.md#ondelete)
  - [onFailure](./docs/v3.2.0/schema/definition/life-cycles.md#onfailure)
  - [onSuccess](./docs/v3.2.0/schema/definition/life-cycles.md#onsuccess)

- [Options](./docs/v3.2.0/schema/definition/index.md#options)

- [Changelog](./docs/CHANGELOG.md#changelog)

## Happy coding! 😎
