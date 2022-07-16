## validate.isArrayOk

You could validate an array of values of your choice. An array of primitives or objects.

```javascript
const { validate } = require("clean-schema");

const options = {
  empty: false,
  sorted: true,
  filter: (genre) => typeof genre === "string" && genre?.trim(),
  modifier: (genre) => genre?.trim().toLowerCase(),
};

const movieGenres = ["action", null, "horror", 1, "comedy", "Horror", "crime"];

console.log(validate.isArrayOk(movieGenres, options)); // { reasons: [], valid: true, validated: ["action", "comedy", "crime", "horror"] }

const invalids = ["   ", [], null, 144];

console.log(validate.isArrayOk(invalids, options)); // { reasons: ["Expected a non-empty array"], valid: false, validated: undefined }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | arr      | any[]  | The array you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property  | Type     | Description                                                                             |
| --------- | -------- | --------------------------------------------------------------------------------------- |
| empty     | boolean  | Whether array could be empty. Default: **false**                                        |
| filter    | function | Function to filter the array. Default: **(data) => false**                              |
| modifier  | function | Function to modify (format) individual values. Default: **undefined**                   |
| sorted    | boolean  | Whether array should be sorted. Default: **true**                                       |
| sorter    | function | Function to sort values. Default: **undefined**                                         |
| sortOrder | number   | Number used to do comparison check when sorted: true and sorter: undefined              |
| unique    | boolean  | Whether array should contain unique values. Default: **true**                           |
| uniqueKey | string   | A key(property) on objects in array used as unique criteria. e.g: "id". Default: **""** |
