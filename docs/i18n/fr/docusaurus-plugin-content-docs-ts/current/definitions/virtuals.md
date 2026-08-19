---
title: Champs virtuels
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs virtuels

Un champ virtuel est un champ purement en entrée qui peut être fourni ou non, utilisé pour déclencher
un changement dans un ou plusieurs champs qui en dépendent.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

type Input = { rawEmail?: string };
type Output = { email: string };

const UserModel = new Schema<Input, Output>((b) =>
  b
    .field(
      b
        .virtual("rawEmail")
        .sanitize((value) => value.trim().toLowerCase())
        .validate((value) => ({ valid: value.includes("@") }))
        .alias("email"),
    )
    .field(
      b
        .dependent("email", "rawEmail")
        .default("")
        .resolve(({ input }) => input.rawEmail ?? ""),
    ),
).getModel();

const { data } = await UserModel.create({ email: "Ada@Example.COM" });
console.log(data);
`}
/>

## Règles

- Un champ virtuel doit avoir au moins un champ dépendant qui en dépend.
- Il doit avoir une règle `validate` ou `allow`.
- Il peut avoir une règle `reValidate`.
- Il peut avoir un `sanitizer` pour transformer l'entrée brute avant que les dépendants ne soient résolus.
- Il peut avoir un `alias` utilisé comme nom de champ d'entrée.
- Il prend en charge les règles `ignore`, `ignoreInit` et `ignoreUpdate`.
- Il peut avoir des écouteurs `onSuccess` et `onFailure`.
