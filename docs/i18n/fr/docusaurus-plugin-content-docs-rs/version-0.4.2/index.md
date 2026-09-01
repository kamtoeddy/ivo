---
title: Démarrage
slug: /
---

# Démarrage

`ivo` pour Rust attend que vous définissiez votre modèle de données avec des structs qui
implémentent `IvoInputStruct` (requis pour les structs d'entrée) et `IvoStruct`. Cela se fait via
leurs macros dérivées respectives.

## Installation

```bash
cargo add ivo
```

## Définir des structs

```rs
use chrono::{DateTime, Utc};
use ivo::{IvoInputStruct, IvoStruct};

#[derive(Clone, PartialEq, IvoInputStruct)]
struct UserInput {
    email: Option<String>,
    phone_number: Option<String>,
    username: String,
}

type Timestamp = DateTime<Utc>;

#[derive(Clone, PartialEq, IvoStruct)]
struct User {
    id: String,
    created_at: Timestamp,
    email: Option<String>,
    phone_number: Option<String>,
    updated_at: Option<Timestamp>,
    username: String,
    username_last_updated_at: Option<Timestamp>,
}
```

### `IvoStruct`

Dériver `IvoStruct` sur `User` génère un struct `PartialUser`, ainsi que des méthodes utilitaires :

```rs
impl IvoStruct for User {
    fn append_updates(&mut self, updates: &Self::Partial);
    fn clone_with_updates(&self, updates: &Self::Partial) -> Self;
}

impl From<User> for PartialUser {
    fn from(value: User) -> PartialUser;
}
```

`PartialUser` obtient un constructeur, des méthodes builder `set_*`/`with_*` et des méthodes
`unset_*` pour chaque champ, ainsi que `into_option()` et `is_empty()` :

```rs
struct PartialUser {
    id: Option<String>,
    created_at: Option<Timestamp>,
    email: Option<String>,
    phone_number: Option<Option<String>>,
    updated_at: Option<Option<Timestamp>>,
    username: Option<String>,
    username_last_updated_at: Option<Option<Timestamp>>,
}
```

L'attribut `#[ivo(...)]` permet de personnaliser les structs partiels générés et leurs champs, par
exemple pour dériver `Serialize`/`Deserialize` ou transmettre des attributs `#[serde(...)]` aux
champs générés - voir le
[README Rust](https://github.com/kamtoeddy/ivo/blob/main/rs/README.md#ivostruct) pour l'exemple
complet.

### `IvoInputStruct`

Dériver `IvoInputStruct` sur `UserInput` implémente automatiquement `IvoStruct` et génère en plus
un struct `UserInputErrors`, utilisé pour retourner les erreurs des
[post-validateurs](https://github.com/kamtoeddy/ivo#post-validator) et des résolveurs de champs
requis groupés.

## Définir un schéma

Les champs d'un schéma appartiennent à l'une des six catégories suivantes - consultez chacune
pour les règles et un exemple exécutable :

- [Champs constants](./definitions/constants.md)
- [Champs dépendants](./definitions/dependents.md)
- [Champs lax](./definitions/lax.md)
- [Champs requis](./definitions/required.md)
- [Horodatages](./definitions/timestamps.md)
- [Champs virtuels](./definitions/virtuals.md)

## Options du schéma

- **Ignore (groupé)** : avec les
  [champs lax](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/ignore.rs) ou les
  [champs virtuels](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/virtuals/ignore.rs)
- **Ignore update (groupé)** : pour
  [l'entité entière](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/opions/mod.rs), avec les
  [champs lax](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/ignore.rs) ou les
  [champs requis](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/required/ignore.rs)
- **Required (groupé)** : avec les
  [champs lax](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/mod.rs) ou les
  [champs virtuels](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/virtuals/mod.rs)
- **Post-validate** : avec les
  [champs lax](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/mod.rs), les
  [champs requis](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/required/mod.rs) ou
  les [champs virtuels](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/virtuals/mod.rs)
- **On success / on delete** : voir [Cycles de vie](./life-cycles.md)

## Options de contexte personnalisées

Les options de contexte permettent de faire transiter des données supplémentaires (injection de
dépendances, cache, i18n, ...) à travers une opération. Voir la
[démo](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## `ErrorSanitizer` personnalisé

Le payload par défaut retourné pour les opérations échouées a la signature suivante :

```rs
type DefaultFieldErrorMetadata = ();

struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

type IvoErrorPayload<Metadata: Clone> = HashMap<String, FieldError<Metadata>>;
```

Pour personnaliser ce payload, fournissez une implémentation du trait `IvoErrorSanitizer` - voir
[cet exemple](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/extras/error_sanitizer.rs).

## Référence API

Les documents ci-dessus couvrent les concepts de haut niveau. Pour la référence API exhaustive
(types, fonctions, macros dérivées) générée par rustdoc, consultez :

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — rustdoc hébergé pour les crates publiées.
  (Pas encore disponible car `ivo` n'a pas été publié sur crates.io.)
- **rustdoc local** — exécutez `cargo doc --no-deps --open` depuis le répertoire `rs/` pour consulter
  la même référence générée localement.
