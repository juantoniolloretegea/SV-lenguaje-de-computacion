#!/usr/bin/env python3
"""R0 — paridad ejecutada SV en nativo / WebAssembly WASI.

La vía WebAssembly ejecuta el mismo `sv-native` compilado a `wasm32-wasip1`.
El host WASI sólo proporciona infraestructura de ejecución; no interpreta SVP.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from typing import Any

from oracle_support import (run, assert_success, assert_json_equal, assert_bytes_equal,
                            assert_rust_rejection,
                            check_invalid_corpus)

ROOT = Path(__file__).resolve().parents[1]
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
INVALID_DIR = ROOT / "tests" / "conformance" / "invalid"


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def version(command: list[str]) -> str:
    completed = run(command)
    if completed.returncode != 0:
        return f"ERROR({completed.returncode}): {completed.stderr.strip()}"
    return (completed.stdout.strip() or completed.stderr.strip()).decode("utf-8")


def case_record(
    name: str,
    category: str,
    native: subprocess.CompletedProcess[bytes],
    wasm: subprocess.CompletedProcess[bytes],
) -> dict[str, Any]:
    return {
        "case": name,
        "category": category,
        "streams_base64": {
            emitter: {stream: base64.b64encode(getattr(proc, stream)).decode("ascii")
                      for stream in ["stdout", "stderr"]}
            for emitter, proc in [("native", native), ("wasm", wasm)]
        },
        "commands": {"native": native.args, "wasm": wasm.args},
        "returncodes": {
            "native": native.returncode,
            "wasm": wasm.returncode,
        },
        "stderr_sha256": {
            "native": sha256_bytes(native.stderr),
            "wasm": sha256_bytes(wasm.stderr),
        },
        "stdout_sha256": {
            "native": sha256_bytes(native.stdout),
            "wasm": sha256_bytes(wasm.stdout),
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
    check_invalid_corpus(invalid_cases)
    if not valid_cases:
        raise AssertionError("corpus válido vacío")

    for source in valid_cases:
        golden = source.with_suffix(".expected.json")
        if not golden.is_file():
            failures.append(f"VALID {source.stem}: falta golden comprometido")
            continue

        native = run([str(native_bin), str(source)])
        wasm = run(["node", "--no-warnings", str(wasi_runner), str(wasm_bin), str(source)])
        records.append(case_record(source.stem, "valid", native, wasm))

        if native.returncode != 0:
            failures.append(f"VALID {source.stem}: nativo falló: {native.stderr.strip()}")
            continue
        if wasm.returncode != 0:
            failures.append(f"VALID {source.stem}: WebAssembly falló: {wasm.stderr.strip()}")
            continue

        try:
            for result in [native, wasm]:
                assert_success(result)
            assert_json_equal(native.stdout, golden.read_bytes())
            assert_bytes_equal(wasm.stdout, native.stdout)
        except AssertionError as exc:
            failures.append(f"VALID {source.stem}: {exc}")
            continue

        print(f"R0 WASM VALID OK: {source.stem}")

    for source in invalid_cases:
        native = run([str(native_bin), str(source)])
        wasm = run(["node", "--no-warnings", str(wasi_runner), str(wasm_bin), str(source)])
        records.append(case_record(source.stem, "invalid", native, wasm))

        try:
            assert_rust_rejection(native, source.stem)
            assert_rust_rejection(wasm, source.stem)
            assert_bytes_equal(wasm.stderr, native.stderr)
        except AssertionError as exc:
            failures.append(f"INVALID {source.stem}: {exc}")
            continue

        print(f"R0 WASM INVALID OK: {source.stem}")

    report: dict[str, Any] = {
        "schema": "sv-native-wasi-parity-v3",
        "scope": "same-svp-native-wasi",
        "diagnostic_parity": "catalog-obligations; native-text-identity; native-wasi-stderr-bytes",
        "complete_catalog_diagnostic_conformance": "not-proven-DFL-001",
        "json_comparison": "ordered-pairs; duplicate-members-rejected; exact-numbers",
        "byte_comparison": "native-wasi-stdout; raw-stream-sha256",
        "wasi_warning_policy": "node --no-warnings; errors remain on stderr",
        "source_head": args.source_head,
        "base_head": args.base_head,
        "artifacts": {
            "native_binary": {
                "path": str(native_bin),
                "bytes": native_bin.stat().st_size,
                "sha256": sha256_file(native_bin),
            },
            "wasm_module": {
                "path": str(wasm_bin),
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
        f"{len(valid_cases)}/{len(valid_cases)} válidos con paridad nativa/WASI y "
        f"{len(invalid_cases)}/{len(invalid_cases)} inválidos rechazados por ambos destinos"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
