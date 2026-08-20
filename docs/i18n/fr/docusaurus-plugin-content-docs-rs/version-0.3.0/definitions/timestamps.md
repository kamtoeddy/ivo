---
title: Horodatages
---

# Horodatages

Les champs d'horodatage sont des champs de sortie exclusivement peuplés automatiquement par le
schéma lors de la création ou de la mise à jour d'un enregistrement.

- Un schéma peut déclarer un champ `created_at` (défini une seule fois, à la création).
- Un schéma peut déclarer un champ `updated_at` (défini à la création et à chaque mise à jour).
- `updated_at` peut être optionnel, auquel cas il n'est mis à jour que lorsque le champ a déjà une
  valeur.

## Exemples

- [Noms par défaut](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/timestamps_with_default_names.rs)
- [Noms personnalisés](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/timestamps_with_custom_names.rs)

## Essayez dans le navigateur

`username` est un champ lax avec une valeur par défaut. `created_at` et `updated_at` sont peuplés
automatiquement par le résolveur d'horodatage.

<RustPlayground demo="timestamps" />
