## isStringOk

To validate strings

```ts
import { isStringOk } from 'ivo';

console.log(isStringOk('dbj jkdbZvjkbv', { regExp: /^[a-zA-Z_\-\S]+$/ })); // { reasons: ["Unacceptable value"], valid: false }

isStringOk('Hello World!', {
  maxLength: 20,
  minLength: 3
}); // { valid: true, validated: "Hello World!" }

console.log(
  isStringOk('pineapple', {
    enums: ['apple', 'banana', 'watermelon']
  })
); // { reasons: ["Unacceptable value"], valid: false }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | value    | any    | The value you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property  | Type      | Description                                                                       |
| --------- | --------- | --------------------------------------------------------------------------------- |
| enums     | string[ ] | The set of values the string is expected to belong to. Default: **undefined**     |
| maxLength | number    | The maximum number of characters the string is expected to have. Default: **225** |
| minLength | number    | The minimum number of characters the string is expected to have. Default: **1**   |
| regExp    | RegExp    | A regular expression the string is expected to match. Default: **undefined**      |
| trim      | boolean   | To have white-spaces removed at beginning and end of a string. Default: **false** |
