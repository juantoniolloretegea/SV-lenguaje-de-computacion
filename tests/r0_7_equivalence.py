#!/usr/bin/env python3
"""R0-7 — equivalencia diferencial sobre el mismo .svp.

Python es únicamente la referencia de prueba. El camino Rust recibe directamente
el mismo archivo .svp y no consume la IR emitida por Python.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import sys
from oracle_support import (run, assert_success, assert_json_equal, assert_bytes_equal,
                            assert_python_rejection, assert_rust_rejection,
                            check_invalid_corpus)
from run_conformance import EXPECTED_INVALID_CODES

ROOT = Path(__file__).resolve().parents[1]
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
INVALID_DIR = ROOT / "tests" / "conformance" / "invalid"

VALID_CASES = sorted(path.stem for path in VALID_DIR.glob("*.svp"))
INVALID_CASES = sorted(path.stem for path in INVALID_DIR.glob("*.svp"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--rust-bin",
        default=str(ROOT / "rust" / "target" / "debug" / "sv-native"),
    )
    args = parser.parse_args()

    failures: list[str] = []
    check_invalid_corpus(INVALID_DIR.glob("*.svp"))
    if not VALID_CASES:
        raise AssertionError("corpus válido vacío")

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

        try:
            assert_success(py)
            assert_success(rust)
            assert_json_equal(py.stdout, golden.read_bytes())
            assert_json_equal(rust.stdout, py.stdout)
            for command, first in [([sys.executable, "src/svp_main.py", str(source)], py),
                                   ([args.rust_bin, str(source)], rust)]:
                repeated = run(command)
                assert_success(repeated)
                assert_bytes_equal(first.stdout, repeated.stdout)
        except AssertionError as exc:
            failures.append(f"VALID {case}: {exc}")
            continue

        print(f"R0-7 VALID OK: {case}")

    for case in INVALID_CASES:
        source = INVALID_DIR / f"{case}.svp"
        py = run([sys.executable, "src/svp_main.py", str(source)])
        rust = run([args.rust_bin, str(source)])

        try:
            assert_python_rejection(py, EXPECTED_INVALID_CODES[source.name])
            assert_rust_rejection(rust, case)
        except AssertionError as exc:
            failures.append(f"INVALID {case}: {exc}")
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
