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

# Properties of a schema

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                        |
| -------- | -------- | ---------------------------------- |
| clone    | function | Async method to copy an instance   |
| create   | function | Async method to create an instance |
| update   | function | Async method to update an instance |

| Property   | Type                    | Description                                                                                                               |
| ---------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| default    | any \| function         | the default value of a propterty. [more](./defaults.md)                                                                   |
| dependent  | boolean                 | to block the direct modification of a property. [more](./dependents.md#dependent-properties)                              |
| onChange   | function \| function[ ] | executed at creation `(unless shouldInit === false)`, during cloning and updates [more](../life-cycles.md#onchange)       |
| onCreate   | function \| function[ ] | executed at creation & during cloning `(unless shouldInit === false)` [more](../life-cycles.md#oncreate)                  |
| onUpdate   | function \| function[ ] | executed during updates [more](../life-cycles.md#onupdate)                                                                |
| readonly   | boolean \| 'lax'        | a propterty whose value should not change [more](./readonly.md)                                                           |
| required   | boolean                 | a property that must be set at creation [more](./required.md)                                                             |
| sideEffect | boolean                 | a property used to modify other properties but don't appear on instances of your model [more](./side-effects.md)          |
| shouldInit | boolean                 | Tells clean-schema whether or not a property should be initialized. Default **true**                                      |
| validator  | function                | A function (async / sync) used to validated the value of a property. [more](../../../v1.4.6/validate/index.md#validators) |

## Options

```ts
interface ITimestamp {
  createdAt?: string;
  updatedAt?: string;
}

options: {
  timestamps: boolean | ITimestamp;
}
```

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
