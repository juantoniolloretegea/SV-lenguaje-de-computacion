# Registro de deuda viva de la fase final del Lenguaje SV

**Continuidad vigente:** [sucesión RETP-105](#relevo-retp-105), integración de las capacidades representacionales y entrega a CYB; se conservan las exclusiones por operación y la deuda no cerrada.

## 1. Naturaleza

Este registro concentra la deuda técnica que sigue siendo relevante para la fase final del Lenguaje SV. No sustituye al registro de evolución ni a la documentación matemática y normativa del Sistema SV.

El cierre de un bloque no exige eliminar toda deuda. Exige que la deuda restante esté identificada, delimitada y no se presente como capacidad ya disponible. Las deudas cerradas que hayan afectado a la continuidad de fases se conservan cuando su trazabilidad sea necesaria para interpretar el estado vigente.

## 2. Deuda viva

### DFL-001 — Concordancia entre IR, catálogo e implementación

**Sucesión candidata RETP-095, 08/09/2026:** [el asiento del maestro](./REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-095) corrige el cierre interno del testigo E011 y exige su causa semántica de pertenencia de salida. La guarda Rust ya existía; la limitación sintáctica de RETP-078/087 se conserva como antecedente. La candidata `461acc633d364a2eca964539a3835529fbc72c58` supera los cuatro flujos, tres pruebas causales Rust y la mutación AT01. No se acredita emisión del código literal E011 ni cierre de DFL-001.

- **Descripción:** persiste una diferencia conocida entre la tabla de diagnósticos definida por la IR v0.2 y el catálogo efectivo de la implementación. La relación entre ambos se mantiene mediante la matriz por identificador y la tabla de correspondencias funcionales.
- **Reducciones acreditadas en FFL-B:** `E112` protege la restricción posicional de `CoupledState`; `E113` la compatibilidad representable entre aristas y conectores; `E114` la unicidad de `(target, position)` en régimen `Simple`; `E212` y `E211` las condiciones representadas sobre `supervise.meta_eval`; `E006` comprueba el tipo del contenido de `CellTarget`, `ComposedTarget` y `SystemTarget`; `E307` la pertenencia de tipos de suceso al `Horizon`; `E406` la no vaciedad de `TransitionData.induced_parameters`; `E011` la pertenencia de las salidas de `AdmissibilityTable` a `output_codomain`; `E213/E214` la legalidad estructural de las proyecciones; `E206/E207` efectivos la presencia de `context` y `mechanism` en `resolve`; y `E215` la concordancia, en número y codominio por posición, entre las entradas de `gate` y `AdmissibilityTable.input_codomains`.
- **Reducción acreditada en N0-01:** `E004 — InvalidCodomain` queda fijado como identidad observable única para un `Codomain` vacío o con miembros repetidos, con rechazo equivalente en Python y Rust. El `E101 — EmptyCodomain` de la IR v0.2 se conserva sólo como antecedente histórico porque el catálogo efectivo asigna `E101` a `VectorLengthMismatch`.
- **Corrección estructural acreditada:** `conflicts` fue retirado de `graph_decl` porque no pertenece a `CompositionGraph` y la superficie vigente no dispone todavía de una declaración completa de `ConflictOperator`.
- **Límites vigentes:** la concurrencia en régimen `General` que requiera `ConflictOperator` permanece sin comprobación material; la procedencia completa de una actualización de `CoupledState` desde un `Connector` concreto no está representada; una lista no vacía de `induced_parameters` no demuestra por sí sola la reconstrucción del operador inducido; E011 y E215 no ejecutan `GateResult.output`; E213/E214 no ejecutan resultados; E206/E207 no cierran J1.6; el tipado del contenido de `Supervisable` no acredita el determinismo de `verdict`, el efecto de `Veto` ni la ejecución completa de la supervisión. `E107 — InvalidTernarizerPartition` pertenece al inventario IR 0.2, sin código efectivo actual. **K1-T/RETP-088 delimita expresamente la ruta no habilitada**: los nombres conservados, iguales o distintos, no acreditan conjuntos ni partición; no existe una operación SV de ternarización ejecutable. Cobertura, disjunción, pertenencia y función total/determinista siguen pendientes antes de habilitar producción. IR 0.3 §2.4 y N0 §19 fijan la condición; la no habilitación no equivale a haber probado J1.5. `CoupledSpec.bridges` recibe el cierre de unicidad J-B0 en RETP-083 (§11), efectivo tras promover su candidata. `Horizon.events` recibe en RETP-084 el dictamen de tipos declarados y unicidad local J-H1 (§12), efectivo tras promover su candidata; no regula por analogía los pares de TransitionData.
- **Precisión diagnóstica pendiente:** `E006 — UndeclaredReference` se utiliza tanto para una referencia inexistente como para una referencia existente de tipo incompatible. FFL-C caracteriza de forma persistente ambos supuestos mediante cuatro comprobaciones, pero no modifica el nombre, el mensaje ni el contrato diagnóstico.
- **Integridad diagnóstica Rust pendiente:** los fallos de bienformación se encapsulan hoy en `CompileError::InvalidProgram(String)` y la identidad `E004` se acredita buscando el literal dentro del mensaje. Debe decidirse una representación estructurada del código diagnóstico antes de que la proliferación de literales convierta su identidad en una convención frágil. Esta deuda no invalida la equivalencia observable de N0-01 ni autoriza a cambiar códigos dentro de ese acto.
- **Concordancia vigente RETP-087:** 51 códigos catalogados no equivalen a emisión estructurada completa. La matriz/CSV de agosto conservan su corte histórico; los inventarios activos enlazan los 86 negativos con obligación y rechazo textual. El testigo con nombre E011 conserva el rechazo sintáctico previo y no acredita esa cláusula por sí solo. RETP-088 rectifica trece atribuciones sintácticas a E001: se enlazan con las producciones gramaticales aplicables; sólo `invalid_tri_literal` conserva esa obligación en el corpus. No cambian rechazos ni se acredita emisión estructurada. Esta corrección documental no cierra DFL-001.
- **Observadores documentales, 09/09/2026, corte `0f434dbb6f6ea8f5f4e270c58c4ad4c5490d8c84`:** el observador G/H de fila 7 (`tests/row7_gh/verificar.mjs`) admite campos no declarados en la raíz del informe y en sus filas; su conformidad no acredita esos campos. El observador CYB (`tests/retorno_cyb/verificar.mjs`) los rechaza mediante `ESQUEMA` y `ESQUEMA_FILA`, respectivamente. Diferencia reproducida por ambas interfaces de línea de órdenes, con controles conformes. Queda registrada dentro de DFL-001; no bloquea la fila 9 ni reabre la entrega, y no supone corrección del observador G/H ni cierre de la deuda.
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

### DFL-005 — Identidad de instancia, ligaduras y campos de `Domain` sin interpretación completa

- **Descripción:** aunque el contrato mínimo de enlace dispone de validación formal inicial, la etapa frontal del compilador no ejecuta todavía una interpretación material completa de `interface`, `exogeneity_mask`, `silent_u`, `transduction_policy`, `u_policy`, `closure_criterion` ni `query_engine`.
- **Estado:** abierta y reconocida.
- **Prioridad:** media.

**Ampliación RETP-086:** [N0 §17](../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#domain-parameters-20260907) cierra sólo la unicidad nominal local de `parameters` (J-D0). La instancia doctrinal `(C,j)` y su ligadura con nombres, captura, admisibilidad y transducción no están representadas. El vacío, la multiplicidad numérica y las cardinalidades no se resuelven por inferencia. **Recepción obligatoria en F, con prioridad bloqueante para operaciones que dependan de esas ligaduras**: fijar contrato, mínimo, multiplicidad y evidencia antes de admitirlas. La aceptación estructural de una lista no equivale a constitución completa ni acredita cobertura. No se pospone esta suficiencia únicamente a K2.

**Recepción RETP-089:** [F-SV/0.1-candidata, §§2–5](../arquitectura/CONTRATO_CANDIDATO_F_DOMINIO_REPRESENTACION_Y_SUFIENCIA_POR_OPERACION_2026_09_07.md) fija el mínimo relativo a la operación, identidad `(C,j)`, asignación explícita y régimen de alias/compartición o capturas alternativas sin elección implícita. Su representación completa sigue pendiente; no se impone una cardinalidad ni unicidad numérica universal. F-IF recibe las pruebas de suficiencia y pérdida; K2 conserva identidad/versionado transversal, sin diferir hasta allí el bloqueo de operaciones dependientes. La transducción mantiene la exclusión K1-T. **Estado: recibida contractualmente como candidata, no cerrada ni materializada.**

**Recepción RETP-090:** [F-IF/1, §5](../arquitectura/F_IF_SEIS_TESTIGOS_SINTETICOS_Y_RELEVO_G_H_2026_09_07.md) localiza pérdidas concretas de interfaz y la ausencia de cadena/certificado comprobables en las operaciones actuales de SV. G/H recibe su aplicabilidad a OP-IMM-001. La suficiencia externa de consultas documentales no resuelve instancias ni ligaduras clínicas; ninguna de las 18 consultas se ofrece por ello en el núcleo. DFL-005 permanece abierta y bloqueante para operaciones dependientes.

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

### DFL-008 — Identidad de fuente y contenido CRLF en la referencia Python

- **Hecho:** `process_file` lee en modo texto con conversión de saltos; una entrada CRLF cambia antes de calcular `source_sha256`. Si CRLF aparece dentro de una cadena, también cambia el literal. Rust conserva los bytes UTF-8 en las sondas correspondientes.
- **Evidencia:** [reparación de oráculos, §4](./ACTA_TECNICA_REPARACION_DE_ORACULOS_2026_09_06.md); `tests/run_oracle_sensitivity.py`, casos `crlf` y `string_crlf`, entradas y salidas originales en su paquete de evidencia.
- **Estado histórico en RETP-078:** abierta y detectada; se reparó el observador, no la lectura Python. **Sucesión RETP-082:** retirada de la vía afectada; véase §10. No se declara reparado el compilador histórico.
- **Condición de cierre:** preservar la entrada y el literal sin normalización silenciosa, con controles LF/CRLF, huellas de bytes y paridad pertinente. La fila 3 se reanuda por N0-02; esta deuda no puede darse por cerrada ni incluirse en una afirmación de identidad de fuente general mientras permanezca abierta.

### DFL-009 — Servicio remoto de SV con realización nativa: evaluación diferida

- **Decisión humana de 07/09/2026, RETP-082:** retomar esta opción en la fila 9, al regresar al Lenguaje después del primer universo de Ciberseguridad Inteligente (I/J). No detiene K1 ni F.
- **Objeto:** valorar la comprobación pública de fuentes SV mediante un ejecutable nativo identificado, sin instalar Rust/Cargo en el equipo del visitante y sin depender de Actions durante el uso del servicio. El Playground actual y los ejecutores de CI conservan su cometido.
- **Opciones por evaluar entonces:** sede del laboratorio u otro anfitrión, incluido Cloudflare y el aprovechamiento de Workers u otros servicios. Debe identificarse el destino material real; aceptar código escrito en Rust no basta para acreditar ejecución de un binario nativo.
- **Evidencia reutilizable:** CLI existente, pruebas nativas/WASI/navegador y [registro 018 del laboratorio](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/blob/243bb59f82b3b859e49394dfddeba82f4bf11848/laboratorio-de-infraestructura-SV/registros/018-COMPARACION_DOTNET_FFI_WASM_2026_09_06.md), con sus realizaciones FFI/WASM y límites. Su corpus histórico no se amplía retrospectivamente.
- **Salida exigida en el retorno:** necesidad concreta del dominio; entradas limitadas a la DSL; identidad del artefacto; aislamiento, recursos, fallos y coste; distinción entre medición del núcleo, red y espera; ensayo previo pertinente en laboratorio antes de promover una realización (PT01/PT04/PT08/PT09/PT11/PT13/PT14).
- **Estado:** diferida; no elige proveedor, contrata servicio ni acredita despliegue o garantías materiales.

### DFL-010 — Campos opcionales repetidos antes de la proyección

- **Origen:** revisión adversarial de la retirada Python en el corte a09b9ef. Una declaración de patrón con `arity: 2; arity: 3;` conserva sólo la última ocurrencia; el defecto se observó en ambos frontales.
- **Obligación:** la gramática 0.2 §1 conserva las producciones de v0.1 §§5.4–5.5, que impiden repetir campos opcionales; v0.2 §14 precisa su recepción. El control de claves JSON posteriores no descubre una pérdida que ya ocurrió durante el análisis de la fuente.
- **Cierre RETP-087:** la [radiografía §18](../arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#campos-opcionales-20260907) inventaría los seis campos opcionales, corrige cuatro y comprueba las dos guardas singulares ya existentes. Repetición e inversión se rechazan antes de perder información; las listas conservan sus elementos. Conformidad 100/100, dos pruebas de integración y 64 testigos en nativo/WASI/navegador exigidos sobre la candidata exacta. **Estado: cerrada en ese alcance tras verificación e integración.** La retirada Python se conserva como antecedente y no recibe esta reparación retrospectivamente.

### DFL-011 — Revisión integral del español en todos los repositorios SV

- **Mandato:** acta de uso del español de 07/09/2026 y RETP-2026-092, conservados íntegramente. Comprende todos los repositorios SV, públicos y privados.
- **Aplicación inmediata:** toda redacción nueva o revisada debe respetar el español correcto, natural, técnico y preciso.
- **Condición final:** una vez resueltas las obligaciones técnicas previas y antes del cierre del núcleo, revisar íntegramente todos los repositorios conforme al acta. La revisión integral no está realizada y no suspende la continuidad de la fila 7.
- **Restauración de 08/09/2026:** se recuperan el acta y su CSV desde `149fcbc0c7cd5a1b9b5870272e5f7f1103376185`, sin alterar sus bytes. La retirada posterior no revocó el mandato ni liberó este identificador.
- **Evidencia:** [acta autorizada](./ACTA_DE_USO_DEL_ESPANOL_EN_TODOS_LOS_REPOSITORIOS_SV_2026_09_07.md), [CSV RETP-092](./RETP_2026_092_USO_DEL_ESPANOL_EN_REPOSITORIOS_SV.csv) y registro maestro RETP-092.
- **Estado:** vigente; revisión integral pendiente antes de la consolidación final. Su alcance y autoridad permanecen intactos.

### DFL-012 — Nombre histórico `cell_ref` para una identidad de nodo

- **Hecho:** la proyección 0.1.0 serializa cada elemento de `TransitionData.induced_parameters` con la clave `cell_ref`, aunque el primer componente de la IR es `NodeId` y, en la representación vigente, debe resolver un `CoupledSpec` perteneciente al grafo del horizonte.
- **Riesgo:** un consumidor puede interpretar el valor como referencia directa a `CellSpec` y colapsar nodos arquitectónicos distintos que compartan la misma especificación celular.
- **Decisión vigente:** no se cambia silenciosamente el nombre dentro del esquema 0.1.0. La semántica tipada se documenta y se protege con un testigo en el que dos nodos distintos comparten `CellSpec` y conservan identidades independientes.
- **Objetivo de corrección:** la primera versión incompatible posterior del esquema de proyección, inicialmente 0.2.0, deberá sustituir `cell_ref` por `node_ref` o una denominación equivalente y publicar una regla de migración inequívoca. La transición no podrá emitir simultáneamente dos claves contradictorias ni aceptar una conversión implícita a `CellSpec`.
- **Estado:** abierta y gobernada; no invalida por sí sola la proyección 0.1.0, pero impide afirmar que su nomenclatura es autosuficiente.
- **Prioridad:** alta antes de estabilizar una interfaz pública externa o cerrar K2.

### DFL-013 — Independencia entre realizaciones semánticas

- **Hecho:** las vías nativa, WASI y de navegador ejecutan la misma custodia Rust de `sv_core` sobre destinos distintos. La retirada de la realización Python eliminó la comparación entre dos implementaciones semánticas independientes.
- **Riesgo:** un defecto compartido por `sv_core` puede producir resultados coincidentes en todos los destinos y no ser detectado por la paridad de transporte.
- **Controles compensatorios actuales:** resultados esperados comprometidos, comprobaciones causales, comparación exacta de bytes y pares JSON, pruebas de sensibilidad y mutaciones dirigidas. Estos controles acreditan consistencia interna y capacidad de detección en sus muestras, pero no restauran la independencia perdida.
- **Condición de cierre:** disponer de una segunda realización semántica independiente para un subconjunto algebraico expresamente delimitado, o de comprobadores derivados de la doctrina y no de la misma realización, con correspondencia verificable de entradas, salidas y exclusiones.
- **Estado:** abierta; bloquea cualquier afirmación de independencia entre realizaciones, pero no invalida por sí sola la conformidad interna de la custodia Rust.
- **Prioridad:** alta antes de la puerta algebraica y de la consolidación nuclear.

## 3. Estado de FFL-B

FFL-B se cerró tras E215 porque las obligaciones restantes identificadas no podían materializarse de forma honesta mediante una comprobación estructural adicional sin ampliar representación, semántica o ejecución.

Una publicación futura, una ampliación matemática o una nueva necesidad técnica podrá justificar una reapertura delimitada. La mera existencia de deuda no constituye por sí sola causa de reapertura.

## 4. Estado de FFL-C

FFL-C se cerró el 20/08/2026 con la evidencia correspondiente a su corte histórico. Las ampliaciones posteriores de la batería de conformidad no reescriben ese cierre, aunque el corpus vigente sea mayor.

FFL-C no modifica el contrato diagnóstico ni acredita capacidades de ejecución material ausentes. FFL-D permanece pendiente.

## 5. Regla de mantenimiento

Toda deuda que afecte a un cierre ya declarado deberá incorporarse a este registro o a su documento sucesor. Sólo podrá retirarse del conjunto de deuda viva mediante cierre acreditado o traslado expresamente justificado a otro bloque; cuando su efecto sobre la continuidad sea material, se conservará la trazabilidad del cierre.

Las actualizaciones deberán expresar hechos, fundamento, evidencia, alcance y estado, sin presentar hipótesis o previsiones como capacidades ya existentes.


## 6. Precisión diagnóstica tras reparar los oráculos · 06/09/2026

RETP-078 exige retorno 1, ausencia de IR y una identidad diagnóstica comprobable
por vía. DFL-001 permanece abierta: la tabla textual de Rust caracteriza sus
rechazos actuales y no constituye paridad diagnóstica completa con Python. En
particular, `admissibility_table_output_fuera_codominio.svp` conserva E011 en
Python y un rechazo sintáctico previo en Rust. El nombre del caso no acredita
cobertura de E011 en Rust. Véase el [acta de oráculos, §3](./ACTA_TECNICA_REPARACION_DE_ORACULOS_2026_09_06.md).

## 7. Cierre relacional N0-02 y residuo N0-03 · 06/09/2026

RETP-079 y el [acta N0-02](../arquitectura/ACTA_TECNICA_N0_02_TOTALIDAD_Y_UNICIDAD_DE_OUTPUT_SEMANTICS_2026_09_06.md) cierran, tras la promoción de su expediente, la totalidad y unicidad de la semántica enlazada por cada `CellSpec`. J-K1 se comprueba en Python y Rust con E115, conservando E102 para la referencia ausente o de tipo incorrecto. Esta concordancia concreta no cierra DFL-001 ni modifica su fotografía diagnóstica histórica.

N0-03 permanece abierto y es el siguiente relevo de K1. La sonda `semantics_unbound_duplicate` retira sólo `CellSpec` del testigo duplicado: Python sigue perdiendo un miembro y Rust emite homónimos. La comprobación universal de la proyección admitida debe cubrir ese supuesto, sin inferir una referencia de codominio ausente. La detección no acredita reparación ni aceptación normativa de la entrada. DFL-008 conserva ambas sondas CRLF y su condición de cierre.

## 8. Cierre N0-03 y continuidad K1 · 06/09/2026

RETP-080 y el [acta N0-03](../arquitectura/ACTA_TECNICA_N0_03_UNICIDAD_DE_MIEMBROS_Y_ESTABILIDAD_DE_PROYECCION_JSON_2026_09_06.md) resuelven, tras su promoción, el testigo residual de semántica duplicada sin `CellSpec`. La misma entrada se conserva y exige E115 en ambos emisores. La ausencia de homónimos se fundamenta en el inventario del esquema emitido y la guarda de sus mapas variables; el recorrido JSON acredita estabilidad del corpus sin pérdida de miembros, tipos ni precisión.

Esta resolución no cierra DFL-001: el texto general E115 se precisa y su identidad se conserva, pero sigue faltando concordancia diagnóstica general. DFL-008 mantiene ambas sondas CRLF y su condición de cierre. N0-04 es el siguiente paso de K1; no se acredita un importador de IR, un serializador canónico Rust completo ni suficiencia operacional universal.

## 9. Cierre N0-04 y continuidad K1 · 07/09/2026

RETP-081 y el [acta N0-04](../arquitectura/ACTA_TECNICA_N0_04_REFERENCIA_REAL_DE_ARQUITECTURA_DEL_HORIZONTE_2026_09_07.md) cierran, tras promoción, la resolución real de `Horizon.architecture` como grafo declarado y bien formado. La identidad de arquitectura del agente queda vinculada por su relación existente al mismo referente. La corrección explícita de un positivo histórico y sus huellas forman parte del expediente.

E006 conserva su contrato efectivo Python y Rust su rechazo textual de referencia tipada; no se amplía el catálogo ni se cierra DFL-001. DFL-008 conserva las dos sondas CRLF. La multiplicidad de sucesos y las relaciones causales de horizonte/frame fuera de J-H0 no se dan por resueltas. Sigue el acto de unicidad de CoupledSpec.bridges bajo BridgeSet; las decisiones de Domain y K1-T conservan el orden de la transición §14.

## 10. Retirada del compilador Python y deuda diferida · 07/09/2026

RETP-082 retira la vía activa afectada por DFL-008. Las cinco fuentes del banco permanecen: identidad y contenido CRLF se exigen directamente a la realización SV; las pérdidas se inyectan sobre los observables para comprobar la sensibilidad del detector. La retirada se acredita mediante esas mismas entradas en nativo, WASI y navegador, en la candidata promovida. El antecedente Python queda accesible en Git, sin recompilarlo como condición de conformidad.

DFL-001 conserva la concordancia diagnóstica y el alcance incompleto del serializador; la eliminación del comparador Python no los resuelve. DFL-009 se recibe en el retorno posterior a I/J. DFL-010 permanece en K1. Ninguna de estas anotaciones abre F ni cierra K1.

## 11. Unicidad BridgeSet y continuidad de deuda · 07/09/2026

RETP-083 impone J-B0 desde fuentes SV: rechazo de repeticiones como valores Nat, preservación de rango, vacío y orden declarado. Reduce esa obligación de DFL-001; no completa la concordancia general entre catálogo y diagnósticos. Los demás límites de §2/DFL-001 permanecen.

La siguiente decisión de K1 es la multiplicidad de `Horizon.events`, cuyo estatuto no se deriva de BridgeSet. Domain, K1-T y DFL-010 conservan su tratamiento; F sigue pendiente. DFL-009 conserva el retorno posterior al primer universo CYB como punto de evaluación, sin bloquear este avance.

## 12. Tipos de suceso del horizonte y continuidad K1 · 07/09/2026

RETP-084 resuelve la multiplicidad de `Horizon.events` por su carácter de declaración de tipos en el Documento III y la IR: una identidad por horizonte, con rechazo de repeticiones y conservación de orden. La recurrencia del mismo tipo en horizontes o datos de transición distintos permanece. No se amplía el catálogo diagnóstico ni se resuelve la multiplicidad de pares dentro de un único TransitionData o la axiomática general de horizontes.

Continúa K1 por el mínimo de Domain.parameters y la multiplicidad de parameter_id. DFL-005/010, los demás límites de DFL-001 y K1-T permanecen; F no queda habilitada. DFL-009 conserva su evaluación al retornar del primer universo CYB como punto de evaluación, sin bloquear este avance.

## 13. Rectificación del observador y de identidades · 08/09/2026

[RETP-094 del maestro](./REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-2026-094--discriminacion-causal-e115-y-rectificacion-registral) corrige cuatro expectativas E115 que descartaban los conjuntos de claves repetidas, ausentes y ajenas. El cierre local queda probado contra intercambios de causa y de referente; no acredita causalidad exhaustiva del corpus ni cierra DFL-001. El corte `bf660b00c0c2b38c7e5e4327b1e89f0ec81348be` supera su propia verificación integrada, identificada en RETP-094; no se cierra por ello DFL-001.

DFL-011 conserva el mandato del español. Se subsana la colisión candidata: `cell_ref` queda en DFL-012 e independencia semántica en DFL-013. La evidencia de RETP-093 conserva su corte y sus límites. DFL-005 y H06/H07 siguen pendientes; H04/H05 sólo tienen el cierre local candidato registrado, sin ejecución ni causalidad entre marcos. Se mantiene la secuencia de filas 7 y 8 y la competencia de Ciberseguridad para constituir su propio universo falsador.


## Sucesión de RETP-096/097 · 08/09/2026

RETP-096 registra la integración de PR #77 y #78 en `main@1706099aef4a0e3846706c3963e7c76313adaf68`. Los cierres locales de `TransitionData`, el refuerzo E115 y el testigo E011 dejan de estar sólo en candidata. Sus límites permanecen: no se cierra DFL-001 ni la fila 7.

RETP-097 propone comprobar H06/H07 **en su relación estructural**: arquitecturas de trayectoria/horizonte/contexto y pertenencia de referencias de consulta; `CoverageReport` contrasta sus tres nombres. La comprobación integrada está pendiente. Ninguna de esas guardas materializa `Interface`, `SilentU`, CQ1–CQ6, permisos o la cobertura del agente. La continuidad histórica entre versiones y la causalidad ejecutiva siguen sin acreditarse.

**DFL-005 permanece abierta y bloqueante por operación.** Su contrato de entrada y sus refutadores están delimitados en [el contrato de fila 7, §4](../arquitectura/CONTRATO_DE_CONTINUIDAD_Y_LIGADURAS_POR_OPERACION_FILA_7_2026_09_08.md#4-dfl-005-contrato-de-entrada-todavía-sin-realización). Falta realizar y probar identidades/versiones, ligaduras, compartición, usos y procedencia. Esta obligación no se sustituye por el cierre local de pertenencia. No se atribuye todavía DFL-005 a una estructura Rust ni a un esquema externo que el núcleo no valide.

DFL-011 (español), DFL-012 (nombre `cell_ref`) y DFL-013 (independencia) conservan identidad y obligaciones. La matriz de tratamiento del incremento no equivale a matriz final de resolución de pérdidas; no habilita el relevo a Ciberseguridad.

**Concordancia de `ArchitectureView` (DFL-003/DFL-001):** la variante material recibe arquitectura, especificaciones celulares, evaluaciones y compuertas; el antecedente IR 0.2 enumera arquitectura, evaluaciones, compuertas y supervisiones. RETP-097 comprueba pertenencia sobre la forma material y conserva esta diferencia como no reconciliada. No se ofrece la comprobación como semántica N4 completa ni se cambia la firma por inferencia.

**Verificación de la candidata RETP-097:** `abe5e544730f76f3052c9afd809446746df34c03` supera R0 Rust, conformidad, R0-8 y paridad nativa/WASI/navegador (ejecuciones y artefactos en el maestro). Once pruebas de contexto y controles de reparación puntual; corpus 14+106; 24/24 mutaciones dirigidas detectadas. H06/H07 quedan comprobadas sólo en la relación estructural descrita, dentro de PR #79, no promovida. Las obligaciones de DFL-005 y de N3/N4 no materializadas mantienen su estado abierto.


## Sucesión candidata RETP-098 · LIG/0.1

DFL-005 recibe una realización en `sv_core::bindings` del [contrato material previo](../arquitectura/CONTRATO_MATERIAL_DE_LIGADURAS_DFL_005_2026_09_08.md): identidad de programa y contrato, instancia, propietario, parámetro, referencias tipadas de captura/admisión, destino, usos ordenados, alias y compartición explícitos, procedencia y alcance con información lateral. La operación concreta se solicita por identidad y versión exactas. La candidata está pendiente de ejecutar sus comprobaciones y no se presenta como capacidad integrada.

**La deuda permanece abierta:** LIG/0.1 comprueba representación y referentes, no significado clínico ni autoridad de sus artefactos; no ejecuta transducción, Q0 ni recuperación de respuestas por F. Faltan la resolución final de las pérdidas del retorno y la candidata completa de fila 7. Las operaciones dependientes que no satisfagan esos contratos siguen fuera de capacidad. H06/H07 conservan el alcance candidato PR #79, sin atribuirles promoción. DFL-011/012/013 permanecen intactas en identidad y sede.


**Verificación RETP-098:** LIG/0.1 queda realizada y comprobada en `c4c50a0677498c66dd77f7c5a61b5e2173fd291c`, PR #80: cuatro flujos conformes, 45 pruebas Rust nuevas, 53 testigos en cuatro entradas con paridad literal entre nativo/WASI/navegador; 43/43 mutaciones dirigidas detectadas (19 nuevas). Los cortes, ejecuciones y artefactos están en RETP-098 del maestro CSV/Markdown. DFL-005 deja de describirse, para este alcance candidato, como ausencia total de representación. Sigue abierta para la resolución operacional del retorno y no se ofrece suficiencia Q0. La construcción privada del resultado conserva las ligaduras; no acredita permisos, interpretación de documentos ni recuperación clínica con S. PR #79 y #80 permanecen sin integrar y pendientes de contraste externo.


## Sucesión candidata RETP-099 · Resolución por operación G/H

DFL-005 se contrasta ahora con ocho pares documentales recibidos a través de LIG/0.1 y una matriz completa de inventario 15/44/81. La sonda aún está pendiente de prueba. La conservación de bytes, su identidad y la recuperación externa de campos no ejecutan significado clínico ni autoridad. Las doce SP integradas permanecen no ejecutadas; las exclusiones, responsables y condiciones de reapertura constan en GH-LIG/0.1 y su matriz. No se borra deuda por disponer de un documento referido.

La fila 7 sigue abierta. No se transforma esta propuesta en cierre de H06/H07 completos, N3/N4, K1-T, DFL-006, DFL-001 o independencia semántica. PR #79/#80 no promovidas; el nuevo transporte y la matriz requieren evidencia y contraste externo antes de emitir la candidata a Ciberseguridad.


**Verificación RETP-099:** el corte `bcf25c5b566a6535ba1c3851346fbb355b98ae84`, PR #81, acredita el transporte de los ocho pares GH-DOC por LIG: 48 transportes y recuperación documental F0/HS, con ocho pérdidas H conservadas y declaradas. Cuatro flujos conformes; ocho pruebas Rust y 16 ataques del observador satisfactorios; paridad literal en tres destinos. Este alcance deja de estar pendiente de realizar. La matriz 15/44/81 conserva responsables y límites; no otorga suficiencia clínica ni cierra las doce SP integradas. La revisión de las tres candidatas apiladas y la decisión de entrega de fila 7 siguen pendientes. No se licencia ninguna operación que necesite las capacidades excluidas.


## Rectificación candidata RETP-100 · Huella contractual ante el observador externo

La auditoría de `b58b4c88…` confirma el transporte y refuta la vigilancia del valor de `contract_sha256` por el observador GH-LIG. DFL-001 conserva esta omisión de observación hasta su corrección probada; no se inventa una nueva identidad DFL. RETP-100 exige contrato completo cotejado con su testigo y recálculo externo. El ensayo anterior no pierde sus ocho pérdidas documentales, pero no acredita el recálculo ahora exigido.

La pila #79/#80 sí modifica capacidad y aceptación de entradas mediante nuevas guardas. La #81 sólo añade pruebas y gobierno. La matriz requiere juicio humano sobre sus tratamientos; el control de inventario no valida esa prosa. Permanecen fila 7 abierta, doce SP no ejecutadas y ausencia de promoción; no se concede capacidad clínica mediante la rectificación del observador.


**Cierre local de la rectificación RETP-100:** `acfd168670ca69af33cc2966d9fce0fc9dd3adcd` supera los cuatro flujos, con 48 huellas recalculadas externamente y 28/28 ataques detectados por causa; informes idénticos nativo/WASI/navegador. Las ocho pérdidas documentales se conservan. La primera ejecución fallida del navegador y la corrección de sincronización constan en el maestro CSV/Markdown. DFL-001 no queda cerrada globalmente; se subsana la omisión concreta del observador. PR #81 no promovida, fila 7 abierta y revisión humana de la prosa de resolución pendiente.


## Continuidad de dependencias de verificación · RETP-101

El mandato del Director requiere entrega final de la DSL sin Python para construcción, generación imprescindible o ejecución. El inventario de entrada contiene 19 scripts Python de pruebas; su ubicación no autoriza permanencia en la entrega. Se prevé retirar dos en RETP-101 sin disminuir controles, conservando los demás como dependencia pendiente y explícita. No se crea una segunda identidad DFL: esta continuidad afecta a los oráculos de DFL-001, a la independencia de DFL-013 y al inventario de dependencias del cierre. DFL-011 y el acta del español permanecen intactos.

La interfaz web usa JavaScript y el observador externo usa Node; el compilador Rust no los incorpora como dependencias Cargo. Esa lectura estática no acredita por sí sola construcción de la distribución final en un entorno limpio. El contrato previo y los controles de representación están en [RETP-101](./DEPENDENCIAS_DE_VERIFICACION_Y_ENTRADAS_GH_2026_09_08.md). Las pruebas b=4 ya existen nativamente, pero su extensión al corpus compartido, el régimen de errores de frontera y la estrategia de entradas adversarias no quedan cerrados por retirar scripts.

**Incremento local verificado:** `a20afe9d76da173295f5c123614189b31d705974`, PR #82 en borrador. Se retiran los dos Python previstos; el generador Rust autónomo y el codificador externo conservan los 48 datos y la constante LIG. Pasan 7/7 pruebas Rust de preparación, 6/6 JavaScript, los cuatro flujos, 28/28 ataques del observador, 43/43 mutaciones previas y la paridad literal de los tres destinos. Se impide la conversión con pérdida en el subconjunto documental; no se restringe Nat. Permanecen 17 archivos Python, las invocaciones y fragmentos Python de CI, Node y `sha256sum`. El recuento de archivos no acredita eliminación de dependencias; la entrega final y su construcción en entorno limpio siguen pendientes. DFL-001/013 no se cierran globalmente y la fila 7 no se promueve.


## Continuidad hacia la candidata completa · RETP-102

Por orden del Director, la revisión externa fuerte se realiza al terminar materialmente la fila 7 y antes de CYB; no detiene los incrementos intermedios. Las menciones anteriores de revisión inmediata son históricas. Continúan todas las comprobaciones internas y las exclusiones por operación de GH-LIG. La nueva candidata deberá acreditar una ruta de construcción/pruebas Rust sin intérpretes; ese alcance no borra los 17 archivos Python ni el Python incrustado en CI, no cierra la distribución final ni la independencia semántica. Contrato previo en [RETP-102](../arquitectura/CANDIDATA_DE_FILA_7_Y_REPRODUCCION_SIN_INTERPRETES_2026_09_08.md).

**Preparación material completada:** `bb23381ba63a0512180ffb80f849a7241520b4c8`, PR #83, supera los cinco flujos. El paquete literal y determinista permite construir y ejecutar 348 pruebas Rust sin intérpretes Python/Node, sin red y sin caché anterior. Los cuatro ataques al paquete son detectados por causa. Se conservan 48 transportes G/H, 48 huellas recalculadas externamente, 28/28 ataques del observador y 43/43 mutaciones dirigidas; la matriz 15/44/81 mantiene las doce SP integradas no ejecutadas.

La candidata queda completa **para su adversarial externa**, no promovida ni entregada. DFL-005 tiene la realización representacional por operación descrita en LIG/0.1 y su contraste documental; no es una deuda ilimitada cerrada por esta prueba. H06/H07 sólo acreditan coherencia estructural, no CQ1–CQ6 ni causalidad ejecutiva. DFL-001/003/004/006/009/011/012/013 y las capacidades clínicas o de autoridad excluidas conservan sus obligaciones. La ruta Rust autónoma está probada en este corte, mientras la retirada global de los 17 Python y del Python de CI sigue explícitamente pendiente. La auditoría final debe examinar el cambio completo desde main y estos límites antes de CYB; no se requiere una nueva pausa de revisión por cada PR de la pila.


### Continuidad RETP-2026-103 — Adversarial del conjunto y fronteras de verificación

El [acta RETP-103](ADVERSARIAL_DE_FILA_7_Y_FRONTERAS_DE_VERIFICACION_2026_09_08.md) registra AF-01, AF-02 y AF-03: lectura JSON que perdía miembros repetidos, inventario que omitía archivos especiales y permisos ambientales no fijados. Quedan corregidos en `0b7fa120f3f09f98399f07bf87cda9410ca1b54e`, con once ataques por la CLI real, ocho de paquete y comparación bajo máscaras 0000/0077; cinco flujos conformes. El maestro CSV/Markdown y el acta conservan cortes, causas y custodia descargada.

DFL-001 conserva sus restantes obligaciones: este cierre candidato es local a los observadores y al paquete. DFL-005 y H06/H07 conservan el alcance representacional delimitado por sus contratos, sin ejecución clínica ni permisos materiales añadidos. La auditoría acumulada queda concluida con las correcciones verificadas; resta integración gobernada y decisión de entrega. No se ofrece Q0 ni se ejecutan las doce SP. La ausencia de intérpretes del paquete no cierra los 17 auxiliares Python y fragmentos de CI residuales, ni DFL-013. DFL-011, el acta del español y RETP-092 conservan identidad y mandato. No se abre Ciberseguridad ni se declara cerrado el núcleo.


### Continuidad RETP-2026-104 — Identidad de la matriz revisada

[RETP-104](REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-104) individualiza AF-04: una prosa de tratamiento alterada aún pasa la CLI y la API del observador en `e13dc8390e72848b4419ebe229d1b81666546eaf`, aunque los duplicados ya son rechazados. La aptitud indicada en RETP-103 queda condicionada a fijar la matriz literal revisada y cotejar todo su contenido. DFL-001 conserva las demás obligaciones. No se reabren AF-01/02/03, no se alteran decisiones de la matriz ni se autoriza ejecución Q0. Contrato previo; corrección pendiente, sin promoción ni apertura CYB.

**Resultado RETP-104:** AF-04 queda corregido en `eef05771aa2b5101571b3de3d58c41bcb4d64a4c`: matriz literal fijada, comparación completa por API, 15 ataques CLI y 104 sustituciones textuales rechazadas; 28 ataques anteriores conservados y cinco flujos conformes. La matriz no cambia. El maestro CSV/Markdown conserva la evidencia descargada y sus huellas. Se levanta la reserva específica al dictamen de RETP-103, sin cerrar DFL-001 general ni las capacidades ejecutivas excluidas, sin dispensar la retirada residual de Python y sin promoción o entrega automática.


<a id="relevo-retp-105"></a>

## Sucesión RETP-2026-105 — Integración y entrega representacional a CYB

[RETP-105 del maestro CSV/Markdown](REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-105) registra la integración real de PR #85, `140d319995a5df685860b348c0fd086ae6130faf`, árbol `46b2bbdbc65799c2753a2cd30fba343dde8140e2`, idéntico al de la cabeza probada `ab61d9bb7a2002d3a4a389e292c2a9b0a850d264`. La entrega adquiere efecto al incorporarse el asiento a main. Sustituye el estado de promoción pendiente de las sucesiones anteriores; no modifica sus pruebas ni sus límites.

| Obligación | Estado tras esta integración y efecto sobre la oferta |
|---|---|
| DFL-005 | LIG/0.1 y su contraste documental quedan integrados; queda cerrado ese subconjunto de representación por operación. La deuda general continúa abierta para campos sin interpretación completa, autoridad y capacidades adicionales. La mera presencia de Domain/Agent no acredita constitución ni permisos. |
| H06/H07; DFL-003/004 | Guardas de coherencia estructural integradas. No se ofrecen CQ1–CQ6 completos, continuidad persistente entre versiones ni causalidad ejecutiva; las operaciones que los necesiten requieren su realización antes de admitirse. |
| DFL-001 | Reparaciones causales y AF-01…AF-04 integradas. Continúan la concordancia diagnóstica general y las obligaciones no agotadas; igualdad documental y paridad de destinos no son prueba universal de semántica. |
| K1-T y DFL-006 | Producción observación→Tri, criticidad y resultados ejecutivos pendientes: excluidos de la oferta actual. Conservan la puerta algebraica y sus comprobaciones posteriores conforme a la tabla rectora. |
| Q0 y SP-01…SP-12 | Suficiencia integrada no acreditada. No se transfieren a CYB como ejecución disponible; el relevo representacional no requiere ejecutar clínicamente Q0. |
| DFL-009 | Evaluación del servicio y soporte diferida a fila 9 tras el retorno CYB; no elegida por esta integración. |
| DFL-011 | Español: mandato íntegro y revisión de todos los repositorios antes del cierre nuclear. No se renumera ni se convierte en freno de fila 8. |
| DFL-012 | Nombre histórico `cell_ref` conservado en serializador 0.1.0, con identidad efectiva de nodo. Migración y compatibilidad pendientes de su decisión de versionado. |
| DFL-013 | No existe segunda realización semántica independiente. Permanece abierta; la paridad nativo/WASI/navegador no la cierra. |
| Dependencias de entrega final | Persisten 17 auxiliares Python e invocaciones de CI y Node externo. El paquete Rust aislado está probado sin esos intérpretes; no se dispensa la retirada global ni se declara completa la distribución final. |
| Infraestructura y agentes materiales | Las garantías R2/R3/R4, soporte, persistencia y autoridad material no se acreditan por el contrato documental CYB ni por esta entrega. Conservan sus etapas propias. |

La fila 7 queda completada para la entrega representacional identificada; el siguiente trabajo sustantivo es fila 8, a cargo de Ciberseguridad Inteligente. La matriz documental fijada conserva su corte candidato: este cambio de gobierno se registra por sucesión, sin modificar sus banderas para fabricar una certificación automática.

El receptor comprobará su corte y el paquete antes de constituir su perímetro y catálogo Excel. Si la operación CYB necesita una capacidad excluida, se registra insuficiencia o se devuelve la obligación al Lenguaje; no se rellena con una cadena opaca, un supuesto permiso ni `U`. Una ausencia de contraejemplo se informa como tal, con su cobertura finita. No se declara cerrado el núcleo ni recibida una auditoría CYB todavía no ejecutada.

<a id="relevo-retp-106"></a>

## Sucesión RETP-2026-106 — Retorno CYB y uso documental contrastado

[RETP-106](REGISTRO_EVOLUCION_TECNICA_PROYECTO.md#retp-106) recibe el relevo CYB aprobado en `169af16d05ffc954454bb5528ec106e5752b7016` y contrasta LIG integrado sobre `66967a80…`. Su candidata `219d6a374c38d1a14a7e191592032f70f89ee402` preserva los originales y supera seis flujos. El [dictamen receptor](RECEPCION_Y_CONTRASTE_OP_CYB_001_2026_09_09.md#53-dictamen-por-capacidad) limita la suficiencia a conservación y uso documental externo: 78 casos, 18 pares y 186 contratos comprobados. No amplía la oferta productiva ni cierra por referencia ninguna deuda ejecutiva.

| Obligación | Resultado recibido y próximo objeto concreto |
|---|---|
| DFL-005 | LIG preserva estas cargas completas y laterales; los 18 pares pierden información sólo en H. Queda por definir qué consumidores operacionales necesitan aplicar atribución, autoridad, tiempo, vistas, cobertura y continuidad; representar el documento no equivale a ejecutar su regla. Mantener los límites por operación hasta realizar ese consumidor. |
| DFL-003/004 y H06/H07 | Se conservan las guardas integradas. RS03/09/10/12 precisan historia, antecedentes, relaciones y reevaluación; el observador externo no acredita CQ1–CQ6 ni causalidad de una trayectoria SV. Definir el uso necesario antes de alterar su representación. |
| Constitución celular CYB | Las 32 definiciones no constituyen célula, agente ni posiciones. El banco D/AG es sintético. Una necesidad de Frame/destino/cobertura celular vuelve al dominio únicamente con la pregunta y operación que la requieren. |
| K1-T y DFL-006 | La concordancia de reglas externas no habilita captura, admisión ni observación→Tri productivas; tampoco criticidad ni consejo ejecutado. Identificar los consumidores dependientes y conservar la puerta algebraica correspondiente. |
| DFL-001 | Los doce ataques al nuevo observador y sus referencias fijadas cubren ese perímetro, sin cierre general de diagnósticos ni de independencia del criterio. |
| DFL-009 | Tras este retorno aprobado llega su oportunidad de evaluación en fila 9: servicio nativo, identidad efectiva, recursos, aislamiento, fallos, repetición y coste. No queda resuelta ni se selecciona plataforma; se cotejará con los ensayos identificados del laboratorio. |
| DFL-011/012/013 y distribución final | Español, nombre histórico `cell_ref` y diversidad independiente conservan sus identidades. Continúan 17 auxiliares Python e invocaciones de CI, más Node externo. Las 359 pruebas aisladas no acreditan su retirada global. |
| R2/R3/R4 | Autenticidad, permisos, difusión, custodia, persistencia y cobertura reales siguen sin acreditación por este banco. Un resultado documental favorable no autoriza una actuación ni certifica infraestructura. |

Recepción y primer contraste efectuados en candidata; fila 9 y núcleo abiertos. Ciberseguridad permanece en pausa controlada. La próxima decisión debe fijar consumidores y dependencias por operación, conservar la necesidad profesional aprobada y escoger la representación suficiente con evidencia; no repetir el catálogo, constituir otro universo ni dispensar pendientes mediante una etiqueta.


<a id="relevo-retp-107"></a>

## Sucesión candidata RETP-107 · Regla de consumo documental y evaluación DFL-009

Base `60bfdf44…`; material `f5a43131c8743867ee039bfe52f1048a52314058`, árbol `7f6b333b88ed2da685289cf1cc33494ba181a558`, [PR #88](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/88), dependiente de #87. El [contrato y resultados](CONTRATO_DE_CONSUMO_DOCUMENTAL_CYB_2026_09_09.md) reemplazan la definición OP sintética del banco por una definición exacta que liga selector, regla, consumidor y ámbito. Los restantes artefactos de constitución, autoridad y captura/admisión siguen siendo sintéticos. El enlace externo está probado; la ligadura productiva CYB no se declara resuelta.

| Obligación | Alcance tras RETP-107 |
|---|---|
| DFL-005 | Se comprueban 186 definiciones y sus módulos, 150 consumos y 36 rechazos de H en el banco. No se acredita constitución de agente ni autoridad institucional, captura/admisión profesional o ligadura a posiciones CYB. La correspondencia operacional de identidad, competencia y alcance es el siguiente objeto; no se copia semántica profesional al núcleo. |
| DFL-009 | **EVALUADA_DOCUMENTALMENTE_CON_BRECHAS_MATERIALES_ABIERTAS.** Se reciben 018/020 en su corte y se delimitan ejecutable local, FFI, WASM, proceso separado, servicio y dependencias. No se acredita servicio remoto, aislamiento contra anfitrión comprometido, recuperación ni enlace de efectos. Nuevas realizaciones se ensayarán en laboratorio antes de promoción. |
| DFL-003/004/006 y K1-T | Conservan productores, consulta, trayectoria, causalidad, captura y transducción productivas pendientes. El consumidor documental no es `query`, Frame, consejo, permiso R1 ni realización algebraica. |
| DFL-001/011/012/013 | Sin cierre ni renumeración. El acta del español y RETP-092 no cambian; independencia de realización SV no recuperada por módulos externos. |
| Autonomía y auxiliares | 360 pruebas Rust aisladas sin Python/Node. Dos módulos mjs se incluyen visiblemente como bytes de prueba, no se interpretan por Rust ni se incorporan a sv-native. Node sigue siendo dependencia del consumo externo; 17 Python previos pendientes de retirada global. |

No se licencia como disponible ninguna capacidad profesional pendiente. El cotejo de requisitos IMM/CYB no suma permisos, células o garantías. DFL-009 deja de ser únicamente una evaluación diferida, pero no pasa a resuelta. Candidatas sin integrar; fila 9 y núcleo abiertos; Ciberseguridad en pausa. README, actas históricas y repositorios de dominio intactos.
