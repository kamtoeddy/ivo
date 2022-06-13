![nodejs logo](https://lh5.googleusercontent.com/_vxxLqmye7utXPvP6UHaQGE__iZEZ8hTqPxm5cZvoETLgTyJ_6dTbzW6OZU4CUh6YnrOBdUEtEQwC1LB_IFw2f9iOMo53IQZ-Kqhwc5yKQkH79DhxkgXYYFchdKQu1CcsG0QNLnG)

# Foreword

Clean-schema's purpose is to help you define and validate your data at creation and during updates. Hence, clean-schema gives you the flexibility of using the database of your choice.

> N.B: Do not forget to handle errors that might be thrown by the create and update methods.See the structure of the error [**ApiError**](#structure-of-apierror) below.

# Installation

First install [Node.js](http://nodejs.org/) Then:

```bash
$ npm i clean-schema

or

$ npm install clean-schema
```

# Importing

```javascript
// Using Nodejs `require`
const { makeModel, Schema } = require("clean-schema");

// Using ES6 imports
import { makeModel, Schema } from "clean-schema";
```

# Defining a model

```javascript
const userSchema = new Schema({
  id: {
    readonly: true,
    validator: validateId,
  },
  name: {
    required: true,
    validator: validateName,
  },
  password: {
    required: true,
    validator: validatePassword,
  },
  role: {
    required: true,
    validator: validateRole,
  },
  isBlocked: {
    default: false,
    validator: validateboolean,
  },
});

const UserModel = makeModel(userSchema);
```

> N.B: Clean-schema will throw an error if the no property is defined or if none of the properties defined are valid.

# Creating an instance

```javascript
const user = await new UserModel({
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

const userUpdate = await new UserModel(user).update({
  id: 2,
  name: "Raymond Reddington",
});

console.log(userUpdate); // { name: "Raymond Reddington"}

await db.update({ id: 1 }, userUpdate);
```

The id did not update because it's a readonly property.

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
       - boolean which tells clean-schema whether or not to add createdAt and updatedAt to instances of your model
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
| onCreate   | array    | An array of functions(async / sync) you want to be executed when an instance of your model gets created. Default **[ ]**                                                            |
| onUpdate   | array    | An array of functions(async / sync) you want to be executed when a particular property of your instance get updated. Default **[ ]**                                                |
| readonly   | boolean  | If true will be required at initialization and will never allow updates. If true with shouldInit: false, will not be initialized but allowed to update only once. Default **false** |
| required   | boolean  | Specifies a property that must be initialised. Default **false**                                                                                                                    |
| sideEffect | boolean  | Used with onUpdate to modify other properties but is not attached to instances of your model. Default **false**                                                                     |
| shouldInit | boolean  | Tells clean-schema whether or not a property should be initialized. Default **true**                                                                                                |
| validator  | function | A function(async / sync) used to validated the value of a property. Must return {reason:string, valid: boolean, validated: undefined or any}. Default **null**                      |

## More on the onCreate & onUpdate properties

These are arrays of sync/async functions which will get called at creation or update of the property they're defined on respectively. Each properly defined method has access to its context ( an object composed of all the properties and values of the instance ) and is expected to return an object which will be attached to the instance at creation or the updated values during an update. See the example below:

```javascript
const { makeModel, Schema } = require("clean-schema");

const userSchema = new Schema({
  firstName: {
    required: true,
    onCreate: [onNameChange],
    onUpdate: [onNameChange],
    validator: validateName,
  },
  lastName: {
    required: true,
    onCreate: [onNameChange],
    onUpdate: [onNameChange],
    validator: validateName,
  },
  fullName: {
    default: "",
    validator: validateName,
  },
});

const UserModel = makeModel(userSchema);

function onNameChange(context) {
  const { firstName, lastName } = context;

  const fullName = `${firstName} ${lastName}`;

  return { fullName };
}
```

> N.B All functions(async / sync), passed to these arrays must return an object with valid properties of the model.

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                                           |
| -------- | -------- | ----------------------------------------------------- |
| clone    | function | Async function to copy an instance                    |
| create   | function | Async function to create an instance                  |
| update   | function | Async function to update an instance                  |
| validate | function | Async function that calls the validator of a property |

# Built-in validation helper

clean-schema has some built-in validators. Feel free to use or build you own validators based on these. Each returns an object with the following structure:

```typescript
validationResults: {
  reason: string, // the reason the validation failed e.g. Invalid name
  valid: boolean, // tells if data was valid or not
  validated: undefined | any // the validated values passed which could have been formated in the custom validator (i.e made ready for the db)
}
```

> N.B: Every validator, even your custom validators are expected to return an object that respects the above structure.

## validate.isArrayOk

You could validate an array of values of your choice. An array of primitives or objects.

```javascript
const { validate } = require("clean-schema");

const options = {
  empty: false,
  sorted: true,
  filter: (genre) => typeof genre === "string" && genre?.trim(),
  modifier: (genre) => genre?.trim().toLowerCase(),
};

const movieGenres = ["action", null, "horror", 1, "comedy", "Horror", "crime"];

console.log(validate.isArrayOk(movieGenres, options)); // { reason: "", valid: true, validated: ["action", "comedy", "crime", "horror"] }

const invalids = ["   ", [], null, 144];

console.log(validate.isArrayOk(invalids, options)); // { reason: "Expected a non-empty array", valid: false, validated: undefined }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | arr      | any[]  | The array you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property  | Type     | Description                                                                             |
| --------- | -------- | --------------------------------------------------------------------------------------- |
| empty     | boolean  | Whether array could be empty. Default: **false**                                        |
| filter    | function | Function to filter the array. Default: **(data) => false**                              |
| modifier  | function | Function to modify (format) individual values. Default: **undefined**                   |
| sorted    | boolean  | Whether array should be sorted. Default: **true**                                       |
| sorter    | function | Function to sort values. Default: **undefined**                                         |
| sortOrder | number   | Number used to do comparison check when sorted: true and sorter: undefined              |
| unique    | boolean  | Whether array should contain unique values. Default: **true**                           |
| uniqueKey | string   | A key(property) on objects in array used as unique criteria. e.g: "id". Default: **""** |

## validate.isBooleanOk

To validate boolean values

```javascript
const { validate } = require("clean-schema");

console.log(validate.isBooleanOk("true")); // { reason: "Expected a boolean", valid: false, validated: undefined }

console.log(validate.isBooleanOk(false)); // { reason: "", valid: true, validated: false }
```

## validate.isNumberOK

To validate numbers. Especially within a range

```javascript
const { validate } = require("clean-schema");

const options = {
  range: {
    bounds: [10, 10.5],
    isInclusiveBottom: false,
  },
};
console.log(validate.isNumberOk(10, options)); // { reason: "too small", valid: false, validated: undefined }

console.log(validate.isNumberOk(10.01, options)); // { reason: "", valid: true, validated: 10.01 }

console.log(validate.isNumberOk("10.01", options)); // { reason: "Expected a number", valid: false, validated: undefined }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | num      | any    | The value you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property              | Type     | Description                                                    |
| --------------------- | -------- | -------------------------------------------------------------- |
| range                 | object   | Object describing the range of values that will pass the check |
| range.bounds          | number[] | The lower and upper bounds. Default: **[-Infinity, Infinity]** |
| range.inclusiveBottom | boolean  | Whether the lower bound should be accepted. Default: **true**  |
| range.inclusiveTop    | boolean  | Whether the upper bound should be accepted. Default: **true**  |

## validate.isStringOk

To validate strings

```javascript
const { validate } = require("clean-schema");

console.log(
  validate.isStringOk("dbj jkdbZvjkbv", { match: /^[a-zA-Z_\-\S]+$/ })
); // { reason: "Unacceptable value", valid: false, validated: undefined }

validate.isStringOk("Hello World!", {
  maxLength: 20,
  minLength: 3,
}); // { reason: "", valid: true, validated: "Hello World!" }

console.log(
  validate.isStringOk("pineapple", {
    enums: ["apple", "banana", "watermelon"],
  })
); // { reason: "Unacceptable value", valid: false, validated: undefined }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | str      | any    | The value you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property  | Type     | Description                                                                      |
| --------- | -------- | -------------------------------------------------------------------------------- |
| match     | RegExp   | A regular expression the string is expected to match. Default: **undefined**     |
| maxLength | number   | The maximum number of characters the string is expected to have. Default: **30** |
| minLength | number   | The minimum number of characters the string is expected to have. Default: **1**  |
| enums     | string[] | The set of values the string is expected to belong to. Default: **undefined**    |

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
