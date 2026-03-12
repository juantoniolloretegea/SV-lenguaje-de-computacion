"""
svp_ast.py — Definiciones del árbol sintáctico abstracto del lenguaje SV

Un nodo por cada declaración y comando de la Gramática superficial mínima v0.1.
Los nodos son inmutables (frozen dataclass).

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

from dataclasses import dataclass, field
from typing import List, Optional, Tuple


# ── Primitivas ────────────────────────────────────────────────────────

@dataclass(frozen=True)
class Loc:
    """Posición en el archivo fuente."""
    line: int
    col: int


@dataclass(frozen=True)
class TriLiteral:
    value: str  # "Zero" | "One" | "U"
    loc: Loc


# ── N0 — Definición ──────────────────────────────────────────────────

@dataclass(frozen=True)
class CodomainDecl:
    name: str
    values: List[str]
    loc: Loc


@dataclass(frozen=True)
class OutputSemanticsDecl:
    name: str
    mappings: List[Tuple[str, str]]  # [(value_name, description)]
    loc: Loc


@dataclass(frozen=True)
class CellSpecDecl:
    name: str
    b: int
    codomain: str
    semantics: str
    role: str  # "Base" | "Supervisor" | "Composite"
    loc: Loc


@dataclass(frozen=True)
class CoupledSpecDecl:
    name: str
    cell: str
    bridges: List[int]
    loc: Loc


@dataclass(frozen=True)
class ConnectorDecl:
    name: str
    source_codomain: str
    target_position: int
    mapping: List[Tuple[str, str]]  # [(codomain_value, tri_literal)]
    loc: Loc


@dataclass(frozen=True)
class AdmissibilityTableDecl:
    name: str
    input_codomains: List[str]
    output_codomain: str
    table: List[Tuple[Tuple[str, ...], str]]  # [((inputs...), output)]
    loc: Loc


@dataclass(frozen=True)
class CaptureSpecDecl:
    name: str
    parameter_id: int
    observation_domain: str
    observation_space: str
    failure_symbol: str  # "Bottom"
    mapping: str
    loc: Loc


@dataclass(frozen=True)
class AdmissibilitySpecDecl:
    name: str
    parameter_id: int
    states: str  # literal "{Ok, Degraded, Failed, U}"
    rule: str
    loc: Loc


@dataclass(frozen=True)
class TernarizerDecl:
    name: str
    observation_space: str
    partition_zero: str
    partition_one: str
    partition_u: str
    mapping: str
    loc: Loc


@dataclass(frozen=True)
class ResSpecDecl:
    name: str
    context: str
    mechanism: str
    mapping: str
    loc: Loc


# ── N1 — Estado ──────────────────────────────────────────────────────

@dataclass(frozen=True)
class CellStateDecl:
    name: str
    spec: str
    vector: List[str]  # list of "Zero"|"One"|"U"
    loc: Loc


@dataclass(frozen=True)
class CoupledStateDecl:
    name: str
    spec: str
    base_vector: List[str]
    updated_vector: List[str]
    loc: Loc


@dataclass(frozen=True)
class EdgeLiteral:
    source: str
    target: str
    position: int
    connector: str
    loc: Loc


@dataclass(frozen=True)
class GraphDecl:
    name: str
    nodes: List[str]
    edges: List[EdgeLiteral]
    relation: str
    regime: str  # "Simple" | "General"
    conflicts: Optional[List[str]] = None
    loc: Loc = field(default_factory=lambda: Loc(0, 0))


# ── Composición ──────────────────────────────────────────────────────

@dataclass(frozen=True)
class SemanticRelationDecl:
    name: str
    kind: str  # "DeclaredRelation"
    table: Optional[str] = None
    constraints: Optional[List[str]] = None
    loc: Loc = field(default_factory=lambda: Loc(0, 0))


@dataclass(frozen=True)
class PatternDecl:
    name: str
    kind: str  # "DeclaredPattern"
    arity: Optional[int] = None
    constraints: Optional[List[str]] = None
    loc: Loc = field(default_factory=lambda: Loc(0, 0))


# ── N3 — Evolución ──────────────────────────────────────────────────

@dataclass(frozen=True)
class HorizonDecl:
    name: str
    architecture: str
    events: List[str]
    loc: Loc


@dataclass(frozen=True)
class FrameDecl:
    name: str
    index: int
    architecture: str
    cell_states: List[str]
    eval_results: List[str]
    gate_results: List[str]
    supervision: List[str]
    criticalities: List[str]
    loc: Loc


@dataclass(frozen=True)
class InducedParamLiteral:
    cell_ref: str
    position: int
    value: str  # "Zero"|"One"|"U"


@dataclass(frozen=True)
class EventStateLiteral:
    event_type: str
    state: str  # "Zero"|"One"|"U"


@dataclass(frozen=True)
class TransitionDataDecl:
    name: str
    horizon_ref: str
    events: List[EventStateLiteral]
    induced_parameters: List[InducedParamLiteral]
    metadata: Optional[List[str]] = None
    loc: Loc = field(default_factory=lambda: Loc(0, 0))


@dataclass(frozen=True)
class TrajectoryEntryLiteral:
    frame: str
    transition: Optional[str] = None


@dataclass(frozen=True)
class TrajectoryDecl:
    name: str
    entries: List[TrajectoryEntryLiteral]
    loc: Loc


# ── N4 — Uso ─────────────────────────────────────────────────────────

@dataclass(frozen=True)
class DomainDecl:
    name: str
    parameters: List[str]
    interface: str
    horizon: str
    capture_specs: List[str]
    admissibility_specs: List[str]
    ternarizers: List[str]
    exogeneity_mask: str
    silent_u: str
    transduction_policy: str
    u_policy: str
    closure_criterion: str
    loc: Loc


@dataclass(frozen=True)
class AgentDecl:
    name: str
    architecture: str
    domain: str
    query_engine: str
    loc: Loc


@dataclass(frozen=True)
class QuerySpecDecl:
    name: str
    query_type: str
    scope: str
    restrictions: List[str]
    loc: Loc


# ── Operadores (comandos) ────────────────────────────────────────────

@dataclass(frozen=True)
class EvalCmd:
    name: str
    input_ref: str
    loc: Loc


@dataclass(frozen=True)
class GateCmd:
    name: str
    inputs: List[str]
    using: str
    loc: Loc


@dataclass(frozen=True)
class ResolveCmd:
    name: str
    with_spec: str
    context: str
    mechanism: str
    loc: Loc


# QueryContext variants
@dataclass(frozen=True)
class QCPointEval:
    ref: str

@dataclass(frozen=True)
class QCTrajectoryView:
    ref: str

@dataclass(frozen=True)
class QCFrameComparison:
    ref1: str
    ref2: str

@dataclass(frozen=True)
class QCArchitectureView:
    arch: str
    cells: List[str]
    evals: List[str]
    gates: List[str]

@dataclass(frozen=True)
class QCCoverageReport:
    ref1: str
    ref2: str
    ref3: str


QueryContext = (QCPointEval | QCTrajectoryView | QCFrameComparison
                | QCArchitectureView | QCCoverageReport)


@dataclass(frozen=True)
class QueryCmd:
    name: str
    spec: str
    by: str
    context: QueryContext
    loc: Loc


# Supervisable variants
@dataclass(frozen=True)
class CellTarget:
    ref: str

@dataclass(frozen=True)
class ComposedTarget:
    ref: str

@dataclass(frozen=True)
class SystemTarget:
    ref: str

SupervisableTarget = CellTarget | ComposedTarget | SystemTarget


@dataclass(frozen=True)
class SuperviseCmd:
    name: str
    meta_eval: str
    target: SupervisableTarget
    loc: Loc


@dataclass(frozen=True)
class ComposeCmd:
    name: str
    graph: str
    relations: List[str]
    patterns: List[str]
    loc: Loc


@dataclass(frozen=True)
class ProjectionCmd:
    name: str
    source: str
    field_name: str
    loc: Loc


# ── Programa ─────────────────────────────────────────────────────────

ASTNode = (CodomainDecl | OutputSemanticsDecl | CellSpecDecl |
           CoupledSpecDecl | ConnectorDecl | AdmissibilityTableDecl |
           CaptureSpecDecl | AdmissibilitySpecDecl | TernarizerDecl |
           ResSpecDecl | CellStateDecl | CoupledStateDecl | GraphDecl |
           SemanticRelationDecl | PatternDecl | HorizonDecl | FrameDecl |
           TransitionDataDecl | TrajectoryDecl | DomainDecl | AgentDecl |
           QuerySpecDecl | EvalCmd | GateCmd | ResolveCmd | QueryCmd |
           SuperviseCmd | ComposeCmd | ProjectionCmd)


@dataclass(frozen=True)
class Program:
    nodes: List[ASTNode]
    source_file: str
