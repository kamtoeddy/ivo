---
title: Champs dépendants
---

# Champs dépendants

Un champ dépendant est un champ exclusif à la sortie dont la valeur est recalculée chaque fois
qu'au moins un de ses champs parents déclarés est fourni (par ex. `username_last_updated_at` ne
devrait être mis à jour que lorsque `username` change).

- Déclaré avec `#[depends_on("parent", ...)]` -- au moins un parent, chacun un littéral de chaîne
  nommant un autre champ du schéma ([lax](./lax.md), [requis](./required.md),
  [virtuel](./virtuals.md), ou un autre champ dépendant ; pas de dépendances circulaires).
- Nécessite un résolveur via `#[resolve(|ctx, opts| -> T)]`, exécuté chaque fois qu'un parent
  change.
- Nécessite une valeur par défaut via `#[default(valeur_ou_résolveur)]`, utilisée jusqu'à ce qu'un
  parent change pour la première fois (et comme repli si aucun parent n'a jamais été fourni).
- Peut utiliser `#[readonly]` pour ne plus accepter de changements une fois que sa valeur diffère
  de sa valeur par défaut.
- Peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md).
- Nécessite `output(...)` sur le schéma, puisqu'il n'apparaît jamais sur la struct d'entrée.

## Exemple

`computed` dépend de `value` (un champ lax dont la valeur par défaut est `0`) et se calcule comme
`value + 1` :

```rust
use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependents_schema {
    struct Fields {
        #[lax(0)]
        pub value: i32,

        #[depends_on("value")]
        #[default(1)]
        #[resolve(|ctx, _opts| ctx.values().value + 1)]
        pub computed: i32,
    }
}

fn main() {
    // `value` prend la valeur par défaut 0 -- mais la valeur par défaut d'un champ lax compte
    // tout de même comme "fournie" pour sa propre résolution, donc `computed` se résout quand
    // même une fois : 0 + 1 = 1.
    let created = dependents_schema::DataModel
        .create(dependents_schema::PartialDataInput { value: None }, ())
        .unwrap();
    println!("{:?}", created.data); // Data { value: 0, computed: 1 }

    let created = dependents_schema::DataModel
        .create(
            dependents_schema::PartialDataInput { value: Some(5) },
            (),
        )
        .unwrap();
    println!("{:?}", created.data); // Data { value: 5, computed: 6 }

    let updated = dependents_schema::DataModel
        .update(
            created.data,
            dependents_schema::PartialDataInput { value: Some(10) },
            (),
        )
        .unwrap();
    println!("{:?}", updated.data); // PartialData { value: Some(10), computed: Some(11) }
}
```

`ctx.values()` à l'intérieur du résolveur donne accès à chaque champ déjà résolu dans le même appel
`create`/`update`, y compris les champs dépendants voisins résolus plus tôt dans le graphe de
dépendances -- voir [Pipeline d'exécution](../execution-pipeline.md) pour l'ordre exact des
phases.

## Autres exemples

- [Valeurs par défaut](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_defaults.rs)
- [Dépendant d'un dépendant](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_on_dependent.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_readonly.rs)

## Essayez-le dans le navigateur

`value` est un champ lax avec une valeur par défaut de `0`. `computed` est un champ dépendant qui
vaut `value + 1` (avec sa propre valeur par défaut de secours de `1`).

<RustPlayground demo="dependents" />
