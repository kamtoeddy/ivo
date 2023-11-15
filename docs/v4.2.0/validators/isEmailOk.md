## isEmailOk

To validate emails

```ts
import { isEmailOk } from 'clean-schema';

console.log(isEmailOk('dbj jkdbZvjkbv')); // { reasons: ["Invalid email"], valid: false }

isEmailOk(' john@doe.com'); // {  valid: true, validated: "john@doe.com" }
```

### Parameters

| Position | Property    | Type   | Description                                       |
| -------- | ----------- | ------ | ------------------------------------------------- |
| 1        | value       | any    | The value you wish to validate                    |
| 2        | customRegEx | RegExp | The custom regular expression that should be used |
