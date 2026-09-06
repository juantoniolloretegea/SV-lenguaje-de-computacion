#!/usr/bin/env python3
"""tests/run_cli_smoke.py — Smoke tests del CLI de referencia SV.

Comprueba exclusivamente el contrato externo mínimo de la CLI:
1) caso válido por stdout
2) caso válido con -o
3) caso inválido con rc=1 y ERROR: por stderr
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path
from oracle_support import (run, assert_success, assert_json_equal,
                            assert_python_rejection)

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
VALID = ROOT / "tests" / "conformance" / "valid"
INVALID = ROOT / "tests" / "conformance" / "invalid"
CLI = SRC / "svp_main.py"
PYTHON = sys.executable


def run_cli(*args: str) -> subprocess.CompletedProcess[bytes]:
    cmd = [PYTHON, str(CLI), *args]
    return run(cmd)


def test_valid_stdout() -> None:
    svp = VALID / "cell_basic.svp"
    expected = VALID / "cell_basic.expected.json"
    proc = run_cli(str(svp))
    if proc.returncode != 0:
        raise AssertionError(f"valid_stdout: rc={proc.returncode} stderr={proc.stderr!r}")
    assert_success(proc)
    assert_json_equal(proc.stdout, expected.read_bytes())


def test_valid_output_file() -> None:
    svp = VALID / "compose_basic.svp"
    expected = VALID / "compose_basic.expected.json"
    with tempfile.TemporaryDirectory() as tmpdir:
        out = Path(tmpdir) / "compose_basic.ir.json"
        proc = run_cli(str(svp), "-o", str(out))
        if proc.returncode != 0:
            raise AssertionError(f"valid_output_file: rc={proc.returncode} stderr={proc.stderr!r}")
        if not out.exists():
            raise AssertionError("valid_output_file: no se creó el archivo de salida")
        if proc.stdout or proc.stderr:
            raise AssertionError("CLI con -o produjo salida adicional")
        assert_json_equal(out.read_bytes(), expected.read_bytes())


def test_invalid_cli() -> None:
    svp = INVALID / "u_coercion.svp"
    with tempfile.TemporaryDirectory() as tmpdir:
        out = Path(tmpdir) / "should_not_exist.ir.json"
        proc = run_cli(str(svp), "-o", str(out))
        if proc.returncode != 1:
            raise AssertionError(f"invalid_cli: rc esperado 1, obtenido {proc.returncode}")
        assert_python_rejection(proc, "E507")
        if out.exists():
            raise AssertionError("invalid_cli: se creó archivo de salida para caso inválido")


def main() -> int:
    tests = [
        ("CLI válido por stdout", test_valid_stdout),
        ("CLI válido con -o", test_valid_output_file),
        ("CLI inválido con rc=1", test_invalid_cli),
    ]
    passed = 0
    for label, fn in tests:
        try:
            fn()
            print(f"[OK] {label}")
            passed += 1
        except Exception as e:
            print(f"[FAIL] {label}: {e}", file=sys.stderr)
            return 1
    print(f"Smoke CLI: {passed}/{len(tests)} pruebas superadas.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
