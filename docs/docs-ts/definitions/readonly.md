---
title: Readonly Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Readonly Fields

Mark a field as `readonly()` to lock it once it has diverged from its default value.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

const CodeModel = new Schema<{ code: string }, { code: string }>(
(b) => b.field(b.lax("code", "PENDING").readonly()),
).getModel();

const { data: created } = await CodeModel.create({ code: "ABC" });
console.log("created:", created);

const { data: updated } = await CodeModel.update(created, { code: "DEF" });
console.log("updated:", updated);

// Now the value has diverged from the default, further updates are ignored.
const { data, error } = await CodeModel.update(
{ ...created, ...updated },
{ code: "GHI" },
);
console.log("second:", { data, error });
`}
/>

## Availability

| Field type  | `.readonly()` | Notes                                                                   |
| ----------- | ------------- | ----------------------------------------------------------------------- |
| `lax`       | Yes           | Only allowed when the default is **static**, not a resolver function.   |
| `required`  | Yes           | The field is locked permanently after creation.                         |
| `dependent` | Yes           | Only allowed with a static default; resolver obeys unlock/freeze rules. |
| `virtual`   | No            | Input-only fields cannot be readonly.                                   |
| `constant`  | No            | Constants are already immutable.                                        |

## Behavior

- At **creation** the provided value (or default) is accepted normally.
- For fields with a static default, updates are accepted only while the current
  value still equals that default. Once the value changes, the field is silently
  ignored during subsequent updates.
- For fields without a static default (required, or dependent with a resolver
  default), the field is locked immediately after creation.
