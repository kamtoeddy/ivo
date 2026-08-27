#!/usr/bin/env python3
"""
Run the /rs and /rs-next Criterion benchmarks and generate a side-by-side
comparison at /scripts/benches/RESULTS.md.
"""

from __future__ import annotations

import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple

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


@dataclass(frozen=True)
class BenchResult:
    name: str
    mean_ns: float


def run_cargo_bench(project_dir: Path) -> None:
    print(f"\n>>> Running cargo bench in {project_dir.relative_to(ROOT)} ...")
    subprocess.run(
        ["cargo", "bench"],
        cwd=project_dir,
        check=True,
        stdout=sys.stdout,
        stderr=sys.stderr,
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
    for name in THROUGHPUT_BENCHES + MEMORY_BENCHES:
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


def generate_markdown(
    old_results: Dict[str, float], new_results: Dict[str, float]
) -> str:
    date = subprocess.check_output(["date", "+%Y-%m-%d"], text=True).strip()

    lines: List[str] = [
        "# `ivo` `/rs` vs `/rs-next` Benchmark Comparison",
        "",
        f"**Date**: {date}",
        "**Command**: `cargo bench` run first in `/rs`, then in `/rs-next`",
        "**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)",
        "",
        "These are fresh, same-machine results for both implementations.",
        "",
        "## Throughput",
        "",
        "| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |",
        "| --- | --- | --- | --- | --- |",
    ]

    for name in THROUGHPUT_BENCHES:
        old_ns = old_results.get(name)
        new_ns = new_results.get(name)
        if old_ns is None or new_ns is None:
            lines.append(f"| {name} | - | - | - | - |")
            continue
        lines.append(
            f"| {name} | {format_time(old_ns)} | {format_time(new_ns)} | "
            f"{pct_change(old_ns, new_ns)} | ~{throughput_ops_per_s(new_ns)} |"
        )

    lines.extend(
        [
            "",
            "## Memory Stress",
            "",
            "| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |",
            "| --- | --- | --- | --- | --- |",
        ]
    )

    for name in MEMORY_BENCHES:
        old_ns = old_results.get(name)
        new_ns = new_results.get(name)
        if old_ns is None or new_ns is None:
            lines.append(f"| {name} | - | - | - | - |")
            continue
        per_op_new = new_ns
        # For x1000 benches, the per-operation time is the reported time / 1000.
        per_op_new = new_ns / 1000
        lines.append(
            f"| {name} | {format_time(old_ns)} | {format_time(new_ns)} | "
            f"{pct_change(old_ns, new_ns)} | {format_time(per_op_new)} |"
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
        ]
    )

    return "\n".join(lines) + "\n"


def main() -> int:
    run_cargo_bench(RS_DIR)
    run_cargo_bench(RS_NEXT_DIR)

    old_results = collect_results(RS_DIR)
    new_results = collect_results(RS_NEXT_DIR)

    markdown = generate_markdown(old_results, new_results)
    RESULTS_PATH.write_text(markdown)

    print(f"\n>>> Wrote comparison to {RESULTS_PATH.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
