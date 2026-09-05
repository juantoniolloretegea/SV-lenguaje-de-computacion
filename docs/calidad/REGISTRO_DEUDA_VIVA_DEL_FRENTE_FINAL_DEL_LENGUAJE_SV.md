# Registro de deuda viva de la fase final del Lenguaje SV

## 1. Naturaleza

Este registro concentra la deuda técnica que sigue siendo relevante para la fase final del Lenguaje SV. No sustituye al registro de evolución ni a la documentación matemática y normativa del Sistema SV.

El cierre de un bloque no exige eliminar toda deuda. Exige que la deuda restante esté identificada, delimitada y no se presente como capacidad ya disponible. Las deudas cerradas que hayan afectado a la continuidad de fases se conservan cuando su trazabilidad sea necesaria para interpretar el estado vigente.

## 2. Deuda viva

### DFL-001 — Concordancia entre IR, catálogo e implementación

- **Descripción:** persiste una diferencia conocida entre la tabla de diagnósticos definida por la IR v0.2 y el catálogo efectivo de la implementación. La relación entre ambos se mantiene mediante la matriz por identificador y la tabla de correspondencias funcionales.
- **Reducciones acreditadas en FFL-B:** `E112` protege la restricción posicional de `CoupledState`; `E113` la compatibilidad representable entre aristas y conectores; `E114` la unicidad de `(target, position)` en régimen `Simple`; `E212` y `E211` las condiciones representadas sobre `supervise.meta_eval`; `E006` comprueba el tipo del contenido de `CellTarget`, `ComposedTarget` y `SystemTarget`; `E307` la pertenencia de tipos de suceso al `Horizon`; `E406` la no vaciedad de `TransitionData.induced_parameters`; `E011` la pertenencia de las salidas de `AdmissibilityTable` a `output_codomain`; `E213/E214` la legalidad estructural de las proyecciones; `E206/E207` efectivos la presencia de `context` y `mechanism` en `resolve`; y `E215` la concordancia, en número y codominio por posición, entre las entradas de `gate` y `AdmissibilityTable.input_codomains`.
- **Reducción acreditada en N0-01:** `E004 — InvalidCodomain` queda fijado como identidad observable única para un `Codomain` vacío o con miembros repetidos, con rechazo equivalente en Python y Rust. El `E101 — EmptyCodomain` de la IR v0.2 se conserva sólo como antecedente histórico porque el catálogo efectivo asigna `E101` a `VectorLengthMismatch`.
- **Corrección estructural acreditada:** `conflicts` fue retirado de `graph_decl` porque no pertenece a `CompositionGraph` y la superficie vigente no dispone todavía de una declaración completa de `ConflictOperator`.
- **Límites vigentes:** la concurrencia en régimen `General` que requiera `ConflictOperator` permanece sin comprobación material; la procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto no está representada; una lista no vacía de `induced_parameters` no demuestra por sí sola la reconstrucción del operador inducido; E011 y E215 no ejecutan `GateResult.output`; E213/E214 no ejecutan resultados; E206/E207 no cierran J1.6; el tipado del contenido de `Supervisable` no acredita el determinismo de `verdict`, el efecto de `Veto` ni la ejecución completa de la supervisión. `E107 — InvalidTernarizerPartition` sigue siendo parcial: Python y Rust admiten tres nombres de partición iguales porque sólo comprueban cadenas no vacías; no están materializadas cobertura, disjunción ni ausencia de solapamiento. `CoupledSpec.bridges` admite posiciones repetidas pese a representar un `BridgeSet`. `Horizon.events` admite repeticiones, cuyo estatuto permanece normativamente indeterminado por la tensión entre la forma de lista y la notación de conjunto.
- **Precisión diagnóstica pendiente:** `E006 — UndeclaredReference` se utiliza tanto para una referencia inexistente como para una referencia existente de tipo incompatible. FFL-C caracteriza de forma persistente ambos supuestos mediante cuatro comprobaciones, pero no modifica el nombre, el mensaje ni el contrato diagnóstico.
- **Integridad diagnóstica Rust pendiente:** los fallos de bienformación se encapsulan hoy en `CompileError::InvalidProgram(String)` y la identidad `E004` se acredita buscando el literal dentro del mensaje. Debe decidirse una representación estructurada del código diagnóstico antes de que la proliferación de literales convierta su identidad en una convención frágil. Esta deuda no invalida la equivalencia observable de N0-01 ni autoriza a cambiar códigos dentro de ese acto.
- **Estado:** gobernada; no bloquea por sí sola los cierres alcanzados.
- **Prioridad:** alta para una futura revisión del contrato diagnóstico.
- **Evidencia:** `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`, `MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`, `CROSSWALK_FUNCIONAL_DE_OBLIGACIONES_DIAGNOSTICAS_IR_V0_2_Y_FRONTEND_VIA_B_2026_08_18.md`, `tests/COBERTURA_OBSERVABLE_FFL_C_2026_08_20.md` y `docs/arquitectura/ACTA_TECNICA_N0_01_UNICIDAD_DE_CODOMAIN_2026_09_04.md`.

