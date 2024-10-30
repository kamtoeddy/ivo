# Validators

A validator is a function that assesses the validity of a property (meaning one validator per property). It can be sync/async but is expected to behave as below.

Properties that can have validators are allowed have upto 2 validators (1 Primary & 1 Secondary) `N.B: they have slightly different signatures`

```ts
import type { MutableSummary } from "ivo";

type Input = {}; // the input type of your model
type Output = {}; // the output type of your model

type FieldError = {
  reason: string;
  metadata?: Record<string, any> | null;
};

type ValidationResults =
  | boolean
  | {
      valid: true; // tells if data was valid or not
      validated?: Input[K]; // the validated values passed which could have been formated in the custom validator (i.e made ready for the db). "K" here represents the property being validated
    }
  | {
      metadata?: Record<string, any>; // an object that will contain extra info on why validation failed
      reason?: string;
      valid: false;
    };

function primaryValidator(value: any, summary: MutableSummary<Input, Output>) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function secondaryValidator<T>(
  value: T,
  summary: MutableSummary<Input, Output>,
) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function validator1(value: any) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function validator2(value: any) {
  // validation logic here

  if (!valid) return false;

  return true;
}

const Model = new Schema({
  dateOfBirth: { required: true, validator: isValidDateOfBirth },
  email: {
    required: true,
    //          👇 primary  &  👇 secondary validators
    validator: [validateEmail, isEmailUnique],
  },
});
```

In the code snippet above we have 2 validators; `validator1` and `validator2`

Although both work just the same, we `validator1` is recommended because:

- it is good to provided the reason why the validation failed and
- returning the `validated` value tells TypeScript more about the type of that property especially if have not explicitly provided the input & output interfaces of your schema

> N.B: if the validator does not return a validated value or it is undefined, the direct value passed will be used even `undefined`.

> N.B: if the validator happens to throw an error, the validation of the said property will fail with reason `validation failed`

## Post validation

If you find the need to perform multiple validation steps on more than one field, you can achieve this with the `postValidate` option of your schema.

### PostValidationConfig:

```ts

type PostValidator = (
    summary: MutableSummary<Input, Output, CtxOptions>,
    propertiesProvided
  ) =>
    | void
    | ValidationResponseObject
    | Promise<void | ValidationResponseObject>

type InputProperty=  keyof Input

type PostValidationConfig = {
  properties: [InputProperty, InputProperty, ...InputProperty[]]; // array of at least 2 input properties
  validator: PostValidator | (PostValidator | PostValidator[])[] ;
};

// and the schema postValidate option's signature

type Options = {
  ...otherOptions;
  postValidate: PostValidationConfig | PostValidationConfig[];
};
```

As illustrated in the example above, the PostValidateConfig is an object that expects two properties:

- `properties` an array of at least two unique input properties on your schema
- `validator`
  - A function or array of (sync/async) functions that will determine the validity of the operation with respect to it's properties.
  - This validator(s) is/are invoked immediately after dependent properties are resolved and if at least one of the properties of it's config has been provided during updates but always gets called at creation
  - `N.B` if validator is an array, the validators at depth level 1 will run sequencially while the validators at depth level 2 will run in parallel

> **If the postValidate option is an array, every set of properties has to be unique for each config**

```ts
// ❌ both configs have wxactly the same properties
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ["email", "username"], validator },
    { properties: ["username", "email"], validator },
  ],
});

// ✅ as from v1.5.1 you can provide subsets of other configs
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ["email", "username", "date_of_birth"], validator },
    { properties: ["email", "username"], validator },
  ],
});

// ✅ this works
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ["email", "username"], validator },
    { properties: ["role", "username"], validator },
  ],
});
```

Example:

```ts
type EventInput = {
  host: User["id"];
  guests: User["id"][];
  startTime: Date;
  stopTime: Date;
};

type Event = { id: number } & EventInput;
```

Assuming the structure above reperesents an event (entity) in an event management system you are building. This event has an id, host, guests, startTime and stopTime as properties and the requirements are as follows:

- the host and guests must be ids of valid users in the system
- startTime must be greater than stopTime
- only the id of the event cannot be changed
- whenever host, guests, startTime or stopTime are modified, you have to make sure that the new state of the event respects the availability of the host and all guests i.e. host and guests should not be booked for another event in the said time frame

With the above requirements, it is clear we have to perform individual validations for host, guests, startTime and stopTime followed by a cross field validation for all 4 properties

```ts
const Model = new Schema(
  {
    id: { constant: true, value: generateEventId },
    host: { required: true, validator: validateHostId },
    guests: { required: true, validator: validateGuestIds },
    startTime: { required: true, validator: validateStartTime },
    stopTime: { required: true, validator: validateStopTime },
  },
  {
    postValidate: {
      properties: ["host", "guests", "startTime", "stopTime"],
      async validator({ context }: MutableSummary<EventInput, Event>) {
        // this is triggered when the individual
        // validations have all been successful

        const { host, guests, startTime, stopTime } = context;

        const [isHostAvailable, guestsAvailable] = await Promise.all([
          await isHostAvailableBetween(host, startTime, stopTime),
          await getGuestsAvailableBetween(guests, startTime, stopTime),
        ]);

        const areAllGuestsAvailable = guestsAvailable.length == guests.length;

        if (isHostAvailable && areAllGuestsAvailable) return;

        const errors = {};

        if (!isHostAvailable) errors["host"] = "Host not available";

        if (!areAllGuestsAvailable)
          errors["guests"] = {
            reason: "Some guests are not available",
            metadata: {
              unAvailableGuests: guests.filter(
                (g) => !guestsAvailable.includes(g),
              ),
            },
          };

        return errors;
      },
    },
  },
).getModel();
```

> N.B: **This option is not inherited during schema extension**

> N.B: if the post-validator happens to throw an error, the validation of the provided properties related to this validator will all fail with reason `validation failed`

## Validation flow

Data validation can occur in multiple stages depending on your schema's configuration

1. Primary validation

   - At this stage, primary validators are triggered, default and constant values are assigned or generated

   - The operation's context here is not safe because it is just made up of raw input but can be updated by the validated values returned from validators

1. Conditional required validation

   - Here, conditional required properties are evaluated
   - The operation's context here is already safe because of the validated values from the Primary validation stage would have been used to update the context
   - The operation's context cannot be updated at this stage

1. Secondary validation

   - This is where secondary validators get triggered
   - The operation's context here is also safe because of the Primary validation and can be updated by the validated values returned from validators

1. Post validation

   - Here, post-validation checks are evaluated with a safe operation context
   - The operation's context cannot be updated at this stage

