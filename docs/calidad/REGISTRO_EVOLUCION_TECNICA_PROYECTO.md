# Registro de evolución técnica del proyecto

## 1. Finalidad

Este registro ofrece la lectura humana del tramo vivo del historial técnico del Lenguaje SV. El CSV maestro `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` conserva la autoridad registral de la numeración RETP.

El detalle de `RETP-2026-000` a `RETP-2026-047` permanece preservado en:

`docs/calidad/historico/REGISTRO_EVOLUCION_TECNICA_PROYECTO_HASTA_RETP_2026_047.md`.

La lectura de continuidad es:

`histórico 000–047 → registro vivo desde 048 → CSV maestro como autoridad de numeración`.

## 2. Tabla del tramo vivo

| ID | Fecha | Hora | Tipo | Frente / fase | Estado |
|---|---|---|---|---|---|
| RETP-2026-048 | 18/08/2026 | 11:34:13 | REAPERTURA_GOBERNADA | Lenguaje SV / Ruta A / retorno a FFL-A | cerrado |
| RETP-2026-049 | 18/08/2026 | 11:58:10 | CIERRE_BLOQUE_Y_APERTURA_SECUENCIAL | Lenguaje SV / FFL-A → FFL-B | cerrado |
| RETP-2026-050 | 18/08/2026 | 12:28:08 | DECISION_GOBIERNO_TECNICO | Lenguaje SV / FFL-B / materialización diagnóstica subordinada | cerrado |
| RETP-2026-051 | 18/08/2026 | 13:05:54 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.2 parcial / E112 | cerrado |
| RETP-2026-052 | 18/08/2026 | 13:15:04 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.3 / E113 ↔ E206 canónico | cerrado |
| RETP-2026-053 | 18/08/2026 | 13:25:03 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J4.3 / E307 ↔ E403 canónico | cerrado |
| RETP-2026-054 | 18/08/2026 | 13:30:27 | SORPRESA_TECNICA_Y_REVERSION | Lenguaje SV / FFL-B / intento E406 revertido | cerrado |
| RETP-2026-055 | 18/08/2026 | 21:35:25 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / P0-A / contrato de estado evaluable | cerrado |
| RETP-2026-056 | 18/08/2026 | 21:35:25 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / P0-B / J3.3 / E212-E211 | cerrado |
| RETP-2026-057 | 18/08/2026 | 22:11:00 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J4.3 / E406 mínimo | cerrado |
| RETP-2026-058 | 19/08/2026 | 06:32:40 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J1.4 / E011 / codominio de salida de `AdmissibilityTable` | cerrado |
| RETP-2026-059 | 19/08/2026 | 07:03:00 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / proyección estructural / E213-E214 | cerrado |
| RETP-2026-060 | 19/08/2026 | 08:20:00 | RECEPCION_DOCTRINAL_LATENTE | Lenguaje SV / fundamentos / no clausura certificada | cerrado |

## 3. Entradas detalladas

### RETP-2026-048 — Reapertura por Ruta A

- **Hecho:** se levanta la pausa preventiva del 16/08/2026 en el alcance autorizado y se reabre el frente técnico con FFL-A como prioridad inmediata.
- **Fundamento:** cierre funcional del antiguo programa de gobierno determinista, publicación y cierre de la Dinámica del Suceso y microauditoría Potencial/Tesauro resuelta por Ruta A.
- **Evidencia:** acta pública de reapertura incorporada a `main` y comprobación del repositorio vigente.
- **Decisión:** reabrir únicamente desde FFL-A; mantener cerradas las compuertas independientes no autorizadas.
- **Estado:** cerrado.

### RETP-2026-049 — Cierre de FFL-A y apertura secuencial de FFL-B

- **Hecho:** FFL-A se cierra bajo Vía B con deuda residual explícita y gobernada; FFL-B pasa a ser el único bloque técnico inmediato.
- **Fundamento:** matriz de concordancia, tabla de correspondencias funcionales y criterios de cierre del frente final.
- **Evidencia:** tablero de bloques y documentación específica de cierre de FFL-A.
- **Decisión:** no exigir convergencia nominal total cuando la deuda restante esté localizada y gobernada; no abrir FFL-C/D/E de forma simultánea.
- **Estado:** cerrado.

