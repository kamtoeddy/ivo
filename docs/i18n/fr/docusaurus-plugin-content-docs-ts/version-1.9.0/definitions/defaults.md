---
title: "Valeurs par défaut"
---

## Valeurs par défaut

La définition de la valeur par défaut d'une propriété donnée peut être effectuée en :

- Renseignant le champ `default` de la définition de la propriété, comme pour `favoriteColor`
- Fournissant une fonction synchrone pour fournir une valeur à l'exécution

  > **`undefined`** est utilisé comme valeur par défaut pour toutes les propriétés dès le départ.

Exemple :

```ts
import { Schema, type SetterFnData } from "ivo";

type SetterData = SetterFnData<Input, Output, CtxOptions>;

const userSchema = new Schema({
  favoriteColor: { default: "indigo", validator: validateColor },
  userName: {
    default: ({ ctx }: SetterData) => "",
    validator: validateUserName,
  },
});
```

> N.B : si la valeur par défaut d'une propriété doit être générée par une fonction et que cette fonction lève une erreur, la valeur de la propriété sera `null`
