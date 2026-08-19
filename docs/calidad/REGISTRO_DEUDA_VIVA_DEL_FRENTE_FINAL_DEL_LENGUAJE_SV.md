# Registro de deuda viva del frente final del Lenguaje SV

## 1. Naturaleza

Este registro concentra la deuda técnica que sigue siendo relevante para el frente final del Lenguaje SV. No sustituye al registro de evolución ni a la documentación matemática y normativa del Sistema SV.

## 2. Deuda viva

### DFL-001 — Concordancia entre IR, catálogo e implementación

- **Descripción:** persiste una diferencia reconocida entre la tabla de diagnósticos definida por la IR v0.2 y el catálogo efectivo de la implementación. La relación entre ambos se mantiene mediante la matriz por identificador y la tabla de correspondencias funcionales.
- **Reducciones acreditadas en FFL-B:** `E112` protege la restricción posicional de `CoupledState`; `E113` la compatibilidad representable entre aristas y conectores; `E114` la unicidad de `(target, position)` en régimen `Simple`; `E212` y `E211` las condiciones ya representadas sobre `supervise.meta_eval`; `E006` comprueba el tipo del contenido de `CellTarget`, `ComposedTarget` y `SystemTarget`; `E307` la pertenencia de tipos de suceso al `Horizon`; `E406` la no vaciedad de `TransitionData.induced_parameters`; `E011` la pertenencia de las salidas de `AdmissibilityTable` a `output_codomain`; `E213/E214` la legalidad estructural de las proyecciones; y `E206/E207` efectivos la presencia de `context` y `mechanism` en `resolve`.
- **Corrección estructural reciente:** `conflicts` fue retirado de `graph_decl` porque no pertenece a `CompositionGraph` y la superficie v0.1 no dispone de una declaración completa de `ConflictOperator`.
- **Límites vigentes:** la concurrencia en régimen `General` que requiera `ConflictOperator` permanece sin comprobación material; la procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto sigue sin estar representada; una lista no vacía de `induced_parameters` no demuestra por sí sola la reconstrucción del operador inducido; el cierre E011 no agota J1.4; E213/E214 no ejecutan resultados; E206/E207 no cierran J1.6; el tipado del contenido de `Supervisable` no acredita el determinismo de `verdict`, el efecto de `Veto` ni la ejecución completa de la supervisión.
- **Precisión diagnóstica pendiente:** `E006 — UndeclaredReference` se utiliza tanto para una referencia inexistente como para una referencia existente de tipo incompatible. La función es efectiva, pero el nombre y el mensaje base del diagnóstico describen con mayor precisión el primer supuesto que el segundo. Esta diferencia debe permanecer visible hasta que se decida su tratamiento.
- **Cobertura positiva pendiente:** la forma válida `SystemTarget(CompositionGraph)` fue comprobada expresamente, pero no dispone todavía de un caso válido específico conservado en la batería principal.
- **Riesgo:** confundir coincidencia numérica con equivalencia material, presentar como cerrada una obligación sólo parcialmente protegida o atribuir ejecución a una comprobación puramente estructural.
- **Estado:** gobernada y no bloqueante para el cierre ya efectuado de FFL-A bajo Vía B; permanece viva para la convergencia futura y para los bloques que correspondan.
- **Prioridad:** alta.
- **Evidencia:** `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`, `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv` y `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`.

### DFL-002 — Sincronización entre documentación pública y evidencia

- **Descripción:** la documentación pública requiere vigilancia continua para no atribuir cierres, capacidades o coberturas que la evidencia no sostenga. El README principal fue actualizado el 19/08/2026 para corregir el desfase acumulado, pero la obligación de vigilancia permanece.
- **Riesgo:** divergencia entre el estado verificable del lenguaje y su presentación pública.
- **Estado:** abierta.
- **Prioridad:** media-alta.

### DFL-003 — Interfaz semántico-diagnóstica materializada, pero no agotada

- **Descripción:** el contrato mínimo de enlace con interfaces futuras cuenta con documentación propia y con una primera validación sobre `Domain`, `Agent`, `QuerySpec` y `query`.
- **Riesgo:** confundir esta validación mínima con el cierre total de N4/Uso o con una interpretación ejecutiva completa de campos todavía no desarrollados.
- **Estado:** acotada y gobernada.
- **Prioridad:** media.

### DFL-004 — Distinción entre `Frame` histórico, reapertura y consulta presente

- **Descripción:** permanece abierta la necesidad de demostrar concordancia fuerte entre el último `Frame` históricamente acreditado, una reapertura legítima a `U`, la cobertura o admisibilidad vigente y la forma en que una consulta expresa el estado actual sin atribuir una clausura superior a la acreditada.
- **Riesgo:** confusión entre historial acreditado y estado presente; conservación indebida de conclusiones fuertes; degradación insuficientemente justificada a `U`; ambigüedad pública de la consulta.
- **Estado:** abierta y gobernada.
- **Prioridad:** alta.

### DFL-005 — Campos de `Domain` sin interpretación ejecutiva completa

- **Descripción:** aunque el contrato mínimo de enlace dispone de validación formal inicial, la etapa frontal del compilador no ejecuta todavía una interpretación material completa de `interface`, `exogeneity_mask`, `silent_u`, `transduction_policy`, `u_policy`, `closure_criterion` ni `query_engine`.
- **Riesgo:** atribuir al núcleo capacidades de observación o clausura que actualmente sólo están descritas como contrato de forma.
- **Estado:** abierta y reconocida.
- **Prioridad:** media.

## 3. Regla de mantenimiento

Toda deuda que afecte al cierre del frente deberá incorporarse a este registro. Sólo podrá retirarse mediante cierre acreditado o traslado expresamente justificado a otro bloque.

Las actualizaciones deberán expresar hechos, fundamento, evidencia, alcance y estado, sin presentar hipótesis o previsiones como capacidades ya existentes.
