#!/usr/bin/env python3
"""
svp_main.py — CLI del parser/lowering de referencia del lenguaje SV

Uso:
    python svp_main.py archivo.svp [-o archivo.ir.json]

Parsea un archivo .svp, valida la bienformación, baja a IR v0.2
y serializa a JSON canónico.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

import sys
import os
import hashlib
import argparse

# Add src to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from svp_lexer import tokenize
from svp_parser import Parser
from svp_validator import Validator
from svp_ir import Lowering
from svp_serialize import serialize
from svp_errors import SVPError


def compute_sha256(content: str) -> str:
    return hashlib.sha256(content.encode("utf-8")).hexdigest()


def process_file(input_path: str, output_path: str = None) -> str:
    """Procesa un archivo .svp completo. Devuelve el JSON serializado."""
    filename = os.path.basename(input_path)

    with open(input_path, "r", encoding="utf-8") as f:
        source = f.read()

    source_hash = compute_sha256(source)

    # 1. Tokenizar
    tokens = tokenize(source, filename)

    # 2. Parsear → AST
    parser = Parser(tokens, filename)
    program = parser.parse()

    # 3. Validar bienformación
    validator = Validator(program)
    validator.validate()

    # 4. Lowering → IR
    lowering = Lowering(program, source_sha256=source_hash)
    ir = lowering.lower()

    # 5. Serializar → JSON
    json_output = serialize(ir)

    # 6. Escribir salida
    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(json_output)
            f.write("\n")

    return json_output


def main():
    argp = argparse.ArgumentParser(
        description="Parser/lowering de referencia del lenguaje SV (.svp → IR v0.2 JSON)"
    )
    argp.add_argument("input", help="Archivo .svp de entrada")
    argp.add_argument("-o", "--output", help="Archivo .ir.json de salida (por defecto: stdout)")
    args = argp.parse_args()

    if not args.input.endswith(".svp"):
        print(f"Advertencia: el archivo no tiene extensión .svp: {args.input}", file=sys.stderr)

    try:
        result = process_file(args.input, args.output)
        if not args.output:
            print(result)
    except SVPError as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"ERROR INTERNO: {e}", file=sys.stderr)
        sys.exit(2)


if __name__ == "__main__":
    main()
