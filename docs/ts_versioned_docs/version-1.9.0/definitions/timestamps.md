---
title: "Timestamps"
sidebar_position: 6
---

# Timestamps

Timestamp fields are output-only fields automatically populated by the schema when a record is created or updated.

- A schema can declare a `createdAt` field (set once, on creation).
- A schema can declare an `updatedAt` field (set on creation and on every update).
- `updatedAt` can be optional, in which case it is only updated when the field already has a value.

## Configuration

Enable timestamps via the schema options:

```ts
new Schema(definitions, { timestamps: true });
```

Override the default field names:

```ts
new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: "updated_at" },
});
```

Use only one timestamp:

```ts
new Schema(definitions, {
  timestamps: { createdAt: "created_at", updatedAt: false },
});
```

Make `updatedAt` non-nullable:

```ts
new Schema(definitions, {
  timestamps: { updatedAt: { key: "updated_at", nullable: false } },
});
```

## Rules

- Timestamps are ignored if provided as input.
- `createdAt` is set once during creation.
- `updatedAt` is refreshed on every successful update.

## API summary

| Option     | Type                                       | Required | Description                                                      |
| ---------- | ------------------------------------------ | -------- | ---------------------------------------------------------------- |
| timestamps | `boolean \| object`                        | No       | Schema option that enables timestamp fields.                     |
| createdAt  | `boolean \| string`                        | No       | Field name for creation time. Default `createdAt`.               |
| updatedAt  | `boolean \| string \| { key?, nullable? }` | No       | Field name and nullability for update time. Default `updatedAt`. |