### RETP-2026-050 — Regla de materialización diagnóstica de FFL-B

- **Hecho:** se fija que FFL-B sólo materializará obligaciones ya representables y sustentadas por fundamento expreso, ruta diagnóstica inequívoca y prueba trazable.
- **Fundamento:** IR canónica v0.2, Vía B, acta de reapertura y procedimiento de verificación técnica.
- **Decisión:** trabajar mediante unidades técnicas acotadas; cada nuevo diagnóstico efectivo deberá sincronizar código, emisión, prueba, catálogo, matriz, correspondencias funcionales y deuda viva.
- **Estado:** cerrado.

### RETP-2026-051 — J2.2 parcial / E112

- **Hecho:** se impone que las diferencias entre `base_vector` y `updated_vector` de `CoupledState` sólo afecten a posiciones pertenecientes al `BridgeSet` correspondiente.
- **Fundamento:** J2.2 de la IR v0.2 y datos ya presentes en AST y validador.
- **Límite:** no queda acreditada la procedencia completa de cada actualización desde un `Connector` concreto.
- **Decisión:** cerrar únicamente la cláusula posicional mediante `E112`.
- **Estado:** cerrado.

### RETP-2026-052 — J2.3 / E113 ↔ E206 canónico

- **Hecho:** se materializan las compatibilidades ya representadas entre arista, posición puente y conector.
- **Fundamento:** J2.3 y E206 canónico; la numeración E206 efectiva estaba ocupada por otra obligación bajo Vía B.
- **Decisión:** utilizar `E113` como ruta efectiva y documentar expresamente la correspondencia funcional, sin renumeración masiva.
- **Estado:** cerrado.

### RETP-2026-053 — J4.3 / E307 ↔ E403 canónico

- **Hecho:** cada tipo de suceso declarado en `TransitionData.events` debe pertenecer al `Horizon` referenciado.
- **Fundamento:** J4.3, E403 canónico y representación ya existente de `TransitionData.events` y `Horizon.events`.
- **Decisión:** materializar la obligación mediante `E307`, sin identificar `TransitionData` con un suceso admisible ni ampliar la semántica eventiva.
- **Estado:** cerrado.

### RETP-2026-054 — Incidencia técnica y reversión del primer intento E406

- **Hecho:** el primer intento de materializar E406 se revirtió al comprobarse que la comparación de cambios excedía el alcance del juicio y reordenaba código ajeno a la modificación necesaria.
- **Fundamento:** disciplina de cambios mínimos y procedimiento de verificación técnica.
- **Decisión:** restaurar íntegramente el estado anterior y mantener E406 pendiente hasta una nueva revisión con alcance estrictamente acotado.
- **Estado:** cerrado.

### RETP-2026-055 — P0-A / estado evaluable

- **Hecho:** `evaluate` vuelve a aceptar `CellState` y `CoupledState`, mientras `Frame.cell_states` conserva exclusivamente `CoupledState`.
- **Fundamento:** gramática v0.1, adenda técnica a la IR v0.2 y matemática de composición intercelular.
- **Evidencia:** verificación independiente con conformidad **42/42**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3** y aceptación de los programas discriminantes.
- **Decisión:** preservar la distinción entre estado simple y acoplado y separar el juicio de `supervise` para P0-B.
- **Estado:** cerrado.

### RETP-2026-056 — P0-B / J3.3 / E212-E211

- **Hecho:** `supervise` exige que `meta_eval` sea un `EvalResult`; `E212` protege el tipo y `E211` protege la procedencia desde una célula con rol `Supervisor`, también por la ruta acoplada.
- **Fundamento:** J3.3, P0-A y Vía B.
- **Evidencia:** verificación independiente con conformidad **44/44**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3** y emisión exacta de E212/E211 en los dos casos negativos específicos.
- **Decisión:** declarar estabilizado P0 en sus dos partes.
- **Estado:** cerrado.

### RETP-2026-057 — J4.3 / E406 mínimo

