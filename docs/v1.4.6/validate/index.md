# Validators

Validators are expected to behave as below

```ts
interface IValidationResults {
  reason?: string; // the reason the validation failed e.g. "Invalid name"
  reasons?: string[]; // the reasons the validation failed e.g. ["Invalid name", "Special characters are not allowed"] or ["Invalid name"]
  valid: boolean; // tells if data was valid or not
  validated?: any; // the validated values passed which could have been formated in the custom validator (i.e made ready for the db)
}

const validator = (value: any, validationContext): IValidationResults => {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
};
```

> N.B: if the validator does not return a validated value or it is undefined, the direct value passed will be used even `undefined`.

## Built-in validation helpers

clean-schema has some built-in validators. Feel free to use or build you own validators based on these. Each returns an object with the following structure:
