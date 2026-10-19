---
title: Nouveautés de la v0.7.0
---

# Nouveautés de la v0.7.0

La v0.7.0 change la façon dont les structs générées reçoivent leurs attributs, ce que reçoivent les
gestionnaires et la façon dont les erreurs sont stockées, et elle rend `create`/`update` beaucoup
plus rapides. La plupart des changements incompatibles sont détectés à la compilation, et les
messages d'erreur de la macro indiquent quoi écrire à la place.

## Changements incompatibles

### Attributs des structs et des champs : `ivo_attrs(...)`

`derive(...)`/`derive_partial(...)` sur les structs générées sont remplacés par `ivo_attrs(...)` et
`ivo_partial_attrs(...)`, qui acceptent **n'importe quel** attribut, pas seulement des derives :

```rust
// v0.6
#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq), derive_partial(Debug))
)]

// v0.7
#[ivo_schema(
    input(UserInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(
        User,
        ivo_attrs(derive(Debug, Clone, PartialEq, serde::Serialize)),
        ivo_partial_attrs(derive(serde::Serialize))
    )
)]
```

Les attributs transmis aux champs ont été renommés de la même façon, et deux nouveaux attributs
s'appliquent au champ dans toutes les structs (ou toutes les structs partielles) dont il fait
partie :

| v0.6                      | v0.7                                                          |
| ------------------------- | ------------------------------------------------------------- |
| `#[input(...)]`           | `#[ivo_input_attrs(...)]`                                     |
| `#[output(...)]`          | `#[ivo_output_attrs(...)]`                                    |
| `#[partial(...)]`         | `#[ivo_partial_attrs(...)]`                                   |
| `#[input_partial(...)]`   | `#[ivo_input_partial_attrs(...)]`                             |
| `#[output_partial(...)]`  | `#[ivo_output_partial_attrs(...)]`                            |
| --                        | `#[ivo_attrs(...)]` (nouveau : toutes les structs non partielles) |

```rust
#[required]
#[ivo_attrs(serde(rename = "userName"))]
#[validate(|v: &String, _, _| Ok(None))]
pub username: String,
```

