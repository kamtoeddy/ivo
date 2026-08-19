---
title: Champs dépendants
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs dépendants

Un champ dépendant est un champ purement en sortie dont la valeur change chaque fois qu'au moins un
champ dont il dépend est fourni et accepté.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

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
`}
/>

## Règles

- Un champ dépendant doit avoir une valeur par défaut statique ou un résolver pour cette valeur.
- Il doit dépendre d'au moins un autre champ : lax, required, virtual ou un autre champ dépendant.
- Il doit avoir un résolver pour générer de nouvelles valeurs chaque fois qu'un champ parent est
  fourni et accepté.
- Il peut utiliser `readonly()` pour arrêter d'accepter les mises à jour une fois que sa valeur
  diffère de sa valeur par défaut.
- Il peut avoir des écouteurs `onDelete` et `onSuccess`.
