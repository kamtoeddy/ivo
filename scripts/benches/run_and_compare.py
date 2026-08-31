#!/usr/bin/env python3
"""
Run the /rs and /rs-next Criterion benchmarks and generate a side-by-side
comparison at /scripts/benches/RESULTS.md.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional

ROOT = Path(__file__).resolve().parent.parent.parent
RS_DIR = ROOT / "rs"
RS_NEXT_DIR = ROOT / "rs-next"
RESULTS_PATH = Path(__file__).resolve().parent / "RESULTS.md"

THROUGHPUT_BENCHES: List[str] = [
    "minimal create",
    "user create",
    "create 20 required fields (sync validators)",
    "dependent chain length 10",
    "create 10 readonly lax fields",
    "no-op update",
    "single field update",
]

MEMORY_BENCHES: List[str] = [
    "memory minimal create x1000",
    "memory user create x1000",
    "memory 20 fields create x1000",
    "memory no-op update x1000",
]

MAIN_DEMO_BENCHES: List[str] = [
    "main_demo create [fail: required errors (email or phone_number)]",
    "main_demo create [fail: required errors (email or phone_number, username)]",
    "main_demo create [fail: validation error (email, slug_id, username)]",
    "main_demo create [fail: re_validation error (username taken)]",
    "main_demo create [fail: post-validation error (slug taken)]",
    "main_demo create [success: 2/4 inputs (a)]",
    "main_demo create [success: 2/4 inputs (b)]",
    "main_demo create [success: 3/4 inputs]",
    "main_demo create [success: 4/4 inputs]",
    "main_demo update [fail: required error (email or phone_number)]",
    "main_demo update [fail: validation error (email, slug_id, username)]",
    "main_demo update [fail: re_validation error (username taken)]",
    "main_demo update [fail: post-validation error (slug taken)]",
    "main_demo update [fail: nothing to update: 1/4 inputs (a)]",
    "main_demo update [fail: nothing to update: 1/4 inputs (b)]",
    "main_demo update [fail: nothing to update: 1/4 inputs (c)]",
    "main_demo update [fail: nothing to update: 1/4 inputs (d)]",
    "main_demo update [fail: nothing to update: 2/4 inputs]",
    "main_demo update [fail: nothing to update: 3/4 inputs]",
    "main_demo update [fail: nothing to update: 4/4 inputs]",
    "main_demo update [success: 1/4 inputs (a)]",
    "main_demo update [success: 1/4 inputs (b)]",
    "main_demo update [success: 1/4 inputs (c)]",
    "main_demo update [success: 1/4 inputs (d)]",
    "main_demo update [success: 3/4 inputs]",
    "main_demo update [success: 4/4 inputs]",
    "main_demo delete",
]


def run_command(cmd: List[str], cwd: Path) -> None:
    print(f"\n>>> Running {' '.join(cmd)} in {cwd.relative_to(ROOT)} ...")
    subprocess.run(cmd, cwd=cwd, check=True, stdout=sys.stdout, stderr=sys.stderr)


def run_benchmarks(project_dir: Path) -> None:
    # Standard throughput/memory benches.
    run_command(["cargo", "bench"], project_dir)
    # main_demo requires the validators feature.
    run_command(
        ["cargo", "bench", "--features", "validators", "--bench", "main_demo"],
        project_dir,
    )


# Criterion sanitizes benchmark names into filesystem directory names by
# replacing characters that are unsafe in a path component with `_`, then
# truncating to 64 characters (no hash suffix -- just a hard cut). Our
# `main_demo` bench names use `:` (e.g. "[fail: ...]") and several run past
# 64 characters (e.g. "... (email, slug_id, username)]"), so both rewrites
# have to be replicated here or those lookups silently miss.
_CRITERION_UNSAFE_CHARS = str.maketrans('?"/\\*<>:|^', "_" * 10)
_CRITERION_MAX_DIR_LEN = 64


def criterion_dir_name(bench_name: str) -> str:
    sanitized = bench_name.translate(_CRITERION_UNSAFE_CHARS)
    return sanitized[:_CRITERION_MAX_DIR_LEN]


def parse_criterion_json(project_dir: Path, bench_name: str) -> Optional[float]:
    """Return the mean point estimate in nanoseconds, or None if missing."""
    estimates_file = (
        project_dir
        / "target"
        / "criterion"
        / criterion_dir_name(bench_name)
        / "new"
        / "estimates.json"
    )
    if not estimates_file.exists():
        print(f"Warning: missing {estimates_file}", file=sys.stderr)
        return None
    data = json.loads(estimates_file.read_text())
    return float(data["mean"]["point_estimate"])


def collect_results(project_dir: Path) -> Dict[str, float]:
    results: Dict[str, float] = {}
    for name in THROUGHPUT_BENCHES + MEMORY_BENCHES + MAIN_DEMO_BENCHES:
        value = parse_criterion_json(project_dir, name)
        if value is not None:
            results[name] = value
    return results


def format_time(ns: float) -> str:
    if ns >= 1_000_000:
        return f"{ns / 1_000_000:.2f} ms"
    if ns >= 1_000:
        return f"{ns / 1_000:.2f} µs"
    return f"{ns:.2f} ns"


def throughput_ops_per_s(ns: float) -> str:
    if ns <= 0:
        return "-"
    ops = 1_000_000_000 / ns
    if ops >= 1_000_000:
        return f"{ops / 1_000_000:.2f}M"
    if ops >= 1_000:
        return f"{ops / 1_000:.2f}k"
    return f"{ops:.0f}"


def pct_change_value(old: float, new: float) -> Optional[float]:
    if old == 0:
        return None
    return (new - old) / old * 100


def pct_change(old: float, new: float) -> str:
    change = pct_change_value(old, new)
    if change is None:
        return "-"
    sign = "+" if change > 0 else ""
    return f"{sign}{change:.1f}%"


def render_table(
    lines: List[str],
    title: str,
    headers: str,
    benches: List[str],
    old_results: Dict[str, float],
    new_results: Dict[str, float],
    extra_col: bool = True,
    per_op_divisor: Optional[int] = None,
) -> None:
    lines.extend(["", f"## {title}", "", headers, "| --- | --- | --- | --- | --- |"])
    for name in benches:
        old_ns = old_results.get(name)
        new_ns = new_results.get(name)
        if old_ns is None or new_ns is None:
            lines.append(f"| {name} | - | - | - | - |")
            continue
        change = pct_change(old_ns, new_ns)
        if per_op_divisor:
            extra = format_time(new_ns / per_op_divisor)
        else:
            extra = f"~{throughput_ops_per_s(new_ns)}"
        lines.append(
            f"| {name} | {format_time(old_ns)} | {format_time(new_ns)} | "
            f"{change} | {extra} |"
        )


def generate_markdown(
    old_results: Dict[str, float], new_results: Dict[str, float]
) -> str:
    date = subprocess.check_output(["date", "+%Y-%m-%d"], text=True).strip()

    lines: List[str] = [
        "# `ivo` `/rs` vs `/rs-next` Benchmark Comparison",
        "",
        f"**Date**: {date}",
        "**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` "
        "run first in `/rs`, then in `/rs-next`",
        "**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)",
        "",
        "These are fresh, same-machine results for both implementations.",
    ]

    render_table(
        lines,
        "Throughput",
        "| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |",
        THROUGHPUT_BENCHES,
        old_results,
        new_results,
    )

    render_table(
        lines,
        "Memory Stress",
        "| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |",
        MEMORY_BENCHES,
        old_results,
        new_results,
        per_op_divisor=1000,
    )

    render_table(
        lines,
        "Main Demo",
        "| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |",
        MAIN_DEMO_BENCHES,
        old_results,
        new_results,
    )

    all_benches = THROUGHPUT_BENCHES + MEMORY_BENCHES + MAIN_DEMO_BENCHES
    changes = [
        c
        for name in all_benches
        if name in old_results and name in new_results
        for c in [pct_change_value(old_results[name], new_results[name])]
        if c is not None
    ]
    regressions = [c for c in changes if c > 0]
    summary_line = (
        f"- Summary: {len(changes)}/{len(all_benches)} benchmarks matched on both "
        f"sides. {'No regressions' if not regressions else f'{len(regressions)} regression(s)'} "
        f"-- `/rs-next` ranges from {min(changes):+.1f}% to {max(changes):+.1f}% "
        "relative to `/rs` across everything measured."
        if changes
        else "- Summary: no benchmarks matched on both sides -- see the `Warning: missing ...` "
        "lines printed during collection."
    )

    lines.extend(
        [
            "",
            "## Notes",
            "",
            summary_line,
            "- Lower time is better; negative `Change` means `/rs-next` is faster.",
            "- The new `update` API takes the existing data by value, so the update",
            "  harnesses clone the data each iteration. The old API borrowed the data.",
            "- Both projects use the same release-profile tuning (`lto = true`,",
            "  `codegen-units = 1`) and the same Tokio runtime.",
            "- `main_demo` benchmarks exercise the same realistic schema (constants,",
            "  lax/required/dependent/virtual fields, timestamps, grouped validation,",
            "  post-validation, hooks) in both implementations, across every distinct",
            "  outcome the schema can produce: each `create`/`update` failure mode",
            "  (required/validation/re-validation/post-validation), each success shape",
            "  (partial vs. full input), every `update [fail: nothing to update: ...]`",
            "  no-op-resubmission case, and `delete`.",
            "- Criterion sanitizes benchmark names into directory names under",
            "  `target/criterion/` (replacing filesystem-unsafe characters with `_` and",
            "  truncating to 64 characters); `criterion_dir_name()` in this script",
            "  replicates both so lookups for the longer `main_demo` names don't miss.",
        ]
    )

    return "\n".join(lines) + "\n"


def main() -> int:
    run_benchmarks(RS_DIR)
    run_benchmarks(RS_NEXT_DIR)

    old_results = collect_results(RS_DIR)
    new_results = collect_results(RS_NEXT_DIR)

    markdown = generate_markdown(old_results, new_results)
    RESULTS_PATH.write_text(markdown)

    print(f"\n>>> Wrote comparison to {RESULTS_PATH.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
