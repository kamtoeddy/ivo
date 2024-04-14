# Validators

A validator is a function that assesses the validity of a property (meaning one validator per property). It can be sync/async but is expected to behave as below.

Properties that can have validators are allowed have upto 2 validators (1 Primary & 1 Secondary) `N.B: they have slightly different signatures`

```ts
import type { MutableSummary } from 'ivo';

type Input = {}; // the input type of your model
type Output = {}; // the output type of your model

type FieldError = {
  reasons: string[];
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
      reason?:
        | string
        | string[]
        | {
            [K in keyof (Input & Aliases)]: FieldError; // dot notation here works if first key is a property, virtual or alias e.g: { "address.street": "too short", "address.zipCode": "invalid code" }
          };
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
    //          👇 primary & 👇 secondary validators
    validator: [isEmailOk, isEmailUnique],
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
type PostValidationConfig = {
  properties: keyof Input[];
  handler: (
    summary: MutableSummary<Input, Output, CtxOptions>,
    propertiesProvided
  ) =>
    | void
    | ValidationResponseObject
    | Promise<void | ValidationResponseObject>;
};

// and the schema postValidate option's signature

type Options = {
  ...otherOptions;
  postValidate: PostValidationConfig | PostValidationConfig[];
};
```

As illustrated in the example above, the PostValidateConfig is an object that expects two properties:

- `properties` an array of at least two unique input properties on your schema
- `handler` a function (sync/async) that will determine the validity of the operation with respect to it's properties. This function is called immediately the initial validation is successful and at least one of the properties of it's config has been provided during updates but always gets called at creation

> **If the postValidate option is an array, every set of properties has to be unique for each config**

```ts
// ❌ both configs have wxactly the same properties
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ['email', 'username'], handler },
    { properties: ['username', 'email'], handler },
  ],
});

// ❌ subsets are not allowed
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ['email', 'username', 'date_of_birth'], handler },
    { properties: ['email', 'username'], handler },
  ],
});

// ✅ this works
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ['email', 'username'], handler },
    { properties: ['role', 'username'], handler },
  ],
});
```

Example:

```ts
type EventInput = {
  host: User['id'];
  guests: User['id'][];
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
      properties: ['host', 'guests', 'startTime', 'stopTime'],
      async handler({ context }: MutableSummary<EventInput, Event>) {
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

        if (!isHostAvailable) errors['host'] = 'Host not available';

        if (!areAllGuestsAvailable)
          errors['guests'] = {
            reason: 'Some guests are not available',
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

## Built-in validation helpers

Here are some built-in validators you could use study to build your own validators:

- [isArrayOk](./isArrayOk.md)
- [isBooleanOk](./isBooleanOk.md)
- [isCreditCardOk](./isCreditCardOk.md)
- [isEmailOk](./isEmailOk.md)
- [isNumberOk](./isNumberOk.md)
- [isStringOk](.isStringOk.md)
