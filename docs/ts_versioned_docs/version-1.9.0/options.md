---
title: Options
sidebar_position: 2
---

# Options

```ts
import type { Context, IvoSummary, ReadonlyIvoSummary } from 'ivo';

type Input = {};
type Output = {};

type IContext = Context<Input, Output>;
type Summary = IvoSummary<Input, Output, CtxOptions>;

type DeleteListener = (data: Outpur, options: CtxOptions) => void | Promise<void>;

type SuccessListener = (summary: Summary) => void | Promise<void>

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

This is a class which will be used to manage your validation errors, hence giving you the power to have custom validation errors. See example [here](https://github.com/kamtoeddy/ivo/blob/main/ts/tests/extras/error-sanitizer.ts)

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
