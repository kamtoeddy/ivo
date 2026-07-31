---
title: Champs requis
---

# Champs requis

Un champ requis est à la fois un champ d'entrée et de sortie dont la valeur doit être fournie à la
création (par ex. `username`).

- Il doit avoir un [validateur](../validators.md).
- Il peut aussi avoir un re-validateur.
- Il peut utiliser `ignore_update` et `readonly` pour empêcher de futures mises à jour.
- Il peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md),
  ainsi que [`on_failure`](../life-cycles.md#onfailure).

## Exemples

- [Requis](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required.rs)
- [Erreur requise personnalisée](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_error.rs)
- [Re-validateurs](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_re_validate.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_readonly.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_ignore_update.rs)

## Essayez-le dans le navigateur

`username` est requis sans autre contrainte - laissez l'entrée vide pour voir l'erreur requise, ou
fournissez une valeur pour la voir acceptée.

<RustPlayground demo="required" />
