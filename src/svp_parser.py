"""
svp_parser.py — Parser de descenso recursivo para archivos .svp

Implementa la EBNF de la Gramática superficial mínima v0.1.
Produce un AST (svp_ast.Program) a partir de una lista de tokens.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

from typing import List, Optional, Tuple

from svp_lexer import Token, TT, PROHIBITED_TOKENS
from svp_ast import *
from svp_errors import (SVPError, E001, E010, E201, E203, E204,
                         E205, E206, E207, E208, E209, E210)


class Parser:
    def __init__(self, tokens: List[Token], filename: str = "<stdin>"):
        self.tokens = tokens
        self.pos = 0
        self.filename = filename

    # ── Utilidades ────────────────────────────────────────────────────

    def _cur(self) -> Token:
        return self.tokens[self.pos]

    def _loc(self) -> Loc:
        t = self._cur()
        return Loc(t.line, t.col)

    def _peek(self, tt: TT) -> bool:
        return self._cur().type == tt

    def _at_end(self) -> bool:
        return self._cur().type == TT.EOF

    def _advance(self) -> Token:
        t = self.tokens[self.pos]
        if t.type in PROHIBITED_TOKENS:
            raise SVPError(E210, t.line, t.col, f"Palabra prohibida en v0.1: {t.value!r}")
        self.pos += 1
        return t

    def _expect(self, tt: TT, msg: str = "") -> Token:
        t = self._cur()
        if t.type != tt:
            detail = msg or f"Se esperaba {tt.name}, se encontró {t.type.name} ({t.value!r})"
            raise SVPError(E001, t.line, t.col, detail)
        return self._advance()

    def _expect_id(self) -> str:
        return self._expect(TT.IDENTIFIER, "Se esperaba un identificador").value

    def _expect_nat(self) -> int:
        return int(self._expect(TT.NAT, "Se esperaba un número natural").value)

    def _expect_str(self) -> str:
        return self._expect(TT.STRING, "Se esperaba una cadena").value

    def _expect_tri(self) -> str:
        t = self._cur()
        if t.type in (TT.ZERO, TT.ONE, TT.U_LIT):
            self._advance()
            return t.value
        raise SVPError(E001, t.line, t.col, "Se esperaba Zero, One o U")

    def _match(self, tt: TT) -> Optional[Token]:
        if self._cur().type == tt:
            return self._advance()
        return None

    def _parse_list(self, parse_elem) -> list:
        """Parsea [elem, elem, ...]"""
        self._expect(TT.LBRACKET)
        items = []
        if not self._peek(TT.RBRACKET):
            items.append(parse_elem())
            while self._match(TT.COMMA):
                items.append(parse_elem())
        self._expect(TT.RBRACKET)
        return items

    def _parse_id_list(self) -> List[str]:
        return self._parse_list(self._expect_id)

    def _parse_nat_list(self) -> List[int]:
        return self._parse_list(self._expect_nat)

    def _parse_tri_list(self) -> List[str]:
        return self._parse_list(self._expect_tri)

    def _field(self, name: str):
        """Consume 'name' ':' y devuelve el token name consumido."""
        t = self._cur()
        if t.type == TT.IDENTIFIER and t.value == name:
            self._advance()
        elif t.value == name:
            self._advance()
        else:
            raise SVPError(E001, t.line, t.col, f"Se esperaba campo '{name}'")
        self._expect(TT.COLON)

    def _kw_field(self, kw_tt: TT):
        """Consume keyword ':' para campos que son palabras clave."""
        self._expect(kw_tt)
        self._expect(TT.COLON)

    # ── Programa ──────────────────────────────────────────────────────

    def parse(self) -> Program:
        nodes = []
        while not self._at_end():
            nodes.append(self._parse_top_level())
        return Program(nodes=nodes, source_file=self.filename)

    def _parse_top_level(self) -> ASTNode:
        t = self._cur()

        # Prohibited keywords
        if t.type in PROHIBITED_TOKENS:
            raise SVPError(E210, t.line, t.col, f"Palabra prohibida en v0.1: {t.value!r}")

        dispatch = {
            TT.KW_CODOMAIN: self._parse_codomain,
            TT.KW_OUTPUT_SEMANTICS: self._parse_output_semantics,
            TT.KW_CELLSPEC: self._parse_cellspec,
            TT.KW_COUPLEDSPEC: self._parse_coupledspec,
            TT.KW_CONNECTOR: self._parse_connector,
            TT.KW_ADMISSIBILITY_TABLE: self._parse_admissibility_table,
            TT.KW_CAPTURE_SPEC: self._parse_capture_spec,
            TT.KW_ADMISSIBILITY_SPEC: self._parse_admissibility_spec,
            TT.KW_TERNARIZER: self._parse_ternarizer,
            TT.KW_RES_SPEC: self._parse_res_spec,
            TT.KW_CELLSTATE: self._parse_cellstate,
            TT.KW_COUPLEDSTATE: self._parse_coupledstate,
            TT.KW_GRAPH: self._parse_graph,
            TT.KW_SEMANTIC_RELATION: self._parse_semantic_relation,
            TT.KW_PATTERN: self._parse_pattern,
            TT.KW_HORIZON: self._parse_horizon,
            TT.KW_FRAME: self._parse_frame,
            TT.KW_TRANSITION_DATA: self._parse_transition_data,
            TT.KW_TRAJECTORY: self._parse_trajectory,
            TT.KW_DOMAIN: self._parse_domain,
            TT.KW_AGENT: self._parse_agent,
            TT.KW_QUERY_SPEC: self._parse_query_spec,
            TT.KW_LET: self._parse_let,
        }

        handler = dispatch.get(t.type)
        if handler is None:
            raise SVPError(E001, t.line, t.col,
                           f"Declaración o comando no reconocido: {t.value!r}")
        return handler()

    # ── N0 — Definición ───────────────────────────────────────────────

    def _parse_codomain(self) -> CodomainDecl:
        loc = self._loc()
        self._advance()  # codomain
        name = self._expect_id()
        self._expect(TT.EQUALS)
        self._expect(TT.LBRACE)
        values = [self._expect_id()]
        while self._match(TT.COMMA):
            values.append(self._expect_id())
        self._expect(TT.RBRACE)
        self._expect(TT.SEMICOLON)
        return CodomainDecl(name=name, values=values, loc=loc)

    def _parse_output_semantics(self) -> OutputSemanticsDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        mappings = []
        while not self._peek(TT.RBRACE):
            k = self._expect_id()
            self._expect(TT.ARROW)
            v = self._expect_str()
            self._expect(TT.SEMICOLON)
            mappings.append((k, v))
        self._expect(TT.RBRACE)
        return OutputSemanticsDecl(name=name, mappings=mappings, loc=loc)

    def _parse_cellspec(self) -> CellSpecDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("b"); b = self._expect_nat(); self._expect(TT.SEMICOLON)
        self._field("codomain"); cod = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("semantics"); sem = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("role"); role = self._parse_role(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return CellSpecDecl(name=name, b=b, codomain=cod, semantics=sem, role=role, loc=loc)

    def _parse_role(self) -> str:
        t = self._cur()
        if t.type in (TT.KW_BASE, TT.KW_SUPERVISOR, TT.KW_COMPOSITE):
            self._advance()
            return t.value
        raise SVPError(E010, t.line, t.col, f"Rol no reconocido: {t.value!r}")

    def _parse_coupledspec(self) -> CoupledSpecDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("cell"); cell = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("bridges"); bridges = self._parse_nat_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return CoupledSpecDecl(name=name, cell=cell, bridges=bridges, loc=loc)

    def _parse_connector(self) -> ConnectorDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("source_codomain"); src = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("target_position"); pos = self._expect_nat(); self._expect(TT.SEMICOLON)
        self._field("mapping"); self._expect(TT.LBRACE)
        mapping = []
        while not self._peek(TT.RBRACE):
            k = self._expect_id()
            self._expect(TT.ARROW)
            v = self._expect_tri()
            self._expect(TT.SEMICOLON)
            mapping.append((k, v))
        self._expect(TT.RBRACE)
        self._expect(TT.SEMICOLON) if self._peek(TT.SEMICOLON) else None
        self._expect(TT.RBRACE)
        return ConnectorDecl(name=name, source_codomain=src, target_position=pos,
                             mapping=mapping, loc=loc)

    def _parse_admissibility_table(self) -> AdmissibilityTableDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("input_codomains"); ins = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("output_codomain"); out = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("table"); self._expect(TT.LBRACE)
        table = []
        while not self._peek(TT.RBRACE):
            self._expect(TT.LPAREN)
            keys = [self._expect_id()]
            while self._match(TT.COMMA):
                keys.append(self._expect_id())
            self._expect(TT.RPAREN)
            self._expect(TT.ARROW)
            val = self._expect_id()
            self._expect(TT.SEMICOLON)
            table.append((tuple(keys), val))
        self._expect(TT.RBRACE)
        self._match(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return AdmissibilityTableDecl(name=name, input_codomains=ins,
                                       output_codomain=out, table=table, loc=loc)

    def _parse_capture_spec(self) -> CaptureSpecDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("parameter_id"); pid = self._expect_nat(); self._expect(TT.SEMICOLON)
        self._field("observation_domain"); od = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("observation_space"); os_ = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("failure_symbol"); self._expect(TT.KW_BOTTOM); self._expect(TT.SEMICOLON)
        self._field("mapping"); mp = self._expect_id(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return CaptureSpecDecl(name=name, parameter_id=pid, observation_domain=od,
                                observation_space=os_, failure_symbol="Bottom", mapping=mp, loc=loc)

    def _parse_admissibility_spec(self) -> AdmissibilitySpecDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("parameter_id"); pid = self._expect_nat(); self._expect(TT.SEMICOLON)
        self._field("states")
        # Parse literal {Ok, Degraded, Failed, U}
        self._expect(TT.LBRACE)
        vals = [self._advance().value]
        while self._match(TT.COMMA):
            vals.append(self._advance().value)
        self._expect(TT.RBRACE)
        self._expect(TT.SEMICOLON)
        self._field("rule"); rule = self._expect_id(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        states = "{" + ", ".join(vals) + "}"
        return AdmissibilitySpecDecl(name=name, parameter_id=pid, states=states,
                                      rule=rule, loc=loc)

    def _parse_ternarizer(self) -> TernarizerDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("observation_space"); os_ = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("partition_zero"); pz = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("partition_one"); po = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("partition_u"); pu = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("mapping"); mp = self._expect_id(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return TernarizerDecl(name=name, observation_space=os_, partition_zero=pz,
                               partition_one=po, partition_u=pu, mapping=mp, loc=loc)

    def _parse_res_spec(self) -> ResSpecDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("context"); ctx = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("mechanism"); mech = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("mapping"); mp = self._expect_id(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return ResSpecDecl(name=name, context=ctx, mechanism=mech, mapping=mp, loc=loc)

    # ── N1 — Estado ───────────────────────────────────────────────────

    def _parse_cellstate(self) -> CellStateDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("spec"); spec = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("vector"); vec = self._parse_tri_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return CellStateDecl(name=name, spec=spec, vector=vec, loc=loc)

    def _parse_coupledstate(self) -> CoupledStateDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("spec"); spec = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("base_vector"); bv = self._parse_tri_list(); self._expect(TT.SEMICOLON)
        self._field("updated_vector"); uv = self._parse_tri_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return CoupledStateDecl(name=name, spec=spec, base_vector=bv,
                                 updated_vector=uv, loc=loc)

    def _parse_edge(self) -> EdgeLiteral:
        loc = self._loc()
        self._expect(TT.KW_EDGE)
        self._expect(TT.LPAREN)
        self._field("source"); src = self._expect_id(); self._expect(TT.COMMA)
        self._field("target"); tgt = self._expect_id(); self._expect(TT.COMMA)
        self._field("position"); pos = self._expect_nat(); self._expect(TT.COMMA)
        self._field("connector"); con = self._expect_id()
        self._expect(TT.RPAREN)
        return EdgeLiteral(source=src, target=tgt, position=pos, connector=con, loc=loc)

    def _parse_graph(self) -> GraphDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("nodes"); nodes = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("edges"); edges = self._parse_list(self._parse_edge); self._expect(TT.SEMICOLON)
        self._field("relation"); rel = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("regime")
        t = self._cur()
        if t.type in (TT.KW_SIMPLE, TT.KW_GENERAL):
            regime = self._advance().value
        else:
            raise SVPError(E001, t.line, t.col, "Se esperaba Simple o General")
        self._expect(TT.SEMICOLON)
        conflicts = None
        if self._cur().type == TT.IDENTIFIER and self._cur().value == "conflicts":
            self._field("conflicts"); conflicts = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return GraphDecl(name=name, nodes=nodes, edges=edges, relation=rel,
                          regime=regime, conflicts=conflicts, loc=loc)

    # ── Composición ───────────────────────────────────────────────────

    def _parse_semantic_relation(self) -> SemanticRelationDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("kind"); self._expect(TT.KW_DECLARED_RELATION); kind = "DeclaredRelation"
        self._expect(TT.SEMICOLON)
        table = None
        constraints = None
        while not self._peek(TT.RBRACE):
            if self._cur().value == "table":
                self._field("table"); table = self._expect_id(); self._expect(TT.SEMICOLON)
            elif self._cur().value == "constraints":
                self._field("constraints"); constraints = self._parse_id_list(); self._expect(TT.SEMICOLON)
            else:
                break
        self._expect(TT.RBRACE)
        return SemanticRelationDecl(name=name, kind=kind, table=table,
                                     constraints=constraints, loc=loc)

    def _parse_pattern(self) -> PatternDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("kind"); self._expect(TT.KW_DECLARED_PATTERN); kind = "DeclaredPattern"
        self._expect(TT.SEMICOLON)
        arity = None
        constraints = None
        while not self._peek(TT.RBRACE):
            if self._cur().value == "arity":
                self._field("arity"); arity = self._expect_nat(); self._expect(TT.SEMICOLON)
            elif self._cur().value == "constraints":
                self._field("constraints"); constraints = self._parse_id_list(); self._expect(TT.SEMICOLON)
            else:
                break
        self._expect(TT.RBRACE)
        return PatternDecl(name=name, kind=kind, arity=arity,
                            constraints=constraints, loc=loc)

    # ── N3 — Evolución ────────────────────────────────────────────────

    def _parse_horizon(self) -> HorizonDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("architecture"); arch = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("events"); events = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return HorizonDecl(name=name, architecture=arch, events=events, loc=loc)

    def _parse_frame(self) -> FrameDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("index"); idx = self._expect_nat(); self._expect(TT.SEMICOLON)
        self._field("architecture"); arch = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("cell_states"); cs = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("eval_results"); er = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("gate_results"); gr = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("supervision"); sv = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("criticalities"); cr = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return FrameDecl(name=name, index=idx, architecture=arch, cell_states=cs,
                          eval_results=er, gate_results=gr, supervision=sv,
                          criticalities=cr, loc=loc)

    def _parse_induced_param(self) -> InducedParamLiteral:
        self._expect(TT.LPAREN)
        cell = self._expect_id()
        self._expect(TT.COMMA)
        pos = self._expect_nat()
        self._expect(TT.COMMA)
        val = self._expect_tri()
        self._expect(TT.RPAREN)
        return InducedParamLiteral(cell_ref=cell, position=pos, value=val)

    def _parse_event_state(self) -> 'EventStateLiteral':
        from svp_ast import EventStateLiteral
        self._expect(TT.LPAREN)
        event_type = self._expect_id()
        self._expect(TT.COMMA)
        state = self._expect_tri()
        self._expect(TT.RPAREN)
        return EventStateLiteral(event_type=event_type, state=state)

    def _parse_transition_data(self) -> TransitionDataDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("horizon_ref"); href = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("events"); events = self._parse_list(self._parse_event_state); self._expect(TT.SEMICOLON)
        self._field("induced_parameters")
        params = self._parse_list(self._parse_induced_param)
        self._expect(TT.SEMICOLON)
        metadata = None
        if not self._peek(TT.RBRACE) and self._cur().value == "metadata":
            self._field("metadata"); metadata = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return TransitionDataDecl(name=name, horizon_ref=href, events=events,
                                   induced_parameters=params, metadata=metadata, loc=loc)

    def _parse_trajectory_entry(self) -> TrajectoryEntryLiteral:
        self._expect(TT.KW_ENTRY)
        self._expect(TT.LPAREN)
        self._field("frame"); fr = self._expect_id()
        tr = None
        if self._match(TT.COMMA):
            self._field("transition"); tr = self._expect_id()
        self._expect(TT.RPAREN)
        return TrajectoryEntryLiteral(frame=fr, transition=tr)

    def _parse_trajectory(self) -> TrajectoryDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("entries")
        entries = self._parse_list(self._parse_trajectory_entry)
        self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return TrajectoryDecl(name=name, entries=entries, loc=loc)

    # ── N4 — Uso ──────────────────────────────────────────────────────

    def _parse_domain(self) -> DomainDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("parameters"); params = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("interface"); iface = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("horizon"); hor = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("capture_specs"); cs = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("admissibility_specs"); asp = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("ternarizers"); trn = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._field("exogeneity_mask"); em = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("silent_u"); su = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("transduction_policy"); tp = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("u_policy"); up = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("closure_criterion"); cc = self._expect_id(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return DomainDecl(name=name, parameters=params, interface=iface, horizon=hor,
                           capture_specs=cs, admissibility_specs=asp, ternarizers=trn,
                           exogeneity_mask=em, silent_u=su, transduction_policy=tp,
                           u_policy=up, closure_criterion=cc, loc=loc)

    def _parse_agent(self) -> AgentDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("architecture"); arch = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("domain"); dom = self._expect_id(); self._expect(TT.SEMICOLON)
        self._field("query_engine"); qe = self._expect_id(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return AgentDecl(name=name, architecture=arch, domain=dom, query_engine=qe, loc=loc)

    def _parse_query_spec(self) -> QuerySpecDecl:
        loc = self._loc()
        self._advance()
        name = self._expect_id()
        self._expect(TT.LBRACE)
        self._field("query_type")
        qt_map = {
            TT.KW_POINT_EVALUATION: "PointEvaluation",
            TT.KW_TRAJECTORY_STATE: "TrajectoryState",
            TT.KW_FRAME_COMPARISON_TYPE: "FrameComparison",
            TT.KW_COVERAGE_STATE: "CoverageState",
            TT.KW_PENDING_U: "PendingU",
            TT.KW_GLOBAL_CRITICALITY: "GlobalCriticality",
        }
        t = self._cur()
        if t.type in qt_map:
            qt = qt_map[self._advance().type]
        else:
            raise SVPError(E001, t.line, t.col, f"Tipo de query no reconocido: {t.value!r}")
        self._expect(TT.SEMICOLON)
        self._field("scope")
        sc_map = {
            TT.KW_CELL: "Cell", TT.KW_PAIR: "Pair",
            TT.KW_ARCHITECTURE: "Architecture", TT.KW_TRAJECTORY: "Trajectory",
        }
        t = self._cur()
        if t.type in sc_map:
            sc = sc_map[self._advance().type]
        else:
            raise SVPError(E001, t.line, t.col, f"Alcance no reconocido: {t.value!r}")
        self._expect(TT.SEMICOLON)
        self._field("restrictions"); rest = self._parse_id_list(); self._expect(TT.SEMICOLON)
        self._expect(TT.RBRACE)
        return QuerySpecDecl(name=name, query_type=qt, scope=sc, restrictions=rest, loc=loc)

    # ── Comandos (let ...) ────────────────────────────────────────────

    def _parse_let(self) -> ASTNode:
        loc = self._loc()
        self._advance()  # let
        name = self._expect_id()
        self._expect(TT.EQUALS)

        t = self._cur()

        # Prohibited keywords check
        if t.type in PROHIBITED_TOKENS:
            raise SVPError(E210, t.line, t.col, f"Palabra prohibida en v0.1: {t.value!r}")

        if t.type == TT.KW_EVALUATE:
            return self._parse_eval_cmd(name, loc)
        elif t.type == TT.KW_GATE:
            return self._parse_gate_cmd(name, loc)
        elif t.type == TT.KW_RESOLVE:
            return self._parse_resolve_cmd(name, loc)
        elif t.type == TT.KW_QUERY:
            return self._parse_query_cmd(name, loc)
        elif t.type == TT.KW_SUPERVISE:
            return self._parse_supervise_cmd(name, loc)
        elif t.type == TT.KW_COMPOSE:
            return self._parse_compose_cmd(name, loc)
        elif t.type == TT.IDENTIFIER:
            # projection: let x = y.field;
            src = self._expect_id()
            self._expect(TT.DOT)
            fld = self._expect_id()
            self._expect(TT.SEMICOLON)
            return ProjectionCmd(name=name, source=src, field_name=fld, loc=loc)
        else:
            raise SVPError(E001, t.line, t.col,
                           f"Se esperaba operador o proyección después de '=', se encontró {t.value!r}")

    def _parse_eval_cmd(self, name: str, loc: Loc) -> EvalCmd:
        self._advance()  # evaluate
        self._expect(TT.LPAREN)
        ref = self._expect_id()
        self._expect(TT.RPAREN)
        self._expect(TT.SEMICOLON)
        return EvalCmd(name=name, input_ref=ref, loc=loc)

    def _parse_gate_cmd(self, name: str, loc: Loc) -> GateCmd:
        self._advance()  # gate
        self._expect(TT.LPAREN)
        inputs = self._parse_id_list()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_USING)
        table = self._expect_id()
        self._expect(TT.RPAREN)
        self._expect(TT.SEMICOLON)
        return GateCmd(name=name, inputs=inputs, using=table, loc=loc)

    def _parse_resolve_cmd(self, name: str, loc: Loc) -> ResolveCmd:
        self._advance()  # resolve
        self._expect(TT.LPAREN)
        self._expect(TT.U_LIT)  # literal U
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_WITH)
        ws = self._expect_id()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_CONTEXT)
        ctx = self._expect_id()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_MECHANISM)
        mech = self._expect_id()
        self._expect(TT.RPAREN)
        self._expect(TT.SEMICOLON)
        return ResolveCmd(name=name, with_spec=ws, context=ctx, mechanism=mech, loc=loc)

    def _parse_query_context(self) -> QueryContext:
        t = self._cur()
        if t.type == TT.KW_POINT_EVAL:
            self._advance(); self._expect(TT.LPAREN)
            r = self._expect_id(); self._expect(TT.RPAREN)
            return QCPointEval(ref=r)
        elif t.type == TT.KW_TRAJECTORY_VIEW:
            self._advance(); self._expect(TT.LPAREN)
            r = self._expect_id(); self._expect(TT.RPAREN)
            return QCTrajectoryView(ref=r)
        elif t.type == TT.KW_FRAME_COMPARISON:
            self._advance(); self._expect(TT.LPAREN)
            r1 = self._expect_id(); self._expect(TT.COMMA)
            r2 = self._expect_id(); self._expect(TT.RPAREN)
            return QCFrameComparison(ref1=r1, ref2=r2)
        elif t.type == TT.KW_ARCHITECTURE_VIEW:
            self._advance(); self._expect(TT.LPAREN)
            a = self._expect_id(); self._expect(TT.COMMA)
            cs = self._parse_id_list(); self._expect(TT.COMMA)
            es = self._parse_id_list(); self._expect(TT.COMMA)
            gs = self._parse_id_list(); self._expect(TT.RPAREN)
            return QCArchitectureView(arch=a, cells=cs, evals=es, gates=gs)
        elif t.type == TT.KW_COVERAGE_REPORT:
            self._advance(); self._expect(TT.LPAREN)
            r1 = self._expect_id(); self._expect(TT.COMMA)
            r2 = self._expect_id(); self._expect(TT.COMMA)
            r3 = self._expect_id(); self._expect(TT.RPAREN)
            return QCCoverageReport(ref1=r1, ref2=r2, ref3=r3)
        raise SVPError(E204, t.line, t.col, "Se esperaba constructor de QueryContext")

    def _parse_query_cmd(self, name: str, loc: Loc) -> QueryCmd:
        self._advance()  # query
        self._expect(TT.LPAREN)
        spec = self._expect_id()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_BY)
        by = self._expect_id()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_IN)
        ctx = self._parse_query_context()
        self._expect(TT.RPAREN)
        self._expect(TT.SEMICOLON)
        return QueryCmd(name=name, spec=spec, by=by, context=ctx, loc=loc)

    def _parse_supervisable(self) -> SupervisableTarget:
        t = self._cur()
        if t.type == TT.KW_CELL_TARGET:
            self._advance(); self._expect(TT.LPAREN)
            r = self._expect_id(); self._expect(TT.RPAREN)
            return CellTarget(ref=r)
        elif t.type == TT.KW_COMPOSED_TARGET:
            self._advance(); self._expect(TT.LPAREN)
            r = self._expect_id(); self._expect(TT.RPAREN)
            return ComposedTarget(ref=r)
        elif t.type == TT.KW_SYSTEM_TARGET:
            self._advance(); self._expect(TT.LPAREN)
            r = self._expect_id(); self._expect(TT.RPAREN)
            return SystemTarget(ref=r)
        raise SVPError(E205, t.line, t.col,
                       "Se esperaba CellTarget, ComposedTarget o SystemTarget")

    def _parse_supervise_cmd(self, name: str, loc: Loc) -> SuperviseCmd:
        self._advance()  # supervise
        self._expect(TT.LPAREN)
        meta = self._expect_id()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_TARGET)
        target = self._parse_supervisable()
        self._expect(TT.RPAREN)
        self._expect(TT.SEMICOLON)
        return SuperviseCmd(name=name, meta_eval=meta, target=target, loc=loc)

    def _parse_compose_cmd(self, name: str, loc: Loc) -> ComposeCmd:
        self._advance()  # compose
        self._expect(TT.LPAREN)
        graph = self._expect_id()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_RELATIONS)
        rels = self._parse_id_list()
        self._expect(TT.COMMA)
        self._kw_field(TT.KW_PATTERNS)
        pats = self._parse_id_list()
        self._expect(TT.RPAREN)
        self._expect(TT.SEMICOLON)
        return ComposeCmd(name=name, graph=graph, relations=rels, patterns=pats, loc=loc)
