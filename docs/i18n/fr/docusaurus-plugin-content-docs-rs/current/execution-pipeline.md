---
title: Pipeline d'exécution
---

# Pipeline d'exécution

`create` et `update` s'exécutent selon une séquence de phases fixe. Connaître cet ordre importe
pour deux choses en particulier : ce que `ctx.values()` peut voir depuis un résolveur (tout ce qui
a été résolu lors d'une phase antérieure, rien d'une phase ultérieure), et quand la validation
échoue "rapidement" -- la plupart des phases pouvant produire une erreur retournent immédiatement
si une erreur est survenue, sans exécuter de phase ultérieure sur des données déjà invalides.

La macro ne génère que les phases dont un schéma donné a réellement besoin -- un schéma sans champ
`#[constant]` n'a aucune étape d'attachement de constante, un schéma sans champ `#[depends_on]` n'a
aucune boucle de résolution de dépendants, et ainsi de suite. Rien de ce qui est décrit ci-dessous
n'est du code mort ou une branche d'exécution pour les schémas qui n'utilisent pas une
fonctionnalité donnée.

## `create`

`create` construit une sortie partielle (chaque champ est une `Option`) au fil de l'exécution,
exposée par `ctx.values()`. Les valeurs ne sont calculées qu'au moment où elles sont nécessaires :
une création dont la validation échoue ne calcule jamais de valeur par défaut de dépendant, de
constante ni d'horodatage.

1. **Ignore** -- évalue `#[ignore]`/`#[ignore_init]` (au niveau du champ et groupé, ensemble), puis
   applique les remplacements `#[ignore_init]`.
2. **Valeurs initiales** -- chaque valeur requise/lax fournie et non ignorée est copiée telle
   quelle, et chaque champ `#[lax]` manquant ou ignoré reçoit sa valeur par défaut.
3. **Requis** -- évalue les `#[required(...)]` conditionnels et vérifie que les champs `#[required]`
   nus ont une valeur. *Échoue rapidement.*
4. **Validate** -- `#[validate]` s'exécute pour les champs requis/lax et virtuels ensemble, en une
   seule phase. Chaque valeur acceptée remplace la valeur initiale. *Échoue rapidement.*
5. **Re-validate** -- `#[re_validate]`, même regroupement que validate, uniquement pour les champs
   ayant validé avec succès. *Échoue rapidement.*
6. **Post-validate (pré)** -- le gestionnaire `pre_validate` de chaque groupe
   `#[post_validate(...)]`, contre un instantané pris avant cette phase. *Échoue rapidement.*
7. **Post-validate (principal)** -- le validateur principal de chaque groupe ; ignoré entièrement si
   l'étape 6 a produit une erreur. *Échoue rapidement.*
8. **Sanitize** -- `#[sanitize]` sur les champs virtuels fournis et non ignorés, une fois l'étape 7
   réussie.
9. **Valeurs par défaut des dépendants** -- chaque champ dépendant reçoit son `#[default(...)]`.
10. **Résolution des dépendants** -- `#[resolve]`, un tour par niveau du graphe de dépendances, en
    boucle jusqu'à ce que plus rien ne change. `ctx.values()` dans un résolveur reflète tout ce qui
    a été résolu lors des tours précédents.
11. **Attachement des constantes** -- `#[constant]`, après la résolution des dépendants, afin qu'un
    résolveur de constante puisse lire les valeurs dépendantes résolues via `ctx.values()`.
12. **Attachement des horodatages** -- `#[created_at]`/`#[updated_at]`, après les constantes. Le
    résolveur partagé n'est appelé qu'une seule fois au maximum.
