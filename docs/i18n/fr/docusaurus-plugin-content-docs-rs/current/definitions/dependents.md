---
title: Champs dépendants
---

# Champs dépendants

Un champ dépendant est un champ de sortie exclusif dont la valeur change chaque fois qu'au moins
un champ dont il dépend est fourni et accepté (par ex. `username_last_updated_at` ne devrait être
mis à jour que lorsque `username` change).

- Il doit avoir soit une valeur statique par défaut, soit un résolveur pour la valeur par défaut.
- Il doit dépendre d'au moins un autre champ - [lax](./lax.md), [requis](./required.md),
  [virtuel](./virtuals.md), ou un autre champ dépendant (sans dépendance circulaire).
- Il doit avoir un résolveur pour générer de nouvelles valeurs chaque fois qu'un champ parent est
  fourni et accepté.
- Il peut utiliser [`readonly`](https://github.com/kamtoeddy/ivo#readonly) pour ne plus accepter
  de mises à jour une fois que sa valeur diffère de sa valeur par défaut.
- Il peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md).

## Exemples

- [Valeurs par défaut](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_defaults.rs)
- [Dépendant d'un dépendant](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_on_dependent.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_readonly.rs)
