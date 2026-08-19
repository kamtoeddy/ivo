# ivo docs site

Source for the `ivo` documentation website — covers both the TypeScript (`/ts`) and Rust (`/rs`)
implementations from a single Docusaurus site.

See [`TODO.md`](./TODO.md) for the phased build roadmap.

## Why Docusaurus

Three requirements are hard to satisfy without framework support: versioned docs, i18n (English +
French at launch), and light/dark/auto theming. Docusaurus ships built-in support for all three,
which avoids hand-rolling that infrastructure on top of a more minimal framework.

## Structure

Two separate `@docusaurus/plugin-content-docs` instances live under one site/theme/nav:

- **`ts` (versioned)** — `docs-ts/` (current/next = v2.0.0, unreleased),
  `ts_versioned_docs/version-1.9.0/`, `ts_versioned_sidebars/`, `ts_versions.json`. **Scope
  decision:** the site only documents the latest released TS version (v1.9.0) plus v2.0.0, not
  the full v0.0.1–v1.8.0 history — `ts/docs/` remains the historical record for those. See
  `TODO.md`'s "Known gaps" for why.
- **`rs` (unversioned)** — `docs-rs/`. Rust only ever documents the latest API; no version
  history to preserve. Content mirrors the TS docs' page layout (index, life-cycles, validators,
  definitions/{constants,dependents,lax,required,virtual}) for parity between the two languages,
  sourced from `rs/README.md` + `rs/examples/*.rs`, with a link out to rustdoc/docs.rs for the
  exhaustive generated API reference.

```
docs/
  docusaurus.config.ts
  sidebars.ts
  package.json                # bun-managed
  src/
    css/custom.css            # Infima overrides, light/dark tokens
    components/
      TsPlayground/           # Sandpack-based live TS/JS editor
      RustPlayground/         # WASM-demo-based interactive Rust component
    pages/index.tsx           # landing page
  docs-ts/
  ts_versioned_docs/version-{0.0.1..1.9.0}/
  ts_versioned_sidebars/
  ts_versions.json
  docs-rs/
  i18n/fr/...                 # French: site chrome, rs docs, latest ts version
  wasm/ivo-playground/        # crate wrapping rs/, wasm-bindgen exports for the demo playground
  scripts/
    import-ts-docs.mjs        # ts/docs/vX.Y.Z -> ts_versioned_docs
    build-rust-wasm.sh        # wasm-pack build + copy artifact into static/
```

## Playgrounds

- **TypeScript**: `@codesandbox/sandpack-react`, resolving `ivo@<version>` from npm per docs page
  so each historical version's playground runs the real code from that release. Falls back to a
  locally-built ESM bundle for versions not yet published to npm (e.g. v2.0.0 pre-release).
- **Rust**: arbitrary in-browser Rust execution isn't practical without a sandboxed compile
  backend, which is unnecessary infra/security burden for a docs site. Instead, `ivo`'s Rust crate
  is compiled to `wasm32-unknown-unknown` (`wasm/ivo-playground/`) exposing one function per
  curated demo (mirroring `rs/examples/*.rs`). Visitors edit the JSON input to a fixed demo and
  see live validation output — no backend, no code-exec surface. New files added to
  `rs/examples/` need a matching wasm export to appear in the playground.

## Development

```bash
bun install
bun run dev            # English dev server (default locale)
bun run dev:fr         # French dev server
bun run build          # Production build for all locales
bun run serve          # Serve the built site locally
bun run typecheck      # TypeScript check
bun run test:e2e       # Playwright e2e tests against the built site
```

### i18n in dev mode

Docusaurus only builds **one locale at a time** in dev mode. `bun run dev` serves the default
(English) locale, so switching to French from the locale dropdown will 404 — this is expected and
does not affect the production build. Use `bun run dev:fr` to develop against the French locale, or
use `bun run build && bun run serve` to test the full multi-locale site locally.

## Content sourcing rules

- The v1.9.0 versioned doc is imported (not hand-copied) from `ts/docs/v1.9.0/` via
  `scripts/import-ts-docs.mjs`, which also injects frontmatter titles (the source markdown has
  none) and is safe to re-run any time that upstream content changes.
- v2.0.0 docs are hand-authored against the final builder-pattern API once `ts/TODO.md`'s
  migration is complete — not generated.
