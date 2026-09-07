#!/usr/bin/env python3
"""Prepara el manifiesto de paridad para el destino WebAssembly de navegador.

El manifiesto contiene el mismo texto `.svp` y, para válidos, el stdout exacto
producido por el binario Rust nativo después de comprobar la igualdad estructural
con pares JSON ordenados entre el esperado comprometido y la realización nativa del SV. El
navegador recibe el texto SVP; nunca recibe IR preconstituida como entrada del
módulo WebAssembly.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys
from run_oracle_sensitivity import sources as sensitivity_sources, verify as verify_sensitivity

from oracle_support import (run, assert_success, assert_json_equal,
                            assert_rust_rejection,
                            check_invalid_corpus, cli_payload)

ROOT = Path(__file__).resolve().parents[1]
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
INVALID_DIR = ROOT / "tests" / "conformance" / "invalid"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--native-bin", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--source-head")
    parser.add_argument("--base-head")
    args = parser.parse_args()

    native_bin = args.native_bin.resolve()
    if not native_bin.is_file():
        parser.error(f"binario inexistente: {native_bin}")

    failures: list[str] = []
    cases: list[dict[str, object]] = []
    check_invalid_corpus(INVALID_DIR.glob("*.svp"))
    if not list(VALID_DIR.glob("*.svp")):
        raise AssertionError("corpus válido vacío")

    for source in sorted(VALID_DIR.glob("*.svp")):
        golden = source.with_suffix(".expected.json")
        native = run([str(native_bin), str(source)])

        if native.returncode != 0:
            failures.append(f"VALID {source.stem}: nativo falló: {native.stderr.strip()}")
            continue
        if not golden.is_file():
            failures.append(f"VALID {source.stem}: falta golden")
            continue

        try:
            assert_success(native)
            assert_json_equal(native.stdout, golden.read_bytes())
        except AssertionError as exc:
            failures.append(f"VALID {source.stem}: {exc}")
            continue

        cases.append(
            {
                "name": source.stem,
                "file_name": source.name,
                "category": "valid",
                "source": source.read_bytes().decode("utf-8"),
                "expected_payload": cli_payload(native.stdout).decode("utf-8"),
            }
        )

    for source in sorted(INVALID_DIR.glob("*.svp")):
        native = run([str(native_bin), str(source)])
        try:
            diagnostic = assert_rust_rejection(native, source.stem)
        except AssertionError as exc:
            failures.append(f"INVALID {source.stem}: {exc}")
            continue

        cases.append(
            {
                "name": source.stem,
                "file_name": source.name,
                "category": "invalid",
                "source": source.read_bytes().decode("utf-8"),
                "expected_diagnostic": diagnostic,
            }
        )

    # Mismas cinco fuentes del banco, comprobadas antes de recibir paridad.
    sensitivity_cases = []
    source_dir = args.output.parent / 'sensitivity-sources'
    source_dir.mkdir(parents=True, exist_ok=True)
    for name, raw in sensitivity_sources().items():
        source = source_dir / f'{name}.svp'
        source.write_bytes(raw)
        native = run([str(native_bin), str(source.resolve())])
        try:
            verify_sensitivity(name, raw, native)
            if native.returncode == 0:
                payload = cli_payload(native.stdout).decode('utf-8')
            else:
                payload = assert_rust_rejection(native, 'output_semantics_clave_repetida')
            sensitivity_cases.append({'name': name, 'file_name': source.name,
                'source': raw.decode('utf-8'), 'error': native.returncode == 1,
                'expected_payload': payload})
        except (AssertionError, ValueError) as exc:
            failures.append(f'SENSITIVITY {name}: {exc}')

    result = {
        "schema": "sv-r0-browser-parity-manifest-v3",
        "source_head": args.source_head,
        "base_head": args.base_head,
        "rule": "browser receives source text only; expected output is test oracle only",
        "counts": {
            "valid": sum(case["category"] == "valid" for case in cases),
            "invalid": sum(case["category"] == "invalid" for case in cases),
        },
        "cases": cases,
        "sensitivity_cases": sensitivity_cases,
        "failures": failures,
    }

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(result, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print(
        "R0 browser manifest: "
        f"{result['counts']['valid']} válidos y {result['counts']['invalid']} inválidos"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
