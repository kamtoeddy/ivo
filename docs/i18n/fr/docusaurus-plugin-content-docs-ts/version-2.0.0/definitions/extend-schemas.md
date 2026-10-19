---
title: Extension de schémas
---

import TsPlayground from '@site/src/components/TsPlayground';

# Extension de schémas

Utilisez `.extend()` pour créer un nouveau schéma qui hérite des champs d'un schéma existant.

<TsPlayground
  ivoVersion="local"
  code={`import { Schema } from "ivo";

async function main() {
  type UserInput = { email?: string };
  type User = { id: string; email: string };

  type AdminInput = UserInput & { role?: string };
  type Admin = User & { role: string };

  const UserSchema = new Schema<UserInput, User>(
    (b) =>
      b
        .field(b.constant("id", () => "user-1"))
        .field(b.lax("email", "")),
    { timestamps: true },
  );

  const AdminModel = UserSchema.extend<AdminInput, Admin>(
    (b) => b.field(b.required("role").allow(["admin", "super-admin"])),
    { useParentOptions: true },
  ).getModel();

  const { data } = await AdminModel.create({ email: "admin@example.com", role: "admin" });
  console.log(data);

}

main();`}
/>

## Options

| Option             | Type                 | Défaut | Description                                                                 |
| ------------------ | -------------------- | ------ | --------------------------------------------------------------------------- |
| `useParentOptions` | `boolean`            | `true` | Hérite de `equalityDepth`, `sanitizeError` et `timestamps`.                 |
| `remove`           | `string \| string[]` | `[]`   | Noms des champs à supprimer du schéma parent.                               |
| ...                | `NS.Options`         | —      | Toute autre option de schéma peut être fournie et remplacera l'héritage.    |

## Règles d'héritage

- Les définitions du parent sont d'abord copiées, puis le constructeur d'extension est appliqué.
- Redéclarer un nom de champ parent **écrase** la définition du parent.
- `remove` supprime les champs après la copie.
- Seuls `equalityDepth`, `sanitizeError` et `timestamps` sont hérités quand `useParentOptions` vaut
  `true`. Définissez-le à `false` pour ne partir que des options passées à `extend()`.

```ts
const StrictAdminSchema = UserSchema.extend<AdminInput, Admin>(
  (b) => b.field(b.required('role').allow(['admin'])),
  { useParentOptions: false, timestamps: false },
);
```
