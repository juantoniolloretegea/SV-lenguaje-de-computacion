"""Oráculos de prueba: bytes, pares JSON y rechazo controlado.

No serializa IR ni corrige las realizaciones. La comparación estructural permite
espacios y escapes JSON equivalentes; conserva orden, multiplicidad y números.
"""
from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class OracleError(AssertionError):
    pass


class DuplicateJsonMember(OracleError):
    pass


def ordered_json(raw: bytes, *, reject_duplicates: bool = True):
    def pairs(entries):
        names = [key for key, _ in entries]
        if reject_duplicates and len(names) != len(set(names)):
            raise DuplicateJsonMember(f"miembro JSON repetido: {names!r}")
        return ("object", tuple(entries))

    def invalid_constant(value):
        raise OracleError(f"constante ajena a JSON: {value}")

    try:
        # Decodificación explícita: no se admite BOM ni otra codificación implícita.
        return json.loads(raw.decode("utf-8"), object_pairs_hook=pairs,
                          parse_int=lambda n: ("number", n),
                          parse_float=lambda n: ("number", n),
                          parse_constant=invalid_constant)
    except (UnicodeError, ValueError) as exc:
        raise OracleError(f"JSON/UTF-8 inválido: {exc}") from exc


def assert_json_equal(left: bytes, right: bytes) -> None:
    if ordered_json(left) != ordered_json(right):
        raise OracleError("divergencia JSON: orden, tipo, número o contenido distinto")


def assert_json_roundtrip(raw: bytes) -> bytes:
    """Recorrido del valor JSON, no serializador de IR ni reparación del programa.

    Conserva tokens numéricos del parser para no estrechar Nat ni redondear.
    Los pares se comprueban antes de producir cualquier mapa.
    """
    parsed = ordered_json(raw)

    def encode(value):
        if isinstance(value, tuple):
            tag, entries = value
            if tag == "object":
                return "{" + ",".join(json.dumps(k, ensure_ascii=False) + ":" + encode(v)
                                       for k, v in entries) + "}"
            if tag == "number":
                return entries
            raise OracleError(f"etiqueta de valor JSON desconocida: {tag}")
        if isinstance(value, list):
            return "[" + ",".join(encode(v) for v in value) + "]"
        return json.dumps(value, ensure_ascii=False, allow_nan=False)

    try:
        repeated = encode(parsed).encode("utf-8")
    except (UnicodeError, ValueError) as exc:
        raise OracleError(f"valor JSON no reproducible: {exc}") from exc
    if ordered_json(repeated) != parsed:
        raise OracleError("pérdida de miembros, tipos, orden o contenido en el recorrido JSON")
    return repeated


def assert_bytes_equal(left: bytes, right: bytes) -> None:
    if left != right:
        raise OracleError("bytes distintos")


def run(command: list[str]) -> subprocess.CompletedProcess[bytes]:
    # El límite pertenece al ejecutor de pruebas, nunca a la semántica del SV.
    return subprocess.run(command, cwd=ROOT, capture_output=True,
                          check=False, timeout=60)


def assert_success(proc: subprocess.CompletedProcess[bytes]) -> None:
    if proc.returncode != 0:
        raise OracleError(f"se esperaba admisión: rc={proc.returncode}; {proc.stderr!r}")
    if proc.stderr:
        raise OracleError(f"admisión con stderr inesperado: {proc.stderr!r}")
    assert_json_roundtrip(proc.stdout)


def cli_payload(raw: bytes) -> bytes:
    """Retira sólo el LF que añade println/print; no recorta el documento."""
    if not raw.endswith(b"\n"):
        raise OracleError("falta el LF final del contrato CLI")
    return raw[:-1]


def diagnostic_text(raw: bytes) -> str:
    try:
        return raw.decode("utf-8")
    except UnicodeError as exc:
        raise OracleError("diagnóstico no UTF-8") from exc


