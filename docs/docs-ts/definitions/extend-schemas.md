---
title: Extending Schemas
---

import TsPlayground from '@site/src/components/TsPlayground';

# Extending Schemas

Use `.extend()` to create a new schema that inherits fields from an existing one.

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

| Option             | Type                 | Default | Description                                                            |
| ------------------ | -------------------- | ------- | ---------------------------------------------------------------------- |
| `useParentOptions` | `boolean`            | `true`  | Inherit `equalityDepth`, `sanitizeError`, and `timestamps`.            |
| `remove`           | `string \| string[]` | `[]`    | Field names to drop from the parent schema.                            |
| ...                | `NS.Options`         | —       | Any other schema option can be provided and will override inheritance. |

## Inheritance rules

- Parent definitions are copied first, then the extension builder is applied.
- Re-declaring a parent field name **overwrites** the parent definition.
- `remove` deletes fields after copying.
- Only `equalityDepth`, `sanitizeError`, and `timestamps` are inherited when
  `useParentOptions` is `true`. Set it to `false` to start from the options
  passed to `extend()` only.

```ts
const StrictAdminSchema = UserSchema.extend<AdminInput, Admin>(
  (b) => b.field(b.required("role").allow(["admin"])),
  { useParentOptions: false, timestamps: false },
);
```
