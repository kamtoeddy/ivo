---
title: Cycles de vie
---

import TsPlayground from '@site/src/components/TsPlayground';

# Cycles de vie

`ivo` expose des hooks à différentes étapes d'une opération.

## Contexte d'opération

Les gestionnaires reçoivent un objet de contexte avec l'état utile de l'opération :

```ts
{
  input: Input; // valeurs d'entrée nettoyées
  rawInput: Input; // valeurs d'entrée d'origine
  values: Output; // valeurs de sortie actuelles (valeurs par défaut et dépendants résolus inclus)
  isUpdate: boolean;
  summary: IvoSummary<Input, Output>;
}
```

## Écouteurs globaux

Définissez-les dans les options du schéma :

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  const UserModel = new Schema(
    (b) => b.field(b.lax("name", "Anonymous")),
    {
      onSuccess: (summary) => console.log("success:", summary.values),
      onDelete: (data) => console.log("deleted:", data),
    },
  ).getModel();

  const { data } = await UserModel.create({ name: "Ada" });
  await UserModel.delete(data!);

}

main();`}
/>

## Écouteurs de champ

Les constructeurs de champs prennent en charge `onSuccess`, `onFailure` et `onDelete` :

```ts
b.required("username")
  .validate(validateUsername)
  .onSuccess((summary) => console.log("username validated", summary))
  .onFailure((summary) => console.log("username failed", summary));
```

## Post-validation

Utilisez `postValidate` pour exécuter une validation transversale après les validateurs de champs :

```ts
new Schema((b) => ..., {
  postValidate: {
    fields: ['email', 'phoneNumber'],
    validator: ({ input }) => [
      !input.email && !input.phoneNumber,
      'Fournissez un email ou un numéro de téléphone',
    ],
  },
});
```

## Options de contexte personnalisées

Passez des données supplémentaires à chaque opération :

```ts
const UserSchema = new Schema<Input, Output, { db: Database }>((b) => ..., {
  // définition du schéma
});

const UserModel = UserSchema.getModel();

const { data, error } = await UserModel.create(input, { db: usersDb });
```

À l'intérieur des validateurs et résolvers, accédez aux options via le contexte :

```ts
b.constant("id", ({ options }) => options.db.nextId());
```
