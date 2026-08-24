#!/usr/bin/env python3
"""R0 — paridad ejecutada Python / Rust nativo / Rust-WebAssembly.

La vía WebAssembly ejecuta el mismo `sv-native` compilado a `wasm32-wasip1`.
El host WASI sólo proporciona infraestructura de ejecución; no interpreta SVP.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
INVALID_DIR = ROOT / "tests" / "conformance" / "invalid"


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


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


def version(command: list[str]) -> str:
    completed = run(command)
    if completed.returncode != 0:
        return f"ERROR({completed.returncode}): {completed.stderr.strip()}"
    return completed.stdout.strip() or completed.stderr.strip()


def case_record(
    name: str,
    category: str,
    py: subprocess.CompletedProcess[str],
    native: subprocess.CompletedProcess[str],
    wasm: subprocess.CompletedProcess[str],
) -> dict[str, Any]:
    return {
        "case": name,
        "category": category,
        "returncodes": {
            "python": py.returncode,
            "native": native.returncode,
            "wasm": wasm.returncode,
        },
        "stdout_sha256": {
            "python": sha256_bytes(py.stdout.encode("utf-8")),
            "native": sha256_bytes(native.stdout.encode("utf-8")),
            "wasm": sha256_bytes(wasm.stdout.encode("utf-8")),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--native-bin", required=True, type=Path)
    parser.add_argument("--wasm-bin", required=True, type=Path)
    parser.add_argument("--wasi-runner", required=True, type=Path)
    parser.add_argument("--report", type=Path)
    parser.add_argument("--source-head")
    parser.add_argument("--base-head")
    args = parser.parse_args()

    native_bin = args.native_bin.resolve()
    wasm_bin = args.wasm_bin.resolve()
    wasi_runner = args.wasi_runner.resolve()
    for path in [native_bin, wasm_bin, wasi_runner]:
        if not path.is_file():
            parser.error(f"archivo inexistente: {path}")

    valid_cases = sorted(VALID_DIR.glob("*.svp"))
    invalid_cases = sorted(INVALID_DIR.glob("*.svp"))
    failures: list[str] = []
    records: list[dict[str, Any]] = []

    for source in valid_cases:
        golden = source.with_suffix(".expected.json")
        if not golden.is_file():
            failures.append(f"VALID {source.stem}: falta golden comprometido")
            continue

        py = run([sys.executable, "src/svp_main.py", str(source)])
        native = run([str(native_bin), str(source)])
        wasm = run(["node", str(wasi_runner), str(wasm_bin), str(source)])
        records.append(case_record(source.stem, "valid", py, native, wasm))

        if py.returncode != 0:
            failures.append(f"VALID {source.stem}: Python falló: {py.stderr.strip()}")
            continue
        if native.returncode != 0:
            failures.append(f"VALID {source.stem}: nativo falló: {native.stderr.strip()}")
            continue
        if wasm.returncode != 0:
            failures.append(f"VALID {source.stem}: WebAssembly falló: {wasm.stderr.strip()}")
            continue

        reference = canonical_json(py.stdout)
        expected = canonical_json(golden.read_text(encoding="utf-8"))
        sovereign = canonical_json(native.stdout)
        webassembly = canonical_json(wasm.stdout)

        if reference != expected:
            failures.append(
                f"VALID {source.stem}: Python ya no coincide con el golden comprometido"
            )
            continue
        if sovereign != reference:
            failures.append(
                f"VALID {source.stem}: divergencia nativo/Python\n"
                f"  Python: {reference}\n"
                f"  Nativo: {sovereign}"
            )
            continue
        if webassembly != sovereign:
            failures.append(
                f"VALID {source.stem}: divergencia WebAssembly/nativo\n"
                f"  Nativo: {sovereign}\n"
                f"  WASM:   {webassembly}"
            )
            continue

        print(f"R0 WASM VALID OK: {source.stem}")

    for source in invalid_cases:
        py = run([sys.executable, "src/svp_main.py", str(source)])
        native = run([str(native_bin), str(source)])
        wasm = run(["node", str(wasi_runner), str(wasm_bin), str(source)])
        records.append(case_record(source.stem, "invalid", py, native, wasm))

        if py.returncode == 0:
            failures.append(f"INVALID {source.stem}: Python aceptó el caso")
            continue
        if native.returncode == 0:
            failures.append(f"INVALID {source.stem}: nativo aceptó el caso")
            continue
        if wasm.returncode == 0:
            failures.append(f"INVALID {source.stem}: WebAssembly aceptó el caso")
            continue

        print(f"R0 WASM INVALID OK: {source.stem}")

    report: dict[str, Any] = {
        "schema": "sv-r0-wasm-three-way-parity-v1",
        "scope": "same-svp-python-native-wasm",
        "diagnostic_parity": "not-proven",
        "source_head": args.source_head,
        "base_head": args.base_head,
        "artifacts": {
            "native_binary": {
                "path": str(native_bin.relative_to(ROOT)),
                "bytes": native_bin.stat().st_size,
                "sha256": sha256_file(native_bin),
            },
            "wasm_module": {
                "path": str(wasm_bin.relative_to(ROOT)),
                "bytes": wasm_bin.stat().st_size,
                "sha256": sha256_file(wasm_bin),
                "target": "wasm32-wasip1",
            },
        },
        "toolchain": {
            "rustc": version(["rustc", "--version"]),
            "cargo": version(["cargo", "--version"]),
            "node": version(["node", "--version"]),
            "python_orchestrator": sys.version.split()[0],
        },
        "counts": {
            "valid": len(valid_cases),
            "invalid": len(invalid_cases),
        },
        "cases": records,
        "failures": failures,
    }

    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(
            json.dumps(report, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print(
        "R0 WASM: "
        f"{len(valid_cases)}/{len(valid_cases)} válidos con paridad de tres vías y "
        f"{len(invalid_cases)}/{len(invalid_cases)} inválidos rechazados por las tres vías"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
