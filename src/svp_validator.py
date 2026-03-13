"""
svp_validator.py — Verificación de bienformación del AST

Comprueba los juicios de bienformación J0.1–J5.2 de la IR v0.2
sobre el AST producido por el parser. No ejecuta nada: solo valida.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

from typing import Dict, Set
from svp_ast import *
from svp_errors import (SVPError, E002, E004, E005, E006, E101, E102,
                         E104, E105, E202, E211, E303)


class Validator:
    def __init__(self, program: Program):
        self.program = program
        self.symbols: Dict[str, ASTNode] = {}
        self.symbol_types: Dict[str, str] = {}  # name -> type tag
        self.errors = []

    def validate(self):
        """Ejecuta todas las validaciones. Lanza SVPError en el primer error."""
        # Fase 1: registrar todos los nombres
        for node in self.program.nodes:
            self._register(node)

        # Fase 2: validar cada nodo
        for node in self.program.nodes:
            self._validate_node(node)

    def _register(self, node: ASTNode):
        name = getattr(node, "name", None)
        if name is None:
            return
        if name in self.symbols:
            raise SVPError(E005, getattr(node, "loc", Loc(0,0)).line,
                           getattr(node, "loc", Loc(0,0)).col,
                           f"Identificador duplicado: {name!r}")
        self.symbols[name] = node
        self.symbol_types[name] = type(node).__name__

    def _require_ref(self, name: str, loc: Loc, expected_type: str = None):
        if name not in self.symbols:
            raise SVPError(E006, loc.line, loc.col,
                           f"Referencia no declarada: {name!r}")
        if expected_type and self.symbol_types[name] != expected_type:
            raise SVPError(E006, loc.line, loc.col,
                           f"{name!r} es {self.symbol_types[name]}, se esperaba {expected_type}")

    def _validate_node(self, node: ASTNode):
        if isinstance(node, CellSpecDecl):
            self._validate_cellspec(node)
        elif isinstance(node, CoupledSpecDecl):
            self._validate_coupledspec(node)
        elif isinstance(node, CellStateDecl):
            self._validate_cellstate(node)
        elif isinstance(node, CoupledStateDecl):
            self._validate_coupledstate(node)
        elif isinstance(node, ConnectorDecl):
            self._validate_connector(node)
        elif isinstance(node, GraphDecl):
            self._validate_graph(node)
        elif isinstance(node, TransitionDataDecl):
            self._validate_transition_data(node)
        elif isinstance(node, GateCmd):
            self._validate_gate(node)
        elif isinstance(node, EvalCmd):
            self._validate_eval(node)
        elif isinstance(node, ResolveCmd):
            self._validate_resolve(node)
        elif isinstance(node, SuperviseCmd):
            self._validate_supervise(node)
        elif isinstance(node, ComposeCmd):
            self._validate_compose(node)
        elif isinstance(node, ProjectionCmd):
            self._validate_projection(node)
        elif isinstance(node, CodomainDecl):
            self._validate_codomain(node)

    # ── Validaciones concretas ────────────────────────────────────────

    def _validate_cellspec(self, node: CellSpecDecl):
        # J1.1: b >= 3
        if node.b < 3:
            raise SVPError(E002, node.loc.line, node.loc.col,
                           f"b = {node.b}, debe ser >= 3")
        self._require_ref(node.codomain, node.loc)
        self._require_ref(node.semantics, node.loc)

    def _validate_coupledspec(self, node: CoupledSpecDecl):
        self._require_ref(node.cell, node.loc, "CellSpecDecl")
        cell = self.symbols[node.cell]
        n = cell.b ** 2
        for pos in node.bridges:
            if pos < 1 or pos > n:
                raise SVPError(E105, node.loc.line, node.loc.col,
                               f"Posición puente {pos} fuera de rango [1, {n}]")

    def _validate_cellstate(self, node: CellStateDecl):
        self._require_ref(node.spec, node.loc, "CellSpecDecl")
        cell = self.symbols[node.spec]
        expected_len = cell.b ** 2
        if len(node.vector) != expected_len:
            raise SVPError(E101, node.loc.line, node.loc.col,
                           f"Vector de longitud {len(node.vector)}, se esperaba {expected_len}")

    def _validate_coupledstate(self, node: CoupledStateDecl):
        self._require_ref(node.spec, node.loc, "CoupledSpecDecl")
        coupled = self.symbols[node.spec]
        cell = self.symbols.get(coupled.cell)
        if cell:
            expected_len = cell.b ** 2
            if len(node.base_vector) != expected_len:
                raise SVPError(E101, node.loc.line, node.loc.col,
                               f"base_vector de longitud {len(node.base_vector)}, se esperaba {expected_len}")
            if len(node.updated_vector) != expected_len:
                raise SVPError(E101, node.loc.line, node.loc.col,
                               f"updated_vector de longitud {len(node.updated_vector)}, se esperaba {expected_len}")

    def _validate_connector(self, node: ConnectorDecl):
        self._require_ref(node.source_codomain, node.loc, "CodomainDecl")

    def _validate_codomain(self, node: CodomainDecl):
        if len(node.values) == 0:
            raise SVPError(E004, node.loc.line, node.loc.col)

    def _validate_graph(self, node: GraphDecl):
        self._require_ref(node.relation, node.loc, "SemanticRelationDecl")
        for n_ref in node.nodes:
            if n_ref not in self.symbols:
                raise SVPError(E006, node.loc.line, node.loc.col,
                               f"Nodo del grafo no declarado: {n_ref!r}")
        # Cycle detection via topological sort
        adj: Dict[str, Set[str]] = {n_ref: set() for n_ref in node.nodes}
        for e in node.edges:
            if e.source in adj:
                adj[e.source].add(e.target)
        visited = set()
        in_stack = set()
        def dfs(v):
            if v in in_stack:
                from svp_errors import E103
                raise SVPError(E103, node.loc.line, node.loc.col,
                               f"Ciclo detectado en el grafo que incluye {v!r}")
            if v in visited:
                return
            in_stack.add(v)
            for w in adj.get(v, []):
                dfs(w)
            in_stack.remove(v)
            visited.add(v)
        for v in node.nodes:
            dfs(v)

    def _validate_transition_data(self, node: TransitionDataDecl):
        self._require_ref(node.horizon_ref, node.loc, "HorizonDecl")

    def _validate_gate(self, node: GateCmd):
        self._require_ref(node.using, node.loc, "AdmissibilityTableDecl")
        for inp in node.inputs:
            self._require_ref(inp, node.loc)
            if self.symbol_types[inp] != "EvalCmd":
                raise SVPError(E202, node.loc.line, node.loc.col,
                               f"Argumento de gate '{inp}' no es EvalResult (es {self.symbol_types[inp]})")

    def _validate_eval(self, node: EvalCmd):
        self._require_ref(node.input_ref, node.loc)

    def _validate_resolve(self, node: ResolveCmd):
        self._require_ref(node.with_spec, node.loc, "ResSpecDecl")

    def _validate_supervise(self, node: SuperviseCmd):
        self._require_ref(node.meta_eval, node.loc)
        target_ref = getattr(node.target, "ref", None)
        if target_ref is not None:
            self._require_ref(target_ref, node.loc)
        meta_node = self.symbols.get(node.meta_eval)
        if meta_node and isinstance(meta_node, EvalCmd):
            state_node = self.symbols.get(meta_node.input_ref)
            if state_node and isinstance(state_node, CellStateDecl):
                spec_node = self.symbols.get(state_node.spec)
                if spec_node and isinstance(spec_node, CellSpecDecl):
                    if spec_node.role != "Supervisor":
                        raise SVPError(E211, node.loc.line, node.loc.col,
                                       f"El primer argumento de supervise debe provenir de una célula con rol Supervisor, "
                                       f"pero '{node.meta_eval}' proviene de '{spec_node.name}' con rol '{spec_node.role}'")

    def _validate_projection(self, node: ProjectionCmd):
        self._require_ref(node.source, node.loc)

    def _validate_compose(self, node: ComposeCmd):
        self._require_ref(node.graph, node.loc, "GraphDecl")
        for r in node.relations:
            self._require_ref(r, node.loc, "SemanticRelationDecl")
        for p in node.patterns:
            self._require_ref(p, node.loc, "PatternDecl")
