## Default Values

Setting the default value of a given property can be done by:

- Populating the default field of the property's definition like `favoriteColor`
- Providing a synchronous function to provide a value at runtime

  > **`undefined`** is used as default value for all properties out of the box.

Example:

```ts
import { Schema } from 'ivo';

const userSchema = new Schema({
  favoriteColor: { default: 'indigo', validator: validateColor },
  userName: { default: (ctx) => '', validator: validateUserName }
});
```
