# Foreword

ivo is a user story focused event-driven schema validator which provides an interface for you to clearly define the behaviour of your entities at creation and during updates

# Installation

```bash
$ npm i ivo
```

# Importing

```js
// Using Nodejs "require"
const { Schema } = require('ivo');

// Using ES6 imports
import { Schema } from 'ivo';
```

# Defining a schema

```ts
import { Schema, type Summary } from 'ivo';

type User = {
  id: string;
  createdAt: string;
  email: null | string;
  username: string;
  phoneNumber: null | string;
  updatedAt: string;
  usernameUpdatableFrom: null | Date;
};

type UserInput = {
  email: string;
  username: string;
  phoneNumber: string;
};

const userSchema = new Schema<UserInput, User>(
  {
    id: { constant: true, value: generateUserId },
    email: {
      default: null,
      required: isEmailOrPhoneRequired,
      validator: validateUserEmail
    },
      },
      validator: validateUsername
    },
    phoneNumber: {
      default: null,
      required: isEmailOrPhoneRequired,
      validator: validatePhoneNumber
    },
    username: {
      required: true,
      shouldUpdate({ usernameUpdatableFrom }) {
        if (!usernameUpdatableFrom) return true;

        return (
          new Date().getTime() >= new Date(usernameUpdatableFrom).getTime()
        );
    usernameUpdatableFrom: {
      default: null,
      dependsOn: 'username',
      resolver({ isUpdate }) {
        if (!isUpdate) return null;

        const now = new Date();

        now.setDate(now.getDate() + 30);

        return now;
      }
    }
  },
  { timestamps: true }
);

function isEmailOrPhoneRequired({
  context: { email, phoneNumber }
}: Summary<UserInput, User>) {
  return [!email && !phoneNumber, 'Provide "email" or "phone" number'] as const;
}

// get the model
const UserModel = userSchema.getModel();
```

# Creating an entity

```ts
const { data, error } = await UserModel.create({
  email: 'txpz@mail.com',
  id: 1, // will be ignored because it is a constant property
  name: 'John Doe', // will be ignored because it is not on schema
  username: 'txpz',
  usernameUpdatableFrom: new Date() // will be ignored because it is a dependent property
});

if (error) return handleError(error);

console.log(data);
// {
//   createdAt: new Date(),
//   email: 'txpz@mail.com',
//   id: 18927934748659724,
//   phoneNumber: null,
//   updatedAt: new Date(),
//   username: 'txpz',
//   usernameUpdatableFrom: null
// }

// data is safe to dump in db
await userDb.insertOne(data);
```

# Updating an entity

```ts
const user = await userDb.findById(18927934748659724);

if (!user) return handleError({ message: 'User not found' });

const { data, error } = await UserModel.update(user, {
  usernameUpdatableFrom: new Date(), // dependent property -> will be ignored
  id: 2, // constant property -> will be ignored
  age: 34, // not on schema -> will be ignored
  userame: 'txpz-1'
});

if (error) return handleError(error);

console.log(data);
// {
//    userame: 'txpz-1',
//    usernameUpdatableFrom: Date, // value returned from resolver -> 30days from now
//   updatedAt: new Date()
// }

await userDb.updateById(user.id, data);
```

```ts
// updating 'username' again will not work

const { error } = await UserModel.update(user, {
  userame: 'txpz-2' // will be ignored because shouldUpdate rule will return false
});

console.log(error);
// {
//   message: 'NOTHING_TO_UPDATE',
//   payload: {}
// }
```

## Docs

- [Defining a schema](./docs/v0.0.1/index.md#defining-a-schema)
  - [allowed values](./docs/v0.0.1/definitions/allowed-values.md#allowed-values)
  - [constant properties](./docs/v0.0.1/definitions/constants.md#constant-properties)
  - [default values](./docs/v0.0.1/definitions/defaults.md#default-values)
  - [dependent properties](./docs/v0.0.1/definitions/dependents.md#dependent-properties)
  - [readonly properties](./docs/v0.0.1/definitions/readonly.md#readonly-properties)
  - [required properties](./docs/v0.0.1/definitions/required.md#required-properties)
  - [virtual properties](./docs/v0.0.1/definitions/virtuals.md#virtual-properties)
  - [validators](./docs/v0.0.1/validators/index.md#validators)
- [Extending Schemas](./docs/v0.0.1/definitions/extend-schemas.md#extending-schemas)
- [The Operation Context](./docs/v0.0.1/life-cycles.md#the-operation-contextt)
- [The Operation Summary](./docs/v0.0.1/life-cycles.md#the-operation-summary)
- [Life Cycles & Handlers](./docs/v0.0.1/life-cycles.md#life-cycle-listeners)

  - [onDelete](./docs/v0.0.1/life-cycles.md#ondelete)
  - [onFailure](./docs/v0.0.1/life-cycles.md#onfailure)
  - [onSuccess](./docs/v0.0.1/life-cycles.md#onsuccess)

- [Options](./docs/v0.0.1/index.md#options)
- [Custom validation errors](./docs/v0.0.1/index.md#errortool)
- [Extra features](./docs/v0.0.1/life-cycles.md#context-options)

- [Changelog](./docs/CHANGELOG.md#changelog)
