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
appelant `handle_failure()` sur l'`IvoFailureHandle` retourné par un `create` ou `update` échoué.

```rust
let failed = DataInputModel.create(input, ()).unwrap_err();
println!("{:?}", failed.errors);
failed.handle_failure(); // déclenche tout on_failure correspondant ; async si un gestionnaire l'est
```

## `onSuccess`

`#[on_success(|ctx, opts| { ... })]` -- enregistré sur n'importe quel champ individuel, ou pour
[un groupe de champs via l'option de schéma](./options.md#on_success) (la forme nue, sans
tableau, se déclenche à chaque succès quels que soient les champs modifiés). Déclenché en appelant
`handle_success()` sur l'`IvoSuccessHandle` retourné par un `create` ou `update` réussi.

```rust
let created = DataInputModel.create(input, ()).unwrap();
println!("{:?}", created.data);
created.handle_success(); // déclenche le(s) gestionnaire(s) on_success dont les champs ont changé
```

`handle_success`/`handle_failure` n'existent sur le handle retourné que si le schéma déclare au
moins un gestionnaire `on_success`/`on_failure` correspondant *quelque part* -- appeler l'un
d'eux sur un schéma qui n'en a aucun est une erreur de compilation (la méthode n'est pas générée),
pas un no-op silencieux. Une fois qu'elle existe, elle reste sûre à appeler sans condition : un
`on_success` groupé dont les champs n'ont pas changé lors de cet appel ne se déclenche simplement
pas, sans que vous ayez à vérifier au préalable.

## Options de contexte personnalisées

Voir [Démarrage - options de contexte personnalisées](./index.md#options-de-contexte-personnalisées)
pour savoir comment faire transiter des données supplémentaires (injection de dépendances, cache,
i18n, ...) à travers les opérations `create`/`update`/`delete` et jusque dans ces gestionnaires.
