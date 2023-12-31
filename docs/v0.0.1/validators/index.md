# Validators

Validators are expected to behave as below

```ts
import type { Summary } from 'ivo';

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
      validated?: keyof Input[K]; // the validated values passed which could have been formated in the custom validator (i.e made ready for the db). "K" here represents the property being validated
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

function validator1(value: any, summary: Summary<Input, Output>) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function validator2(value: any, summary: Summary<Input, Output>) {
  // validation logic here

  if (valid) return false;

  return true;
}
```

In the code snippet above we have 2 validators; `validator1` and `validator2`

Although both work just the same, we `validator1` is recommended because:

- it is good to provided the reason why the validation failed and
- returning the `validated` value tells TypeScript more about the type of that property especially if have not explicitly provided the input & output interfaces of your schema

> N.B: if the validator does not return a validated value or it is undefined, the direct value passed will be used even `undefined`.

## Built-in validation helpers

Here are some built-in validators you could use study to build your own validators:

- [isArrayOk](./isArrayOk.md)
- [isBooleanOk](./isBooleanOk.md)
- [isCreditCardOk](./isCreditCardOk.md)
- [isEmailOk](./isEmailOk.md)
- [isNumberOk](./isNumberOk.md)
- [isStringOk](.isStringOk.md)
