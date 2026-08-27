Here is the plan for porting the `/rs/benches` setup to `/rs-next/` and comparing the two implementations.

## Proposed plan

1. **Add benchmark infrastructure to `rs-next/`**
   - Add `criterion = { version = "0.5", features = ["async_tokio"] }` to `rs-next/Cargo.toml` dev-dependencies.
   - Add `[[bench]]` entries for `throughput` and `memory`, both with `harness = false`.

2. **Recreate the benchmark schemas in `rs-next/benches/schemas.rs`**
   - Port each old schema to the new `#[ivo_schema]` macro API:
     - `Minimal`
     - `User` (lax `id` + required `name`/`email`/`age`)
     - `ManyField20` (20 required fields)
     - `Chain10` (10-field dependent chain)
     - `Readonly10` (10 readonly lax fields)
   - Use async validators/resolvers so the benchmark call shape matches the old async `IvoModel` usage (`.create(...).await`, `.update(...).await`).

3. **Recreate the benchmark runners**
   - `rs-next/benches/throughput.rs` — same benchmark names and workloads as `rs/benches/throughput.rs`.
   - `rs-next/benches/memory.rs` — same x1000 iteration workloads as `rs/benches/memory.rs`.

4. **(Optional) Align release-profile tuning**
   - `rs/Cargo.toml` already has `[profile.release] lto = true` and `codegen-units = 1`.
   - For a fair comparison I would add the same profile to `rs-next/Cargo.toml`.

5. **Run both benchmark suites**
   - Run `cargo bench` in `rs/` and `rs-next/` on the same machine in the same session.
   - Capture Criterion’s mean-time/throughput output for each benchmark.

6. **Document the comparison**
   - Create `rs-next/benches/RESULTS.md` with side-by-side tables and percentage differences.

## Questions before I proceed

1. **May I add `[profile.release] lto = true / codegen-units = 1` to `rs-next/Cargo.toml`?**  
   The old implementation already has this tuning, so without it the comparison would be unfair.

2. **Should I re-run the old `rs` benchmarks now, or compare against the existing `rs/benches/RESULTS.md` numbers?**  
   Fresh same-machine numbers are more accurate, but re-running both suites will take a few minutes.

3. **Which Tokio runtime should the new benches use?**  
   The old code uses `tokio::runtime::Runtime::new()` (multi-thread). The old `README.md` says “Tokio current-thread runtime.” I’ll match the old code exactly unless you want current-thread.

4. **Any extra schemas or metrics you want included?**  
   Otherwise I’ll stick to the existing throughput + memory set.

Please let me know your preferences, and I’ll start implementing.
