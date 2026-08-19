---
title: "Définir un schéma"
---

# Définir un schéma

Clean schema considère qu'une propriété est correctement définie si elle est `dependent`, `readonly`, `required`, une `virtual` ou si elle possède une valeur `default` autre que _undefined_

> N.B : Clean schema lèvera une erreur si une propriété n'est pas correctement définie.
> Le constructeur Schema accepte 2 arguments :

1. definitions (obligatoire)
1. [options (optionnel)](#options)

Le constructeur de schéma prend également deux types génériques que vous pouvez utiliser pour améliorer l'inférence de types de vos données `Input` & `Output`.

```ts
const userSchema = new Schema<Input, Output>(definitions, options);
```

```ts
import { Schema } from "ivo";

type UserInput = {
  dob: Date | null;
  firstName: string;
  lastName: string;
};

type User = {
  dob: Date | null;
  firstName: string;
  lastName: string;
  fullName: string;
};

const userSchema = new Schema<UserInput, User>({
  dob: { required: true, validator: validateDob },
  firstName: { required: true, validator: validateName },
  lastName: { required: true, validator: validateName },
  fullName: {
    default: "",
    dependsOn: ["firstName", "lastName"],
    resolver({ ctx: { firstName, lastName } }) {
      return `${firstName} ${lastName}`;
    },
  },
});

const UserModel = userSchema.getModel();
```

## Propriétés d'un modèle

Ces méthodes sont asynchrones car les validateurs personnalisés peuvent également être asynchrones.

| Propriété | Type     | Description                                                    |
| --------- | -------- | -------------------------------------------------------------- |
| create    | function | Méthode asynchrone pour créer une instance                     |
| delete    | function | Méthode asynchrone pour déclencher tous les écouteurs onDelete |
| update    | function | Méthode asynchrone pour mettre à jour une instance             |

## Règles acceptées

| Propriété    | Type                         | Description                                                                                                                                                                                             |
| ------------ | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| allow        | any[ ] \| object             | utilisé pour spécifier les valeurs qui doivent être acceptées pour une propriété. [En savoir plus](./definitions/allowed-values.md#valeurs-autorisées)                                                  |
| constant     | boolean                      | à utiliser avec la règle **`value`** pour spécifier une propriété avec une valeur constante. [plus](./definitions/constants.md#propriétés-constantes)                                                   |
| default      | any \| function              | la valeur par défaut d'une propriété. [plus](./definitions/defaults.md#valeurs-par-défaut)                                                                                                              |
| dependsOn    | string \| string[ ]          | une propriété ou une liste de propriétés dont dépend ladite propriété. [plus](./definitions/dependents.md)                                                                                              |
| ignore       | function                     | une fonction utilisée pour déterminer si la valeur d'entrée d'une propriété doit être ignorée. Cela agit comme `shouldInit` + `shouldUpdate`                                                            |
| onDelete     | function \| function[ ]      | exécuté lorsque la méthode delete d'un modèle est invoquée [plus](./life-cycles.md#ondelete)                                                                                                            |
| onFailure    | function \| function[ ]      | exécuté après une opération infructueuse [plus](./life-cycles.md#onfailure)                                                                                                                             |
| onSuccess    | function \| function[ ]      | exécuté après une opération réussie [plus](./life-cycles.md#onsuccess)                                                                                                                                  |
| readonly     | boolean \| 'lax'             | une propriété dont la valeur ne doit pas changer [plus](./definitions/readonly.md)                                                                                                                      |
| required     | boolean \| function          | une propriété qui doit être définie pendant une opération [plus](./definitions/required.md)                                                                                                             |
| sanitizer    | function                     | Cela peut être utilisé pour transformer une propriété virtuelle avant que ses propriétés dépendantes ne soient résolues. [plus](./definitions/virtuals.md#sanitiser)                                    |
| shouldInit   | false \| function(): boolean | Un booléen ou un setter qui indique à ivo si une propriété doit être initialisée ou non.                                                                                                                |
| shouldUpdate | false \| function(): boolean | Un booléen ou un setter qui indique à ivo si une propriété doit être initialisée ou non.                                                                                                                |
| validator    | function                     | Une fonction (async / sync) utilisée pour valider la valeur d'une propriété. [plus](./validators.md)                                                                                                    |
| value        | any \| function              | valeur ou setter d'une propriété constante. [plus](./definitions/constants.md#propriétés-constantes)                                                                                                    |
| virtual      | boolean                      | une propriété d'assistance qui peut être utilisée pour fournir un contexte supplémentaire mais n'apparaît pas sur les instances de votre modèle [plus](./definitions/virtuals.md#propriétés-virtuelles) |

## Options

```ts
import type { Context, IvoSummary, ReadonlyIvoSummary } from 'ivo';

type Input = {};
type Output = {};

type IContext = Context<Input, Output>;
type Summary = IvoSummary<Input, Output, CtxOptions>;

type DeleteListener = (data: Outpur, options: CtxOptions) => void | Promise<void>;

type SuccessListener = (summary: Summary) => void | Promise<void>

type Timestamp = {
  createdAt?: string
  updatedAt?: string
}

interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

type SchemaOptions = {
  equalityDepth?: number
  errorTool?: ErrorToolClass // more on this below 👇
  onDelete?: DeleteListener | DeleteListener[]
  onSuccess?: SuccessListener | SuccessListener[]
  postValidate?: PostValidationConfig | PostValidationConfig[]
  setMissingDefaultsOnUpdate?: boolean
  shouldUpdate?: boolean | (summary: ISummary) => boolean
  timestamps?: boolean | Timestamp
  useParentOptions?: boolean // 👈 only for extended schemas
}

const options: SchemaOptions = {}

const schema = new Schema<Input, Output, CtxOptions, ErrorToolClass>(definitions, options)
```

Plus de détails sur les utilitaires `Context` & `Summary` peuvent être trouvés [ici](./life-cycles.md#le-contexte-de-lopération)

### equalityDepth (défaut : 1)

C'est le nombre utilisé pour déterminer si la valeur d'une propriété a changé pendant les mises à jour.

Pour déterminer si une propriété a changé, sa valeur est comparée à sa valeur par défaut et à sa valeur précédente. Comme l'égalité entre objets n'est pas toujours évidente, la valeur `equalityDepth` fournie est utilisée pour déterminer si les propriétés de votre schéma qui acceptent des objets (qui peuvent avoir des objets imbriqués) comme valeurs ont changé pendant les mises à jour.

Les valeurs possibles pour ce nombre vont de `0` à `+Infinity`. La valeur par défaut est `1`, ce qui signifie **un niveau d'imbrication**.

Voici un extrait pour démontrer comment le simple changement d'arrangement des valeurs de propriétés imbriquées (sans même changer leurs valeurs réelles) peut affecter les résultats d'une mise à jour :

```ts
const user = {
  name: "John Doe",
  bio: {
    facebook: { displayName: "john", handle: "john3434" },
    twitter: { displayName: "John Doe", handle: "john_on_twitter" },
  },
};

// depth == 0

Model.update(user, { bio: user.bio }).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio
Model.update(user, {
  bio: {
    twitter: { displayName: "John Doe", handle: "john_on_twitter" },
    facebook: { displayName: "john", handle: "john3434" },
  },
}).then(({ data, error }) => {
  console.log(data);
  // {
  //   bio: {
  //     facebook: { displayName: 'john', handle: 'john3434' },
  //     twitter: { displayName: 'John Doe', handle: 'john_on_twitter' }
  //     }
  // }

  console.log(error); // null
});

// depth == 1

Model.update(user, { bio: user.bio }).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio
Model.update(user, {
  bio: {
    twitter: { displayName: "John Doe", handle: "john_on_twitter" },
    facebook: { displayName: "john", handle: "john3434" },
  },
}).then(({ data, error }) => {
  console.log(data); // null
  console.log(error.message); // Nothing to update
});

// 👇 changing the positions of facebook & twitter in bio and the positions of displayName & handle
Model.update(user, {
  bio: {
    twitter: { handle: "john_on_twitter", displayName: "John Doe" },
    facebook: { displayName: "john", handle: "john3434" },
  },
}).then(({ data, error }) => {
  console.log(data);
  // {
  //   bio: {
  //     facebook: { displayName: 'john', handle: 'john3434' },
  //     twitter: { handle: 'john_on_twitter', displayName: 'John Doe' }
  //     }
  // }

  console.log(error); // null
});
```

### errorTool

C'est une classe qui sera utilisée pour gérer vos erreurs de validation, vous donnant ainsi le pouvoir d'avoir des erreurs de validation personnalisées. Voir l'exemple [ici](https://github.com/kamtoeddy/ivo/blob/main/ts/tests/extras/error-sanitizer.ts)

```ts
import type { ValidationErrorMessage, IErrorTool } from "ivo";

// the class should have this signature 👇
interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

// the instances of your ErrorTool class should have this signature 👇
interface IErrorTool<ExtraData extends ObjectType = {}> {
  /** return what your validation error should look like from this method */
  get data(): IValidationError<ExtraData>;

  /** array of fields that have failed validation */
  get fields(): string[];

  /** determines if validation has failed */
  get isLoaded(): boolean;

  /** used to append a field to your final validation error */
  set(field: FieldKey, error: FieldError, value?: any): this;

  /** method to set the value of the validation error message */
  setMessage(message: ValidationErrorMessage): this;
}

type IValidationError<ExtraData extends ObjectType = {}> = ({
  message: ValidationErrorMessage;
} & ExtraData) & {};
```

### onDelete

Cela peut être une fonction ou un tableau de fonctions avec la signature `DeleteListener` ci-dessus. Ces fonctions seront déclenchées en même temps que les écouteurs onDelete des propriétés individuelles lorsque la méthode `Model.delete` est invoquée. En savoir plus [ici](./life-cycles.md#ondelete)

### onSuccess

Cela peut être une fonction ou un tableau de fonctions avec la signature `SuccessListener` ci-dessus. Ces fonctions seront déclenchées en même temps que les écouteurs onSuccess des propriétés individuelles lorsque la méthode handleSuccess est invoquée lors de la création et pendant les mises à jour de toute propriété. En savoir plus [ici](./life-cycles.md#onsuccess)

### postValidate

Pour valider l'intégrité de plusieurs champs après la validation initiale. Plus d'informations [ici](./validators.md#validation-postérieure)

### setMissingDefaultsOnUpdate

Un booléen. S'il est défini sur `true`, il vérifiera toutes les propriétés pouvant avoir une valeur par défaut des données existantes passées à la méthode de mise à jour du modèle `Model.update(existingData, updates)`, et pour toutes les propriétés ayant la valeur `undefined`, il générera leurs valeurs par défaut, les ajoutera au contexte de l'opération avant de valider les mises à jour fournies.

Si l'opération de mise à jour réussit, les valeurs par défaut nouvellement générées seront également ajoutées aux valeurs mises à jour retournées si elles ne sont pas déjà présentes dans les valeurs mises à jour. Valeur par défaut **false**

### shouldUpdate (défaut : true)

Un booléen ou une fonction qui attend le résumé de l'opération et retourne une valeur booléenne. Cette valeur est lue/calculée avant que les valeurs fournies pendant les mises à jour aient été validées.

Si sa valeur ou sa valeur calculée est true, les validations des mises à jour se poursuivront, sinon l'opération échouera avec le message d'erreur `Nothing to update`

```ts
new Schema(
  { id: { constant: true, value: generateId } },
  { shouldUpdate: () => (condition ? true : false) },
);
```

### timestamps (défaut : false)

Si timestamps est défini sur true, vous aurez automatiquement les propriétés `createdAt` et `updatedAt` attachées aux instances de votre modèle lors de la création et pendant la mise à jour. Mais vous pouvez remplacer les options et utiliser vos propres propriétés comme dans l'exemple ci-dessous. Valeur par défaut **false**

Remplacer une seule

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at" },
});
```

Ou les deux

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: "updated_at" },
});
```

Pour utiliser un seul timestamp, passez false pour la clé du timestamp à éliminer

```js
let transactionSchema = new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: false },
});

// or
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: false },
});
```

À partir de la v1.6.1, `updated_at` est `null` à la création

```js
// make updatedAt non-nullable
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: { key: "updated_at", nullable: false } },
});

// or non-nullable whilte keeping the default key
let transactionSchema = new Schema(definitions, {
  timestamps: { updatedAt: { nullable: false } },
});
```

### useParentOptions (défaut : true)

Lors de l'extension de schémas, les schémas étendus héritent automatiquement de toutes les options (sauf les méthodes de cycle de vie) du schéma de base. Définir `useParentOptions: false` dans les options du schéma étendu empêchera ce comportement. La valeur par défaut est `true`

## Essayez-le dans le navigateur

<TsPlayground
ivoVersion="1.9.0"
code={`
import { Schema, type IvoSummary } from 'ivo';

type UserInput = {
email: string | null;
username: string;
};

type User = {
id: string;
createdAt: Date;
email: string | null;
username: string;
};

const userSchema = new Schema<UserInput, User>(
{
id: { constant: true, value: () => Math.random().toString(36).slice(2) },
email: {
default: null,
validator: (value: string) =>
typeof value === 'string' && value.includes('@')
? true
: { valid: false, reason: 'Invalid email' },
},
username: {
required: true,
validator: (value: string) =>
value.length >= 3
? true
: { valid: false, reason: 'Username too short' },
},
},
{ timestamps: true },
);

const UserModel = userSchema.getModel();

async function main() {
const { data, error } = await UserModel.create({
email: 'john.doe@mail.com',
username: 'john_doe',
});

console.log('data:', data);
console.log('error:', error);
}

main();
`}
/>
