# Registro de deuda viva del frente final del Lenguaje SV

## 1. Naturaleza

Este registro concentra únicamente la deuda viva relevante para el frente final del Lenguaje SV. No sustituye al registro técnico general ni a la doctrina superior.

## 2. Deuda viva

### DFL-001 — Concordancia IR ↔ catálogo ↔ implementación

- **Descripción:** persiste una tensión reconocida entre la norma diagnóstica superior de la IR v0.2 y el catálogo efectivo de la implementación. La divergencia se gobierna mediante dos instrumentos complementarios: la matriz por identificador y la tabla de correspondencias funcionales de obligaciones canónicas. El estado vivo distingue las coincidencias por identificador, las divergencias semánticas, los códigos sólo presentes en la IR y los códigos sólo presentes en la implementación. En FFL-B se han materializado cierres parciales o completos sobre obligaciones concretas: `E112` para la restricción posicional de `CoupledState`; `E113` para la compatibilidad contextual de aristas y conectores; `E212` y `E211` para las precondiciones de `supervise`; `E307` para la pertenencia de tipos de suceso al `Horizon`; `E406` para la no-vaciedad de `TransitionData.induced_parameters`; y `E011` para exigir que cada salida literal de `AdmissibilityTable.table` pertenezca al `output_codomain` declarado. La suficiencia adicional de `TransitionData` para reconstruir el operador inducido permanece abierta. El cierre de `E011` no agota J1.4: permanecen fuera de su alcance las obligaciones adicionales de determinismo, asimetría documentada y relación semántica previa en cuanto no estén ya protegidas por otras rutas acreditadas.
- **Riesgo:** confundir coincidencia numérica con protección funcional o presentar como cerrada una obligación canónica que sólo esté materializada de forma parcial.
- **Estado:** gobernada y no bloqueante para el cierre ya efectuado de FFL-A bajo Vía B; permanece viva para la convergencia futura y para los bloques técnicos que correspondan.
- **Prioridad:** alta.
- **Evidencia de gobierno:** `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`, `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv` y `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`, cuyo nombre histórico de archivo se conserva por trazabilidad y cuya función es la de tabla de correspondencias funcionales.

### DFL-002 — Sincronización entre documentación pública y evidencia

- **Descripción:** la documentación pública requiere vigilancia continua para no atribuir cierres, capacidades o coberturas que la evidencia no sostenga.
- **Riesgo:** distorsión del estado verificable del frente.
- **Estado:** abierta.
- **Prioridad:** media-alta.

### DFL-003 — ABI semántico-diagnóstico materializado, pero no agotado

- **Descripción:** el contrato mínimo de enganche y el ABI semántico-diagnóstico cuentan ya con una pieza autónoma de arquitectura y con una primera capa de validación sobre `Domain`, `Agent`, `QuerySpec` y `query`.
- **Riesgo:** confundir esta materialización mínima con el cierre total de N4/Uso o con una interpretación ejecutiva plena de todos los campos todavía opacos de `Domain`.
- **Estado:** acotada y gobernada.
- **Prioridad:** media.

### DFL-004 — Distinción entre `Frame` histórico, reapertura y consulta presente

- **Descripción:** permanece abierta la necesidad de demostrar concordancia fuerte entre el último `Frame` históricamente acreditado, la reapertura legítima a `U`, la cobertura o admisibilidad vigente y la forma en que la consulta expresa el estado actual sin atribuir una clausura superior a la acreditada.
- **Riesgo:** confusión entre historial acreditado y estado presente; conservación ilegítima de clausuras fuertes; degradación insuficientemente gobernada a `U`; ambigüedad pública de la consulta.
- **Estado:** abierta y gobernada.
- **Prioridad:** alta.

### DFL-005 — Campos de `Domain` todavía sin interpretación ejecutiva completa

- **Descripción:** aunque el contrato mínimo de enganche dispone de validación formal inicial, la etapa frontal del compilador no ejecuta todavía una semántica material completa de `interface`, `exogeneity_mask`, `silent_u`, `transduction_policy`, `u_policy`, `closure_criterion` ni `query_engine`.
- **Riesgo:** atribuir al núcleo capacidades perceptivas o de clausura que hoy sólo están gobernadas como contrato de forma.
- **Estado:** abierta y reconocida.
- **Prioridad:** media.

## 3. Regla de mantenimiento

Toda deuda viva que afecte al cierre del frente deberá incorporarse a este registro. Sólo podrá retirarse por cierre acreditado o por traslado formalmente justificado a otro bloque.

Las actualizaciones deberán formularse mediante hechos, fundamento, evidencia, alcance y estado, sin convertir hipótesis o previsiones en capacidad material existente.
