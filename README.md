# Foreword

ivo is a user story focused event-driven schema validator which provides an interface for you to clearly define the behaviour of your entities at creation and during updates

# Installation

```bash
$ npm i ivo
```

# Importing

```js
// CJS
const { Schema } = require("ivo");

// ESM
import { Schema } from "ivo";
```

# Defining a schema

```ts
import { Schema, type MutableSummary } from "ivo";

type UserInput = {
  email: string;
  username: string;
  phoneNumber: string;
};

type User = {
  id: string;
  createdAt: Date;
  email: string | null;
  username: string;
  phoneNumber: string | null;
  updatedAt: Date;
  usernameLastUpdatedAt: Date | null;
};

const userSchema = new Schema<UserInput, User>(
  {
    id: { constant: true, value: generateUserID },
    email: {
      default: null,
      required: isEmailOrPhoneRequired,
      validator: [validateEmail, makeSureEmailIsUnique],
    },
    phoneNumber: {
      default: null,
      required: isEmailOrPhoneRequired,
      validator: validatePhoneNumber,
    },
    username: {
      required: true,
      validator: [validateUsername, makeSureUsernameIsUnique],
      shouldUpdate({ usernameLastUpdatedAt }) {
        if (!usernameLastUpdatedAt) return true;

        const timeDifferenceInMillisecs =
          new Date().getTime() - usernameUpdatableFrom.getTime();
        const thirtyDaysInMillisecs = 2_592_000_000;

        return timeDifferenceInMillisecs >= thirtyDaysInMillisecs;
      },
    },
    usernameLastUpdatedAt: {
      default: null,
      dependsOn: "username",
      resolver: ({ isUpdate }) => (isUpdate ? new Date() : null),
    },
  },
  { timestamps: true },
);

function isEmailOrPhoneRequired({
  context: { email, phoneNumber },
}: MutableSummary<UserInput, User>) {
  return [!email && !phoneNumber, 'Provide "email" or "phone" number'] as const;
}

async function makeSureEmailIsUnique(email: string) {
  const userWithEmail = await usersDb.findByEmail(email);

  return userWithEmail ? { valid: false, reason: "Email already taken" } : true;
}

async function makeSureUsernameIsUnique(username: string) {
  const userWithUsername = await usersDb.findByUsername(username);

  return userWithUsername
    ? { valid: false, reason: "Username already taken" }
    : true;
}

// get the model
const UserModel = userSchema.getModel();
```

# Creating an entity

```ts
const { data, error } = await UserModel.create({
  email: "john.doe@mail.com",
  id: 5, // will be ignored because it is a constant property
  name: "John Doe", // will be ignored because it is not on schema
  username: "john_doe",
  usernameLastUpdatedAt: new Date(), // will be ignored because it is a dependent property
});

if (error) return handleError(error);

console.log(data);
// {
//   createdAt: new Date(),
//   email: 'john.doe@mail.com',
//   id: 101,
//   phoneNumber: null,
//   updatedAt: new Date(),
//   username: 'john_doe',
//   usernameLastUpdatedAt: null
// }

// data is safe to dump in db
await usersDb.insertOne(data);
```

# Updating an entity

```ts
const user = await usersDb.findByID(101);

if (!user) return handleError({ message: "User not found" });

const { data, error } = await UserModel.update(user, {
  usernameLastUpdatedAt: new Date(), // dependent property -> will be ignored
  id: 75, // constant property -> will be ignored
  age: 34, // not on schema -> will be ignored
  username: "johndoe",
});

if (error) return handleError(error);

console.log(data);
// {
//   updatedAt: new Date(),
//   username: 'johndoe',
//   usernameLastUpdatedAt: Date // value returned from resolver -> 30days from now
// }

await usersDb.updateByID(user.id, data);
```

```ts
// updating 'username' again will not work

const { error } = await UserModel.update(user, {
  username: "john-doe", // will be ignored because shouldUpdate rule will return false
});

console.log(error);
// {
//   message: 'NOTHING_TO_UPDATE',
//   payload: {}
// }
```

## Docs

- [Defining a schema](./docs/v1.6.0/index.md#defining-a-schema)

  - [allowed values](./docs/v1.6.0/definitions/allowed-values.md#allowed-values)
  - [constant properties](./docs/v1.6.0/definitions/constants.md#constant-properties)
  - [default values](./docs/v1.6.0/definitions/defaults.md#default-values)
  - [dependent properties](./docs/v1.6.0/definitions/dependents.md#dependent-properties)
  - [readonly properties](./docs/v1.6.0/definitions/readonly.md#readonly-properties)
  - [required properties](./docs/v1.6.0/definitions/required.md#required-properties)
  - [virtual properties](./docs/v1.6.0/definitions/virtuals.md#virtual-properties)
  - [validators](./docs/v1.6.0/validators.md#validators)
  - [Extending Schemas](./docs/v1.6.0/definitions/extend-schemas.md#extending-schemas)
  - [The Operation Context](./docs/v1.6.0/life-cycles.md#the-operation-contextt)
  - [The Operation Summary](./docs/v1.6.0/life-cycles.md#the-operation-summary)
  - [Life Cycles & Handlers](./docs/v1.6.0/life-cycles.md#life-cycle-listeners)

  - [onDelete](./docs/v1.6.0/life-cycles.md#ondelete)
  - [onFailure](./docs/v1.6.0/life-cycles.md#onfailure)
  - [onSuccess](./docs/v1.6.0/life-cycles.md#onsuccess)

  - [Options](./docs/v1.6.0/index.md#options)
  - [Custom validation errors](./docs/v1.6.0/index.md#errortool)
  - [Extra features](./docs/v1.6.0/life-cycles.md#context-options)

- [Changelog](./docs/CHANGELOG.md#changelog)
