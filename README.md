# Foreword

Clean-schema is a user story focused utility that helps you build factory methods to manage entities and their values in your domain.

In short, it is an event-driven schema validator which provides an interface for you to clearly define the behaviour of your entities at creation and during updates.

# Installation

```bash
$ npm i clean-schema
```

# Importing

```js
// Using Nodejs `require`
const { Schema } = require('clean-schema');

// Using ES6 imports
import { Schema } from 'clean-schema';
```

# Defining a schema

```ts
import { Schema, type Summary } from 'clean-schema';

type UserRole = 'admin' | 'user';

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

type ISummary = Summary<Input, Output>;

const userSchema = new Schema<Input, Output>(
  {
    firstName: {
      required: true,
      validator: validateString('invalid first name')
    },
    fullName: {
      default: '',
      dependent: true,
      dependsOn: ['firstName', 'lastName'],
      resolver: getFullName
    },
    id: { constant: true, value: generateUserId },
    lastName: {
      required: true,
      validator: validateString('invalid last name')
    },
    password: { required: true, validator: validatePassword },
    role: { default: 'user', shouldInit: false, validator: validateRole }
  },
  { timestamps: true }
);

// resolvers
function getFullName({ context: { firstName, lastName } }: ISummary) {
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

  if (!valid) return { valid, reason: 'minimum characters should be 8' };

  return { valid, validated: hashPassword(validated) };
}

function validateRole(value) {
  const isValidString = validateString()(value);

  if (!isValidString.valid) return isValidString;

  const valid = ['admin', 'user'].includes(validated);

  if (!valid) return { valid, reason: 'invalid user role' };

  return { valid, validated: isValidString.validated };
}

function validateString(message = '') {
  return (value) => {
    const typeProvided = typeof value;

    if (typeProvided != 'string') {
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
import userDb from 'db-of-choice'; // use any db that supports the information you are modelling

const { data: user, handleSuccess } = await UserModel.create({
  firstName: 'John',
  fullName: 'Mr. James',
  id: 1,
  lastName: 'Doe',
  lastSeen: new Date(),
  name: 'John Doe',
  password: 'au_34ibUv^T-adjInFjj',
  role: 'admin'
});

console.log(user);
//  {
//   createdAt: new Date(),
//   firstName: "John",
//   fullName: "John Doe",
//   id: 18927934748659724,
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
const user = await userDb.findById(18927934748659724);

if (!user) throw new Error('User not found');

const { data, handleSuccess } = await UserModel.update(user, {
  firstName: 'Peter',
  id: 2,
  age: 34,
  fullName: 'Tony Stark'
});

// age is ignored because it is not a valid property
// fullName is ignored because it is dependent
// id is ignored because it is a constant
console.log(data); // { firstName: "Peter", fullName: "Peter Doe", updatedAt: new Date() }

await userDb.updateOne({ id: user.id }, data);

await handleSuccess();
```

## Docs

- [Defining a schema](./docs/v4.0.0/index.md#defining-a-schema)
  - [constant properties](./docs/v4.0.0/definitions/constants.md#constant-properties)
  - [default values](./docs/v4.0.0/definitions/defaults.md#default-values)
  - [dependent properties](./docs/v4.0.0/definitions/dependents.md#dependent-properties)
  - [readonly properties](./docs/v4.0.0/definitions/readonly.md#readonly-properties)
  - [required properties](./docs/v4.0.0/definitions/required.md#required-properties)
  - [virtuals](./docs/v4.0.0/definitions/virtuals.md#virtual-properties)
  - [validators](./docs/v4.0.0/validators/index.md#validators)
    - [isArrayOk](./docs/v4.0.0/validators/isArrayOk.md)
    - [isBooleanOk](./docs/v4.0.0/validators/isBooleanOk.md)
    - [isEmailOk](./docs/v4.0.0/validators/isEmailOk.md)
    - [isNumberOk](./docs/v4.0.0/validators/isNumberOk.md)
    - [isStringOk](./docs/v4.0.0/validators/isStringOk.md)
- [Extending Schemas](./docs/v4.0.0/definitions/extend-schemas.md#extending-schemas)
- [The Operation Context](./docs/v4.0.0/life-cycles.md#the-operation-contextt)
- [The Operation Summary](./docs/v4.0.0/life-cycles.md#the-operation-summary)
- [Life Cycles & Handlers](./docs/v4.0.0/life-cycles.md#life-cycle-listeners)

  - [onDelete](./docs/v4.0.0/life-cycles.md#ondelete)
  - [onFailure](./docs/v4.0.0/life-cycles.md#onfailure)
  - [onSuccess](./docs/v4.0.0/life-cycles.md#onsuccess)

- [Options](./docs/v4.0.0/index.md#options)
- [Custom validation errors](./docs/v4.0.0/index.md#errortool)
- [Internationalization](./docs/v4.0.0/life-cycles.md#context-options)

- [Changelog](./docs/CHANGELOG.md#changelog)
