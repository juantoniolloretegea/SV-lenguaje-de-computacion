#!/usr/bin/env python3
"""Conformidad SV: fuentes DSL frente a esperados comprometidos y rechazo controlado.

El observador no interpreta SV ni genera resultados esperados. El catálogo
conserva las obligaciones diagnósticas; no afirma que la CLI emita esos códigos.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import sys
from oracle_support import (run, assert_success, assert_json_equal, assert_bytes_equal,
                            assert_rust_rejection, RUST_REJECTION_TOKENS,
                            check_invalid_corpus)
from run_row7_transitiondata_conformance import EXPECTED as ROW7_REJECTION_TOKENS

EXPECTED_OBLIGATIONS = {
    "semantic_relation_table_repetida.svp": "Gramática0.2/§14",
    "semantic_relation_constraints_repetida.svp": "Gramática0.2/§14",
    "pattern_arity_repetida.svp": "Gramática0.2/§14",
    "pattern_constraints_repetida.svp": "Gramática0.2/§14",
    "semantic_relation_campos_invertidos.svp": "Gramática0.2/§14",
    "pattern_campos_invertidos.svp": "Gramática0.2/§14",
    "domain_parametro_nominal_repetido.svp": "J-D0/Domain.parameters",
    "horizon_tipo_suceso_repetido.svp": "J-H1/Horizon.events",
    "coupledspec_puente_repetido.svp": "J1.2/BridgeSet",
    "horizon_architecture_ausente.svp": "E006",
    "horizon_architecture_tipo_incorrecto.svp": "E006",
    "agent_arquitecturas_reales_distintas.svp": "E402",
    "output_semantics_sin_celda_repetida.svp": "E115",
    "connector_clave_repetida.svp": "E007",
    "output_semantics_vacia.svp": "E115",
    "output_semantics_clave_ausente.svp": "E115",
    "output_semantics_clave_ajena.svp": "E115",
    "output_semantics_clave_repetida.svp": "E115",

    "identificador_palabra_reservada.svp": "Gramática0.2/§11",
    "identificador_guion_bajo_inicial.svp": "Gramática0.2/§11",
    "identificador_marca_combinante.svp": "Gramática0.2/§11",
    "identificador_alfabeto_fuera_perfil.svp": "Gramática0.2/§11",
    "identificador_latino_fuera_perfil.svp": "Gramática0.2/§11",
    "nat_digitos_no_ascii.svp": "Gramática0.2/§11",
    "admissibility_table_incompleta.svp": "E009",
    "admissibility_table_output_fuera_codominio.svp": "E011",
    "admissibility_spec_estados_legacy.svp": "E110",
    "admissibility_spec_failed_legacy.svp": "E110",
    "admissibility_spec_u_legacy.svp": "E110",
    "bridge_position_fuera_de_rango.svp": "E105",
    "cellstate_vector_length_mismatch.svp": "E101",
    "codomain_miembro_duplicado.svp": "E004",
    "coupledstate_update_fuera_bridges.svp": "E112",
    "edge_position_fuera_bridges.svp": "E113",
    "edge_connector_target_position_mismatch.svp": "E113",
    "edge_connector_source_codomain_mismatch.svp": "E113",
    "transition_event_fuera_horizon.svp": "E307",
    "transition_induced_parameters_vacios.svp": "E406",
    "compose_graph_tipo_incorrecto.svp": "E006/compose(graph):CompositionGraph",
    "frame_architecture_tipo_incorrecto.svp": "E006/Frame.architecture:CompositionGraph",
    "transition_event_tipo_repetido.svp": "H05/TransitionData.events",
    "transition_induced_destino_repetido.svp": "H05/TransitionData.induced_parameters",
    "transition_induced_node_ausente.svp": "H04/NodeId",
    "transition_induced_node_fuera_arquitectura.svp": "H04/NodeId∈Horizon.architecture",
    "transition_induced_node_tipo_incorrecto.svp": "H04/NodeId:CoupledSpec",
    "transition_induced_position_cero.svp": "H04/1≤position≤n",
    "transition_induced_position_fuera_rango.svp": "H04/1≤position≤n",
    "bad_b_value.svp": "E002",
    "conector_mapping_incompleto.svp": "E007",
    "conector_target_no_ternario.svp": "E104",
    "duplicate_identifier.svp": "E005",
    "compose_cycle_graph.svp": "E103",
    "graph_conflicts_fuera_de_v0_1.svp": "Gramática0.1/graph_decl",
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
    "resolve_alias_estado_no_u.svp": "E305",
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

# Los nueve testigos compartidos conservan una sola identidad textual, usada
# tanto por la conformidad general como por la comprobación causal acotada.
RUST_REJECTION_TOKENS.update(
    {Path(name).stem: token for name, token in ROW7_REJECTION_TOKENS.items()}
)

ROOT = Path(__file__).resolve().parents[1]
VALID_DIR = ROOT / "tests" / "conformance" / "valid"
INVALID_DIR = ROOT / "tests" / "conformance" / "invalid"

VALID_CASES = sorted(path.stem for path in VALID_DIR.glob("*.svp"))
INVALID_CASES = sorted(path.stem for path in INVALID_DIR.glob("*.svp"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--rust-bin",
        default=str(ROOT / "rust" / "target" / "debug" / "sv-native"),
    )
    args = parser.parse_args()

    failures: list[str] = []
    check_invalid_corpus(INVALID_DIR.glob("*.svp"))
    if not VALID_CASES:
        raise AssertionError("corpus válido vacío")

    for case in VALID_CASES:
        source = VALID_DIR / f"{case}.svp"
        golden = source.with_suffix(".expected.json")

        rust = run([args.rust_bin, str(source)])
        if rust.returncode != 0:
            failures.append(f"VALID {case}: camino Rust falló: {rust.stderr.strip()}")
            continue

        try:
            assert_success(rust)
            assert_json_equal(rust.stdout, golden.read_bytes())
            repeated = run([args.rust_bin, str(source)])
            assert_success(repeated)
            assert_bytes_equal(rust.stdout, repeated.stdout)
        except AssertionError as exc:
            failures.append(f"VALID {case}: {exc}")
            continue

        print(f"Conformidad SV VALID OK: {case}")

    for case in INVALID_CASES:
        source = INVALID_DIR / f"{case}.svp"
        rust = run([args.rust_bin, str(source)])

        try:
            assert_rust_rejection(rust, case)
        except AssertionError as exc:
            failures.append(f"INVALID {case}: {exc}")
            continue

        print(f"Conformidad SV INVALID OK: {case}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print(
        "Conformidad SV: "
        f"{len(VALID_CASES)}/{len(VALID_CASES)} válidos frente a esperados comprometidos y "
        f"{len(INVALID_CASES)}/{len(INVALID_CASES)} inválidos rechazados sobre el mismo .svp"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
