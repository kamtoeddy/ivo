---
title: "Propriétés virtuelles"
---

## Propriétés virtuelles

Ces propriétés servent à manipuler des propriétés dépendantes au niveau de votre modèle, mais n'apparaissent pas sur les instances et ne sont donc pas envoyées dans votre base de données.

- Elles (les virtuelles) doivent avoir :

  - `virtual: true`
  - Un validateur et
  - Au moins une propriété qui dépend d'elles

- Elles peuvent avoir (**`shouldInit: false`**) ou `shouldInit` sous forme de fonction
- Elles peuvent avoir (**`shouldUpdate: false`**) ou `shouldUpdate` sous forme de fonction
- Elles peuvent avoir `required` sous forme de fonction
- Elles peuvent avoir des [alias](#alias)
- Elles peuvent avoir des [sanitizeurs](#sanitiser)
- Elles **NE PEUVENT PAS** être dépendantes, avoir de valeur par défaut, être strictement requises ni en lecture seule

Exemple :

```ts
import { Schema } from "ivo";

type UserInput = {
  blockUser: boolean;
};

type User = {
  isBlocked: boolean;
};

// definition
const User = new Schema<UserInput, User>({
  blockUser: { virtual: true, validator: validateBoolean },
  isBlocked: {
    default: false,
    dependsOn: "blockUser",
    resolver: ({ ctx }) => ctx.blockUser,
  },
}).getModel();

function validateBoolean(value) {
  if (![false, true].includes(value))
    return { valid: false, reason: `${value} is not a boolean` };
  return { valid: true };
}

// creating
const user = await User.create({ blockUser: true, name: "Peter" });

console.log(user); // { isBlocked: true }
```

Le résultat de l'opération ci-dessus est un objet avec une seule propriété `isBlocked`. `name` est absent car il n'appartient pas à notre schéma, mais `blockUser` est absent car il est virtuel et, comme il a été fourni, la valeur de `isBlocked` est `true` au lieu de la valeur par défaut (`false`).

Le même concept s'applique à l'opération `update`.

## Alias

Un alias est simplement un nom **externe** supplémentaire pour une propriété virtuelle.

### Comment définir un alias

- Seules les propriétés virtuelles peuvent avoir des alias
- Un alias doit être de type `string`
- Il ne peut pas être le nom d'une autre propriété ou virtuelle de votre modèle (sauf si l'alias est le nom d'une propriété dépendante de cette virtuelle)
- Pour de meilleurs résultats avec TypeScript, les définitions de types fournies doivent correspondre pour votre alias et sa propriété virtuelle (voir l'exemple 1 ci-dessous)

### Exemples

Exemple 1 : Alias portant le nom d'une propriété dépendante associée

```ts
type Input = {
  _virtualQuantity?: number;
};

type Output = {
  quantity: number;
};

type Aliases = {
  quantity: number;
};

const StoreItem = new Schema<Input, Output, Aliases>({
  quantity: {
    default: 0,
    dependsOn: "_virtualQuantity",
    resolver: ({ ctx }) => ctx._virtualQuantity,
  },
  _virtualQuantity: {
    alias: "quantity",
    vitual: true,
    validator: validateVirtualQuantity,
  },
}).getModel();

// this
const { data: item1 } = await StoreItem.create({ _virtualQuantity: 100 });

// is the same as this
const { data: item2 } = await StoreItem.create({ quantity: 100 });

console.log(item1, item2); // { quantity: 100 } { quantity: 100 }
```

Si la propriété virtuelle et l'alias sont fournis en même temps, la dernière valeur est prise en compte.

```ts
const { data: item1 } = await StoreItem.create({
  quantity: 20,
  _virtualQuantity: 100,
});

const { data: item2 } = await StoreItem.create({
  _virtualQuantity: 11,
  quantity: 5,
});

console.log(item1, item2); // { quantity: 100 } { quantity: 5 }
```

Exemple 2 : Alias avec un nom non lié

```ts
const StoreItem = new Schema({
  quantity: {
    default: 0,
    dependsOn: "_virtualQuantity",
    resolver: ({ ctx }) => ctx._virtualQuantity,
  },
  _virtualQuantity: {
    alias: "qty",
    vitual: true,
    validator: validateVirtualQuantity,
  },
}).getModel();

// this
const { data: item1 } = await StoreItem.create({ _virtualQuantity: 100 });

// is the same as this
const { data: item2 } = await StoreItem.create({ qty: 100 });

console.log(item1, item2); // { quantity: 100 } { quantity: 100 }
```

> N.B : n'essayez pas d'accéder aux propriétés virtuelles dans le [`contexte d'opération`](../life-cycles.md#le-contexte-de-lopération) avec leurs alias, car elles ne sont pas reconnues là-bas. Les alias ne fonctionnent que lorsqu'ils sont passés aux méthodes `create` et `update` de vos modèles.

## Sanitiser

Ceci doit être utilisé lorsque votre propriété virtuelle peut exister sous plusieurs formes. Cette fonction est exécutée dès que la dernière étape de validation (post-validation) est terminée. Elle peut être synchrone ou asynchrone et n'a accès qu'à un seul argument, le [résumé d'opération](../life-cycles.md#le-résumé-de-lopération).

Un bon cas d'usage serait la gestion d'envois de fichiers. L'exemple ci-dessous montre comment vous pourriez téléverser un fichier vers un stockage local ou cloud, puis obtenir les métadonnées que vous souhaitez persister. Après l'assainissement, le résolveur des propriétés qui dépendent (`metadata` dans notre cas) de ces virtuelles est exécuté avec les nouvelles valeurs des propriétés virtuelles.

> N.B : si le sanitisateur lève une erreur, la valeur avant assainissement sera utilisée.

```ts
import { Schema, type IvoSummary } from "ivo";

type FileMetadata = { size: number; url: string };

type Input = {
  file: File | FileMetadata;
  name: string;
};

type Output = {
  id: string;
  metadata: FileMetadata;
  name: string;
};

const FileModel = new Schema<Input, Output>({
  id: { constant: true, value: generateID },
  metadata: {
    default: { size: 0, url: "" },
    dependsOn: "file",
    resolver: ({ ctx }) => ctx.file as FileMetadata,
  },
  name: { required: true, validator: validateName },
  file: {
    vitual: true,
    sanitizer: sanitizeFile,
    validator: validateFile,
  },
}).getModel();

async function sanitizeFile({ ctx: { file } }: IvoSummary<Input, Output>) {
  // upload file
  const { size, url } = await uploadFile(file);

  return { size, url } as FileMetadata;
}
```
