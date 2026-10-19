---
title: Validateurs
---

import TsPlayground from '@site/src/components/TsPlayground';

# Validateurs

Les validateurs déterminent si une valeur est acceptable. Ils peuvent être synchrones ou asynchrones.

## Types de retour

Un validateur peut renvoyer :

- `true` — la valeur est valide.
- `{ valid: true }` — la valeur est valide.
- `{ valid: false, reason: string }` — la valeur est invalide avec une raison.
- `false` — la valeur est invalide (utilise la raison par défaut).

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  function validateUsername(username: string) {
    if (username.length < 3) {
      return { valid: false, reason: "Le nom d'utilisateur doit faire au moins 3 caractères" };
    }
    return true;
  }

  const UserModel = new Schema<any, { username: string }>((b) =>
    b.field(b.required("username").validate(validateUsername)),
  ).getModel();

  const { data, error } = await UserModel.create({ username: "ab" });
  console.log({ data, error: error?.payload });

}

main();`}
/>

## Validateurs asynchrones

```ts
async function makeSureUsernameIsUnique(username: string) {
  const existing = await usersDb.findByUsername(username);
  return existing ? { valid: false, reason: 'Ce nom d\'utilisateur est déjà pris' } : true;
}
```

## Validateurs multiples

Vous pouvez passer un tableau de validateurs. Ils s'exécutent dans l'ordre et doivent tous réussir.

```ts
b.required('username').validate([validateUsername, makeSureUsernameIsUnique]);
```

## Re-validateurs

Les re-validateurs s'exécutent lors des mises à jour. S'ils ne sont pas fournis, le validateur de
création est réutilisé.

```ts
b.required('username').validate(validateUsername).reValidate(validateUsernameUpdate);
```

## Valeurs autorisées

Comme alternative à un validateur, vous pouvez restreindre un champ à un ensemble fixe de valeurs :

```ts
b.required('role').allow(['admin', 'editor', 'viewer']);
```

## Validateurs intégrés

`ivo` fournit quelques validateurs que vous pouvez utiliser tels quels, ou étudier pour écrire les
vôtres. Ils renvoient tous une réponse de validation :

```ts
type ValidationResponse<T> =
  | { valid: true; validated: T }
  | { valid: false; reason: string; metadata: Record<string, unknown> | null };
```

### validateBoolean

```ts
import { validateBoolean } from 'ivo';

validateBoolean('true'); // { valid: false, reason: "Expected a boolean", metadata: null }
validateBoolean(false); // { valid: true, validated: false }
```

### validateCreditCard

Vérifie qu'un **numéro** de carte à 16 chiffres est bien formé (algorithme de Luhn) ; la carte
elle-même n'est pas vérifiée. Les chaînes et les nombres sont acceptés, et la valeur validée garde
le type de l'entrée.

```ts
import { validateCreditCard } from 'ivo';

validateCreditCard(''); // { valid: false, reason: "Invalid card number", metadata: null }
validateCreditCard(5420596721435293); // { valid: true, validated: 5420596721435293 }
validateCreditCard('4111111111111111'); // { valid: true, validated: "4111111111111111" }
```

### validateEmail

La valeur est nettoyée (`trim`), puis la valeur **entière** doit être une adresse e-mail.

```ts
import { validateEmail } from 'ivo';

validateEmail(' john@doe.com'); // { valid: true, validated: "john@doe.com" }
validateEmail('contact john@doe.com now'); // { valid: false, reason: "Invalid email", metadata: null }
```

| Position | Paramètre | Type    | Description                                            |
| -------- | --------- | ------- | ------------------------------------------------------ |
| 1        | value     | unknown | La valeur à valider                                    |
| 2        | regExp    | RegExp  | Une expression régulière personnalisée (facultative)   |

### makeArrayValidator

Crée un validateur (asynchrone) pour des tableaux de primitives ou d'objets.

```ts
import { makeArrayValidator } from 'ivo';

const validateGenres = makeArrayValidator({
  filter: (genre) => typeof genre === 'string' && !!genre.trim(),
  modifier: (genre: string) => genre.trim().toLowerCase(),
  min: { value: 1, error: 'Expected a non-empty array' },
  sort: true,
  unique: true,
});

await validateGenres(['action', null, 'horror', 1, 'comedy', 'Horror', 'crime']);
// { valid: true, validated: ["action", "comedy", "crime", "horror"] }

await validateGenres(['   ', [], null, 144]);
// { valid: false, reason: "Expected a non-empty array", metadata: { min: 1 } }
```

Les étapes s'exécutent dans cet ordre : `filter` → `modifier` → `postModFilter` → `unique` →
`min`/`max` → `map` → `sort`.

| Option          | Type                                    | Description                                                                                                  |
| --------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `filter`        | `(item) => boolean \| Promise<boolean>` | **Obligatoire.** Garde les éléments acceptés. Les filtres asynchrones sont attendus.                         |
| `modifier`      | `(item) => T \| Promise<T>`             | Transforme chaque élément gardé.                                                                             |
| `postModFilter` | `(item) => boolean \| Promise<boolean>` | Filtre les éléments transformés (nécessite `modifier`). Les filtres asynchrones sont attendus.               |
| `map`           | `(item) => T \| Promise<T>`             | Transforme les éléments finaux (nécessite `modifier`).                                                       |
| `unique`        | `boolean`                               | Supprime les doublons en gardant la première occurrence. Les éléments d'origine sont gardés (une `Date` reste une `Date`). |
| `uniqueKey`     | `string`                                | Avec `unique`, la propriété qui identifie un objet, p. ex. `"id"`. Les clés sont comparées avec leur type (`1` ≠ `"1"`) ; pour une clé répétée, le dernier élément est gardé à la place du premier. |
| `min` / `max`   | `number \| { value: number; error: string }` | Limites de taille, vérifiées après `unique`.                                                            |
| `sort`          | `boolean \| (a, b) => number`           | `true` trie avec `<` ; une fonction est utilisée comme comparateur.                                          |
| `sortOrder`     | `"asc" \| "desc"`                       | Avec `sort: true`. Par défaut : `"asc"`.                                                                     |

### makeNumberValidator

```ts
import { makeNumberValidator } from 'ivo';