# Identidades textuales observables en frontend.rs/wellformed.rs y sus módulos.
# No se equiparan automáticamente a los códigos del catálogo ni constituyen un nuevo catálogo del núcleo.
RUST_REJECTION_TOKENS = {
    "horizon_architecture_ausente": "referencia no declarada: Missing",
    "horizon_architecture_tipo_incorrecto": "K: se esperaba CompositionGraph",
    "agent_arquitecturas_reales_distintas": "Agent AG: architecture incompatible con Domain",
    "output_semantics_sin_celda_repetida": "E115 (InvalidOutputSemantics): OutputSemantics S: repetidas=[A]",
    "connector_clave_repetida": "Connector Conn: clave duplicada",
    "output_semantics_vacia": "E115 (InvalidOutputSemantics)",
    "output_semantics_clave_ausente": "E115 (InvalidOutputSemantics)",
    "output_semantics_clave_ajena": "E115 (InvalidOutputSemantics)",
    "output_semantics_clave_repetida": "E115 (InvalidOutputSemantics)",

    "admissibility_spec_estados_legacy": 'InvalidAdmissibilityState("Failed")',
    "admissibility_spec_failed_legacy": 'InvalidAdmissibilityState("Failed")',
    "admissibility_spec_u_legacy": 'InvalidAdmissibilityState("U")',
    "admissibility_table_incompleta": "tabla incompleta",
    # Diferencia de fase existente: el cierre interno legado se rechaza al analizar.
    "admissibility_table_output_fuera_codominio": "esperado }, recibido Sym(';')",
    "agent_architecture_mismatch": "architecture incompatible con Domain",
    "bad_b_value": "b debe ser >= 3",
    "bridge_position_fuera_de_rango": "posición puente fuera de rango",
    "cellstate_vector_length_mismatch": "longitud de vector incompatible",
    "codomain_miembro_duplicado": "E004 (InvalidCodomain): codomain KDuplicado repite: APTO",
    "compose_cycle_graph": "ciclo detectado",
    "compose_patterns_vacios": "relations/patterns no puede estar vacío",
    "compose_relations_vacias": "relations/patterns no puede estar vacío",
    "conector_mapping_incompleto": "mapping incompleto o inconsistente",
    "conector_target_no_ternario": 'InvalidTri("APTO")',
    "coupledstate_update_fuera_bridges": "actualización fuera de BridgeSet",
    "domain_chain_mismatch": "parameter_id incompatibles",
    "duplicate_identifier": "identificador duplicado: K3",
    "edge_connector_source_codomain_mismatch": "source_codomain incompatible",
    "edge_connector_target_position_mismatch": "target_position incompatible",
    "edge_position_fuera_bridges": "posición fuera de BridgeSet",
    "frame_criticality_no_producible": "CriticalitiesNotProducible",
    "frame_estado_arquitectura_ajena": "StateOutsideArchitecture",
    "frame_eval_duplicado": "DuplicateEvalSource",
    "frame_eval_externo": "EvalSourceOutsideFrame",
    "frame_gate_input_externo": "GateInputOutsideFrame",
    "frame_supervision_externa": "SupervisionMetaEvalOutsideFrame",
    "gate_codominio_posicional_incompatible_con_tabla": "codominios posicionales incompatibles",
    "gate_input_no_evalresult": "S1 no es un resultado de operación",
    "gate_numero_entradas_incompatible_con_tabla": "número de entradas incompatible",
    "gate_undeclared_input": "referencia no declarada: E999",
    "graph_conflicts_fuera_de_v0_1": 'esperado }, recibido Word(\\"conflicts\\", 255)',
    "graph_simple_concurrencia_mismo_puente": "concurrencia en régimen Simple",
    "identificador_alfabeto_fuera_perfil": "carácter léxico no admitido U+03B1",
    "identificador_guion_bajo_inicial": "carácter léxico no admitido U+005F",
    "identificador_latino_fuera_perfil": "carácter léxico no admitido U+00F6",
    "identificador_marca_combinante": "carácter léxico no admitido U+0303",
    "identificador_palabra_reservada": "palabra protegida donde se esperaba identificador: frame",
    "invalid_role_literal": "rol no reconocido: Experto",
    "invalid_tri_literal": 'InvalidTri("Dos")',
    "max_keyword": "palabra protegida donde se esperaba identificador: max",
    "nat_digitos_no_ascii": "carácter léxico no admitido U+0663",
    "output_semantics_no_declarada": "referencia no declarada: SemK3Ausente",
    "pending_u_reconocido_no_habilitado": "PendingU no habilitado",
    "projection_campo_inexistente": "campo inexistente",
    "projection_source_no_resultado": "K1 no es un resultado de operación",
    "projection_undeclared_source": "referencia no declarada: E999",
    "query_context_opaco": "esperado (, recibido Sym(')')",
    "query_context_type_mismatch": "QueryContext incompatible con QuerySpec",
    "resolve_alias_estado_no_u": "TargetIsNotU",
    "resolve_instancia_incompatible": "ContextMismatch",
    "resolve_missing_context": "esperado context, recibido mechanism",
    "resolve_missing_mechanism": "esperado ,, recibido Sym(')')",
    "resolve_target_fuera_rango": "PositionOutOfRange",
    "resolve_target_no_u": "TargetIsNotU",
    "supervise_celltarget_tipo_incorrecto": "CellTarget de tipo incorrecto",
    "supervise_composedtarget_tipo_incorrecto": "ComposedTarget de tipo incorrecto",
    "supervise_coupled_wrong_role": "meta_eval no procede de Supervisor",
    "supervise_meta_no_evalresult": "SMeta no es un resultado de operación",
    "supervise_systemtarget_tipo_incorrecto": "EBase no es un objeto declarado",
    "supervise_target_opaco": "esperado (, recibido Sym(')')",
    "supervise_undeclared_target": "referencia no declarada: E999",
    "supervise_wrong_role": "meta_eval no procede de Supervisor",
    "trajectory_alternance_violation": "entrada no final sin transición",
    "transition_data_horizon_no_declarado": "referencia no declarada: H_ausente",
    "transition_event_fuera_horizon": "suceso fuera del Horizon",
    "transition_induced_parameters_vacios": "induced_parameters vacío",
    "u_coercion": 'InvalidTri("null")',
}


def assert_rejection(proc, token: str) -> str:
    if proc.returncode != 1 or proc.stdout:
        raise OracleError(f"rechazo Rust inválido: rc={proc.returncode}; stdout={proc.stdout!r}")
    text = diagnostic_text(proc.stderr)
    # Un pánico (101), señal, fallo de carga (2), vacío o excepción no satisfacen esto.
    if not re.fullmatch(r'SVP no admitido: (?:InvalidProgram\("[^\n]+"\)|Frontend\([^\n]+\))\n', text):
        raise OracleError(f"no es un rechazo CompileError controlado: {text!r}")
    if token not in text:
        raise OracleError(f"falta identidad textual {token!r}: {text!r}")
    return text[len("SVP no admitido: "):-1]


def assert_rust_rejection(proc, case: str) -> str:
    return assert_rejection(proc, RUST_REJECTION_TOKENS[case])


def check_invalid_corpus(paths) -> None:
    from run_conformance import EXPECTED_CATALOG_CODES
    names = {path.name for path in paths}
    if not names or names != set(EXPECTED_CATALOG_CODES):
        raise OracleError("corpus inválido vacío o distinto del catálogo de obligaciones")
    if {Path(name).stem for name in names} != set(RUST_REJECTION_TOKENS):
        raise OracleError("corpus inválido distinto de las identidades textuales Rust")
