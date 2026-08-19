---
title: Champs requis
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs requis

Un champ requis est à la fois un champ d'entrée et de sortie dont la valeur doit être fournie à la
création.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  const UserModel = new Schema<any, { username: string }>((b) =>
    b.field(
      b
        .required("username")
        .validate((value) =>
          value.length >= 3
            ? true
            : { valid: false, reason: "Le nom d'utilisateur doit faire au moins 3 caractères" }
        ),
    ),
  ).getModel();

  const { data, error } = await UserModel.create({ username: "ab" });
  console.log({ data, error: error?.payload });

}

main();`}
/>

## Règles

- Un champ requis doit avoir une règle `validate` ou `allow`.
- Il peut aussi avoir une règle `reValidate` pour les mises à jour.
- Il prend en charge `ignoreUpdate` et `readonly()` pour empêcher les mises à jour ultérieures.
- Il peut avoir des écouteurs `onDelete`, `onSuccess` et `onFailure`.
- Utilisez `requiredError(...)` pour personnaliser le message d'erreur quand le champ est manquant.

## Requis conditionnel

Un champ lax peut être rendu conditionnellement requis :

```ts
b.lax('email', null).required(({ input }) => [
  !input.phoneNumber,
  'Fournissez un email ou un numéro de téléphone',
]);
```
