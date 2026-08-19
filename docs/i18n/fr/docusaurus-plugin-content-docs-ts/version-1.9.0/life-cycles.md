---
title: "Cycles de vie"
---

# Cycles de vie

## Le contexte de l'opération

Il s'agit d'un objet composé d'un mélange de valeurs d'entrée et de sortie de l'instance pendant une opération de cycle de vie (création ou mise à jour), ainsi que de toutes les propriétés virtuelles (si elles sont présentes pendant l'opération) définies dans votre schéma.

```ts
import { type Context } from "ivo";

type Ctx = Context<Input, Output>;
```

### Options du contexte

C'est un moyen de fournir des informations supplémentaires (liées ou non à votre schéma) aux opérations de création, de mise à jour et de suppression. De bons cas d'utilisation seraient l'**injection de dépendances (DI)** et l'**internationalisation (i18n)**.

Comment utiliser :

```ts
type UserInput = {
  email: string;
  name: string;
};

type User = {
  email: string;
  id: string;
  name: string;
};

interface UserRepo {
  findByEmail: (email: User["email"]) => Promise<User | null>;
  //  ... other methods
}

type CtxOptions = {
  lang: "en" | "de" | "fr"; // lang for i18n
  userRepo: UserRepo; // userRepo for DI
};

// 1) define your schema
const Model = new Schema<UserInput, User, CtxOptions>({
  id: { constant: true, value: generateUserId },
  email: {
    required: true,
    async validator(value, { options: { userRepo } }) {
      if (!isEmail(value))
        return { valid: false, reason: "Invalid email provided" };

      const isEmailTaken = await userRepo.findByEmail(value);

      return isEmailTaken
        ? { valid: false, reason: "email already taken" }
        : true;
    },
  },
  name: { required: true, validator: validateName },
}).getModel();

// 2) pass it to related operations
import { userRepo } from "data-access/users";

// creating an entity   👇
Model.create(input, { lang: "en", userRepo });

// updating an entity             👇
Model.update(entity, changes, { lang: "en", userRepo });

// deleting an entity    👇
Model.delete(entity, { lang: "en", userRepo });

// 3) access the context options as below

// in a validator
function validateName(value, summary: IvoSummary<UserInput, User, CtxOptions>) {
  const { options, updateOptions } = summary;
  const { lang } = options;

  // ... further processing

  // update options
  updateOptions({ lang: "de" });

  return true;
}
```

## Le résumé de l'opération

```ts
import type { Context, IvoSummary, ReadonlyIvoSummary } from "ivo";

type Input = {};
type Output = {};

type IContext = Context<Input, Output>;
type Summary = IvoSummary<Input, Output, CtxOptions>;

// 👇 S represents is what `Summary` looks like
type S =
  | Readonly<{
      changes: null;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>;
      updateOptions: (updates: Partial<CtxOptions>) => void;
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>;
      updateOptions: (updates: Partial<CtxOptions>) => void;
    }>;

type ReadonlySummary = ReadonlyIvoSummary<Input, Output, CtxOptions>;

// 👇 Rs represents is what `ReadonlySummary` looks like
type Rs =
  | Readonly<{
      changes: null;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>; // 👇 notice that the `updateOptions` method is missing
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>; // 👇 notice that the `updateOptions` method is missing
    }>;

const Model = new Schema<Input, Output>(definitions).getModel();

type FailureHandler = (
  ctx: IContext,
  options: CtxOptions,
) => void | Promise<void>;

type HandlerWithSummary = (summary: ReadonlySummary) => void | Promise<void>;
```

## Gestionnaires de cycle de vie

Ce sont des fonctions qui sont invoquées pendant une opération de cycle de vie (`création`, `échec` ou `mise à jour`).

### onDelete

