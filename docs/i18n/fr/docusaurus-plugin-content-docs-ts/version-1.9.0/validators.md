---
title: "Validateurs"
---

# Validateurs

Un validateur est une fonction qui évalue la validité d'une propriété (c'est-à-dire un validateur par propriété). Il peut être synchrone/asynchrone, mais doit se comporter comme indiqué ci-dessous.

Les propriétés pouvant avoir des validateurs peuvent en avoir jusqu'à 2 (1 principal et 1 secondaire) `N.B: they have slightly different signatures`

```ts
import type { IvoSummary } from "ivo";

type Input = {}; // the input type of your model
type Output = {}; // the output type of your model

type FieldError = {
  reason: string;
  metadata?: Record<string, any> | null;
};

type ValidationResults =
  | boolean
  | {
      valid: true; // tells if data was valid or not
      validated?: Input[K]; // the validated values passed which could have been formated in the custom validator (i.e made ready for the db). "K" here represents the property being validated
    }
  | {
      metadata?: Record<string, any>; // an object that will contain extra info on why validation failed
      reason?: string;
      valid: false;
    };

function primaryValidator(
  value: any,
  summary: IvoSummary<Input, Output, CtxOptions>,
) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function secondaryValidator<T>(
  value: T,
  summary: IvoSummary<Input, Output, CtxOptions>,
) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function validator1(value: any) {
  // validation logic here

  if (valid) return { valid, validated };

  return { reason, valid };
}

function validator2(value: any) {
  // validation logic here

  if (!valid) return false;

  return true;
}

const Model = new Schema({
  dateOfBirth: { required: true, validator: isValidDateOfBirth },
  email: {
    required: true,
    //          👇 primary  &  👇 secondary validators
    validator: [validateEmail, isEmailUnique],
  },
});
```

Dans l'extrait de code ci-dessus, nous avons 2 validateurs ; `validator1` et `validator2`

Bien que les deux fonctionnent de la même manière, `validator1` est recommandé car :

- il est préférable de fournir la raison pour laquelle la validation a échoué ;
- retourner la valeur `validated` donne à TypeScript plus d'informations sur le type de cette propriété, notamment si vous n'avez pas explicitement fourni les interfaces d'entrée et de sortie de votre schéma.

> N.B : si le validateur ne retourne pas de valeur validée ou si celle-ci est `undefined`, la valeur directement passée sera utilisée, même `undefined`.

> N.B : si le validateur lève une erreur, la validation de ladite propriété échouera avec la raison `validation failed`

## Validation postérieure

Si vous avez besoin d'effectuer plusieurs étapes de validation sur plus d'un champ, vous pouvez le faire avec l'option `postValidate` de votre schéma.

### PostValidationConfig:

```ts

type PostValidator = (
    summary: IvoSummary<Input, Output, CtxOptions>,
    propertiesProvided
  ) =>
    | void
    | ValidationResponseObject
    | Promise<void | ValidationResponseObject>

type InputProperty=  keyof Input

type PostValidationConfig = {
  properties: [InputProperty, InputProperty, ...InputProperty[]]; // array of at least 2 input properties
  validator: PostValidator | (PostValidator | PostValidator[])[] ;
};

// and the schema postValidate option's signature

type Options = {
  ...otherOptions;
  postValidate: PostValidationConfig | PostValidationConfig[];
};
```

Comme illustré dans l'exemple ci-dessus, `PostValidateConfig` est un objet qui attend deux propriétés :

- `properties` : un tableau d'au moins deux propriétés d'entrée uniques de votre schéma
- `validator`
  - Une fonction ou un tableau de fonctions (synchrones/asynchrones) qui détermineront la validité de l'opération par rapport à ses propriétés.
  - Ce(s) validateur(s) est/sont invoqué(s) immédiatement après la résolution des propriétés dépendantes, et si au moins une des propriétés de sa configuration a été fournie lors d'une mise à jour, mais il est toujours appelé lors de la création
  - `N.B` : si le validateur est un tableau, les validateurs au niveau de profondeur 1 s'exécutent séquentiellement, tandis que ceux au niveau de profondeur 2 s'exécutent en parallèle

> **Si l'option `postValidate` est un tableau, chaque ensemble de propriétés doit être unique pour chaque configuration.**

```ts
// ❌ both configs have wxactly the same properties
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ["email", "username"], validator },
    { properties: ["username", "email"], validator },
  ],
});

// ✅ as from v1.5.1 you can provide subsets of other configs
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ["email", "username", "date_of_birth"], validator },
    { properties: ["email", "username"], validator },
  ],
});

// ✅ this works
const schema = new Schema(definitions, {
  postValidate: [
    { properties: ["email", "username"], validator },
    { properties: ["role", "username"], validator },
  ],
});
```

Exemple :

```ts
type EventInput = {
  host: User["id"];
  guests: User["id"][];
  startTime: Date;
  stopTime: Date;
};

type Event = { id: number } & EventInput;
```

En supposant que la structure ci-dessus représente un événement (entité) dans un système de gestion d'événements que vous construisez. Cet événement a les propriétés id, host, guests, startTime et stopTime, et les exigences sont les suivantes :

