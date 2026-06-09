## isNumberOK

To validate numbers. Especially within a range

```ts
import { isNumberOk } from 'ivo';

// 👇 here is the signature of the metadata returned on failure
type Metadata =
  | { allowed: number[] }
  | {
      min: number | null;
      max: number | null;
      inclusiveBottom: boolean;
      inclusiveTop: boolean;
    };

const options = {
  range: {
    bounds: [10, 10.5],
    isInclusiveBottom: false
  }
};
console.log(isNumberOk(10, options)); // { reasons: ["too small"], valid: false, metadata:{ min: 10, max: 10.5, inclusiveBottom: false,  inclusiveTop: true } }

console.log(isNumberOk(10.01, options)); // { valid: true, validated: 10.01, metadata }

console.log(isNumberOk('10.05', options)); // { valid: true, validated: 10.05, metadata }

console.log(isNumberOk(30, { allow: [0, -1, 35] })); // { reasons: ["Unacceptable value"], valid: false, metadata :{ allowed: [0, -1, 35] } }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | value    | any    | The value you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property              | Type      | Description                                                    |
| --------------------- | --------- | -------------------------------------------------------------- |
| range                 | object    | Object describing the range of values that will pass the check |
| range.bounds          | number[ ] | The lower and upper bounds. Default: **[-Infinity, Infinity]** |
| range.inclusiveBottom | boolean   | Whether the lower bound should be accepted. Default: **true**  |
| range.inclusiveTop    | boolean   | Whether the upper bound should be accepted. Default: **true**  |
