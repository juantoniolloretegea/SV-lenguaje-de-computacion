"""
svp_lexer.py — Tokenizador de archivos .svp

Reconoce las palabras clave, literales, puntuación y estructura
definidas en la Gramática superficial mínima v0.1.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

from dataclasses import dataclass
from enum import Enum, auto
from typing import List

from svp_errors import SVPError, E001


class TT(Enum):
    """Token types."""
    # Literals
    IDENTIFIER = auto()
    NAT = auto()
    STRING = auto()

    # Ternary
    ZERO = auto()
    ONE = auto()
    U_LIT = auto()

    # Keywords — declarations
    KW_CODOMAIN = auto()
    KW_OUTPUT_SEMANTICS = auto()
    KW_CELLSPEC = auto()
    KW_COUPLEDSPEC = auto()
    KW_CONNECTOR = auto()
    KW_ADMISSIBILITY_TABLE = auto()
    KW_CAPTURE_SPEC = auto()
    KW_ADMISSIBILITY_SPEC = auto()
    KW_TERNARIZER = auto()
    KW_RES_SPEC = auto()
    KW_CELLSTATE = auto()
    KW_COUPLEDSTATE = auto()
    KW_GRAPH = auto()
    KW_SEMANTIC_RELATION = auto()
    KW_PATTERN = auto()
    KW_HORIZON = auto()
    KW_FRAME = auto()
    KW_TRANSITION_DATA = auto()
    KW_TRAJECTORY = auto()
    KW_DOMAIN = auto()
    KW_AGENT = auto()
    KW_QUERY_SPEC = auto()

    # Keywords — commands
    KW_LET = auto()
    KW_EVALUATE = auto()
    KW_GATE = auto()
    KW_RESOLVE = auto()
    KW_QUERY = auto()
    KW_SUPERVISE = auto()
    KW_COMPOSE = auto()

    # Keywords — fields
    KW_USING = auto()
    KW_WITH = auto()
    KW_CONTEXT = auto()
    KW_MECHANISM = auto()
    KW_BY = auto()
    KW_IN = auto()
    KW_TARGET = auto()
    KW_RELATIONS = auto()
    KW_PATTERNS = auto()

    # Keywords — roles
    KW_BASE = auto()
    KW_SUPERVISOR = auto()
    KW_COMPOSITE = auto()

    # Keywords — regimes
    KW_SIMPLE = auto()
    KW_GENERAL = auto()

    # Keywords — supervisable constructors
    KW_CELL_TARGET = auto()
    KW_COMPOSED_TARGET = auto()
    KW_SYSTEM_TARGET = auto()

    # Keywords — query context constructors
    KW_POINT_EVAL = auto()
    KW_TRAJECTORY_VIEW = auto()
    KW_FRAME_COMPARISON = auto()
    KW_ARCHITECTURE_VIEW = auto()
    KW_COVERAGE_REPORT = auto()

    # Keywords — query types
    KW_POINT_EVALUATION = auto()
    KW_TRAJECTORY_STATE = auto()
    KW_FRAME_COMPARISON_TYPE = auto()
    KW_COVERAGE_STATE = auto()
    KW_PENDING_U = auto()
    KW_GLOBAL_CRITICALITY = auto()

    # Keywords — scopes
    KW_CELL = auto()
    KW_PAIR = auto()
    KW_ARCHITECTURE = auto()
    # KW_TRAJECTORY reused

    # Keywords — relation/pattern kinds
    KW_DECLARED_RELATION = auto()
    KW_DECLARED_PATTERN = auto()

    # Keywords — misc
    KW_EDGE = auto()
    KW_ENTRY = auto()
    KW_BOTTOM = auto()
    KW_TRUE = auto()

    # Punctuation
    LBRACE = auto()
    RBRACE = auto()
    LBRACKET = auto()
    RBRACKET = auto()
    LPAREN = auto()
    RPAREN = auto()
    SEMICOLON = auto()
    COLON = auto()
    COMMA = auto()
    EQUALS = auto()
    ARROW = auto()   # ->
    DOT = auto()

    # Prohibited keywords (must produce error)
    KW_MAX = auto()
    KW_MIN = auto()
    KW_NULL = auto()
    KW_NONE = auto()
    KW_NAN = auto()

    EOF = auto()


KEYWORDS = {
    "codomain": TT.KW_CODOMAIN,
    "output_semantics": TT.KW_OUTPUT_SEMANTICS,
    "cellspec": TT.KW_CELLSPEC,
    "coupledspec": TT.KW_COUPLEDSPEC,
    "connector": TT.KW_CONNECTOR,
    "admissibility_table": TT.KW_ADMISSIBILITY_TABLE,
    "capture_spec": TT.KW_CAPTURE_SPEC,
    "admissibility_spec": TT.KW_ADMISSIBILITY_SPEC,
    "ternarizer": TT.KW_TERNARIZER,
    "res_spec": TT.KW_RES_SPEC,
    "cellstate": TT.KW_CELLSTATE,
    "coupledstate": TT.KW_COUPLEDSTATE,
    "graph": TT.KW_GRAPH,
    "semantic_relation": TT.KW_SEMANTIC_RELATION,
    "pattern": TT.KW_PATTERN,
    "horizon": TT.KW_HORIZON,
    "frame": TT.KW_FRAME,
    "transition_data": TT.KW_TRANSITION_DATA,
    "trajectory": TT.KW_TRAJECTORY,
    "domain": TT.KW_DOMAIN,
    "agent": TT.KW_AGENT,
    "query_spec": TT.KW_QUERY_SPEC,
    "let": TT.KW_LET,
    "evaluate": TT.KW_EVALUATE,
    "gate": TT.KW_GATE,
    "resolve": TT.KW_RESOLVE,
    "query": TT.KW_QUERY,
    "supervise": TT.KW_SUPERVISE,
    "compose": TT.KW_COMPOSE,
    "using": TT.KW_USING,
    "with": TT.KW_WITH,
    "context": TT.KW_CONTEXT,
    "mechanism": TT.KW_MECHANISM,
    "by": TT.KW_BY,
    "in": TT.KW_IN,
    "target": TT.KW_TARGET,
    "relations": TT.KW_RELATIONS,
    "patterns": TT.KW_PATTERNS,
    "Base": TT.KW_BASE,
    "Supervisor": TT.KW_SUPERVISOR,
    "Composite": TT.KW_COMPOSITE,
    "Simple": TT.KW_SIMPLE,
    "General": TT.KW_GENERAL,
    "CellTarget": TT.KW_CELL_TARGET,
    "ComposedTarget": TT.KW_COMPOSED_TARGET,
    "SystemTarget": TT.KW_SYSTEM_TARGET,
    "PointEval": TT.KW_POINT_EVAL,
    "TrajectoryView": TT.KW_TRAJECTORY_VIEW,
    "FrameComparison": TT.KW_FRAME_COMPARISON,
    "ArchitectureView": TT.KW_ARCHITECTURE_VIEW,
    "CoverageReport": TT.KW_COVERAGE_REPORT,
    "PointEvaluation": TT.KW_POINT_EVALUATION,
    "TrajectoryState": TT.KW_TRAJECTORY_STATE,
    "FrameComparison": TT.KW_FRAME_COMPARISON_TYPE,
    "CoverageState": TT.KW_COVERAGE_STATE,
    "PendingU": TT.KW_PENDING_U,
    "GlobalCriticality": TT.KW_GLOBAL_CRITICALITY,
    "Cell": TT.KW_CELL,
    "Pair": TT.KW_PAIR,
    "Architecture": TT.KW_ARCHITECTURE,
    "Trajectory": TT.KW_TRAJECTORY,
    "DeclaredRelation": TT.KW_DECLARED_RELATION,
    "DeclaredPattern": TT.KW_DECLARED_PATTERN,
    "edge": TT.KW_EDGE,
    "entry": TT.KW_ENTRY,
    "Bottom": TT.KW_BOTTOM,
    "True": TT.KW_TRUE,
    "Zero": TT.ZERO,
    "One": TT.ONE,
    "U": TT.U_LIT,
    # Prohibited
    "max": TT.KW_MAX,
    "min": TT.KW_MIN,
    "null": TT.KW_NULL,
    "None": TT.KW_NONE,
    "NaN": TT.KW_NAN,
}

# Tokens that are prohibited — parser must reject these
PROHIBITED_TOKENS = {TT.KW_MAX, TT.KW_MIN, TT.KW_NULL, TT.KW_NONE, TT.KW_NAN}


@dataclass(frozen=True)
class Token:
    type: TT
    value: str
    line: int
    col: int


def tokenize(source: str, filename: str = "<stdin>") -> List[Token]:
    """Tokeniza un archivo .svp completo. Devuelve lista de tokens."""
    tokens: List[Token] = []
    i = 0
    line = 1
    col = 1
    n = len(source)

    while i < n:
        ch = source[i]

        # Whitespace
        if ch in " \t\r":
            i += 1
            col += 1
            continue
        if ch == "\n":
            i += 1
            line += 1
            col = 1
            continue

        # Comments: -- to end of line
        if ch == "-" and i + 1 < n and source[i + 1] == "-":
            while i < n and source[i] != "\n":
                i += 1
            continue

        # Arrow: ->
        if ch == "-" and i + 1 < n and source[i + 1] == ">":
            tokens.append(Token(TT.ARROW, "->", line, col))
            i += 2
            col += 2
            continue

        # String literal
        if ch == '"':
            start_col = col
            i += 1
            col += 1
            s = []
            while i < n and source[i] != '"':
                if source[i] == "\n":
                    line += 1
                    col = 1
                else:
                    col += 1
                s.append(source[i])
                i += 1
            if i >= n:
                raise SVPError(E001, line, start_col, "Cadena no cerrada")
            i += 1  # skip closing quote
            col += 1
            tokens.append(Token(TT.STRING, "".join(s), line, start_col))
            continue

        # Punctuation
        punct = {
            "{": TT.LBRACE, "}": TT.RBRACE,
            "[": TT.LBRACKET, "]": TT.RBRACKET,
            "(": TT.LPAREN, ")": TT.RPAREN,
            ";": TT.SEMICOLON, ":": TT.COLON,
            ",": TT.COMMA, "=": TT.EQUALS,
            ".": TT.DOT,
        }
        if ch in punct:
            tokens.append(Token(punct[ch], ch, line, col))
            i += 1
            col += 1
            continue

        # Numbers
        if ch.isdigit():
            start_col = col
            num = []
            while i < n and source[i].isdigit():
                num.append(source[i])
                i += 1
                col += 1
            tokens.append(Token(TT.NAT, "".join(num), line, start_col))
            continue

        # Identifiers and keywords
        if ch.isalpha() or ch == "_":
            start_col = col
            word = []
            while i < n and (source[i].isalnum() or source[i] == "_"):
                word.append(source[i])
                i += 1
                col += 1
            w = "".join(word)
            tt = KEYWORDS.get(w, TT.IDENTIFIER)
            tokens.append(Token(tt, w, line, start_col))
            continue

        # Unknown character
        raise SVPError(E001, line, col, f"Carácter no reconocido: {ch!r}")

    tokens.append(Token(TT.EOF, "", line, col))
    return tokens
