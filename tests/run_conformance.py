#!/usr/bin/env python3
"""run_conformance.py — Ejecutor de la batería de conformidad DSL → IR.

Los casos válidos se comparan contra JSON esperados comprometidos en el
repositorio. Los casos inválidos deben terminar con el código diagnóstico
exacto declarado. El ejecutor no regenera ni modifica los oráculos.
"""

import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from svp_errors import SVPError
from svp_main import process_file

IR_VERSION = "0.3"
GRAMMAR_VERSION = "0.2"
SERIALIZER_VERSION = "0.1.0"

EXPECTED_INVALID_CODES = {
    "admissibility_table_incompleta.svp": "E009",
    "admissibility_table_output_fuera_codominio.svp": "E011",
    "admissibility_spec_estados_legacy.svp": "E110",
    "bridge_position_fuera_de_rango.svp": "E105",
    "cellstate_vector_length_mismatch.svp": "E101",
    "coupledstate_update_fuera_bridges.svp": "E112",
    "edge_position_fuera_bridges.svp": "E113",
    "edge_connector_target_position_mismatch.svp": "E113",
    "edge_connector_source_codomain_mismatch.svp": "E113",
    "transition_event_fuera_horizon.svp": "E307",
    "transition_induced_parameters_vacios.svp": "E406",
    "bad_b_value.svp": "E002",
    "conector_mapping_incompleto.svp": "E007",
    "conector_target_no_ternario.svp": "E104",
    "duplicate_identifier.svp": "E005",
    "compose_cycle_graph.svp": "E103",
    "graph_conflicts_fuera_de_v0_1.svp": "E001",
    "graph_simple_concurrencia_mismo_puente.svp": "E114",
    "gate_input_no_evalresult.svp": "E202",
    "gate_undeclared_input.svp": "E006",
    "gate_numero_entradas_incompatible_con_tabla.svp": "E215",
    "gate_codominio_posicional_incompatible_con_tabla.svp": "E215",
    "invalid_role_literal.svp": "E010",
    "invalid_tri_literal.svp": "E001",
    "max_keyword.svp": "E210",
    "projection_undeclared_source.svp": "E006",
    "projection_source_no_resultado.svp": "E213",
    "projection_campo_inexistente.svp": "E214",
    "query_context_opaco.svp": "E204",
    "resolve_missing_context.svp": "E206",
    "resolve_missing_mechanism.svp": "E207",
    "resolve_target_no_u.svp": "E305",
    "resolve_target_fuera_rango.svp": "E305",
    "resolve_instancia_incompatible.svp": "E305",
    "supervise_target_opaco.svp": "E205",
    "supervise_undeclared_target.svp": "E006",
    "supervise_wrong_role.svp": "E211",
    "supervise_meta_no_evalresult.svp": "E212",
    "supervise_coupled_wrong_role.svp": "E211",
    "supervise_celltarget_tipo_incorrecto.svp": "E006",
    "supervise_composedtarget_tipo_incorrecto.svp": "E006",
    "supervise_systemtarget_tipo_incorrecto.svp": "E006",
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
    "frame_estado_arquitectura_ajena.svp": "E308",
    "frame_eval_externo.svp": "E308",
    "frame_eval_duplicado.svp": "E308",
    "frame_gate_input_externo.svp": "E308",
    "frame_supervision_externa.svp": "E308",
    "frame_criticality_no_producible.svp": "E308",
}


def canonicalize_json_text(text: str) -> str:
    data = json.loads(text)
    return json.dumps(data, ensure_ascii=False, sort_keys=True, indent=2) + "\n"


def expected_json_path(valid_dir: str, fname: str) -> str:
    stem, _ = os.path.splitext(fname)
    return os.path.join(valid_dir, f"{stem}.expected.json")


def run_tests() -> int:
    base = os.path.dirname(os.path.abspath(__file__))
    valid_dir = os.path.join(base, "conformance", "valid")
    invalid_dir = os.path.join(base, "conformance", "invalid")

    passed = 0
    failed = 0
    errors = []

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

                assert doc.get("ir_version") == IR_VERSION
                assert doc.get("grammar_version") == GRAMMAR_VERSION
                assert "source_sha256" in doc
                assert doc.get("serializer_version") == SERIALIZER_VERSION

                produced = canonicalize_json_text(result)
                with open(exp_path, "r", encoding="utf-8") as fh:
                    expected = canonicalize_json_text(fh.read())

                if produced != expected:
                    raise AssertionError(
                        f"JSON canónico distinto del expected: {os.path.basename(exp_path)}"
                    )

                print(f" ✓ {fname}")
                passed += 1

            except Exception as exc:
                print(f" ✗ {fname}: {exc}")
                errors.append((fname, str(exc)))
                failed += 1

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

            except SVPError as exc:
                actual_code = exc.error_def.code
                if expected_code is None:
                    print(f" ✗ {fname}: sin código esperado; obtuvo {actual_code}")
                    errors.append((fname, f"Sin código esperado: {actual_code}"))
                    failed += 1
                elif actual_code != expected_code:
                    print(f" ✗ {fname}: esperado {expected_code}, obtenido {actual_code}")
                    errors.append((fname, f"Esperado {expected_code}, obtenido {actual_code}"))
                    failed += 1
                else:
                    print(f" ✓ {fname}: {actual_code} ({exc.error_def.name})")
                    passed += 1

            except Exception as exc:
                print(f" ? {fname}: error inesperado — {exc}")
                errors.append((fname, str(exc)))
                failed += 1

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
