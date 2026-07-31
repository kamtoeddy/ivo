# Docs website - TODO

Build plan for the `ivo` documentation website (this `/docs` directory): a single Docusaurus
site covering both the TypeScript (`/ts`, v1.9.0 + v2.0.0 only - see scope decision below) and
Rust (`/rs`, latest API only) implementations, with light/dark/auto theming, interactive
playgrounds for both languages, and English + French i18n. See [`README.md`](./README.md) for the
architecture rationale and directory layout.

## Phase 0 - Scaffold

- [x] `bun create docusaurus` (TypeScript template) into `/docs`
- [x] Strip default blog plugin/content (not needed)
- [x] Configure two `@docusaurus/plugin-content-docs` instances: `id: 'ts'` (versioned,
      `path: 'docs-ts'`, `routeBasePath: 'docs/ts'`) and `id: 'rs'` (unversioned,
      `path: 'docs-rs'`, `routeBasePath: 'docs/rs'`)
- [x] Basic nav bar (TypeScript / Rust doc links, version dropdown, locale dropdown, GitHub),
      footer
- [x] Verify `bun run build` produces a working site (`bun run start` for local dev)

## Phase 1 - Content migration

- [x] Write `scripts/import-ts-docs.mjs`: copies `ts/docs/v1.9.0` into
      `ts_versioned_docs/version-1.9.0/`, generates `ts_versions.json` + version sidebar
- [x] **Scope decision (changed from the original plan):** the site only documents TS v1.9.0
      (latest released) and v2.0.0 (unreleased, this branch) - not the full v0.0.1-v1.8.0 history.
      `ts/docs/` remains the historical record for those older versions. This makes the v1.8.0
      empty-folder gap and the v0.0.1 `validators/`-directory-vs-`validators.md` inconsistency
      moot for the site (neither v0.0.1 nor v1.8.0 are imported).
- [x] Verify all imported internal links resolve after import - found and fixed real pre-existing
      bugs in `ts/docs/v1.9.0/*.md` in the process (see "Fixed while migrating" below)
- [x] Author `docs-rs/` content from `rs/README.md`, structured to mirror the TS docs' page
      layout (index, life-cycles, validators, definitions/{constants,dependents,lax,required,virtuals})
      for parity between the two languages
- [ ] Link out to rustdoc/docs.rs from `docs-rs/` for the exhaustive generated API reference
      (types/functions) rather than duplicating it by hand (docs.rs publish not set up yet)
