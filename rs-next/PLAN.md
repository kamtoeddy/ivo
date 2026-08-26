# Ivo Rust v2 Implementation Plan

This document outlines the implementation of the `#[ivo_schema]` macro-driven API defined in [`GOAL.md`](./GOAL.md).

## Guiding principles

1. **Start small and incrementally add field types/options.** Implement `#[required]` and `#[lax]` first, then constants, dependents, virtuals, timestamps, and grouped options.
2. **Preserve existing semantics.** The current `rs/src` runtime is the reference. Generated code should behave identically unless GOAL.md explicitly changes a behavior.
3. **Generate specialized code per schema.** Omit pipeline phases and runtime branches that the schema does not need.
4. **Keep the existing `IvoStruct` / `IvoInputStruct` derives.** `#[ivo_schema]` translates new passthrough attributes into `#[ivo(...)]` and applies the existing derives.
5. **Maintain a separate `rs-next` workspace.** Do not break the existing `rs` crate during v2 development.

## Crate structure

Create a new Cargo workspace under `rs-next/`:

```text
rs-next/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── ivo/                      # v2 runtime library
│   │   └── Cargo.toml            # depends on ivo-derive
│   └── ivo-derive/               # v2 proc-macro crate
│       └── Cargo.toml            # #[ivo_schema], #[derive(IvoStruct)], #[derive(IvoInputStruct)]
└── tests/
```

Decisions:

- Port the existing `rs/crates/derive` logic for `IvoStruct` / `IvoInputStruct` into `rs-next/crates/ivo-derive`, then extend it with `#[ivo_schema]`.
- Reuse the existing `rs/crates/validators` crate by path dependency if it does not depend on `rs`-specific internals; otherwise port it.
- Runtime types (`IvoContext`, `IvoErrorSanitizer`, etc.) live in `rs-next/crates/ivo/src`.

## Phase 1: Bootstrap

1. Create workspace `Cargo.toml`.
2. Create `ivo-derive` proc-macro crate with dependencies: `proc-macro2`, `quote`, `syn` (`full` feature).
3. Create `ivo` library crate depending on `ivo-derive`.
4. Port minimal trait definitions (`IvoStruct`, `IvoInputStruct`, `IvoErrorSanitizer`) and context/options types needed to compile generated code.
5. Set up a basic test crate that compiles a minimal `#[ivo_schema]`.

## Phase 2: Schema parser

Implement the `#[ivo_schema]` attribute macro.

### 2.1 Top-level macro arguments

Parse:

```rust
#[ivo_schema(
    input(User, derive(Debug), derive_partial(Deserialize)),
    output(User, derive(Debug), derive_partial(Serialize)),
    ctx_options(UserCtxOptions),
    error_sanitizer(ErrorSanitizer),
)]
```

- `input(...)` is required; extract struct name, `derive(...)`, `derive_partial(...)`.
- `output(...)` is optional; same extraction.
- `ctx_options(Type)` and `error_sanitizer(Type)` are optional.

### 2.2 Module body parsing

Find by module name (not attribute):

- `mod fields { ... }` — required.
- `mod options { ... }` — optional.

### 2.3 Field parsing

For each field in `mod fields`, parse:

- Field-type attribute: `#[required]`, `#[lax(default)]`, `#[constant(value)]`, `#[dependent]`, `#[virtual(alias = "...")]`, `#[created_at]`, `#[updated_at]`.
- Visibility keyword.
- Field name and type.
- Behavior attributes: `#[validate(...)]`, `#[re_validate(...)]`, `#[sanitize(...)]`, `#[resolve(...)]`, `#[default(...)]`, `#[value(...)]`, `#[depends_on(...)]`, `#[readonly]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[required_error(...)]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`.
- Passthrough attributes: `#[input(...)]`, `#[output(...)]`, `#[partial(...)]`, `#[input_partial(...)]`, `#[output_partial(...)]`.

