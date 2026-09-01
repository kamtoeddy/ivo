---
title: Validateurs
---

# Validateurs

Un validateur évalue (et peut transformer) la valeur d'un champ. Les champs
[lax](./definitions/lax.md), [requis](./definitions/required.md) et
[virtuels](./definitions/virtuals.md) peuvent chacun en avoir jusqu'à deux : un `#[validate]`
primaire et un `#[re_validate]` secondaire, qui ne s'exécute qu'une fois le validateur primaire
déjà réussi.

Les deux partagent la même signature : `|value, ctx, opts| -> Result<Option<T>, (String,
Option<Metadata>)>`.

- `Ok(None)` accepte la valeur telle quelle.
- `Ok(Some(nouvelle_valeur))` la remplace.
- `Err((raison, metadata))` la rejette -- `metadata` vaut `None` sauf si vous utilisez un
  [`IvoErrorSanitizer` personnalisé](./options.md#payloads-derreur-personnalisés-avec-ivoerrorsanitizer).

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validate_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[re_validate(|v: String, _, _| Ok(Some(format!("revalidated-{v}"))))]
        pub username: String,
    }
}

fn main() {
    let created = re_validate_schema::DataInputModel
        .create(
            re_validate_schema::PartialDataInput {
                username: Some("jane".into()),
            },
            (),
        )
        .unwrap();

    println!("{:?}", created.data); // DataInput { username: "revalidated-jane" }
}
```

`#[re_validate]` nécessite la présence de `#[validate]` sur le même champ -- c'est une erreur de
compilation sinon. Voir un exemple réel (vérifier qu'un nom d'utilisateur n'est pas déjà pris, via
`ctx_options`) dans
[`examples/main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/main_demo/src/domain.rs).

- Validateurs et re-validateurs : voir les
  [champs lax](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/lax_with_validators.rs),
  les
  [champs requis](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/required_with_re_validate.rs)
  et les [champs virtuels](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/virtuals.rs)
- Erreurs requises personnalisées : voir
  [Champs requis](./definitions/required.md#erreur-requise-personnalisée)

## Validateurs intégrés

Activez la fonctionnalité optionnelle `validators` (`ivo = { version = "*", features =
["validators"] }`, crate `ivo-validators`) pour un petit ensemble de validateurs intégrés :

- `validate_email(value: &str) -> Result<String, String>`
- `validate_credit_card(value: &str) -> Result<String, String>`

```rust
#[lax(None)]
#[validate(|v: Option<String>, _, _| {
    let Some(email) = v else { return Ok(None) };
    match validate_email(&email) {
        Ok(validated) => Ok(Some(Some(validated))),
        Err(e) => Err((e, None)),
    }
})]
pub email: Option<String>,
```

Voir le
[code source du crate](https://github.com/kamtoeddy/ivo/blob/main/rs-next/crates/validators/src/lib.rs)
pour les détails d'implémentation, et
[`main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/main_demo/src/domain.rs)
pour un schéma qui les utilise.
