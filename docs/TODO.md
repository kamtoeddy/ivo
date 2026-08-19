# Docs website - TODO

Build plan for the `ivo` documentation website (this `/docs` directory): a single Docusaurus
site covering both the TypeScript (`/ts`, v1.9.0 + v2.0.0 only - see scope decision below) and Rust
(`/rs`, latest API only) implementations, with light/dark/auto theming, interactive playgrounds for
both languages, and English + French i18n. See [`README.md`](./README.md) for the architecture
rationale and directory layout.

## Phase 0 - Scaffold

- [x] `bun create docusaurus` (TypeScript template) into `/docs`
- [x] Strip default blog plugin/content (not needed)
- [x] Configure two `@docusaurus/plugin-content-docs` instances: `id: 'ts'` (versioned,
      `path: 'docs-ts'`, `routeBasePath: 'docs/ts'`) and `id: 'rs'` (unversioned,
      `path: 'docs-rs'`, `routeBasePath: 'docs/rs`)
- [x] Basic nav bar (TypeScript / Rust doc links, version dropdown, locale dropdown, GitHub),
      footer
- [x] Verify `bun run build` produces a working site (`bun run dev` for local dev)

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
- [x] Link out to rustdoc/docs.rs from `docs-rs/` for the exhaustive generated API reference
      (types/functions) rather than duplicating it by hand. Added an "API reference" section on
      `docs-rs/index.md` (and its French translation) linking to docs.rs plus local `cargo doc`
      instructions, since `ivo` has not been published to crates.io yet.
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
- [x] Playground component stacking (editor above output) at mobile widths - `RustPlayground` CSS
      already stacks panes vertically at `max-width: 768px`; verified after adding the new Phase 4
      demos.
- [x] Landing page (`src/pages/index.tsx`): real hero copy, 3-item feature grid
      (`HomepageFeatures`), and a `QuickLinks` component mirroring the root `README.md`'s "Quick
      links" table (TS/Rust docs, main demo, examples) - verified rendered links resolve
- [x] Brand assets: replaced Docusaurus placeholders with `ivo`-branded `logo.svg/png`, generated
      favicon/touch icon sizes via `scripts/generate-brand-assets.mjs`, and a `social-card.png`.
      Updated `docusaurus.config.ts` to use the new assets and `headTags` for icons / webmanifest.

## Phase 3 - TypeScript playground

- [x] Add `@codesandbox/sandpack-react` dependency
- [x] `src/components/TsPlayground/` component (`Sandpack`, `theme="auto"`, wrapped in
      `BrowserOnly`): resolves `ivo@<version>` from npm per docs page. Confirmed `ivo` is
      published (`registry.npmjs.org/ivo`, `dist-tags.latest` = `1.9.0`)
- [x] Registered globally for MDX via `src/theme/MDXComponents.tsx`, so docs pages use
      `<TsPlayground />` with no per-file import
