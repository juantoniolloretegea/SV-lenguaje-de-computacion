#!/usr/bin/env python3
"""R0-8 — curvas materiales controladas para N01, N02 y N03.

Genera programas SVP válidos únicamente con construcciones ya vigentes y mide
el ejecutable nativo `sv-native`. Los fuentes generados se preservan junto con
los datos para que cada punto sea reproducible.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import resource
import statistics
import subprocess
import time
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
TIME_BIN = Path("/usr/bin/time")
RSS_RE = re.compile(r"SV_RSS_KB=(\d+)")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def run_text(command: list[str]) -> str:
    return subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=True,
    ).stdout.strip()


def percentile(values: list[int], p: float) -> int:
    ordered = sorted(values)
    if len(ordered) == 1:
        return ordered[0]
    rank = (len(ordered) - 1) * p
    lo = int(rank)
    hi = min(lo + 1, len(ordered) - 1)
    frac = rank - lo
    return round(ordered[lo] * (1.0 - frac) + ordered[hi] * frac)


def summarize_ns(values: list[int]) -> dict[str, int | float]:
    ordered = sorted(values)
    return {
        "runs": len(values),
        "min_ns": ordered[0],
        "median_ns": int(statistics.median(ordered)),
        "mean_ns": round(statistics.fmean(ordered), 2),
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


def write_source(directory: Path, name: str, source: str) -> Path:
    path = directory / f"{name}.svp"
    path.write_text(source, encoding="utf-8")
    return path


def generate_n01(directory: Path, digits: int) -> Path:
    value = "1" + "0" * (digits - 1)
    source = (
        "admissibility_spec AS1 {\n"
        f"  parameter_id: {value};\n"
        "  states: {NotAdmitted, Ok, Degraded};\n"
        "  rule: ReglaEscala;\n"
        "}\n"
    )
    return write_source(directory, f"n01_nat_digits_{digits}", source)


def generate_n02_objects(directory: Path, count: int) -> Path:
    lines = [f"codomain K{i} = {{ A, B, C }};" for i in range(count)]
    return write_source(directory, f"n02_objects_{count}", "\n".join(lines) + "\n")


def generate_n02_identifier(directory: Path, length: int) -> Path:
    if length < 2:
        raise ValueError("identifier length >= 2")
    name = "K" + "X" * (length - 1)
    return write_source(directory, f"n02_identifier_{length}", f"codomain {name} = {{ A, B, C }};\n")


def generate_n03_frame(directory: Path, nodes: int) -> Path:
    vector = "[Zero, One, U, Zero, Zero, One, U, Zero, One]"
    parts = [
        "codomain K3 = { APTO, NO_APTO, INDETERMINADO };",
        'output_semantics Klin { APTO -> "ok"; NO_APTO -> "no"; INDETERMINADO -> "u"; }',
        "cellspec C1 { b: 3; codomain: K3; semantics: Klin; role: Base; }",
        "",
    ]
    for i in range(nodes):
        parts.append(f"coupledspec CC{i} {{ cell: C1; bridges: [3]; }}")
    parts.append("")
    for i in range(nodes):
        parts.extend(
            [
                f"coupledstate S{i} {{",
                f"  spec: CC{i};",
                f"  base_vector: {vector};",
                f"  updated_vector: {vector};",
                "}",
            ]
        )
    parts.extend(
        [
            "",
            "semantic_relation RArch {",
            "  kind: DeclaredRelation;",
            "  constraints: [CoherenciaLocal];",
            "}",
            "",
            "graph Arch1 {",
            "  nodes: [" + ", ".join(f"CC{i}" for i in range(nodes)) + "];",
            "  edges: [];",
            "  relation: RArch;",
            "  regime: Simple;",
            "}",
            "",
            "frame F1 {",
            "  index: 1;",
            "  architecture: Arch1;",
            "  cell_states: [" + ", ".join(f"S{i}" for i in range(nodes)) + "];",
            "  eval_results: [];",
            "  gate_results: [];",
            "  supervision: [];",
            "  criticalities: [];",
            "}",
            "",
        ]
    )
    return write_source(directory, f"n03_frame_nodes_{nodes}", "\n".join(parts))


def measure_wall(binary: Path, source: Path, warmups: int, runs: int) -> tuple[list[int], str]:
    output_hash: str | None = None
    for _ in range(warmups):
        completed = subprocess.run(
            [str(binary), str(source)], cwd=ROOT,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True,
        )
        digest = sha256_bytes(completed.stdout)
        if output_hash is None:
            output_hash = digest
        elif digest != output_hash:
            raise RuntimeError(f"salida no estable: {source.name}")

    values: list[int] = []
    for _ in range(runs):
        start = time.perf_counter_ns()
        completed = subprocess.run(
            [str(binary), str(source)], cwd=ROOT,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True,
        )
        values.append(time.perf_counter_ns() - start)
        digest = sha256_bytes(completed.stdout)
        if output_hash is None:
            output_hash = digest
        elif digest != output_hash:
            raise RuntimeError(f"salida no estable: {source.name}")
    assert output_hash is not None
    return values, output_hash


def measure_rss(binary: Path, source: Path, runs: int) -> list[int]:
    if not TIME_BIN.exists():
        raise RuntimeError("/usr/bin/time no disponible")
    values: list[int] = []
    for _ in range(runs):
        completed = subprocess.run(
            [str(TIME_BIN), "-f", "SV_RSS_KB=%M", str(binary), str(source)],
            cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            text=True, check=True,
        )
        match = RSS_RE.search(completed.stderr)
        if match is None:
            raise RuntimeError(f"RSS no interpretable: {source.name}")
        values.append(int(match.group(1)))
    return values


def measure_cpu_batch(binary: Path, source: Path, runs: int) -> dict[str, int | float]:
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter_ns()
    for _ in range(runs):
        subprocess.run(
            [str(binary), str(source)], cwd=ROOT,
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, check=True,
        )
    elapsed = time.perf_counter_ns() - start
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    user_ns = round((after.ru_utime - before.ru_utime) * 1_000_000_000)
    sys_ns = round((after.ru_stime - before.ru_stime) * 1_000_000_000)
    return {
        "runs": runs,
        "batch_wall_ns": elapsed,
        "batch_user_ns": user_ns,
        "batch_sys_ns": sys_ns,
        "wall_ns_per_process": round(elapsed / runs, 2),
        "user_ns_per_process": round(user_ns / runs, 2),
        "sys_ns_per_process": round(sys_ns / runs, 2),
    }


def measure_case(binary: Path, source: Path, family: str, scale: int, warmups: int, runs: int, rss_runs: int, cpu_batch_runs: int) -> dict[str, Any]:
    wall, output_hash = measure_wall(binary, source, warmups, runs)
    rss = measure_rss(binary, source, rss_runs)
    cpu = measure_cpu_batch(binary, source, cpu_batch_runs)
    record = {
        "family": family,
        "scale": scale,
        "source_file": str(source),
        "source_bytes": source.stat().st_size,
        "source_sha256": sha256_file(source),
        "output_sha256": output_hash,
        "wall": summarize_ns(wall),
        "rss_kb": summarize_int(rss),
        "cpu_batch": cpu,
        "raw_wall_ns": wall,
        "raw_rss_kb": rss,
    }
    print(
        f"R0-8 SCALE {family}={scale}: bytes={record['source_bytes']} "
        f"median={record['wall']['median_ns']} ns rss={record['rss_kb']['median']} KiB"
    )
    return record


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rust-bin", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--generated-dir", required=True, type=Path)
    parser.add_argument("--source-head", required=True)
    parser.add_argument("--base-head", required=True)
    parser.add_argument("--warmups", type=int, default=3)
    parser.add_argument("--runs", type=int, default=20)
    parser.add_argument("--rss-runs", type=int, default=3)
    parser.add_argument("--cpu-batch-runs", type=int, default=100)
    args = parser.parse_args()

    binary = args.rust_bin.resolve()
    generated = args.generated_dir.resolve()
    generated.mkdir(parents=True, exist_ok=True)

    plans: list[tuple[str, int, Path]] = []
    for digits in [1, 10, 100, 1000, 5000]:
        plans.append(("N01_nat_digits", digits, generate_n01(generated, digits)))
    for count in [1, 10, 50, 100, 500]:
        plans.append(("N02_ir_objects", count, generate_n02_objects(generated, count)))
    for length in [8, 64, 256, 1024, 4096]:
        plans.append(("N02_identifier_length", length, generate_n02_identifier(generated, length)))
    for nodes in [1, 10, 50, 100, 250]:
        plans.append(("N03_frame_nodes", nodes, generate_n03_frame(generated, nodes)))

    result: dict[str, Any] = {
        "schema": "sv-r0-8-scale-v1",
        "sut": {
            "source_head": args.source_head,
            "base_head": args.base_head,
            "checkout_head": run_text(["git", "rev-parse", "HEAD"]),
            "binary": str(binary.relative_to(ROOT)),
            "binary_bytes": binary.stat().st_size,
            "binary_sha256": sha256_file(binary),
            "build_profile": "release",
        },
        "environment": {
            "platform": platform.platform(),
            "machine": platform.machine(),
            "rustc": run_text(["rustc", "--version"]),
            "cargo": run_text(["cargo", "--version"]),
            "runner_os": os.environ.get("RUNNER_OS"),
            "runner_arch": os.environ.get("RUNNER_ARCH"),
            "image_os": os.environ.get("ImageOS"),
            "image_version": os.environ.get("ImageVersion"),
        },
        "protocol": {
            "warmups_per_point": args.warmups,
            "wall_runs_per_point": args.runs,
            "rss_runs_per_point": args.rss_runs,
            "cpu_batch_runs_per_point": args.cpu_batch_runs,
            "clock_role": "external-metrology-only",
            "generated_programs": "valid SVP using only current grammar constructs",
        },
        "points": [],
    }

    for family, scale, source in plans:
        result["points"].append(
            measure_case(
                binary, source, family, scale,
                args.warmups, args.runs, args.rss_runs, args.cpu_batch_runs,
            )
        )

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"R0-8: curvas escritas en {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
