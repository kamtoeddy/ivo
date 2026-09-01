---
title: Démarrage
slug: /
---

# Démarrage

Cette documentation couvre `ivo` pour Rust **v0.5.0**.

Les schémas se déclarent, ils ne se construisent pas de façon impérative : une seule macro
d'attribut, `#[ivo_schema(...)]`, prend un module contenant vos déclarations de champs et génère
les structs d'entrée/sortie, leurs équivalents partiels/erreurs, ainsi qu'un modèle typé et
spécifique au schéma avec des méthodes `create`/`update`/`delete`.

## Installation

```bash
cargo add ivo
```

## Démarrage rapide

```rust
use chrono::{DateTime, Utc};
use ivo::ivo_schema;

type Timestamp = DateTime<Utc>;

#[ivo_schema(
    input(PostInput, derive(Debug, Clone, PartialEq)),
    output(Post, derive(Debug, Clone, PartialEq))
)]
mod post_schema {
    use super::Timestamp;
    use chrono::Utc;

    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,

        #[required]
        #[validate(|title: String, _, _| {
            if title.trim().len() < 3 {
                return Err(("title must be at least 3 characters long".into(), None));
            }
            Ok(Some(title.trim().to_string()))
        })]
        pub title: String,

        #[lax(String::new())]
        pub body: String,
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}

use post_schema::{PartialPostInput, PostModel};

fn main() {
    let created = PostModel
        .create(
            PartialPostInput {
                title: Some("Hello, ivo!".into()),
                body: Some("My first post.".into()),
            },
            (), // ctx_options -- `()` quand le schéma n'en déclare aucune
        )
        .unwrap();

    println!("{:#?}", created.data); // -> Post { id, created_at, updated_at, title, body }

    let updated = PostModel
        .update(
            created.data,
            PartialPostInput {
                title: None,
                body: Some("Edited.".into()),
            },
            (),
        )
        .unwrap();

    println!("{:#?}", updated.data); // -> PartialPost { body: Some("Edited."), .. le reste à None }
}
```

- `input(...)` nomme la struct d'entrée générée et est toujours obligatoire ; `output(...)` nomme
  la struct de sortie et n'est obligatoire que lorsque le schéma a des champs exclusifs à l'entrée
  (`#[ivo_virtual]`) ou à la sortie (`#[constant]`, `#[depends_on(...)]`, horodatages). Un schéma
  ne contenant que des champs `#[required]`/`#[lax]` peut omettre `output(...)` entièrement et
  utiliser une seule struct pour les deux.
- `derive(...)` ajoute des dérivations à la struct générée ; `derive_partial(...)` les ajoute à son
  équivalent partiel (par ex. pour dériver `Serialize`/`Deserialize` pour le transport réseau).
- La macro génère une valeur unité `{OutputName}Model` (ou `{InputName}Model` pour un schéma à une
  seule struct) -- `post_schema::PostModel.create(...)` fonctionne directement, sans `::new()`.
- `create`/`update`/`delete` ne sont `async` que si au moins un gestionnaire qu'ils invoquent est
  asynchrone -- sinon, la méthode générée (et tout `handle_success`/`handle_failure` qu'elle
  retourne) est purement synchrone, sans imposer de dépendance à un runtime.

## Définir un schéma

Les champs d'un schéma appartiennent à l'une de six catégories - voir chacune pour les règles et
un exemple exécutable :

- [Champs constants](./definitions/constants.md)
- [Champs dépendants](./definitions/dependents.md)
- [Champs lax](./definitions/lax.md)
- [Champs requis](./definitions/required.md)
- [Horodatages](./definitions/timestamps.md)
- [Champs virtuels](./definitions/virtuals.md)

Voir [Validateurs](./validators.md) pour le fonctionnement de `#[validate]`/`#[re_validate]`, et
[Cycles de vie](./life-cycles.md) pour `#[on_success]`/`#[on_failure]`/`#[on_delete]`.

## Options du schéma

Le comportement groupé et transversal aux champs s'attache à un élément `const _: () = ();`
anonyme à l'intérieur du module du schéma, et non enchaîné à l'appel de la macro -- voir
[Options du schéma](./options.md) pour `ignore`, `ignore_update`, `required`, `post_validate`,
`on_success`, `on_delete` et `timestamps`, chacune avec un exemple exécutable.

## Options de contexte personnalisées

`ctx_options(VotreType)` fait transiter une valeur de votre propre type (injection de
dépendances, cache, données propres à la requête, ...) à travers chaque gestionnaire d'un appel
`create`/`update`. Voir
[Options du schéma - Options de contexte personnalisées](./options.md#options-de-contexte-personnalisées),
ou la démo complète dans
[`examples/main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## `ErrorSanitizer` personnalisé

Le payload par défaut retourné pour les opérations échouées a la signature suivante :

```rust
type DefaultFieldErrorMetadata = ();

struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

type IvoErrorPayload<Metadata> = HashMap<String, FieldError<Metadata>>;
```

Pour personnaliser ce payload, fournissez une implémentation du trait `IvoErrorSanitizer` via
`error_sanitizer(VotreSanitizer)` -- voir
[Options du schéma - Payloads d'erreur personnalisés](./options.md#payloads-derreur-personnalisés-avec-ivoerrorsanitizer).

## Référence de l'API

La documentation narrative ci-dessus couvre les concepts de haut niveau. Pour la référence
exhaustive de l'API générée (types, fonctions, macros dérivées), voir :

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — rustdoc hébergé pour le crate publié.
- **[crates.io/crates/ivo](https://crates.io/crates/ivo)** — page du registre de crates (versions,
  dépendances, README).
- **rustdoc local** — exécutez `cargo doc --no-deps --open` depuis le répertoire `rs/` pour
  parcourir la même référence générée localement.