Store each field as a typed `FieldDef` enum variant.

### 2.4 Options parsing

For each `const _: () = ()` item in `mod options`, parse attached attributes:

- `#[ignore([...], |ctx, opts| ...)]`
- `#[ignore_update([...], |partial, full, opts| ...)]`
- `#[required([...], |ctx, opts| ...)]`
- `#[post_validate([...], |b| b.validate(...).pre_validate(...))]`
- `#[on_success([...], |b| b.handle(...))]`
- `#[on_delete(|data, opts| ...)]`
- `#[timestamps(|| ...)]`

### 2.5 Syntax classification

For each handler closure/function item, classify as sync or async:

- `|...| expr` → sync.
- `|...| async move { ... }` / `|...| async { ... }` → async.
- `fn_name` → sync.
- `async fn_name` → async.

## Phase 3: Schema validation

Build a `Schema` struct and enforce all compile-time rules from GOAL.md.

1. **Field name uniqueness.** No duplicate field names or aliases.
2. **Attribute whitelist.** Each field type only accepts its allowed attributes.
3. **Attribute prerequisites.**
   - `#[re_validate]` requires `#[validate]`.
   - `#[readonly]` on required requires `#[validate]`.
   - `#[readonly]` on lax/dependent requires a static default.
4. **Repetition rules.** Only `#[on_delete]`, `#[on_success]`, `#[on_failure]` may repeat on a field.
5. **Dependency graph rules.**
   - Every dependent has ≥1 parent.
   - Parents cannot be constants or timestamps.
   - No circular dependencies.
   - No redundant/transitive dependencies.
   - Virtual fields must be referenced by at least one dependent.
   - Alias uniqueness and validity.
6. **Single/dual struct mode.**
   - If only `#[required]` / `#[lax]` fields and no timestamps → single-struct mode; `output(...)` must not be provided.
   - If any `#[constant]`, `#[dependent]`, `#[virtual]`, `#[created_at]`, `#[updated_at]` → dual-struct mode; `output(...)` is required.
7. **Grouped option validation.**
   - Cardinality rules (≥2, 0 or ≥2, etc.).
   - Field-type restrictions per option.
   - No aliases in field arrays.
   - No duplicate field names within one option.
8. **Timestamp rules.**
   - At most one `#[created_at]` and one `#[updated_at]`.
   - Timestamp resolver must exist if any timestamp field is declared.
   - Resolver is synchronous.

## Phase 4: Struct generation

Generate the input and output structs.

1. Place fields on input/output according to field type (GOAL.md §11).
2. Apply user-provided derives from top-level `input(...)` / `output(...)` plus `#[derive(IvoInputStruct)]` / `#[derive(IvoStruct)]`.
3. Preserve field visibility exactly as written.
4. Translate passthrough attributes:
   - `#[input(...)]` → emitted on input struct field.
   - `#[output(...)]` → emitted on output struct field.
   - `#[partial(...)]` / `#[input_partial(...)]` / `#[output_partial(...)]` → translated to `#[ivo(...)]` on the appropriate generated struct field.
5. In single-struct mode, derive both `IvoInputStruct` and `IvoStruct` on the same struct; avoid duplicate impls.
6. Generate the schema model struct (e.g., `UserSchemaModel`) with visibility matching the output struct.

## Phase 5: Schema model generation

Generate `create` and `update` methods on the schema model. Generate `delete` only if the schema declares at least one field-level or grouped `on_delete` handler.

### 5.1 Dynamic pipeline

Generate only the phases needed for the schema:

- Field filtering (`ignore`, `ignore_init`, `ignore_update`, `readonly`).
- Default attachment (`lax`, `dependent`).
- Missing-required evaluation.
- Validation / re-validation.
- Post-validation.
- Virtual sanitization.
- Dependent resolution loop.
- Constant attachment.
- Timestamp attachment.
- Update validity evaluation (strip unchanged fields).
- Success/failure trigger preparation.

