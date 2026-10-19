---
title: Cycles de vie
---

import TsPlayground from '@site/src/components/TsPlayground';

# Cycles de vie

`ivo` expose des hooks à différentes étapes d'une opération.

## Contexte d'opération

Les validateurs, résolveurs et écouteurs reçoivent un objet de contexte (`ctx`) avec l'état de
l'opération :

```ts
{
  input: Partial<Input>; // valeurs d'entrée nettoyées
  rawInput: Partial<Input>; // valeurs d'entrée d'origine
  values: Output; // valeurs actuelles (en mise à jour : anciennes valeurs + modifications)
  isUpdate: boolean;
  changes: Partial<Output> | null; // en mise à jour, les modifications ; null à la création
  previousValues: Output | null; // en mise à jour, les valeurs avant la mise à jour ; null à la création
  options: CtxOptions; // une nouvelle copie à chaque lecture
  updateOptions: (options: Partial<CtxOptions>) => void; // non fourni aux écouteurs
}
```

Les résolveurs de constantes, de valeurs par défaut et `ignoreUpdate` reçoivent un contexte plus
réduit (`input`, `rawInput`, `options` et `updateOptions`, plus `values` pour les constantes et
`previousValues` pour `ignoreUpdate`).

Le contexte est figé (`Object.freeze`) et ses propriétés sont des accesseurs (getters). Lisez-les
directement ou par déstructuration :

```ts
.validate((value, ctx) => {
  const { isUpdate, values } = ctx; // ✅
  // ...
});
```

:::caution[Depuis la v2.1.0]
Les accesseurs sont définis sur le prototype du contexte et non sur l'objet lui-même : `{ ...ctx }`,
`Object.keys(ctx)` et `JSON.stringify(ctx)` ne les listent plus. Copiez plutôt les propriétés
dont vous avez besoin, p. ex. `{ values: ctx.values, isUpdate: ctx.isUpdate }`.
:::

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
