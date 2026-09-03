# Valoración técnica y encaje de OP-IMM-001 con el Lenguaje SV

**Fecha:** 3 de septiembre de 2026  
**Emisor:** unidad responsable del Lenguaje de computación SV  
**Destinatario:** dominio de Inmunología — `OP-IMM-001 / Q0 v0`  
**Naturaleza:** respuesta técnica de encaje; no modifica el Lenguaje SV, no abre R2, R3, R4 ni una garantía y no autoriza desarrollo ni uso clínico  
**Dictamen:** `ENCAJA_CON_CAMBIOS`

## 1. Dictamen ejecutivo

`OP-IMM-001` encaja en la arquitectura del Lenguaje SV, pero **no cabe de forma completa y fiel en la realización vigente ni es ejecutable hoy como herramienta clínica**.

El corte leído permite demostrar una parte estructural: perfiles fuente explícitos, conservación de los bytes fuente, IR 0.3, valores ternarios, captura, admisibilidad, ternarización, celdas `SV(9,3)`, composición declarada, autoridad y mediación intra-proceso, y equivalencia del núcleo Rust entre ejecución nativa y WebAssembly. No permite demostrar la identidad de ejecución completa exigida por `OP-IMM-001`, la trazabilidad tipada por parámetro, la pérdida de distinción, la salida clínica canónica completa, la persistencia autoritativa, la recuperación material, la cadena de confianza ni la aptitud clínica.

La integración correcta exige tres fronteras simultáneas:

1. un perfil externo, versionado y verificable del dominio y del motor para los 27 parámetros, sus fuentes, reglas, terminologías, configuraciones y resultados;
2. extensiones compatibles del Lenguaje sólo para invariantes cuya equivalencia causal se demuestre en más de un universo y que no puedan mantenerse fuera de la IR sin pérdida normativa;
3. las fases ya previstas de persistencia, confianza material e integración adversarial cuando el Director autorice su apertura.

No se recomienda modificar ahora gramática, IR, catálogo diagnóstico, serialización ni código. El siguiente paso útil es un experimento sintético de integración estrictamente no clínica, precedido por decisiones arquitectónicas expresas y por una autorización separada.

Esta secuencia condiciona la **integración técnica con el Lenguaje**, no la continuidad constitutiva del dominio. Inmunología puede seguir abriendo y agotando universos, raíces, parámetros, reglas, fuentes y consecuencias conforme a su propio método, sin cuota prefijada y sin esperar a R2. Cada universo conserva constitución, identidad y prueba propias: no existe herencia automática de requisitos, estados, reglas ni soluciones desde `OP-IMM-001`.

## 2. Cortes, método y límites

### 2.1 Cortes exactos

| Sede | Corte leído | Función en esta valoración |
|---|---|---|
| `SV-lenguaje-de-computacion` | `main@3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d` | especificación, realización, pruebas, fases y deuda del Lenguaje |
| `SVperitus-dataset` | `dominio-inmunologia@3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d` | solicitud, expediente clínico, cadena G6–G10 y marco técnico de `OP-IMM-001` |

El corte del Lenguaje es descendiente de `8f485c64495bf4825d2202195f1bd43dccaf8a49`, cierre registral H1–H6 y estabilización del Playground. La única modificación posterior hasta `3c122d1f…` afecta al inventario de publicaciones; no modifica Rust, WebAssembly, gramática, IR, perfiles, ejemplos ni interfaz ejecutable.

### 2.2 Método

La valoración se basa en:

- inventario completo de ambos árboles en los cortes declarados;
- lectura de la solicitud y de su cadena clínica y técnica vinculante;
- contraste con especificaciones, contratos, actas de cierre, código Rust, referencia Python, pruebas y deuda viva del Lenguaje;
- separación estricta entre capacidad especificada, capacidad realizada, capacidad probada y responsabilidad externa.

No se deduce capacidad desde el nombre de una fase ni desde una intención futura. `R2` figura abierta documentalmente, pero **no ha sido abierta ni ejecutada por esta valoración**. `R3`, `R4` y las Garantías I y II continúan sin demostración.

### 2.3 Invariante de célula

La célula mínima del Lenguaje SV es **`SV(9,3)`**: nueve posiciones sobre base ternaria. No existen células SV de cardinalidad 1, 2, 3 o 6 y queda prohibido fabricarlas, simularlas mediante relleno, duplicar parámetros para alcanzar nueve o mezclar agrupaciones clínicas distintas en una célula.

Las cardinalidades `(6,1,3,2,6,9)` de G6 son cardinalidades de **agrupaciones clínicas externas**, no tamaños de célula. En el corte actual, sólo `M-MODIFIER-001`, con nueve parámetros, puede someterse a evaluación como candidato a una posible proyección sobre una célula `SV(9,3)`. Las otras cinco agrupaciones deben permanecer en un esquema de dominio externo hasta que una decisión arquitectónica sustentada por contraste entre universos determine una representación legítima.

La cardinalidad nueve es necesaria, pero no suficiente. `M-MODIFIER-001` no queda constituida como célula por esta valoración: antes tendrían que demostrarse la correspondencia exacta de sus nueve posiciones, la partición ternaria disjunta y exhaustiva de cada observable admitido, el codominio, la semántica de salida, la operación y la suficiencia relativa a esa operación.

## 3. Índice probatorio

Los códigos de localización empleados en la matriz remiten a documentos o realizaciones fijados por commit.

### 3.1 Inmunología

- **IMM-REQ:** [solicitud de valoración](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/marco-tecnico-de-universos-subdominios-y-modulos/01-op-imm-001-informacion-preinmunosupresion-adultos/Solicitud_de_valoracion_y_encaje_tecnico_de_OP-IMM-001_con_el_Lenguaje_SV_2026-09-03.md).
- **IMM-TECH:** [marco técnico de responsabilidad, trazabilidad, reproducibilidad y criticidad](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/marco-tecnico-de-universos-subdominios-y-modulos/01-op-imm-001-informacion-preinmunosupresion-adultos/Marco_tecnico_de_responsabilidad_trazabilidad_reproducibilidad_y_criticidad_OP-IMM-001_v0.1_2026-09-03.md).
- **IMM-G6:** [propiedad matricial A0](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/cambio-rumbo/03-base-documental-candidata/11-matrices-en-evaluacion/G6-MAT_propiedad_matricial_A0_OP-IMM-001_v0.1_2026-09-03.md).
- **IMM-G7:** [usos, composición y rutas](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/cambio-rumbo/03-base-documental-candidata/12-rutas-en-evaluacion/G7-RUT_usos_composicion_y_rutas_OP-IMM-001_v0.1_2026-09-03.md).
- **IMM-G8:** [ITI y laboratorio](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/cambio-rumbo/03-base-documental-candidata/13-iti-y-laboratorio/G8-ITI_OP-IMM-001_v0.1_2026-09-03.md).
- **IMM-G9:** [contraste empírico](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/cambio-rumbo/03-base-documental-candidata/14-contraste-empirico/G9-EMP_contraste_sistematico_OP-IMM-001_v0.1_2026-09-03.md).
- **IMM-G10:** [requisitos demostrados y encaje preliminar](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/3bea6b714be3bd1330e6ca6bbbc228b0eb9c065d/dominios/inmunologia/cambio-rumbo/03-base-documental-candidata/15-requisitos-lenguaje-sv/G10-SV_requisitos_demostrados_OP-IMM-001_v0.1_2026-09-03.md).

