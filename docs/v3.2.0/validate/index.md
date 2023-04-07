# Validators

Validators are expected to behave as below

```ts
import type { Summary } from "clean-schema";

type Input = {}; // the input type of your model
type Output = {}; // the output type of your model

type ValidationResults =
  | boolean
  | {
      valid: true; // tells if data was valid or not
      validated?: keyof Input[K]; // the validated values passed which could have been formated in the custom validator (i.e made ready for the db). "K" here represents the property being validated
    }
  | {
      reason?: string; // the reason the validation failed e.g. "Invalid name"
      reasons?: string[]; // the reasons the validation failed e.g. ["Invalid name", "Special characters are not allowed"] or ["Invalid name"]
      otherReasons?: {
        [K in keyof Input]: string | string[];
      };
      valid: false;
    };

const validator1 = (
  value: any,
  summary: Summary<Input, Output>
) => {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
};

const validator2 = (
  value: any,
  summary: Summary<Input, Output>
) {
  // validation logic here

  if (valid) return false

  return true
};
```

In the code snippet above we have 2 validators; `validator1` and `validator2`

Although both work just the same, we `validator1` is recommended because:

- it is good to provided the reason why the validation failed and
- returning the `validated` value tells TypeScript more about the type of that property especially if have not explicitly provided the input & output interfaces of your schema

> N.B: if the validator does not return a validated value or it is undefined, the direct value passed will be used even `undefined`.

## Built-in validation helpers

Clean-schema has some built-in validators. Feel free to use or build you own validators based on these:

- [isArrayOk](../../v2.6.0/validate/isArrayOk.md)
- [isBooleanOk](../../v2.6.0/validate/isBooleanOk.md)
- [isCreditCardOk](../../../v2.6.0/validate/isCreditCardOk.md)
- [isEmailOk](../../v2.6.0/validate/isEmailOk.md)
- [isNumberOk](../../v2.6.0/validate/isNumberOk.md)
- [isStringOk](./isStringOk.md)
