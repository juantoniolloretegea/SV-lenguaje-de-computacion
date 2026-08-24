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
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
INVALID_DIR = ROOT / "tests" / "conformance" / "invalid"

VALID_CASES = sorted(path.stem for path in VALID_DIR.glob("*.svp"))
INVALID_CASES = sorted(path.stem for path in INVALID_DIR.glob("*.svp"))


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

    for case in VALID_CASES:
        source = VALID_DIR / f"{case}.svp"
        golden = source.with_suffix(".expected.json")

        py = run([sys.executable, "src/svp_main.py", str(source)])
        if py.returncode != 0:
            failures.append(f"VALID {case}: referencia Python falló: {py.stderr.strip()}")
            continue

        rust = run([args.rust_bin, str(source)])
        if rust.returncode != 0:
            failures.append(f"VALID {case}: camino Rust falló: {rust.stderr.strip()}")
            continue

        reference = canonical_json(py.stdout)
        expected = canonical_json(golden.read_text(encoding="utf-8"))
        sovereign = canonical_json(rust.stdout)

        if reference != expected:
            failures.append(
                f"VALID {case}: la referencia vigente ya no coincide con el golden comprometido"
            )
            continue
        if sovereign != reference:
            failures.append(
                f"VALID {case}: divergencia Rust/Python\n"
                f"  Python: {reference}\n"
                f"  Rust:   {sovereign}"
            )
            continue

        print(f"R0-7 VALID OK: {case}")

    for case in INVALID_CASES:
        source = INVALID_DIR / f"{case}.svp"
        py = run([sys.executable, "src/svp_main.py", str(source)])
        rust = run([args.rust_bin, str(source)])

        if py.returncode == 0:
            failures.append(
                f"INVALID {case}: la referencia Python aceptó un caso comprometido como inválido"
            )
            continue
        if rust.returncode == 0:
            failures.append(
                f"INVALID {case}: Rust aceptó una entrada que la referencia rechaza"
            )
            continue

        print(f"R0-7 INVALID OK: {case}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print(
        "R0-7: "
        f"{len(VALID_CASES)}/{len(VALID_CASES)} válidos equivalentes y "
        f"{len(INVALID_CASES)}/{len(INVALID_CASES)} inválidos rechazados sobre el mismo .svp"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