Voir [Démarrage](./index.md#démarrage-rapide) pour toutes les règles.

### `#[post_validate]` : plus de `validate =`

Le validateur est passé directement, en second argument :

```rust
// v0.6
#[post_validate(["password", "confirm_password"], validate = |ctx, _opts| { /* ... */ })]

// v0.7
#[post_validate(["password", "confirm_password"], |ctx, _opts| {
    let input = ctx.input();
    if input.password != input.confirm_password {
        let mut errors = UserInputErrors::new();
        errors.set_confirm_password("passwords do not match", None);
        return Err(errors);
    }
    Ok(None)
})]
const _: () = ();
```

`pre_validate = ...` ne change pas.

### Les validateurs reçoivent la valeur par référence

Les validateurs et re-validateurs reçoivent maintenant `&T` au lieu de `T`. Renvoyez `Ok(None)`
pour garder la valeur telle quelle, ou `Ok(Some(...))` avec une valeur possédée pour la remplacer :

```rust
// v0.6
#[validate(|v: String, _, _| Ok(Some(v.trim().to_string())))]

// v0.7
#[validate(|v: &String, _, _| Ok(Some(v.trim().to_string())))]
```

### Les gestionnaires reçoivent `&IvoContext`

Les résolveurs, validateurs, post-validateurs et gestionnaires `on_success`/`on_failure` reçoivent
maintenant le contexte par référence. Les closures aux paramètres inférés (`|ctx, _opts| ...`)
n'ont pas besoin de changer ; clonez le contexte si vous devez le déplacer ailleurs :

```rust
#[on_success(|ctx, _opts| {
    let ctx = ctx.clone();
    std::thread::spawn(move || println!("created {:?}", ctx.values().id));
})]
const _: () = ();
```

Les fonctions utilisées comme gestionnaires doivent prendre `&IvoContext<...>` au lieu de
`IvoContext<...>`.

### `ctx.values()` renvoie la sortie partielle

`ctx.values()` renvoie maintenant la struct de sortie **partielle** du schéma : chaque champ est
une `Option`. Pendant `create`, elle se remplit au fil du pipeline (un champ pas encore atteint vaut
`None`) ; pendant une mise à jour et dans les gestionnaires `on_success`, chaque champ vaut `Some` :

```rust
// v0.6
#[resolve(|ctx, _opts| ctx.values().max_attempts * 2)]

// v0.7
#[resolve(|ctx, _opts| ctx.values().max_attempts.unwrap_or_default() * 2)]
```

Voir [ce que contient `ctx.values()`](./execution-pipeline.md#ce-que-contient-ctxvalues) à chaque
étape.

### Les raisons d'erreur sont des `Cow<'static, str>`

La raison d'erreur d'un champ est maintenant un `IvoErrorReason` (`Cow<'static, str>`) : un
littéral est utilisé tel quel, sans allocation, et un message construit à l'exécution est possédé.
Les validateurs en créent un avec `.into()` :

```rust
#[validate(|v: &String, _, _| {
    let name = v.trim();
    if name.len() < 3 {
        return Err(("too short".into(), None));
    }
    if name.len() > 20 {
        return Err((format!("at most 20 characters, got {}", name.len()).into(), None));
    }
    Ok(Some(name.to_string()))
})]
```

Les erreurs sont maintenant indexées par les noms de champs du schéma, en `&'static str` :

```rust
// v0.6 : HashMap<String, FieldError<M>>, avec `reason: String`
// v0.7 : HashMap<&'static str, FieldError<M>>, avec `reason: IvoErrorReason`
for (field, error) in &errors {
    let reason: &str = &error.reason;
    println!("{field}: {reason}");
}

let owned: String = errors.get("username").unwrap().reason.clone().into_owned();
```

`IvoErrorReason` est exporté à la racine du crate. Un nettoyeur d'erreurs personnalisé reçoit ces
erreurs (voir [Options](./options.md)).

## Nouveautés

### Fonctions sans argument comme constantes et valeurs par défaut

`#[constant(...)]`, `#[lax(...)]` et le `#[default(...)]` d'un champ dépendant acceptent le chemin
d'une fonction sans argument. Elle est appelée pour produire la valeur : `|| make_value()` n'est
plus nécessaire. Le chemin d'une `const` ou d'une `static` est toujours utilisé comme valeur :

```rust
fn new_id() -> u64 {
    42
}

fn default_role() -> String {
    "member".to_string()
}

const MAX_ATTEMPTS: u32 = 3;

// dans le module du schéma (avec `use super::{default_role, new_id, MAX_ATTEMPTS};`)
struct Fields {
    #[constant(new_id)] // appelée : 42
    pub id: u64,

    #[lax(default_role)] // appelée : "member"
    pub role: String,

    #[lax(MAX_ATTEMPTS)] // utilisée comme valeur : 3
    pub max_attempts: u32,
}
```

## Corrections

- `validate_email` (fonctionnalité `validators`) n'accepte que les valeurs qui sont entièrement une
  adresse e-mail, et plus celles qui en contiennent simplement une.
- `validate_credit_card` (fonctionnalité `validators`) accepte les numéros de carte valides dont le
  chiffre de contrôle est `0`.

## Performances

`create` et `update` allouent et clonent beaucoup moins : les contextes partagent l'entrée brute et
les valeurs précédentes au lieu de les copier, les gestionnaires et validateurs empruntent au lieu
de prendre possession, les raisons d'erreur n'allouent que lorsqu'elles sont construites à
l'exécution, et seules les parties du contexte qui ont changé sont reconstruites entre les phases.
Par rapport au début du travail sur les performances de la v0.7, sur un Apple M4 :

| Benchmark                     | Avant     | Après     | Écart |
| ----------------------------- | --------- | --------- | ----- |
| minimal create                | 56.91 ns  | 26.97 ns  | -53 % |
| user create                   | 523.96 ns | 215.06 ns | -59 % |
| dependent chain length 10     | 1.46 µs   | 213.25 ns | -85 % |
| create 10 readonly lax fields | 1.61 µs   | 644.49 ns | -60 % |
| no-op update                  | 205.81 ns | 55.66 ns  | -73 % |
| single field update           | 576.87 ns | 199.61 ns | -65 % |

Les résultats complets sont dans
[`rs/PERF_RESULTS.md`](https://github.com/kamtoeddy/ivo/blob/main/rs/PERF_RESULTS.md).
