#!/usr/bin/env python3
"""run_conformance.py — Ejecutor de la suite de conformidad DSL → IR

Para cada caso válido:
parsea, valida, baja y compara el JSON canónico producido con su
`.expected.json` correspondiente.

Para cada caso inválido:
parsea y comprueba que falla con el código exacto esperado.

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
    "admissibility_table_incompleta.svp": "E009",
    "bridge_position_fuera_de_rango.svp": "E105",
    "cellstate_vector_length_mismatch.svp": "E101",
    "bad_b_value.svp": "E002",
    "conector_mapping_incompleto.svp": "E007",
    "conector_target_no_ternario.svp": "E104",
    "duplicate_identifier.svp": "E005",
    "compose_cycle_graph.svp": "E103",
    "gate_input_no_evalresult.svp": "E202",
    "gate_undeclared_input.svp": "E006",
    "invalid_role_literal.svp": "E010",
    "invalid_tri_literal.svp": "E001",
    "max_keyword.svp": "E210",
    "projection_undeclared_source.svp": "E006",
    "query_context_opaco.svp": "E204",
    "supervise_target_opaco.svp": "E205",
    "supervise_undeclared_target.svp": "E006",
    "supervise_wrong_role.svp": "E211",
    "trajectory_alternance_violation.svp": "E304",
    "u_coercion.svp": "E507",
    "domain_chain_mismatch.svp": "E401",
    "agent_architecture_mismatch.svp": "E402",
    "query_context_type_mismatch.svp": "E403",
    "output_semantics_no_declarada.svp": "E102",
    "compose_relations_vacias.svp": "E208",
    "compose_patterns_vacios.svp": "E209",
    "pending_u_reconocido_no_habilitado.svp": "E403",
    "transition_data_horizon_no_declarado.svp": "E303",
}


def canonicalize_json_text(text: str) -> str:
    """Normaliza texto JSON a una forma canónica estable."""
    data = json.loads(text)
    return json.dumps(data, ensure_ascii=False, sort_keys=True, indent=2) + "\n"


def expected_json_path(valid_dir: str, fname: str) -> str:
    stem, _ = os.path.splitext(fname)
    return os.path.join(valid_dir, f"{stem}.expected.json")


def run_tests():
    base = os.path.dirname(os.path.abspath(__file__))
    valid_dir = os.path.join(base, "conformance", "valid")
    invalid_dir = os.path.join(base, "conformance", "invalid")

    passed = 0
    failed = 0
    errors = []

    # ── Casos válidos ──────────────────────────────────────────────────
    print("═══ Casos válidos ═══")
    if os.path.isdir(valid_dir):
        for fname in sorted(os.listdir(valid_dir)):
            if not fname.endswith(".svp"):
                continue

            path = os.path.join(valid_dir, fname)
            exp_path = expected_json_path(valid_dir, fname)

            try:
                if not os.path.exists(exp_path):
                    raise FileNotFoundError(
                        f"Falta expected JSON para {fname}: {os.path.basename(exp_path)}"
                    )

                result = process_file(path)
                doc = json.loads(result)

                assert "ir_version" in doc
                assert "grammar_version" in doc
                assert "source_sha256" in doc
                assert "serializer_version" in doc
                assert doc["ir_version"] == "0.2"
                assert doc["grammar_version"] == "0.1"

                produced = canonicalize_json_text(result)
                with open(exp_path, "r", encoding="utf-8") as fh:
                    expected = canonicalize_json_text(fh.read())

                if produced != expected:
                    raise AssertionError(
                        f"JSON canónico distinto del expected: {os.path.basename(exp_path)}"
                    )

                print(f" ✓ {fname}")
                passed += 1

            except Exception as e:
                print(f" ✗ {fname}: {e}")
                errors.append((fname, str(e)))
                failed += 1

    # ── Casos inválidos ────────────────────────────────────────────────
    print("\n═══ Casos inválidos (deben fallar con código exacto) ═══")
    if os.path.isdir(invalid_dir):
        for fname in sorted(os.listdir(invalid_dir)):
            if not fname.endswith(".svp"):
                continue

            path = os.path.join(invalid_dir, fname)
            expected_code = EXPECTED_INVALID_CODES.get(fname)

            try:
                process_file(path)
                print(f" ✗ {fname}: debería haber fallado pero produjo JSON")
                errors.append((fname, "No falló"))
                failed += 1

            except SVPError as e:
                actual_code = e.error_def.code

                if expected_code is None:
                    print(
                        f" ✗ {fname}: no tiene código esperado declarado y falló con {actual_code}"
                    )
                    errors.append(
                        (fname, f"Sin código esperado declarado: {actual_code}")
                    )
                    failed += 1

                elif actual_code != expected_code:
                    print(
                        f" ✗ {fname}: código esperado {expected_code}, obtenido {actual_code}"
                    )
                    errors.append(
                        (fname, f"Esperado {expected_code}, obtenido {actual_code}")
                    )
                    failed += 1

                else:
                    print(f" ✓ {fname}: {actual_code} ({e.error_def.name})")
                    passed += 1

            except Exception as e:
                print(f" ? {fname}: error inesperado — {e}")
                errors.append((fname, str(e)))
                failed += 1

    # ── Resumen ────────────────────────────────────────────────────────
    print("\n═══ Resumen ═══")
    print(f" Pasados: {passed}")
    print(f" Fallidos: {failed}")

    if errors:
        print("\n Errores:")
        for name, msg in errors:
            print(f" {name}: {msg}")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(run_tests())
