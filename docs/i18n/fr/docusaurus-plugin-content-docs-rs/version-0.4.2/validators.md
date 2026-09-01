---
title: Validateurs
---

# Validateurs

Un validateur est une fonction qui évalue la validité de la valeur d'un champ (un validateur par
champ). Un champ peut avoir jusqu'à deux validateurs (primaire et re-validateur) - voir le
[README racine](https://github.com/kamtoeddy/ivo#resolvers) pour les définitions complètes de
`validator`, `re-validator`, `post-validator` et `required resolver`, qui s'appliquent de la même
manière en Rust qu'en TypeScript.

- Validateurs et re-validateurs : voir les
  [champs lax](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs), les
  [champs requis](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_re_validate.rs)
  et les [champs virtuels](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- Erreurs requises personnalisées : voir
  [cet exemple](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_error.rs)

## Validateurs intégrés

Activez la fonctionnalité optionnelle `validators` (`ivo = { version = "*", features =
["validators"] }`, crate `ivo-validators`) pour un petit ensemble de validateurs intégrés :

- `validate_email(value: &str) -> Result<String, String>`
- `validate_credit_card(value: &str) -> Result<String, String>`

Voir le
[code source du crate](https://github.com/kamtoeddy/ivo/blob/main/rs/crates/validators/src/lib.rs)
pour les détails d'implémentation, et
[`main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/main.rs) pour
un schéma qui les utilise.
