---
title: Cycles de vie
---

# Cycles de vie

`ivo` permet de réagir aux changements d'une entité métier ou de ses champs individuels. Les
concepts ci-dessous sont partagés entre les deux implémentations - voir le
[README racine](https://github.com/kamtoeddy/ivo#lifecycle-events) pour les définitions complètes
et indépendantes du langage. Cette page explique comment les mettre en place en Rust.

## onDelete

Déclenché manuellement en appelant la méthode `delete` du modèle d'un schéma. Abonnez-vous pour
l'entité entière via les options du schéma, ou par champ de sortie. Voir les fonctions de test
`should_properly_trigger_on_delete_handlers` et `should_properly_trigger_all_on_delete_handlers`
[ici](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/opions/mod.rs).

## onFailure

Déclenché manuellement en appelant la fonction `handle failure` retournée par une opération de
création ou de mise à jour échouée. Abonnez-vous sur les champs d'entrée individuels ayant au
moins un validateur.

## onSuccess

Déclenché manuellement en appelant la fonction `handle success` retournée par une opération de
création ou de mise à jour réussie. Abonnez-vous sur n'importe quel champ individuel, ou sur
[un groupe de champs via les options du schéma](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/option_on_success.rs)
(un tableau de champs vide s'abonne aux changements de l'entité entière).

## Options de contexte personnalisées

Voir [Démarrage - options de contexte personnalisées](./index.md#options-de-contexte-personnalisées) pour
savoir comment faire transiter des données supplémentaires (injection de dépendances, cache,
i18n, ...) à travers les opérations de création/mise à jour/suppression et jusque dans ces
gestionnaires.
