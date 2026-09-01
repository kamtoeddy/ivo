---
title: Options du schéma
sidebar_position: 3
---

# Options du schéma

Les options groupées et transversales aux champs s'attachent à un élément `const _: () = ();`
anonyme directement à l'intérieur du module du schéma -- et non enchaînées à l'appel
`#[ivo_schema(...)]` lui-même. Utilisez-les lorsqu'une règle ou un effet de bord implique plusieurs
champs, ou lorsque vous voulez réagir à l'entité dans son ensemble. Plusieurs attributs d'option
peuvent être empilés sur un même const, ou répartis sur plusieurs.

## `ignore`

Ignore le traitement d'un groupe de champs lax ou virtuels ensemble, selon une condition partagée.
Nécessite au moins deux champs, et s'applique à la fois à `create` et `update`.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod ignore_group_schema {
    struct Fields {
        #[lax(String::new())]
        pub email: String,

        #[lax(String::new())]
        pub phone: String,
    }

    #[ignore(["email", "phone"], |ctx, _opts| {
        ctx.input().email.as_deref() == Some("skip")
    })]
    const _: () = ();
}

fn main() {
    let created = ignore_group_schema::DataInputModel
        .create(
            ignore_group_schema::PartialDataInput {
                email: Some("skip".into()),
                phone: Some("123".into()),
            },
            (),
        )
        .unwrap();

    println!("{:?}", created.data); // DataInput { email: "", phone: "" } -- les deux ignorés, valeurs par défaut utilisées
}
```

Voir [`lax_with_ignore.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
et `#[ignore]` au niveau du champ sur les champs virtuels (page Champs virtuels, section Champs de
la barre latérale).

## `ignore_update`

Même idée que `ignore`, mais évalué uniquement lors des mises à jour. `#[ignore_update([...],
handler)]` nécessite au moins deux champs ; pour ignorer _l'entité entière_ lors d'une mise à jour,
omettez le tableau et utilisez plutôt la forme nue au niveau de l'entité,
`#[ignore_update(handler)]`.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod ignore_update_group_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[lax(0)]
        pub b: i32,
    }

    #[ignore_update(["a", "b"], |ctx, _opts| {
        ctx.input().a == Some(42)
    })]
    const _: () = ();
}

fn main() {
    let data = ignore_update_group_schema::DataInput { a: 42, b: 1 };

    // les deux champs ignorés -> rien ne change réellement -> "rien à mettre à jour"
    let err = ignore_update_group_schema::DataInputModel
        .update(
            data,
            ignore_update_group_schema::PartialDataInput {
                a: Some(42),
                b: Some(2),
            },
            (),
        )
        .unwrap_err();

    assert!(err.errors.is_none()); // `errors` à `None` signifie "rien à mettre à jour", pas un échec de validation
}
```

Voir [`lax_with_ignore_update.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs).

## `required`

Impose qu'au moins un des champs lax/virtuels listés soit fourni. Le gestionnaire ne s'exécute que
lorsque _aucun_ des champs listés n'a été fourni, et retourne `Option<{InputName}Errors>` -- `Some`
fusionne les erreurs par champ dans le payload, `None` signifie que l'exigence ne s'applique pas.
Nécessite au moins deux champs. Couramment utilisé pour des règles du type "fournir un email ou un
téléphone".

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_group_schema {
    struct Fields {
        #[lax(None)]
        pub email: Option<String>,

        #[lax(None)]
        pub phone_number: Option<String>,
    }

    #[required(["email", "phone_number"], |ctx, _opts| {
        if ctx.input().email.is_some() || ctx.input().phone_number.is_some() {
            return None;
        }

        let reason = "provide either an email or a phone number";
        let mut errors = DataInputErrors::new();
        errors.set_email(reason, None);
        errors.set_phone_number(reason, None);
        Some(errors)
    })]
    const _: () = ();
}

fn main() {
    let err = required_group_schema::DataInputModel
        .create(
            required_group_schema::PartialDataInput {
                email: None,
                phone_number: None,
            },
            (),
        )
        .unwrap_err();

    println!("{:?}", err.errors); // "email" et "phone_number" portent tous deux la même raison
}
```

`DataInputErrors` est généré automatiquement aux côtés de `DataInput`/`PartialDataInput`. Voir le
même schéma dans
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## `post_validate`

Validation transversale aux champs, exécutée après le `re_validate` de chaque champ individuel.
Peut aussi retourner des valeurs mises à jour pour les champs du groupe lui-même (`pre_validate`
s'exécute en premier et peut alimenter le `validate` principal avec des valeurs mises à jour).
Nécessite au moins deux champs, parmi les champs lax, requis ou virtuels.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_schema {
    struct Fields {
        #[lax(String::new())]
        pub password: String,

        #[lax(String::new())]
        pub confirm_password: String,
    }

    #[post_validate(["password", "confirm_password"], validate = |ctx, _opts| {
        let input = ctx.input();

        if input.password != input.confirm_password {
            let mut errors = DataInputErrors::new();
            errors.set_confirm_password("passwords do not match", None);
            return Err(errors);
        }

        Ok(None)
    })]
    const _: () = ();
}

fn main() {
    let err = post_validate_schema::DataInputModel
        .create(
            post_validate_schema::PartialDataInput {
                password: Some("a".into()),
                confirm_password: Some("b".into()),
            },
            (),
        )
        .unwrap_err();

    println!("{:?}", err.errors); // {"confirm_password": "passwords do not match"}
}
```

Voir la validation transversale dans
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## `on_success`

