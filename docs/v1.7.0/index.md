# Defining a schema

Clean schema considers a property to be properly defined if it is `dependent`, `readonly`, `required`, a `virtual` or has a `default` value other than _undefined_

> N.B: Clean schema will throw an error if a property is not properly defined.
> The Schema constructor accepts 2 arguments:

1. definitions (required)
1. [options (optional)](#options)

The schema constructor also takes two generic types you could use to improve on the type inference of your `Input` & `Output` data.

```ts
const userSchema = new Schema<Input, Output>(definitions, options);
```

```ts
import { Schema } from "ivo";

type UserInput = {
  dob: Date | null;
  firstName: string;
  lastName: string;
};

type User = {
  dob: Date | null;
  firstName: string;
  lastName: string;
  fullName: string;
};

const userSchema = new Schema<UserInput, User>({
  dob: { required: true, validator: validateDob },
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
  fullName: {
    default: "",
    dependsOn: ["firstName", "lastName"],
    resolver({ context: { firstName, lastName } }) {
      return `${firstName} ${lastName}`;
    },
  },
});

const UserModel = userSchema.getModel();
```

# Properties of a model

These methods are async because custom validators could be async as well.

| Property | Type     | Description                                    |
| -------- | -------- | ---------------------------------------------- |
| create   | function | Async method to create an instance             |
| delete   | function | Async method to trigger all onDelete listeners |
| update   | function | Async method to update an instance             |

# Accepted rules

| Property     | Type                         | Description                                                                                                                                                     |
| ------------ | ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| allow        | any[ ] \| object             | used to specify the values that should be accepted for a property. [See more](./definitions/allowed-values.md#allowed-values)                                   |
| constant     | boolean                      | use with **`value`** rule to specify a property with a forever constant value. [more](./definitions/constants.md#constant-properties)                           |
| default      | any \| function              | the default value of a propterty. [more](./definitions/defaults.md#default-values)                                                                              |
| dependsOn    | string \| string[ ]          | a property or list of property the said property depends on. [more](./definitions/dependents.md#dependent-properties)                                           |
| ignore       | function                     | a function used to determine whether the input value of a property should be ignored. This acts as `shouldInit` + `shouldUpdate`                                |
| onDelete     | function \| function[ ]      | executed when the delete method of a model is invoked [more](./life-cycles.md#ondelete)                                                                         |
| onFailure    | function \| function[ ]      | executed after an unsucessful operation [more](./life-cycles.md#onfailure)                                                                                      |
| onSuccess    | function \| function[ ]      | executed after a sucessful operation [more](./life-cycles.md#onsuccess)                                                                                         |
| readonly     | boolean \| 'lax'             | a propterty whose value should not change [more](./definitions/readonly.md#readonly-properties)                                                                 |
| required     | boolean \| function          | a property that must be set during an operation [more](./definitions/required.md#required-properties)                                                           |
| sanitizer    | function                     | This could be used to transform a virtual property before their dependent properties get resolved. [more](./definitions/virtuals.md#sanitizer)                  |
| shouldInit   | false \| function(): boolean | A boolean or setter that tells ivo whether or not a property should be initialized.                                                                             |
| shouldUpdate | false \| function(): boolean | A boolean or setter that tells ivo whether or not a property should be initialized.                                                                             |
| validator    | function                     | A function (async / sync) used to validated the value of a property. [more](../v3.4.0/validate/index.md#validators)                                             |
| value        | any \| function              | value or setter of constant property. [more](./definitions/constants.md#constant-properties`)                                                                   |
| virtual      | boolean                      | a helper property that can be used to provide extra context but does not appear on instances of your model [more](./definitions/virtuals.md#virtual-properties) |

# Options

```ts
import type {DeletionContext, ImmutableContext, ImmutableSummary, ValidationErrorMessage } from 'ivo'

type Input = {}
type Output = {}

type IContext = Context<Input, Output>
type ISummary = ImmutableSummary<Input, Output>

type DeleteListener = (data: DeletionContext<Output>) => void | Promise<void>

type SuccessListener = (summary: ISummary) => void | Promise<void>

type Timestamp = {
  createdAt?: string
  updatedAt?: string
}

interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

type SchemaOptions = {
  equalityDepth?: number
  errorTool?: ErrorToolClass // more on this below 👇
  onDelete?: DeleteListener | DeleteListener[]
  onSuccess?: SuccessListener | SuccessListener[]
  postValidate?: PostValidationConfig | PostValidationConfig[]
  setMissingDefaultsOnUpdate?: boolean
  shouldUpdate?: boolean | (summary: ISummary) => boolean
  timestamps?: boolean | Timestamp
  useParentOptions?: boolean // 👈 only for extended schemas
}

const options: SchemaOptions = {}

const schema = new Schema<Input, Output, CtxOptions, ErrorToolClass>(definitions, options)
```

More details on the `Context` & `Summary` utiliies can be found [here](./life-cycles.md#the-operation-context)

## equalityDepth (default: 1)

This is the number used to determine if the value of a property has changed during updates.

To determine if a property has changed, it's value is compared against it's default value and previous value. Because object equality is not always straightforward, the `equalityDepth` provided is used to determine if properties of your schema that accept objects (which may have nested objects) as values have changed during updates

The possible values allowed for this number range from `0` to `+Infinity`. The default value is `1`, which means **one level of nesting**.

Here is a snippet to demonstrate how changing just the arragement of values of nested properties (without even changing their actual values) can affect the results of an update:

```ts
const user = {
  name: "John Doe",
  bio: {
    facebook: { displayName: "john", handle: "john3434" },
    twitter: { displayName: "John Doe", handle: "john_on_twitter" },
  },
};

// depth == 0

Model.update(user, { bio: user.bio }).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio
Model.update(user, {
  bio: {
    twitter: { displayName: "John Doe", handle: "john_on_twitter" },
    facebook: { displayName: "john", handle: "john3434" },
  },
}).then(({ data, error }) => {
  console.log(data);
  // {
  //   bio: {
  //     facebook: { displayName: 'john', handle: 'john3434' },
  //     twitter: { displayName: 'John Doe', handle: 'john_on_twitter' }
  //     }
  // }

  console.log(error); // null
});

// depth == 1

Model.update(user, { bio: user.bio }).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio
Model.update(user, {
  bio: {
    twitter: { displayName: "John Doe", handle: "john_on_twitter" },
    facebook: { displayName: "john", handle: "john3434" },
  },
}).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio and the positions of displayName & handle
Model.update(user, {
  bio: {
    twitter: { handle: "john_on_twitter", displayName: "John Doe" },
    facebook: { displayName: "john", handle: "john3434" },
  },
}).then(({ data, error }) => {
  console.log(data);
  // {
  //   bio: {
  //     facebook: { displayName: 'john', handle: 'john3434' },
  //     twitter: { handle: 'john_on_twitter', displayName: 'John Doe' }
  //     }
  // }

  console.log(error); // null
});
```

## errorTool

This is a class which will be used to manage your validation errors, hence giving you the power to have custom validation errors. See example [here](../../tests/schema/samples/custom-error-tool/index.ts)

```ts
import type { ValidationErrorMessage, IErrorTool } from "ivo";

// the class should have this signature 👇
interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

// the instances of your ErrorTool class should have this signature 👇
interface IErrorTool<ExtraData extends ObjectType = {}> {
  /** return what your validation error should look like from this method */
  get data(): IValidationError<ExtraData>;

  /** array of fields that have failed validation */
  get fields(): string[];

  /** determines if validation has failed */
  get isLoaded(): boolean;

  /** used to append a field to your final validation error */
  set(field: FieldKey, error: FieldError, value?: any): this;

  /** method to set the value of the validation error message */
  setMessage(message: ValidationErrorMessage): this;
}

type IValidationError<ExtraData extends ObjectType = {}> = ({
  message: ValidationErrorMessage;
} & ExtraData) & {};
```

## onDelete

This could be a function or an array of functions with the `DeleteListener` signature above. These functions would be triggered together with the onDelete listeners of individual properties when the `Model.delete` method is invoked. See more [here](./life-cycles.md#ondelete)

## onSuccess

This could be a function or an array of functions with the `SuccessListener` signature above. These functions would be triggered together with the onSuccess listeners of individual properties when the handleSuccess method is invoked at creation & during updates of any property. See more [here](./life-cycles.md#onsuccess)

## postValidate

To validate integrity of more than one field after initial validation. More on this [here](./validators.md#post-validation)

## setMissingDefaultsOnUpdate

A boolean. If set to `true`, it'll check all defaultable properties of the existing data passed to the model's update method `Model.update(existingData, updates)`, for all the properties with value `undefined` it'll generate their default values, add these them to the operation's context before validating the updates provided.

If the update operation is successful, the newly generated default values will also be added to the updated values returned if not already present on the updated values. Default **false**

## shouldUpdate (default: true)

A boolean or a function that expects the operation's summary and returns a boolean value. This value is read/computed before the values provided during updates have been validated.

If it's value or computed value if true, validations for updates will proceed else, the operation will fail with error message `Nothing to update`

```ts
new Schema(
  { id: { constant: true, value: generateId } },
  { shouldUpdate: () => (condition ? true : false) },
);
```

## timestamps (default: false)

If timestamps is set to true, you'll automatically have the `createdAt` and `updatedAt` properties attached to instances of your model at creation & during update. But you can overwrite the options and use your own properties like in the example below. Default **false**

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

As of v1.6.1, `updated_at` is `null` at creation

```js
// make updatedAt non-nullable
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: { key: "updated_at", nullable: false } },
});

// or non-nullable whilte keeping the default key
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: { nullable: false } },
});
```

## useParentOptions (default: true)

When extending schemas, extended schemas automatically inherit all options(except life cycle methods) of base schema. Setting `useParentOptions: false` in extended schema option will prevent this behaviour. Default is `true`