- le `host` et les `guests` doivent être des identifiants d'utilisateurs valides dans le système
- `startTime` doit être supérieur à `stopTime`
- seul l'`id` de l'événement ne peut pas être modifié
- chaque fois que `host`, `guests`, `startTime` ou `stopTime` sont modifiés, vous devez vous assurer que le nouvel état de l'événement respecte la disponibilité du `host` et de tous les `guests`, c'est-à-dire que le `host` et les `guests` ne doivent pas être réservés pour un autre événement pendant cette période

Avec les exigences ci-dessus, il est clair que nous devons effectuer des validations individuelles pour `host`, `guests`, `startTime` et `stopTime`, suivies d'une validation inter-champs pour les 4 propriétés.

```ts
const Model = new Schema(
  {
    id: { constant: true, value: generateEventId },
    host: { required: true, validator: validateHostId },
    guests: { required: true, validator: validateGuestIds },
    startTime: { required: true, validator: validateStartTime },
    stopTime: { required: true, validator: validateStopTime },
  },
  {
    postValidate: {
      properties: ["host", "guests", "startTime", "stopTime"],
      async validator({ ctx }: IvoSummary<EventInput, Event>) {
        // this is triggered when the individual
        // validations have all been successful

        const { host, guests, startTime, stopTime } = ctx;

        const [isHostAvailable, guestsAvailable] = await Promise.all([
          await isHostAvailableBetween(host, startTime, stopTime),
          await getGuestsAvailableBetween(guests, startTime, stopTime),
        ]);

        const areAllGuestsAvailable = guestsAvailable.length == guests.length;

        if (isHostAvailable && areAllGuestsAvailable) return;

        const errors = {};

        if (!isHostAvailable) errors["host"] = "Host not available";

        if (!areAllGuestsAvailable)
          errors["guests"] = {
            reason: "Some guests are not available",
            metadata: {
              unAvailableGuests: guests.filter(
                (g) => !guestsAvailable.includes(g),
              ),
            },
          };

        return errors;
      },
    },
  },
).getModel();
```

> N.B : **Cette option n'est pas héritée lors de l'extension d'un schéma.**

> N.B : si le post-validateur lève une erreur, la validation des propriétés fournies liées à ce validateur échouera toutes avec la raison `validation failed`

## Flux de validation

La validation des données peut se dérouler en plusieurs étapes selon la configuration de votre schéma.

1. Validation principale

   - À ce stade, les validateurs principaux sont déclenchés, les valeurs par défaut et constantes sont attribuées ou générées
   - Le `ctx` de l'opération ici n'est pas sûr, car il est constitué de données brutes, mais il peut être mis à jour par les valeurs validées retournées par les validateurs

1. Validation conditionnelle des champs requis

   - Ici, les propriétés requises conditionnelles sont évaluées
   - Le `ctx` de l'opération est déjà sûr, car les valeurs validées de l'étape de validation principale ont été utilisées pour mettre à jour le `ctx`
   - Le `ctx` de l'opération ne peut pas être mis à jour à ce stade

1. Validation secondaire

   - C'est ici que les validateurs secondaires sont déclenchés
   - Le `ctx` de l'opération est également sûr grâce à la validation principale et peut être mis à jour par les valeurs validées retournées par les validateurs

1. Validation postérieure

   - Ici, les vérifications post-validation sont évaluées avec un `ctx` d'opération sûr
   - Pour mettre à jour le `ctx` de l'opération, le validateur peut retourner la valeur validée comme ci-dessous

   ```ts
   function postValidator({}: IvoSummary) {
     return {
       propertyName: { validated: newValue },
       // other properties here
     };
   }
   ```

   > N.B : toute tentative de mise à jour de la valeur d'une propriété (en utilisant la méthode ci-dessus) non enregistrée dans une configuration de post-validation spécifique sera ignorée

1. Assainissement des propriétés virtuelles ; plus d'informations [ici](./definitions/virtuals.md#sanitiser)

1. Résolution des propriétés dépendantes ; plus d'informations [ici](./definitions/dependents.md)

## Aides de validation intégrées

Voici quelques validateurs intégrés que vous pouvez étudier pour construire vos propres validateurs :

### validateBoolean

Pour valider les valeurs booléennes

```ts
import { validateBoolean } from "ivo";

console.log(validateBoolean("true")); // { reason: "Expected a boolean", valid: false }

console.log(validateBoolean(false)); // { valid: true, validated: false }
```

### validateCreditCard

Une petite méthode utilitaire pour tester si un **`numéro de carte`** de crédit/débit est valide ; pas la carte elle-même.

```ts
import { validateCreditCard } from "ivo";

console.log(validateCreditCard(""));
// { reason: "Invalid card number", valid: false }

console.log(validateCreditCard(5420596721435293));
// { valid: true, validated: 5420596721435293}

console.log(validateCreditCard("5420596721435293"));
// { valid: true, validated: "5420596721435293"}
```

Elle retourne :

```ts
type ValidationResponse =
  | { reasons: string[]; valid: false }
  | { valid: true; validated: number | string };
```

