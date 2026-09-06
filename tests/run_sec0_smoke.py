#!/usr/bin/env python3
"""tests/run_sec0_smoke.py — Semilla adversarial mínima SEC-0B.

Comprueba tres propiedades básicas del compilador de referencia:
1) determinismo de lowering en un válido con identificadores largos;
2) rechazo limpio y estable en una colisión nominal (`E005`);
3) determinismo de lowering en una estructura moderadamente profunda con `query`.
"""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
ADV = ROOT / "tests" / "adversarial"

sys.path.insert(0, str(SRC))

from svp_main import process_file  # type: ignore
from svp_errors import SVPError  # type: ignore
from oracle_support import ordered_json, assert_bytes_equal


def assert_deterministic(path: Path, label: str) -> None:
    first = process_file(str(path)).encode("utf-8")
    second = process_file(str(path)).encode("utf-8")
    ordered_json(first)
    ordered_json(second)
    assert_bytes_equal(first, second)


def test_long_identifier_valid() -> None:
    assert_deterministic(ADV / "long_identifier_valid.svp", "long_identifier_valid")


def test_duplicate_name_invalid() -> None:
    path = ADV / "duplicate_name_invalid.svp"
    try:
        process_file(str(path))
    except SVPError as e:
        code = getattr(getattr(e, "error_def", None), "code", None)
        if code != "E005":
            raise AssertionError(f"duplicate_name_invalid: se esperaba E005, obtenido {code}")
        return
    raise AssertionError("duplicate_name_invalid: el compilador aceptó una colisión nominal")


def test_deep_nested_query_valid() -> None:
    # El antecedente de gramática 0.1 permanece archivado; DFL-007 ya lo retiró.
    assert_deterministic(ROOT / "tests/conformance/valid/query_context_all_variants.svp",
                         "query_context_all_variants")


def main() -> int:
    tests = [
        ("válido con identificadores largos", test_long_identifier_valid),
        ("inválido por colisión nominal", test_duplicate_name_invalid),
        ("válido con query moderadamente profunda", test_deep_nested_query_valid),
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
    print(f"SEC-0B smoke: {passed}/{len(tests)} pruebas superadas.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
