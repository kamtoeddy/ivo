# Defining a schema

The Schema constructor accepts 2 arguments:

1. definitions (required)
1. options (optional)

```javascript
const userSchema = new Schema(definitions, options);
```

```javascript
const { makeModel, Schema } = require("clean-schema");

const userSchema = new Schema({
  dob: {
    required: true,
    validator: validateDob,
  },
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

| Property   | Type     | Description                                                                                                                                                                                                                                                                               |
| ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| default    | any      | A value of any type you wish to use for a given property. Default **undefined**                                                                                                                                                                                                           |
| dependent  | boolean  | If set to true, clean-schema will prevent any external modification of the property; making it's value soley dependent on another property via the onCreate / onUpdate handlers. Default **false**                                                                                        |
| onCreate   | array    | An array of functions(async / sync) you want to be executed when an instance of your model gets created. Default **[ ]**                                                                                                                                                                  |
| onUpdate   | array    | An array of functions(async / sync) you want to be executed when a particular property of your instance get updated. Default **[ ]**                                                                                                                                                      |
| readonly   | boolean  | If true will be required at initialization and will never allow updates. If true with shouldInit: false, will not be initialized but allowed to update only once. Default **false**                                                                                                       |
| required   | boolean  | Specifies a property that must be initialised. Default **false**                                                                                                                                                                                                                          |
| sideEffect | boolean  | Used with onUpdate to modify other properties but is not attached to instances of your model. Must have a validator, must have at least one onUpdate handler. onCreate handlers are ignored because the onUpdate handlers are used both at creation and during updates. Default **false** |
| shouldInit | boolean  | Tells clean-schema whether or not a property should be initialized. Default **true**                                                                                                                                                                                                      |
| validator  | function | A function(async / sync) used to validated the value of a property. [See interface](#validator-interface). Default **null**                                                                                                                                                               |

## Inheritance

Below is an example of how you can make a schema inherit from another:

```javascript
const { makeModel, Schema } = require("clean-schema");

const adminSchema = new Schema(
  {
    extraPermissions: {
      required: true,
      validator: validateExtraPermissions,
    },
  },
  { timestamp: { createdAt: "created_at" } }
).extend(userSchema, { remove: ["dob"] });

const AdminModel = makeModel(adminSchema);
```

## The Validation Context

This is an object comprized of values of the instance being manipulated ( created / updated ) plus any side effect values defined in your schema.

## onCreate & onUpdate handlers

These handlers are expected to have the structure below

```javascript
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
    onUpdate: [onComplete],
    validator: (val) => validateBoolean(val),
  },
});

// destructuring isComplete from the validation context
function onComplete({ isComplete }) {
  return { completedAt: isComplete ? new Date() : "" };
}
```

> If the handler does not return an object with the schema's properties or side effect properties, the value returned is simply ignored.

Clean schema considers a property to be properly defined if it is `dependent`, `readonly`, `required`, a `sideEffect` or has a `default` value other than _undefined_

> N.B: Clean schema will throw an error if a property is not properly defined.

## Options

```typescript
interface ITimestamp {
  createdAt?: string;
  updatedAt?: string;
}

options: {
  timestamp: boolean | ITimestamp;
}
```

If timestamp is set to true, you'll automatically have the `createdAt` and `updatedAt` properties attached to instances of your model at creation, cloning & update. But you can override the options and use your own properties like in the example below. Default **false**

```javascript
// override both
const transactionSchema = new Schema(definitions, {
  timestamp: { createdAt: "created_at", updatedAt: "updated_at" },
});

// or one
const transactionSchema = new Schema(definitions, {
  timestamp: { createdAt: "created_at" },
});
```

### Validator

Validator functions are expected to have the structure below

```typescript
const validator = (valueToValidate: any, validationContext) => {
  // validation logic here

return {
  reasons?: string[],
  valid: boolean,
  validated?: any,}
};
```
