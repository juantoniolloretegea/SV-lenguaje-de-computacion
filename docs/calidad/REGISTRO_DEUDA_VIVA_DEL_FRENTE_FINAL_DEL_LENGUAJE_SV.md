# Registro de deuda viva de la fase final del Lenguaje SV

## 1. Naturaleza

Este registro concentra la deuda técnica que sigue siendo relevante para la fase final del Lenguaje SV. No sustituye al registro de evolución ni a la documentación matemática y normativa del Sistema SV.

El cierre de un bloque no exige eliminar toda deuda. Exige que la deuda restante esté identificada, delimitada y no se presente como capacidad ya disponible.

## 2. Deuda viva

### DFL-001 — Concordancia entre IR, catálogo e implementación

- **Descripción:** persiste una diferencia conocida entre la tabla de diagnósticos definida por la IR v0.2 y el catálogo efectivo de la implementación. La relación entre ambos se mantiene mediante la matriz por identificador y la tabla de correspondencias funcionales.
- **Reducciones acreditadas en FFL-B:** `E112` protege la restricción posicional de `CoupledState`; `E113` la compatibilidad representable entre aristas y conectores; `E114` la unicidad de `(target, position)` en régimen `Simple`; `E212` y `E211` las condiciones representadas sobre `supervise.meta_eval`; `E006` comprueba el tipo del contenido de `CellTarget`, `ComposedTarget` y `SystemTarget`; `E307` la pertenencia de tipos de suceso al `Horizon`; `E406` la no vaciedad de `TransitionData.induced_parameters`; `E011` la pertenencia de las salidas de `AdmissibilityTable` a `output_codomain`; `E213/E214` la legalidad estructural de las proyecciones; `E206/E207` efectivos la presencia de `context` y `mechanism` en `resolve`; y `E215` la concordancia, en número y codominio por posición, entre las entradas de `gate` y `AdmissibilityTable.input_codomains`.
- **Corrección estructural acreditada:** `conflicts` fue retirado de `graph_decl` porque no pertenece a `CompositionGraph` y la superficie v0.1 no dispone de una declaración completa de `ConflictOperator`.
- **Límites vigentes:** la concurrencia en régimen `General` que requiera `ConflictOperator` permanece sin comprobación material; la procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto no está representada; una lista no vacía de `induced_parameters` no demuestra por sí sola la reconstrucción del operador inducido; E011 y E215 no ejecutan `GateResult.output`; E213/E214 no ejecutan resultados; E206/E207 no cierran J1.6; el tipado del contenido de `Supervisable` no acredita el determinismo de `verdict`, el efecto de `Veto` ni la ejecución completa de la supervisión.
- **Precisión diagnóstica pendiente:** `E006 — UndeclaredReference` se utiliza tanto para una referencia inexistente como para una referencia existente de tipo incompatible. FFL-C caracteriza de forma persistente ambos supuestos mediante cuatro comprobaciones, con resultado **4/4**, pero no modifica el nombre, el mensaje ni el contrato diagnóstico. El nombre y el mensaje base describen con mayor precisión el primer supuesto.
- **Cobertura positiva resuelta en FFL-C:** `SystemTarget(CompositionGraph)` dispone desde el 20/08/2026 de un caso válido específico conservado en la batería principal y forma parte de la conformidad acreditada de **58/58**.
- **Estado:** gobernada; no bloquea los cierres de FFL-B ni FFL-C.
- **Prioridad:** alta para una futura revisión del contrato diagnóstico, no para reabrir FFL-B o FFL-C por sí sola.
- **Evidencia:** `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`, `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`, `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md` y `tests/COBERTURA_OBSERVABLE_FFL_C_2026_08_20.md`.

### DFL-002 — Sincronización entre documentación pública y evidencia

- **Descripción:** la documentación pública requiere vigilancia continua para no atribuir cierres, capacidades o coberturas que la evidencia no sostenga.
- **Estado:** abierta como obligación de mantenimiento documental; no bloquea FFL-B ni FFL-C.
- **Prioridad:** media-alta.

### DFL-003 — Interfaz semántico-diagnóstica materializada, pero no agotada

- **Descripción:** el contrato mínimo de enlace con interfaces futuras cuenta con documentación propia y con una primera validación sobre `Domain`, `Agent`, `QuerySpec` y `query`.
- **Riesgo:** confundir esta validación mínima con el cierre total de N4/Uso o con una interpretación ejecutiva completa de campos todavía no desarrollados.
- **Estado:** acotada y gobernada.
- **Prioridad:** media.

### DFL-004 — Distinción entre `Frame` histórico, reapertura y consulta presente

- **Descripción:** permanece abierta la necesidad de demostrar concordancia fuerte entre el último `Frame` históricamente acreditado, una reapertura legítima a `U`, la cobertura o admisibilidad vigente y la forma en que una consulta expresa el estado actual sin atribuir una clausura superior a la acreditada.
- **Estado:** abierta y gobernada.
- **Prioridad:** alta.

### DFL-005 — Campos de `Domain` sin interpretación ejecutiva completa

- **Descripción:** aunque el contrato mínimo de enlace dispone de validación formal inicial, la etapa frontal del compilador no ejecuta todavía una interpretación material completa de `interface`, `exogeneity_mask`, `silent_u`, `transduction_policy`, `u_policy`, `closure_criterion` ni `query_engine`.
- **Estado:** abierta y reconocida.
- **Prioridad:** media.

### DFL-006 — Resultados y datos sin productor o ejecución material en la superficie v0.1

- **Descripción:** `Frame.criticalities` admite referencias a `CriticalityResult`, pero la superficie v0.1 no dispone de un operador que produzca ese resultado. Del mismo modo, `GateResult.output` y determinadas consecuencias de `SupervisionResult` pertenecen a una fase de ejecución todavía no materializada.
- **Estado:** reconocida; fuera del alcance cerrado de FFL-B y FFL-C.
- **Prioridad:** posterior y dependiente de decisión arquitectónica expresa.

## 3. Estado de FFL-B

FFL-B se cerró tras E215 porque las obligaciones restantes identificadas no podían materializarse de forma honesta mediante una comprobación estructural adicional sin ampliar representación, semántica o ejecución.

Una publicación futura, una ampliación matemática o una nueva necesidad técnica podrá justificar una reapertura delimitada. La mera existencia de deuda no constituye por sí sola causa de reapertura.

## 4. Estado de FFL-C

FFL-C se cierra el 20/08/2026 con evidencia reproducible de:

- conformidad **58/58** — 10 casos válidos y 48 inválidos;
- pruebas rápidas de la interfaz de línea de órdenes **3/3**;
- SEC-0 **3/3**;
- caracterización de E006 **4/4**.

Los 48 casos inválidos cubren directamente 37 de los 47 códigos efectivos. Los diez códigos restantes quedan clasificados por alcanzabilidad, ruta diagnóstica alternativa o preservación estructural. La ausencia de un caso inválido directo no se convierte en deuda artificial cuando la superficie vigente no permite producir honestamente ese caso.

FFL-C no modifica el contrato diagnóstico ni acredita capacidades de ejecución material ausentes. FFL-D y FFL-E permanecen pendientes.

## 5. Regla de mantenimiento

Toda deuda que afecte a un cierre ya declarado deberá incorporarse a este registro o a su documento sucesor. Sólo podrá retirarse mediante cierre acreditado o traslado expresamente justificado a otro bloque.

Las actualizaciones deberán expresar hechos, fundamento, evidencia, alcance y estado, sin presentar hipótesis o previsiones como capacidades ya existentes.
