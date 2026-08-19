---
title: "Propriétés en lecture seule"
---

# Propriétés en lecture seule

La valeur d'une telle propriété ne changera au maximum que deux fois selon votre cas d'utilisation.
Toute tentative de modification de la valeur après qu'elle a changé sera ignorée.

- Si elle est définie sur `true`, elle sera requise lors de l'initialisation et n'autorisera jamais de
  mises à jour.
- Si elle est définie sur `true` avec `shouldInit: false`, elle ne sera pas initialisée mais autorisera
  une seule mise à jour.
- Si elle est définie sur `lax`, elle ne sera pas requise lors de la création ni lors des mises à jour
  (sauf si elle est requise conditionnellement). Lorsque sa valeur diffère de la valeur par défaut,
  elle n'acceptera plus de mises à jour.

Elles **`ne peuvent pas être strictement requises`** mais peuvent être
[requises conditionnellement](./required.md#propriétés-conditionnellement-requises)

Elles doivent avoir une valeur par défaut si elles sont [dépendantes](./dependents.md),
[requises conditionnellement](./required.md#propriétés-conditionnellement-requises) ou si leur
initialisation est bloquée (c'est-à-dire `shouldInit: false`)

Exemple :

```ts
import { Schema } from "ivo";

const orderSchema = new Schema({
  completedAt: {
    default: "",
    readonly: true,
    dependsOn: "isComplete",
    resolver: ({ ctx }) => (ctx.isComplete ? new Date() : ""),
  },
  isComplete: {
    default: false,
    readonly: true,
    shouldInit: false,
    validator: validateBoolean,
  },
  receiptNumber: {
    default: null,
    readonly: "lax",
    validator: validateReceipt,
  },
});
```
