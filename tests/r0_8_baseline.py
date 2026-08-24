#!/usr/bin/env python3
"""R0-8 — medición basal externa del camino nativo soberano.

Este arnés no interpreta SVP ni sustituye al SUT. Lanza el ejecutable Rust
`sv-native` sobre los `.svp` válidos comprometidos y registra metrología del
proceso: latencia extremo a extremo, RSS máximo, tamaño/huella del binario,
entorno y huellas de entradas/salidas.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import statistics
import subprocess
import sys
import time
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
TIME_BIN = Path("/usr/bin/time")
RSS_RE = re.compile(r"SV_RSS_KB=(\d+)")
CPU_RE = re.compile(r"SV_CPU_USER_S=([0-9.]+) SV_CPU_SYS_S=([0-9.]+)")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def run_text(command: list[str], *, cwd: Path = ROOT) -> str:
    completed = subprocess.run(
        command,
        cwd=cwd,
        text=True,
        capture_output=True,
        check=True,
    )
    return completed.stdout.strip()


def read_first_line(path: Path, prefix: str) -> str | None:
    if not path.exists():
        return None
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.startswith(prefix):
            return line.split(":", 1)[1].strip()
    return None


def linux_environment() -> dict[str, Any]:
    cpu_model = read_first_line(Path("/proc/cpuinfo"), "model name")
    mem_total = read_first_line(Path("/proc/meminfo"), "MemTotal")
    return {
        "platform": platform.platform(),
        "system": platform.system(),
        "release": platform.release(),
        "machine": platform.machine(),
        "python_orchestrator": platform.python_version(),
        "cpu_count_logical": os.cpu_count(),
        "cpu_model": cpu_model,
        "mem_total": mem_total,
        "rustc": run_text(["rustc", "--version"]),
        "cargo": run_text(["cargo", "--version"]),
        "git_head": run_text(["git", "rev-parse", "HEAD"]),
    }


def percentile(sorted_values: list[int], p: float) -> int:
    if not sorted_values:
        raise ValueError("lista vacía")
    if len(sorted_values) == 1:
        return sorted_values[0]
    rank = (len(sorted_values) - 1) * p
    lo = int(rank)
    hi = min(lo + 1, len(sorted_values) - 1)
    frac = rank - lo
    return round(sorted_values[lo] * (1.0 - frac) + sorted_values[hi] * frac)


def wall_measurements(binary: Path, source: Path, warmups: int, runs: int) -> tuple[list[int], str]:
    expected_hash: str | None = None

    for _ in range(warmups):
        completed = subprocess.run(
            [str(binary), str(source)],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=True,
        )
        digest = sha256_bytes(completed.stdout)
        if expected_hash is None:
            expected_hash = digest
        elif digest != expected_hash:
            raise RuntimeError(f"salida no estable durante calentamiento: {source.name}")

    values: list[int] = []
    for _ in range(runs):
        start = time.perf_counter_ns()
        completed = subprocess.run(
            [str(binary), str(source)],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=True,
        )
        elapsed = time.perf_counter_ns() - start
        digest = sha256_bytes(completed.stdout)
        if expected_hash is None:
            expected_hash = digest
        elif digest != expected_hash:
            raise RuntimeError(f"salida no estable entre repeticiones: {source.name}")
        values.append(elapsed)

    assert expected_hash is not None
    return values, expected_hash


def rss_measurements(binary: Path, source: Path, runs: int) -> tuple[list[int], list[float], list[float]]:
    if not TIME_BIN.exists():
        raise RuntimeError("/usr/bin/time no disponible; no puede medirse RSS de forma declarada")

    rss_values: list[int] = []
    user_values: list[float] = []
    sys_values: list[float] = []
    for _ in range(runs):
        completed = subprocess.run(
            [
                str(TIME_BIN),
                "-f",
                "SV_RSS_KB=%M\nSV_CPU_USER_S=%U SV_CPU_SYS_S=%S",
                str(binary),
                str(source),
            ],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            text=True,
            check=True,
        )
        rss_match = RSS_RE.search(completed.stderr)
        cpu_match = CPU_RE.search(completed.stderr)
        if rss_match is None or cpu_match is None:
            raise RuntimeError(f"metrología no interpretable para {source.name}: {completed.stderr!r}")
        rss_values.append(int(rss_match.group(1)))
        user_values.append(float(cpu_match.group(1)))
        sys_values.append(float(cpu_match.group(2)))
    return rss_values, user_values, sys_values


def summarize_ns(values: list[int]) -> dict[str, int | float]:
    ordered = sorted(values)
    return {
        "runs": len(values),
        "min_ns": ordered[0],
        "median_ns": int(statistics.median(ordered)),
        "mean_ns": round(statistics.fmean(ordered), 2),
        "p90_ns": percentile(ordered, 0.90),
        "p95_ns": percentile(ordered, 0.95),
        "max_ns": ordered[-1],
    }


def summarize_int(values: list[int]) -> dict[str, int | float]:
    ordered = sorted(values)
    return {
        "runs": len(values),
        "min": ordered[0],
        "median": int(statistics.median(ordered)),
        "mean": round(statistics.fmean(ordered), 2),
        "max": ordered[-1],
    }


def summarize_float(values: list[float]) -> dict[str, int | float]:
    ordered = sorted(values)
    return {
        "runs": len(values),
        "min": ordered[0],
        "median": statistics.median(ordered),
        "mean": round(statistics.fmean(ordered), 6),
        "max": ordered[-1],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rust-bin", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--warmups", type=int, default=5)
    parser.add_argument("--runs", type=int, default=30)
    parser.add_argument("--rss-runs", type=int, default=5)
    args = parser.parse_args()

    binary = args.rust_bin.resolve()
    if not binary.is_file():
        parser.error(f"binario inexistente: {binary}")
    if args.warmups < 0 or args.runs < 1 or args.rss_runs < 1:
        parser.error("warmups >= 0; runs >= 1; rss-runs >= 1")

    cases = sorted(VALID_DIR.glob("*.svp"))
    if not cases:
        raise RuntimeError("no hay casos válidos comprometidos")

    result: dict[str, Any] = {
        "schema": "sv-r0-8-baseline-v1",
        "scope": "native-end-to-end-valid-corpus",
        "clock_role": "external-metrology-only",
        "sut": {
            "binary": str(binary.relative_to(ROOT)),
            "binary_bytes": binary.stat().st_size,
            "binary_sha256": sha256_file(binary),
            "build_profile": "release",
        },
        "environment": linux_environment(),
        "protocol": {
            "warmups_per_case": args.warmups,
            "wall_runs_per_case": args.runs,
            "rss_runs_per_case": args.rss_runs,
            "wall_clock": "Python time.perf_counter_ns around one sv-native process",
            "rss": "GNU /usr/bin/time %M per sv-native process",
            "cpu": "GNU /usr/bin/time %U/%S per sv-native process",
            "stdout": "captured and SHA-256 checked for stability",
        },
        "cases": [],
    }

    for source in cases:
        walls, output_hash = wall_measurements(binary, source, args.warmups, args.runs)
        rss, cpu_user, cpu_sys = rss_measurements(binary, source, args.rss_runs)
        case = {
            "name": source.stem,
            "source_file": str(source.relative_to(ROOT)),
            "source_bytes": source.stat().st_size,
            "source_sha256": sha256_file(source),
            "output_sha256": output_hash,
            "wall": summarize_ns(walls),
            "rss_kb": summarize_int(rss),
            "cpu_user_s": summarize_float(cpu_user),
            "cpu_sys_s": summarize_float(cpu_sys),
            "raw_wall_ns": walls,
            "raw_rss_kb": rss,
            "raw_cpu_user_s": cpu_user,
            "raw_cpu_sys_s": cpu_sys,
        }
        result["cases"].append(case)
        print(
            f"R0-8 BASELINE {source.stem}: "
            f"median={case['wall']['median_ns']} ns "
            f"p95={case['wall']['p95_ns']} ns "
            f"rss_median={case['rss_kb']['median']} KiB"
        )

    medians = [int(case["wall"]["median_ns"]) for case in result["cases"]]
    rss_medians = [int(case["rss_kb"]["median"]) for case in result["cases"]]
    result["corpus_summary"] = {
        "valid_cases": len(result["cases"]),
        "case_median_wall_ns_min": min(medians),
        "case_median_wall_ns_median": int(statistics.median(medians)),
        "case_median_wall_ns_max": max(medians),
        "case_median_rss_kb_min": min(rss_medians),
        "case_median_rss_kb_median": int(statistics.median(rss_medians)),
        "case_median_rss_kb_max": max(rss_medians),
    }

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"R0-8: evidencia escrita en {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