const validate = makeNumberValidator({ min: 10, max: 10.5 });

validate(9); // { valid: false, reason: "too_small", metadata: { min: 10, max: 10.5 } }
validate(10.01); // { valid: true, validated: 10.01 }
validate('10.05'); // { valid: false, reason: "Expected a number", metadata: null }
validate(10n); // les bigints sont acceptés : { valid: true, validated: 10 }

makeNumberValidator({ allow: [0, -1, 35] })(30);
// { valid: false, reason: "Value not allowed", metadata: { allowed: [0, -1, 35] } }
```

| Option        | Type                                              | Description                                                   |
| ------------- | ------------------------------------------------- | ------------------------------------------------------------- |
| `allow`       | `T[] \| { values: T[]; error: string }`           | Seules ces valeurs sont valides (exclut les options suivantes). |
| `exclude`     | `T \| T[] \| { values: T \| T[]; error: string }` | Valeurs jamais valides.                                       |
| `min` / `max` | `number \| { value: number; error: string }`      | Limites inclusives.                                           |
| `nullable`    | `boolean`                                         | Accepte `null` et `undefined` (validés comme `null`).         |

### makeStringValidator

```ts
import { makeStringValidator } from 'ivo';

makeStringValidator({ max: 20, min: 3 })('Hello World!');
// { valid: true, validated: "Hello World!" }

makeStringValidator({ allow: ['apple', 'banana', 'watermelon'] })('pineapple');
// { valid: false, reason: "Value not allowed", metadata: { allowed: ["apple", "banana", "watermelon"] } }

const pattern = /^[a-zA-Z_\-\S]+$/;

makeStringValidator({ regExp: { value: pattern, error: 'Invalid username' } })('dbj jkdbZvjkbv');
// { valid: false, reason: "Invalid username", metadata: null }
```

Les étapes s'exécutent dans cet ordre : `exclude` → `allow` → `nullable` → vérification du type →
`regExp` → `normalize` → `trim` → `escapeHTML` → `min`/`max`.

| Option        | Type                                              | Description                                                                                   |
| ------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `allow`       | `T[] \| { values: T[]; error: string }`           | Seules ces valeurs sont valides, renvoyées telles quelles (exclut les options suivantes).     |
| `exclude`     | `T \| T[] \| { values: T \| T[]; error: string }` | Valeurs jamais valides.                                                                       |
| `escapeHTML`  | `boolean`                                         | Échappe `&`, `<`, `>`, `"` et `'` en entités HTML. Par défaut : **`true`**.                   |
| `min` / `max` | `number \| { value: number; error: string }`      | Limites de longueur. Par défaut : `1` et `255`. Vérifiées sur la valeur échappée.             |
| `normalize`   | `boolean`                                         | Normalise la valeur (Unicode). Par défaut : `true`.                                           |
| `normalForm`  | `"NFC" \| "NFD" \| "NFKC" \| "NFKD"`              | La forme de normalisation. Par défaut : `"NFC"`.                                              |
| `nullable`    | `boolean`                                         | Accepte `""`, `null` et `undefined` (validés comme `null`).                                   |
| `regExp`      | `{ value: RegExp; error: string }`                | La valeur (non nettoyée, non échappée) doit y correspondre.                                   |
| `trim`        | `boolean`                                         | Supprime les espaces au début et à la fin.                                                    |

#### Échappement HTML

Depuis la v2.1.0, les chaînes sont échappées par défaut, pour qu'une valeur puisse être affichée
dans une page comme du texte sans être interprétée comme du balisage :

```ts
makeStringValidator()('<b>Tom & "Jerry"</b>');
// { valid: true, validated: "&lt;b&gt;Tom &amp; &quot;Jerry&quot;&lt;/b&gt;" }

makeStringValidator({ escapeHTML: false })('<b>Tom & "Jerry"</b>');
// { valid: true, validated: "<b>Tom & \"Jerry\"</b>" }
```

`min` et `max` s'appliquent à la valeur échappée (celle qui est enregistrée) ; l'échappement peut
donc rendre une valeur trop longue :

```ts
makeStringValidator({ max: 5 })('<&'); // "&lt;&amp;" fait 9 caractères
// { valid: false, reason: "too_long", metadata: { min: 1, max: 5 } }
```

:::caution
L'échappement n'est pas idempotent : une valeur validée une seconde fois (p. ex. un texte déjà
échappé renvoyé lors d'une mise à jour) est échappée de nouveau (`&amp;` devient `&amp;amp;`).
Utilisez `escapeHTML: false` pour les champs qui contiennent du balisage ou que vous échappez à
l'affichage.
:::
