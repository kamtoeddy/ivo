---
title: Champs requis
---

# Champs requis

Un champ requis est à la fois un champ d'entrée et de sortie dont la valeur doit être fournie à la
création (par ex. `username`). Il est optionnel (et immuable sauf autorisation explicite) lors
d'une mise à jour.

- Déclaré avec l'attribut de type de champ nu `#[required]`.
- Doit avoir un [validateur](../validators.md) via `#[validate]` ; peut aussi avoir un
  `#[re_validate]`.
- Peut personnaliser l'erreur de champ manquant via `#[required_error(...)]` -- une chaîne
  statique ou une closure `|raw_input, opts| -> String`.
- Peut utiliser `#[ignore_update]` (forme résolveur uniquement) et `#[readonly]` pour empêcher de
  futures mises à jour.
- Peut avoir des gestionnaires d'événements [`on_delete` et `on_success`](../life-cycles.md), ainsi
  que [`on_failure`](../life-cycles.md#onfailure).

## Exemple

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v.len() < 3 {
                return Err(("username too short".into(), None));
            }
            Ok(None)
        })]
        pub username: String,
    }
}

fn main() {
    let (err, _ctx_options) = required_schema::DataInputModel
        .create(required_schema::PartialDataInput { username: None }, ())
        .unwrap_err();
    println!("{:?}", err.get("username").unwrap().reason); // "field is required"

    let (created, _ctx_options) = required_schema::DataInputModel
        .create(
            required_schema::PartialDataInput {
                username: Some("jane".into()),
            },
            (),
        )
        .unwrap();
    println!("{:?}", created); // DataInput { username: "jane" }
}
```

## Erreur requise personnalisée

```rust
#[required]
#[required_error(|_raw_input, _opts| "\"username\" was not provided!".to_string())]
#[validate(|v: String, _, _| Ok(Some(v)))]
pub username: String,
```

## Autres exemples

- [Requis](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required.rs)
- [Erreur requise personnalisée](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_error.rs)
- [Re-validateurs](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_re_validate.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_readonly.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_ignore_update.rs)

## Essayez-le dans le navigateur

`username` est requis sans autre contrainte - laissez l'entrée vide pour voir l'erreur requise, ou
fournissez une valeur pour la voir acceptée.

<RustPlayground demo="required" />
