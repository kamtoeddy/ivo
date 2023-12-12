# Foreword

ivo is a user story focused utility that helps you build factory methods to manage entities and their values in your domain.

In short, it is an event-driven schema validator which provides an interface for you to clearly define the behaviour of your entities at creation and during updates.

# Installation

```bash
$ npm i ivo
```

# Importing

```js
// Using Nodejs `require`
const { Schema } = require('ivo');

// Using ES6 imports
import { Schema } from 'ivo';
```

# Defining a schema

```ts
import { Schema, type Summary } from 'ivo';

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

// get the model
const UserModel = userSchema.getModel();
```

# Creating an entity

```ts
import userDb from 'db-of-choice'; // use any db that supports the information you are modelling

const { data, error } = await UserModel.create({
  firstName: 'John',
  fullName: 'Mr. James',
  id: 1,
  lastName: 'Doe',
  lastSeen: new Date(),
  name: 'John Doe',
  password: 'au_34ibUv^T-adjInFjj',
  role: 'admin'
});

if (error) return handleError(error);

console.log(data);
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

await userDb.insert(data);
```

# Updating an entity

```ts
const user = await userDb.findById(18927934748659724);

if (!user) return handleError({ message: 'User not found' });

const { data, error } = await UserModel.update(user, {
  firstName: 'Peter',
  id: 2,
  age: 34,
  fullName: 'Tony Stark'
});

if (error) return handleError(error);

// age is ignored because it is not a valid property
// fullName is ignored because it is dependent
// id is ignored because it is a constant
console.log(data); // { firstName: "Peter", fullName: "Peter Doe", updatedAt: new Date() }

await userDb.updateById(user.id, data);
```

## Docs

- [Defining a schema](./docs/v0.0.1/index.md#defining-a-schema)
  - [allowed values](./docs/v0.0.1/index.md#accepted-rules)
  - [constant properties](./docs/v0.0.1/definitions/constants.md#constant-properties)
  - [default values](./docs/v0.0.1/definitions/defaults.md#default-values)
  - [dependent properties](./docs/v0.0.1/definitions/dependents.md#dependent-properties)
  - [readonly properties](./docs/v0.0.1/definitions/readonly.md#readonly-properties)
  - [required properties](./docs/v0.0.1/definitions/required.md#required-properties)
  - [virtuals](./docs/v0.0.1/definitions/virtuals.md#virtual-properties)
  - [validators](./docs/v0.0.1/validators/index.md#validators)
    - [isArrayOk](./docs/v0.0.1/validators/isArrayOk.md)
    - [isBooleanOk](./docs/v0.0.1/validators/isBooleanOk.md)
    - [isEmailOk](./docs/v0.0.1/validators/isEmailOk.md)
    - [isNumberOk](./docs/v0.0.1/validators/isNumberOk.md)
    - [isStringOk](./docs/v0.0.1/validators/isStringOk.md)
- [Extending Schemas](./docs/v0.0.1/definitions/extend-schemas.md#extending-schemas)
- [The Operation Context](./docs/v0.0.1/life-cycles.md#the-operation-contextt)
- [The Operation Summary](./docs/v0.0.1/life-cycles.md#the-operation-summary)
- [Life Cycles & Handlers](./docs/v0.0.1/life-cycles.md#life-cycle-listeners)

  - [onDelete](./docs/v0.0.1/life-cycles.md#ondelete)
  - [onFailure](./docs/v0.0.1/life-cycles.md#onfailure)
  - [onSuccess](./docs/v0.0.1/life-cycles.md#onsuccess)

- [Options](./docs/v0.0.1/index.md#options)
- [Custom validation errors](./docs/v0.0.1/index.md#errortool)
- [Internationalization](./docs/v0.0.1/life-cycles.md#context-options)

- [Changelog](./docs/CHANGELOG.md#changelog)
