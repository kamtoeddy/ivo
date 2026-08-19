---
title: "Propriétés constantes"
---

## Propriétés constantes

Ce type de propriété est défini lors de la création et ne change jamais.

- Il nécessite 2 règles :

  - **`constant`** qui doit être **`true`** et
  - **`value`** qui est soit une **`fixed value`**, soit un setter (fonction synchrone/asynchrone qui retourne une valeur générée)

- Les handlers `onDelete` et `onSuccess` sont les seuls gestionnaires de cycle de vie
  pris en charge par les propriétés constantes. Ces handlers s'exécutent une fois lors
  de la création d'une instance ('onSuccess') et une nouvelle fois lors du 'onDelete'

Exemple :

```ts
import { Schema, type SetterFnData } from "ivo";

type Input = {
  userName: string;
};

type Output = {
  dateJoined: Date;
  id: string;
  role: string;
};

type SetterData = SetterFnData<Input, Output, CtxOptions>;

const userSchema = new Schema<Input, Output>({
  dateJoined: { constant: true, value: () => new Date() },
  id: {
    constant: true,
    value: ({ ctx }: SetterFnData) => `${ctx.userName}-${Date.now}`, // ⚠️ ctx is possibly not safe because it runs before values get validated
  },
  role: { constant: true, value: "user" },
  userName: { required: true, validator: validateUserName },
});
```

> N.B. : si la valeur d'une constante doit être générée par une fonction et que cette fonction lance une erreur, la valeur de la constante sera `null`
