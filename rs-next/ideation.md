I have one clarifying question before I begin:

**Should the script overwrite/update `rs-next/benches/RESULTS.md` as well, or only create `/scripts/benches/RESULTS.md`?** You explicitly mentioned the latter, but I want to confirm since the plan originally discussed `rs-next/benches/RESULTS.md`.

Assuming you want only `/scripts/benches/RESULTS.md`, here is how I’ll proceed:

1. Create `/scripts/benches/`.
2. Write a Python script that:
   - Runs `cargo bench` in `/rs` and `/rs-next`.
   - Parses Criterion’s JSON outputs under `target/criterion/`.
   - Generates a fresh side-by-side `RESULTS.md`.
3. Run the script.
4. Report the key findings.

If that sounds right, I’ll start. If you also want `rs-next/benches/RESULTS.md` updated, let me know.
