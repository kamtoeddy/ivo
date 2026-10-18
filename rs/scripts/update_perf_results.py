#!/usr/bin/env python3
"""
Run the ivo_rs Criterion benchmarks and rewrite PERF_RESULTS.md with the results.

The new results are compared against results that were already recorded,
never against nothing:

- HEAD: the results in PERF_RESULTS.md as committed at git HEAD.
- Partial output: the results recorded right after the API update that made
  `create` build a partial output (`ctx.values()` returning the partial
  output struct). This reference is carried over from run to run.

PERF_RESULTS.md is rewritten as a single report. Benchmarks without any
recorded result yet are listed but not compared; their results are stored in
the report, so runs after it is committed can compare them.

Usage (from anywhere):
    python3 rs/scripts/update_perf_results.py            # full Criterion runs
    python3 rs/scripts/update_perf_results.py --quick    # shorter runs
    python3 rs/scripts/update_perf_results.py --skip-run # reuse the last results
"""

from __future__ import annotations

import argparse
import datetime
import json
import platform
import re
import subprocess
import sys
import time
import tomllib
from dataclasses import dataclass
from pathlib import Path

RS_DIR = Path(__file__).resolve().parent.parent
REPO_DIR = RS_DIR.parent
RESULTS_FILE = RS_DIR / "PERF_RESULTS.md"
SCRIPT_PATH = Path(__file__).resolve().relative_to(REPO_DIR)

QUICK_ARGS = ["--warm-up-time", "1", "--measurement-time", "3"]

DATA_PATTERN = re.compile(r"<!-- data: (\{.*?\}) -->", re.S)
TIME_PATTERN = re.compile(r"^([\d.]+)\s*(ns|µs|us|ms|s)$")
TIME_UNITS = {"ns": 1, "µs": 1e3, "us": 1e3, "ms": 1e6, "s": 1e9}

# Where the partial-output reference came from before it was carried in this
# report's data (the section written when that API update landed).
PARTIAL_OUTPUT_LABEL = "Partial output"
PARTIAL_OUTPUT_DESCRIPTION = (
    "the results recorded right after the API update that made `create` build a partial "
    "output (`ctx.values()` returns the partial output struct), on 2026-10-18"
)
LEGACY_PARTIAL_OUTPUT_HEADING = "Partial output at creation"


@dataclass
class BenchTarget:
    name: str
    source: Path
    features: list[str]

    def bench_names(self) -> list[str]:
        """The benchmark ids registered by this target, in source order."""
        sources = [self.source]
        if self.source.name == "mod.rs":
            sources += sorted(p for p in self.source.parent.glob("*.rs") if p != self.source)
        names: list[str] = []
        for source in sources:
            names += re.findall(r'bench_function\(\s*"((?:[^"\\]|\\.)*)"', source.read_text())
        return names


def bench_targets() -> list[BenchTarget]:
    manifest = tomllib.loads((RS_DIR / "Cargo.toml").read_text())
    targets = []
    for bench in manifest.get("bench", []):
        name = bench["name"]
        source = RS_DIR / bench.get("path", f"benches/{name}.rs")
        targets.append(BenchTarget(name, source, bench.get("required-features", [])))
    if not targets:
        sys.exit("no [[bench]] targets found in rs/Cargo.toml")
    return targets


