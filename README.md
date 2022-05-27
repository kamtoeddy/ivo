![nodejs logo](https://lh5.googleusercontent.com/_vxxLqmye7utXPvP6UHaQGE__iZEZ8hTqPxm5cZvoETLgTyJ_6dTbzW6OZU4CUh6YnrOBdUEtEQwC1LB_IFw2f9iOMo53IQZ-Kqhwc5yKQkH79DhxkgXYYFchdKQu1CcsG0QNLnG)

# Foreword

Node-schema's purpose is to help you define and validate your data at creation and during updates. Hence, Node-schema gives you the flexibility of using the database of your choice.

> N.B: Do not forget to handle errors that might be thrown by the create and update methods. See the structure of the error **@ApiError** below.

# Installation

First install [Node.js](http://nodejs.org/) Then:

```bash
$ npm i @blacksocks/node-schema

or

$ npm install @blacksocks/node-schema
```

# Importing

```javascript
// Using Nodejs `require`
const { makeModel, Schema } = require("@blacksocks/node-schema");

// Using ES6 imports
import { makeModel, Schema } from "@blacksocks/node-schema";
```

# Defining a model

```javascript
const userSchema = new Schema({
    id: {
        readonly: true,
        validator: validateId
     },
     name: {
        required: true,
        validator: validateName
     },
     password: {
        required: true,
        validator: validatePassword
     },
     role: {
        required: true,
        validator: validateRole
     },
     isBlocked: {
        default: false,
        validator: validateBoolean
     },
});

const UserModel = makeModel(userSchema;);
```

> N.B: Node-schema will throw an error if the no property is defined or if none of the properties defined are valid.

# Creating an instance

```javascript
const user = new UserModel({
  id: 1,
  name: "James Spader",
  password: "AbsdivinnnBbnkl-adjfbjj",
  role: "app-user",
}).create();

console.log(user); // { id: 1, name: "James Spader", password: "AbsdivinnnBbnkl-adjfbjj", role: "app-user"}

const db = require("db-of-choice"); // use db of your choice

await db.insert(user);
```

# Updating instances

```javascript
const user = await db.query({ id: 1 });

if (!user) return null;

const userUpdate = new UserModel(user).update({
  id: 2,
  name: "Raymond Reddington",
});

console.log(userUpdate); // { name: "Harold Cooper"}

db.update({ id: 1 }, userUpdate);
```

# Properties of the _Schema_ class

The Schema constructor accepts 2 properties:

- definitions:
  - **This is a required property**
  - This is object contains all the definitions of the structure of your data
  - See illustration above **@Defining a model**
- options
  - This is optional
  - An object composed of the following optional properties:
    1. **extensionOf**
       - The schema object your current schema inherits from
       - default: null
    1. **timestamp**
       - boolean which tells node-schema whether or not to add createdAt and updatedAt to instances of your model
       - default: false

```javascript
const adminSchema = new Schema(
  { ...definitions },
  { extensionOf: userSchema, timestamp: true }
);
```

# Properties of a _definition object_

```javascript
// the value of id in the definitions object below
// is the definition object
const definitions = {
  id: {
    readonly: true,
    validator: validateId,
  },
};

const adminSchema = new Schema(
  { ...definitions },
  { extensionOf: userSchema, timestamp: true }
);
```

| Property   | Type     | Description                                                                                                                                                                         |
| ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| default    | any      | A value of any type you wish to use for a given property                                                                                                                            |
| onCreate   | array    | An array of **synchronous** functions you want to be executed when an instance of your model gets created. Default **[ ]**                                                          |
| onUpdate   | array    | An array of **synchronous** functions you want to be executed when a particular property of your instance get updated. Default **[ ]**                                              |
| readonly   | boolean  | If true will be required at initialization and will never allow updates. If true with shouldInit: false, will not be initialized but allowed to update only once. Default **false** |
| required   | boolean  | Specifies a property that must be initialised. Default **false**                                                                                                                    |
| sideEffect | boolean  | Used with onUpdate to modify other properties but is not attached to instances of your model. Default **false**                                                                     |
| shouldInit | boolean  | Tells node-schema whether or not a property should be initialized. Default **false**                                                                                                |
| validator  | function | An **synchronous** function used to validated the value of a property. Must return {reason:string, valid: boolean, validated: undefined or any}. Default **null**                   |

# Properties of a model

| Property | Type     | Description                       |
| -------- | -------- | --------------------------------- |
| clone    | function | To clone an instance              |
| create   | function | To create an instance             |
| update   | function | To update an instance             |
| validate | function | Calls the validator of a property |

# Built-in validation helper

Node-schema has some built-in validators. Feel free to use or build you own validators based on these.

```javascript
const { validate } = require("@blacksocks/node-schema");

validate.isBooleanOk(val);

validate.isNumberOK(val, {
  range: { bounds: [10, 50.5], inclusiveBottom: false },
});

validate.isStringOk(val, {
  maxLength: 20,
  minLength: 3,
  enums: ["admin", "app-user", "moderator"],
});
```

- **each returns** { reason, valid, validated}

```typescript
// each returns an object with this structure:
validationResults: {
  reason: string, // the reason the validation failed e.g. Invalid name
  valid: boolean, // tells if data was valid or not
  validated: undefined | any // the validated values passed which could have been formated in the custom validator (i.e made ready for the db)
}
```

> N.B: Every validator, even your custom validators are expected to return an object that respects the above structure.

# Structure of ApiError

As stated earlier, the create and update methods may throw errors. They will, if the data passed are invalid.

1. ### With model({...values}).create()

   - This will happen if any of the values passed were invalid

1. ### With model({...values}).update({...updates})
   - This will happen if any of the updates passed were invalid,
   - if none of the values passed are different from the actual values

```typescript
ApiError: {
  message: string, // e.g. Validation Error
  payload: {
    [key]: string[] // e.g. name: ["Invalid name", "too long"]
  },
  statusCode: number // e.g. 400
}
```

## Happy coding! 😎
