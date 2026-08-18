# Registro de Evolución Técnica del Proyecto

## Finalidad

Este registro coordina la lectura humana del historial técnico del Lenguaje SV con el CSV maestro `REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`.

El CSV maestro conserva la autoridad registral y la numeración RETP. Este archivo ofrece la lectura humana del tramo vivo y debe mantenerse sincronizado con aquél.

## Preservación del histórico detallado

Para evitar que el registro humano vivo vuelva a crecer hasta dificultar la reentrada rápida, el detalle completo de `RETP-2026-000` a `RETP-2026-047` queda preservado sin modificación en:

`docs/calidad/historico/REGISTRO_EVOLUCION_TECNICA_PROYECTO_HASTA_RETP_2026_047.md`

La continuidad humana completa se lee, por tanto, como:

`histórico detallado 000–047 → este registro vivo desde 048 → CSV maestro para la serie completa y su autoridad de numeración`.

Esta compactación no reescribe ni elimina el histórico anterior. El archivo histórico conserva literalmente el contenido que tenía este registro al cierre de `RETP-2026-047`.

## Tabla del tramo vivo

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

## Entradas detalladas del tramo vivo

### RETP-2026-048 — Lenguaje SV / Ruta A / retorno a FFL-A

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 11:34:13  
- **Tipo de hito:** REAPERTURA_GOBERNADA  
- **Resumen del cambio:** Se levanta la pausa preventiva del 16/08 en el alcance expresamente autorizado y se reabre el frente técnico por Ruta A, con FFL-A como prioridad inmediata y sin expansión semántica automática.  
- **Motivo / argumento:** Quedaron satisfechos el cierre funcional del antiguo programa de gobierno determinista, la reconciliación de la Dinámica general del Suceso y la microauditoría Potencial/Tesauro, que concluyó que ninguna de esas dos piezas bloquea el frente técnico inmediato.  
- **Base doctrinal / técnica:** `ACTA_TECNICA_DE_ORDENACION_DE_CONTINUIDAD_SEMANTICA_Y_ARQUITECTONICA_DEL_LENGUAJE_SV_2026_08_16.md`; Dinámica del Suceso, DOI `10.21428/39829d0b.8ea18396`; decisión humana soberana de Ruta A.  
- **Artefactos afectados:** `docs/calidad/ACTA_TECNICA_DE_REAPERTURA_DEL_LENGUAJE_SV_POR_RUTA_A_Y_RETORNO_A_FFL_A_2026_08_18.md`; `docs/calidad/README.md`.  
- **Evidencia:** acta pública de reapertura materializada en `main`; comprobación del repositorio fresco y de la pieza matemática canónica.  
- **Impacto:** gobierno técnico; trazabilidad; continuidad operativa; control de alcance.  
- **Objeción adversarial:** riesgo de interpretar Ruta A como autorización indiscriminada para gramática, IR, backend, IA, seguridad posterior o `NL→SVP`.  
- **Decisión:** reabrir únicamente desde FFL-A y mantener cerradas las compuertas independientes no autorizadas.  
- **Estado:** cerrado.

### RETP-2026-049 — Lenguaje SV / FFL-A → FFL-B

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 11:58:10  
- **Tipo de hito:** CIERRE_BLOQUE_Y_APERTURA_SECUENCIAL  
- **Resumen del cambio:** Se cierra FFL-A bajo Vía B con deuda residual explícita y gobernada y se activa secuencialmente FFL-B como único bloque técnico inmediato.  
- **Motivo / argumento:** El crosswalk funcional acreditó concordancia suficiente para cerrar el contrato diagnóstico sin exigir convergencia nominal total entre IR canónica y catálogo implementativo; la deuda restante quedó localizada y sometida a tratamiento posterior.  
- **Base doctrinal / técnica:** IR canónica v0.2; decisión C1C de regularización por Vía B; matriz diagnóstica; crosswalk funcional; criterios de cierre del frente final.  
- **Artefactos afectados:** `docs/calidad/TABLERO_DE_BLOQUES_CERRABLES_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.csv`; `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`; documentación de crosswalk y cierre FFL-A.  
- **Evidencia:** commits `5a709a1a6530ae9a071f3cfdffeaf4ac57430f99`, `fb661ea8d178473f115ee03097da53bca9ec17f7` y `53982de75d5e6ab147151dbb42cae61f9c62064e`; tablero fresco con FFL-A cerrado y FFL-B abierto.  
- **Impacto:** coherencia diagnóstica; gobierno técnico; trazabilidad; secuenciación.  
- **Objeción adversarial:** riesgo de leer cierre bajo Vía B como desaparición de toda deuda o como autorización para abrir simultáneamente FFL-C/D/E.  
- **Decisión:** dar FFL-A por cerrado con deuda gobernada y mantener FFL-B como siguiente bloque único por dependencia material.  
- **Estado:** cerrado.

