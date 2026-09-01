---
title: Champs constants
---

# Champs constants

Un champ constant est un champ exclusif à la sortie dont la valeur est calculée une seule fois, à
la création, et n'est jamais acceptée depuis l'entrée ni modifiée par une mise à jour (par ex.
`id`).

- Déclaré avec `#[constant(valeur_ou_résolveur)]` -- une valeur statique, ou un résolveur
  `|ctx, opts| -> T` (synchrone ou asynchrone, avec accès au contexte et aux options comme tout
  autre gestionnaire).
- Nécessite `output(...)` sur le schéma, puisqu'il n'apparaît jamais sur la struct d'entrée.
- Peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md).

## Exemple

`id` est une constante statique. `label` est calculé une seule fois via un résolveur en closure
sans argument :

```rust
use ivo::ivo_schema;

#[ivo_schema(
    input(ItemInput, derive(Debug, Clone, PartialEq)),
    output(Item, derive(Debug, Clone, PartialEq))
)]
mod item_schema {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[constant(|| "generated".to_string())]
        pub label: String,

        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub name: String,
    }
}

fn main() {
    let created = item_schema::ItemModel
        .create(
            item_schema::PartialItemInput {
                name: Some("widget".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(created.data.id, 1234);
    assert_eq!(created.data.label, "generated");

    println!("{:#?}", created.data);
    // Item { id: 1234, label: "generated", name: "widget" }
}
```

Une mise à jour de `item_schema::PartialItemInput` n'a aucun champ `id`/`label` -- il n'y a rien à
soumettre pour une constante, et aucun moyen de la modifier après la création.

## Essayez-le dans le navigateur

`id` est une constante (toujours `1234`) ; `username` est lax avec une valeur par défaut. Modifiez
l'entrée et exécutez.

<RustPlayground demo="constants" />
