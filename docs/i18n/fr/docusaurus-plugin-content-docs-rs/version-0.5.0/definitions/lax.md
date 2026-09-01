---
title: Champs lax
---

# Champs lax

Un champ lax est à la fois un champ d'entrée et de sortie dont la valeur peut ou non être fournie
à la création (par ex. `email`, `phone_number`).

- Déclaré avec `#[lax(valeur_ou_résolveur)]` -- une valeur par défaut statique, ou un résolveur
  `|ctx, opts| -> T`, utilisé chaque fois que le champ est absent.
- Peut avoir un [validateur et un re-validateur](../validators.md) via
  `#[validate]`/`#[re_validate]`.
- Peut utiliser `#[ignore]`, `#[ignore_init]` et `#[ignore_update]` pour ignorer le traitement sous
  condition.
- Peut utiliser `#[readonly]` pour rejeter les mises à jour, si la valeur par défaut est statique.
- Peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md), ainsi
  que [`on_failure`](../life-cycles.md#onfailure) s'il a un validateur.

## Exemple

`bio` retombe sur une valeur par défaut statique lorsqu'elle est absente, et est validée lorsqu'elle
est fournie :

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_schema {
    struct Fields {
        #[lax("default_bio".to_string())]
        #[validate(|v: String, _, _| {
            if v.len() > 100 {
                return Err(("bio too long".into(), None));
            }
            Ok(None)
        })]
        pub bio: String,
    }
}

fn main() {
    let created = lax_schema::DataInputModel
        .create(lax_schema::PartialDataInput { bio: None }, ())
        .unwrap();

    assert_eq!(created.data.bio, "default_bio");
    println!("{:?}", created.data); // DataInput { bio: "default_bio" }
}
```

## Autres exemples

- [Valeurs par défaut](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_defaults.rs)
- [Validateurs et re-validateurs](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_readonly.rs)
- [Requis conditionnel](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs)

## Essayez-le dans le navigateur

`username` a une valeur par défaut statique et aucun validateur - laissez l'entrée vide pour voir
la valeur par défaut s'appliquer.

<RustPlayground demo="lax_defaults" />
