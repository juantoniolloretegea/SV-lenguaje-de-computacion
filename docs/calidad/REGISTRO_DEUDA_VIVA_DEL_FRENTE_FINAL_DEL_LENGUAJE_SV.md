# Registro de deuda viva del frente final del lenguaje SV

## 1. Naturaleza

Este registro concentra únicamente la deuda viva relevante para el frente final del lenguaje SV. No sustituye al registro técnico general.

## 2. Deuda viva inicial

### DFL-001 — Concordancia IR ↔ catálogo ↔ implementación

- **Descripción:** persiste una tensión reconocida entre la norma diagnóstica superior (IR v0.2) y el catálogo implementativo efectivo. La divergencia queda localizada en dos planos complementarios: matriz por identificador y crosswalk funcional de obligaciones canónicas. El estado vigente distingue 4 coincidencias semánticas por mismo ID, 20 divergencias por mismo ID, 14 códigos solo IR y 16 códigos solo implementación. En FFL-B, la cláusula posicional de la obligación canónica `E202 — IllegalBridgeUpdate` queda materializada parcialmente mediante `E112`, mientras la exigencia de procedencia desde un `Connector` bien formado permanece abierta. La obligación canónica `E206 — EdgeConnectorMismatch` queda protegida funcionalmente mediante `E113` para la compatibilidad contextual de posición, `target_position` y codominio fuente, complementada por las comprobaciones del mapping ya existentes. La obligación canónica `E403 — UndeclaredHorizonEvent` queda protegida funcionalmente mediante `E307`, que compara los tipos de suceso de `TransitionData` con el `Horizon` referenciado.
- **Riesgo:** confundir convergencia numérica con protección funcional, o presentar como cerrada una obligación canónica que solo está parcial o prospectivamente materializada.
- **Estado:** gobernada y no bloqueante para el cierre de FFL-A bajo Vía B; permanece viva para convergencia futura y para los bloques de implementación/ABI que correspondan.
- **Prioridad:** alta.
- **Evidencia de gobierno:** `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`, `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv` y `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`.

### DFL-002 — Sincronización documentación pública ↔ evidencia

- **Descripción:** la documentación pública requiere vigilancia continua para no sobreatribuir cierre o cobertura no acreditados por la suite.
- **Riesgo:** distorsión del estado verificable del frente.
- **Estado:** abierta.
- **Prioridad:** media-alta.

### DFL-003 — ABI semántico-diagnóstico materializado, pero todavía no agotado

- **Descripción:** el contrato mínimo de enganche y ABI semántico-diagnóstico ya queda materializado en una pieza autónoma de arquitectura y en una primera capa de validación fuerte sobre `Domain`, `Agent`, `QuerySpec` y `query`.
- **Riesgo:** confundir esta materialización mínima con cierre total de N4/Uso o con interpretación ejecutiva plena de todos los campos opacos de `Domain`.
- **Estado:** acotada y gobernada.
- **Prioridad:** media.

## 3. Regla

Toda nueva deuda viva relevante deberá incorporarse aquí si afecta al cierre del frente, y deberá desaparecer de este registro solo por cierre acreditado o traslado formalmente justificado.

### DFL-004 — Distinción entre frame histórico, reapertura y consulta presente

- **Descripción:** persiste como deuda viva del frente la necesidad de demostrar concordancia fuerte entre el último frame históricamente acreditado, la reapertura legítima a `U`, la cobertura/admisibilidad vigente y la forma en que la consulta expresa el estado actual sin sobreatribuir cierre.
- **Riesgo:** confusión entre historial acreditado y estado presente; persistencia ilegítima de cierres fuertes; degradación insuficientemente gobernada a `U`; ambigüedad pública de la consulta.
- **Estado:** abierta y gobernada.
- **Prioridad:** alta.

### DFL-005 — Campos opacos de Domain todavía no interpretados por runtime

- **Descripción:** aunque el contrato mínimo de enganche ya dispone de validación formal inicial, el frontend no ejecuta todavía una semántica material completa de `interface`, `exogeneity_mask`, `silent_u`, `transduction_policy`, `u_policy`, `closure_criterion` ni `query_engine`.
- **Riesgo:** sobreatribuir al núcleo capacidades perceptivas o de cierre que hoy solo están gobernadas como contrato de forma.
- **Estado:** abierta y reconocida.
- **Prioridad:** media.
