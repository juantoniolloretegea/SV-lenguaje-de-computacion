#!/usr/bin/env python3
"""tests/run_sec0_smoke.py — Semilla adversarial mínima SEC-0B.

Comprueba tres propiedades básicas del compilador de referencia:
1) determinismo de lowering en un válido con identificadores largos;
2) rechazo limpio y estable en una colisión nominal (`E005`);
3) determinismo de lowering en una estructura moderadamente profunda con `query`.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
ADV = ROOT / "tests" / "adversarial"

sys.path.insert(0, str(SRC))

from svp_main import process_file  # type: ignore
from svp_errors import SVPError  # type: ignore


def canonicalize_json_text(text: str) -> str:
    data = json.loads(text)
    return json.dumps(data, ensure_ascii=False, sort_keys=True, indent=2) + "\n"


def assert_deterministic(path: Path, label: str) -> None:
    first = canonicalize_json_text(process_file(str(path)))
    second = canonicalize_json_text(process_file(str(path)))
    if first != second:
        raise AssertionError(f"{label}: lowering no determinista entre dos ejecuciones consecutivas")


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
    assert_deterministic(ADV / "deep_nested_query_valid.svp", "deep_nested_query_valid")


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
