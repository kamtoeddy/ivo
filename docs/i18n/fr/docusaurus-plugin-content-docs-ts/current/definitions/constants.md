---
title: Champs constants
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs constants

Un champ constant est un champ purement en sortie dont la valeur est définie une fois à la création
et ne change jamais.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  const OrderModel = new Schema<any, { id: string; total: number }>((b) =>
    b
      .field(b.constant("id", () => "order-123"))
      .field(b.lax("total", 0)),
  ).getModel();

  const { data } = await OrderModel.create({ id: "ignored", total: 99 });
  console.log(data);

}

main();`}
/>

## Règles

- Une constante doit avoir une valeur statique ou une fonction résolver.
- Les constantes sont ignorées lorsqu'elles sont fournies en entrée.
- Les constantes ne peuvent pas être mises à jour.
- Elles prennent en charge les écouteurs `onDelete` et `onSuccess`.
