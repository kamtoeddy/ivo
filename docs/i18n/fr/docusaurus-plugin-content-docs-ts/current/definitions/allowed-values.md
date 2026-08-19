---
title: Valeurs autorisées
---

import TsPlayground from '@site/src/components/TsPlayground';

# Valeurs autorisées

Restreignez un champ à un ensemble fixe de valeurs autorisées au lieu d'écrire un validateur
personnalisé.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  const UserModel = new Schema(
    (b) =>
      b
        .field(b.required("role").allow(["admin", "editor", "viewer"]))
        .field(b.lax("status", "draft").allow(["draft", "published", "archived"])),
  ).getModel();

  const { data, error } = await UserModel.create({ role: "admin" });
  console.log("created:", data);

  const { error: invalid } = await UserModel.create({ role: "superuser" });
  console.log("invalid reason:", invalid?.payload?.role?.reason);
  console.log("invalid metadata:", invalid?.payload?.role?.metadata);

}

main();`}
/>

## Disponibilité

| Type de champ | `.allow()` | Notes                                              |
| ------------- | ---------- | -------------------------------------------------- |
| `required`    | Oui        | Peut remplacer `.validate()` entièrement.          |
| `lax`         | Oui        | Peut remplacer `.validate()` entièrement.          |
| `virtual`     | Oui        | Doit d'abord appeler `.validate()`, puis `.allow()`. |
| `dependent`   | Non        | Les champs dépendants sont résolus, pas restreints.|
| `constant`    | Non        | Les constantes sont définies par le schéma.        |

## Règles

- Le tableau doit contenir au moins **deux** valeurs.
- La valeur par défaut statique d'un champ `lax` doit faire partie des valeurs autorisées.
- Les valeurs sont comparées avec `equalityDepth` configuré.
- Utilisez `.allowError()` pour personnaliser le message d'erreur.

```ts
b.required('role')
  .allow(['admin', 'editor'])
  .allowError((value, allowed) => `"${value}" n'est pas un rôle valide`);
```
