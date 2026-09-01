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

1. **Ignore** -- évalue `#[ignore]`/`#[ignore_init]` (au niveau du champ et groupé, ensemble), puis
   applique les remplacements `#[ignore_init]`.
2. **Requis** -- évalue les `#[required(...)]` conditionnels et vérifie que les champs `#[required]`
   nus ont une valeur. *Échoue rapidement.*
3. **Validate** -- `#[validate]` s'exécute pour les champs requis/lax et virtuels ensemble, en une
   seule phase. Les valeurs par défaut des champs `#[lax]` non renseignés sont appliquées à cette
   même étape. *Échoue rapidement.*
4. **Re-validate** -- `#[re_validate]`, même regroupement que validate, uniquement pour les champs
   ayant validé avec succès. *Échoue rapidement.*
5. **Post-validate (pré)** -- le gestionnaire `pre_validate` de chaque groupe
   `#[post_validate(...)]`, contre un instantané pris avant cette phase. *Échoue rapidement.*
6. **Post-validate (principal)** -- le gestionnaire `validate` principal de chaque groupe ; ignoré
   entièrement si l'étape 5 a produit une erreur. *Échoue rapidement.*
7. **Sanitize** -- `#[sanitize]` sur les champs virtuels fournis et non ignorés, une fois l'étape 6
   réussie.
8. **Résolution des dépendants** -- `#[resolve]`, un tour par niveau du graphe de dépendances, en
   boucle jusqu'à ce que plus rien ne change. `ctx.values()` dans un résolveur reflète tout ce qui
   a été résolu lors des tours précédents.
9. **Attachement des constantes** -- `#[constant]`, après la résolution des dépendants, afin qu'un
   résolveur de constante puisse lire les valeurs dépendantes résolues via `ctx.values()`.
10. **Attachement des horodatages** -- `#[created_at]`/`#[updated_at]`, après les constantes. Le
    résolveur partagé n'est appelé qu'une seule fois au maximum.
11. Préparation des déclencheurs `on_success`/`on_failure` pour le tuple retourné (voir
    [Cycles de vie - Déclencher les gestionnaires](./life-cycles.md#déclencher-les-gestionnaires)).

## `update`

1. **Ignore** -- évalue `#[ignore]`/`#[ignore_update]`, puis applique les remplacements
   `#[ignore_update]` nus.
2. **Point de contrôle "rien à mettre à jour" 1** -- si aucun champ requis/lax/virtuel réellement
   soumis ne survit au filtrage ignore/`#[readonly]`, échoue immédiatement avec "rien à mettre à
   jour", avant même que la vérification des champs requis ne s'exécute.
3. **Requis** -- `#[required(...)]` conditionnel uniquement (le `#[required]` nu est réservé à la
   création). *Échoue rapidement.*
4. **Validate** -- identique à l'étape 3 de `create`. *Échoue rapidement.*
5. **Re-validate** -- identique à l'étape 4 de `create`. *Échoue rapidement.*
6. **Post-validate** -- `pre_validate` puis `validate` principal, avec le même contrôle que les
   étapes 5-6 de `create`. *Échoue rapidement.*
7. **Évaluation de la validité de la mise à jour** -- recalcule l'ensemble des changements et
   retire tout champ dont la valeur s'avère inchangée, une seule fois, juste après post-validate
   (`raw_input()` continue de montrer ce qui a été réellement soumis ; `input()` reflète ce
   filtrage). *Échoue rapidement.*
8. **Point de contrôle "rien à mettre à jour" 2** -- s'il ne reste plus rien à changer après
   l'étape 7 *et* qu'aucun champ virtuel n'est encore pertinent (son ou ses dépendants ne se sont
   pas encore résolus), échoue immédiatement, avant que la résolution des dépendants ne s'exécute.
9. **Sanitize** -- même condition que l'étape 7 de `create`.
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
