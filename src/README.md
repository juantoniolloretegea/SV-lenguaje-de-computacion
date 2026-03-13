# src/ — Parser/lowering de referencia del lenguaje SV

## Qué hace

Toma un archivo `.svp` escrito en la Gramática superficial mínima v0.1, lo parsea, aplica la validación implementada vigente —subordinada a la IR canónica v0.2— y produce la representación IR serializada en JSON canónico.

## Qué no hace

No ejecuta programas. No calcula evaluaciones (conteos, T(n), clasificación). No resuelve U materialmente. No ejecuta compuertas como semántica aplicada. No produce salidas de backend. Solo parsea, valida, baja y serializa.

## Uso

```bash
python svp_main.py archivo.svp                    # salida a stdout
python svp_main.py archivo.svp -o archivo.ir.json  # salida a archivo
```

## Módulos

| Archivo | Función |
|---------|---------|
| `svp_errors.py` | Catálogo centralizado de errores E001–E507 |
| `svp_ast.py` | Definiciones del AST (un nodo por declaración/comando) |
| `svp_lexer.py` | Tokenizador de archivos `.svp` |
| `svp_parser.py` | Parser de descenso recursivo (EBNF → AST) |
| `svp_validator.py` | Verificación implementada de bienformación; la concordancia fuerte total con J0.1–J5.2 sigue pendiente |
| `svp_ir.py` | Objetos IR v0.2 + lowering (AST → IR) |
| `svp_serialize.py` | Serialización IR → JSON canónico |
| `svp_main.py` | CLI (archivo.svp → archivo.ir.json) |

## Alcance

Implementa exactamente las 22 declaraciones y los 7 comandos de la Gramática superficial mínima v0.1. Cero azúcar. Cero extensiones. Cero funcionalidad más allá de lo especificado.

La cobertura diagnóstica actual no equivale todavía a la totalidad de los juicios J0.1–J5.2 ni cierra completamente la capa N4/Uso. La convergencia fuerte entre IR, catálogo, validator, suite y documentación pública sigue siendo deuda futura del proyecto.

## JSON canónico de salida

- Codificación: UTF-8
- Claves: ordenadas alfabéticamente
- Salida: determinista, reproducible entre ejecuciones
- Campos de cabecera: `ir_version`, `grammar_version`, `serializer_version`, `source_file`, `source_sha256`

## Subordinación

Gramática superficial mínima v0.1 → IR canónica v0.2 → Frontera normativa v0 → Fundamentos (R3).

---

*Lenguaje de computación del Sistema Vectorial SV.*
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0 | ISSN 2695-6411*
