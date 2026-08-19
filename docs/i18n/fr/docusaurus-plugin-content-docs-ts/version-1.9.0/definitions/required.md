---
title: "Propriétés requises"
---

# Propriétés requises

Une propriété avec `required: true` est une propriété qui doit être fournie lors de la création. Elle doit avoir un validateur et ne peut pas avoir de valeur par défaut.

Exemple :

```ts
import { Schema } from "ivo";

const userSchema = new Schema({
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
});
```

## Propriétés conditionnellement requises

```ts
type RequiredError = string | { reason?: string; metadata?: object | null };
```

Une telle propriété est requise en fonction du résumé de l'opération. La valeur de **`required`** doit être une fonction qui retourne `boolean` | `[boolean, RequiredError]` | `Promise<boolean | [boolean, RequiredError]>`.

> N.B. : Si l'erreur de la propriété requise n'est pas fournie ou si la valeur fournie pour `requiredError` n'est pas une chaîne de caractères, `[propertyName] is required!` sera utilisé.

> N.B. : Si aucune valeur n'est retournée, l'opération continuera avec `required: false`.

> N.B. : si la fonction `required` lance une erreur, l'opération continuera avec `required: false`.

Exemple :

```ts
import { Schema, type IvoSummary } from "ivo";

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
    required({ ctx: { isPublished, price } }: IvoSummary<Book>) {
      const isRequired = price == null && isPublished;

      return [isRequired, "A price is required to publish a book!"];
    },
    validator: validatePrice,
  },
});
```
