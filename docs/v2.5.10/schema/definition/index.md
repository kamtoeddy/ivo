# Defining a schema

Clean schema considers a property to be properly defined if it is `dependent`, `readonly`, `required`, a `sideEffect` or has a `default` value other than _undefined_

> N.B: Clean schema will throw an error if a property is not properly defined.
> The Schema constructor accepts 2 arguments:

1. definitions (required)
1. [options (optional)](#options)

```js
const userSchema = new Schema(definitions, options);
```

```js
const { Schema } = require("clean-schema");

const userSchema = new Schema({
  dob: {
    required: true,
    validator: validateDob,
  },
  firstName: {
    required: true,
    onChange: onNameChange,
    validator: validateName,
  },
  lastName: {
    required: true,
    onChange: onNameChange,
    validator: validateName,
  },
  fullName: {
    default: "",
    validator: validateName,
  },
});

const UserModel = userSchema.getModel();

function onNameChange(context) {
  const { firstName, lastName } = context;

  return { fullName: `${firstName} ${lastName}` };
}
```

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                                                             |
| -------- | -------- | ----------------------------------------------------------------------- |
| clone    | function | Async method to copy an instance                                        |
| create   | function | Async method to create an instance                                      |
| delete   | function | Async method to trigger all onDelete listeners                          |
| update   | function | Async method to update an instance                                      |
| validate | function | Async method used to validate a property based onthe validator provided |

| Property      | Type                    | Description                                                                                                                                                             |
| ------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| constant      | boolean                 | use with **`value`** rule to specify a property with a forever constant value. [more](../../../v1.5.0/schema/definition/constants.md#constant-properties-v150)          |
| default       | any \| function         | the default value of a propterty. [more](../../../v1.4.10/schema/definition/defaults.md#default-values)                                                                 |
| dependent     | boolean                 | to block the direct modification of a property. [more](../../../v1.4.10/schema/definition/dependents.md#dependent-properties)                                           |
| onChange      | function \| function[ ] | executed at creation `(unless shouldInit === false)`, during cloning and updates [more](./life-cycles.md#onchange)                                                      |
| onCreate      | function \| function[ ] | executed at creation & during cloning `(unless shouldInit === false)` [more](./life-cycles.md#oncreate)                                                                 |
| onDelete      | function \| function[ ] | executed when the delete method of a model is invoked [more](./life-cycles.md#ondelete)                                                                                 |
| onFailure     | function \| function[ ] | executed after an unsucessful operation [more](./life-cycles.md#onfailure)                                                                                              |
| onSuccess     | function \| function[ ] | executed after a sucessful operation [more](./life-cycles.md#onsuccess)                                                                                                 |
| onUpdate      | function \| function[ ] | executed during updates [more](./life-cycles.md#onupdate)                                                                                                               |
| readonly      | boolean \| 'lax'        | a propterty whose value should not change [more](../../../v1.4.10/schema/definition/readonly.md#readonly-properties)                                                    |
| required      | boolean \| function     | a property that must be set during an operation [more](../../../v1.5.0/schema/definition/required.md#required-properties)                                               |
| requiredError | any \| function         | the error message to use when using a callable required property [more](../../../v1.5.0/schema/definition/required.md#required-by-v150)                                 |
| sideEffect    | boolean                 | a property used to modify other properties but don't appear on instances of your model [more](../../../v1.5.0/schema/definition/side-effects.md#side-effect-properties) |
| shouldInit    | boolean                 | Tells clean-schema whether or not a property should be initialized. Default **true**                                                                                    |
| validator     | function                | A function (async / sync) used to validated the value of a property. [more](../../../v1.4.6/validate/index.md#validators)                                               |
| value         | any \| function         | value or setter of constant property. [more](./constants.md#constant-properties-v150`)                                                                                  |

# Options

```ts
interface ITimestamp {
  createdAt?: string;
  updatedAt?: string;
}

options: {
  errors: "silent" | "throw";
  timestamps: boolean | ITimestamp;
}
```

## errors

This option is to specify the way the errors should be treated. If set to `silent`, the errors will be returned in the operation's resolved results but if set to `throw`, it will simply throw the error(you may want to use in a try-catch block). The default value is **`'silent'`**

This is the structure of the error returned or thrown

```ts
type SchemaError = {
  message: string; // e.g. Validation Error
  payload: {
    [key: string]: string[]; // e.g. name: ["Invalid name", "too long"]
  };
  statusCode: number; // e.g. 400
};
```

## timestamps

If timestamps is set to true, you'll automatically have the `createdAt` and `updatedAt` properties attached to instances of your model at creation, cloning & update. But you can override the options and use your own properties like in the example below. Default **false**

```js
// override one
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at" },
});

// or both
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: "updated_at" },
});
```