Une fonction `void` ou un tableau de fonctions `void` (async/sync) que vous souhaitez exécuter chaque fois qu'une instance de votre modèle est supprimée. C'est-à-dire chaque fois que la méthode **`model.delete`** est invoquée. Ces écouteurs ont accès à un contexte sans propriétés virtuelles, même s'ils sont passés à la méthode `delete` du modèle. Valeur par défaut **[ ]**. Elles doivent respecter la signature ci-dessous.

```ts
// signature
function onDelete(data: Output, options: CtxOptions) {
  const { id, name } = data;
  const { lang } = options; // { lang: "en" }
}

// how to trigger after deleting an entity
Model.delete(entity, { lang: "en" });
```

### onFailure

Une fonction ou un tableau de fonctions (async/sync) que vous souhaitez exécuter chaque fois que les opérations **`create`** et **`update`** échouent. Valeur par défaut **[ ]**.

> N.B. : elles ne sont autorisées que sur les propriétés qui prennent en charge et disposent de validateurs.

Ces gestionnaires doivent être déclenchés manuellement en invoquant la méthode `handleFailure` de l'objet de résultats de l'opération retourné par les méthodes `create` et `update` de vos modèles.

> Si l'opération réussit, `error` et `handleFailure` seront `null`.

```js
// signature
function onFailure(ctx: IContext, options: CtxOptions) {
  const { id, name } = ctx;
  const { lang } = options; // { lang: "en" }
}


const { error, handleFailure } = await UserModel.create(userData);

// how to trigger after a validation error
if (error) await handleFailure();
```

### onSuccess

Une fonction, un [objet de configuration](#objets-de-configuration) ou un tableau d'objets de configuration ou de fonctions (async/sync) que vous souhaitez exécuter chaque fois que les opérations **`create`** et **`update`** réussissent. Les gestionnaires de cet événement doivent attendre le résumé de l'opération comme seul paramètre. Valeur par défaut **[ ]**. Les gestionnaires doivent respecter le `type HandlerWithSummary` comme indiqué ci-dessus.

Ces gestionnaires doivent être déclenchés manuellement en invoquant la méthode `handleSuccess` de l'objet de résultats de l'opération retourné par les méthodes `create` et `update` de vos modèles.

> N.B. : si l'opération échoue, `data` et `handleSuccess` seront `null`.

```js
// signature
function onSuccess(summary: Summary) {
  const { ctx, options } = summary;
  const { id, name } = ctx;
  const { lang } = options; // { lang: "en" }
}


const { data, error, handleSuccess } = await UserModel.create(userData);

// how to trigger after successful validation
if (data) await handleSuccess();
```

#### Objets de configuration

Ils ont été introduits dans la version 1.4.1 pour permettre plus de simplicité et de flexibilité lors de la gestion des gestionnaires de succès liés à plusieurs propriétés. Un objet de configuration de succès doit avoir la forme suivante :

```ts
type ConfigObject = {
  properties: ArrayOfMinSizeTwo<keyof (Input & Output)>;
  handler: HandlerWithSummary | HandlerWithSummary[];
};
```

Exemple :

```ts
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: handler, // the handler will be executed during all success operations
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [handler1, handler2], // the handlers will be executed during all success operations
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: {
    properties: ["email", "name"],
    handler, // always executed at creation during updates with either email or name
  },
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: {
    properties: ["email", "name"],
    handler: [handler1, handler2], // always executed at creation during updates with either email or name
  },
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [
    handler1, // executed during all success operations
    { properties: ["id", "email"], handler: handler2 },
    { properties: ["firstName", "lastName"], handler: [handler3, handler4] },
  ],
});

// ✅ as from v1.5.1 you can provide subsets of other configs
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [
    { properties: ["id", "email", "firstName"], handler: handler2 },
    { properties: ["email", "firstName"], handler: [handler3, handler4] },
  ],
});

// ❌ this is not allowed
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [
    { properties: ["id", "email"], handler: [handler1, handler2] },
    { properties: ["email", "id"], handler: handler3 },
  ],
});
```
