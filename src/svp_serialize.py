"""
svp_serialize.py — Serialización canónica IR → JSON

Produce JSON determinista: UTF-8, claves ordenadas, formato estable,
sin variación arbitraria entre ejecuciones. Salida reproducible.

Autor: Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351
ISSN 2695-6411 | CC BY-NC-ND 4.0
"""

import json
from svp_ir import IRProgram


def serialize(ir: IRProgram) -> str:
    """Serializa un IRProgram a JSON canónico determinista."""
    doc = {
        "grammar_version": ir.grammar_version,
        "ir_version": ir.ir_version,
        "serializer_version": ir.serializer_version,
        "source_file": ir.source_file,
        "source_sha256": ir.source_sha256,
        "objects": [obj.to_dict() for obj in ir.objects],
        "operations": [op.to_dict() for op in ir.operations],
    }
    return json.dumps(doc, indent=2, sort_keys=True, ensure_ascii=False)