### RETP-2026-050 — Lenguaje SV / FFL-B / materialización diagnóstica subordinada

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 12:28:08  
- **Tipo de hito:** DECISION_GOBIERNO_TECNICO  
- **Resumen del cambio:** Se fija para FFL-B que sólo se materializarán obligaciones ya representables por datos existentes, con fundamento IR/doctrinal expreso, ruta diagnóstica inequívoca y prueba trazable; los nuevos códigos efectivos deberán sincronizar error, emisión, prueba, catálogo, matriz, crosswalk y deuda.  
- **Motivo / argumento:** La reapertura técnica heredó divergencias reales entre la IR y la implementación, pero no autoriza inventar semántica ni diagnósticos por comodidad. Era necesario fijar una regla de microcierre antes de continuar la cadena de implementación.  
- **Base doctrinal / técnica:** IR canónica v0.2; acta de reapertura Ruta A; Vía B diagnóstica; procedimiento de auditoría técnica; estado fresco de parser/lowering/validator/serializer y suite.  
- **Artefactos afectados:** `docs/calidad` y `docs/referencia` afectados por el gobierno de nuevos diagnósticos efectivos; `src/` y `tests/` como superficie subordinada de cada microcierre.  
- **Evidencia:** commit `0c14b320285b6bf55d36e5c3be1b332b3166cb1a` y auditoría del repositorio fresco previa a los microcierres posteriores.  
- **Impacto:** gobierno técnico; coherencia diagnóstica; control de alcance; mantenibilidad.  
- **Objeción adversarial:** riesgo doble de dejar obligaciones reales sin enforcement o de crear códigos/semántica para rellenar casillas históricas.  
- **Decisión:** trabajar por microbloques verificables y no materializar ninguna obligación sin ruta técnica y diagnóstica suficientemente acreditada.  
- **Estado:** cerrado.

### RETP-2026-051 — Lenguaje SV / FFL-B / J2.2 parcial / E112

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 13:05:54  
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO  
- **Resumen del cambio:** Se materializa la restricción posicional de `CoupledState` para impedir modificaciones fuera del `BridgeSet` del `CoupledSpec` de destino y se sincroniza E112 con catálogo, matriz, crosswalk y deuda viva.  
- **Motivo / argumento:** J2.2 ya disponía de datos suficientes para comprobar el componente posicional sin introducir nueva semántica; la procedencia por conector no estaba todavía suficientemente representada y queda fuera de este microcierre.  
- **Base doctrinal / técnica:** IR canónica v0.2, J2.2; AST y validator vigentes; decisión de FFL-B sobre materialización mínima bajo Vía B.  
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; `tests/conformance/invalid/coupledstate_update_fuera_bridges.svp`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; matriz/crosswalk/deuda de `docs/calidad`.  
- **Evidencia:** commits `33f60e0c174b702c4c9a5f3ead17b5b810c7f486` y `64d4fc5d21c8f69cc2f88df5f872741bd0f7239c`; lectura posterior de emisión y sincronización documental.  
- **Impacto:** coherencia IR-implementación; trazabilidad; control de alcance; mantenibilidad.  
- **Objeción adversarial:** riesgo de presentar J2.2 como cerrado en su totalidad cuando sólo se acredita el componente posicional.  
- **Decisión:** cerrar únicamente la parte posicional mediante E112 y conservar fuera del alcance la procedencia por conector hasta disponer de representación inequívoca.  
- **Estado:** cerrado.

