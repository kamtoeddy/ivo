## isArrayOk

You could validate an array of values of your choice. An array of primitives or objects.

```ts
import { isArrayOk } from 'ivo';

const options = {
  empty: false,
  sorted: true,
  filter: (genre) => typeof genre === 'string' && genre?.trim(),
  modifier: (genre) => genre?.trim().toLowerCase()
};

const movieGenres = ['action', null, 'horror', 1, 'comedy', 'Horror', 'crime'];

console.log(isArrayOk(movieGenres, options)); // { valid: true, validated: ["action", "comedy", "crime", "horror"] }

const invalids = ['   ', [], null, 144];

console.log(isArrayOk(invalids, options)); // { reasons: ["Expected a non-empty array"], valid: false }
```

### Parameters

| Position | Property | Type   | Description                                                                |
| -------- | -------- | ------ | -------------------------------------------------------------------------- |
| 1        | value    | any[ ] | The array you wish to validate                                             |
| 2        | options  | object | The options you want to apply for the validation. See its properties below |

### Options

| Property  | Type            | Description                                                                             |
| --------- | --------------- | --------------------------------------------------------------------------------------- |
| empty     | boolean         | Whether array could be empty. Default: **false**                                        |
| filter    | function        | A sync or async function to filter the array. Default: **(data) => false**              |
| modifier  | function        | A sync or async function to modify (format) individual values. Default: **undefined**   |
| sorted    | boolean         | Whether array should be sorted. Default: **true**                                       |
| sorter    | function        | Function to sort values. Default: **undefined**                                         |
| sortOrder | 'asc' \| 'desc' | Order used to do comparison check when sorted: true and sorter: undefined               |
| unique    | boolean         | Whether array should contain unique values. Default: **true**                           |
| uniqueKey | string          | A key(property) on objects in array used as unique criteria. e.g: "id". Default: **""** |