1. Sanitization of virtual properties more on this [here](../definitions/virtuals.md#sanitizer)

1. Resolvement of dependent properties more on this [here](../definitions/dependents.md#dependent-properties)

## Built-in validation helpers

Here are some built-in validators you could use study to build your own validators:

### validateBoolean

To validate boolean values

```ts
import { validateBoolean } from "ivo";

console.log(validateBoolean("true")); // { reason: "Expected a boolean", valid: false }

console.log(validateBoolean(false)); // { valid: true, validated: false }
```

### validateCreditCard

A tiny utility method to test if a credit/debit **`Card Number`** is valid; not the credit card itself

```ts
import { validateCreditCard } from "ivo";

console.log(validateCreditCard(""));
// { reason: "Invalid card number", valid: false }

console.log(validateCreditCard(5420596721435293));
// { valid: true, validated: 5420596721435293}

console.log(validateCreditCard("5420596721435293"));
// { valid: true, validated: "5420596721435293"}
```

It returns:

```ts
type ValidationResponse =
  | { reasons: string[]; valid: false }
  | { valid: true; validated: number | string };
```

### validateEmail

To validate emails

```ts
import { validateEmail } from "ivo";

console.log(validateEmail("dbj jkdbZvjkbv")); // { reason: "Invalid email", valid: false }

validateEmail(" john@doe.com"); // {  valid: true, validated: "john@doe.com" }
```

#### Parameters

| Position | Property    | Type   | Description                                       |
| -------- | ----------- | ------ | ------------------------------------------------- |
| 1        | value       | any    | The value you wish to validate                    |
| 2        | customRegEx | RegExp | The custom regular expression that should be used |

### makeArrayValidator

You could validate an array of values of your choice. An array of primitives or objects.

```ts
import { makeArrayValidator } from "ivo";

const options = {
  min: { value: 1, error: "Expected a non-empty array" },
  sorted: true,
  filter: (genre) => typeof genre === "string" && genre?.trim(),
  modifier: (genre) => genre?.trim().toLowerCase(),
};

const movieGenres = ["action", null, "horror", 1, "comedy", "Horror", "crime"];

const validate = makeArrayValidator(options);

console.log(validate(movieGenres)); // { valid: true, validated: ["action", "comedy", "crime", "horror"] }

const invalids = ["   ", [], null, 144];

console.log(validate(invalids)); // { reason: "Expected a non-empty array", valid: false }
```

#### Options

| Property  | Type            | Description                                                                             |
| --------- | --------------- | --------------------------------------------------------------------------------------- |
| filter    | function        | A sync or async function to filter the array. Default: **(data) => false**              |
| modifier  | function        | A sync or async function to modify (format) individual values. Default: **undefined**   |
| sorted    | boolean         | Whether array should be sorted. Default: **true**                                       |
| sorter    | function        | Function to sort values. Default: **undefined**                                         |
| sortOrder | 'asc' \| 'desc' | Order used to do comparison check when sorted: true and sorter: undefined               |
| unique    | boolean         | Whether array should contain unique values. Default: **true**                           |
| uniqueKey | string          | A key(property) on objects in array used as unique criteria. e.g: "id". Default: **""** |

### makeNumberValidator

To validate numbers

```ts
import { makeNumberValidator } from "ivo";

type AllowConfig<T> =
  | ArrayOfMinSizeTwo<T>
  | { values: ArrayOfMinSizeTwo<T>; error: string | string[] };

type ExclusionConfig<T> =
  | T
  | ArrayOfMinSizeTwo<T>
  | { values: T | ArrayOfMinSizeTwo<T>; error: string | string[] };

type ValueError<T = number> = { value: T; error: string | string[] };

type NumberValidatorOptions<T extends number | any = number> = {
  exclude?: ExclusionConfig<T>;
} & XOR<
  { allow: AllowConfig<T> },
  {
    max?: number | ValueError;
    min?: number | ValueError;
    nullable?: boolean;
  }
>;

const options = { min: 10, max: 10.5 };

const validate = makeNumberValidator(options);

console.log(validate(10)); // { reason: "too small", valid: false, metadata: { min: 10, max: 10.5, inclusiveBottom: false,  inclusiveTop: true } }

console.log(validate(10.01)); // { valid: true, validated: 10.01, metadata }

console.log(validate("10.05")); // { valid: true, validated: 10.05, metadata }

console.log(makeNumberValidator({ allow: [0, -1, 35] }, 30)); // { reason: "Value not allowed", valid: false, metadata: { allowed: [0, -1, 35] } }
```

### makeStringValidator

To validate strings

```ts
import { makeStringValidator } from "ivo";

type StringValidatorOptions<T extends string | any = string> = {
  exclude?: ExclusionConfig<T>;
} & XOR<
  { allow: AllowConfig<T> },
  {
    max?: number | ValueError;
    min?: number | ValueError;
    nullable?: boolean;
    regExp?: ValueError<RegExp>;
    trim?: boolean;
  }
>;

const pattern = /^[a-zA-Z_\-\S]+$/;

console.log(
  makeStringValidator(
    {
      regExp: {
        value: pattern,
        error: `string should match this pattern: ${pattern}`,
      },
    },
    "dbj jkdbZvjkbv",
  ),
); // { reason: "Value not allowed", valid: false }

console.log(makeStringValidator({ max: 20, min: 3 }, "Hello World!")); // { valid: true, validated: "Hello World!" }

console.log(
  makeStringValidator(
    { allow: ["apple", "banana", "watermelon"] },
    "pineapple",
  ),
); // { reason: "Value not allowed", valid: false, metadata: { allowed: ['apple', 'banana', 'watermelon'] } }
```
