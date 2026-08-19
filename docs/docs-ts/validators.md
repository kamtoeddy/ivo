---
title: Validators
---

import TsPlayground from '@site/src/components/TsPlayground';

# Validators

Validators decide whether a value is acceptable. They can be synchronous or asynchronous.

## Return types

A validator may return:

- `true` — value is valid.
- `{ valid: true }` — value is valid.
- `{ valid: false, reason: string }` — value is invalid with a reason.
- `false` — value is invalid (uses default reason).

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

function validateUsername(username: string) {
if (username.length < 3) {
return { valid: false, reason: "Username must be at least 3 characters" };
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

## Async validators

```ts
async function makeSureUsernameIsUnique(username: string) {
  const existing = await usersDb.findByUsername(username);
  return existing ? { valid: false, reason: "Username already taken" } : true;
}
```

## Multiple validators

You can pass an array of validators. They run in order and all must pass.

```ts
b.required("username").validate([validateUsername, makeSureUsernameIsUnique]);
```

## Re-validators

Re-validators run during updates. If not provided, the create validator is reused.

```ts
b.required("username")
  .validate(validateUsername)
  .reValidate(validateUsernameUpdate);
```

## Allowed values

As an alternative to a validator, you can restrict a field to a fixed set of values:

```ts
b.required("role").allow(["admin", "editor", "viewer"]);
```
