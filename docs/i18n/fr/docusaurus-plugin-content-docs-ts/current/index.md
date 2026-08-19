---
slug: /
title: Premiers pas
---

import TsPlayground from '@site/src/components/TsPlayground';

# Premiers pas

`ivo` pour TypeScript vous permet de définir un schéma avec un constructeur de champs fluent, puis
d'en dériver un modèle avec les méthodes `create`, `update` et `delete`.

## Installation

```bash
npm i ivo
```

## Définir un schéma

Un schéma est créé avec `new Schema((b) => ...)` où `b` est un `FieldBuilder`. Les champs sont
déclarés avec `b.required(...)`, `b.lax(...)`, `b.constant(...)`, `b.dependent(...)` ou
`b.virtual(...)`, puis passés à `b.field(...)`.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

type UserInput = {
  email: string | null;
  phoneNumber: string | null;
  username: string;
};

type User = {
  id: string;
  createdAt: Date;
  email: string | null;
  phoneNumber: string | null;
  updatedAt: Date | null;
  username: string;
  usernameLastUpdatedAt: Date | null;
};

const isEmailOrPhoneRequired = ({ input }: any) => [
  !input.email && !input.phoneNumber,
  "Fournissez un email ou un numéro de téléphone",
];

const validateEmail = (value: string | null) =>
  value && value.includes("@")
    ? true
    : { valid: false, reason: "Email invalide" };

const validatePhoneNumber = (value: string | null) =>
  value && value.length >= 5
    ? true
    : { valid: false, reason: "Numéro de téléphone invalide" };

const validateUsername = (value: string) =>
  value.length >= 3
    ? true
    : { valid: false, reason: "Le nom d'utilisateur doit faire au moins 3 caractères" };

const userSchema = new Schema<UserInput, User>(
  (b) =>
    b
      .field(b.constant("id", () => "user-123"))
      .field(
        b
          .lax("email", null)
          .required(isEmailOrPhoneRequired)
          .validate(validateEmail),
      )
      .field(
        b
          .lax("phoneNumber", null)
          .required(isEmailOrPhoneRequired)
          .validate(validatePhoneNumber),
      )
      .field(
        b
          .required("username")
          .validate(validateUsername)
          .ignoreUpdate(({ previousValues }) => {
            const last = previousValues.usernameLastUpdatedAt;
            if (!last) return false;

            const thirtyDays = 2_592_000_000;
            return new Date().getTime() - last.getTime() < thirtyDays;
          }),
      )
      .field(
        b
          .dependent("usernameLastUpdatedAt", "username")
          .default(null)
          .resolve(({ isUpdate }) => (isUpdate ? new Date() : null)),
      ),
  { timestamps: true },
);

const UserModel = userSchema.getModel();

const { data, error } = await UserModel.create({
  email: "john.doe@mail.com",
  username: "john_doe",
});
console.log("created:", data);

const user = { ...data!, updatedAt: new Date() };
const { data: updated } = await UserModel.update(user, { username: "johndoe" });
console.log("updated:", updated);
`}
/>

## Méthodes du modèle

Le modèle renvoyé par `schema.getModel()` expose des méthodes asynchrones :

| Méthode  | Description                                                        |
| -------- | ------------------------------------------------------------------ |
| `create` | Crée une nouvelle instance à partir d'une entrée partielle.        |
| `update` | Applique une mise à jour partielle à une instance existante.       |
| `delete` | Déclenche tous les écouteurs `onDelete` sur l'entité fournie.      |

## Créer une entité

Les propriétés inconnues et les propriétés réservées à la sortie (`constant`, `dependent`,
`timestamps`) sont ignorées automatiquement.

```ts
const { data, error } = await UserModel.create({
  email: "john.doe@mail.com",
  id: 5, // ignoré car 'id' est constant
  name: "John Doe", // ignoré car il n'est pas dans le schéma
  username: "john_doe",
  updatedAt: new Date(), // ignoré car c'est un timestamp
  usernameLastUpdatedAt: new Date(), // ignoré car c'est un champ dépendant
});

if (error) return handleError(error);

console.log(data);
// {
//   id: '...',
//   createdAt: Date,
//   email: 'john.doe@mail.com',
//   phoneNumber: null,
//   updatedAt: null,
//   username: 'john_doe',
//   usernameLastUpdatedAt: null
// }
```

## Mettre à jour une entité

```ts
const user = await usersDb.findByID(id);
if (!user) return handleError({ message: "Utilisateur non trouvé" });

const { data, error } = await UserModel.update(user, {
  usernameLastUpdatedAt: new Date(), // dépendant -> ignoré
  id: 75, // constant -> ignoré
  age: 34, // non présent dans le schéma -> ignoré
  username: "johndoe",
});

if (error) return handleError(error);

console.log(data);
// {
//   updatedAt: Date,
//   username: 'johndoe',
//   usernameLastUpdatedAt: Date
// }
```

## Catégories de champs

- [Valeurs autorisées](./definitions/allowed-values.md)
- [Champs constants](./definitions/constants.md)
- [Valeurs par défaut](./definitions/defaults.md)
- [Champs dépendants](./definitions/dependents.md)
- [Extension de schémas](./definitions/extend-schemas.md)
- [Champs lax](./definitions/lax.md)
- [Champs en lecture seule](./definitions/readonly.md)
- [Champs requis](./definitions/required.md)
- [Champs virtuels](./definitions/virtuals.md)

## Options du schéma

Le deuxième argument de `new Schema` accepte des options :

```ts
new Schema((b) => ..., {
  equalityDepth: 1,
  sanitizeError: (payload, ctxOptions) => payload,
  onDelete: [listener],
  onSuccess: [listener],
  postValidate: { fields: ['email', 'phoneNumber'], validator: ... },
  ignore: { fields: ['secret'], handler: () => true },
  ignoreUpdate: { fields: ['email'], handler: () => true },
  required: { fields: ['email', 'phoneNumber'], handler: ... },
  timestamps: true,
});
```

| Option          | Description                                                                   |
| --------------- | ----------------------------------------------------------------------------- |
| `equalityDepth` | Profondeur d'imbrication utilisée pour comparer les valeurs lors des mises à jour (défaut : `1`). |
| `sanitizeError` | Transforme le payload d'erreur avant qu'il ne soit renvoyé.                   |
| `onDelete`      | Écouteur(s) global(aux) invoqué(s) par `model.delete`.                        |
| `onSuccess`     | Écouteur(s) global(aux) invoqué(s) après une création/mise à jour réussie.    |
| `postValidate`  | Configuration de validation transversale (`fields` + `validator`).            |
| `ignore`        | Ignore les champs d'entrée lorsque le gestionnaire renvoie `true`.            |
| `ignoreUpdate`  | Ignore les valeurs de mise à jour des champs listés lorsque le gestionnaire renvoie `true`. |
| `required`      | Contrainte requise transversale (`fields` + `handler`).                       |
| `timestamps`    | Active `createdAt`/`updatedAt` (booléen ou `{ createdAt?, updatedAt? }`).     |

Voir [Cycles de vie](./life-cycles.md) et [Validateurs](./validators.md) pour en savoir plus.

## Étendre un schéma

Utilisez `.extend()` pour créer un nouveau schéma qui hérite des champs et options du parent :

```ts
const AdminSchema = userSchema.extend<AdminInput, AdminOutput>(
  (b) => b.field(b.required("role").validate(validateRole)),
  { useParentOptions: true },
);
```

Définissez `useParentOptions: false` pour abandonner les options du parent et ne partir que des
options fournies. Les champs peuvent être supprimés avec l'option `remove`.