Enregistre un gestionnaire qui s'exécute après un `create` ou `update` réussi, via `handle_success()`
sur le handle retourné. La forme nue, sans tableau, se déclenche à chaque succès ;
`#[on_success([...], handler)]` nécessite au moins un champ et se déclenche lorsqu'au moins un des
champs listés fait partie du payload de succès.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod hooks_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[lax(0)]
        pub b: i32,
    }

    #[on_success(|_ctx, _opts| {
        println!("[on_success]: entity created or updated");
    })]
    const _: () = ();

    #[on_success(["a", "b"], |_ctx, _opts| {
        println!("[on_success]: a and/or b changed");
    })]
    const _: () = ();
}

fn main() {
    let created = hooks_schema::DataInputModel
        .create(
            hooks_schema::PartialDataInput {
                a: Some(1),
                b: None,
            },
            (),
        )
        .unwrap();

    created.handle_success(); // affiche les deux lignes ci-dessus
}
```

Voir l'exemple exécutable
[`option_on_success.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/option_on_success.rs)
pour plus de détails, y compris les champs dépendants et virtuels.

## `on_delete`

Enregistre un ou plusieurs gestionnaires qui s'exécutent lorsque la méthode `delete` générée d'un
schéma est invoquée, en plus de tout gestionnaire `#[on_delete]` par champ -- voir la page Cycles
de vie (onDelete) dans la barre latérale.

```rust
#[on_delete(|data, _opts| {
    println!("deleting entity with a = {}", data.a);
})]
const _: () = ();
```

## `timestamps`

Le résolveur partagé et **synchrone** pour les champs
`#[created_at]`/`#[updated_at]`/`#[optional_updated_at]` -- voir la page Horodatages dans la
section Champs de la barre latérale pour la vue d'ensemble.

```rust
#[timestamps(|| chrono::Utc::now())]
const _: () = ();
```

Accepte soit une closure sans argument, soit un chemin de fonction nu
(`#[timestamps(chrono::Utc::now)]`).

## Options de contexte personnalisées

`ctx_options(VotreType)` dans l'appel de la macro fait transiter une valeur de votre propre type
(injection de dépendances, cache, données propres à la requête, ...) à travers chaque gestionnaire
d'un appel `create`/`update`, enveloppée dans un verrou lecture/écriture afin que les gestionnaires
concurrents puissent la partager et la modifier en toute sécurité. Les gestionnaires asynchrones
utilisent `opts.read().await`/`opts.write().await` ; les gestionnaires synchrones utilisent
`opts.read_sync()`/`opts.write_sync()`.

```rust
use ivo::ivo_schema;

#[derive(Clone, Default)]
pub struct AppCtxOptions {
    pub calls: u32,
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(AppCtxOptions)
)]
mod ctx_options_schema {
    use super::AppCtxOptions;

    struct Fields {
        #[required]
        #[validate(|v: String, _, opts| {
            opts.write_sync().calls += 1;
            Ok(Some(v))
        })]
        pub name: String,
    }
}

fn main() {
    let created = ctx_options_schema::DataInputModel
        .create(
            ctx_options_schema::PartialDataInput {
                name: Some("jane".into()),
            },
            AppCtxOptions::default(),
        )
        .unwrap();

    println!(
        "name={:?} calls={}",
        created.data.name,
        created.ctx_options.read_sync().calls
    );
}
```

Passez `()` lorsqu'un schéma ne déclare aucune `ctx_options(...)`, comme dans tous les autres
exemples de cette page. Voir
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs)
pour un exemple complet et réaliste (recherches de dépendances, vérifications d'unicité, et
mutation à travers plusieurs gestionnaires dans le même appel).

## Payloads d'erreur personnalisés avec `IvoErrorSanitizer`

Par défaut, `ivo` retourne les erreurs sous forme de `HashMap<String, FieldError<()>>`. Changez la
forme du payload d'erreur en implémentant `IvoErrorSanitizer` et en le passant via
`error_sanitizer(...)` :

```rust
use std::collections::HashMap;
use ivo::{ivo_schema, IvoErrorPayload, IvoErrorSanitizer};

struct MyErrorSanitizer;

impl IvoErrorSanitizer<()> for MyErrorSanitizer {
    type Metadata = Vec<String>;
    type Payload = HashMap<String, Vec<String>>;

    fn sanitize(payload: IvoErrorPayload<Self::Metadata>, _opts: &()) -> Self::Payload {
        payload
            .into_iter()
            .map(|(name, err)| {
                let mut messages = vec![err.reason];
                if let Some(meta) = err.metadata {
                    messages.extend(meta);
                }
                (name, messages)
            })
            .collect()
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    error_sanitizer(MyErrorSanitizer)
)]
mod sanitized_schema {
    use super::MyErrorSanitizer;

    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v.len() < 3 {
                return Err(("too short".into(), Some(vec!["min length is 3".into()])));
            }
            Ok(None)
        })]
        pub username: String,
    }
}

fn main() {
    let err = sanitized_schema::DataInputModel
        .create(
            sanitized_schema::PartialDataInput {
                username: Some("ab".into()),
            },
            (),
        )
        .unwrap_err();

    println!("{:?}", err.errors); // {"username": ["too short", "min length is 3"]}
}
```

Voir l'exemple complet, incluant un type `ctx_options` personnalisé, dans
[`tests/extras/error_sanitizer.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/extras/error_sanitizer.rs).

## Référence de l'API

Pour la liste exhaustive des signatures et contraintes des options groupées, voir :

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — rustdoc hébergé pour le crate publié.
- **rustdoc local** — exécutez `cargo doc --no-deps --open` depuis le répertoire `rs/` pour
  parcourir la même référence générée localement.
