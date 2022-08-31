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

const UserModel = userSchema.getModel(userSchema);

function onNameChange(context) {
  const { firstName, lastName } = context;

  return { fullName: `${firstName} ${lastName}` };
}
```

| Property   | Type                    | Description                                                                                                                                                                                                                                                                               |
| ---------- | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| default    | any                     | A value of any type you wish to use for a given property. Default **undefined**                                                                                                                                                                                                           |
| dependent  | boolean                 | If set to true, clean-schema will prevent any external modification of the property; making it's value solely dependent on another property via the onCreate / onUpdate handlers. Default **false**                                                                                       |
| onChange   | listener \| listener[ ] | A function or array of functions(async / sync) you want to execute everytime an instance of your model gets created or updated. **`NB:`** `These listeners(onChange) are always executed after onCreate & onUpdate listeners of the same property.` Default **[ ]**                       |
| onCreate   | listener \| listener[ ] | A function or array of functions(async / sync) you want to execute everytime an instance of your model gets created. Default **[ ]**                                                                                                                                                      |
| onUpdate   | listener \| listener[ ] | A function or array of functions(async / sync) you want to execute everytime the property definedo on get updated. Default **[ ]**                                                                                                                                                        |
| readonly   | boolean \| 'lax'        | If true will be required at initialization and will never allow updates. If true with shouldInit: false, will not be initialized but allowed to update only once. Default **false**                                                                                                       |
| required   | boolean                 | Specifies a property that must be initialised. Default **false**                                                                                                                                                                                                                          |
| sideEffect | boolean                 | Used with onUpdate to modify other properties but is not attached to instances of your model. Must have a validator, must have at least one onUpdate handler. onCreate handlers are ignored because the onUpdate handlers are used both at creation and during updates. Default **false** |
| shouldInit | boolean                 | Tells clean-schema whether or not a property should be initialized. Default **true**                                                                                                                                                                                                      |
| validator  | function                | A function(async / sync) used to validated the value of a property. [See interface](#validators). Default **null**                                                                                                                                                                        |

## The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( cloning, creation or update ) plus any side effect values (if present during the operation) defined in your schema.

## Life Cycle listeners

These are functions that are invoked during a life cycle operation an recieve the [operation context](#the-operation-context) as only parameter. They are expected to have the structure of the `onComplete function` below

```js
const transactionSchema = new Schema({
  completedAt: {
    default: "",
    readonly: true,
    dependent: true,
  },
  isComplete: {
    default: false,
    readonly: true,
    shouldInit: false,
    onUpdate: onComplete,
    validator: (val) => validateBoolean(val),
  },
});

// destructuring isComplete from the validation context
function onComplete({ isComplete }) {
  return { completedAt: isComplete ? new Date() : "" };
}
```

> If the handler does not return an object with the schema's properties or side effect properties, the value returned is simply ignored.

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

## Validators

Validators are expected to behave as below

```ts
const validator = (valueToValidate: any, ...args?, validationContext) => {
  // validation logic here

return {
  reasons?: string[],
  valid: boolean,
  validated?: any,}
};
```