### RETP-2026-052 — Lenguaje SV / FFL-B / J2.3 / E113 ↔ E206 canónico

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 13:15:04  
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO  
- **Resumen del cambio:** Se materializan las compatibilidades de `Edge` ya representadas: posición dentro del `BridgeSet` del destino, igualdad entre `Edge.position` y `Connector.target_position` y compatibilidad del codominio fuente del conector con la célula transmisora; se sincroniza E113 como ruta efectiva de la obligación canónica E206.  
- **Motivo / argumento:** La IR ya contenía los objetos y referencias necesarios para comprobar J2.3 sin ampliar gramática ni IR; el identificador E206 implementativo ya estaba ocupado con otra semántica bajo Vía B.  
- **Base doctrinal / técnica:** IR canónica v0.2, J2.3 y E206 canónico; AST/validator vigentes; contrato diagnóstico Vía B.  
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; casos inválidos de compatibilidad Edge/Connector; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; matriz/crosswalk/deuda de `docs/calidad`.  
- **Evidencia:** commits `b59bbb345b0e9d0ead8457fd80f8a7f8299ff129` y `a2dd1ef59272ee9149283ccdc3c4d5cde7201f4b`; contraste material de las tres comprobaciones.  
- **Impacto:** coherencia IR-implementación; trazabilidad; contrato diagnóstico; mantenibilidad.  
- **Objeción adversarial:** riesgo de renumerar E206 implementativo o de declarar equivalencia semántica por coincidencia de identificador.  
- **Decisión:** mantener Vía B y registrar E113 como ruta efectiva explícita de la obligación E206 canónico sin renumeración masiva.  
- **Estado:** cerrado.

### RETP-2026-053 — Lenguaje SV / FFL-B / J4.3 / E307 ↔ E403 canónico

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 13:25:03  
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO  
- **Resumen del cambio:** Se impone que cada tipo de suceso declarado en `TransitionData.events` pertenezca al `Horizon` referenciado y se sincroniza E307 como ruta efectiva de la obligación canónica E403.  
- **Motivo / argumento:** `TransitionData.events` y `Horizon.events` ya existían materialmente en la gramática/AST; la comprobación era un descenso directo de una obligación ya representable y no exigía nueva semántica.  
- **Base doctrinal / técnica:** IR canónica v0.2, J4.3 y E403 canónico; Dinámica del Suceso como marco superior ya reconciliado; AST/validator vigentes.  
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; `tests/conformance/invalid/transition_event_fuera_horizon.svp`; `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`; matriz/crosswalk/deuda de `docs/calidad`.  
- **Evidencia:** commits `91af0f0ac9d8c0b5b8d8c24e738aa3fe78477199` y `d2c51e48bdc61e771a8c7744ea3bc571ee40a430`; comprobación posterior de pertenencia y sincronización documental.  
- **Impacto:** coherencia IR-implementación; trazabilidad; preservación semántica; mantenibilidad.  
- **Objeción adversarial:** riesgo de identificar `TransitionData` con suceso admisible o de convertir esta pertenencia de tipos en una ampliación de la semántica eventiva.  
- **Decisión:** limitar el microcierre a la pertenencia de tipos ya declarados al `Horizon` y no introducir nuevos nodos ni composición de sucesos.  
- **Estado:** cerrado.

