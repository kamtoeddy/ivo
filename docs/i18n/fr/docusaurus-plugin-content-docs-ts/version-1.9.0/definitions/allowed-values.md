---
title: "Valeurs autorisées"
---

## Valeurs autorisées

Cette option sert à spécifier les valeurs acceptées pour une propriété. Elle peut être utilisée sans validateur, mais si un validateur est fourni, les valeurs sont d'abord vérifiées par rapport à cette liste avant d'être transmises au validateur

Exemple:

```ts
import { Schema } from "ivo";

const userSchema = new Schema({
  role: { default: "user", allow: ["admin", "moderator", "user"] },
  name: { required: true, validator: validateName },
});
```

## `NotAllowedError`:

Si vous devez définir des erreurs personnalisées pour gérer les valeurs invalides, vous pouvez le faire comme dans l'exemple ci-dessous

```ts
import { Schema } from "ivo";

const userSchema = new Schema({
  role: {
    default: "user",
    allow: {
      values: ["admin", "moderator", "user"],
      error: "Invalid role provided",
    },
  },
  name: { required: true, validator: validateName },
});

// l'erreur ci-dessus peut correspondre au type suivant
type NotAllowedError =
  | string
  | InputFieldError
  | ((valueProvided: any, allowedValues: any[]) => string | InputFieldError);
```

> N.B: si la valeur du `NotAllowedError` doit être générée par une fonction et que cette fonction lève une erreur, le message d'erreur par défaut sera utilisé
