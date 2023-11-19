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
import { Schema } from 'clean-schema';

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
    default: '',
    dependent: true,
    dependsOn: ['firstName', 'lastName'],
    resolver({ context: { firstName, lastName } }) {
      return `${firstName} ${lastName}`;
    }
  }
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
| constant     | boolean                      | use with **`value`** rule to specify a property with a forever constant value. [more](./definitions/constants.md#constant-properties)                           |
| default      | any \| function              | the default value of a propterty. [more](./definitions/defaults.md#default-values)                                                                              |
| dependent    | boolean                      | to block the direct modification of a property. [more](./definitions/dependents.md#dependent-properties)                                                        |
| onDelete     | function \| function[ ]      | executed when the delete method of a model is invoked [more](./life-cycles.md#ondelete)                                                                         |
| onFailure    | function \| function[ ]      | executed after an unsucessful operation [more](./life-cycles.md#onfailure)                                                                                      |
| onSuccess    | function \| function[ ]      | executed after a sucessful operation [more](./life-cycles.md#onsuccess)                                                                                         |
| readonly     | boolean \| 'lax'             | a propterty whose value should not change [more](./definitions/readonly.md#readonly-properties)                                                                 |
| required     | boolean \| function          | a property that must be set during an operation [more](./definitions/required.md#required-properties)                                                           |
| sanitizer    | function                     | This could be used to transform a virtual property before their dependent properties get resolved. [more](./definitions/virtuals.md#sanitizer)                  |
| shouldInit   | false \| function(): boolean | A boolean or setter that tells clean-schema whether or not a property should be initialized.                                                                    |
| shouldUpdate | false \| function(): boolean | A boolean or setter that tells clean-schema whether or not a property should be initialized.                                                                    |
| validator    | function                     | A function (async / sync) used to validated the value of a property. [more](../v3.4.0/validate/index.md#validators)                                             |
| value        | any \| function              | value or setter of constant property. [more](./definitions/constants.md#constant-properties`)                                                                   |
| virtual      | boolean                      | a helper property that can be used to provide extra context but does not appear on instances of your model [more](./definitions/virtuals.md#virtual-properties) |

# Options

```ts
import type { Context, Summary, ValidationErrorMessage } from 'clean-schema'

type Input = {}
type Output = {}

type IContext = Context<Input, Output>
type ISummary = Summary<Input, Output>

type DeleteListener = (data: Readonly<Output>) => void | Promise<void>

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
  errors?: 'silent' | 'throw'
  onDelete?: DeleteListener | [DeleteListener]
  onSuccess?: SuccessListener | [SuccessListener]
  setMissingDefaultsOnUpdate?: boolean
  shouldUpdate?: boolean | (summary: ISummary) => boolean
  timestamps?: boolean | Timestamp
  useParentOptions?: boolean // 👈 only for extended schemas
}

const options: SchemaOptions = {}

const schema = new Schema<Input, Output, CtxOptions, ErrorToolClass>(definitions, options)
```

More details on the `Context` & `Summary` utiliies can be found [here](./life-cycles.md#the-operation-context)

## equalityDepth

This is the number used to determine if the value of a property has changed during updates.

To determine if a property has changed, it's value is compared against it's default value and previous value. Because object equality is not always straightforward, the `equalityDepth` provided is used to determine if properties of your schema that accept objects (which may have nested objects) as values have changed during updates

The possible values allowed for this number range from `0` to `+Infinity`. The default value is `1`, which means **one level of nesting**.

Here is a snippet to demonstrate how changing just the arragement of values of nested properties (without even changing their actual values) can affect the results of an update:

```ts
const user = {
  name: 'John Doe',
  bio: {
    facebook: { displayName: 'john', handle: 'john3434' },
    twitter: { displayName: 'John Doe', handle: 'john_on_twitter' }
  }
};

// depth == 0

Model.update(user, { bio: user.bio }).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio
Model.update(user, {
  bio: {
    twitter: { displayName: 'John Doe', handle: 'john_on_twitter' },
    facebook: { displayName: 'john', handle: 'john3434' }
  }
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
    twitter: { displayName: 'John Doe', handle: 'john_on_twitter' },
    facebook: { displayName: 'john', handle: 'john3434' }
  }
}).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio and the positions of displayName & handle
Model.update(user, {
  bio: {
    twitter: { handle: 'john_on_twitter', displayName: 'John Doe' },
    facebook: { displayName: 'john', handle: 'john3434' }
  }
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
import type { ValidationErrorMessage, IErrorTool } from 'clean-schema';

// the class should have this signature 👇
interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

// the instances of your ErrorTool class should have this signature 👇
interface IErrorTool<ExtraData extends ObjectType = {}> {
  /** return what your validation error should look like from this method */
  get data(): IValidationError<ExtraData>;

  /** return a custom error that will be thrown when validation fails & Schema.option.errors == 'throws' */
  get error(): Error;

  /** array of fields that have failed validation */
  get fields(): string[];

  /** determines if validation has failed */
  get isLoaded(): boolean;

  /** used to append a field to your final validation error */
  add(field: FieldKey, error: FieldError, value?: any): this;

  /** method to set the value of the validation error message */
  setMessage(message: ValidationErrorMessage): this;
}

type IValidationError<ExtraData extends ObjectType = {}> = ({
  message: ValidationErrorMessage;
} & ExtraData) & {};
```

## errors

This option is to specify the way the errors should be treated. If set to `silent`, the errors will be returned in the operation's resolved results but if set to `throw`, it will simply throw the error(you may want to use in a try-catch block). The default value is **`'silent'`**

This is the structure of the error returned or thrown

```ts
type SchemaErrorMessage =
  | 'INVALID_DATA'
  | 'INVALID_SCHEMA'
  | 'NOTHING_TO_UPDATE'
  | 'VALIDATION_ERROR';

type SchemaError = {
  message: SchemaErrorMessage;
  payload: {
    [key: string]: string[]; // e.g. name: ["Invalid name", "too long"]
  };
};
```

## onDelete

This could be a function or an array of functions with the `DeleteListener` signature above. These functions would be triggered together with the onDelete listeners of individual properties when the `Model.delete` method is invoked. See more [here](./life-cycles.md#ondelete)

## onSuccess

This could be a function or an array of functions with the `SuccessListener` signature above. These functions would be triggered together with the onSuccess listeners of individual properties when the handleSuccess method is invoked at creation & during updates of any property. See more [here](./life-cycles.md#onsuccess)

## setMissingDefaultsOnUpdate

A boolean. If set to `true`, it'll check all defaultable properties of the existing data passed to the model's update method `Model.update(existingData, updates)`, for all the properties with value `undefined` it'll generate their default values, add these them to the operation's context before validating the updates provided.

If the update operation is successful, the newly generated default values will also be added to the updated values returned if not already present on the updated values. Default **false**

## shouldUpdate

A boolean or a function that expects the operation's summary and returns a boolean value. This value is read/computed before the values provided during updates have been validated.

If it's value or computed value if true, validations for updates will proceed else, the operation will fail with error message `Nothing to update`

Unlike the shouldUpdate rule available on individual properties, this function can be asynchronous and can also be used to update the [context's options](life-cycles.md#context-options)

```ts
new Schema(
  {
    id: { constant: true, value: generateId }
  },
  {
    async shouldUpdate() {
      if (condition1) return false;
      if (condition2) return true;

      if (condition3) return { update: false };
      if (condition4) return { update: true };

      if (condition5)
        return { update: false, contextOptionsUpdate: { lang: 'en' } };

      return { update: true, contextOptionsUpdate: { lang: 'de' } };
    }
  }
);
```

## timestamps

If timestamps is set to true, you'll automatically have the `createdAt` and `updatedAt` properties attached to instances of your model at creation & during update. But you can overwrite the options and use your own properties like in the example below. Default **false**

Overwrite one

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: 'created_at' }
});
```

Or both

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: 'created_at', updatedAt: 'updated_at' }
});
```

To use one timestamp alone, pass false for the timestamp key to eliminate

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: 'created_at', updatedAt: false }
});

// or
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: false }
});
```

## useParentOptions

When extending schemas, extended schemas automatically inherit all options(except life cycle methods) of base schema. Setting `useParentOptions: false` in extended schema option will prevent this behaviour. Default is `true`
