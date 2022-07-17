## validate.isNumberOK

To validate numbers. Especially within a range

```javascript
const { validate } = require("clean-schema");

const options = {
  range: {
    bounds: [10, 10.5],
    isInclusiveBottom: false,
  },
};
console.log(validate.isNumberOk(10, options)); // { reasons: ["too small"], valid: false, validated: undefined }

console.log(validate.isNumberOk(10.01, options)); // { reasons: [], valid: true, validated: 10.01 }

console.log(validate.isNumberOk("10.01", options)); // { reasons: ["Expected a number"], valid: false, validated: undefined }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | value    | any    | The value you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property              | Type     | Description                                                    |
| --------------------- | -------- | -------------------------------------------------------------- |
| range                 | object   | Object describing the range of values that will pass the check |
| range.bounds          | number[] | The lower and upper bounds. Default: **[-Infinity, Infinity]** |
| range.inclusiveBottom | boolean  | Whether the lower bound should be accepted. Default: **true**  |
| range.inclusiveTop    | boolean  | Whether the upper bound should be accepted. Default: **true**  |
