"""
svp_ir.py — Definiciones de objetos IR v0.2 del lenguaje SV
svp_lowering.py — Lowering: AST validado → IR v0.2

Cada nodo AST baja a exactamente un objeto IR. n se deriva de b como b².
Esto no es ejecución del sistema; es cumplimiento de la especificación.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

from dataclasses import dataclass, field, asdict
from typing import Any, Dict, List, Optional, Tuple
from svp_ast import *


# ── Objetos IR ────────────────────────────────────────────────────────

@dataclass
class IRObject:
    """Base para todo objeto de la IR."""
    level: str      # "N0", "N1", "N2", "N3", "N4"
    ir_type: str    # nombre canónico del tipo IR
    name: str
    fields: Dict[str, Any]

    def to_dict(self) -> dict:
        return {
            "level": self.level,
            "type": self.ir_type,
            "name": self.name,
            "fields": self.fields,
        }


@dataclass
class IROperation:
    """Operación IR derivada de un comando."""
    op_type: str
    name: str
    inputs: Dict[str, Any]
    result_type: str

    def to_dict(self) -> dict:
        return {
            "type": self.op_type,
            "name": self.name,
            "inputs": self.inputs,
            "result_type": self.result_type,
        }


@dataclass
class IRProgram:
    ir_version: str = "0.2"
    grammar_version: str = "0.1"
    source_file: str = ""
    source_sha256: str = ""
    serializer_version: str = "0.1.0"
    objects: List[IRObject] = field(default_factory=list)
    operations: List[IROperation] = field(default_factory=list)


# ── Lowering ──────────────────────────────────────────────────────────

class Lowering:
    """Transforma un AST validado en un IRProgram."""

    def __init__(self, program: Program, source_sha256: str = ""):
        self.ast_program = program
        self.source_sha256 = source_sha256

    def lower(self) -> IRProgram:
        ir = IRProgram(
            source_file=self.ast_program.source_file,
            source_sha256=self.source_sha256,
        )

        for node in self.ast_program.nodes:
            obj, op = self._lower_node(node)
            if obj is not None:
                ir.objects.append(obj)
            if op is not None:
                ir.operations.append(op)

        return ir

    def _lower_node(self, node: ASTNode) -> Tuple[Optional[IRObject], Optional[IROperation]]:
        if isinstance(node, CodomainDecl):
            return self._lower_codomain(node), None
        elif isinstance(node, OutputSemanticsDecl):
            return self._lower_output_semantics(node), None
        elif isinstance(node, CellSpecDecl):
            return self._lower_cellspec(node), None
        elif isinstance(node, CoupledSpecDecl):
            return self._lower_coupledspec(node), None
        elif isinstance(node, ConnectorDecl):
            return self._lower_connector(node), None
        elif isinstance(node, AdmissibilityTableDecl):
            return self._lower_admissibility_table(node), None
        elif isinstance(node, CaptureSpecDecl):
            return self._lower_capture_spec(node), None
        elif isinstance(node, AdmissibilitySpecDecl):
            return self._lower_admissibility_spec(node), None
        elif isinstance(node, TernarizerDecl):
            return self._lower_ternarizer(node), None
        elif isinstance(node, ResSpecDecl):
            return self._lower_res_spec(node), None
        elif isinstance(node, CellStateDecl):
            return self._lower_cellstate(node), None
        elif isinstance(node, CoupledStateDecl):
            return self._lower_coupledstate(node), None
        elif isinstance(node, GraphDecl):
            return self._lower_graph(node), None
        elif isinstance(node, SemanticRelationDecl):
            return self._lower_semantic_relation(node), None
        elif isinstance(node, PatternDecl):
            return self._lower_pattern(node), None
        elif isinstance(node, HorizonDecl):
            return self._lower_horizon(node), None
        elif isinstance(node, FrameDecl):
            return self._lower_frame(node), None
        elif isinstance(node, TransitionDataDecl):
            return self._lower_transition_data(node), None
        elif isinstance(node, TrajectoryDecl):
            return self._lower_trajectory(node), None
        elif isinstance(node, DomainDecl):
            return self._lower_domain(node), None
        elif isinstance(node, AgentDecl):
            return self._lower_agent(node), None
        elif isinstance(node, QuerySpecDecl):
            return self._lower_query_spec(node), None
        elif isinstance(node, EvalCmd):
            return None, self._lower_eval(node)
        elif isinstance(node, GateCmd):
            return None, self._lower_gate(node)
        elif isinstance(node, ResolveCmd):
            return None, self._lower_resolve(node)
        elif isinstance(node, QueryCmd):
            return None, self._lower_query(node)
        elif isinstance(node, SuperviseCmd):
            return None, self._lower_supervise(node)
        elif isinstance(node, ComposeCmd):
            return None, self._lower_compose(node)
        elif isinstance(node, ProjectionCmd):
            return None, self._lower_projection(node)
        return None, None

    # ── N0 ────────────────────────────────────────────────────────────

    def _lower_codomain(self, n: CodomainDecl) -> IRObject:
        return IRObject("N0", "Codomain", n.name, {"values": list(n.values)})

    def _lower_output_semantics(self, n: OutputSemanticsDecl) -> IRObject:
        return IRObject("N0", "OutputSemantics", n.name,
                         {"mappings": {k: v for k, v in n.mappings}})

    def _lower_cellspec(self, n: CellSpecDecl) -> IRObject:
        return IRObject("N0", "CellSpec", n.name, {
            "b": n.b,
            "n": n.b ** 2,  # derivación estructural cerrada
            "codomain": n.codomain,
            "semantics": n.semantics,
            "role": n.role,
        })

    def _lower_coupledspec(self, n: CoupledSpecDecl) -> IRObject:
        return IRObject("N0", "CoupledSpec", n.name, {
            "cell": n.cell,
            "bridges": list(n.bridges),
        })

    def _lower_connector(self, n: ConnectorDecl) -> IRObject:
        return IRObject("N0", "Connector", n.name, {
            "source_codomain": n.source_codomain,
            "target_position": n.target_position,
            "mapping": {k: v for k, v in n.mapping},
        })

    def _lower_admissibility_table(self, n: AdmissibilityTableDecl) -> IRObject:
        table_entries = []
        for keys, val in n.table:
            table_entries.append({"inputs": list(keys), "output": val})
        return IRObject("N0", "AdmissibilityTable", n.name, {
            "input_codomains": list(n.input_codomains),
            "output_codomain": n.output_codomain,
            "table": table_entries,
        })

    def _lower_capture_spec(self, n: CaptureSpecDecl) -> IRObject:
        return IRObject("N0", "CaptureSpec", n.name, {
            "parameter_id": n.parameter_id,
            "observation_domain": n.observation_domain,
            "observation_space": n.observation_space,
            "failure_symbol": n.failure_symbol,
            "mapping": n.mapping,
        })

    def _lower_admissibility_spec(self, n: AdmissibilitySpecDecl) -> IRObject:
        return IRObject("N0", "AdmissibilitySpec", n.name, {
            "parameter_id": n.parameter_id,
            "states": n.states,
            "rule": n.rule,
        })

    def _lower_ternarizer(self, n: TernarizerDecl) -> IRObject:
        return IRObject("N0", "Ternarizer", n.name, {
            "observation_space": n.observation_space,
            "partition_zero": n.partition_zero,
            "partition_one": n.partition_one,
            "partition_u": n.partition_u,
            "mapping": n.mapping,
        })

    def _lower_res_spec(self, n: ResSpecDecl) -> IRObject:
        return IRObject("N0", "ResSpec", n.name, {
            "context": n.context,
            "mechanism": n.mechanism,
            "mapping": n.mapping,
        })

    # ── N1 ────────────────────────────────────────────────────────────

    def _lower_cellstate(self, n: CellStateDecl) -> IRObject:
        return IRObject("N1", "CellState", n.name, {
            "spec": n.spec,
            "vector": list(n.vector),
        })

    def _lower_coupledstate(self, n: CoupledStateDecl) -> IRObject:
        return IRObject("N1", "CoupledState", n.name, {
            "spec": n.spec,
            "base_vector": list(n.base_vector),
            "updated_vector": list(n.updated_vector),
        })

    def _lower_graph(self, n: GraphDecl) -> IRObject:
        edges = []
        for e in n.edges:
            edges.append({
                "source": e.source,
                "target": e.target,
                "position": e.position,
                "connector": e.connector,
            })
        fields = {
            "nodes": list(n.nodes),
            "edges": edges,
            "relation": n.relation,
            "regime": n.regime,
        }
        if n.conflicts:
            fields["conflicts"] = list(n.conflicts)
        return IRObject("N1", "CompositionGraph", n.name, fields)

    # ── Composición ───────────────────────────────────────────────────

    def _lower_semantic_relation(self, n: SemanticRelationDecl) -> IRObject:
        fields = {"kind": n.kind}
        if n.table: fields["table"] = n.table
        if n.constraints: fields["constraints"] = list(n.constraints)
        return IRObject("N0", "SemanticRelation", n.name, fields)

    def _lower_pattern(self, n: PatternDecl) -> IRObject:
        fields = {"kind": n.kind}
        if n.arity is not None: fields["arity"] = n.arity
        if n.constraints: fields["constraints"] = list(n.constraints)
        return IRObject("N0", "Pattern", n.name, fields)

    # ── N3 ────────────────────────────────────────────────────────────

    def _lower_horizon(self, n: HorizonDecl) -> IRObject:
        return IRObject("N3", "Horizon", n.name, {
            "architecture": n.architecture,
            "events": list(n.events),
        })

    def _lower_frame(self, n: FrameDecl) -> IRObject:
        return IRObject("N3", "Frame", n.name, {
            "index": n.index,
            "architecture": n.architecture,
            "cell_states": list(n.cell_states),
            "eval_results": list(n.eval_results),
            "gate_results": list(n.gate_results),
            "supervision": list(n.supervision),
            "criticalities": list(n.criticalities),
        })

    def _lower_transition_data(self, n: TransitionDataDecl) -> IRObject:
        params = []
        for p in n.induced_parameters:
            params.append({
                "cell_ref": p.cell_ref,
                "position": p.position,
                "value": p.value,
            })
        events = []
        for e in n.events:
            events.append({"event_type": e.event_type, "state": e.state})
        fields = {
            "horizon_ref": n.horizon_ref,
            "events": events,
            "induced_parameters": params,
        }
        if n.metadata:
            fields["metadata"] = list(n.metadata)
        return IRObject("N3", "TransitionData", n.name, fields)

    def _lower_trajectory(self, n: TrajectoryDecl) -> IRObject:
        entries = []
        for e in n.entries:
            entry = {"frame": e.frame}
            if e.transition:
                entry["transition"] = e.transition
            entries.append(entry)
        return IRObject("N3", "Trajectory", n.name, {"entries": entries})

    # ── N4 ────────────────────────────────────────────────────────────

    def _lower_domain(self, n: DomainDecl) -> IRObject:
        return IRObject("N4", "Domain", n.name, {
            "parameters": list(n.parameters),
            "interface": n.interface,
            "horizon": n.horizon,
            "capture_specs": list(n.capture_specs),
            "admissibility_specs": list(n.admissibility_specs),
            "ternarizers": list(n.ternarizers),
            "exogeneity_mask": n.exogeneity_mask,
            "silent_u": n.silent_u,
            "transduction_policy": n.transduction_policy,
            "u_policy": n.u_policy,
            "closure_criterion": n.closure_criterion,
        })

    def _lower_agent(self, n: AgentDecl) -> IRObject:
        return IRObject("N4", "Agent", n.name, {
            "architecture": n.architecture,
            "domain": n.domain,
            "query_engine": n.query_engine,
        })

    def _lower_query_spec(self, n: QuerySpecDecl) -> IRObject:
        return IRObject("N4", "QuerySpec", n.name, {
            "query_type": n.query_type,
            "scope": n.scope,
            "restrictions": list(n.restrictions),
        })

    # ── Operadores ────────────────────────────────────────────────────

    def _lower_eval(self, n: EvalCmd) -> IROperation:
        return IROperation("evaluate", n.name,
                            {"state": n.input_ref}, "EvalResult")

    def _lower_gate(self, n: GateCmd) -> IROperation:
        return IROperation("gate", n.name,
                            {"eval_results": list(n.inputs), "table": n.using},
                            "GateResult")

    def _lower_resolve(self, n: ResolveCmd) -> IROperation:
        return IROperation("resolve", n.name,
                            {"with_spec": n.with_spec, "context": n.context,
                             "mechanism": n.mechanism},
                            "ResolutionRecord")

    def _lower_query(self, n: QueryCmd) -> IROperation:
        ctx = self._lower_query_context(n.context)
        return IROperation("query", n.name,
                            {"spec": n.spec, "by": n.by, "context": ctx},
                            "QueryResult")

    def _lower_query_context(self, ctx: QueryContext) -> dict:
        if isinstance(ctx, QCPointEval):
            return {"variant": "PointEval", "ref": ctx.ref}
        elif isinstance(ctx, QCTrajectoryView):
            return {"variant": "TrajectoryView", "ref": ctx.ref}
        elif isinstance(ctx, QCFrameComparison):
            return {"variant": "FrameComparison", "refs": [ctx.ref1, ctx.ref2]}
        elif isinstance(ctx, QCArchitectureView):
            return {"variant": "ArchitectureView", "architecture": ctx.arch,
                    "cells": ctx.cells, "evals": ctx.evals, "gates": ctx.gates}
        elif isinstance(ctx, QCCoverageReport):
            return {"variant": "CoverageReport", "refs": [ctx.ref1, ctx.ref2, ctx.ref3]}
        return {}

    def _lower_supervise(self, n: SuperviseCmd) -> IROperation:
        target = self._lower_supervisable(n.target)
        return IROperation("supervise", n.name,
                            {"meta_eval": n.meta_eval, "target": target},
                            "SupervisionResult")

    def _lower_supervisable(self, t: SupervisableTarget) -> dict:
        if isinstance(t, CellTarget):
            return {"variant": "CellTarget", "ref": t.ref}
        elif isinstance(t, ComposedTarget):
            return {"variant": "ComposedTarget", "ref": t.ref}
        elif isinstance(t, SystemTarget):
            return {"variant": "SystemTarget", "ref": t.ref}
        return {}

    def _lower_compose(self, n: ComposeCmd) -> IROperation:
        return IROperation("compose", n.name,
                            {"graph": n.graph, "relations": list(n.relations),
                             "patterns": list(n.patterns)},
                            "Architecture")

    def _lower_projection(self, n: ProjectionCmd) -> IROperation:
        return IROperation("projection", n.name,
                            {"source": n.source, "field": n.field_name},
                            "Projected")
