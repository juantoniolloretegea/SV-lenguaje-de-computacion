# Acta técnica de cierre FFL-B — E011 y codominio de salida de `AdmissibilityTable`

**Fecha:** 19/08/2026  
**Estado:** CERRADO  
**Ámbito:** Lenguaje SV / FFL-B / `AdmissibilityTable` / validación de definición  
**Base técnica:** IR canónica v0.2, definición de `AdmissibilityTable` y J1.4

## 1. Objeto

Esta acta registra el cierre de una obligación de tipado ya representada en la IR y en la superficie vigente: toda salida literal de una fila de `AdmissibilityTable` debe pertenecer al `output_codomain` declarado por la propia tabla.

El cierre no modifica la gramática superficial ni la IR canónica. Materializa en la validación una condición que ya deriva de la firma:

`table : [Codomain] -> Codomain`

## 2. Fundamento

La IR v0.2 define `AdmissibilityTable` mediante los campos `input_codomains`, `output_codomain` y `table`. La tabla produce valores del codominio de salida declarado. J1.4 exige además completitud, determinismo, documentación de la asimetría cuando exista y relación semántica previa.

La tabla canónica de errores de la IR no asigna un código autónomo a la pertenencia de la salida literal al `output_codomain`. Bajo la regularización vigente por Vía B se utiliza el identificador efectivo libre:

`E011 — TableOutputNotInCodomain`

Este diagnóstico no sustituye ni redefine `E105 — IncompleteAdmissibilityTable` ni `E106 — MissingSemanticRelation` de la IR canónica.

## 3. Materialización

La validación efectiva realiza las siguientes operaciones:

1. comprueba que `output_codomain` referencia un `CodomainDecl` existente;
2. obtiene el conjunto de valores declarados por ese codominio;
3. comprueba cada salida literal de `table`;
4. emite `E011` si alguna salida no pertenece al codominio declarado;
5. conserva de forma separada `E009 — TableInputMismatch` para faltantes, elementos adicionales o duplicados en el producto cartesiano de entradas.

El caso negativo específico es:

`tests/conformance/invalid/admissibility_table_output_fuera_codominio.svp`

## 4. Evidencia

La modificación funcional fue comprobada en la rama `agent/ffl-b-e011-table-output`, sobre la base `7d9da2580a0301a7efe346389011a8bc66cbab07`, con la confirmación funcional:

`c42ea2b7f8ba5a52143107ae3058e36bc8154a98`

Una verificación independiente en modo de solo lectura confirmó:

- batería de conformidad: **46/46**, código de salida 0;
- pruebas rápidas de la interfaz de línea de órdenes: **3/3**, código de salida 0;
- SEC-0: **3/3**, código de salida 0;
- `admissibility_table_output_fuera_codominio.svp`: emisión exacta de `E011 — TableOutputNotInCodomain`;
- `admissibility_table_incompleta.svp`: conservación de `E009 — TableInputMismatch`;
- `gate_table.svp`: aceptación y producción de IR canónica.

La comparación entre la base y la confirmación funcional comprende únicamente el alta de `E011`, su punto de emisión, un caso negativo específico y su asociación en la batería de conformidad. La normalización del salto de línea final de `src/svp_validator.py` no altera el comportamiento.

## 5. Alcance y límites

Este cierre acredita exclusivamente:

`toda salida literal de AdmissibilityTable.table pertenece a output_codomain`.

No acredita por extensión:

- cierre completo de J1.4;
- ejecución material de `GateResult`;
- determinismo ejecutivo general de `gate`;
- documentación o comprobación de asimetrías;
- satisfacción completa de la relación semántica previa `R(A)`;
- correspondencia de `E011` con un identificador canónico autónomo inexistente en la tabla de errores de la IR v0.2.

## 6. Estado resultante

`E011 — TableOutputNotInCodomain` queda materializado y cubierto por un caso explícito de conformidad dentro del contrato diagnóstico efectivo bajo Vía B.

FFL-B permanece abierto. Este cierre no abre FFL-C, FFL-D, FFL-E, infraestructura de ejecución, Rust, WASM, IA propia ni programación de SVP mediante lenguaje natural.
