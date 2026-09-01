Here's my recommendation, ranked by what I'd actually do first:

## 1. Measure before touching anything (do this first, ~30 min, low risk)

Rather than `cargo flamegraph` (needs an external tool, gives relative time not allocation counts), a cheaper and more direct answer: wrap the global allocator in a counting shim for one of the existing memory-stress benchmarks and print total allocations + bytes for `main_demo create`/`update`. That tells us in absolute terms whether "9 ctx rebuilds + N per-handler clones" is 50 allocations or 5,000 — which determines whether anything below is worth doing at all. I'd rather have that number than guess.

## 2. If it matters: a surgical fix, not the big redesign

I looked closer at what's actually driving the cost, and there are two separable things bundled into one finding:

- **The per-phase rebuild** (`IvoContext::new(input.clone(), ..., output.clone(), ...)`, 9× per call) — this clone is _real and currently unavoidable_: `input`/`output` are plain mutable locals the generated code assigns fields into directly between phases, and the ctx needs a fresh snapshot reflecting that. Nothing cheap fixes this without a genuine architecture change.
- **The per-handler clone** (`ctx.clone()`/`__round_ctx.clone()` at every resolver/hook call site) — this one _is_ fixable cheaply. `IvoContext` is fully immutable after construction (I checked — no setters, no in-place mutation anywhere, `ctx` bindings only ever get wholesale-reassigned via `ctx = IvoContext::new(...)`, never mutated). That means the several concurrent handlers in a `join!` batch that each currently deep-clone the same frozen snapshot could instead share it.

**My actual suggestion**: change `IvoContext`'s internal storage from owning `I`/`O` directly to owning `Arc<I>`/`Arc<O>` (must be `Arc`, not `Rc` — `I`/`O` already require `Send + Sync` elsewhere, and the benchmarks run on a multi-thread Tokio runtime). This:

- Makes every `ctx.clone()`/`__round_ctx.clone()` call site an atomic refcount bump instead of a deep struct clone, with **zero changes needed at any of those 40+ call sites** in the derive macro — `#[derive(Clone)]` on `IvoContext` just does the right thing automatically once the fields are `Arc`-wrapped.
- Doesn't touch handler signatures, doesn't touch the `join!` concurrency model, doesn't introduce lifetimes anywhere.
- Only changes: `IvoContext`'s field types, `new()`'s body (wrap in `Arc::new(...)`), and the accessor bodies (`&self.values` → `self.values.as_ref()` or similar) — all internal to one `impl` block in `src/internals/types.rs`.
- Leaves the 9-rebuilds-per-call cost as-is (that one genuinely needs the fresh clone, for the reason above).

This is deliberately the smaller, contained fix rather than the "make `IvoContext` borrow instead of own" redesign I flagged earlier — that one would ripple through every handler signature and needs its own scoped plan; this one doesn't.

## 3. Separately, worth a quick audit regardless

Whether or not #2 happens: are all 9 rebuilds per `create`/`update` actually necessary, or does one immediately follow another with no field changing in between (making one of them a wasted clone)? That's orthogonal to #2 and could shrink the count of _real_ clones too, not just make the per-handler ones cheap.

I'd start with #1 and only do #2/#3 if the numbers say it's worth it. Want me to build that allocation-counting benchmark now?
