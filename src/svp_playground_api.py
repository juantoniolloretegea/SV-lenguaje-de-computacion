#!/usr/bin/env python3
"""
svp_playground_api.py — API estable para el SVP Playground

Contrato:
- Entrada: código fuente .svp como texto + nombre de archivo
- Salida: JSON envelope estable, siempre serializado como cadena
"""

import json
import hashlib
import os
import sys

# Asegura imports locales desde src/
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from svp_lexer import tokenize
from svp_parser import Parser
from svp_validator import Validator
from svp_ir import Lowering
from svp_serialize import serialize
from svp_errors import SVPError


def compute_sha256(content: str) -> str:
    return hashlib.sha256(content.encode("utf-8")).hexdigest()


def compile_for_playground(source: str, filename: str = "playground.svp") -> str:
    """
    API estable para el playground.

    Devuelve SIEMPRE una cadena JSON con esta forma:

    {
      "ok": true/false,
      "ir_json": "... o null",
      "error": { ... } o null,
      "meta": { ... } o null
    }
    """
    source_hash = compute_sha256(source)

    try:
        # 1. Tokenizar
        tokens = tokenize(source, filename)

        # 2. Parsear -> AST
        parser = Parser(tokens, filename)
        program = parser.parse()

        # 3. Validar bienformación
        validator = Validator(program)
        validator.validate()

        # 4. Lowering -> IR
        lowering = Lowering(program, source_sha256=source_hash)
        ir = lowering.lower()

        # 5. Serializar -> JSON canónico IR
        ir_json = serialize(ir)

        # 6. Extraer metadatos reales desde la propia IR serializada
        ir_payload = json.loads(ir_json)
        meta = {
            "ir_version": ir_payload.get("ir_version"),
            "grammar_version": ir_payload.get("grammar_version"),
            "serializer_version": ir_payload.get("serializer_version"),
            "source_file": ir_payload.get("source_file"),
            "source_sha256": ir_payload.get("source_sha256"),
        }

        envelope = {
            "ok": True,
            "ir_json": ir_json,
            "error": None,
            "meta": meta,
        }

    except SVPError as e:
        envelope = {
            "ok": False,
            "ir_json": None,
            "error": {
                "code": getattr(e, "code", "E000"),
                "message": str(e),
                "line": getattr(e, "line", None),
                "column": getattr(e, "column", None),
            },
            "meta": {
                "source_file": filename,
                "source_sha256": source_hash,
            },
        }

    except Exception as e:
        envelope = {
            "ok": False,
            "ir_json": None,
            "error": {
                "code": "INTERNAL",
                "message": str(e),
                "line": None,
                "column": None,
            },
            "meta": {
                "source_file": filename,
                "source_sha256": source_hash,
            },
        }

    return json.dumps(envelope, ensure_ascii=False, sort_keys=True, indent=2)
