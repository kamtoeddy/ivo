# Required Properties

A property with `required === true` is one that must be initialised at creation and cloning. It must have a validator and cannot have a default value (nor setter)

> Out of the box, required is **`false`**

Example:

```ts
import { Schema } from 'ivo';

const userSchema = new Schema({
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName }
});
```

## Conditionally Required Properties

```ts
type RequiredError =
  | string
  | { reason?: string | string[]; metadata?: object | null };
```

Such a property is required depending on the summary of the operation. The value of **`required`** must be a function that returns `boolean` | `[boolean, RequiredError]`.

If the required error is not provided or if the value provided for requiredError is not a string, `[propertyName] is required!` would be used.

If nothing is returned, the operation will proceed with `required: false`

Example:

```ts
import { Schema, type Summary } from 'ivo';

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
    required({ context: { isPublished, price } }: Summary<Book>) {
      const isRequired = price == null && isPublished;

      return [isRequired, 'A price is required to publish a book!'];
    },
    validator: validatePrice
  }
});
```
