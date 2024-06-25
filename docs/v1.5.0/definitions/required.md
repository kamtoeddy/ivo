# Required Properties

A property with `required: true` is one that must be provided at creation. It must have a validator and cannot have a default value

Example:

```ts
import { Schema } from 'ivo';

const userSchema = new Schema({
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
});
```

## Conditionally Required Properties

```ts
type RequiredError =
  | string
  | { reason?: string | string[]; metadata?: object | null };
```

Such a property is required depending on the summary of the operation. The value of **`required`** must be a function that returns `boolean` | `[boolean, RequiredError]` | `Promise<boolean | [boolean, RequiredError]>`.

> N.B: If the required error is not provided or if the value provided for requiredError is not a string, `[propertyName] is required!` will be used.

> N.B: If nothing is returned, the operation will proceed with `required: false`

> N.B: if the required function happens to throw an error, the operation will proceed with `required: false`

Example:

```ts
import { Schema, type MutableSummary } from 'ivo';

type Book = {
  bookId: string;
  isPublished: boolean;
  price: number | null;
};

const bookSchema = new Schema<Book>({
  bookId: { required: true, validator: validateBookId },
  isPublished: { default: false, validator: validateBoolean },
  price: {
    default: null,
    required({ context: { isPublished, price } }: MutableSummary<Book>) {
      const isRequired = price == null && isPublished;

      return [isRequired, 'A price is required to publish a book!'];
    },
    validator: validatePrice,
  },
});
```
