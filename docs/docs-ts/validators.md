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

async function main() {
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

}

main();`}
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

See [Lax fields](./definitions/lax.md#allowed-values), [Required fields](./definitions/required.md#allowed-values), and [Virtual fields](./definitions/virtuals.md#allowed-values) for details.

## Built-in validation helpers

`ivo` ships with a few validators you can use as they are, or study to build your own. They all
return a validation response:

```ts
type ValidationResponse<T> =
  | { valid: true; validated: T }
  | { valid: false; reason: string; metadata: Record<string, unknown> | null };
```

### validateBoolean

```ts
import { validateBoolean } from "ivo";

validateBoolean("true"); // { valid: false, reason: "Expected a boolean", metadata: null }
validateBoolean(false); // { valid: true, validated: false }
```

### validateCreditCard

Checks that a 16-digit card **number** is well formed (Luhn check); it does not check the card
itself. Strings and numbers are accepted, and the validated value keeps the input's type.

```ts
import { validateCreditCard } from "ivo";

validateCreditCard(""); // { valid: false, reason: "Invalid card number", metadata: null }
validateCreditCard(5420596721435293); // { valid: true, validated: 5420596721435293 }
validateCreditCard("4111111111111111"); // { valid: true, validated: "4111111111111111" }
```

### validateEmail

The value is trimmed, then the **whole** value must be an email address.

```ts
import { validateEmail } from "ivo";

validateEmail(" john@doe.com"); // { valid: true, validated: "john@doe.com" }
validateEmail("contact john@doe.com now"); // { valid: false, reason: "Invalid email", metadata: null }
```

| Position | Parameter | Type    | Description                                     |
| -------- | --------- | ------- | ----------------------------------------------- |
| 1        | value     | unknown | The value to validate                           |
| 2        | regExp    | RegExp  | A custom regular expression to use (optional)   |

### makeArrayValidator

Builds an (async) validator for arrays of primitives or objects.

```ts
import { makeArrayValidator } from "ivo";

const validateGenres = makeArrayValidator({
  filter: (genre) => typeof genre === "string" && !!genre.trim(),
  modifier: (genre: string) => genre.trim().toLowerCase(),
  min: { value: 1, error: "Expected a non-empty array" },
  sort: true,
  unique: true,
});

await validateGenres(["action", null, "horror", 1, "comedy", "Horror", "crime"]);
// { valid: true, validated: ["action", "comedy", "crime", "horror"] }

await validateGenres(["   ", [], null, 144]);
// { valid: false, reason: "Expected a non-empty array", metadata: { min: 1 } }
```

The steps run in this order: `filter` → `modifier` → `postModFilter` → `unique` → `min`/`max` →
`map` → `sort`.

| Option          | Type                                  | Description                                                                                                  |
| --------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `filter`        | `(item) => boolean \| Promise<boolean>` | **Required.** Keeps the items it accepts. Async filters are awaited.                                       |
| `modifier`      | `(item) => T \| Promise<T>`           | Transforms each kept item.                                                                                   |
| `postModFilter` | `(item) => boolean \| Promise<boolean>` | Filters the modified items (requires `modifier`). Async filters are awaited.                               |
| `map`           | `(item) => T \| Promise<T>`           | Maps the final items (requires `modifier`).                                                                  |
| `unique`        | `boolean`                             | Removes duplicates, keeping the first occurrence. The original items are kept (a `Date` stays a `Date`).     |
| `uniqueKey`     | `string`                              | With `unique`, the property that identifies an object, e.g. `"id"`. Keys are compared by type (`1` ≠ `"1"`); for a repeated key, the last item is kept at the position of the first. |
| `min` / `max`   | `number \| { value: number; error: string }` | Size limits, checked after `unique`.                                                                  |
| `sort`          | `boolean \| (a, b) => number`         | `true` sorts with `<`; a function is used as the comparator.                                                 |
| `sortOrder`     | `"asc" \| "desc"`                     | With `sort: true`. Default: `"asc"`.                                                                          |

### makeNumberValidator

```ts
import { makeNumberValidator } from "ivo";

const validate = makeNumberValidator({ min: 10, max: 10.5 });

