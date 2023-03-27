# Defining a schema

Clean schema considers a property to be properly defined if it is `dependent`, `readonly`, `required`, a `sideEffect` or has a `default` value other than _undefined_

> N.B: Clean schema will throw an error if a property is not properly defined.
> The Schema constructor accepts 2 arguments:

1. definitions (required)
1. [options (optional)](#options)

The schema constructor also takes two generic interfaces you could use to improve on the type inference of your `InputType` & `OutputType`.

```ts
const userSchema = new Schema<I, O>(definitions, options);
```

```ts
import { Schema } from "clean-schema";

type UserDTO = {
  dob?: Date | null;
  firstName: string;
  lastName: string;
};

type UserType = {
  dob?: Date | null;
  firstName: string;
  lastName: string;
  fullName: string;
};

const userSchema = new Schema<UserDTO, UserType>({
  dob: { required: true, validator: validateDob },
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
  fullName: {
    default: "",
    dependent: true,
    dependsOn: ["firstName", "lastName"],
    resolver: (ctx) => `${ctx.firstName} ${ctx.lastName}`,
  },
});

const UserModel = userSchema.getModel();
```

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| clone    | function | Async method to copy an instance                                         |
| create   | function | Async method to create an instance                                       |
| delete   | function | Async method to trigger all onDelete listeners                           |
| update   | function | Async method to update an instance                                       |
| validate | function | Async method used to validate a property based on the validator provided |

# Accepted rules

| Property     | Type                         | Description                                                                                                                                               |
| ------------ | ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| constant     | boolean                      | use with **`value`** rule to specify a property with a forever constant value. [more](../../../v3.0.0/schema/definition/constants.md#constant-properties) |
| default      | any \| function              | the default value of a propterty. [more](../../../v3.0.0/schema/definition/defaults.md#default-values)                                                    |
| dependent    | boolean                      | to block the direct modification of a property. [more](../../../v3.0.0/schema/definition/dependents.md#dependent-properties)                              |
| onDelete     | function \| function[ ]      | executed when the delete method of a model is invoked [more](./life-cycles.md#ondelete)                                                                   |
| onFailure    | function \| function[ ]      | executed after an unsucessful operation [more](./life-cycles.md#onfailure)                                                                                |
| onSuccess    | function \| function[ ]      | executed after a sucessful operation [more](./life-cycles.md#onsuccess)                                                                                   |
| readonly     | boolean \| 'lax'             | a propterty whose value should not change [more](../../../v3.0.0/schema/definition/readonly.md#readonly-properties)                                       |
| required     | boolean \| function          | a property that must be set during an operation [more](../../../v3.0.0/schema/definition/required.md#required-properties)                                 |
| shouldInit   | false \| function(): boolean | A boolean or setter that tells clean-schema whether or not a property should be initialized.                                                              |
| shouldUpdate | false \| function(): boolean | A boolean or setter that tells clean-schema whether or not a property should be initialized.                                                              |
| validator    | function                     | A function (async / sync) used to validated the value of a property. [more](../../../v1.4.6/validate/index.md#validators)                                 |
| value        | any \| function              | value or setter of constant property. [more](../../../v3.0.0/schema/definition/constants.md#constant-properties`)                                         |
| virtual      | boolean                      | a helper property that can be used to provide extra context but does not appear on instances of your model [more](./virtuals.md#virtual-properties)       |

# Options

```ts
type Timestamp = {
  createdAt?: string;
  updatedAt?: string;
};

options: {
  errors: "silent" | "throw";
  timestamps: boolean | Timestamp;
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

## onSuccess

This could be a function or an array of functions with the `SuccessListener` signature below. These functions would be triggered together with the onSuccess listeners of individual properties when the handleSuccess method is invoked at creation & during updates of any property. See more [here](./life-cycles.md#onsuccess)

It's summary contains the values of the entire entity

```ts
import type { ContextType } from "clean-schema";

type Input = { name: string; price: number };

type Output = { id: string; name: string };

type Context = ContextType<Input, Output>;

const BookModel = new Schema<Input, Output>(
  {
    id: { constant: true, value: generateId },
    name: { required: true, validator: validateBookName },
    price: { default: 0, validator: validatePrice },
  },
  { onSuccess }
).getModel();

function onSuccess(summary: OperationSummary) {}

type OperationSummary =
  | {
      context: Context;
      operationName: "creation";
      previousValue: undefined;
      value: Output;
    }
  | {
      context: Context;
      operationName: "update";
      previousValue: Output;
      value: Output;
    };

type SuccessListener = (
  operationSummary: OperationSummary
) => void | Promise<void>;
```

## timestamps

If timestamps is set to true, you'll automatically have the `createdAt` and `updatedAt` properties attached to instances of your model at creation, cloning & update. But you can overwrite the options and use your own properties like in the example below. Default **false**

Overwrite one

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at" },
});
```

Or both

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: "updated_at" },
});
```

To use one timestamp alone, pass false for the timestamp key to eliminate

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: false },
});

// or
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: false },
});
```
