#!/usr/bin/env python3
"""
run_conformance.py — Ejecutor de la suite de conformidad DSL → IR

Para cada caso válido: parsea, valida, baja y comprueba que produce JSON.
Para cada caso inválido: parsea y comprueba que falla con error.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

import os
import sys
import json

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from svp_main import process_file
from svp_errors import SVPError


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
                # Verificar que es JSON válido
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
    print("\n═══ Casos inválidos (deben fallar) ═══")
    if os.path.isdir(invalid_dir):
        for fname in sorted(os.listdir(invalid_dir)):
            if not fname.endswith(".svp"):
                continue
            path = os.path.join(invalid_dir, fname)
            try:
                result = process_file(path)
                # Si llega aquí, no falló — es un error del test
                print(f"  ✗ {fname}: debería haber fallado pero produjo JSON")
                errors.append((fname, "No falló"))
                failed += 1
            except SVPError as e:
                print(f"  ✓ {fname}: {e.error_def.code} ({e.error_def.name})")
                passed += 1
            except Exception as e:
                # Otro error inesperado
                print(f"  ? {fname}: error inesperado — {e}")
                errors.append((fname, str(e)))
                failed += 1

    # ── Resumen ───────────────────────────────────────────────────────
    print(f"\n═══ Resumen ═══")
    print(f"  Pasados: {passed}")
    print(f"  Fallidos: {failed}")
    if errors:
        print(f"\n  Errores:")
        for name, msg in errors:
            print(f"    {name}: {msg}")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(run_tests())
