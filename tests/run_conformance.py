#!/usr/bin/env python3
"""
run_conformance.py — Ejecutor de la suite de conformidad DSL → IR

Para cada caso válido: parsea, valida, baja y comprueba que produce JSON.
Para cada caso inválido: parsea y comprueba que falla con error.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from svp_errors import SVPError
from svp_main import process_file


EXPECTED_INVALID_CODES = {
    "bad_b_value.svp": "E002",
    "gate_undeclared_input.svp": "E006",
    "max_keyword.svp": "E210",
    "projection_undeclared_source.svp": "E006",
    "supervise_undeclared_target.svp": "E006",
    "supervise_wrong_role.svp": "E211",
    "u_coercion.svp": "E001",
}


def run_tests():
    base = os.path.dirname(os.path.abspath(__file__))
    valid_dir = os.path.join(base, "conformance", "valid")
    invalid_dir = os.path.join(base, "conformance", "invalid")

    passed = 0
    failed = 0
    errors = []

    # ── Casos válidos ─────────────────────────────────────────────────
    print("═══ Casos válidos ═══")
    if os.path.isdir(valid_dir):
        for fname in sorted(os.listdir(valid_dir)):
            if not fname.endswith(".svp"):
                continue
            path = os.path.join(valid_dir, fname)
            try:
                result = process_file(path)
                doc = json.loads(result)
                assert "ir_version" in doc
                assert "grammar_version" in doc
                assert "source_sha256" in doc
                assert "serializer_version" in doc
                assert doc["ir_version"] == "0.2"
                assert doc["grammar_version"] == "0.1"
                print(f"  ✓ {fname}")
                passed += 1
            except Exception as e:
                print(f"  ✗ {fname}: {e}")
                errors.append((fname, str(e)))
                failed += 1

    # ── Casos inválidos ───────────────────────────────────────────────
    print("\n═══ Casos inválidos (deben fallar con código exacto) ═══")
    if os.path.isdir(invalid_dir):
        for fname in sorted(os.listdir(invalid_dir)):
            if not fname.endswith(".svp"):
                continue
            path = os.path.join(invalid_dir, fname)
            expected_code = EXPECTED_INVALID_CODES.get(fname)
            try:
                process_file(path)
                print(f"  ✗ {fname}: debería haber fallado pero produjo JSON")
                errors.append((fname, "No falló"))
                failed += 1
            except SVPError as e:
                actual_code = e.error_def.code
                if expected_code is None:
                    print(f"  ✗ {fname}: no tiene código esperado declarado y falló con {actual_code}")
                    errors.append((fname, f"Sin código esperado declarado: {actual_code}"))
                    failed += 1
                elif actual_code != expected_code:
                    print(f"  ✗ {fname}: código esperado {expected_code}, obtenido {actual_code}")
                    errors.append((fname, f"Esperado {expected_code}, obtenido {actual_code}"))
                    failed += 1
                else:
                    print(f"  ✓ {fname}: {actual_code} ({e.error_def.name})")
                    passed += 1
            except Exception as e:
                print(f"  ? {fname}: error inesperado — {e}")
                errors.append((fname, str(e)))
                failed += 1

    # ── Resumen ───────────────────────────────────────────────────────
    print("\n═══ Resumen ═══")
    print(f"  Pasados: {passed}")
    print(f"  Fallidos: {failed}")
    if errors:
        print("\n  Errores:")
        for name, msg in errors:
            print(f"    {name}: {msg}")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(run_tests())