13. **Conversion** -- la sortie partielle, désormais complète, est convertie en struct de sortie.
14. Préparation des déclencheurs `on_success`/`on_failure` pour le tuple retourné (voir
    [Cycles de vie - Déclencher les gestionnaires](./life-cycles.md#déclencher-les-gestionnaires)).

### Ce que contient `ctx.values()`

`ctx.values()` retourne la struct de sortie partielle du schéma : chaque champ est donc une
`Option`. À la création, un champ vaut `None` jusqu'à la phase qui le définit :

| Gestionnaire | Champs valant `Some` |
| --- | --- |
| ignore | aucun |
| required, validate | les valeurs requises/lax fournies (brutes) et les valeurs par défaut des champs lax manquants |
| re-validate, post-validate, sanitize | les mêmes, avec les valeurs validées à la place des valeurs brutes |
| valeur par défaut d'un dépendant | chaque champ requis/lax |
| résolveur d'un dépendant | chaque champ requis/lax et dépendant (valeurs par défaut ou résolues) |
| constante | tout sauf les horodatages |
| `on_success` | chaque champ |
| `on_failure` | ce qui était défini au moment de l'échec de la validation |

Pendant un `update` (et dans ses gestionnaires `on_success`/`on_failure`), chaque champ vaut
`Some` : l'enregistrement stocké, avec les changements de cette mise à jour appliqués jusque-là.

Chaque gestionnaire reçoit `ctx` par référence (`&IvoContext`) ; clonez-le pour le conserver
au-delà du gestionnaire.

## `update`

1. **Ignore** -- évalue `#[ignore]`/`#[ignore_update]`, puis applique les remplacements
   `#[ignore_update]` nus.
2. **Point de contrôle "rien à mettre à jour" 1** -- si aucun champ requis/lax/virtuel réellement
   soumis ne survit au filtrage ignore/`#[readonly]`, échoue immédiatement avec "rien à mettre à
   jour", avant même que la vérification des champs requis ne s'exécute.
3. **Requis** -- `#[required(...)]` conditionnel uniquement (le `#[required]` nu est réservé à la
   création). *Échoue rapidement.*
4. **Validate** -- identique à l'étape 4 de `create`. *Échoue rapidement.*
5. **Re-validate** -- identique à l'étape 5 de `create`. *Échoue rapidement.*
6. **Post-validate** -- `pre_validate` puis `validate` principal, avec le même contrôle que les
   étapes 6-7 de `create`. *Échoue rapidement.*
7. **Évaluation de la validité de la mise à jour** -- recalcule l'ensemble des changements et
   retire tout champ dont la valeur s'avère inchangée, une seule fois, juste après post-validate
   (`raw_input()` continue de montrer ce qui a été réellement soumis ; `input()` reflète ce
   filtrage). *Échoue rapidement.*
8. **Point de contrôle "rien à mettre à jour" 2** -- s'il ne reste plus rien à changer après
   l'étape 7 *et* qu'aucun champ virtuel n'est encore pertinent (son ou ses dépendants ne se sont
   pas encore résolus), échoue immédiatement, avant que la résolution des dépendants ne s'exécute.
9. **Sanitize** -- même condition que l'étape 8 de `create`.
10. **Résolution des dépendants** -- un passage par niveau du graphe de dépendances.
11. **Point de contrôle "rien à mettre à jour" 3** -- si l'ensemble des changements est encore vide
    après la résolution des dépendants, échoue avec "rien à mettre à jour".
12. **Attachement des horodatages** -- `#[updated_at]`/`#[optional_updated_at]`.
13. Préparation des déclencheurs `on_success`/`on_failure` pour le tuple retourné (voir
    [Cycles de vie - Déclencher les gestionnaires](./life-cycles.md#déclencher-les-gestionnaires)).

## Pourquoi trois points de contrôle "rien à mettre à jour" ?

Chacun capture une façon différente pour une mise à jour de se révéler être un no-op : ne soumettre
que des champs filtrés avant même le début de la validation (point de contrôle 1), soumettre un
champ avec sa valeur actuelle inchangée (point de contrôle 2), ou soumettre un champ virtuel dont
le dépendant se résout à la valeur qu'il avait déjà (point de contrôle 3, car la pertinence d'un
champ virtuel ne peut être connue qu'une fois son dépendant réellement résolu). Les trois se
manifestent de la même façon : le payload `Err` d'`update` vaut `None` -- ce n'est pas un échec de
validation, il n'y a simplement rien à faire.

```rust
let (err, _ctx_options) = DataInputModel.update(existing, updates, ()).unwrap_err();
assert!(err.is_none()); // "rien à mettre à jour", pas une erreur de validation
```
