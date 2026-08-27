"""Perfil léxico cerrado de identificadores SVP para la Gramática 0.2.

El perfil es deliberadamente finito. La admisión de un carácter no depende de
``str.isalpha`` ni de la versión Unicode del intérprete.
"""

PROFILE_ID = "svp-grammar-0.2-lex-es-1"
SPANISH_IDENTIFIER_LETTERS = frozenset(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzÁÉÍÓÚÜÑáéíóúüñ"
)


def is_identifier_start(ch: str) -> bool:
    return len(ch) == 1 and ch in SPANISH_IDENTIFIER_LETTERS


def is_identifier_continue(ch: str) -> bool:
    return is_identifier_start(ch) or ch == "_" or ("0" <= ch <= "9")
