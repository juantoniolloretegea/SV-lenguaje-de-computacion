# N0 — Radiografía de objetos, invariantes y oráculos del núcleo SV

**Fecha:** 4 de septiembre de 2026  
**Repositorio:** `juantoniolloretegea/SV-lenguaje-de-computacion`  
**Rama:** `cierre-nuclear-20260904`  
**Base exacta:** `main@736ea643d7f65ba4bf26dbbb321383b8becc8d64`  
**Naturaleza:** clasificación forense y orden de cierre; acto exclusivamente documental  
**Estado:** `N0_CONSTITUIDA · NUCLEO_NO_CERRADO`  
**Efecto material sobre el Lenguaje:** ninguno  

## 0. Precisión terminológica obligatoria

La sigla `N0` de este documento identifica una **etapa de radiografía previa al cierre nuclear**. No designa ni modifica el nivel `N0 — Definición` de la IR canónica.

Este acto no constituye una nueva meta-IR, un `PerfilDominio`, un agente, un superagente, una fase de persistencia ni una semántica de dominio.

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

Promover una sonda a oráculo exige una regla normativa previa. No se escribirá primero la prueba para obligar después a que la semántica adopte su presupuesto.

## 7. Orden de trabajo que resulta de N0

### K1 — Cierre intrínseco y determinaciones inmediatas

1. cerrar `Codomain` como conjunto representado sin duplicados;
2. cerrar la relación `CellSpec ↔ OutputSemantics ↔ Codomain`;
3. impedir claves JSON homónimas y probar estabilidad de la proyección;
4. resolver `Horizon.architecture` como `CompositionGraph` real;
5. dictaminar el mínimo estructural de `Domain.parameters` y la multiplicidad de `parameter_id`, sin inventar todavía una correspondencia no representada;
6. actualizar la deuda viva, la concordancia diagnóstica y el recuento documental de la batería afectados.

Cada punto se realizará en un commit delimitado o en una unidad indivisible justificada. No se mezclará con contenido de Inmunología o Ciberseguridad.

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
