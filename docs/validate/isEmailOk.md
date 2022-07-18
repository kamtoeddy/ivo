## validate.isEmailOk

To validate emails

```javascript
const { validate } = require("clean-schema");

console.log(validate.isEmailOk("dbj jkdbZvjkbv")); // { reasons: ["Invalid email"], valid: false, validated: undefined }

validate.isEmailOk(" john@doe.com"); // { reasons: [], valid: true, validated: "john@doe.com" }
```

### Parameters

| Position | Property    | Type   | Description                                       |
| -------- | ----------- | ------ | ------------------------------------------------- |
| 1        | value       | any    | The value you wish to validate                    |
| 2        | customRegEx | RegExp | The custom regular expression that should be used |
