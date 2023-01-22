# Required Properties

A property with `required === true` is one that must be initialised at creation and cloning. It must have a validator and cannot have a default value (nor setter)

> Out of the box, required is **`false`**

Example:

```ts
import { Schema } from "clean-schema";

const userSchema = new Schema({
  firstName: {
    required: true,
    validator: validateName,
  },
  lastName: {
    required: true,
    validator: validateName,
  },
});
```

## Required By

Such a property is required depending on the context of the operation

Example:

```ts
import { Schema } from "clean-schema";

const bookSchema = new Schema({
  bookId: {
    required: true,
    validator: validateBookId,
  },
  isPublished: {
    default: false,
    validator: validateBoolean,
  },
  price: {
    default: null,
    required: (ctx) => ctx.price == null && ctx.isPublished,
    requiredError: "A price is required to publish a book!",
    validator: validatePrice,
  },
});
```