validate(9); // { valid: false, reason: "too_small", metadata: { min: 10, max: 10.5 } }
validate(10.01); // { valid: true, validated: 10.01 }
validate("10.05"); // { valid: false, reason: "Expected a number", metadata: null }
validate(10n); // bigints are accepted: { valid: true, validated: 10 }

makeNumberValidator({ allow: [0, -1, 35] })(30);
// { valid: false, reason: "Value not allowed", metadata: { allowed: [0, -1, 35] } }
```

| Option          | Type                                          | Description                                           |
| --------------- | --------------------------------------------- | ----------------------------------------------------- |
| `allow`         | `T[] \| { values: T[]; error: string }`       | Only these values are valid (excludes the options below). |
| `exclude`       | `T \| T[] \| { values: T \| T[]; error: string }` | Values that are never valid.                      |
| `min` / `max`   | `number \| { value: number; error: string }`  | Inclusive limits.                                     |
| `nullable`      | `boolean`                                     | Accept `null` and `undefined` (validated as `null`).  |

### makeStringValidator

```ts
import { makeStringValidator } from "ivo";

makeStringValidator({ max: 20, min: 3 })("Hello World!");
// { valid: true, validated: "Hello World!" }

makeStringValidator({ allow: ["apple", "banana", "watermelon"] })("pineapple");
// { valid: false, reason: "Value not allowed", metadata: { allowed: ["apple", "banana", "watermelon"] } }

const pattern = /^[a-zA-Z_\-\S]+$/;

makeStringValidator({ regExp: { value: pattern, error: "Invalid username" } })("dbj jkdbZvjkbv");
// { valid: false, reason: "Invalid username", metadata: null }
```

The steps run in this order: `exclude` → `allow` → `nullable` → type check → `regExp` →
`normalize` → `trim` → `escapeHTML` → `min`/`max`.

| Option          | Type                                          | Description                                                                                     |
| --------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `allow`         | `T[] \| { values: T[]; error: string }`       | Only these values are valid, returned as they are (excludes the options below).                 |
| `exclude`       | `T \| T[] \| { values: T \| T[]; error: string }` | Values that are never valid.                                                                |
| `escapeHTML`    | `boolean`                                     | Escape `&`, `<`, `>`, `"` and `'` as HTML entities. Default: **`true`**.                        |
| `min` / `max`   | `number \| { value: number; error: string }`  | Length limits. Default: `1` and `255`. Checked on the escaped value.                            |
| `normalize`     | `boolean`                                     | Unicode-normalize the value. Default: `true`.                                                   |
| `normalForm`    | `"NFC" \| "NFD" \| "NFKC" \| "NFKD"`          | The normalization form. Default: `"NFC"`.                                                       |
| `nullable`      | `boolean`                                     | Accept `""`, `null` and `undefined` (validated as `null`).                                      |
| `regExp`        | `{ value: RegExp; error: string }`            | The (untrimmed, unescaped) value must match it.                                                 |
| `trim`          | `boolean`                                     | Trim the value.                                                                                 |

#### Escaping HTML

Since v2.1.0, strings are HTML-escaped by default, so a value can be rendered in a page as text
without being interpreted as markup:

```ts
makeStringValidator()('<b>Tom & "Jerry"</b>');
// { valid: true, validated: "&lt;b&gt;Tom &amp; &quot;Jerry&quot;&lt;/b&gt;" }

makeStringValidator({ escapeHTML: false })('<b>Tom & "Jerry"</b>');
// { valid: true, validated: "<b>Tom & \"Jerry\"</b>" }
```

`min` and `max` apply to the escaped value (the one that is stored), so escaping can make a value
too long:

```ts
makeStringValidator({ max: 5 })("<&"); // "&lt;&amp;" is 9 characters
// { valid: false, reason: "too_long", metadata: { min: 1, max: 5 } }
```

:::caution
Escaping is not idempotent: a value that is validated again (e.g. already-escaped text sent back
in an update) is escaped again (`&amp;` becomes `&amp;amp;`). Set `escapeHTML: false` for fields
that hold markup or that you escape when rendering.
:::
