---
title: Timestamps
---

import TsPlayground from '@site/src/components/TsPlayground';

# Timestamps

Timestamps are output-only fields that `ivo` populates automatically. Enable them
with the `timestamps` schema option.

## Default behavior

With `timestamps: true`, `ivo` adds `createdAt` and `updatedAt` to every created
or updated entity.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
  type User = {
    id: string;
    createdAt: Date;
    updatedAt: Date | null;
    username: string;
  };

  const UserModel = new Schema<{ username: string }, User>(
    (b) =>
      b
        .field(b.constant("id", () => "user-1"))
        .field(b.required("username").validate((v) =>
          typeof v === "string" && v.length >= 3
            ? true
            : { valid: false, reason: "Username too short" },
        )),
    { timestamps: true },
  ).getModel();

  const { data: created } = await UserModel.create({ username: "ada" });
  console.log("created:", created);

  const { data: updated } = await UserModel.update(created!, { username: "bob" });
  console.log("updated:", updated);
}

main();`}
/>

## Custom keys

Use an object to rename the fields. `updatedAt` can also be configured as
non-nullable.

```ts
new Schema((b) => ..., {
  timestamps: {
    createdAt: "created_at",
    updatedAt: { key: "updated_at", nullable: false },
  },
});
```

## Rules

- Timestamps are ignored if provided as input.
- `createdAt` is set once, during creation.
- `updatedAt` is set during creation and refreshed on every successful update.
- Setting `updatedAt` to `false` disables the update timestamp while keeping `createdAt`.
