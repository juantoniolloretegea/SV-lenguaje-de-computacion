# N0 — Radiografía de objetos, invariantes y oráculos del núcleo SV

**Fecha:** 4 de septiembre de 2026  
**Repositorio:** `juantoniolloretegea/SV-lenguaje-de-computacion`  
**Rama:** `cierre-nuclear-20260904`  
**Base exacta:** `main@736ea643d7f65ba4bf26dbbb321383b8becc8d64`  
**Naturaleza:** clasificación forense y orden de cierre; acto exclusivamente documental  
**Estado:** `N0_CONSTITUIDA · NUCLEO_NO_CERRADO`  
**Efecto material sobre el Lenguaje:** ninguno  
**Reconciliación posterior:** leída y comprobada sobre `main@230a205b08f4c54c9c8d9c1c7ad35b2f6ddbbfc4`; queda subordinada a `AGENTS.md`, a `RETP-2026-072`, a `RETP-2026-073` y a los Pilares y restricciones de diseño del Lenguaje SV.

## 0. Precisión terminológica obligatoria

La sigla `N0` de este documento identifica una **etapa de radiografía previa al cierre nuclear**. No designa ni modifica el nivel `N0 — Definición` de la IR canónica.

Este acto no constituye una nueva meta-IR, un `PerfilDominio`, un agente, un superagente, una fase de persistencia ni una semántica de dominio.

La clasificación forense y la secuencia propuestas en este corte no pueden utilizarse para decidir células, valores de `b`, asignaciones de parámetros, cobertura de agentes, bus, host ni ejecución algebraica no acreditada. Cuando una pieza posterior de rango aplicable precise el orden de trabajo, prevalece esa pieza sin borrar la identidad histórica de esta radiografía.

## 1. Objeto

Esta radiografía fija qué existe hoy, qué invariante falta, qué prueba puede falsarlo y en qué momento debe resolverse. Su función es impedir dos errores simétricos:

1. cerrar como universal una forma obtenida de un solo dominio;
2. posponer defectos intrínsecos del Lenguaje bajo el pretexto de esperar nuevos dominios.

La radiografía clasifica objetos ya existentes. No introduce una capa normativa nueva para evitar corregirlos.

## 2. Dictamen ejecutivo

El corte examinado contiene un núcleo pequeño, tipado y sin mapas genéricos en la IR Rust, pero admite estados estructurales que contradicen o dejan indeterminadas obligaciones de su propia representación.

Se distinguen tres clases:

- **cierre intrínseco inmediato:** puede decidirse leyendo la propia IR, sin conocer Inmunología ni otro dominio;
- **determinación normativa previa:** el defecto es visible, pero la representación vigente no basta para elegir una corrección sin declarar antes el significado del campo;
- **decisión de frontera:** depende de cómo dos dominios heterogéneos utilicen identidad, nombres, procedencia y ensamblaje; no debe cerrarse desde un solo caso.

Por tanto:

```text
N0 = RADIOGRAFIA_Y_ORDEN_DE_ORACULOS
N0 = NO_META_IR
N0 = NO_IMPLEMENTACION
CIERRE_INTRINSECO = PROCEDE_TRAS_N0
DECISIONES_DE_FRONTERA = DIFERIDAS_HASTA_DOS_CONTRATOS_DE_DOMINIO
DOMINIO = NO_AGENTE
SUPERAGENTE = NO_CONSTITUIDO
R2 = SIN_EJECUCION_MATERIAL
```

## 3. Corte y evidencia leída

### 3.1 Identidad de los árboles

| Árbol | SHA Git en la base |
|---|---|
| `rust/` | `41bfb11c6de2a9a890908f877b528e73c607352b` |
| `src/` | `3672e4fcc918c940004ccd08f5093dc0244a15ae` |
| `tests/` | `55d5ed67b2b5416b4c4b104e0f9e21a9dcd3d756` |
| `grammar/` | `0b3d439f635693cd44de17c4ae6245bc9ebbaa71` |
| `spec/` | `93f0b7df19305e9d1ac499c90dd9630eb172d45a` |

### 3.2 Piezas rectoras y realizaciones

| Pieza | SHA-256 | Lectura aplicable |
|---|---|---|
| `IR_CANONICA_BIENFORMACION_SV_v0_3.md` | `80abb22eb40561ca2e77b0bb3c03dc3571b1e141e4531650de432b14462e8fee` | sucesora normativa de IR v0.2; conserva lo no sustituido expresamente |
| `GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md` | `8aba8c16859afb1a59407a20453819d42dcc46bb3c0d9bbb2dc4125cf4ca5793` | superficie vigente; baja a IR v0.3 |
| `docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md` | `3ed5df0e851538fe6f19a705251296d519917c253c6b35f33bafa6793f5db339` | DFL-001..006 vivas; DFL-007 cerrada |
| `rust/sv_core/src/ir.rs` | `81cd04c33fb314991dc486d5ca8969042c0467db4a967298bc68ba9d1d2cb25b` | tipos cerrados de la IR Rust |
| `rust/sv_core/src/wellformed.rs` | `9c4c6efffcde4eb9be272259f7b3a6817959dfb337d8b2d0615d12386d75c52e` | juicios estructurales Rust |
| `rust/sv_core/src/equivalence.rs` | `fd1bb360827153b153db0c5051f5659e0e8a8c201b27f5b044a40c4c216252f3` | proyección diferencial; no serializador canónico completo |
| `rust/sv_core/src/lib.rs` | `dca8ee2106c61079109727bc26ffcbb3260b485d8528ceef8ff9761f140299e2` | unidad fuente, identidad y ensamblaje multifuente |
| `src/svp_validator.py` | `089d2d73fe9fbbf86556dfd0d0f3e8a27ceed3c8f2554bf2b9160f2c993243fa` | validación de referencia Python |
| `src/svp_serialize.py` | `05f94ea8a366e366b991693bb5767180d160b2f9f33c67cd46ade558d04b83e8` | serialización canónica completa de la IR implementada en Python |

También se leen los cuatro documentos de transición de `docs/dominios/inmunologia/`, ya integrados mediante la PR #60.

### 3.3 Línea basal comprobada en este acto

Sobre la base exacta se reejecutó la conformidad de referencia Python:

```text
validos = 12/12
invalidos = 67/67
total = 79/79
oraculos_validos_modificados = 0
```

El presente acto no reabre ni vuelve a certificar Rust, nativo o WebAssembly. Conserva la evidencia vigente declarada en DFL-007 y exige reejecución de las dos realizaciones cuando comience cada corrección material.

### 3.4 Derivas documentales observadas y no corregidas por N0

La sección 8 de `IR_CANONICA_BIENFORMACION_SV_v0_3.md` declara una batería de 72 casos —11 válidos y 61 inválidos—, mientras que el árbol exacto de pruebas ejecutado en este acto contiene 79 —12 válidos y 67 inválidos—. La divergencia es una deriva documental verificable: no invalida la ejecución 79/79, pero impide presentar el recuento de la IR v0.3 como fotografía vigente. Deberá reconciliarse de forma explícita; N0 no reescribe retrospectivamente ninguno de los dos cortes.

DFL-005 identifica los seis campos opacos de `Domain` y `Agent.query_engine`, pero no enumera `Domain.parameters`. La realización actual tampoco impone a esa lista no vaciedad, unicidad ni una ligadura con `CaptureSpec.parameter_id`. El registro de deuda deberá ampliar DFL-005 o constituir una deuda sucesora, según el dictamen normativo de N0-06.

La identidad de los diagnósticos relativos a `Codomain` tampoco está reconciliada entre la tabla normativa heredada y las realizaciones vigentes. N0 registra el desacuerdo bajo DFL-001; el acto N0-01 deberá fijar una sola identidad observable antes de congelar nuevos oráculos.

### 3.5 Adversarial posterior a N0-01 y clasificación de sus hallazgos

Una revisión ejecutada sobre la cabeza `a4f00b96809e48ecc1c4b01da17bf9ef24400cdf` confirmó el cierre material de N0-01 y ejerció tres sondas hermanas. Las tres fueron admitidas por Python y Rust; esa admisión es anterior a N0-01 y no fue introducida por su parche.

| Sonda | Hecho de realización | Estatuto y destino |
|---|---|---|
| `Ternarizer` con las tres particiones nombradas igual | ambos validadores sólo comprueban que las cadenas no estén vacías | `E107 — InvalidTernarizerPartition` ya constaba como `PARCIAL`: no están acreditadas cobertura, disjunción ni ausencia de solapamiento. Es deuda prioritaria antes de acreditar cualquier productor material de `Tri.U`; no es extensión de N0-01. |
| `CoupledSpec` con `bridges: [3, 3]` | se comprueba el rango de cada posición, no su unicidad | la IR heredada declara `BridgeSet` y J1.2 exige un subconjunto. Es candidato intrínseco para un microcierre separado; N0-01 no autoriza deduplicación ni reparación silenciosa. |
| `Horizon` con `events: [E, E]` | no existe comprobación de repetición | el campo se expresa como lista `[EventType]`, aunque la notación asociada usa un conjunto. Se registra como ambigüedad normativa: no se declara defecto ni se parchea hasta fijar si la multiplicidad tiene significado. |

La revisión confirmó además que Rust agrupa hoy los fallos de bienformación bajo `CompileError::InvalidProgram(String)` y que la identidad `E004` se comprueba como contenido textual. Python dispone de una definición diagnóstica estructurada. La diferencia es deuda del contrato diagnóstico bajo DFL-001; no invalida la equivalencia observable probada para N0-01 y no se corrige dentro de este acto.

## 4. Frontera arquitectónica preservada

### 4.1 Núcleo, contrato, perfil, instancia e infraestructura

| Estrato | Contenido | Regla de inclusión |
|---|---|---|
| núcleo | tipos, referencias, bienformación, determinismo, serialización y operaciones formalmente constituidas | debe conservar sentido al borrar todo vocabulario de dominio |
| contrato de dominio | identidad, versión, perímetro, suficiencia, pérdidas y obligaciones de acoplamiento | expresa cómo entra un dominio; no contiene su ontología material |
| perfil de dominio | significado, parámetros, reglas, fuentes y operaciones propias de un dominio | queda fuera de la semántica universal salvo invariante transversal demostrado |
| instancia | programa, caso, datos y ejecución identificados contra un perfil y un corte | no crea tipos universales por su mera existencia |
| infraestructura | carga, almacenamiento, adaptadores, ejecución material, recuperación y despliegue | no redefine la semántica del núcleo |

La clasificación es un criterio de trabajo. No crea un objeto llamado `Meta-IR`.

### 4.2 Perfiles fuente y perfiles de dominio

`SVP-ES` y `SVP-EN` convergen a una identidad canónica común y, por definición, no introducen semántica adicional en la IR. Un perfil de dominio aporta semántica material propia. No existe, por tanto, identidad categorial entre ambos.

Sí se trasladan, como disciplina de registro cerrado:

- selección explícita y nunca inferida;
- identidad y versión;
- ausencia de autodetección y de caída silenciosa;
- aislamiento entre perfiles;
- conservación de procedencia y huellas;
- ampliación futura sólo por constitución expresa;
- pruebas de unicidad, colisión y rechazo de valores ajenos.

### 4.3 Dominio y agente

Un dominio define íntegramente su perímetro relativo al corte declarado. Un agente es una realización posterior que puede consumir todo el dominio o una cobertura explícita y tipada de éste.

La IR vigente no representa esa cobertura parcial: `Agent` referencia un solo `Domain` y no dispone de `AgentCapability`, proyección de subdominio ni lista de dominios. Esta carencia no autoriza a identificar dominio y agente, ni a afirmar que todo agente deba recorrer todo el dominio.

## 5. Radiografía vinculante de siete familias

### N0-01 — `Codomain` no impone unicidad de miembros

**Tipo vigente:** `Codomain { values: Vec<String> }`.

**Realización vigente:** Python y Rust sólo rechazan el codominio vacío. Una secuencia con un mismo miembro repetido conserva dos posiciones sintácticas para una identidad que normativamente actúa como conjunto.

**Clasificación:** `CIERRE_INTRINSECO_INMEDIATO`.

**Invariante candidato:**

```text
values != []
card(values) = card(set(values))
```

**Oráculo negativo mínimo:** un `codomain` con un miembro repetido debe rechazarse en referencia Python y Rust antes del lowering observable.

### N0-02 — `OutputSemantics` no está cerrada contra el codominio de `CellSpec`

**Tipo vigente:** `OutputSemantics { mappings: Vec<(String, String)> }` no contiene referencia propia a un `Codomain`. La relación efectiva nace en `CellSpec(codomain, semantics)`.

**Realización vigente:** ambos validadores comprueban que `CellSpec` refiere objetos existentes y del tipo esperado, pero no comparan las claves de `mappings` con los miembros de su `codomain`. Tampoco rechazan claves repetidas.

**Clasificación:** `CIERRE_INTRINSECO_INMEDIATO`.

**Invariante candidato, formulado donde la relación existe:** para todo `CellSpec C`, cada miembro de `C.codomain` aparece exactamente una vez como clave de `C.semantics`, y no aparece ninguna clave ajena.

```text
keys_unicas(C.semantics.mappings) = set(C.codomain.values)
```

Esto no exige que dos textos descriptivos distintos no puedan coincidir. La unicidad exigida es una interpretación por símbolo, no una restricción léxica injustificada sobre el texto de interpretación.

**Oráculos negativos mínimos:** semántica vacía sobre codominio no vacío; clave ausente; clave ajena; clave repetida.

### N0-03 — La admisión de claves homónimas destruye la proyección JSON

**Realización vigente:** Python convierte los pares de `OutputSemantics` en un mapa y una clave repetida pierde silenciosamente una entrada. Rust emite el mapa mediante composición textual y puede producir dos miembros JSON con el mismo nombre. La proyección admitida deja de ser un punto fijo fiable de `parse(serialize(x))`.

**Clasificación:** `CIERRE_INTRINSECO_INMEDIATO`, derivado de N0-02.

**Invariante candidato:** ningún programa admitido puede producir nombres homónimos dentro de un mismo objeto JSON.

**Oráculo positivo mínimo:** para todo programa admitido del corpus ampliado, la proyección observable debe sobrevivir a `parse → serialize` sin pérdida de miembros ni cambio semántico.

El cierre no convierte `equivalence_json` en serializador canónico completo. Su comentario de alcance debe conservarse mientras esa función siga siendo una proyección diferencial.

### N0-04 — `Horizon.architecture` no denota necesariamente un `CompositionGraph`

**Tipo vigente:** `Horizon { architecture: String, events: Vec<String> }`; la IR normativa tipa `architecture` como `CompositionGraph`.

**Realización vigente:** Python y Rust sólo exigen que la cadena no esté vacía. `Frame.architecture` sí se resuelve como referencia. `Agent` compara su cadena con `Horizon.architecture`, pero la igualdad entre dos cadenas no acredita existencia.

**Clasificación:** `CIERRE_INTRINSECO_INMEDIATO`.

**Invariante candidato:** todo `Horizon.architecture` debe resolver un `CompositionGraph` declarado. La coherencia de `Agent.architecture` debe ser coherencia con esa referencia real, no sólo igualdad textual.

**Oráculos negativos mínimos:** horizonte con arquitectura inexistente; horizonte con referencia existente de tipo distinto; agente cuya arquitectura no coincide con la arquitectura real de su dominio.

### N0-05 — Estatuto no decidido de las unidades fuente vacías en ensamblaje

**Realización vigente:** `compile_svp_assembly` exige al menos dos `SourceUnit`, compila cada unidad por separado y valida globalmente el programa reunido. No exige que cada unidad produzca al menos un objeto u operación.

**Hecho:** el ensamblador puede recibir unidades superficial o semánticamente vacías. No está determinado si una unidad vacía es una entrada legítima, una guarda de interfaz o un programa mal formado.

**Clasificación:** `DETERMINACION_DE_FRONTERA`.

**Razón para no parchear ahora:** rechazarla sin especificación convertiría una preferencia de interfaz en semántica del Lenguaje; admitirla por omisión convertiría silencio en contrato.

**Siguiente prueba:** conservar ensamblajes con cero, una y dos unidades vacías como sondas no normativas hasta que dos contratos de dominio permitan decidir si la unidad vacía conserva alguna identidad legítima.

### N0-06 — El perímetro de `Domain` y la cobertura de `Agent` no están constituidos por completo

**Tipo vigente de `Domain`:** lista `parameters`, cadenas de captura/admisibilidad/ternarización y campos `interface`, `exogeneity_mask`, `silent_u`, `transduction_policy`, `u_policy` y `closure_criterion`. No existe campo de versión.

**Validación real:**

- `horizon`, `capture_specs`, `admissibility_specs` y `ternarizers` se resuelven y se tipan;
- se exige igualdad entre los conjuntos de `parameter_id` de captura y admisibilidad;
- se exige que cada espacio de captura tenga ternarizador;
- `parameters` puede estar vacío, contener duplicados y no mantiene una ligadura tipada con los `parameter_id`;
- los seis campos opacos de `Domain` y `Agent.query_engine` no tienen interpretación ejecutiva completa, conforme a DFL-005.

**Clasificación dividida:**

1. no vaciedad, unicidad de `parameters` y detección de `parameter_id` duplicados son `DETERMINACION_NORMATIVA_PREVIA_AL_CIERRE_INTRINSECO`;
2. la correspondencia entre un nombre de `parameters` y un `parameter_id` no es expresable hoy sin declarar una relación nueva; no puede inventarse mediante posición, convención de nombre o igualdad de cardinalidades;
3. versión, significado de campos opacos, cobertura parcial y consumo multidominio pertenecen al contrato y al contraste posterior.

**Corrección a una lectura excesiva:** no se da por implementable una «correspondencia con `capture_specs`» que la IR actual no sabe representar. Primero se determinará si requiere un campo, una referencia tipada o un manifiesto externo enlazado por identidad y hash.

**Sondas previas:** `parameters = []`; nombre repetido; dos `CaptureSpec` con el mismo `parameter_id`; dos `AdmissibilitySpec` con el mismo `parameter_id`; igualdad de conjuntos obtenida ocultando duplicados; cardinalidades incompatibles.

### N0-07 — Procedencia, orden, nombres y alcance del ensamblaje

**Realización vigente:** cada `SourceUnit` conserva fuente, nombre y perfil durante el análisis. La identidad agregada incluye unidades en orden. El `IrProgram` ensamblado conserva una identidad de conjunto y un nombre sintético, pero los objetos reunidos no conservan individualmente la unidad de procedencia.

**Consecuencias observables:**

- invertir dos unidades cambia el hash y los bytes;
- los identificadores viven en un espacio global y los homónimos se rechazan;
- no existe resolución por dominio ni calificación de nombres;
- `compose` opera sobre `CompositionGraph`; no acepta `Domain` ni `Agent`;
- el ensamblaje multifuente y `compose` son operaciones distintas y ninguna constituye un superagente.

**Clasificación:** `DETERMINACION_DE_FRONTERA`.

La sensibilidad al orden no se declara defecto por sí misma: puede ser canónica si el orden forma parte de la identidad. Tampoco se declara correcta por omisión. Con los contratos IMM y CYB sobre la mesa deberá decidirse:

1. si el orden es semántico, registral o irrelevante;
2. qué procedencia debe sobrevivir por objeto y operación;
3. si los nombres permanecen globales, se califican por dominio o se resuelven mediante importación explícita;
4. qué forma canónica y qué política de colisión rigen;
5. si la composición de agentes se constituye después como operación nueva o queda expresamente fuera del núcleo cerrado.

## 6. Inventario de oráculos

### 6.1 Oráculos que pueden convertirse en regresión tras N0

| ID | Entrada | Resultado exigido |
|---|---|---|
| `OR-N0-01` | `Codomain` con miembro repetido | rechazo en Python y Rust |
| `OR-N0-02` | `CellSpec` con `OutputSemantics` vacía y codominio no vacío | rechazo |
| `OR-N0-03` | semántica con miembro del codominio ausente | rechazo |
| `OR-N0-04` | semántica con clave ajena al codominio | rechazo |
| `OR-N0-05` | semántica con clave repetida | rechazo antes del lowering observable |
| `OR-N0-06` | programa admitido serializado, leído y reserializado | sin pérdida ni clave homónima |
| `OR-N0-07` | `Horizon.architecture` inexistente | rechazo |
| `OR-N0-08` | `Horizon.architecture` de tipo distinto | rechazo |

Cada parche deberá incluir su recíproco válido y mantener:

```text
referencia_python = CONFORME
rust_nativo = CONFORME
wasm = CONFORME_CUANDO_LA_SUPERFICIE_AFECTADA_SE_EXPONGA
oraculos_previos = BYTE_IDENTICOS
```

El identificador diagnóstico exacto se fijará en el parche normativo correspondiente. No se reutilizará un código cuyo significado vigente sea distinto sólo para evitar actualizar el contrato diagnóstico.

### 6.2 Sondas que no son todavía oráculos normativos

| ID | Sonda | Pregunta pendiente |
|---|---|---|
| `SON-N0-01` | `Domain.parameters = []` | ¿puede existir un dominio de perímetro vacío? |
| `SON-N0-02` | parámetros nominales repetidos | ¿qué identidad y unicidad rigen? |
| `SON-N0-03` | `parameter_id` repetido en cadenas | ¿la multiplicidad es ilegal o tiene significado declarado? |
| `SON-N0-04` | lista nominal y capturas con cardinalidad distinta | ¿cómo se liga nombre, identidad numérica y captura? |
| `SON-N0-05` | dos unidades fuente vacías | ¿unidad válida o ausencia de programa? |
| `SON-N0-06` | mismas unidades en orden inverso | ¿el orden pertenece a la identidad? |
| `SON-N0-07` | homónimos locales en dos dominios | ¿espacio global, calificación o importación? |
| `SON-N0-08` | dos agentes sobre un dominio | capacidad vigente: debe permanecer admisible |
| `SON-N0-09` | un agente sobre cobertura parcial | hoy no representable; no debe simularse mediante prosa |
| `SON-N0-10` | un agente sobre dos dominios | hoy no representable; exige decisión separada |
| `SON-N0-11` | las tres particiones de `Ternarizer` con el mismo nombre | ¿qué representación permite probar J1.5 sin inventar los conjuntos observacionales? |
| `SON-N0-12` | `CoupledSpec.bridges = [3, 3]` | la semántica `BridgeSet` exige decidir y probar unicidad sin reparación silenciosa |
| `SON-N0-13` | `Horizon.events = [E, E]` | ¿la lista representa un conjunto o una secuencia con multiplicidad? |

Promover una sonda a oráculo exige una regla normativa previa. No se escribirá primero la prueba para obligar después a que la semántica adopte su presupuesto.

## 7. Orden de trabajo que resulta de N0

### K1 — Cierre intrínseco y determinaciones inmediatas

1. cerrar `Codomain` como conjunto representado sin duplicados;
2. cerrar la relación `CellSpec ↔ OutputSemantics ↔ Codomain`;
3. impedir claves JSON homónimas y probar estabilidad de la proyección;
4. resolver `Horizon.architecture` como `CompositionGraph` real;
5. cerrar en acto separado la unicidad de `CoupledSpec.bridges` como representación de `BridgeSet`, si el cotejo normativo no descubre una contradicción;
6. dictaminar el estatuto de multiplicidad de `Horizon.events` sin convertir por analogía una lista en conjunto;
7. dictaminar el mínimo estructural de `Domain.parameters` y la multiplicidad de `parameter_id`, sin inventar todavía una correspondencia no representada;
8. actualizar la deuda viva, la concordancia diagnóstica y el recuento documental de la batería afectados.

Cada punto se realizará en un commit delimitado o en una unidad indivisible justificada. No se mezclará con contenido de Inmunología o Ciberseguridad.

### K1-T — Puerta previa a la producción material de `Tri.U`

Antes de acreditar una ruta ejecutable que produzca `Tri.U` mediante `Ternarizer`, deberá cerrarse o impedirse explícitamente la deuda `E107/J1.5`: cobertura completa, disjunción y ausencia de solapamiento de `(B_0, B_1, B_U)`. La representación vigente mediante cadenas no autoriza al núcleo a inventar esos conjuntos, inferirlos desde el dominio ni aceptar en silencio que no puede comprobarlos.

Esta puerta no ordena implementar ahora el ternarizador, no modifica el álgebra y no altera la separación de competencias entre dominio, agente y Lenguaje.

### F — Contrato candidato de dominio

Después de K1 se fijará documentalmente un contrato candidato que distinga identidad, versión, perímetro, operaciones, suficiencia y pérdida. No se afirmará que el contrato sea universal ni se identificará con `PerfilFuente`.

### G/H — Retorno acotado a Inmunología

El contrato candidato se someterá a `OP-IMM-001` y a sus requisitos reconciliados. El retorno no exige completar los 32 universos inmunológicos. Debe producir el contrato IMM del corte y registrar toda pérdida sin adaptar el caso por conveniencia.

### I/J — Segundo falsador: ciberseguridad inteligente

Tras incorporar el retorno inmunológico, el dominio CYB constituirá un primer perímetro completo y al menos una operación falsadora. Si utiliza modelos de IA, cada modelo será entrada capturada, versionada y sellada; no autoridad viva invocada durante una ejecución que se pretenda reproducible.

### K2 — Decisiones de frontera

Sólo con los contratos IMM y CYB disponibles se decidirán:

- unidad fuente vacía;
- orden y forma canónica del ensamblaje;
- procedencia por objeto;
- calificación y colisión de nombres;
- ubicación de la versión del dominio;
- correspondencia nominal de parámetros cuando siga pendiente.

### M — Composición de agentes, separada del cierre de dominio

La cobertura parcial, el consumo multidominio y una eventual `COMPOSE_TYPED_AGENTS` se decidirán después y bajo custodia basal. Podrán constituirse o quedar como deuda explícita. No condicionan que un dominio se defina entero ni autorizan a llamar superagente al ensamblaje vigente.

## 8. Rust y cadena de herramientas

La diferencia actual entre un flujo que usa `stable` flotante y otros que fijan Rust `1.98.0` es una deuda de reproducibilidad de infraestructura. No exige cambiar la semántica ni la IR.

La actualización de parche, la declaración de MSRV y la unificación del régimen de toolchain deberán realizarse en un commit de higiene separado. No se mezclarán con los cierres N0-01..N0-07, para que una variación de compilador no contamine la atribución causal de los resultados.

## 9. Límites de autorización

N0 no autoriza:

- modificar gramática, IR, Python, Rust, WASM, Worker o Playground;
- crear `PerfilDominio`, `AgentCapability`, subdominio o superagente;
- integrar material clínico o de ciberseguridad en el núcleo;
- abrir R2, R3, R4, laboratorio, datos reales o despliegue;
- declarar universalidad desde dos perfiles;
- cerrar toda Inmunología antes de continuar;
- tratar una compilación verde como prueba de aptitud clínica u operacional.

## 10. Condición de salida de N0

N0 queda cumplida cuando este documento sea el único cambio de su primer commit y la rama conserve como padre exacto `736ea643d7f65ba4bf26dbbb321383b8becc8d64`.

El siguiente acto material será N0-01: especificación, realización y prueba de unicidad de `Codomain`. No se iniciará junto con esta radiografía.

## 11. Vector de continuidad

```text
BASE = 736ea643d7f65ba4bf26dbbb321383b8becc8d64
RAMA = cierre-nuclear-20260904
PRIMER_COMMIT = SOLO_N0
FAMILIAS_RADIOGRAFIADAS = 7
DERIVA_CORPUS_IR_V0_3 = 72_DECLARADOS_VS_79_VIGENTES
CIERRE_INTRINSECO_SIGUIENTE = CODOMAIN_SIN_DUPLICADOS
PERFIL_DOMINIO = CONTRATO_CANDIDATO_NO_TIPO_VIGENTE
INMUNOLOGIA = PRIMER_FALSADOR_EN_PAUSA_CONTROLADA
CIBERSEGURIDAD_INTELIGENTE = SEGUNDO_FALSADOR_DIFERIDO
DOMINIO_Y_AGENTE = SEPARADOS
ENSAMBLAJE_Y_SUPERAGENTE = NO_EQUIVALENTES
NUCLEO_CERRADO = NO
```

## 12. Recepción posterior de N0-02 · 06/09/2026

RETP-079 constituye J-K1 y realiza el cierre de cada relación `CellSpec–OutputSemantics–Codomain`, con diagnóstico E115 y corpus de 85 casos. El [acta N0-02](./ACTA_TECNICA_N0_02_TOTALIDAD_Y_UNICIDAD_DE_OUTPUT_SEMANTICS_2026_09_06.md) fija evidencia, perfiles, límites y condición de promoción. Esta radiografía conserva sus hallazgos en el corte que los originó; no describe esos defectos como recién introducidos.

Tras integrar ese expediente, la continuación es N0-03: ausencia global de miembros JSON homónimos y estabilidad de proyección. `semantics_unbound_duplicate` conserva un testigo explícito no cubierto por el vínculo de N0-02. La deuda CRLF sigue en DFL-008 y la concordancia diagnóstica general en DFL-001. No se declara cerrado el conjunto de K1.

## 13. Recepción posterior de N0-03 · 06/09/2026

RETP-080 y el [acta N0-03](./ACTA_TECNICA_N0_03_UNICIDAD_DE_MIEMBROS_Y_ESTABILIDAD_DE_PROYECCION_JSON_2026_09_06.md) constituyen J-J0, completan la unicidad de todas las semánticas y comprueban estabilidad de la proyección admitida. El corpus pasa a 88 = 14 válidos + 74 inválidos; los emisores y sus versiones conservan su realización. E115 recibe explícitamente la comprobación no enlazada y la precisión de su texto general Python. La promoción permanece ligada a la candidata y a sus controles.

Después sigue N0-04: referencia real de `Horizon.architecture` a `CompositionGraph` y coherencia relacional pertinente. Esta recepción no decide N0-05 ni salta las obligaciones intrínsecas restantes, K1-T o las deudas DFL-001/008. La radiografía inicial conserva sus hallazgos históricos.

## 14. Recepción posterior de N0-04 · 07/09/2026

RETP-081 y el [acta N0-04](./ACTA_TECNICA_N0_04_REFERENCIA_REAL_DE_ARQUITECTURA_DEL_HORIZONTE_2026_09_07.md) realizan J-H0. Se precisa la frase abreviada de la radiografía: IR v0.2 declara `ArchitectureId` en Horizon, no literalmente `CompositionGraph`; este último es el referente ahora resuelto y el tipo declarado para Agent. Los antecedentes permanecen identificados. La obligación afecta a todos los horizontes, incluso sin consumidores, sin resolver parcialmente unidades fuente.

El corpus pasa a 91 = 14 válidos + 77 inválidos. Se documenta la corrección de un positivo histórico cuya arquitectura no estaba declarada; no se afirma que sus bytes anteriores sigan admitidos. Tras la promoción sigue la unicidad de `CoupledSpec.bridges` conforme a §7, luego las determinaciones de Horizon.events y Domain. N0-05/N0-07 permanecen en K2; DFL-001/008 y K1-T conservan sus deudas.

<a id="cierre-bridgeset-20260907"></a>
## 15. Cierre incremental de unicidad de BridgeSet · 07/09/2026

**RETP-2026-083. Entrada:** main `a09b9efef51f88de29048b9b35e7ac085dc0918f`, PR #68, y retirada Python RETP-082 conservada en commit separado. Se reciben los Pilares RETP-073, perfiles RETP-075, transición §§12–23 y la obligación §7/K1.1–5 de esta radiografía.

La sonda SON-N0-12 se convierte en juicio J-B0, [IR 0.3 §6.5](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#bridgeset-j-b0), por cotejo de la definición `BridgeSet` y J1.2 de la IR heredada. No se encontró contradicción entre su carácter de subconjunto y el límite explícito del conjunto vacío. La lista superficial conserva el orden de las posiciones válidas; una repetición se rechaza sin deduplicación. No se generaliza esta decisión a `Horizon.events`.

| Comprobación | Evidencia y límite |
|---|---|
| Contraejemplo antes/después | La fuente nueva `coupledspec_puente_repetido.svp`, con `[3,3]`, era admitida por el binario nativo del corte a09b9ef. La candidata la rechaza antes de admitir IR, identificando `CC` y la posición `3`. |
| Juicio nativo | Tres pruebas de integración desde fuentes SV: repetición/identidad Nat, conjunto vacío/orden y precedencia del rango. No se modifica el emisor ni otra familia de objetos. |
| Perfiles y ensamblaje | 18 testigos sintéticos en `tests/k1_bridge_cases.py`: ES/EN, vacío, orden `[9,1,3]`, repetición, `03/3`, rango y ensamblaje mixto en ambas orientaciones y órdenes. El auxiliar `assembly_probe` sólo transporta fuentes a la API pública existente; no amplía la CLI productiva. |
| Oráculo independiente | Las posiciones admitidas y el diagnóstico de rechazo están prescritos por cada testigo. La comparación nativo/WASM se efectúa después de esa comprobación. El corpus pasa a 92 = 14 válidos + 78 inválidos; los 14 esperados y 91 fuentes anteriores permanecen intactos. |
| Destinos | Esos mismos 18 testigos se exigen en nativo, WASI y navegador real; la batería histórica y las cinco sondas de sensibilidad se conservan. La emisión nativa previa de los 14 positivos y 77 rechazos se compara literalmente con la candidata. |
| PT01/PT02/PT04/PT13/PT14 | Identidad de fuente/candidata, perfiles y límites diagnósticos; corpus, comandos y artefactos de los cuatro flujos. Laboratorio 016/018 reutilizado dentro de su alcance, sin nueva plataforma ni aumento retrospectivo de pruebas. |

**Promoción:** cierre limitado efectivo tras verificar los cuatro flujos sobre la candidata exacta e integrarla. Los identificadores de los flujos se mantienen; los artefactos incluyen salidas y huellas de los ejecutables utilizados. La PR identifica la cabeza, su base y los resultados efectivos. Generar un manifiesto local no acredita ejecución WASM.

**Relevo:** sigue K1 por la decisión sobre multiplicidad de `Horizon.events`, después el mínimo de `Domain` y las condiciones de K1-T conforme a §7 y transición §14. DFL-010 conserva la pérdida de campos opcionales antes de F. DFL-009 se recibe en el retorno del primer universo CYB (fila 9). N0-05/N0-07 conservan K2. No se abre F ni se cierra núcleo, álgebra o R2/R3/R4.

<a id="horizon-events-20260907"></a>
## 16. Dictamen de multiplicidad de Horizon.events y cierre J-H1 · 07/09/2026

**RETP-2026-084. Entrada:** main `16232b63438a6bc7ef7a8a6d9efc202d89c86679`, PR #69 integrada. Se reciben completos los Pilares RETP-073, el acta de perfiles RETP-075, esta radiografía y la transición con relevo §24. Se cotejan IR v0.2/v0.3, gramática 0.2 con la producción heredada de §5.5, los juicios y consumidores actuales. Continúa la fila 3/K1, punto 6 de §7.

### 16.1. Cotejo y decisión de representación

| Fuente | Alcance que fundamenta la decisión |
|---|---|
| [Documento III (DOI)](https://doi.org/10.21428/39829d0b.bb86c65d), DOI 10.21428/39829d0b.bb86c65d; §§3.2–3.5, 4.1 y 9 | ℋ(𝒜) enumera tipos relevantes declarados desde el dominio. Las instancias pertenecen al dato de transición. Una colección de episodios observados no define el horizonte. Su axiomática general permanece abierta. |
| [IR v0.2 en el corte recibido](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/16232b63438a6bc7ef7a8a6d9efc202d89c86679/IR_CANONICA_BIENFORMACION_SV_v0_2.md), nivel 3 y J4.3 | `Horizon.events` contiene EventType; `TransitionData.events` contiene pares EventType/Tri. J4.3 comprueba pertenencia al horizonte. |
| Gramática 0.2 §1 y producción heredada `horizon_decl`, §5.5 de 0.1 | La superficie usa lista de identificadores; esa elección sintáctica no decide por sí sola una semántica de episodios ni su multiplicidad. |
| `wellformed.rs`, consumidor TransitionData | La comprobación de pertenencia usa un conjunto de nombres del horizonte. Es coherente con la distinción documental, pero el comportamiento del código no es su fundamento normativo. |

**Dictamen:** la multiplicidad de un nombre dentro de `Horizon.events` no representa ocurrencias diferentes. J-H1 exige una sola declaración por identidad y horizonte. Se rechaza una repetición sin deduplicar ni ordenar. Tipos con nombres distintos no se fusionan por parecido; el dominio conserva su constitución. Se resuelve SON-N0-13 por cotejo propio, sin trasladar por analogía el dictamen de BridgeSet.

No se altera `TransitionData` ni se regula por este acto la multiplicidad de pares del mismo tipo dentro de un único dato. La recurrencia en datos distintos se conserva; no acredita ejecución material del operador inducido ni causalidad completa de una trayectoria. El rechazo previo de horizonte vacío permanece y no se presenta como teorema deducido de su carácter de conjunto.

### 16.2. Realización, testigos y conservación

J-H1 se añade después de los controles existentes, incluido el recorrido completo de referencias J-H0. El núcleo rechaza antes de admitir IR y diagnostica el horizonte y el tipo repetido. Se preservan la precedencia de referencia ausente/tipo incorrecto, vacío y pertenencia de TransitionData. No cambia parser, emisor, versiones, esquema ni API productiva. El compilador Python continúa retirado.

El nuevo negativo `horizon_tipo_suceso_repetido.svp` contiene `[B,A,B]`: la base lo admitía y la candidata lo rechaza. El corpus es **93 = 14 válidos + 79 inválidos**; los 92 casos anteriores conservan fuentes, retornos y salidas literales, y los 14 esperados permanecen intactos.

Tres pruebas de integración comprueban repetición, identidad local/orden y precedencia diagnóstica. El banco `tests/k1_horizon_cases.py` contiene **20 testigos**: ES/EN, orden, repetición no adyacente, vacío, nombres distintos, dos horizontes que comparten tipos, dos datos de transición que instancian el mismo tipo y ensamblaje mixto en ambos sentidos y órdenes. Sus expectativas de campos y diagnósticos se prescriben antes de comparar destinos. Reutiliza el transporte de testigos y `assembly_probe`, sin crear otra implementación de SV.

El control anterior de N0-04 conservaba `[B,A,B]` sin fijar su estatuto. En `horizon_architecture.rs` se cambia explícitamente a `[B,A]` para seguir comprobando orden y referencia; el historial conserva el original y la nueva prueba recibe su rechazo. No se presenta esa modificación como conservación literal de aquel testigo ni se reescribe su acta histórica.

Se exigen los mismos 20 testigos en **nativo, WASI y navegador real**, junto con los 18 de BridgeSet, las cinco sondas de sensibilidad, el corpus y las regresiones previas. Los cuatro trabajos CI conservan identidad. PT01/PT02/PT04/PT13/PT14 reciben fuentes, perfiles, diagnósticos, comandos y artefactos de la candidata exacta. Los registros 016/018 del laboratorio mantienen su alcance histórico; no se promueve otra plataforma.

**Promoción:** cierre limitado efectivo tras los cuatro flujos correctos e integración. El expediente de la PR asociada a `k1-horizon-events-20260907` identifica cabeza, base y árbol probado; generar un manifiesto no acredita ejecución WASM.

**Relevo:** sigue K1 por el mínimo estructural de `Domain.parameters` y la multiplicidad de `parameter_id`, según §7 y transición §14. DFL-010, concordancia diagnóstica y K1-T conservan sus obligaciones. F permanece pendiente; DFL-009 se reevalúa en la fila 9, al retornar del primer universo CYB. No se consolidan núcleo, dominio, álgebra ni R2/R3/R4.

<a id="reconciliacion-documento-iii-20260907"></a>
### 16.3. DOI del Documento III y recepción de su actualización · 07/09/2026

**RETP-2026-085. Corte del Lenguaje:** `02cd27d6c056897949acd39ef555bc041d0781ff`, PR #70 integrada. Se reciben los Pilares RETP-073, perfiles RETP-075, secuencia §25, frontera e IR aplicables. Por precisión humana se sustituye la dirección de una release concreta por el [DOI estable del Documento III](https://doi.org/10.21428/39829d0b.bb86c65d), en §16.1, IR v0.3 §6.6 y las referencias heredadas de IR v0.2 y Frontera v0. RETP-084 conserva el dato histórico de la release citada originalmente; este asiento corrige su recepción sin atribuirle retroactivamente otra consulta.

**Texto efectivamente cotejado.** El [repositorio del autor en `b8fd32978292d25adf9b87cf71e409005dce642c`](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/III_horizonte_sucesos_reevaluacion_discreta.md), blob `92caea3f63ace2f494c4871f69680fd38ab3ee77`, identifica la Release 2. Se ha leído completo y comparado con el [antecedente Git de Release 1](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/1afa2ab4159a66411f2a01e8965ca3e9554d42e6/documentos/composicion/III_horizonte_sucesos_reevaluacion_discreta.md), blob `e6cb8f4b12d8ef31647f750dd5e79c599272515d`. El [cambio del autor de 15/03/2026](https://github.com/juantoniolloretegea/SV-matematica-semantica/commit/2d8b02ef262da3253e79742064156fc13f947ff6) permite inspeccionar las diferencias.

El cotejo completo identifica: adición del enlace a la demostración audiovisual; retirada del enlace antiguo; ajustes de presentación; cambio de orden expositivo en §3.1, sin cambiar el referente 𝒜; y seis sustituciones de notación del operador inducido, de 𝒯 a 𝒰. Aisladas esas diferencias y el espacio de presentación, el texto restante coincide palabra por palabra. La [página auxiliar del autor](https://juantoniolloretegea.github.io/SV-matematica-semantica/documento-iii/) identifica también Release 2 y declara el vídeo como apoyo explicativo subordinado al documento.

| Obligación examinada | Resultado del cotejo e impacto en el Lenguaje |
|---|---|
| §§3.2–3.5 y 4.1: tipos declarados frente a instancias | Se conserva la definición de ℋ(𝒜) y la separación de νₙ. J-H1 mantiene su fundamento; el dominio declara relevancia y el Lenguaje preserva identidad, orden explícito y rechazo de repeticiones. |
| §§3.6 y 5.2: operador inducido y suficiencia constitutiva | Se conserva la condición de suficiencia. Se corrige en Frontera §3 la correspondencia `InducedTransitionOp` → 𝒰_{νₙ}; 𝒯 sigue designando la tabla de admisibilidad. No se añade una operación ni se acredita ejecución de trayectorias. |
| §§1.4, 3.4, 3.7, 5 y 6: tiempo, frame y trayectoria | Permanecen ordinalidad, inmutabilidad, crecimiento sin reescritura, reproducibilidad condicionada y distinción de regímenes estricto/auditable. J-H1 no demuestra esas garantías materiales ni convierte su lista en cronología. |
| §3.8: cambio de criticidad | La relación δ_Γ sigue sin presuponer resta numérica. No se introduce conversión ni cálculo nuevo. |
| §8: frontera con transducción | Se conserva la separación con el Documento IV. K1-T mantiene sus condiciones; el cierre de Horizon no acredita Ternarizer. |
| §9 y cuestiones locales pendientes | Siguen fuera del cierre doctrinal la axiomática general de horizontes, convergencia, mérito interdominio y terminación de ciclos. La actualización no resuelve por sí sola horizonte vacío ni multiplicidad de pares dentro de un único TransitionData. Tampoco convierte las deudas de realización en garantías satisfechas. |

**Dictamen y límite de evidencia:** en el texto identificado del autor no se detecta una obligación normativa nueva omitida por J-H1 ni una cuestión que la actualización haya cerrado y aquí se mantuviera indebidamente abierta. Sí se subsanan la referencia obsoleta y la correspondencia notacional heredada. La consulta directa de la página editorial, tanto desde el DOI como de Release 2, devolvió HTTP 403; este cotejo acredita el contenido del repositorio identificado, no identidad literal con la página editorial servida hoy ni ausencia de una edición posterior no reflejada allí. El DOI queda como puerta estable y el commit como identidad de la evidencia consultada.

**Verificación y relevo:** cambio documental; código, gramática ejecutable, esquema, corpus, esperados y workflows conservan los blobs de la PR #70. Se revisan cuatro sustituciones de URL, notación, enlaces locales y concordancia RETP, y se exige el flujo Conformidad SVP sobre esta candidata. La paridad nativo/WASI/navegador de PR #70 conserva su corte y no se vuelve a contabilizar como una prueba nueva. Continúa K1 por `Domain.parameters` y `parameter_id`; F, DFL-001/005/010 y K1-T mantienen sus puertas. DFL-009 sigue diferida al retorno del primer universo CYB (fila 9).

<a id="domain-parameters-20260907"></a>
## 17. Dictamen K1 sobre Domain: unicidad nominal y ligadura aún no representada · 07/09/2026

**RETP-086; entrada:** main `4536bd051cc58bf183b8b9efa5a5f818f18090f9`, PR #71 integrada. Leídos íntegramente AGENTS, Pilares RETP-073, perfiles RETP-075 y transición con su §25; también N0-06/§7, IR v0.2 nivel 4, IR v0.3, Gramática v0.1 §5.6 conservada por v0.2, contrato mínimo de interfaces y DFL-005. El expediente continúa la fila 3/K1. El acceso editorial queda aplazado por decisión humana; no se declara recuperada otra edición web.

### 17.1. Fuente y resultado del cotejo

El texto del autor en `SV-matematica-semantica@b8fd32978292d25adf9b87cf71e409005dce642c` identifica:

- [Documento V §5.1](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/V_invariantes_agentes_operador_consulta.md), blob `d609810b4072a4b61e29ef10f4d26751862b0c86`: 𝒫 es un conjunto de instancias `(C,j)`, no de meros números `j`.
- [Documento IV §§4.2–4.3 y 6.1–6.4](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/composicion/IV_transduccion_alfabeto_ternario_interfaz_parametrica.md), blob `66ee444aa72c216267750b679378a22f0ad7cc8a`: captura por parámetro exógeno e interfaz de instancias con célula. El cotejo aquí se limita a identidad y exposición; no reactiva las formulaciones históricas de fallo→U sustituidas expresamente por IR v0.3 §2 y los Pilares §1.5.

| Pregunta de N0-06 | Dictamen y sede |
|---|---|
| Nombres repetidos en `parameters` | J-D0 exige unicidad nominal local y preservación de orden. Repetir el mismo identificador no declara una segunda instancia distinta. El Lenguaje puede rechazarlo sin inventar su correspondencia. |
| `parameters = []` | El conjunto doctrinal no tiene aquí cláusula expresa de no vaciedad. No se deduce un rechazo de `b≥3` ni del número de capturas. Su admisión estructural previa se conserva como límite; F deberá decidir el mínimo del contrato operacional de dominio. |
| `parameter_id` repetido | La instancia exige `(C,j)`. Por ejemplo, `(C1,1)` y `(C2,1)` son distintas aunque repitan el numeral; la IR no enlaza ese numeral con `C`. Tampoco representa un selector de capturas alternativas para una misma instancia. Prohibir toda repetición o declararla unívocamente válida sería asumir la relación ausente. F debe fijarla antes de usarla. |
| Igualdad de conjuntos de captura/admisibilidad | Se conserva la comprobación existente, pero no demuestra multiplicidad, correspondencia entre objetos ni cobertura por instancia. Las listas originales siguen en IR/proyección; no se deduplican. |
| Cardinalidad nominal distinta de la cadena | No se exige igualdad: una cardinalidad coincidente tampoco reconstruye las identidades. Se preserva la entrada y se mantiene la falta de ligadura en DFL-005. |

Los Pilares §1.4 impiden presumir inyectividad, sobreyectividad o biyectividad de una asignación. J-D0 no las establece: nombres diferentes pueden seguir careciendo de referente, o requerir un contrato que detecte alias de una misma instancia. **Este cierre no acredita una constitución completa de Domain ni resuelve N0-06 entero.** F recibe estas insuficiencias representacionales de forma explícita; K2 conserva versión, procedencia y nombres en su alcance posterior.

### 17.2. Realización y evidencia exigida

La [IR v0.3 §6.7](../../IR_CANONICA_BIENFORMACION_SV_v0_3.md#domain-parameters-j-d0) fija J-D0 antes del parche. `wellformed.rs` añade una comprobación global posterior a los juicios existentes; no modifica parser, tipos, serializador, Nat, primitivas, álgebra ni datos de dominio. El rechazo es textual y controlado: `Domain D: parámetro nominal repetido: B`, sin nuevo código catalogado y sin IR ni U como sustituto.

- Conformidad: **94 = 14 válidos + 80 inválidos**, con el único nuevo negativo `domain_parametro_nominal_repetido.svp`. Sus bytes eran admitidos por la base y son rechazados por la candidata.
- Los **93 observables anteriores** conservan literalmente retorno, stdout y stderr frente al binario construido desde la base; los **14 esperados comprometidos** no cambian.
- Tres pruebas de integración Rust ejercen dominio sin consumidor, identidad exacta/orden y precedencia de rechazos previos. La regresión nativa completa conserva R0/R1 en su alcance.
- [Banco de 24 testigos](../../tests/k1_domain_cases.py): ocho variantes por perfil y ocho ensamblajes mixtos en ambos órdenes. Catorce ejercen J-D0 o guardas anteriores; diez preservan representaciones con ligadura pendiente (vacío, cardinalidad y multiplicidad numérica), sin certificarlas como contratos completos. El transporte existente verifica retorno, ausencia de IR en rechazo, diagnóstico exacto, campos y orden; compara bytes nativo/WASI y carga la misma fuente en el módulo de navegador.
- Se exigen los cuatro flujos sobre la candidata exacta: Conformidad SVP, R0 Rust, R0-8 y R0 WASM. WASI y navegador deben ejecutar el corpus completo, los 24 testigos nuevos y los bancos previos de BridgeSet/Horizon/sensibilidad. La paridad entre realizaciones sólo complementa el juicio contra la DSL y sus esperados.

**Perfiles y entorno:** SVP-ES/SVP-EN; Gramática 0.2, IR 0.3 y proyección 0.1.0 conservan versión. PT01/PT02/PT04/PT13/PT14 reciben identidad, preservación, diagnóstico, paridad y entorno. Local: Rust/Cargo 1.98.0, destino `x86_64-unknown-linux-gnu`; las ejecuciones CI registran sus compiladores y anfitriones propios. El código Python del banco sólo prepara fuentes y observa procesos, sin compilador SV ni autoridad semántica. No interviene contenido IMM/CYB ni se promueve otra plataforma. Se reutilizan los observadores y el alcance de laboratorio 016/018 sin repetir esa campaña ni extenderle estos resultados.

### 17.3. Relevo

El cierre nominal es efectivo tras la integración de la candidata verificada. Sigue la fila 3: concordancia diagnóstica/deuda/corpus, DFL-010 y delimitación o cierre de K1-T. F permanece pendiente de esa salida. Al abrir F, el contrato debe resolver el mínimo, la identidad de instancia y las ligaduras/multiplicidad aquí localizadas antes de admitir operaciones que las necesiten; no podrán darse por resueltas mediante cadenas opacas. DFL-009 continúa diferida al retorno del primer universo CYB, fila 9. No se cierra dominio, álgebra, núcleo ni R2/R3/R4.