- **Hecho:** `E406 — InsufficientTransitionData` rechaza `TransitionData` con `induced_parameters` vacío.
- **Fundamento:** J4.3 y tabla canónica de errores de la IR v0.2, que asignan E406 a esa condición exacta.
- **Evidencia:** verificación independiente con conformidad **45/45**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3**, emisión exacta de E406, conservación del caso válido y precedencia de E307.
- **Límite:** no se acredita por este cierre la suficiencia de una lista no vacía para reconstruir el operador inducido.
- **Decisión:** cerrar exclusivamente la cláusula de no-vaciedad y no abrir automáticamente otro juicio técnico.
- **Estado:** cerrado.

### RETP-2026-058 — J1.4 / E011 / codominio de salida de `AdmissibilityTable`

- **Hecho:** `E011 — TableOutputNotInCodomain` rechaza una fila de `AdmissibilityTable` cuya salida literal no pertenece al `output_codomain` declarado.
- **Fundamento:** la IR v0.2 tipa `AdmissibilityTable.table` como función hacia el codominio de salida; la condición estaba representada por los datos existentes y no requería modificar gramática ni IR.
- **Evidencia:** verificación independiente en modo de solo lectura con conformidad **46/46**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3**, emisión exacta de E011, conservación de E009 para tabla incompleta y aceptación de `gate_table.svp`.
- **Límite:** E011 no equivale a E105 ni E106 canónicos, no cierra todo J1.4 y no acredita ejecución material de `GateResult`.
- **Decisión:** cerrar exclusivamente la pertenencia de las salidas literales al `output_codomain` declarado y conservar abiertas las restantes obligaciones que no estén acreditadas por otra ruta.
- **Estado:** cerrado.

### RETP-2026-059 — Proyección estructural / E213-E214

- **Hecho:** `E213 — ProjectionSourceNotResult` rechaza una fuente declarada que no produzca un objeto de resultado proyectable y `E214 — ProjectionFieldNotFound` rechaza un campo ajeno al esquema canónico del resultado correspondiente. Una fuente inexistente conserva `E006` con precedencia.
- **Fundamento:** la superficie v0.1 ya contiene la operación de proyección y la IR v0.2 fija los esquemas de `EvalResult`, `GateResult`, `ResolutionRecord`, `QueryResult` y `SupervisionResult`.
- **Evidencia:** verificación independiente en modo de solo lectura con conformidad **48/48**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3**, emisión exacta de E213/E214 y conservación de `resolve_projection.svp`.
- **Límite:** el cierre es estructural; no ejecuta resultados ni calcula campos. `target`, `context` y `mechanism` pertenecen a esquemas canónicos, pero son palabras reservadas y no están disponibles actualmente como identificadores de campo tras el punto.
- **Decisión:** cerrar las dos precondiciones estructurales de proyección sin modificar gramática, IR ni infraestructura de ejecución.
- **Estado:** cerrado.

### RETP-2026-060 — Recepción latente de no clausura certificada

- **Hecho:** se recibe en la sede operativa la publicación «No clausura certificada en sistemas finitos de resolución» con estatuto `LATENTE_LEGITIMO`, sin modificar código, gramática ni IR.
- **Fundamento:** DOI `10.21428/39829d0b.f0892864`; sede `SV-matematica-semantica/documentos/fundamentos/`; jerarquía doctrina → especificación → lenguaje.
- **Evidencia:** el árbol de `main` en `552a142276c08fdba4db5281d67af774f14ed1f6` no contenía asiento ni acta de recepción; el registro maestro recibe únicamente la fila `RETP-2026-060`.
- **Límite:** el certificado de no clausura no es trabajo pendiente, no es un cuarto valor, no altera `Σ = {0, 1, U}`, no crea operador nuevo, no obliga cambio de lenguaje SVP y no abre AUTH, aprendizaje trazable ni etapa frontal adicional.
- **Decisión:** recibir y preservar; no implementar. La traducción material exigirá un acta arquitectónica previa.
- **Estado:** cerrado.

## 4. Autoridad registral

La numeración y los tipos RETP permanecen gobernados por `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.
