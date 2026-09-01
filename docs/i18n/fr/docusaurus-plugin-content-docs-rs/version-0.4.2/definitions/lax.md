---
title: Champs lax
---

# Champs lax

Un champ lax est à la fois un champ d'entrée et de sortie dont la valeur peut ou non être fournie
à la création (par ex. `email`, `phone_number`).

- Il doit avoir soit une valeur statique par défaut, soit un résolveur pour la valeur par défaut.
- Il peut avoir un [validateur](../validators.md).
- Il peut aussi avoir un re-validateur.
- Il peut utiliser les règles de provision `ignore`, `ignore_init` et `ignore_update`.
- Il peut utiliser `readonly` si la valeur par défaut est statique.
- Il peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md),
  ainsi que [`on_failure`](../life-cycles.md#onfailure) s'il a un validateur.

## Exemples

- [Valeurs par défaut](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_defaults.rs)
- [Validateurs et re-validateurs](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_readonly.rs)
- [Requis](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs)

## Essayez-le dans le navigateur

`username` a une valeur par défaut statique et aucun validateur - laissez l'entrée vide pour voir
la valeur par défaut s'appliquer.

<RustPlayground demo="lax_defaults" />