### 5.2 Sync/async method signatures

- Determine if `create`/`update` are sync or async based on directly-invoked handlers.
- Generate `delete` only when at least one `on_delete` handler exists; determine if it is sync or async based on those handlers.
- Determine trigger sync/async based on `on_success` / `on_failure` handlers.
- Expose `handle_success` on `IvoSuccessHandle` only when `on_success` handlers exist; otherwise omit the method.
- Expose `handle_failure` on `IvoFailureHandle` only when `on_failure` handlers exist; otherwise omit the method.

### 5.3 Handler triggers

Return `impl FnOnce()` triggers:

- Sync trigger returns `()`.
- Async trigger returns `impl Future<Output = ()>`.
- No-op trigger when no handlers exist.
- `IvoSuccessHandle` only provides `handle_success` when the schema has `on_success` handlers.
- `IvoFailureHandle` only provides `handle_failure` when the schema has `on_failure` handlers.

### 5.4 Return types

- `create`: `Result<(Output, SuccessTrigger, CtxOptions), (ErrorSanitizer::Payload, FailureTrigger, CtxOptions)>`.
- `update`: `Result<(PartialOutput, SuccessTrigger, CtxOptions), (Option<ErrorSanitizer::Payload>, FailureTrigger, CtxOptions)>`.
- `delete`: generated only when `on_delete` handlers exist; returns `()` and is async if any `on_delete` handler is async.

## Phase 6: Runtime support

Implement or port runtime types consumed by generated code.

1. **Contexts**
   - `IvoContext<I, O>` with accessors: `input`, `raw_input`, `values`, `changes`, `full_values`, `previous_values`, `is_update`.
   - `IvoDefaultCtx<I>` for lax/dependent defaults.
   - `IvoConstantCtx<I, O>` for constants.
2. **Options wrappers**
   - `IvoRwCtxOptions<CtxOptions>` (`Arc<RwLock<CtxOptions>>`) for core handlers.
   - `IvoCtxOptions<CtxOptions>` (`Arc<CtxOptions>`) for lifecycle hooks.
3. **Error handling**
   - `IvoErrorSanitizer` trait with `Metadata` and `Payload` associated types.
   - `IvoErrorPayload`, `FieldError`, default sanitizer.
4. **Partial struct methods**
   - `new`, `is_empty`, `into_option`, `set_*`, `with_*`, `unset_*`.
   - `IvoStruct::append_updates`.

## Phase 7: Testing strategy

1. **Compile-time tests.** Ensure invalid schemas produce `compile_error!` with clear messages.
2. **Semantic parity tests.** Port the existing `rs/tests` and `rs/examples` to the new API and assert identical behavior.
3. **Feature-specific tests.** One test file per field type and per grouped option.
4. **Sync/async tests.** Verify sync-only schemas generate sync methods; async handlers generate async methods.
5. **Performance tests.** Port `rs/benches` and compare against the optimized baseline in `rs/benches/RESULTS.md`.

## Phase 8: Incremental delivery order

1. Minimal required/lax schemas with `create`.
2. Add `update` and `delete`.
3. Add `#[constant]`.
4. Add `#[dependent]` with dependency resolution.
5. Add `#[virtual]` and sanitization.
6. Add timestamps.
7. Add grouped options one at a time: `ignore`, `ignore_update`, `required`, `post_validate`, `on_success`, `on_delete`.
8. Add passthrough attributes and derive mapping.
9. Optimize generated code and run benchmarks.

## Open implementation decisions

1. Should `#[ivo_schema]` live in the existing `ivo-derive` crate or a new crate?
2. Should `rs-next/crates/ivo` re-export types from `rs` or be a clean rewrite?
3. Should generated code use named future types or `impl Future` to avoid unstable features?

These should be resolved in Phase 1 before writing significant parser code.
