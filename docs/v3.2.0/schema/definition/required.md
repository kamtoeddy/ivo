# Required Properties

A property with `required === true` is one that must be initialised at creation and cloning. It must have a validator and cannot have a default value (nor setter)

> Out of the box, required is **`false`**

Example:

```ts
import { Schema } from "clean-schema";

const userSchema = new Schema({
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
});
```

## Conditionally Required Properties

Such a property is required depending on the summary of the operation. The value of **`required`** must be a function that returns `boolean` | `[boolean, string | undefined]`.

`[boolean, string]` represents [required, requiredError]. If the required error is not provided, `[propertyName] is required!` would be used.

If nothing is returned, the operation will proceed with `required: false`

Example:

```ts
import { Schema, type GetSummary } from "clean-schema";

type Book = {
  bookId: string;
  isPublished: boolean;
  price: number | null;
};

type Summary = GetSummary<Book>;

const bookSchema = new Schema<Book>({
  bookId: { required: true, validator: validateBookId },
  isPublished: { default: false, validator: validateBoolean },
  price: {
    default: null,
    required({ context: { isPublished, price } }: Summary) {
      const isRequired = price == null && isPublished;

      return [isRequired, "A price is required to publish a book!"];
    },
    validator: validatePrice,
  },
});
```
