---
title: "Propriétés dépendantes"
---

# Propriétés dépendantes

Toute tentative externe de modifier la valeur d'une propriété dépendante sera ignorée ; sa valeur ne peut donc être modifiée que par ses fonctions résolveur.

Une telle propriété `doit` avoir les règles suivantes :

- **default** : Il s'agit d'une valeur ou fonction qui sera utilisée comme valeur par défaut (ou pour générer une valeur par défaut) pour ladite propriété
- **dependsOn** : Au moins une autre propriété ou un [`virtual`](./virtuals.md#propriétés-virtuelles) de votre modèle dont elle doit dépendre. Il peut s'agir d'une chaîne de caractères ou d'un tableau de propriétés.
- **resolver** : Une fonction (synchrone ou asynchrone) qui sera invoquée pour générer la nouvelle valeur de ladite propriété lorsque l'une de ses dépendances change. Cette fonction est invoquée après la dernière étape de validation (post-validation) et l'exécution des [sanitizers](./virtuals.md#sanitiser).
  > N.B : si le résolveur lève une erreur, la valeur de la propriété sera `null` lors de la création, mais si cela se produit lors d'une mise à jour, la propriété sera ignorée

Les propriétés dépendantes peuvent également être utilisées en combinaison avec d'autres règles comme **readonly**, [**gestionnaires de cycle de vie**](../life-cycles.md#gestionnaires-de-cycle-de-vie), etc., mais **`ne peuvent pas être required`**

Exemple :

```ts
import { Schema, type IvoSummary } from "ivo";

type Input = {
  firstName: string;
  lastName: string;
};

type Output = {
  firstName: string;
  fullName: string;
  lastName: string;
};

const userSchema = new Schema<Input, Output>({
  firstName: { required: true, validator: validateName },
  fullName: {
    default: "",
    dependsOn: ["firstName", "lastName"],
    resolver: resolveFullName,
  },
  lastName: { required: true, validator: validateName },
});

function resolveFullName({ ctx }: IvoSummary<Input, Output>) {
  const { firstName, lastName } = ctx;

  return `${firstName} ${lastName}`;
}
```
