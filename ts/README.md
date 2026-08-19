<p align="center">
  <img src="https://raw.githubusercontent.com/kamtoeddy/ivo/main/docs/static/img/logo.png" alt="ivo logo" width="120" />
</p>

# TypeScript Implementation

This is the documentation of the TypeScript implementation of ivo.

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
import { Schema } from "ivo";

type UserInput = {
  email: string | null;
  phoneNumber: string | null;
  username: string;
};

type User = {
  id: string;
  createdAt: Date;
  email: string | null;
  phoneNumber: string | null;
  updatedAt: Date | null;
  username: string;
  usernameLastUpdatedAt: Date | null;
};

const userSchema = new Schema<UserInput, User>(
  (b) =>
    b
      .field(b.constant("id", generateUserID))
      .field(
        b
          .lax("email", null)
          .validate(validateEmail)
          .reValidate(makeSureEmailIsUnique),
      )
      .field(b.lax("phoneNumber", null).validate(validatePhoneNumber))
      .field(
        b
          .required("username")
          .validate(validateUsername)
          .reValidate(makeSureUsernameIsUnique)
          .ignoreUpdate(({ previousValues }) => {
            const usernameLastUpdatedAt = previousValues.usernameLastUpdatedAt;

            if (!usernameLastUpdatedAt) return false;

            const timeDifferenceInMillisecs =
              new Date().getTime() - usernameLastUpdatedAt.getTime();
            const thirtyDaysInMillisecs = 2_592_000_000;

            return timeDifferenceInMillisecs < thirtyDaysInMillisecs;
          }),
      )
      .field(
        b
          .dependent("usernameLastUpdatedAt", "username")
          .default(null)
          .resolve(({ isUpdate }) => (isUpdate ? new Date() : null)),
      ),
  {
    timestamps: true,
    required: {
      fields: ["email", "phoneNumber"],
      handler({ values: { email, phoneNumber } }) {
        return !email && !phoneNumber
          ? 'Provide "email" or "phone" number'
          : false;
      },
    },
  },
);

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
  updatedAt: new Date(), // will be ignored because it is a timestamp
  usernameLastUpdatedAt: new Date(), // will be ignored because it is a dependent property
});

if (error) return handleError(error);

console.log(data);
// {
//   createdAt: new Date(),
//   email: 'john.doe@mail.com',
//   id: 101,
//   phoneNumber: null,
//   updatedAt: null,
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
  usernameLastUpdatedAt: add(new Date(), { days: 31 }), // dependent property -> will be ignored
  id: 75, // constant property -> will be ignored
  age: 34, // not on schema -> will be ignored
  username: "johndoe",
});

if (error) return handleError(error);

console.log(data);
// {
//   updatedAt: new Date(),
//   username: 'johndoe',
//   usernameLastUpdatedAt: new Date() // value returned from resolver -> current date
// }

await usersDb.updateByID(user.id, data);
```

```ts
// any further attempt to update 'username' will be ignored until
// the 'shouldUpdate' rule returns true

const { error } = await UserModel.update(user, { username: "john-doe" });

console.log(error);
// {
//   message: 'NOTHING_TO_UPDATE',
//   payload: {}
// }
```

## Docs

[Read the docs](https://ivo.kamtoeddy.com/docs/ts)
