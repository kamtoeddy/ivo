---
title: Champs constants
---

# Champs constants

Une constante est un champ de sortie exclusif dont la valeur ne doit jamais changer après la
création (par ex. `id`).

- Il doit avoir soit une valeur statique, soit un résolveur.
- Il peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md).

## Exemple

- [Valeurs statiques et dynamiques](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/constants.rs)

## Essayez-le dans le navigateur

`id` est une constante (toujours `1234`) ; `username` est lax avec une valeur par défaut. Modifiez
l'entrée et exécutez.

<RustPlayground demo="constants" />
