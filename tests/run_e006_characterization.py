#!/usr/bin/env python3
"""Caracterización observable de E006 sin modificar el contrato diagnóstico.

Comprueba dos situaciones distintas que actualmente comparten E006:
1) referencia inexistente;
2) referencia existente de tipo incompatible.

La prueba no altera el validador ni atribuye a E006 un significado nuevo.
"""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
INVALID = ROOT / "tests" / "conformance" / "invalid"

sys.path.insert(0, str(SRC))

from svp_errors import SVPError  # type: ignore
from svp_main import process_file  # type: ignore


CASES = [
    (
        "referencia inexistente",
        "supervise_undeclared_target.svp",
        "Referencia no declarada: 'E999'",
    ),
    (
        "CellTarget con tipo incompatible",
        "supervise_celltarget_tipo_incorrecto.svp",
        "'G1' es GateCmd, se esperaba EvalCmd",
    ),
    (
        "ComposedTarget con tipo incompatible",
        "supervise_composedtarget_tipo_incorrecto.svp",
        "'EBase' es EvalCmd, se esperaba GateCmd",
    ),
    (
        "SystemTarget con tipo incompatible",
        "supervise_systemtarget_tipo_incorrecto.svp",
        "'EBase' es EvalCmd, se esperaba GraphDecl",
    ),
]


def run_case(label: str, filename: str, expected_detail: str) -> None:
    path = INVALID / filename
    try:
        process_file(str(path))
    except SVPError as exc:
        if exc.error_def.code != "E006":
            raise AssertionError(
                f"{label}: se esperaba E006, obtenido {exc.error_def.code}"
            )
        if exc.detail != expected_detail:
            raise AssertionError(
                f"{label}: detalle distinto; esperado {expected_detail!r}, "
                f"obtenido {exc.detail!r}"
            )
        return
    raise AssertionError(f"{label}: el caso fue aceptado y debía emitir E006")


def main() -> int:
    passed = 0
    for label, filename, expected_detail in CASES:
        try:
            run_case(label, filename, expected_detail)
            print(f"[OK] {label}")
            passed += 1
        except Exception as exc:
            print(f"[FAIL] {label}: {exc}", file=sys.stderr)
            return 1

    print(f"Caracterización E006: {passed}/{len(CASES)} comprobaciones superadas.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