### DFL-002 — Sincronización entre documentación pública y evidencia

- **Descripción:** la documentación pública requiere vigilancia continua para no atribuir cierres, capacidades o coberturas que la evidencia no sostenga.
- **Estado:** abierta como obligación de mantenimiento documental.
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

### DFL-006 — Resultados y datos sin productor o ejecución material

- **Descripción:** `Frame.criticalities` admite referencias a `CriticalityResult`, pero la superficie vigente no dispone de un operador que produzca ese resultado. Del mismo modo, `GateResult.output` y determinadas consecuencias de `SupervisionResult` pertenecen a una fase de ejecución todavía no materializada.
- **Estado:** reconocida; fuera del alcance de los cierres que no materializan esas capacidades.
- **Prioridad:** posterior y dependiente de decisión arquitectónica expresa.

### DFL-007 — Huecos heredados de cierre gramatical de la realización Rust — CERRADA

- **Descripción histórica:** durante la ampliación de la verificación asociada a B2 se identificaron tres producciones cerradas que la realización Rust aceptaba indebidamente: `SemanticRelation.kind` fuera de `DeclaredRelation`, `Pattern.kind` fuera de `DeclaredPattern` y `Graph.regime` fuera de `Simple | General`.
- **Efecto material histórico:** un valor ajeno en `Graph.regime` permitía evitar la restricción de concurrencia aplicable al régimen `Simple`. La deuda era anterior a B2 y no constituía una regresión introducida por los perfiles fuente.
- **Corrección:** la realización Rust impone los tres dominios sobre la identidad canónica común y la bienformación conserva una comprobación defensiva equivalente. La misma frontera protege `SVP-ES` y `SVP-EN`.
- **Regresión permanente:** quedan incorporadas pruebas Rust para los dominios cerrados y seis sondas directas de navegador, DG-01/02/03 en ambos perfiles fuente.
- **Evidencia ejecutable:** conformidad R0-7 **79/79** — 12 válidos y 67 inválidos; `sv_core` **210/210**; pruebas específicas de dominios cerrados **5/5**; sondas de navegador **6/6**; `sv_wasm` **2/2**; documentación ejecutable `sv_core` **17/17**.
- **DD-01:** la Gramática 0.2 fija expresamente la forma vigente de los cierres internos de `connector.mapping` y `admissibility_table.table`, sin reescribir la v0.1 histórica ni ampliar el lenguaje.
- **VH-01:** el antiguo `tests/adversarial/deep_nested_query_valid.svp` queda reclasificado como `tests/adversarial/historico/deep_nested_query_legacy_gramatica_0_1.svp`, con estatuto histórico explícito.
- **Continuidad:** R0 queda nuevamente cerrado en el perímetro correctivo afectado; R1 queda revalidado sobre la base corregida; la suspensión específica de R2 causada por DFL-007 queda levantada y R2 recupera su estado abierto previo.
- **Estado:** **cerrada el 29/08/2026**.
- **Evidencia de cierre:** `ACTA_TECNICA_DE_CONFORMIDAD_CIERRE_CORRECTIVO_B2_Y_RESTAURACION_CONTINUIDAD_2026_08_29.md`, PR #55 y ejecuciones de conformidad asociadas.
- **Límite:** el cierre de DFL-007 no materializa `ConflictOperator` ni completa J2.3 para concurrencia en régimen `General`; esa deuda permanece dentro de DFL-001 y de la documentación normativa correspondiente.

## 3. Estado de FFL-B

FFL-B se cerró tras E215 porque las obligaciones restantes identificadas no podían materializarse de forma honesta mediante una comprobación estructural adicional sin ampliar representación, semántica o ejecución.

Una publicación futura, una ampliación matemática o una nueva necesidad técnica podrá justificar una reapertura delimitada. La mera existencia de deuda no constituye por sí sola causa de reapertura.

## 4. Estado de FFL-C

FFL-C se cerró el 20/08/2026 con la evidencia correspondiente a su corte histórico. Las ampliaciones posteriores de la batería de conformidad no reescriben ese cierre, aunque el corpus vigente sea mayor.

FFL-C no modifica el contrato diagnóstico ni acredita capacidades de ejecución material ausentes. FFL-D permanece pendiente.

## 5. Regla de mantenimiento

Toda deuda que afecte a un cierre ya declarado deberá incorporarse a este registro o a su documento sucesor. Sólo podrá retirarse del conjunto de deuda viva mediante cierre acreditado o traslado expresamente justificado a otro bloque; cuando su efecto sobre la continuidad sea material, se conservará la trazabilidad del cierre.

Las actualizaciones deberán expresar hechos, fundamento, evidencia, alcance y estado, sin presentar hipótesis o previsiones como capacidades ya existentes.
