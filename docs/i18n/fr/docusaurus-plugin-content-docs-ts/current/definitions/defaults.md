---
title: Valeurs par défaut
---

import TsPlayground from '@site/src/components/TsPlayground';

# Valeurs par défaut

Les valeurs par défaut remplacent les entrées manquantes lors de la création. Elles peuvent être
statiques ou des fonctions résolver.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  const ItemModel = new Schema(
    (b) =>
      b
        .field(b.constant("id", () => "item-1"))
        .field(b.lax("name", "Anonymous"))
        .field(b.lax("createdBy", ({ options }) => options.userId))
        .field(
          b
            .dependent("slug", "name")
            .default("")
            .resolve(({ input }) => input.name!.toLowerCase().replace(/\\s+/g, "-")),
        ),
  ).getModel();

  const { data } = await ItemModel.create({}, { userId: "u-123" });
  console.log(data);

}

main();`}
/>

## Comportement des valeurs par défaut

| Type de champ | Valeur par défaut requise ? | Statique / résolver | Notes                                                            |
| ------------- | --------------------------- | ------------------- | ---------------------------------------------------------------- |
| `lax`         | Oui                         | Les deux            | Utilisée lorsque le champ est absent de l'entrée à la création.  |
| `dependent`   | Oui                         | Les deux            | La valeur par défaut statique est utilisée quand aucune dépendance n'est déclenchée. |
| `constant`    | Valeur requise              | Les deux            | Définie à la création ; les entrées/mises à jour sont ignorées.  |
| `required`    | Non                         | —                   | Doit être fourni par l'appelant.                                 |
| `virtual`     | Non                         | —                   | Champ uniquement en entrée ; utilisez un champ dépendant pour matérialiser une valeur par défaut. |

## Contexte du résolver

Les résolvers de valeur par défaut pour les champs `lax` et `dependent` reçoivent le contexte de
création :

```ts
{
  input: Partial<Input>;     // valeurs d'entrée nettoyées
  rawInput: Partial<Input>;  // valeurs d'entrée d'origine
  options: CtxOptions;       // options de contexte de l'opération
  updateOptions: (updates) => void;
}
```

Si un résolver lève une erreur, la valeur du champ devient `null`.
