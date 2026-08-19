---
title: Dependent Fields
---

import TsPlayground from '@site/src/components/TsPlayground';

# Dependent Fields

A dependent field is a purely output field whose value changes whenever at least one field it
depends on is provided and accepted.

<TsPlayground
ivoVersion="local"
code={`import { Schema } from "ivo";

type Input = { firstName?: string; lastName?: string };
type Output = { firstName: string; lastName: string; fullName: string };

const UserModel = new Schema<Input, Output>((b) =>
b
.field(b.lax("firstName", ""))
.field(b.lax("lastName", ""))
.field(
b
.dependent("fullName", ["firstName", "lastName"])
.default("")
.resolve(({ ctx }) => \`\${ctx.firstName} \${ctx.lastName}\`.trim()),
),
).getModel();

const { data } = await UserModel.create({ firstName: "Ada", lastName: "Lovelace" });
console.log(data);
`}
/>

## Rules

- A dependent field must have either a static default or a resolver for the default value.
- It must depend on at least one other field: lax, required, virtual, or another dependent field.
- It must have a resolver to generate new values whenever a parent field is provided and accepted.
- It may use `readonly()` to stop accepting further updates once its value differs from its default.
- It may have `onDelete` and `onSuccess` listeners.