- [x] Embedded in v1.9.0's "Defining a schema" page: `scripts/import-ts-docs.mjs` appends a `## Try
it in the browser` section with a runnable schema example to the _imported copy_ of
      `index.md` only (not the GitHub-facing source in `ts/docs/`, where raw JSX would render
      oddly as plain markdown). Build succeeds; `BrowserOnly` fallback confirmed present in the
      built HTML
- [x] v2.0.0 (`docs-ts/`) content authored and wrapped in `<TsPlayground ivoVersion="local">`,
      using a locally-built ESM bundle (`docs/static/ivo-2.0.0/`) since v2.0.0 is not yet on npm.
- [x] Playground rendering verified end-to-end via Playwright e2e tests (iframe/Sandpack wrapper
      appears and loads).
- [ ] Not yet verified that every edited playground example runs without runtime errors - e2e
      currently checks presence, not console output. Add functional output assertions if needed.

## Phase 4 - Rust playground

- [x] New crate `docs/wasm/ivo-playground/` (own `Cargo.toml`, path-deps on `../../../rs`, not a
      member of `rs/`'s workspace) with `wasm-bindgen` async exports for 6 curated demos:
      `constantsCreate`, `laxDefaultsCreate`, `requiredCreate`, `virtualsCreate`,
      `dependentsCreate`, `timestampsCreate` - each mirroring its `rs/examples/*.rs` counterpart's
      field config exactly (structure copied from the real example, handlers/prints dropped since
      the playground only needs create-and-show-result)
- [x] `scripts/build-rust-wasm.sh`: checks for `wasm-pack` + `wasm32-unknown-unknown` target,
      `wasm-pack build --target web --out-dir ../../static/wasm/ivo-playground`. Re-run confirmed
      fast/idempotent
- [x] `src/components/RustPlayground/` MDX component: `BrowserOnly` + dynamic `import()` of the
      built JS glue, JSON `<textarea>` input + "Run" button + output pane, responsive (stacks on
      ≤768px, matching the Phase 2 note), registered globally via `src/theme/MDXComponents.tsx`
- [x] Embedded `<RustPlayground demo="..." />` in `docs-rs/definitions/{constants,dependents,lax,required,timestamps,virtuals}.md`
- [x] **Verified end-to-end, not just "builds"**: ran the built wasm module directly in Node
      (bypassing the browser) against all 6 demos - output matches the original examples' own
      assertions exactly, including the literal required-field error message
      (`"username" is required!`) from `rs/examples/required.rs`. Also confirmed webpack correctly
      rehashes the `.wasm` binary reference in the production bundle (`grep` for the emitted
      hashed filename in the built JS chunk) and serves it with `content-type: application/wasm`
- [x] Playground rendering and interactivity verified via Playwright e2e tests (click Run and check
      output).
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
- [x] `docs-rs/` translated into French (all pages).
- [x] `docs-ts/` (v2.0.0) translated into French.
- [ ] `ts_versioned_docs/version-1.9.0/` not yet fully translated into French - copied English
      content into `i18n/fr/docusaurus-plugin-content-docs-ts/version-1.9.0/` as a fallback so
      routes exist and don't 404. Full translation is the remaining i18n work.

## Phase 6 - v2.0.0 content authoring

- [x] Author `docs-ts/` ("current" version = v2.0.0) against the final builder-pattern API surface
      (`ts/src/schema/fields/{lax,required,virtual}.ts`, `constants.ts`, `dependents.ts`)
- [x] Standardise versioning for release: `lastVersion: "current"` so v2.0.0 is the default landing
      version; v1.9.0 archived at `/docs/ts/1.9.0`.
- [ ] On v2.0.0 release: cut `docs-ts/` as the `version-2.0.0` versioned snapshot with
      `docusaurus docs:version ts 2.0.0`, then reset `docs-ts/` for the next development cycle.

## Phase 7 - CI/CD (Cloudflare Pages)

- [x] `.github/workflows/docs-deploy.yml`: triggers on PRs (build-check only, needs no secrets)
      and pushes to `main` (build + deploy), scoped to `docs/**` + `ts/docs/**` + `rs/**` paths.
      Runs `scripts/import-ts-docs.mjs` + `scripts/build-rust-wasm.sh` (via `dtolnay/rust-toolchain`
      with the `wasm32-unknown-unknown` target + `jetli/wasm-pack-action`, matching this session's
      verified local toolchain), then `bun install && bun run build`
- [x] Added `.github/workflows/docs-e2e.yml` to run Playwright e2e tests against the built site on
      PRs and pushes to `main`.
- [x] Deploy step wired via `cloudflare/pages-action@v1`, `directory: docs/build`,
      `projectName: ivo-docs`, gated to `push` + `refs/heads/main` only
- [ ] **Blocked on user action** (external accounts, can't be done by the agent): create a
      Cloudflare Pages project named `ivo-docs`, add `CLOUDFLARE_API_TOKEN` +
      `CLOUDFLARE_ACCOUNT_ID` as GitHub Actions repo secrets, and update `url` in
      `docusaurus.config.ts` from the `ivo.pages.dev` placeholder to the real assigned domain once
      known
- [ ] Not yet run in actual GitHub Actions (no CI access this session) - the individual commands
      and e2e tests were verified locally, but the workflow YAML itself is unexercised

## Known gaps / explicit scope decisions

- TS docs cover v1.9.0 + v2.0.0 only, not the full v0.0.1-v1.8.0 history (explicit scope decision
  made mid-build; `ts/docs/` remains the historical record for older versions).
- Full French translation of v1.9.0 is out of scope for launch; it currently falls back to English
  content so routes don't 404.
- Rust playground supports curated demos with editable JSON input, not arbitrary Rust source -
  running arbitrary Rust in-browser would require a sandboxed compile backend, rejected as
  unnecessary infra/security burden for a docs site.