- [x] Cross-link `rs/examples/*.rs` and `rs/tests/**` from the relevant `docs-rs/` pages (as GitHub
      blob URLs - files outside `/docs` aren't part of the Docusaurus content graph)

### Fixed while migrating (pre-existing bugs in `ts/docs/v1.9.0/*.md`, not import-script bugs)

- `index.md` linked to a nonexistent `../v3.4.0/validate/index.md` and a nonexistent
  `../../tests/schema/samples/custom-error-tool/index.ts` - repointed to `./validators.md` and the
  real file (`ts/tests/extras/error-sanitizer.ts`, linked via GitHub since it's outside `/docs`).
- `validators.md` linked to `../definitions/*.md` (one level too high) instead of
  `./definitions/*.md`.
- Several `[text](file.md#heading)` links targeted a heading that was that page's sole/first `#`
  heading - Docusaurus never assigns anchor ids to `h1`, so these resolved to nothing. Fixed by
  either linking to the page itself (no fragment) or, where a page legitimately has multiple
  top-level sections (`index.md`, `life-cycles.md`), demoting the later ones from `#` to `##`
  (and their children down a level) so they become real, addressable sections.
- `extend-schemas.md` linked `#shouldupdate` but the actual heading is
  `## shouldUpdate (default: true)`, which slugs to `#shouldupdate-default-true`.
- `definitions/virtuals.md` linked `#the-operation-ctx`; the real anchor is
  `#the-operation-context`.
- `definitions/constants.md` was linked with a stray trailing backtick in the anchor
  (`#constant-properties\``).

## Phase 2 - Theme & responsiveness

- [x] Light/dark/auto mode works out of the box via Docusaurus classic theme
      (`colorMode: { respectPrefersColorScheme: true }`); toggle confirmed present in built HTML
- [x] Infima variable overrides in `src/css/custom.css` - indigo brand palette (light + dark
      tints), replacing the default Docusaurus green
- [x] Nav drawer / sidebar collapse are Infima defaults (responsive out of the box); the
      hand-written `QuickLinks` table relies on Infima's global `table { display: block; overflow:
      auto }` for horizontal scroll on narrow viewports (verified in built CSS) - no wrapper needed
- [ ] Playground component stacking (editor above output) at mobile widths - revisit once Phase
      3/4 components exist
- [x] Landing page (`src/pages/index.tsx`): real hero copy, 3-item feature grid
      (`HomepageFeatures`), and a `QuickLinks` component mirroring the root `README.md`'s "Quick
      links" table (TS/Rust docs, main demo, examples) - verified rendered links resolve
- [ ] Brand assets are still Docusaurus placeholders (`static/img/logo.svg`, `favicon.ico`,
      `docusaurus-social-card.jpg`) - no image-generation tooling available this session; needs a
      real `ivo` logo/favicon/social-card pass

## Phase 3 - TypeScript playground

- [x] Add `@codesandbox/sandpack-react` dependency
- [x] `src/components/TsPlayground/` component (`Sandpack`, `theme="auto"`, wrapped in
      `BrowserOnly`): resolves `ivo@<version>` from npm per docs page. Confirmed `ivo` is
      published (`registry.npmjs.org/ivo`, `dist-tags.latest` = `1.9.0`)
- [x] Registered globally for MDX via `src/theme/MDXComponents.tsx`, so docs pages use
      `<TsPlayground />` with no per-file import
- [x] Embedded in v1.9.0's "Defining a schema" page: `scripts/import-ts-docs.mjs` appends a `## Try
      it in the browser` section with a runnable schema example to the *imported copy* of
      `index.md` only (not the GitHub-facing source in `ts/docs/`, where raw JSX would render
      oddly as plain markdown). Build succeeds; `BrowserOnly` fallback confirmed present in the
      built HTML
- [ ] Not yet verified in an actual browser (no browser-automation tool available this session) -
      confirm the Sandpack iframe loads and the example runs before calling this done
- [ ] v2.0.0 (`docs-ts/index.md`) still a placeholder - deferred to Phase 6 (no real API to demo
      yet); add its own `<TsPlayground ivoVersion="2.0.0" .../>` once that content is authored
- [ ] Fallback path for v2.0.0 once authored but not yet published to npm: load a locally-built
      ESM bundle instead of resolving from the registry

## Phase 4 - Rust playground

- [x] New crate `docs/wasm/ivo-playground/` (own `Cargo.toml`, path-deps on `../../../rs`, not a
      member of `rs/`'s workspace) with `wasm-bindgen` async exports for 3 curated demos:
      `constantsCreate`, `laxDefaultsCreate`, `requiredCreate` - each mirroring its
      `rs/examples/*.rs` counterpart's field config exactly (structure copied from the real
      example, handlers/prints dropped since the playground only needs create-and-show-result)
- [x] `scripts/build-rust-wasm.sh`: checks for `wasm-pack` + `wasm32-unknown-unknown` target,
      `wasm-pack build --target web --out-dir ../../static/wasm/ivo-playground`. Re-run confirmed
      fast/idempotent
- [x] `src/components/RustPlayground/` MDX component: `BrowserOnly` + dynamic `import()` of the
      built JS glue, JSON `<textarea>` input + "Run" button + output pane, responsive (stacks on
      ≤768px, matching the Phase 2 note), registered globally via `src/theme/MDXComponents.tsx`
- [x] Embedded `<RustPlayground demo="..." />` in `docs-rs/definitions/{constants,lax,required}.md`
- [x] **Verified end-to-end, not just "builds"**: ran the built wasm module directly in Node
      (bypassing the browser) against all 3 demos - output matches the original examples' own
      assertions exactly, including the literal required-field error message
      (`"username" is required!`) from `rs/examples/required.rs`. Also confirmed webpack correctly
      rehashes the `.wasm` binary reference in the production bundle (`grep` for the emitted
      hashed filename in the built JS chunk) and serves it with `content-type: application/wasm`
- [ ] Not yet click-tested in an actual browser (no browser-automation tool available this
      session) - the Node-level and bundle-level checks above are strong signals but aren't a
      substitute for seeing it run
- [ ] `virtuals`, `dependents`, `timestamps` demos not yet implemented (only 3 of the 6 originally
      scoped demos - `constants`, `lax_defaults`, `required` - to prove the pipeline first;
      `dependents`/`virtuals` need the `dependsOn`/alias wiring which is more involved)
- [x] Documented the maintenance convention (new `rs/examples/*.rs` needs a matching wasm export)
      in both this file and the `DEMOS` map comment in `RustPlayground/index.tsx`

## Phase 5 - i18n (English + French)

- [x] Enable Docusaurus i18n: `defaultLocale: 'en'`, `locales: ['en', 'fr']`
- [x] Locale switcher in the nav bar
- [x] `docusaurus write-translations --locale fr` for site chrome, then translated
      `navbar.json`/`footer.json`/version labels/sidebar category labels, plus swept `code.json`
      for theme strings Docusaurus's own French locale data left untranslated (found 3: the
      "system mode" toggle label, and the mobile nav dropdown's expand/collapse aria-labels - all
      user-visible, all fixed)
- [x] Translated the landing page: converted `src/pages/index.tsx`, `HomepageFeatures` and
      `QuickLinks` to use `@docusaurus/Translate`/`translate()` (not hardcoded strings), then
      filled in the 17 generated `homepage.*` keys in `i18n/fr/code.json`. Verified both locales
      render correctly in the built HTML (`grep` for French vs. English homepage text)
- [x] `docs-rs/` translated into French (all 8 pages:
      `i18n/fr/docusaurus-plugin-content-docs-rs/current/**`). Build's own broken-anchor checker
      caught one cross-reference that needed updating for the translated heading slug
      (`#custom-context-options` → `#options-de-contexte-personnalisées`) - same technique used in
      Phase 1. All 8 French URLs verified 200 via a served build
- [ ] `docs-ts/` (v2.0.0 placeholder) and `ts_versioned_docs/version-1.9.0/` not yet translated -
      v1.9.0 alone is ~1500 lines across 11 files; deliberately not rushing a full technical
      translation of that volume in one pass (mistranslating a validation-rule nuance is worse
      than leaving it English-with-a-fallback for now). Real next step, not abandoned.

## Phase 6 - v2.0.0 content authoring

- [ ] Blocked on: `ts/TODO.md` builder-pattern migration finishing (definitions test migration +
      remaining phases)
- [ ] Author `docs-ts/` ("current"/next version, currently a placeholder page) against the final
      builder API surface (`ts/src/schema/fields/{lax,required,virtual}.ts`, `constants.ts`,
      `dependents.ts`)
- [ ] On v2.0.0 release: cut `docs-ts/` as the `version-2.0.0` versioned snapshot, update
      `lastVersion` in `docusaurus.config.ts` from `'1.9.0'` to `'2.0.0'`

## Phase 7 - CI/CD (Cloudflare Pages)

- [x] `.github/workflows/docs-deploy.yml`: triggers on PRs (build-check only, needs no secrets)
      and pushes to `main` (build + deploy), scoped to `docs/**` + `ts/docs/**` + `rs/**` paths.
      Runs `scripts/import-ts-docs.mjs` + `scripts/build-rust-wasm.sh` (via `dtolnay/rust-toolchain`
      with the `wasm32-unknown-unknown` target + `jetli/wasm-pack-action`, matching this session's
      verified local toolchain), then `bun install && bun run build`
- [x] Deploy step wired via `cloudflare/pages-action@v1`, `directory: docs/build`,
      `projectName: ivo-docs`, gated to `push` + `refs/heads/main` only
- [x] Kept fully separate from the existing `rs-ci.yml` / `ts-*-ci.yml` workflows (own file, own
      path filters)
- [ ] **Blocked on user action** (external accounts, can't be done by the agent): create a
      Cloudflare Pages project named `ivo-docs`, add `CLOUDFLARE_API_TOKEN` +
      `CLOUDFLARE_ACCOUNT_ID` as GitHub Actions repo secrets, and update `url` in
      `docusaurus.config.ts` from the `ivo.pages.dev` placeholder to the real assigned domain once
      known
- [ ] Not yet run in actual GitHub Actions (no CI access this session) - the individual commands
      (`node scripts/import-ts-docs.mjs`, `bash scripts/build-rust-wasm.sh`, `bun run build`) were
      all verified locally in this session, but the workflow YAML itself is unexercised

## Known gaps / explicit scope decisions

- TS docs cover v1.9.0 + v2.0.0 only, not the full v0.0.1-v1.8.0 history (explicit scope decision
  made mid-build; `ts/docs/` remains the historical record for older versions).
- Full French translation is out of scope for launch beyond site chrome + Rust docs + the TS
  content that exists; ships incrementally as Phase 5/6 land.
- Rust playground supports curated demos with editable JSON input, not arbitrary Rust source -
  running arbitrary Rust in-browser would require a sandboxed compile backend, rejected as
  unnecessary infra/security burden for a docs site.
- v2.0.0 docs cannot be authored until the builder-pattern API in `ts/TODO.md` is finalized.
