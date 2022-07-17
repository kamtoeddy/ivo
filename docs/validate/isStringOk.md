## validate.isStringOk

To validate strings

```javascript
const { validate } = require("clean-schema");

console.log(
  validate.isStringOk("dbj jkdbZvjkbv", { match: /^[a-zA-Z_\-\S]+$/ })
); // { reasons: ["Unacceptable value"], valid: false, validated: undefined }

validate.isStringOk("Hello World!", {
  maxLength: 20,
  minLength: 3,
}); // { reasons: [], valid: true, validated: "Hello World!" }

console.log(
  validate.isStringOk("pineapple", {
    enums: ["apple", "banana", "watermelon"],
  })
); // { reasons: ["Unacceptable value"], valid: false, validated: undefined }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | str      | any    | The value you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property  | Type     | Description                                                                      |
| --------- | -------- | -------------------------------------------------------------------------------- |
| match     | RegExp   | A regular expression the string is expected to match. Default: **undefined**     |
| maxLength | number   | The maximum number of characters the string is expected to have. Default: **40** |
| minLength | number   | The minimum number of characters the string is expected to have. Default: **1**  |
| enums     | string[] | The set of values the string is expected to belong to. Default: **undefined**    |
