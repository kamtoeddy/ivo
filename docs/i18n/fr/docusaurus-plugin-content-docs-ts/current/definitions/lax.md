---
title: Champs lax
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs lax

Un champ lax est à la fois un champ d'entrée et de sortie dont la valeur peut être fournie ou non
à la création.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

const UserModel = new Schema<any, { role: string }>((b) =>
  b.field(b.lax("role", "user")),
).getModel();

const { data } = await UserModel.create({});
console.log(data);
`}
/>

## Règles

- Un champ lax doit avoir une valeur par défaut statique ou une fonction résolver.
- Il peut avoir une règle `validator` et/ou `reValidate`.
- Il peut être conditionnellement requis via `required(handler)`.
- Il prend en charge les règles `ignore`, `ignoreInit` et `ignoreUpdate`.
- Il prend en charge `readonly()` quand la valeur par défaut est statique.
- Il peut avoir des écouteurs `onDelete`, `onSuccess` et `onFailure`.

## Résolvers par défaut

La valeur par défaut peut être une valeur statique ou une fonction recevant le contexte de
l'opération :

```ts
b.lax('timezone', 'UTC');
b.lax('locale', ({ ctx }) => ctx.locale ?? 'en');
```
