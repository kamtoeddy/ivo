---
title: Horodatages
---

# Horodatages

Les champs d'horodatage sont des champs exclusifs à la sortie, automatiquement remplis par un
résolveur au niveau du schéma à la création ou à la mise à jour d'un enregistrement.

- `#[created_at]` -- défini une seule fois, à la création.
- `#[updated_at]` -- défini à la création et à chaque mise à jour.
- `#[optional_updated_at]` -- comme `#[updated_at]`, mais typé `Option<T>` et défini seulement
  lorsqu'une mise à jour a réellement lieu ; reste `None` jusque-là.
- Un schéma peut déclarer zéro ou un de chaque. Les deux utilisent le même résolveur partagé et
  **synchrone**, déclaré une seule fois via `#[timestamps(|| ...)]` (ou un chemin de fonction nu)
  sur un élément const anonyme.
- Nécessite `output(...)` sur le schéma, puisque les horodatages n'apparaissent jamais sur la
  struct d'entrée.

## Exemple : noms de champs par défaut

```rust
use chrono::{DateTime, Utc};
use ivo::ivo_schema;

type Timestamp = DateTime<Utc>;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod timestamps_schema {
    use super::Timestamp;
    use chrono::Utc;

    struct Fields {
        #[lax("default_username".to_string())]
        pub username: String,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}

fn main() {
    let created = timestamps_schema::DataModel
        .create(timestamps_schema::PartialDataInput { username: None }, ())
        .unwrap();

    println!("{:#?}", created.data);
    // Data { username: "default_username", created_at: ..., updated_at: ... }
    // created_at == updated_at juste après la création
}
```

## Exemple : noms personnalisés, `updated_at` optionnel

Les champs d'horodatage peuvent porter n'importe quel nom -- c'est l'attribut, pas le nom du champ,
qui compte. Utilisez `#[optional_updated_at]` lorsque "jamais mis à jour" doit être un véritable
état `None` distinct plutôt que de retomber sur l'horodatage de création :

```rust
struct Fields {
    #[lax("default_username".to_string())]
    pub username: String,

    #[created_at]
    pub inserted_at: Timestamp,

    #[optional_updated_at]
    pub modified_at: Option<Timestamp>,
}
```

`modified_at` vaut `None` sur l'enregistrement fraîchement créé, et ne devient `Some(...)` qu'après
la première `update`.

## Autres exemples

- [Noms par défaut](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/timestamps_with_default_names.rs)
- [Noms personnalisés](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/timestamps_with_custom_names.rs)

## Essayez-le dans le navigateur

`username` est un champ lax avec une valeur par défaut. `created_at` et `updated_at` sont remplis
automatiquement par le résolveur d'horodatage.

<RustPlayground demo="timestamps" />
