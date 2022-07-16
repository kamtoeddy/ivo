# Built-in validation helper

clean-schema has some built-in validators. Feel free to use or build you own validators based on these. Each returns an object with the following structure:

```typescript
validationResults: {
  reasons?: string[], // the reasons the validation failed e.g. ["Invalid name"]
  valid: boolean, // tells if data was valid or not
  validated?: any // the validated values passed which could have been formated in the custom validator (i.e made ready for the db)
}
```

> N.B: Every validator, even your custom validators are expected to return an object that respects the above structure.
