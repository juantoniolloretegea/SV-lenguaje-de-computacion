# Traceable Learning — mapa de continuidad futura para IR N3/N4 y compuerta pre-DSL

**Versión:** v0.2  
**Fecha:** 16/08/2026  
**Rama:** `traceable-learning-ir-v0.1`  
**Estado:** `LATENTE_LEGITIMO / VIGILANCIA_DOCTRINAL_ACTIVA`; documentación prospectiva; sin autorización de integración  
**Mapa antecedente:** `TRACEABLE_LEARNING_MAPA_DE_CONTINUIDAD_IR_N3_N4_v0_1.md`  
**Acta de recepción:** `docs/calidad/ACTA_TECNICA_DE_RECEPCION_DOCTRINAL_Y_PRESERVACION_DE_CONTINUIDAD_DEL_APRENDIZAJE_TRAZABLE_HACIA_IR_N3_N4_2026_08_15.md`  
**Acta complementaria de fijación:** `docs/calidad/ACTA_TECNICA_COMPLEMENTARIA_DE_FIJACION_DE_FUENTE_DOCTRINAL_Y_PRESERVACION_PRE_DSL_DEL_APRENDIZAJE_TRAZABLE_2026_08_16.md`

## 1. Propósito

Este documento sucede al mapa v0.1 sin invalidarlo históricamente. Preserva el puente entre la publicación doctrinal sobre aprendizaje trazable y una eventual traducción futura al Lenguaje SV, incorporando el perímetro final de custodia/continuidad y un contraste explícito con la IR v0.2 y la gramática superficial v0.1.

No define sintaxis, no modifica la IR canónica vigente, no constituye propuesta de merge y no convierte el checker de la publicación en especificación del lenguaje.

## 2. Fuente doctrinal fijada

La fuente doctrinal propia que deberá utilizar una futura reapertura queda identificada por:

- `SV-matematica-semantica/documentos/fundamentos/aprendizaje-trazable-en-inteligencia-artificial/`;
- publicación española: **“Aprendizaje trazable en inteligencia artificial: evolución estructural del conocimiento con frames ternarios y trazas acumulativas”**;
- DOI español: **`10.21428/39829d0b.bebc607c`**;
- publicación internacional: **“Traceable Learning in Artificial Intelligence: Structural Knowledge Evolution with Ternary Frames and Cumulative Traces”**;
- *Journal of Automated Reasoning*, Submission ID **`6a8347e5-23d3-4f03-87e7-1a8e95e5e594`**, versión de envío **v1.0**, estado **Technical check** a 16/08/2026;
- cápsula Code Ocean v1, enviada y en verificación, DOI provisional **`10.24433/CO.4645115.v1`**;
- checker/cápsula: 18 casos finitos de referencia.

El DOI provisional de Code Ocean es identificador de preservación/reproducibilidad, no una autorización de integración técnica.

## 3. Principio de traducción

La secuencia obligatoria permanece:

`publicación doctrinal fijada → mapa de obligaciones → matriz doctrina/IR → diseño IR tipado → juicios de bienformación → errores → lowering/DSL → validator → runner/motor → evidencia`

No se invertirá esa secuencia. En particular:

- la sintaxis no determinará retrospectivamente la semántica;
- los nombres JSON de la cápsula no adquirirán rango canónico;
- la existencia de una realización ejecutable no sustituirá la definición de tipos, juicios ni errores del Lenguaje SV.

## 4. Núcleo semántico que debe preservarse

### 4.1. Capas distintas

Deben permanecer separados:

1. `Frame`/vector local;
2. trayectoria del sistema;
3. registro histórico de conocimiento;
4. proyección activa del conocimiento;
5. episodio finito sobre el que se formula el predicado de aprendizaje.

Una única estructura de almacenamiento puede realizar más de una capa solo si preserva formalmente esas distinciones; una decisión de persistencia no puede colapsarlas semánticamente.

### 4.2. Evolución, incremento y aprendizaje

El futuro contrato deberá conservar:

- `Evol_D(E)`: existencia de cambio estructural registrado;
- `Inc_D(E)`: incorporaciones históricamente nuevas y sustentadas;
- `Learn_D(E)`: evolución con al menos un incremento sustentado.

Pérdida pura puede producir evolución sin aprendizaje. Crecimiento cardinal no es condición necesaria ni suficiente.

### 4.3. Novedad histórica y recuperación

La frescura no puede calcularse únicamente sobre el estado activo inicial del episodio. Debe consultarse o preservarse la historia anterior necesaria para distinguir:

- adquisición históricamente nueva;
- recuperación de una clase conocida y retirada;
- recuperación acompañada de una nueva relación, ruta, argumento o contenido no equivalente.

### 4.4. Soporte y política

El soporte de un incremento se apoya en testigos finitos registrados. La política `ρ_E` fija la frontera finita de soporte admisible y la completitud relativa. Debe estar fijada antes del veredicto; cambiarla después del resultado crea otra instancia.

### 4.5. Histórico frente a operacional

No se confundirá:

- la proposición histórica `Learn_D(E)` sobre un episodio completo y un registro de soporte completo;
- la consulta operacional `DecLearn_D(E*)`, que puede devolver `LEARN`, `NO_LEARN` o `U` después del agotamiento declarado de la base accesible.

`U` operacional no es error de entrada, ausencia de campo, pendiente de análisis ni negación histórica.

### 4.6. Autoridad

El aprendizaje no confiere autoridad sobre la especificación rectora definida por autoridad humana. Una revisión humana es una operación distinta y versionada. Un episodio no cruza silenciosamente una revisión de esa especificación.

## 5. Correspondencia revisada con la IR v0.2

| Objeto doctrinal | Función | Anclaje IR actual | Estatuto pre-DSL |
|---|---|---|---|
| frame `F_r` + vector `v_r` | manifestación local inmutable | N3 `Frame` + estados/resultados inferiores | anclaje fuerte ya existente; no duplicar |
| traza acumulativa `Γ_{0:r}` | historia estructural append-only | N3 `Trajectory` | afinidad fuerte; preservar distinción respecto del registro de conocimiento |
| registro de conocimiento `L_r` | altas, retiradas, ejecuciones y procedencia relevantes | sin nodo canónico específico | diseño abierto; posible N3 o estructura subordinada, no decidir por almacenamiento |
| conocimiento activo `(X,R,Λ)` | proyección vigente de contenidos, relaciones y rutas | N4 / uso y consulta | diseño abierto; no confundir con `Frame` completo ni con historial |
| episodio `E_{i:j}` | unidad finita de comparación | frontera N3/N4 | diseño abierto; debe referir segmento de trayectoria y una única versión rectora |
| acción `Act_D` | reproducción determinista del registro | semántica de uso | operación interna futura; no necesariamente palabra superficial |
| testigo `W_a` | soporte finito reconstruible | N4 `Justification` es punto próximo | afinidad, no identidad; debe conservar ocurrencias ejecutadas y procedencia |
| política `ρ_E` | frontera de soporte y completitud relativa | `QuerySpec`/`QueryContext` + gobierno de dominio | requiere tipo/contrato expreso antes de DSL |
| `Evol_D(E)` | evolución estructural | resultado analítico sobre N3 | candidato de semántica de consulta |
| `Inc_D(E)` | incrementos nuevos y sustentados | resultado estructurado | no reducir a escalar |
| `Learn_D(E)` | proposición histórica fuerte | consulta/resultado N4 | debe permanecer separada del veredicto operacional |
| `DecLearn_D(E*)` | `LEARN / NO_LEARN / U` tras agotamiento | N4 `QueryResult` | encaje conceptual fuerte; requiere nuevo contrato de contexto/justificación |
| especificación rectora humana | dominio, semántica, operadores, cierre, validez y autoridad | presión transversal N0/N4 y AUTH | identidad/versionado obligatorios; no confundir con conocimiento aprendido |
| revisión humana | cambio autorizado de especificación rectora | gobierno/versionado | externa a transiciones ordinarias de máquina |
| restart code `Q_r` | reconstrucción determinista de estado | persistencia futura | fuera del núcleo mínimo salvo decisión explícita |

## 6. Hechos ya aprovechables de la IR vigente

La IR v0.2 aporta propiedades que deben reutilizarse, no reescribirse:

- `Frame` es inmutable;
- `Trajectory` es append-only;
- `TransitionData` referencia horizonte y cambios inducidos;
- N4 ya contiene `Domain`, `Agent`, `QuerySpec`, `QueryContext` y `QueryResult`;
- `QueryResult` ya exige `TypedResponse`, `Justification` y `QueryMetadata`;
- los juicios J5 obligan a justificación reconstruible y prohíben pasos opacos en consulta;
- una consulta no modifica la trayectoria.

La futura integración debe tensar y extender esos contratos solo donde la doctrina lo exija, evitando duplicación nominal.

## 7. Hechos ya aprovechables y huecos de la gramática v0.1

La gramática superficial actual ya posee:

- declaraciones `domain`, `agent`, `query_spec`, `frame`, `transition_data`, `trajectory`;
- operador `query` con contexto explícito;
- invariantes de inmutabilidad de `Frame` y append-only de `Trajectory` garantizados por tipo;
- prohibición de `query` con contexto opaco.

Los `query_type` actuales son:

`PointEvaluation`, `TrajectoryState`, `FrameComparison`, `CoverageState`, `PendingU`, `GlobalCriticality`.

No existe todavía:

- consulta de aprendizaje;
- declaración superficial de episodio;
- declaración de registro histórico de conocimiento;
- política/testigo de soporte como objetos superficiales;
- contexto específico de episodio de aprendizaje.

Esto se registra como **hueco de diseño futuro**, no como defecto de la gramática v0.1. No se añadirá una palabra reservada `learn` ni equivalente hasta cerrar la matriz pre-DSL.

## 8. Obligaciones N3 antes de una extensión

### N3-A — Registro histórico distinto de trayectoria

Debe decidirse si la IR incorpora un nodo específico o una realización subordinada a `Trajectory`. La decisión debe preservar historia del sistema, historia de conocimiento y proyección activa como planos diferenciables.

### N3-B — Anclaje inmutable

Toda entrada de conocimiento que refiera frame, transición, ejecución o entrada externa debe usar referencia estable y no permitir inserción retrospectiva.

### N3-C — Historia anterior al episodio

La realización debe poder distinguir una recuperación antigua de una adquisición fresca. El estado activo al límite `i` no basta por sí solo.

### N3-D — Límites ordinales y versión rectora

Un episodio referencia límites ordinales `i < j`, no tiempos físicos constitutivos, y una única versión de la especificación rectora humana.

### N3-E — Ocurrencias ejecutadas

Razonamiento o composición solo pueden atribuirse como soporte si la ocurrencia concreta fue ejecutada, registrada y reconstruible.

## 9. Obligaciones N4 antes de una extensión

### N4-A — Consulta histórica frente a operacional

Debe existir una distinción tipada entre proposición histórica y consulta presente. No debe utilizarse `U` para encubrir malformación o trabajo no agotado.

### N4-B — Soporte y completitud relativa

La política de soporte forma parte del contexto autorizado y debe estar fijada antes del veredicto.

### N4-C — Justificación concreta

Un resultado positivo debe referir los incrementos y testigos concretos que lo sustentan bajo la política declarada.

### N4-D — Procedencia

La procedencia de contenido, relación, ruta, soporte y decisión debe sobrevivir a composición y transducción.

### N4-E — Identidad de la especificación rectora

Un identificador superficial no basta si el contenido puede mutar bajo el mismo nombre. Se requiere identidad/versionado inmutable o digest vinculado al contenido.

## 10. Custodia y continuidad bajo pérdida

La primera versión del mapa conservaba la diferencia entre reconstrucción de estado y procedencia. La fuente doctrinal final obliga a ampliar esa separación. Deben tratarse como propiedades distintas:

1. disponibilidad del soporte histórico originario;
2. reconstruibilidad del estado;
3. reconstruibilidad de procedencia/genealogía;
4. suficiencia del registro retenido para volver a cerrar operacionalmente el veredicto histórico.

No se inferirá ninguna de las otras tres a partir de una sola. En particular:

- perder un soporte no implica perder el estado si permanece un restart code suficiente;
- reconstruir el estado no reconstruye automáticamente la procedencia;
- una vista retenida insuficiente puede terminar en `U` tras agotamiento sin convertir el hecho histórico en `NO_LEARN`;
- una prueba hallada posteriormente no se registra retroactivamente como testigo histórico originario.

Este perímetro enlaza directamente con la custodia estructural del diseño, del DSL y de los laboratorios.

## 11. Interacción obligatoria con frentes latentes y transversales

### 11.1. AUTH

A.2 r2 y J6 permanecen preservados en `sv-auth-v0.2` sin integración. Una futura sintaxis de aprendizaje no podrá reutilizar nombres ni permisos AUTH por comodidad. Aprender y estar autorizado son dimensiones distintas.

### 11.2. REAL/SIM

Una rama simulada no constituye historia real. Un resultado de simulación o una consulta IA no puede fabricar un aprendizaje histórico real sin el proceso de constitución y registro correspondiente.

### 11.3. Capa IA trazable por agente

La nota técnica de previsión ya exige consulta trazable, justificación y metadatos reconstructibles. La futura semántica de aprendizaje deberá integrarse con ese contrato sin elevar una capa IA auxiliar a fuente de verdad o autoridad.

### 11.4. Custodia estructural

Persistencia, serialización, parser, validator, lowering, consultas y laboratorios deberán declarar cómo preservan la estructura y qué fallos de continuidad detectan. La custodia no puede añadirse después como metadato decorativo.

## 12. Banco mínimo de propiedades adversariales

La futura traducción no copiará los fixtures actuales, pero deberá volver a hacer observables, adaptadas al contrato final, al menos estas 18 propiedades:

1. adquisición nueva de contenido sustentado;
2. aprendizaje solo por relación;
3. aprendizaje solo por ruta;
4. pérdida sin aprendizaje;
5. razonamiento ejecutado sin aprendizaje;
6. adquisición sin razonamiento interno;
7. composición participante en soporte con aprendizaje;
8. composición ejecutada sin aprendizaje;
9. composición participante pero no esencial por existir soporte alternativo;
10. reapertura 0/1 → `U` con aprendizaje y contramodelo de pérdida pura con iguales extremos locales;
11. reexpresión representacional equivalente sin incremento;
12. consulta agotada con base retenida insuficiente → `U`;
13. intento de reescritura de especificación rectora por máquina → rechazo;
14. recuperación de clase conocida sin segundo incremento;
15. recuperación con una ruta genuinamente nueva;
16. mutación fresca sin soporte → evolución sin aprendizaje;
17. clase adquirida y retirada antes del episodio, recuperada durante el episodio → no fresca;
18. identidad de especificación rectora alterada bajo igual rótulo superficial → rechazo.

## 13. Preguntas arquitectónicas abiertas

No quedan resueltas por este mapa:

- ubicación definitiva del registro de conocimiento en N3, N4 o estructura subordinada;
- relación exacta entre conocimiento activo y `Frame` completo;
- tipo canónico de episodio;
- equivalencia representacional por tipo;
- catálogo de errores;
- serialización de política de soporte y testigos;
- granularidad de procedencia;
- forma canónica de identidad/versionado de especificación rectora;
- incorporación o aplazamiento de restart codes;
- nueva variante de `QueryContext` o reutilización tipada de una existente;
- sintaxis superficial final;
- lowering;
- interacción concreta con AUTH integrado cuando ese frente se reabra;
- handoff REAL/SIM;
- reglas ejecutables de custodia y precedencia.

## 14. Compuerta pre-DSL

La primera tarea al reabrir este frente **no será programar**. Deberá construirse y aprobarse una matriz con columnas mínimas:

| Doctrina | IR v0.2 actual | Cambio mínimo | Sintaxis candidata | Juicio de bienformación | Error observable | Lowering | Evidencia/prueba | AUTH/REAL-SIM/custodia |
|---|---|---|---|---|---|---|---|---|

La matriz deberá resolver al menos:

- qué objetos requieren nodo IR nuevo y cuáles caben en objetos existentes;
- qué elementos deben ser visibles en superficie y cuáles son semántica interna;
- cómo se representa el episodio sin duplicar `Trajectory`;
- cómo se transporta la historia previa necesaria para frescura;
- cómo se fija `ρ_E` antes del veredicto;
- cómo se expresa o referencia el testigo sin convertir el DSL en lenguaje de pruebas universal;
- cómo se produce `LEARN / NO_LEARN / U` dentro del contrato de `QueryResult`;
- qué errores son malformación y nunca `U`;
- qué cruces de versión rectora son ilegales;
- qué operaciones pertenecen a AUTH y no a aprendizaje;
- cómo se preservan REAL/SIM y custodia.

Solo después de cerrar esa matriz y de aprobar un acta de reapertura podrá decidirse si procede modificar N3, N4, N0/N2, gramática, parser, validator o capas posteriores.

## 15. Regla de no apropiación de la cápsula

La cápsula Code Ocean y el checker de 18 casos son una realización finita de ejemplos y regresión. Sirven como evidencia reproducible y banco de propiedades, no como fuente de tipos ni como contrato de serialización del Lenguaje SV.

Ningún JSON, nombre de campo, digest o convención interna de la cápsula adquiere rango canónico por existir o por haber superado la ejecución reproducible.

## 16. Punto de continuación

Cuando el director humano abra el frente DSL, el orden correcto será:

1. contrastar este mapa v0.2 con la cabeza vigente de `main`;
2. levantar la matriz pre-DSL;
3. atacar adversarialmente la matriz contra IR v0.2, gramática v0.1, AUTH, REAL/SIM y custodia;
4. abrir acta de reapertura si el diseño queda suficientemente cerrado;
5. solo entonces tocar IR, gramática, parser o lowering.

## 17. Cierre

**Doctrina fuente:** fijada.  
**DOI español:** `10.21428/39829d0b.bebc607c`.  
**Code Ocean:** `10.24433/CO.4645115.v1`, provisional y en verificación.  
**JAR:** Submission ID `6a8347e5-23d3-4f03-87e7-1a8e95e5e594`, v1.0, Technical check.  
**IR vigente:** no modificada.  
**DSL vigente:** no modificado.  
**Código ejecutable:** no modificado.  
**Merge a `main`:** no autorizado.  
**Siguiente paso legítimo:** matriz pre-DSL y acta de reapertura antes de implementación.