### validateEmail

Pour valider les adresses e-mail

```ts
import { validateEmail } from "ivo";

console.log(validateEmail("dbj jkdbZvjkbv")); // { reason: "Invalid email", valid: false }

validateEmail(" john@doe.com"); // {  valid: true, validated: "john@doe.com" }
```

#### Paramètres

| Position | Propriété   | Type   | Description                                     |
| -------- | ----------- | ------ | ----------------------------------------------- |
| 1        | value       | any    | La valeur que vous souhaitez valider            |
| 2        | customRegEx | RegExp | L'expression régulière personnalisée à utiliser |

### makeArrayValidator

Vous pouvez valider un tableau de valeurs de votre choix. Un tableau de primitives ou d'objets.

```ts
import { makeArrayValidator } from "ivo";

const options = {
  min: { value: 1, error: "Expected a non-empty array" },
  sorted: true,
  filter: (genre) => typeof genre === "string" && genre?.trim(),
  modifier: (genre) => genre?.trim().toLowerCase(),
};

const movieGenres = ["action", null, "horror", 1, "comedy", "Horror", "crime"];

const validate = makeArrayValidator(options);

console.log(validate(movieGenres)); // { valid: true, validated: ["action", "comedy", "crime", "horror"] }

const invalids = ["   ", [], null, 144];

console.log(validate(invalids)); // { reason: "Expected a non-empty array", valid: false }
```

#### Options

| Propriété | Type            | Description                                                                                                         |
| --------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| filter    | function        | Une fonction synchrone ou asynchrone pour filtrer le tableau. Par défaut : **(data) => false**                      |
| modifier  | function        | Une fonction synchrone ou asynchrone pour modifier (formater) les valeurs individuelles. Par défaut : **undefined** |
| sorted    | boolean         | Indique si le tableau doit être trié. Par défaut : **true**                                                         |
| sorter    | function        | Fonction pour trier les valeurs. Par défaut : **undefined**                                                         |
| sortOrder | 'asc' \| 'desc' | Ordre utilisé pour la comparaison lorsque `sorted` vaut `true` et `sorter` vaut `undefined`                         |
| unique    | boolean         | Indique si le tableau doit contenir des valeurs uniques. Par défaut : **true**                                      |
| uniqueKey | string          | Une clé (propriété) des objets du tableau utilisée comme critère d'unicité, par exemple : "id". Par défaut : **""** |

### makeNumberValidator

Pour valider les nombres

```ts
import { makeNumberValidator } from "ivo";

type AllowConfig<T> =
  | ArrayOfMinSizeTwo<T>
  | { values: ArrayOfMinSizeTwo<T>; error: string | string[] };

type ExclusionConfig<T> =
  | T
  | ArrayOfMinSizeTwo<T>
  | { values: T | ArrayOfMinSizeTwo<T>; error: string | string[] };

type ValueError<T = number> = { value: T; error: string | string[] };

type NumberValidatorOptions<T extends number | any = number> = {
  exclude?: ExclusionConfig<T>;
} & XOR<
  { allow: AllowConfig<T> },
  {
    max?: number | ValueError;
    min?: number | ValueError;
    nullable?: boolean;
  }
>;

const options = { min: 10, max: 10.5 };

const validate = makeNumberValidator(options);

console.log(validate(10)); // { reason: "too small", valid: false, metadata: { min: 10, max: 10.5, inclusiveBottom: false,  inclusiveTop: true } }

console.log(validate(10.01)); // { valid: true, validated: 10.01, metadata }

console.log(validate("10.05")); // { valid: true, validated: 10.05, metadata }

console.log(makeNumberValidator({ allow: [0, -1, 35] }, 30)); // { reason: "Value not allowed", valid: false, metadata: { allowed: [0, -1, 35] } }
```

### makeStringValidator

Pour valider les chaînes de caractères

```ts
import { makeStringValidator } from "ivo";

type StringValidatorOptions<T extends string | any = string> = {
  exclude?: ExclusionConfig<T>;
} & XOR<
  { allow: AllowConfig<T> },
  {
    max?: number | ValueError;
    min?: number | ValueError;
    normalForm?: "NFC" | "NFD" | "NFKC" | "NFKD";
    normalize?: boolean;
    nullable?: boolean;
    regExp?: ValueError<RegExp>;
    trim?: boolean;
  }
>;

const pattern = /^[a-zA-Z_\-\S]+$/;

console.log(
  makeStringValidator(
    {
      regExp: {
        value: pattern,
        error: `string should match this pattern: ${pattern}`,
      },
    },
    "dbj jkdbZvjkbv",
  ),
); // { reason: "Value not allowed", valid: false }

console.log(makeStringValidator({ max: 20, min: 3 }, "Hello World!")); // { valid: true, validated: "Hello World!" }

console.log(
  makeStringValidator(
    { allow: ["apple", "banana", "watermelon"] },
    "pineapple",
  ),
); // { reason: "Value not allowed", valid: false, metadata: { allowed: ['apple', 'banana', 'watermelon'] } }
```