### 3.2 Lenguaje SV

- **LSV-README:** [estado soberano del Lenguaje](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/README.md).
- **LSV-FRONT:** [frontera normativa](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md).
- **LSV-IR:** [IR canónica 0.3](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/IR_CANONICA_BIENFORMACION_SV_v0_3.md), [tipos Rust](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/rust/sv_core/src/ir.rs) y [bienformación Rust](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/rust/sv_core/src/wellformed.rs).
- **LSV-PROFILES:** [perfiles fuente SVP-ES/SVP-EN](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md) y [adenda léxica](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/ADENDA_NORMATIVA_PERFIL_LEXICO_GRAMATICA_SVP_0_2_2026_08_27.md).
- **LSV-R0:** [cierre integral R0](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/calidad/ACTA_TECNICA_DE_CIERRE_INTEGRAL_R0_PRIMERA_REALIZACION_SOBERANA_SV_2026_08_24.md), [proyección diferencial Rust](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/rust/sv_core/src/equivalence.rs) y [serializador canónico Python](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/src/svp_serialize.py).
- **LSV-R1:** [cierre R1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md) y contratos R1-0 a R1-5 en `docs/arquitectura/`.
- **LSV-R2:** [apertura documental de R2](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/arquitectura/ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md) y [contrato R2-0](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/arquitectura/CONTRATO_R2_0_PERSISTENCIA_CONTINUIDAD_Y_RECUPERACION_2026_08_25.md).
- **LSV-SEC:** [entorno soberano](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/ESPECIFICACION_ARQUITECTONICA_ENTORNO_EJECUCION_SOBERANO_SV_V0.md) y contratos abstractos `SEC.0-M`, `SEC.0-X`, `SEC.0-D` y `SEC.0-T` en `docs/arquitectura/`.
- **LSV-FFLE:** [contrato mínimo de suficiencia representacional](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/arquitectura/CONTRATO_MINIMO_DE_SUFIENCIA_REPRESENTACIONAL_POR_OPERACION_PARA_EL_LENGUAJE_SV_2026_08_21.md) y [matriz de impacto](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/arquitectura/MATRIZ_DE_IMPACTO_DE_LA_SUFIENCIA_REPRESENTACIONAL_EN_LA_ESPECIFICACION_Y_LA_IMPLEMENTACION_2026_08_21.md).
- **LSV-DEBT:** [registro de deuda viva](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3c122d1f79a1fcf7f9c3f02db5e7534b4efb7c2d/docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md).

## 4. Arquitectura de encaje y responsabilidad

| Capa | Responsabilidad propia | Lo que no debe absorber |
|---|---|---|
| Dominio de Inmunología | finalidad clínica, 27 parámetros, propiedad, fuentes, reglas clínicas, criticidad, terminologías aplicables y significado de los resultados | sintaxis universal, persistencia física o afirmaciones de conformidad del Lenguaje |
| Lenguaje SV | formas, tipos, IR, `Tri`, bienformación, semántica formal, operaciones puras, autoridad y fronteras de efectos que hayan sido constituidas | criterio médico, integración hospitalaria, almacenamiento o autorización sanitaria |
| SV-motor y conectores | carga de manifiestos, transducciones, perfil de salida, trazas de ejecución, adaptadores FHIR/terminologías y mediación con servicios | declarar por sí mismos verdad clínica o fabricar autoridad |
| Infraestructura | persistencia, recuperación, aislamiento, secretos, reloj, identidad material, construcción, distribución, carga y observabilidad | resolver semántica clínica o cambiar resultados normativos |
| Organización sanitaria o responsable del producto | autoridad, validación clínica, privacidad, calidad, riesgo, retención, vigilancia y conformidad regulatoria | trasladar sus obligaciones al Lenguaje |

La organización gobierna el uso; el motor consume conjuntamente el perfil de dominio y el programa SV; el Lenguaje determina sólo las formas y operaciones que le corresponden; la infraestructura sostiene las garantías materiales. Ninguna capa acredita por sí sola a las demás.

### 4.1 Límites nucleares vigentes que afectan al encaje

La estabilidad del Playground y el cierre registral B2 no eliminan deuda del núcleo. La inspección de la realización vigente obliga a conservar, como mínimo, estas reservas:

| Límite material | Hecho del corte | Consecuencia para OP-IMM-001 |
|---|---|---|
| codominio | `wellformed.rs` exige que el codominio no esté vacío, pero no acredita unicidad de todos sus miembros | una salida OP no puede confiar todavía en que un codominio mal constituido sea rechazado |
| `OutputSemantics` | la realización no prueba totalidad, unicidad ni pertenencia de cada símbolo al codominio declarado | no puede acreditarse una plantilla clínica total y unívoca sólo porque el programa compile |
| proyección JSON | la proyección diferencial Rust no es el serializador canónico completo y la cadena vigente no acredita rechazo universal de claves homónimas | OP necesita un serializador propio cerrado y casos negativos de colisión |
| `Domain` | varios campos se conservan nominalmente sin interpretación ejecutiva completa; DFL-005 permanece viva | enumerar los 27 parámetros no demuestra que sus políticas y criterios sean ejecutados |
| criticidad | no existe productor material de `CriticalityResult`; DFL-006 permanece viva | `Frame.criticalities` debe seguir vacío y la criticidad OP fuera del Frame |
| referencias | la bienformación local de `Horizon` no basta por sí sola para acreditar toda referencia arquitectónica | el experimento debe incluir referencias colgantes como casos negativos externos |
| resolución humana | una revisión no equivale automáticamente a cierre clínico positivo ni a persistencia de la decisión | toda adjudicación OP debe ser un acto nuevo, explícito y atribuible |

