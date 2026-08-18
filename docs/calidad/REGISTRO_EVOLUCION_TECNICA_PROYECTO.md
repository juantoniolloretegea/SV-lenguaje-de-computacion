# Registro de evolución técnica del proyecto

## 1. Finalidad

Este registro ofrece la lectura humana del tramo vivo del historial técnico del Lenguaje SV. El CSV maestro `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv` conserva la autoridad registral de la numeración RETP.

El detalle de `RETP-2026-000` a `RETP-2026-047` permanece preservado en:

`docs/calidad/historico/REGISTRO_EVOLUCION_TECNICA_PROYECTO_HASTA_RETP_2026_047.md`.

La lectura de continuidad es:

`histórico 000–047 → registro vivo desde 048 → CSV maestro como autoridad de numeración`.

Las entradas de este archivo se redactan conforme a `CRITERIO_DE_REDACCION_PUBLICA_DE_LOS_REGISTROS_DE_CALIDAD_2026_08_18.md`.

## 2. Tabla del tramo vivo

| ID | Fecha | Hora | Tipo | Frente / fase | Estado |
|---|---|---|---|---|---|
| RETP-2026-048 | 18/08/2026 | 11:34:13 | REAPERTURA_GOBERNADA | Lenguaje SV / Ruta A / retorno a FFL-A | cerrado |
| RETP-2026-049 | 18/08/2026 | 11:58:10 | CIERRE_BLOQUE_Y_APERTURA_SECUENCIAL | Lenguaje SV / FFL-A → FFL-B | cerrado |
| RETP-2026-050 | 18/08/2026 | 12:28:08 | DECISION_GOBIERNO_TECNICO | Lenguaje SV / FFL-B / materialización diagnóstica subordinada | cerrado |
| RETP-2026-051 | 18/08/2026 | 13:05:54 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.2 parcial / E112 | cerrado |
| RETP-2026-052 | 18/08/2026 | 13:15:04 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J2.3 / E113 ↔ E206 canónico | cerrado |
| RETP-2026-053 | 18/08/2026 | 13:25:03 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J4.3 / E307 ↔ E403 canónico | cerrado |
| RETP-2026-054 | 18/08/2026 | 13:30:27 | INCIDENCIA_TECNICA_Y_REVERSION | Lenguaje SV / FFL-B / intento E406 revertido | cerrado |
| RETP-2026-055 | 18/08/2026 | 21:35:25 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / P0-A / contrato de estado evaluable | cerrado |
| RETP-2026-056 | 18/08/2026 | 21:35:25 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / P0-B / J3.3 / E212-E211 | cerrado |
| RETP-2026-057 | 18/08/2026 | 22:11:00 | CAMBIO_FUNCIONAL_GOBERNADO | Lenguaje SV / FFL-B / J4.3 / E406 mínimo | cerrado |

> El tipo histórico `SORPRESA_TECNICA_Y_REVERSION` de RETP-054 se conserva en el CSV maestro. En esta lectura humana se expresa como **incidencia técnica y reversión** por adecuación al criterio de redacción pública, sin alterar el hecho registrado.

## 3. Entradas detalladas

### RETP-2026-048 — Reapertura por Ruta A

- **Hecho:** se levanta la pausa preventiva del 16/08/2026 en el alcance autorizado y se reabre el frente técnico con FFL-A como prioridad inmediata.
- **Fundamento:** cierre funcional del antiguo programa de gobierno determinista, publicación y cierre de la Dinámica del Suceso y microauditoría Potencial/Tesauro resuelta por Ruta A.
- **Evidencia:** acta pública de reapertura incorporada a `main` y comprobación del repositorio fresco.
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
- **Fundamento:** IR canónica v0.2, Vía B, acta de reapertura y procedimiento de auditoría técnica.
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
- **Fundamento:** disciplina de cambios mínimos y procedimiento de auditoría técnica.
- **Decisión:** restaurar íntegramente el estado anterior y mantener E406 pendiente hasta una nueva revisión con alcance estrictamente acotado.
- **Estado:** cerrado.

### RETP-2026-055 — P0-A / estado evaluable

- **Hecho:** `evaluate` vuelve a aceptar `CellState` y `CoupledState`, mientras `Frame.cell_states` conserva exclusivamente `CoupledState`.
- **Fundamento:** gramática v0.1, adenda técnica a la IR v0.2 y matemática de composición intercelular.
- **Evidencia:** verificación externa de la rama correspondiente con conformidad **42/42**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3** y aceptación de los programas discriminantes.
- **Decisión:** preservar la distinción entre estado simple y acoplado y separar el juicio de `supervise` para P0-B.
- **Estado:** cerrado.

### RETP-2026-056 — P0-B / J3.3 / E212-E211

- **Hecho:** `supervise` exige que `meta_eval` sea un `EvalResult`; `E212` protege el tipo y `E211` protege la procedencia desde una célula con rol `Supervisor`, también por la ruta acoplada.
- **Fundamento:** J3.3, P0-A y Vía B.
- **Evidencia:** verificación externa con conformidad **44/44**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3** y emisión exacta de E212/E211 en los dos casos negativos específicos.
- **Decisión:** declarar estabilizado P0 en sus dos partes.
- **Estado:** cerrado.

### RETP-2026-057 — J4.3 / E406 mínimo

- **Hecho:** `E406 — InsufficientTransitionData` rechaza `TransitionData` con `induced_parameters` vacío.
- **Fundamento:** J4.3 y tabla canónica de errores de la IR v0.2, que asignan E406 a esa condición exacta.
- **Evidencia:** verificación externa con conformidad **45/45**, pruebas rápidas de línea de órdenes **3/3**, SEC-0 **3/3**, emisión exacta de E406, conservación del caso válido y precedencia de E307.
- **Límite:** no se acredita por este cierre la suficiencia de una lista no vacía para reconstruir el operador inducido.
- **Decisión:** cerrar exclusivamente la cláusula de no-vaciedad y no abrir automáticamente otro juicio técnico.
- **Estado:** cerrado.

## 4. Regla de continuidad

El registro vivo deberá actualizarse junto con el CSV maestro cuando un nuevo hito cambie materialmente el estado gobernable del proyecto.

No se utilizará el registro para narrar pasos mecánicos de trabajo ni deliberaciones internas. Su función es conservar hechos, fundamento, evidencia, decisión y estado de forma pública, sobria y verificable.
