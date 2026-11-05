---
title: Champs dépendants
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs dépendants

Un champ dépendant est un champ purement en sortie dont la valeur change chaque fois qu'au moins un
champ dont il dépend change -- voir [quand le résolveur s'exécute](#quand-le-résolveur-sexécute).

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  type Input = { firstName?: string; lastName?: string };
  type Output = { firstName: string; lastName: string; fullName: string };

  const UserModel = new Schema<Input, Output>((b) =>
    b
      .field(b.lax("firstName", ""))
      .field(b.lax("lastName", ""))
      .field(
        b
          .dependent("fullName", ["firstName", "lastName"])
          .default("")
          .resolve(({ ctx }) => \`\${ctx.firstName} \${ctx.lastName}\`.trim()),
      ),
  ).getModel();

  const { data } = await UserModel.create({ firstName: "Ada", lastName: "Lovelace" });
  console.log(data);

}

main();`}
/>

## Quand le résolveur s'exécute

- **À la création**, un champ dépendant est résolu si au moins un de ses parents est :
  - un champ requis ou virtuel, fourni et accepté ;
  - un champ lax, fourni et accepté avec une valeur différente de sa valeur par défaut statique
    (un champ lax dont la valeur par défaut est un résolveur -- une fonction -- compte dès
    qu'il est fourni, car sa valeur par défaut n'est pas connue sans l'appeler) ;
  - un champ dépendant, résolu à une valeur différente de sa valeur par défaut.
- **Lors d'une mise à jour**, un champ dépendant est résolu si au moins un de ses parents a été
  fourni et accepté, ou résolu, avec une valeur différente de la précédente. Un champ virtuel
  fourni compte toujours, puisqu'il n'a pas de valeur précédente.

Un champ dépendant résolu à la valeur qu'il avait déjà n'est pas un changement : les champs qui en
dépendent ne sont donc pas résolus.

## Règles

- Un champ dépendant doit avoir une valeur par défaut statique ou un résolver pour cette valeur.
- Il doit dépendre d'au moins un autre champ : lax, required, virtual ou un autre champ dépendant.
- Il doit avoir un résolver pour générer de nouvelles valeurs chaque fois qu'un champ parent change
  (voir [quand le résolveur s'exécute](#quand-le-résolveur-sexécute)).
- Il peut utiliser `readonly()` pour arrêter d'accepter les mises à jour une fois que sa valeur
  diffère de sa valeur par défaut.
- Il peut avoir des écouteurs `onDelete` et `onSuccess`.
