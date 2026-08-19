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
`}
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
