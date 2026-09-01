---
title: Cycles de vie
---

# Cycles de vie

`ivo` permet de réagir aux changements d'une entité métier ou de ses champs individuels. Les
concepts ci-dessous sont partagés entre les deux implémentations - voir le
[README racine](https://github.com/kamtoeddy/ivo#lifecycle-events) pour les définitions complètes
et indépendantes du langage. Cette page explique comment les mettre en place en Rust.

## `onDelete`

`#[on_delete(|data, opts| { ... })]` -- déclenché directement en appelant la méthode `delete`
générée d'un schéma. Abonnez-vous par champ de sortie, ou pour l'entité entière via
[l'option de schéma `on_delete`](./options.md#on_delete). `delete` n'est généré que lorsque le
schéma déclare au moins un gestionnaire `on_delete` (au niveau du champ ou du schéma), et n'est
`async` que si l'un d'eux l'est.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod delete_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[on_delete(|data, _opts| {
            println!("[username]: on_delete: {}", data.username);
        })]
        pub username: String,
    }
}

fn main() {
    let data = delete_schema::DataInput {
        username: "jane".into(),
    };

    delete_schema::DataInputModel.delete(&data, ());
}
```

## `onFailure`

`#[on_failure(|ctx, opts| { ... })]` -- enregistré sur un champ ayant un validateur, déclenché en
appelant le handle retourné comme troisième élément du tuple `Err` d'un `create` ou `update`
échoué.

## `onSuccess`

`#[on_success(|ctx, opts| { ... })]` -- enregistré sur n'importe quel champ individuel, ou pour
[un groupe de champs via l'option de schéma](./options.md#on_success) (la forme nue, sans
tableau, se déclenche à chaque succès quels que soient les champs modifiés). Déclenché en appelant
le handle retourné comme troisième élément du tuple `Ok` d'un `create` ou `update` réussi.

## Déclencher les gestionnaires

`create`/`update` retournent `(data, ctx_options)` lorsque le schéma n'a aucun gestionnaire
`on_success`/`on_failure` correspondant nulle part, et `(data, ctx_options, handle)` lorsque c'est
le cas -- appeler `handle` déclenche tous les gestionnaires correspondants pour cet appel. `handle`
est un simple `FnOnce()` si tous les gestionnaires capturés sont synchrones, ou
`FnOnce() -> impl Future<Output = ()>` (appelez-le, puis faites `.await` sur le résultat) si l'un
d'eux est asynchrone -- résolu une fois par schéma à la compilation, pas via une vérification à
l'exécution.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod notify_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v.is_empty() {
                return Err(("username must not be empty".into(), None));
            }
            Ok(Some(v))
        })]
        #[on_success(|ctx, _| {
            println!("[username]: on_success: {}", ctx.values().username);
        })]
        #[on_failure(|ctx, _| {
            println!("[username]: on_failure: {:?}", ctx.input().username);
        })]
        pub username: String,
    }
}

fn main() {
    let (created, _ctx_options, handle_success) = notify_schema::DataInputModel
        .create(notify_schema::DataInput { username: "jane".into() }, ())
        .ok()
        .unwrap();
    println!("{:?}", created);
    handle_success(); // déclenche le gestionnaire on_success correspondant

    let (errors, _ctx_options, handle_failure) = notify_schema::DataInputModel
        .create(notify_schema::DataInput { username: "".into() }, ())
        .err()
        .unwrap();
    println!("{:?}", errors);
    handle_failure(); // déclenche le gestionnaire on_failure correspondant
}
```

`Result::unwrap()`/`unwrap_err()` exigent `Debug` sur l'autre variante du `Result`, ce que la
fermeture du gestionnaire ne peut pas fournir -- utilisez `.ok().unwrap()` / `.err().unwrap()` à la
place (`Option::unwrap()` n'a pas cette contrainte). Lorsqu'un schéma n'a ni gestionnaire
`on_success` ni `on_failure`, le tuple n'a pas d'élément de déclenchement et
`.unwrap()`/`.unwrap_err()` classiques fonctionnent très bien, comme dans l'exemple de
[Démarrage](./index.md#démarrage-rapide).

## Options de contexte personnalisées

Voir [Démarrage - options de contexte personnalisées](./index.md#options-de-contexte-personnalisées)
pour savoir comment faire transiter des données supplémentaires (injection de dépendances, cache,
i18n, ...) à travers les opérations `create`/`update`/`delete` et jusque dans ces gestionnaires.
