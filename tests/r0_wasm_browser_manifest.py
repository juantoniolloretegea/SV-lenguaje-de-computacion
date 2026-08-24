#!/usr/bin/env python3
"""Prepara el manifiesto de paridad para el destino WebAssembly de navegador.

El manifiesto contiene el mismo texto `.svp` y, para válidos, el stdout exacto
producido por el binario Rust nativo después de comprobar que Python = golden =
nativo. El navegador recibe el texto SVP; nunca recibe IR preconstituida como
entrada del módulo WebAssembly.
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

    for source in sorted(VALID_DIR.glob("*.svp")):
        golden = source.with_suffix(".expected.json")
        py = run([sys.executable, "src/svp_main.py", str(source)])
        native = run([str(native_bin), str(source)])

        if py.returncode != 0:
            failures.append(f"VALID {source.stem}: Python falló: {py.stderr.strip()}")
            continue
        if native.returncode != 0:
            failures.append(f"VALID {source.stem}: nativo falló: {native.stderr.strip()}")
            continue
        if not golden.is_file():
            failures.append(f"VALID {source.stem}: falta golden")
            continue

        reference = canonical_json(py.stdout)
        expected = canonical_json(golden.read_text(encoding="utf-8"))
        sovereign = canonical_json(native.stdout)
        if reference != expected or sovereign != reference:
            failures.append(f"VALID {source.stem}: Python/golden/nativo divergen")
            continue

        cases.append(
            {
                "name": source.stem,
                "file_name": source.name,
                "category": "valid",
                "source": source.read_text(encoding="utf-8"),
                "expected_stdout": native.stdout,
            }
        )

    for source in sorted(INVALID_DIR.glob("*.svp")):
        py = run([sys.executable, "src/svp_main.py", str(source)])
        native = run([str(native_bin), str(source)])
        if py.returncode == 0:
            failures.append(f"INVALID {source.stem}: Python aceptó el caso")
            continue
        if native.returncode == 0:
            failures.append(f"INVALID {source.stem}: nativo aceptó el caso")
            continue

        cases.append(
            {
                "name": source.stem,
                "file_name": source.name,
                "category": "invalid",
                "source": source.read_text(encoding="utf-8"),
                "expected_stdout": None,
            }
        )

    result = {
        "schema": "sv-r0-browser-parity-manifest-v1",
        "source_head": args.source_head,
        "base_head": args.base_head,
        "rule": "browser receives source text only; expected output is test oracle only",
        "counts": {
            "valid": sum(case["category"] == "valid" for case in cases),
            "invalid": sum(case["category"] == "invalid" for case in cases),
        },
        "cases": cases,
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