Estas carencias no demuestran un fallo material del ensamblador multifuente. Demuestran que **compilar o ensamblar no basta para acreditar el contrato clínico-técnico de OP-IMM-001**.

## 5. Matriz completa de los 44 requisitos

| Requisito_ID | estado | capacidad actual | localizador | prueba | límite | componente responsable | dependencia | horizonte | siguiente decisión |
|---|---|---|---|---|---|---|---|---|---|
| `REQ-IMM-LSV-001` | `EXTENSION_COMPATIBLE_NECESARIA` | `IrProgram` conserva fichero, hash fuente, objetos y operaciones; no conserva el perfil fuente ni la tupla completa | IMM-TECH; LSV-IR | inspección de `IrProgram` | faltan finalidad, reglas, configuración, fuentes, dependencias, terminologías, jurisdicción, instante y estado humano | dominio + motor; Lenguaje sólo si se demuestra equivalencia causal en más de un universo | decisión sobre manifiesto de ejecución | desarrollo próximo | especificar primero un manifiesto externo versionado y decidir después qué campos deben entrar en IR |
| `REQ-IMM-LSV-002` | `REPRESENTABLE_HOY_NO_INTEGRADO` | perfiles explícitos conservan los bytes UTF-8 originales y calculan `source_sha256` sin normalización Unicode | LSV-PROFILES; LSV-IR | conformidad de perfiles y campo `source_sha256` | la normalización NFC del dato clínico, si se usa, debe ser otra transformación versionada y preservar original y hash | Lenguaje para `.svp`; motor/dominio para carga clínica | perfil OP de entrada | ahora | probar en OP diferencias de byte, Unicode y perfil sin alterar el fuente |
| `REQ-IMM-LSV-003` | `EXTENSION_COMPATIBLE_NECESARIA` | existe serialización canónica Python de IR y paridad Rust nativo/WASM de una proyección | LSV-R0; LSV-IR | corpus diferencial y paridad de destinos | no existe salida clínica OP canónica; tampoco se acreditan en Rust totalidad/unicidad de `OutputSemantics` ni rechazo universal de colisiones JSON | motor + Lenguaje | identidad completa y serializador OP | desarrollo próximo | fijar el objeto exacto cuya igualdad se exige, cerrar sus invariantes y definir su serializador canónico |
| `REQ-IMM-LSV-004` | `DISPONIBLE_HOY_DEMOSTRADO` | Python canoniza la IR completa de referencia; Rust nativo y WASM igualan su proyección diferencial | LSV-R0 | pruebas R0-7/R0-8 y código de ambos serializadores | `equivalence_json` no debe presentarse como serializador canónico completo de Rust | Lenguaje | ninguna | ahora | conservar separados ambos alcances en toda evidencia OP |
| `REQ-IMM-LSV-005` | `EXTENSION_COMPATIBLE_NECESARIA` | la semántica pura y la serialización ordenada reducen variación | LSV-R0; LSV-SEC | pruebas deterministas existentes | la identidad OP incluye reloj, plataforma, concurrencia y mensajes aún no ligados a una salida normativa completa | Lenguaje + motor + infraestructura | salida canónica e identidad de ejecución | desarrollo próximo; R3/R4 para materialidad | enumerar todas las fuentes de no determinismo y añadir vectores por fuente |
| `REQ-IMM-LSV-006` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | el Lenguaje usa literales declarados; no necesita generar prosa clínica | IMM-G7; IMM-G8; LSV-FRONT | inspección de gramática y rutas de salida | las plantillas clínicas y su autorización son semántica del dominio/producto | dominio + motor + organización | perfil de salida clínica | ahora | fijar catálogo cerrado, versión, autoría y hash fuera de la gramática |
| `REQ-IMM-LSV-007` | `EXTENSION_COMPATIBLE_NECESARIA` | `Tri`, `Bottom`, admisibilidad y diagnósticos son formas distintas | LSV-FRONT; LSV-IR; LSV-SEC | tipos y validadores vigentes | no hay un único resultado OP tipado que separe además regla/configuración ausente y ejecución inválida con detalle | Lenguaje + motor | diseño de `OperationResult`/`ParameterResult` | desarrollo próximo | definir una suma cerrada de estados sin colisiones ni conversiones implícitas |
| `REQ-IMM-LSV-008` | `REPRESENTABLE_HOY_NO_INTEGRADO` | `SEC.0-D` prohíbe convertir imposibilidad técnica en autoridad, permiso o valor de dominio | LSV-SEC; IMM-TECH | contrato abstracto y diagnósticos fuera de `Tri` | OP no está integrado ni probado contra ese contrato | Lenguaje + motor | perfil OP de errores | ahora | añadir vectores donde cada fallo técnico sólo produzca `EJECUCION_TECNICA_NO_VALIDA` |
| `REQ-IMM-LSV-009` | `EXTENSION_COMPATIBLE_NECESARIA` | R1 distingue decisión, permiso, ejecución y registro intra-proceso; existe resolución de `U` | LSV-R1; LSV-IR | contratos y tipos R1 | la resolución vigente no acredita por sí sola cierre clínico positivo ni reapertura persistente de solo adición | Lenguaje + motor + organización | identidad humana y R2 para durabilidad | desarrollo próximo; R2 después | especificar adjudicación y reapertura como eventos nuevos, nunca reescritura |
| `REQ-IMM-LSV-010` | `FASE_FUTURA_YA_PREVISTA` | R1 liga decisiones y efectos dentro del proceso | LSV-R1; LSV-R2; LSV-SEC | contrato de compromiso mediado | no hay ligadura material demostrada entre lectura persistida, decisión y efecto | R2 + motor + almacenamiento | autorización de R2 | R2/R4 | aportar caso TOCTOU OP a R2 sin abrirlo mediante este documento |
| `REQ-IMM-LSV-011` | `FASE_FUTURA_YA_PREVISTA` | `SEC.0-M` distingue intento y efecto externo no acreditado | LSV-SEC; LSV-R2 | contratos abstractos | falta realización durable por frontera de efecto | R2 + motor + conectores + infraestructura | autorización de R2 | R2/R4 | convertir las fronteras OP en casos de fallo y recuperación de R2 |
| `REQ-IMM-LSV-012` | `EXTENSION_COMPATIBLE_NECESARIA` | la IR conserva declaraciones; R1 genera trazas intra-proceso de actos protegidos | LSV-IR; LSV-R1; IMM-G8 | inspección de tipos | no existe `ParameterResult` con entrada, regla, fuente, transformaciones, intermedios y resultado | dominio + motor; Lenguaje si se universaliza | esquema de resultado y procedencia | desarrollo próximo | definir esquema externo completo y medir qué invariantes se repiten en otros universos |
| `REQ-IMM-LSV-013` | `FASE_FUTURA_YA_PREVISTA` | FFL-E prevé testigo de pérdida y suficiencia por operación | LSV-FFLE | contrato `RepresentationRequirement`/certificado de frontera | no está materializado en IR 0.3 ni persistido | Lenguaje + R2 | realización futura de FFL-E y persistencia | fase futura | usar los casos OP como requisitos, sin llamar `U` a una insuficiencia representacional |
| `REQ-IMM-LSV-014` | `DECISION_ARQUITECTONICA_REQUERIDA` | la IR admite referencias declaradas; `Frame.criticalities` debe permanecer vacío en la realización vigente | IMM-TECH; LSV-IR; LSV-DEBT | rechazo actual de criticidades no vacías; DFL-006 | no existe productor material de `CriticalityResult`; la criticidad clínica no puede inventarse | dominio + Lenguaje + organización | regla clínica versionada y decisión sobre productor | antes de integración | mantener criticidad fuera del Frame hasta decisión formal del Director |
| `REQ-IMM-LSV-015` | `REPRESENTABLE_HOY_NO_INTEGRADO` | R1 separa constitución, revisión, permiso y ejecución; el fallo técnico no crea autoridad | LSV-R1; LSV-SEC | contratos cerrados | falta un flujo OP que lo pruebe y persistencia de la intervención | Lenguaje + motor + organización | esquema de adjudicación | ahora; R2 para durabilidad | probar que la intervención humana crea un acto nuevo y no cambia el fallo original |
| `REQ-IMM-LSV-016` | `EXTENSION_COMPATIBLE_NECESARIA` | R1 representa autoridad, delegación, revisión, decisión sellada y revocación intra-proceso | LSV-R1; IMM-TECH | contratos R1-0 a R1-5 | no existe manifiesto OP completo ni firma/persistencia material demostrada | dominio + motor + organización; R2/R3 para materialidad | identidad de reglas y autoridad | desarrollo próximo; R2/R3 | definir identificadores y hashes obligatorios antes de elegir firma o infraestructura |
| `REQ-IMM-LSV-017` | `EXTENSION_COMPATIBLE_NECESARIA` | repositorio, corpus y artefactos con huellas permiten evidencia reproducible parcial | LSV-R0; LSV-SEC | reconstrucción y regresiones existentes | falta paquete autocontenido OP con entradas, manifiestos, salidas, versiones y trazas | dominio + motor + Lenguaje | requisitos 001, 003, 012 y 016 | desarrollo próximo | especificar un paquete auditable sin dependencia de explicaciones externas |
| `REQ-IMM-LSV-018` | `FASE_FUTURA_YA_PREVISTA` | `SEC.0-M` y R2 definen estado autoritativo y persistencia | LSV-SEC; LSV-R2 | contratos, no realización | ninguna tecnología ni durabilidad está demostrada | R2 + motor + almacenamiento | autorización de R2 | R2 | aportar inventario de objetos OP que deben persistir y sus invariantes |
| `REQ-IMM-LSV-019` | `FASE_FUTURA_YA_PREVISTA` | recuperación y continuidad están previstas | LSV-SEC; LSV-R2 | casos abstractos de caída | falta ejecución material y prueba de recuperación | R2 + infraestructura | 018 | R2/R4 | definir oráculos OP para caída de proceso, sistema y suministro |
| `REQ-IMM-LSV-020` | `FASE_FUTURA_YA_PREVISTA` | escritura parcial, corrupción, retroceso, clonación y bifurcación están tipados como riesgos | LSV-SEC; LSV-R2 | contratos `SEC.0-M` y R2-0 | no hay almacenamiento realizado ni detectores demostrados | R2 + infraestructura | 018 | R2/R4 | incorporar vectores OP a la campaña autorizada de continuidad |
| `REQ-IMM-LSV-021` | `FASE_FUTURA_YA_PREVISTA` | la especificación distingue autoridad, vista, copia, respaldo y réplica | LSV-SEC; LSV-R2 | definiciones contractuales | distinción no realizada materialmente | R2 + motor + infraestructura | 018 | R2 | declarar qué vistas OP son no autoritativas y probar su incapacidad de escribir |
| `REQ-IMM-LSV-022` | `FASE_FUTURA_YA_PREVISTA` | R1 separa permiso, intento y efecto; `SEC.0-M` prevé reanudación e incertidumbre | LSV-R1; LSV-SEC; LSV-R2 | contratos abstractos | faltan idempotencia persistente y reconciliación de efecto externo | R2 + motor + conector | 010, 011 y 018 | R2/R4 | definir claves de idempotencia y estados de reconciliación sin afirmar exactamente-una-vez |
| `REQ-IMM-LSV-023` | `DISPONIBLE_HOY_DEMOSTRADO` | la asignación por capas está fijada en esta valoración y concuerda con SEC/R2 | LSV-SEC; LSV-R2; sección 4 | contraste de responsabilidades | asignar no equivale a realizar | Lenguaje: invariantes; motor: orquestación; almacenamiento/SO: materialidad; organización: gobierno | ninguna | ahora | conservar esta separación en cada requisito futuro |
| `REQ-IMM-LSV-024` | `REPRESENTABLE_HOY_NO_INTEGRADO` | SEC fija propiedades mínimas abstractas sin prescribir base de datos o sistema operativo | LSV-SEC | contratos M/X/T/D | aún no existe perfil mínimo OP ni pruebas materiales | Lenguaje para invariantes; infraestructura para tecnología | aplicabilidad OP | ahora | seleccionar sólo propiedades causales al universo y posponer elecciones tecnológicas |
| `REQ-IMM-LSV-025` | `EXTENSION_COMPATIBLE_NECESARIA` | cadenas y objetos declarativos pueden conservar identificadores opacos | LSV-IR; IMM-TECH | campos abiertos y hashes fuente | no hay tipo vigente que exija conjuntamente sistema, versión, código, URI, jurisdicción, vigencia y hash | dominio + motor; Lenguaje sólo si se demuestra equivalencia causal en más de un universo | decisión 026 | desarrollo próximo | crear esquema externo cerrado y validar presencia, formato y hash |
| `REQ-IMM-LSV-026` | `DECISION_ARQUITECTONICA_REQUERIDA` | IR admite nombres/referencias, pero no gobierna metadatos clínicos completos | LSV-IR; LSV-FFLE; IMM-G10 | inspección estructural | incorporar detalles de un único dominio al núcleo universal sería prematuro | Director + dominio + Lenguaje | al menos otro universo y análisis de pérdida | antes de cambiar IR | mantener en perfil/conector todo dato no requerido por la semántica del Lenguaje |
| `REQ-IMM-LSV-027` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | FHIR puede transportar procedencia y auditoría como referencias externas | IMM-TECH | encaje de interfaces, no implementación | `Provenance` y `AuditEvent` no prueban verdad clínica, autoridad ni ejecución material | motor/conector + organización | perfiles FHIR autorizados | desarrollo próximo | mapear sin equivalencias semánticas implícitas y conservar identidad/hashes |
| `REQ-IMM-LSV-028` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | los recursos FHIR pueden referenciarse desde el manifiesto del dominio | IMM-TECH; LSV-IR | representabilidad de identificadores opacos | el núcleo no debe interpretar automáticamente recursos clínicos | dominio + conector FHIR | perfiles, versiones y jurisdicción | desarrollo próximo | definir cardinalidad, versión, propósito y hash por tipo de referencia |
| `REQ-IMM-LSV-029` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | códigos y sistemas pueden viajar como datos declarados | IMM-TECH | estructura externa propuesta | equivalencia, retiro, licencia, jurisdicción y versión son gobierno terminológico, no palabras clave del Lenguaje | dominio + servicio terminológico + organización | perfil terminológico versionado | desarrollo próximo/universal | representar códigos exactos y equivalencias tipadas, nunca cadenas supuestamente equivalentes |
| `REQ-IMM-LSV-030` | `NO_APLICABLE_JUSTIFICADO` | OP-IMM-001 no consume imagen médica en su alcance vigente | IMM-REQ; IMM-TECH | inventario de 27 parámetros y exclusiones | una necesidad futura de imagen exigiría nueva aplicabilidad; no autoriza una forma nueva hoy | módulo de imagen + conector DICOM | cambio constituido del universo | no aplicable ahora | conservar DICOM fuera del núcleo; si aparece, empezar por UID/hash y perfil externo |
| `REQ-IMM-LSV-031` | `REPRESENTABLE_HOY_NO_INTEGRADO` | el Lenguaje no interpreta por sí mismo imágenes, códigos ni informes; exige operaciones y reglas declaradas | LSV-FRONT; LSV-SEC | ausencia de intérprete clínico implícito | faltan perfiles OP que bloqueen toda entrada sin regla constituida | Lenguaje + dominio + motor | reglas y autoridad | ahora | crear casos negativos sintéticos para entradas sin regla o con versión no vigente |
| `REQ-IMM-LSV-032` | `DISPONIBLE_HOY_DEMOSTRADO` | especificaciones, trazabilidad, pruebas, revisión de cambios y evidencias de construcción pueden contribuir a controles técnicos | LSV-R0; LSV-SEC; LSV-DEBT | repositorio y campañas auditables | contribuir evidencia no declara conformidad con ninguna norma ni cubre producto, clínica u organización | Lenguaje + sistema de calidad del producto | matriz de aplicabilidad normativa | ahora | mapear evidencia concreta a controles aplicables sin usar lenguaje de certificación |
| `REQ-IMM-LSV-033` | `DISPONIBLE_HOY_DEMOSTRADO` | la frontera de responsabilidad está identificada | LSV-SEC; sección 4 | asignación explícita | validación clínica, gestión de riesgos del producto, privacidad, vigilancia, despliegue y autorización quedan fuera | organización/fabricante/promotor/autoridad | ninguna | ahora | mantener un registro externo de obligaciones con propietario y disparador |
| `REQ-IMM-LSV-034` | `EXTENSION_COMPATIBLE_NECESARIA` | Git, actas, pruebas y deuda enlazan cambios del Lenguaje | LSV-R0; LSV-DEBT | historial y registros públicos | no hay matriz OP requisito-riesgo-control-código-prueba-versión | Lenguaje para sus controles; producto para matriz total | identificadores estables de requisitos | desarrollo próximo | constituir trazabilidad bidireccional sólo para elementos realmente implementados |
| `REQ-IMM-LSV-035` | `FASE_FUTURA_YA_PREVISTA` | R0 acredita el núcleo compartido; R3/R4 y Garantías I/II reservan cadena material e integración adversarial | LSV-R0; LSV-SEC; LSV-README | paridad actual y hoja de fases | no están probadas procedencia completa, raíz de confianza, carga o distribución soberanas | R3/R4 + infraestructura + Lenguaje | cierre autorizado de fases previas | R3/R4 | usar OP como consumidor adversarial cuando corresponda, sin anticipar garantías |
| `REQ-IMM-LSV-036` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | el Lenguaje puede exigir referencias y no filtrar secretos en salidas normativas | LSV-SEC | contratos de frontera | custodia de secretos, datos y registros depende del entorno y la organización | infraestructura + motor + organización | modelo de amenazas y datos | antes de datos reales | clasificar datos, aislar almacenes y probar redacción fuera del núcleo semántico |
| `REQ-IMM-LSV-037` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | la identidad por hashes permite minimizar contenido sin perder toda ligadura | IMM-TECH; LSV-SEC | diseño de referencias | retención, base jurídica y seudonimización son políticas del producto/organización; el hash puede seguir siendo dato personal | organización + infraestructura + dominio | evaluación de protección de datos | antes de datos reales | fijar plazos, bases, borrado, separación de claves y evidencia mínima por finalidad |
| `REQ-IMM-LSV-038` | `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | el Lenguaje aporta conformidad técnica parcial, nunca autorización clínica | IMM-REQ; LSV-README | prohibiciones y estados de garantía | retirar la prohibición exige además validación clínica, seguridad, privacidad, calidad, infraestructura y autorización | organización/fabricante/promotor/autoridades | cierre de todos los controles aplicables | revisión futura | mantener `OP-IMM-001_USO_CON_DATOS_REALES = PROHIBIDO` hasta decisión formal externa |
| `REQ-IMM-LSV-039` | `REPRESENTABLE_HOY_NO_INTEGRADO` | puede diseñarse un corpus sintético de 27 parámetros y una proyección estructural limitada | IMM-G6; IMM-G8; LSV-IR | propuesta de sección 7 | no acredita resultado clínico, persistencia, criticidad ni aptitud; requiere autorización antes de ejecutar | dominio + Lenguaje + motor | decisiones de sección 8 | siguiente acto autorizado | aprobar primero manifiesto, oráculos, límites y casos negativos |
| `REQ-IMM-LSV-040` | `REPRESENTABLE_HOY_NO_INTEGRADO` | perfiles, hashes y declaraciones de captura/admisibilidad/ternarización pueden ensayarse con `Tri` sin cambiar gramática/IR | LSV-PROFILES; LSV-IR; IMM-G6 | corpus sintético propuesto | las cinco agrupaciones menores no son celdas y la agrupación de nueve aún no ha demostrado constitución celular; parámetros y trazas quedan en esquema externo | dominio + Lenguaje + motor | mapa estable de IDs | ahora, tras autorización | probar la estructura externa y, separadamente, la candidatura celular de `M-MODIFIER-001` |
| `REQ-IMM-LSV-041` | `DISPONIBLE_HOY_DEMOSTRADO` | están identificadas las necesidades que exceden el corte: identidad completa, resultados/trazas tipados, testigo de pérdida, salida canónica total e invariantes pendientes de codominio, salida y referencias | LSV-IR; LSV-FFLE; LSV-DEBT | contraste de campos, validadores y realizaciones | identificar una necesidad no autoriza el cambio; el primer recurso debe ser un perfil externo | Director + Lenguaje | evidencia de más universos y decisión 026 | antes de cualquier cambio | no modificar gramática; valorar futura IR/serialización sólo si el perfil externo es insuficiente y cerrar por separado la deuda nuclear aplicable |
| `REQ-IMM-LSV-042` | `DISPONIBLE_HOY_DEMOSTRADO` | se separan perfiles/manifiestos/corpus reversibles de tipos IR, semántica, geometría y fases difíciles de revertir | IMM-G10; LSV-FFLE | análisis de alternativas | la reversibilidad operativa depende después de persistencia y migraciones | Director + responsables de cada capa | inventario de decisiones | ahora | autorizar por separado cada decisión difícil y registrar su razón |
| `REQ-IMM-LSV-043` | `DISPONIBLE_HOY_DEMOSTRADO` | el orden técnico queda definido sin fechas | secciones 7 y 8 | dependencias explícitas | no es compromiso de producción | Director + dominio + Lenguaje + motor | decisiones previas de cada etapa | incremental | seguir el orden: decisiones, perfil externo, corpus, prueba estructural, contraste inter-universos, fases materiales |
| `REQ-IMM-LSV-044` | `DISPONIBLE_HOY_DEMOSTRADO` | existen puertas de revisión que permiten crear más universos sin congelar una infraestructura universal | IMM-REQ; LSV-FFLE; sección 9 | disparadores explícitos | las puertas no permiten ocultar deuda ni usar capacidades no probadas | Director + gobierno técnico | nuevos universos, revisión por pares y evidencia empírica | incremental/universal | revisar al segundo universo, ante pérdida representacional y antes de datos reales o cambio de IR |

### 5.1 Recuento y ausencia de ambigüedad

| Estado | Requisitos |
|---|---:|
| `DISPONIBLE_HOY_DEMOSTRADO` | 8 |
| `REPRESENTABLE_HOY_NO_INTEGRADO` | 7 |
| `EXTENSION_COMPATIBLE_NECESARIA` | 10 |
| `FASE_FUTURA_YA_PREVISTA` | 9 |
| `RESPONSABILIDAD_DE_OTRO_COMPONENTE` | 7 |
| `DECISION_ARQUITECTONICA_REQUERIDA` | 2 |
| `NO_APLICABLE_JUSTIFICADO` | 1 |
| `NO_ADMISIBLE` | 0 |
| `NO_DETERMINADO` | 0 |
| **Total** | **44** |

La ausencia de filas `NO_ADMISIBLE` no legitima cualquier implementación. Sí serían no admisibles las soluciones prohibidas en las secciones 2.3 y 8. La matriz clasifica necesidades; no avala medios incompatibles para satisfacerlas.

En los requisitos de naturaleza analítica —asignación de responsabilidades, identificación de límites, orden y puertas de revisión— `DISPONIBLE_HOY_DEMOSTRADO` significa que la respuesta puede obtenerse de evidencia ya existente en el corte; no significa que exista una nueva capacidad ejecutable ni que esta valoración la haya incorporado al Lenguaje.

## 6. Respuestas a las diez preguntas de bisturí

### 6.1 ¿Puede la IR 0.3 representar los 27 parámetros, sus fuentes, su resultado ternario y la pérdida de distinción sin crear tipos nuevos?

**No de forma fiel.** Puede enumerar 27 nombres opacos en `Domain.parameters`, usar identificadores numéricos en captura/admisibilidad y representar vectores `Tri`. No puede expresar de manera tipada la identidad y propiedad de cada parámetro, sus fuentes, transformaciones, `ParameterResult` ni el testigo de la distinción perdida. Las seis cardinalidades de G6 no son seis tamaños de célula. Sólo el grupo de nueve puede someterse a una prueba de constitución como candidato a `SV(9,3)`; la cardinalidad no autoriza su proyección.

### 6.2 ¿Puede representar la ejecución técnicamente inválida fuera de `Tri` con el detalle requerido?

**Puede separarla en principio, pero no con todo el detalle OP en un único resultado vigente.** Los errores, `Bottom`, admisibilidad y `Tri` ocupan planos distintos, y `SEC.0-D` prohíbe convertir fallo técnico en valor del dominio. Falta integrar una suma cerrada que distinga dato no admitido, regla o configuración ausente y ejecución inválida con causa, fase y evidencia.

### 6.3 ¿Qué objeto vigente conserva hoy la identidad completa de entrada, reglas, configuración, fuentes y programa?

**Ninguno.** `IrProgram` conserva fichero y hash de fuente junto a objetos y operaciones. Los artefactos de ensamblaje añaden identidad de unidades, pero tampoco forman la tupla completa definida por `IMM-TECH`. Debe existir primero un manifiesto externo de ejecución.

### 6.4 ¿Qué salida es actualmente canónica y cuál no?

La referencia Python produce la serialización canónica completa de la IR que implementa. Rust nativo y WebAssembly producen de forma equivalente la proyección diferencial definida en `equivalence_json`. Esa proyección no es el serializador canónico completo de Rust. No existe todavía una salida clínica OP canónica completa.

### 6.5 ¿Qué traza sobrevive hoy al proceso y cuál depende de R2?

Sobreviven los artefactos que un proceso externo escriba —fuentes, hashes, corpus, resultados de CI y registros del repositorio—, pero el Lenguaje no demuestra un almacén autoritativo de trazas OP. R1 acredita trazas y ligaduras intra-proceso. Estado, decisiones, fuentes, configuraciones, revocaciones y trazas durables dependen de R2 y de su realización material.

### 6.6 ¿Qué parte de R2 puede reutilizar directamente los casos de fallo de OP-IMM-001?

OP puede aportar casos para estado autoritativo, dependencias persistentes, discrepancia de hashes, caída entre comprobación y efecto, escritura parcial, recuperación, retroceso, clonación, bifurcación, vistas no autoritativas, efecto externo incierto, repetición idempotente, revocación y retención. Son **entradas futuras de prueba**; aportarlas no abre R2 ni demuestra sus propiedades.

### 6.7 ¿FHIR, DICOM y las terminologías deben tratarse como perfiles externos, dependencias persistentes, tipos de IR o combinaciones?

FHIR y las terminologías deben comenzar como perfiles externos versionados y, cuando sean causalmente relevantes, como dependencias persistentes del manifiesto con sistema, versión, jurisdicción y hash. Sólo un invariante semántico causalmente equivalente demostrado en varios universos justificaría un tipo de IR. DICOM no es aplicable al alcance actual; si aparece, debe empezar como referencia externa opaca verificable por UID/hash, no como semántica del núcleo.

### 6.8 ¿Dónde reside la frontera exacta entre Lenguaje SV y SV-motor?

El Lenguaje define formas, tipos, bienformación, semántica formal y operaciones puras, además de contratos de autoridad y efectos que hayan sido constituidos. El motor carga manifiestos, conecta fuentes, ejecuta transducciones y plantillas de dominio, produce el paquete OP, media adaptadores y entrega persistencia a la infraestructura. El motor no puede redefinir `Tri`, sanear un fallo, fabricar autoridad ni cambiar una salida normativa.

### 6.9 ¿Qué requisito contradice o fuerza indebidamente la arquitectura vigente?

Ningún requisito de resultado es en sí incompatible, pero varias realizaciones posibles sí lo serían:

- crear células menores que `SV(9,3)`;
- rellenar, duplicar o mezclar parámetros para aparentar nueve posiciones;
- normalizar silenciosamente los bytes del programa `.svp`;
- insertar terminologías o reglas médicas como palabras clave universales;
- convertir FHIR, DICOM o un registro de auditoría en verdad clínica automática;
- cargar persistencia, regulación, validación clínica o gobierno organizativo sobre el núcleo;
- presentar como salida canónica completa la proyección diferencial Rust;
- ejecutar R2 o cambiar IR antes de autorización y evidencia comparada entre universos.

### 6.10 ¿Cuál es la integración mínima útil demostrable ahora sin afirmar aptitud clínica?

La definida en la sección 7: manifiesto externo exacto, datos sintéticos, 27 IDs estables, proyección estructural limitada, prueba previa de constitución de `M-MODIFIER-001` como candidato a `SV(9,3)`, cinco agrupaciones externas, casos negativos y comparación separada de Python y Rust/WASM. Su salida sólo puede titularse **evidencia estructural no clínica**.

## 7. Propuesta mínima de integración sintética

Esta propuesta no constituye autorización de ejecución.

1. Congelar los dos commits citados y todas las versiones del perfil OP.
2. Crear un manifiesto externo versionado que contenga la tupla de identidad completa de `IMM-TECH`, los 27 parámetros, sus seis propietarios, reglas, fuentes, configuraciones, terminologías y hashes.
3. Usar exclusivamente datos sintéticos, sin identificadores ni datos derivados de pacientes.
4. Fijar un mapa estable y reversible de los 27 identificadores de dominio a identificadores técnicos, sin convertir el número técnico en significado clínico.
5. Declarar en `.svp`, cuando pasen la gramática y bienformación vigentes, sólo captura, admisibilidad, ternarización, referencias de dominio y operaciones estructurales ya existentes.
6. Someter `M-MODIFIER-001` a una prueba previa de constitución de célula. Sólo si demuestra correspondencia posicional, particiones, codominio, semántica de salida, operación y suficiencia podrá proyectarse sobre `SV(9,3)`. Mantener las agrupaciones de 6, 1, 3, 2 y 6 parámetros en el esquema externo; no son células.
7. Mantener `Frame.criticalities = []`. La criticidad contextual vive en el perfil externo hasta que exista productor formal autorizado.
8. Construir casos positivos y negativos, al menos:
   - misma identidad produce mismos bytes dentro de cada serializador declarado;
   - cambio de un byte produce identidad diferente;
   - perfil fuente incorrecto se rechaza sin caída silenciosa;
   - fallo técnico produce sólo `EJECUCION_TECNICA_NO_VALIDA`;
   - regla o configuración ausente no produce `0`, `1`, `U` ni perfil parcial;
   - intervención humana crea un acto nuevo y no reescribe el fallo;
   - criticidades no vacías son rechazadas por el corte vigente;
   - ninguna agrupación de 1, 2, 3 o 6 se admite como célula;
   - no se emplea relleno, duplicación ni mezcla para completar nueve posiciones.
9. Comparar por separado la serialización canónica Python y la equivalencia de la proyección Rust nativo/WASM. No derivar una igualdad de la otra.
10. Emitir un paquete autocontenido con fuentes, manifiestos, hashes, resultados y límites, rotulado `EVIDENCIA_ESTRUCTURAL_NO_CLINICA`.

El experimento demuestra representación parcial y disciplina de frontera. No demuestra criterio clínico, criticidad, persistencia, recuperación, seguridad material, utilidad clínica, conformidad ni autorización.

## 8. Decisiones reservadas al Director

| Decisión | Recomendación técnica | Motivo |
|---|---|---|
| Representación de las seis agrupaciones G6 | conservar las seis agrupaciones en esquema externo; admitir el grupo de nueve sólo como candidato a `SV(9,3)` y exigir constitución semántica antes de proyectarlo | preserva la célula mínima y evita identificar cardinalidad con célula, relleno o mezcla semántica |
| Identidad, `ParameterResult` y traza | comenzar con perfil externo completo | permite obtener evidencia sin congelar IR desde un único universo |
| Promoción de campos a IR | decidir sólo ante pérdida normativa demostrada y repetición en más universos | evita particularizar el Lenguaje para Inmunología |
| Criticidad | mantenerla fuera de `Frame` y sin productor implícito | DFL-006 sigue viva y la realización rechaza criticidades no vacías |
| Salida canónica OP | serializador cerrado del motor, distinto de `equivalence_json` | la salida clínica no es la IR ni la proyección diferencial Rust |
| Plantillas clínicas | propiedad del dominio, cerradas, versionadas, autorizadas y con hash | evita prosa generativa y semántica médica en la gramática |
| Inicio del experimento | autorización posterior específica, con alcance y oráculos cerrados | esta respuesta no modifica ni abre fases |
| Uso de datos reales | mantener prohibición hasta decisión organizativa y regulatoria formal | excede las competencias del Lenguaje |

## 9. Dependencias, orden y puntos de revisión

### 9.1 Orden técnico sin fechas

1. aceptar o corregir las decisiones de la sección 8;
2. constituir el perfil externo y el manifiesto de ejecución;
3. revisar el corpus sintético y sus oráculos antes de ejecutarlo;
4. autorizar, en acto separado, la integración estructural mínima;
5. medir qué información se pierde y qué invariantes se repiten en otros universos;
6. decidir si alguna carencia justifica una futura extensión de IR o serialización;
7. aportar los casos materiales a R2/R3/R4 sólo cuando cada fase sea autorizada;
8. someter el producto y la organización a sus procesos clínicos, de seguridad, calidad y regulación antes de cualquier dato real.

### 9.2 Puertas de revisión

- **Segundo universo constituido:** comparar requisitos sin herencia automática; sólo una coincidencia demostrada y causalmente equivalente puede proponerse como candidata común antes de universalizar tipos o metadatos.
- **Pérdida representacional demostrada:** decidir si se resuelve en perfil, motor o IR.
- **Primer efecto externo:** revisar autoridad, idempotencia, incertidumbre y recuperación.
- **Apertura autorizada de R2:** incorporar los casos OP de persistencia sin alterar su contrato por conveniencia de dominio.
- **Antes de cambiar gramática o IR:** adversarial inter-universos y regresión completa del Lenguaje.
- **Antes de usar datos reales:** cierre independiente de seguridad, privacidad, calidad, infraestructura, validación clínica y autorización.
- **Revisión por pares y pluralidad suficiente de universos:** revisión universal de interoperabilidad y arquitectura.

## 10. Riesgos de sobreingeniería y de infradiseño

### 10.1 Sobreingeniería que debe evitarse

- universalizar desde un solo universo la tupla OP completa;
- incorporar FHIR, DICOM o terminologías a la gramática;
- elegir ahora una base de datos, firma, orquestador o plataforma para satisfacer contratos todavía no abiertos materialmente;
- exigir imagen, alta disponibilidad o exactamente-una-vez sin aplicabilidad clínica constituida;
- modificar la geometría de célula para encajar cardinalidades externas;
- abrir R2 como efecto lateral de esta valoración.

### 10.2 Infradiseño que debe bloquearse

- usar nombres opacos de parámetros como si fueran trazabilidad suficiente;
- confundir `U` con dato ausente, regla ausente, fallo técnico o pérdida representacional;
- declarar canonicidad total desde una proyección parcial;
- dejar fuentes, reglas, configuraciones o terminologías fuera de la identidad causal;
- permitir que una adjudicación humana reescriba el antecedente;
- posponer deuda sin responsable, dependencia y puerta de revisión;
- convertir una compilación o una prueba sintética en afirmación de seguridad clínica.

## 11. Carencias vivas y componente responsable

| Carencia | Situación | Responsable primario | Disparador |
|---|---|---|---|
| identidad completa de ejecución | no existe en un objeto vigente | dominio + motor; posible extensión futura del Lenguaje | perfil externo y contraste con más universos |
| resultado y traza tipados por parámetro | no existen en IR 0.3 | dominio + motor; Lenguaje sólo si se demuestra equivalencia causal en más de un universo | experimento sintético y análisis de pérdida |
| testigo de pérdida de distinción | previsto por FFL-E, no realizado | Lenguaje | futura autorización de realización |
| productor de criticidad | ausente; DFL-006 | dominio + Lenguaje | decisión arquitectónica específica |
| unicidad de codominio y totalidad/unicidad de salida | no acreditadas por la bienformación Rust vigente | Lenguaje | cierre nuclear específico, separado de R2 |
| colisiones de proyección JSON y referencias colgantes | rechazo completo no acreditado | Lenguaje | casos negativos y cierre nuclear específico |
| salida clínica canónica | ausente | motor + dominio | cierre del perfil de salida |
| persistencia y recuperación | contratos existentes, realización no demostrada | R2 + infraestructura | apertura material autorizada de R2 |
| cadena de construcción y carga soberana | no demostrada | R3/R4 + infraestructura | fases previas cerradas y autorización |
| interoperabilidad clínica | perfiles no constituidos | dominio + conectores + organización | aplicabilidad y jurisdicción concretas |
| aptitud clínica y conformidad | prohibidas/no acreditadas | responsable del producto y organización sanitaria | expediente, validación y autorización formales |

## 12. Veredicto final y estado resultante

El veredicto es **`ENCAJA_CON_CAMBIOS`** con el siguiente alcance exacto:

- **encaja hoy** la preparación de una proyección estructural, sintética y no clínica sobre capacidades ya existentes; la constitución de `M-MODIFIER-001` como célula sigue pendiente de prueba;
- **requiere cambios compatibles** en perfiles/manifiestos del dominio y del motor, y posiblemente en una futura IR sólo tras evidencia comparada entre universos;
- **requiere fases futuras ya previstas** para persistencia, recuperación, confianza material e integración adversarial;
- **requiere otros componentes** para interoperabilidad clínica, infraestructura, privacidad, calidad, regulación y validación;
- **no autoriza** cambios, experimento, apertura de fase, despliegue ni uso con datos reales.

```text
OP-IMM-001_ENCAJE_LENGUAJE = ENCAJA_CON_CAMBIOS
OP-IMM-001_PROYECCION_ESTRUCTURAL_HOY = REPRESENTABLE_NO_INTEGRADA
OP-IMM-001_EJECUCION_CLINICA_HOY = NO_DISPONIBLE
OP-IMM-001_USO_CON_DATOS_REALES = PROHIBIDO
CELULA_MINIMA_SV = SV(9,3)
CELULA_MENOR_QUE_SV(9,3) = PROHIBIDA
RELLENO_DUPLICACION_O_MEZCLA_PARA_COMPLETAR_CELDA = PROHIBIDO
LENGUAJE_SV_MODIFICADO_POR_ESTA_RESPUESTA = NO
R2_R3_R4_O_GARANTIA_ABIERTA_POR_ESTA_RESPUESTA = NO
EXPERIMENTO_SINTETICO_AUTORIZADO_POR_ESTA_RESPUESTA = NO
SIGUIENTE_ACTO = DECISION_ARQUITECTONICA_Y_AUTORIZACION_SEPARADA
```
