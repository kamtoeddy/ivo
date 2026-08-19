---
title: Required Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Required Fields

A required field is both an input and output field whose value must be provided at creation.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

async function main() {
  const UserModel = new Schema<any, { username: string }>((b) =>
  b.field(
  b
  .required("username")
  .validate((value) =>
  value.length >= 3
  ? true
  : { valid: false, reason: "Username must be at least 3 characters" }
  ),
  ),
  ).getModel();

  const { data, error } = await UserModel.create({ username: "ab" });
  console.log({ data, error: error?.payload });

}

main();`}
/>

## Rules

- A required field must have a `validate` or `allow` rule.
- It may also have a `reValidate` rule for updates.
- It supports `ignoreUpdate` and `readonly()` to prevent further updates.
- It may have `onDelete`, `onSuccess`, and `onFailure` listeners.
- Use `requiredError(...)` to customize the error message when the field is missing.

## Conditional requiredness

A lax field can be made conditionally required:

```ts
b.lax("email", null).required(({ input }) => [
  !input.phoneNumber,
  "Provide email or phone number",
]);
```
