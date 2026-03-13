#!/usr/bin/env python3
"""tests/run_cli_smoke.py — Smoke tests del CLI de referencia SV.

Comprueba exclusivamente el contrato externo mínimo de la CLI:
1) caso válido por stdout
2) caso válido con -o
3) caso inválido con rc=1 y ERROR: por stderr
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
VALID = ROOT / "tests" / "conformance" / "valid"
INVALID = ROOT / "tests" / "conformance" / "invalid"
CLI = SRC / "svp_main.py"
PYTHON = sys.executable


def canonicalize_json_text(text: str) -> str:
    data = json.loads(text)
    return json.dumps(data, ensure_ascii=False, sort_keys=True, indent=2) + "\n"


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def run_cli(*args: str) -> subprocess.CompletedProcess[str]:
    cmd = [PYTHON, str(CLI), *args]
    return subprocess.run(cmd, capture_output=True, text=True, cwd=str(ROOT))


def assert_equal(label: str, left: str, right: str) -> None:
    if left != right:
        raise AssertionError(f"{label}: salida distinta de expected")


def test_valid_stdout() -> None:
    svp = VALID / "cell_basic.svp"
    expected = VALID / "cell_basic.expected.json"
    proc = run_cli(str(svp))
    if proc.returncode != 0:
        raise AssertionError(f"valid_stdout: rc={proc.returncode} stderr={proc.stderr!r}")
    actual = canonicalize_json_text(proc.stdout)
    wanted = canonicalize_json_text(read_text(expected))
    assert_equal("valid_stdout", actual, wanted)


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
        actual = canonicalize_json_text(read_text(out))
        wanted = canonicalize_json_text(read_text(expected))
        assert_equal("valid_output_file", actual, wanted)


def test_invalid_cli() -> None:
    svp = INVALID / "u_coercion.svp"
    with tempfile.TemporaryDirectory() as tmpdir:
        out = Path(tmpdir) / "should_not_exist.ir.json"
        proc = run_cli(str(svp), "-o", str(out))
        if proc.returncode != 1:
            raise AssertionError(f"invalid_cli: rc esperado 1, obtenido {proc.returncode}")
        if "ERROR:" not in proc.stderr:
            raise AssertionError(f"invalid_cli: stderr sin prefijo ERROR:, stderr={proc.stderr!r}")
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
