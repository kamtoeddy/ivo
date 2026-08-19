---
title: Champs en lecture seule
---

import TsPlayground from '@site/src/components/TsPlayground';

# Champs en lecture seule

Marquez un champ comme `readonly()` pour le verrouiller une fois qu'il a divergé de sa valeur par
défaut.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

const CodeModel = new Schema<{ code: string }, { code: string }>(
  (b) => b.field(b.lax("code", "PENDING").readonly()),
).getModel();

const { data: created } = await CodeModel.create({ code: "ABC" });
console.log("created:", created);

const { data: updated } = await CodeModel.update(created, { code: "DEF" });
console.log("updated:", updated);

// Maintenant la valeur a divergé de la valeur par défaut, les mises à jour suivantes sont ignorées.
const { data, error } = await CodeModel.update(
  { ...created, ...updated },
  { code: "GHI" },
);
console.log("second:", { data, error });
`}
/>

## Disponibilité

| Type de champ | `.readonly()` | Notes                                                                       |
| ------------- | ------------- | --------------------------------------------------------------------------- |
| `lax`         | Oui           | Autorisé uniquement quand la valeur par défaut est **statique**, pas une fonction résolver. |
| `required`    | Oui           | Le champ est verrouillé définitivement après la création.                   |
| `dependent`   | Oui           | Autorisé uniquement avec une valeur par défaut statique ; le résolver suit les règles de déverrouillage/verrouillage. |
| `virtual`     | Non           | Les champs uniquement en entrée ne peuvent pas être en lecture seule.       |
| `constant`    | Non           | Les constantes sont déjà immuables.                                         |

## Comportement

- À la **création**, la valeur fournie (ou par défaut) est acceptée normalement.
- Pour les champs avec une valeur par défaut statique, les mises à jour sont acceptées tant que la
  valeur actuelle est égale à cette valeur par défaut. Une fois modifiée, le champ est silencieusement
  ignoré lors des mises à jour suivantes.
- Pour les champs sans valeur par défaut statique (required, ou dependent avec un résolver par défaut),
  le champ est verrouillé immédiatement après la création.
