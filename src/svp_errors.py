"""
svp_errors.py — Catálogo centralizado de errores del lenguaje SV

Todos los errores E001–E507 definidos en la IR canónica v0.2.
Cada error lleva código, nombre canónico, mensaje base, nivel y fase de emisión.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

from dataclasses import dataclass
from enum import Enum
from typing import Optional


class ErrorPhase(Enum):
    PARSE = "parse"
    VALIDATE = "validate"
    LOWER = "lower"


class ErrorLevel(Enum):
    LAYER0 = 0  # Definición
    LAYER1 = 1  # Estado
    LAYER2 = 2  # Resultado
    LAYER3 = 3  # Evolución
    LAYER4 = 4  # Uso
    LAYER5 = 5  # Serialización/conformidad


@dataclass(frozen=True)
class SVPErrorDef:
    code: str
    name: str
    message: str
    level: ErrorLevel
    phase: ErrorPhase


# ── Capa 0 — Definición ──────────────────────────────────────────────

E001 = SVPErrorDef("E001", "InvalidTriValue",
    "Valor ternario no reconocido: se esperaba Zero, One o U",
    ErrorLevel.LAYER0, ErrorPhase.PARSE)

E002 = SVPErrorDef("E002", "InvalidBValue",
    "El valor de b debe ser un entero >= 3",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E003 = SVPErrorDef("E003", "NSquaredViolation",
    "n debe ser b² (derivado automáticamente, no declarado por el usuario)",
    ErrorLevel.LAYER0, ErrorPhase.LOWER)

E004 = SVPErrorDef("E004", "EmptyCodomain",
    "El codominio declarado no puede estar vacío",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E005 = SVPErrorDef("E005", "DuplicateIdentifier",
    "Identificador ya declarado en el ámbito actual",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E006 = SVPErrorDef("E006", "UndeclaredReference",
    "Referencia a identificador no declarado",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E007 = SVPErrorDef("E007", "InvalidConnectorMapping",
    "El mapeo del conector no cubre todos los valores del codominio fuente",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E008 = SVPErrorDef("E008", "ConnectorTargetNotTri",
    "El destino del mapeo del conector debe ser un literal ternario (Zero, One, U)",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E009 = SVPErrorDef("E009", "TableInputMismatch",
    "Las entradas de la tabla de admisibilidad no cubren el producto cartesiano de los codominios",
    ErrorLevel.LAYER0, ErrorPhase.VALIDATE)

E010 = SVPErrorDef("E010", "InvalidRole",
    "Rol no reconocido: se esperaba Base, Supervisor o Composite",
    ErrorLevel.LAYER0, ErrorPhase.PARSE)

# ── Capa 1 — Estado ──────────────────────────────────────────────────

E101 = SVPErrorDef("E101", "VectorLengthMismatch",
    "La longitud del vector no coincide con b² de la CellSpec referenciada",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

E102 = SVPErrorDef("E102", "SpecNotFound",
    "La CellSpec o CoupledSpec referenciada no existe",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

E103 = SVPErrorDef("E103", "GraphCycleDetected",
    "El grafo de composición contiene ciclos (prohibido)",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

E104 = SVPErrorDef("E104", "GraphMissingRelation",
    "El grafo de composición no tiene relación semántica declarada",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

E105 = SVPErrorDef("E105", "BridgePositionOutOfRange",
    "Posición puente fuera del rango [1, n] de la célula",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

E106 = SVPErrorDef("E106", "MissingSemanticRelation",
    "Composición sin relación semántica previa declarada",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

E111 = SVPErrorDef("E111", "UnorderedCodomain",
    "Codominio usado en compuerta sin orden documentado",
    ErrorLevel.LAYER1, ErrorPhase.VALIDATE)

# ── Capa 2 — Resultado ───────────────────────────────────────────────

E201 = SVPErrorDef("E201", "LiteralResultForbidden",
    "No se permite construir literales de EvalResult, GateResult, ResolutionRecord, QueryResult ni SupervisionResult",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E202 = SVPErrorDef("E202", "GateInputNotEvalResult",
    "Los argumentos de gate deben ser identificadores de EvalResult",
    ErrorLevel.LAYER2, ErrorPhase.VALIDATE)

E203 = SVPErrorDef("E203", "GateMissingTable",
    "gate invocado sin tabla de admisibilidad nombrada (using)",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E204 = SVPErrorDef("E204", "QueryMissingContext",
    "query invocado sin constructor explícito de QueryContext",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E205 = SVPErrorDef("E205", "SuperviseOpaqueTarget",
    "supervise invocado sin constructor explícito de Supervisable",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E206 = SVPErrorDef("E206", "ResolveMissingContext",
    "resolve invocado sin contexto de evidencia",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E207 = SVPErrorDef("E207", "ResolveMissingMechanism",
    "resolve invocado sin mecanismo de revisión",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E208 = SVPErrorDef("E208", "ComposeMissingRelations",
    "compose invocado sin lista de relaciones",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E209 = SVPErrorDef("E209", "ComposeMissingPatterns",
    "compose invocado sin lista de patrones",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E210 = SVPErrorDef("E210", "MaxMinForbidden",
    "max/min no están disponibles en la superficie v0.1",
    ErrorLevel.LAYER2, ErrorPhase.PARSE)

E211 = SVPErrorDef("E211", "SuperviseMetaNotSupervisor",
    "El primer argumento de supervise debe provenir de una célula con rol Supervisor",
    ErrorLevel.LAYER2, ErrorPhase.VALIDATE)

# ── Capa 3 — Evolución ───────────────────────────────────────────────

E301 = SVPErrorDef("E301", "FrameMutationForbidden",
    "No se permite modificar un Frame existente (inmutable por tipo)",
    ErrorLevel.LAYER3, ErrorPhase.VALIDATE)

E302 = SVPErrorDef("E302", "TrajectoryMutationForbidden",
    "No se permite modificar, eliminar ni reordenar entradas de una Trajectory (append-only por tipo)",
    ErrorLevel.LAYER3, ErrorPhase.VALIDATE)

E303 = SVPErrorDef("E303", "TransitionDataMissingHorizon",
    "TransitionData declarado sin referencia a Horizon",
    ErrorLevel.LAYER3, ErrorPhase.VALIDATE)

E304 = SVPErrorDef("E304", "TrajectoryAlternanceViolation",
    "La secuencia de entradas de Trajectory no respeta las invariantes de alternancia",
    ErrorLevel.LAYER3, ErrorPhase.VALIDATE)

# ── Capa 4 — Uso ─────────────────────────────────────────────────────

E401 = SVPErrorDef("E401", "DomainMissingHorizon",
    "Domain declarado sin horizonte de sucesos",
    ErrorLevel.LAYER4, ErrorPhase.VALIDATE)

E402 = SVPErrorDef("E402", "AgentMissingDomain",
    "Agent declarado sin dominio",
    ErrorLevel.LAYER4, ErrorPhase.VALIDATE)

E403 = SVPErrorDef("E403", "QuerySpecInvalid",
    "QuerySpec con tipo o alcance no reconocido",
    ErrorLevel.LAYER4, ErrorPhase.VALIDATE)

# ── Capa 5 — Serialización / Conformidad ─────────────────────────────

E501 = SVPErrorDef("E501", "SerializationNonDeterministic",
    "La serialización JSON no es determinista",
    ErrorLevel.LAYER5, ErrorPhase.LOWER)

E507 = SVPErrorDef("E507", "UCoercionDetected",
    "Coerción implícita de U detectada (prohibición constitutiva)",
    ErrorLevel.LAYER5, ErrorPhase.VALIDATE)


# ── Catálogo completo ────────────────────────────────────────────────

ERRORS = {e.code: e for e in [
    E001, E002, E003, E004, E005, E006, E007, E008, E009, E010,
    E101, E102, E103, E104, E105, E106, E111,
    E201, E202, E203, E204, E205, E206, E207, E208, E209, E210, E211,
    E301, E302, E303, E304,
    E401, E402, E403,
    E501, E507,
]}


@dataclass
class SVPError(Exception):
    """Error emitido durante el procesamiento de un archivo .svp."""
    error_def: SVPErrorDef
    line: Optional[int] = None
    col: Optional[int] = None
    detail: str = ""

    def __str__(self):
        loc = f" [{self.line}:{self.col}]" if self.line is not None else ""
        det = f" — {self.detail}" if self.detail else ""
        return f"{self.error_def.code} ({self.error_def.name}){loc}: {self.error_def.message}{det}"