### RETP-2026-054 — Lenguaje SV / FFL-B / intento E406 revertido

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 13:30:27  
- **Tipo de hito:** SORPRESA_TECNICA_Y_REVERSION  
- **Resumen del cambio:** Se revierte íntegramente el intento de materializar E406 para `TransitionData` sin cambios inducidos al detectarse un diff no mínimo que remaquetaba código ajeno al alcance del microparche; el árbol final vuelve al estado anterior al intento y E406 permanece pendiente.  
- **Motivo / argumento:** Aunque la obligación canónica de `TransitionData` no vacío es candidata legítima, la forma concreta del parche violaba la disciplina de radio corto y podía ocultar cambios accidentales bajo una modificación funcional pequeña.  
- **Base doctrinal / técnica:** IR canónica v0.2; disciplina de parches mínimos; procedimiento de auditoría técnica; estado FFL-B previo al intento E406.  
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py` y cualquier documentación tocada por el intento, todos restaurados al estado previo mediante reversión.  
- **Evidencia:** commits `bc58066a89748d9712cfdb83214082b8d4755715` y `e69ef46e53d93fb4a6e690602666b832afea674d`; comparación posterior del árbol sin diferencias respecto del estado preintento.  
- **Impacto:** control de calidad; trazabilidad; disciplina de parche; mantenibilidad.  
- **Objeción adversarial:** riesgo de aceptar un parche funcionalmente plausible pese a un diff amplio no justificado, o de confundir la reversión con rechazo de la obligación E406.  
- **Decisión:** revertir el intento completo; mantener E406 como siguiente candidato únicamente mediante diff estrictamente mínimo y nueva verificación.  
- **Estado:** cerrado.

### RETP-2026-055 — Lenguaje SV / FFL-B / P0-A / contrato de estado evaluable

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 21:35:25  
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO  
- **Resumen del cambio:** Se reconcilia el contrato `CellState` / `CoupledState` / `evaluate` / `Frame`: `evaluate` acepta ambos estados evaluables, mientras `Frame.cell_states` conserva `CoupledState`; P0-A queda cerrado tras evidencia dinámica independiente.  
- **Motivo / argumento:** Una regresión local había estrechado `_validate_eval` a `CellStateDecl`, contradiciendo la gramática vigente y la evaluación de configuración acoplada ya constituida, además de romper una sonda SEC-0 y ejemplos documentados.  
- **Base doctrinal / técnica:** Gramática superficial mínima v0.1; IR v0.2 y adenda técnica de estado evaluable acoplado; Documento I de composición intercelular; disciplina de radio corto.  
- **Artefactos afectados:** `ADENDA_TECNICA_IR_v0_2_ESTADO_EVALUABLE_ACOPLADO_2026_08_18.md`; `src/svp_validator.py`; sondas y ejemplos afectados; `docs/calidad/ACTA_TECNICA_DE_RECONCILIACION_DEL_CONTRATO_CELLSTATE_COUPLEDSTATE_EVALUATE_Y_FRAME_2026_08_18.md`.  
- **Evidencia:** rama `agent/ffl-b-evaluable-state-reconcile` verificada en `b9db1a268e7acf8283f99eb6d7d09da243a9293c`; conformidad **42/42**, CLI **3/3**, SEC-0 **3/3**; aceptación de las tres sondas discriminantes; cierre documental en `0fde188d77a3ebd2fb08f2ba33f51dd173a834c4`.  
- **Impacto:** coherencia IR-implementación; trazabilidad; control de regresión; secuenciación.  
- **Objeción adversarial:** riesgo de reparar la regresión relajando `Frame` para admitir `CellState`, o de mezclar en el mismo juicio el hueco independiente de `supervise`.  
- **Decisión:** mantener `Frame.cell_states : CoupledState`, restaurar `evaluate(CellState | CoupledState)` y separar P0-B como juicio posterior.  
- **Estado:** cerrado.

### RETP-2026-056 — Lenguaje SV / FFL-B / P0-B / J3.3 / E212-E211

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 21:35:25  
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO  
- **Resumen del cambio:** Se impone `meta_eval : EvalResult` en `supervise` mediante `E212 — SuperviseMetaNotEvalResult` y se extiende `E211 — SuperviseMetaNotSupervisor` a la procedencia por `CoupledState → CoupledSpec → CellSpec`; P0-B queda cerrado tras adversarial dinámica independiente.  
- **Motivo / argumento:** J3.3 exige que el primer argumento de `supervise` sea un `EvalResult` procedente de una célula de segundo orden; el validator sólo comprobaba existencia y rol cuando el objeto ya era un `EvalCmd`, dejando atravesar referencias existentes de otro tipo y omitiendo la ruta acoplada autorizada por P0-A.  
- **Base doctrinal / técnica:** IR canónica v0.2, J3.3; P0-A; regularización diagnóstica por Vía B; contrato efectivo de `supervise`.  
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; dos adversariales nuevos; catálogo, matriz, crosswalk, deuda viva y `docs/calidad/ACTA_TECNICA_P0_B_CONTRATO_META_EVAL_DE_SUPERVISE_2026_08_18.md`.  
- **Evidencia:** rama `agent/ffl-b-p0b-supervise-meta` verificada en `8080e22ddd103b6a33ae157ce86bdf1de540025d`; conformidad **44/44**, CLI **3/3**, SEC-0 **3/3**; emisión exacta `E212` para tipo incorrecto y `E211` para rol incorrecto por ruta acoplada; conservación de `supervise_targets`, `evaluate(CoupledState)` y `Frame.cell_states : CoupledState`.  
- **Impacto:** coherencia IR-implementación; contrato diagnóstico; trazabilidad; control de regresión; mantenibilidad.  
- **Objeción adversarial:** riesgo de colapsar en un único diagnóstico el fallo de tipo y el fallo de rol, o de reutilizar indebidamente `E306`, que protege el etiquetado del `target : Supervisable`.  
- **Decisión:** mantener `E212` para tipo de `meta_eval` y `E211` para procedencia Supervisor; declarar P0 estabilizado en sus dos partes y conservar `E406` sin abrir hasta nueva microauditoría mínima.  
- **Estado:** cerrado.

### RETP-2026-057 — Lenguaje SV / FFL-B / J4.3 / E406 mínimo

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 22:11:00  
- **Tipo de hito:** CAMBIO_FUNCIONAL_GOBERNADO  
- **Resumen del cambio:** Se materializa y cierra `E406 — InsufficientTransitionData` para rechazar `TransitionData` con `induced_parameters` vacío, con convergencia exacta por identificador y sin atribuir cierre completo de J4.3.  
- **Motivo / argumento:** La IR v0.2 asigna E406 exactamente a `induced_parameters` vacío; gramática, AST, parser y lowering ya representaban la lista y sólo faltaba la comprobación de no-vaciedad en el validator.  
- **Base doctrinal / técnica:** IR canónica v0.2, J4.3 y E406; acta de reapertura por Ruta A; decisión FFL-B de materialización diagnóstica subordinada; disciplina de radio corto.  
- **Artefactos afectados:** `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; `tests/conformance/invalid/transition_induced_parameters_vacios.svp`; catálogo, matriz, crosswalk, deuda viva y `docs/calidad/ACTA_TECNICA_FFL_B_E406_INDUCED_PARAMETERS_MINIMO_2026_08_18.md`.  
- **Evidencia:** rama `agent/ffl-b-e406-min-transition` verificada en `a1d3d98b1971fe3cb7c1941dc1b3d9ce41b1e54c`; conformidad **45/45**, CLI **3/3**, SEC-0 **3/3**; emisión exacta `E406` para lista vacía; `transition_data_events.svp` continúa válido; `E307` se conserva y precede a E406; diff de un commit y radio corto.  
- **Impacto:** coherencia IR-implementación; convergencia diagnóstica; trazabilidad; control de regresión; disciplina de parche.  
- **Objeción adversarial:** riesgo de convertir la no-vaciedad en prueba de reconstructibilidad del operador inducido o declarar cerrado todo J4.3.  
- **Decisión:** cerrar sólo la cláusula de no-vaciedad mediante E406; mantener fuera de cierre la suficiencia reconstructiva y cualquier validación adicional de los triples `induced_parameters`; no abrir automáticamente otro microcierre.  
- **Estado:** cerrado.
