# Builder pattern migration - TODO

Migrating field definitions from plain object literals to the Rust-style
typestate builder pattern (`src/schema/fields/`), across the source types and
the full `tests/` suite.

## Done

- [x] Widen `types.ts` (export missing types, `Buildable` unions for
      lax/required/virtual field slots)
- [x] `LaxFieldBuilder` (`fields/lax.ts`)
- [x] `RequiredFieldBuilder` (`fields/required.ts`)
- [x] `VirtualFieldBuilder` (`fields/virtual.ts`)
- [x] Wire all three into `createFieldBuilder()` factory
- [x] Source (`tsconfig.json`) and test (`tests/tsconfig.json`) typecheck
      clean

- [x] Integration tests for lax/required/virtual builders
      (`tests/samples/field-builder-{lax,required,virtual}.test.ts`, 36 new
      tests, `@ts-expect-error` lines verified via disable/restore ritual)
- [x] Verify builder foundation end-to-end (617 pass / 0 fail,
      `bun run cleanup` clean)

## In progress

- [ ] Migrate `tests/definitions/*.ts` to builder pattern (discarding
      scenarios the builder makes unrepresentable, e.g. `allow` + `validate`
      together). Approach: only the well-formed `valid`/`behaviour` schemas
      are converted to builder syntax; `invalid`-config tests deliberately
      pass malformed raw objects to test schema-core's defensive runtime
      validation and are left as plain objects (the builder can't represent
      malformed configs by design). Done so far: lax-properties, basic,
      extended-schemas, constant-properties, readonly-properties,
      dependent-properties, life-cycle-handlers, required-properties,
      allowed-values, should-init-and-update-rule (589/589 passing; 28
      tests discarded in allowed-values - all asserted `allow`+`validate`
      coexisting, now structurally unrepresentable). Remaining:
      virtual-properties (last of the 11 definitions files).

## Not started
- [ ] Migrate field defs in `tests/options/*.ts` to builder pattern (the
      `options` argument itself stays a plain object, per backlog item below)
- [ ] Migrate `tests/samples/*` and remaining test files
- [ ] Final full regression pass

## Backlog (deferred)

- [ ] Schema options builder pattern - not doing this for now. Keep the
      `Schema` constructor's second argument (`equalityDepth`, `onDelete`,
      `onSuccess`, `postValidate`, `ignore`, `ignoreUpdate`, `required`,
      `sanitizeError`, `timestamps`) as a plain object literal everywhere.
      Revisit only if explicitly requested later.
