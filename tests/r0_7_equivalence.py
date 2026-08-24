#!/usr/bin/env python3
"""R0-7 — equivalencia diferencial sobre el mismo .svp.

Python es únicamente la referencia de prueba. El camino Rust recibe directamente
el mismo archivo .svp y no consume la IR emitida por Python.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]

CASES = [
    "cell_basic",
    "admissibility_spec_states_permutados",
    "resolve_projection",
    "frame_cell_spec_compartida_valida",
]


def canonical_json(text: str) -> str:
    return json.dumps(
        json.loads(text),
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    )


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--rust-bin",
        default=str(ROOT / "rust" / "target" / "debug" / "sv-native"),
    )
    args = parser.parse_args()

    failures: list[str] = []
    for case in CASES:
        source = ROOT / "tests" / "conformance" / "valid" / f"{case}.svp"
        golden = source.with_suffix(".expected.json")

        py = run([sys.executable, "src/svp_main.py", str(source)])
        if py.returncode != 0:
            failures.append(f"{case}: referencia Python falló: {py.stderr.strip()}")
            continue

        rust = run([args.rust_bin, str(source)])
        if rust.returncode != 0:
            failures.append(f"{case}: camino Rust falló: {rust.stderr.strip()}")
            continue

        reference = canonical_json(py.stdout)
        expected = canonical_json(golden.read_text(encoding="utf-8"))
        sovereign = canonical_json(rust.stdout)

        if reference != expected:
            failures.append(
                f"{case}: la referencia vigente ya no coincide con el golden comprometido"
            )
            continue
        if sovereign != reference:
            failures.append(
                f"{case}: divergencia Rust/Python\n"
                f"  Python: {reference}\n"
                f"  Rust:   {sovereign}"
            )
            continue

        print(f"R0-7 OK: {case}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print(f"R0-7: {len(CASES)}/{len(CASES)} casos equivalentes sobre el mismo .svp")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
