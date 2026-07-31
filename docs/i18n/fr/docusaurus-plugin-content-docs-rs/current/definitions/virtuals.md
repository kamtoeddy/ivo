---
title: Champs virtuels
---

# Champs virtuels

Un champ virtuel est un champ d'entrée exclusif dont la valeur peut ou non être fournie à la
création, utilisé pour déclencher un changement sur un ou plusieurs champs qui en dépendent.

- Il doit avoir un ou plusieurs [champs dépendants](./dependents.md) qui en dépendent.
- Il doit avoir un [validateur](../validators.md).
- Il peut aussi avoir un re-validateur.
- Il peut avoir un `alias` - un nom de champ différent sur le struct d'entrée, utilisé à la place
  du nom de champ réel (autorisé uniquement si le champ de sortie correspondant est un champ
  dépendant qui dépend directement de ce champ virtuel).
- Il peut avoir un sanitizer.
- Il peut utiliser les règles de provision `ignore`, `ignore_init` et `ignore_update`.
- Il peut avoir des gestionnaires d'événements [`on_failure` et `on_success`](../life-cycles.md).

## Exemples

- [Validateurs et re-validateurs](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- [Avec nom d'alias](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name.rs)
- [Avec nom d'alias identique au dépendant](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name_same_as_dependent.rs)
- [Requis](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_update.rs)