def command_output(cmd: list[str], cwd: Path = RS_DIR) -> str | None:
    try:
        return subprocess.run(
            cmd, cwd=cwd, check=True, capture_output=True, text=True
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


# --- running and collecting -------------------------------------------------


def run_target(target: BenchTarget, quick: bool) -> None:
    cmd = ["cargo", "bench", "--bench", target.name]
    if target.features:
        cmd += ["--features", ",".join(target.features)]
    cmd += ["--", "--noplot"] + (QUICK_ARGS if quick else [])
    print(f"\n>>> {' '.join(cmd)}", flush=True)
    subprocess.run(cmd, cwd=RS_DIR, check=True)


def criterion_dir() -> Path:
    metadata = command_output(["cargo", "metadata", "--format-version", "1", "--no-deps"])
    if metadata is None:
        sys.exit("`cargo metadata` failed")
    return Path(json.loads(metadata)["target_directory"]) / "criterion"


def collect(targets: list[BenchTarget], not_before: float | None) -> dict[str, dict[str, float]]:
    """Median time in nanoseconds per benchmark, grouped by bench target."""
    available = {}
    for info_file in criterion_dir().glob("**/new/benchmark.json"):
        title = json.loads(info_file.read_text())["title"]
        available[title] = info_file.parent / "estimates.json"

    results: dict[str, dict[str, float]] = {}
    problems = []
    for target in targets:
        results[target.name] = {}
        for name in target.bench_names():
            estimates_file = available.get(name)
            if estimates_file is None:
                problems.append(f"{target.name}: no Criterion results for {name!r}")
            elif not_before is not None and estimates_file.stat().st_mtime < not_before:
                problems.append(f"{target.name}: {name!r} was not re-run")
            else:
                estimates = json.loads(estimates_file.read_text())
                results[target.name][name] = float(estimates["median"]["point_estimate"])
    if problems:
        sys.exit("\n".join(["could not collect every benchmark result:", *problems]))
    return results


# --- recorded results ---------------------------------------------------------


def parse_time(text: str) -> float | None:
    match = TIME_PATTERN.match(text.strip())
    return float(match.group(1)) * TIME_UNITS[match.group(2)] if match else None


def section(markdown: str, heading_text: str) -> str | None:
    """The body of the first `##` section whose heading contains `heading_text`."""
    match = re.search(rf"^## [^\n]*{re.escape(heading_text)}[^\n]*$", markdown, re.M)
    if not match:
        return None
    end = re.search(r"^## ", markdown[match.end() :], re.M)
    return markdown[match.end() : match.end() + end.start()] if end else markdown[match.end() :]


def after_column(markdown: str) -> dict[str, float]:
    """Benchmark -> time from the `After` column of `| Benchmark | Before | After |` tables."""
    times = {}
    for row in re.findall(r"^\|(.+)\|\s*$", markdown, re.M):
        cells = [c.strip() for c in row.split("|")]
        if len(cells) >= 3 and (after := parse_time(cells[2])) is not None:
            times[cells[0]] = after
    return times


def recorded_baselines() -> tuple[dict[str, float], dict[str, float], str]:
    """The (HEAD, partial output) results recorded in PERF_RESULTS.md at git HEAD."""
    path = RESULTS_FILE.relative_to(REPO_DIR).as_posix()
    head_file = command_output(["git", "show", f"HEAD:{path}"], cwd=REPO_DIR) or ""
    head_commit = command_output(["git", "rev-parse", "--short", "HEAD"], cwd=REPO_DIR) or "HEAD"

    if match := DATA_PATTERN.search(head_file):
        # a report written by this script
        data = json.loads(match.group(1))
        head = {n: t for benches in data["results"].values() for n, t in benches.items()}
        return head, data.get("partial_output", {}), head_commit

    # the hand-written file this report replaced: its latest section holds
    # HEAD's results, and the partial-output section the reference ones
    sections = re.split(r"^(?=## )", head_file, flags=re.M)
    latest = next((s for s in reversed(sections) if "| Before | After |" in s), "")
    partial = section(head_file, LEGACY_PARTIAL_OUTPUT_HEADING) or ""
    return after_column(latest), after_column(partial), head_commit


# --- report -----------------------------------------------------------------


def format_time(ns: float) -> str:
    if ns >= 1_000_000:
        return f"{ns / 1_000_000:.2f} ms"
    if ns >= 1_000:
        return f"{ns / 1_000:.2f} µs"
    return f"{ns:.2f} ns"


def format_change(before: float | None, now: float) -> str:
    return "-" if before is None else f"{(now - before) / before * 100:+.1f}%"


def machine_description() -> str:
    cpu = None
    if sys.platform == "darwin":
        cpu = command_output(["sysctl", "-n", "machdep.cpu.brand_string"])
    elif Path("/proc/cpuinfo").exists():
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    return f"{cpu or platform.processor() or 'unknown CPU'}, {platform.system()} {platform.release()}"


def measured_commit() -> str:
    commit = command_output(["git", "rev-parse", "--short", "HEAD"]) or "unknown commit"
    # untracked files (notes, new scripts, ...) don't change what's benchmarked
    dirty = command_output(["git", "status", "--porcelain", "--untracked-files=no", "--", "."])
    return f"`{commit}`" + (" with uncommitted changes" if dirty else "")


def render_report(
    targets: list[BenchTarget],
    results: dict[str, dict[str, float]],
    head: dict[str, float],
    partial: dict[str, float],
    head_commit: str,
    quick: bool,
) -> str:
    now = datetime.datetime.now().astimezone()
    rustc = command_output(["rustc", "--version"]) or "unknown rustc"
    args = " --quick" if quick else ""
    lines = [
        "# `ivo_rs` benchmark results",
        "",
        f"<!-- Generated by {SCRIPT_PATH}; edits to this file are overwritten. -->",
        "",
        f"**Date**: {now:%Y-%m-%d %H:%M %Z}",
        f"**Measured**: {measured_commit()}",
        f"**Toolchain**: {rustc}",
        f"**Machine**: {machine_description()}",
        f"**Command**: `python3 {SCRIPT_PATH}{args}`",
        "",
        "Times are Criterion's median estimate; a negative change means faster. Each result is",
        "compared with:",
        "",
        f"- **HEAD**: the results recorded in this file at `{head_commit}`.",
        f"- **{PARTIAL_OUTPUT_LABEL}**: {PARTIAL_OUTPUT_DESCRIPTION}.",
    ]
    not_compared = []
    for target in targets:
        rows = []
        for name, ns in results[target.name].items():
            before_head, before_partial = head.get(name), partial.get(name)
            if before_head is None and before_partial is None:
                not_compared.append(name)
                continue
            rows.append(
                f"| {name} "
                f"| {format_time(before_partial) if before_partial is not None else '-'} "
                f"| {format_time(before_head) if before_head is not None else '-'} "
                f"| {format_time(ns)} "
                f"| {format_change(before_partial, ns)} "
                f"| {format_change(before_head, ns)} |"
            )
        if not rows:
            continue
        features = f" (features: {', '.join(target.features)})" if target.features else ""
        lines += [
            "",
            f"## `{target.name}`{features}",
            "",
            f"| Benchmark | {PARTIAL_OUTPUT_LABEL} | HEAD | Now "
            f"| vs {PARTIAL_OUTPUT_LABEL} | vs HEAD |",
            "| --- | --- | --- | --- | --- | --- |",
            *rows,
        ]
    if not_compared:
        lines += [
            "",
            "## Not compared yet",
            "",
            "No earlier result was recorded for these. Once this report is committed, later runs",
            "compare them with it:",
            "",
            *(f"- {name}" for name in not_compared),
        ]
    data = {
        "date": now.isoformat(timespec="seconds"),
        "results": results,
        "partial_output": partial,
    }
    lines += ["", f"<!-- data: {json.dumps(data)} -->", ""]
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "--quick",
        action="store_true",
        help="shorter Criterion runs (--warm-up-time 1 --measurement-time 3)",
    )
    parser.add_argument(
        "--skip-run",
        action="store_true",
        help="don't run the benchmarks; use Criterion's latest results",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=RESULTS_FILE,
        help=f"where to write the report (default: {RESULTS_FILE.relative_to(REPO_DIR)})",
    )
    args = parser.parse_args()

    targets = bench_targets()
    head, partial, head_commit = recorded_baselines()
    if not head and not partial:
        sys.exit(
            f"no recorded results found in {RESULTS_FILE.relative_to(REPO_DIR)} at HEAD "
            "to compare against"
        )

    started = time.time()
    if not args.skip_run:
        for target in targets:
            run_target(target, args.quick)
    results = collect(targets, not_before=None if args.skip_run else started)

    args.output.write_text(render_report(targets, results, head, partial, head_commit, args.quick))
    count = sum(len(benches) for benches in results.values())
    print(f"\nWrote {count} benchmark results to {args.output}")


if __name__ == "__main__":
    main()
