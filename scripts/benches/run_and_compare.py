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
    "main_demo create",
    "main_demo update",
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


def parse_criterion_json(project_dir: Path, bench_name: str) -> Optional[float]:
    """Return the mean point estimate in nanoseconds, or None if missing."""
    estimates_file = (
        project_dir / "target" / "criterion" / bench_name / "new" / "estimates.json"
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


def pct_change(old: float, new: float) -> str:
    if old == 0:
        return "-"
    change = (new - old) / old * 100
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

    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- Lower time is better; negative `Change` means `/rs-next` is faster.",
            "- The new `update` API takes the existing data by value, so the update",
            "  harnesses clone the data each iteration. The old API borrowed the data.",
            "- Both projects use the same release-profile tuning (`lto = true`,",
            "  `codegen-units = 1`) and the same Tokio runtime.",
            "- `main_demo` benchmarks exercise the same realistic schema (constants,",
            "  lax/required/dependent/virtual fields, timestamps, grouped validation,",
            "  post-validation, hooks) in both implementations.",
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
