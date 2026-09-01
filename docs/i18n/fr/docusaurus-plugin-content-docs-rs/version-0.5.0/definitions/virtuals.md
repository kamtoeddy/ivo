---
title: Champs virtuels
---

# Champs virtuels

Un champ virtuel est un champ exclusif à l'entrée dont la valeur peut ou non être fournie à la
création, utilisé pour déclencher un changement sur un ou plusieurs
[champs dépendants](./dependents.md) qui dépendent de lui -- il est validé/assaini comme un vrai
champ, mais n'est jamais stocké directement sur la struct de sortie.

- Déclaré avec `#[ivo_virtual]`, ou `#[ivo_virtual("alias")]` pour l'exposer sous un nom différent
  sur la struct d'entrée générée.
- Doit être référencé par au moins un champ `#[depends_on(...)]` -- soit par son nom déclaré, soit
  par son alias s'il en a un.
- Doit avoir un [validateur](../validators.md) via `#[validate]` ; peut aussi avoir un
  `#[re_validate]`.
- Peut avoir un mutateur `#[sanitize(|value, ctx, opts| -> T)]`, exécuté après le succès de
  validate/re-validate/post_validate.
- Peut utiliser `#[ignore]`, `#[ignore_init]` et `#[ignore_update]`.
- Peut avoir des gestionnaires d'événements [`on_failure` et `on_success`](../life-cycles.md).
- Nécessite `output(...)` sur le schéma, puisqu'il n'apparaît jamais sur la struct de sortie.

## Exemple : sans alias

`computed` dépend du champ virtuel `trigger` et reflète la valeur qui lui a été donnée :

```rust
use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtuals_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub trigger: String,

        #[depends_on("trigger")]
        #[default(String::new())]
        #[resolve(|ctx, _opts| ctx.input().trigger.clone().unwrap_or_default())]
        pub computed: String,
    }
}

fn main() {
    let created = virtuals_schema::DataModel
        .create(
            virtuals_schema::PartialDataInput {
                trigger: Some("hello".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(created.data.computed, "hello");
    println!("{:?}", created.data); // Data { computed: "hello" }
}
```

## Exemple : avec alias

`#[ivo_virtual("password_confirmation")]` expose le champ sous le nom `password_confirmation` sur
la struct d'entrée, tandis que le champ lui-même reste nommé (et référencé par
`#[depends_on(...)]`) `password_confirm` :

```rust
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtuals_alias_schema {
    struct Fields {
        #[ivo_virtual("password_confirmation")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub password_confirm: String,

        #[depends_on("password_confirm")]
        #[default(String::new())]
        #[resolve(|ctx, _opts| ctx.input().password_confirmation.clone().unwrap_or_default())]
        pub password: String,
    }
}
```

Notez `ctx.input().password_confirmation` ci-dessus -- le champ de la struct d'entrée porte le nom
de l'_alias_, pas le nom déclaré du champ virtuel. `#[depends_on("password_confirm")]` utilise
toujours le nom déclaré (un alias peut aussi coïncider avec le nom d'un champ dépendant existant --
voir l'exemple de collision d'alias ci-dessous).

## Autres exemples

- [Validateurs et re-validateurs](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- [Avec alias](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name.rs)
- [Avec alias identique au dépendant](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name_same_as_dependent.rs)
- [Requis conditionnel](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_update.rs)

## Essayez-le dans le navigateur

`virtual_field` est un champ virtuel d'entrée. La sortie `dependent` utilise sa valeur lorsqu'elle
est fournie, sinon elle retombe sur une valeur par défaut. Laissez `virtual_field` vide ou
retirez-le pour voir la valeur par défaut.

<RustPlayground demo="virtuals" />
