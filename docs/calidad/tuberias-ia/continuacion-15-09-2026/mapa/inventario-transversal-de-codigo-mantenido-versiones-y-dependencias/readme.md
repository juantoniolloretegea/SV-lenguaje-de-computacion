# Inventario transversal de código mantenido, versiones y dependencias

**Versión V.1 · 7 de octubre de 2026.** Sede de consulta conjunta del código relacionado con las tuberías de IA del Lenguaje de Computación y con el ensayo de IA y observabilidad del Motor.

El código tiene varias sedes en GitHub, determinadas por su función y por el ensayo al que pertenece. Este inventario reúne sus referencias y condiciones de uso. No traslada el núcleo, no sustituye los registros de calidad y no convierte las realizaciones experimentales en componentes admitidos del SV.

## 1. Dictamen y alcance

El núcleo del Lenguaje se mantiene en **`SV-lenguaje-de-computacion/rust/sv_core`**. Los desarrollos documentales e históricos de las tuberías de IA permanecen en ese repositorio; las realizaciones de inferencia, suministro documental, instrumentación, adjudicación y presentación de los ensayos se conservan en **`SV-motor/laboratorio/ensayo-ia-y-observabilidad`**, con sus versiones y modelos.

La V.1 incorpora **440 archivos de fuentes, configuración o construcción** a sus sedes de dependencia y relaciona otros **22 que ya eran idénticos en su destino**. Son 462 registros por archivo y versión; no representan 462 componentes originales ni 440 desarrollos inéditos. Algunas versiones comparten contenido con otros antecedentes conservados. Se preserva su relación con cada ensayo para permitir el contraste.

La publicación se ha recuperado de GitHub y cotejado por bytes y SHA-256 mediante Rust. Esta comprobación acredita identidad material de los archivos publicados. **La reconstrucción funcional completa desde una copia limpia de GitHub permanece pendiente** para los conjuntos que dependen de recursos reservados, referencias de entorno o proyectos externos. No se ha ejecutado inferencia ni una nueva campaña de pruebas para elaborar este inventario.

| Referencia de comprobación | Revisión fijada |
|---|---|
| Lenguaje: núcleo, tuberías y mapa antes de incorporar este inventario | `2c400a1c53657690c1767ec294f5d8d5b61aa67a` |
| Motor: estado previo a la conservación V.1 | `a8cd2319c004f76a1159cb382f7ed93b2d648bf3` |
| Motor: incorporación principal de fuentes | `74751569645baac43f3f3a307789a8b00df326f5` |
| Motor: cierre de la conservación V.1 | `02d5aee5fafe71105e2a451273adba888e87308a` |
| Comparación de ramas de Lenguaje | Base `79069f741571efb5da5356049374ce646e6ee38e`; referencias conservadas en el CSV |

La fecha identifica una comprobación, no una garantía de que una rama o dependencia externa permanezca sin cambios. Los enlaces de consulta de esta edición fijan revisiones para conservar su significado.

## 2. Sedes, funciones y dependencias

| Conjunto | Código y función | Dependencia y condición de uso |
|---|---|---|
| Núcleo del Lenguaje | [rust/sv_core](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/2c400a1c53657690c1767ec294f5d8d5b61aa67a/rust/sv_core), versión declarada `0.1.0`; 59 fuentes `.rs`, incluidas pruebas | Sus declaraciones y contratos gobiernan el alcance del núcleo. Las realizaciones del laboratorio no se incorporan a él por proximidad documental. |
| Destinos nativo y WebAssembly | [Espacio de trabajo rust](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/2c400a1c53657690c1767ec294f5d8d5b61aa67a/rust): `sv_core`, `sv_native`, `sv_wasm` | El `Cargo.toml` raíz declara esos tres miembros. Las fuentes de ejemplos, manifiestos y bloqueos se identifican en el inventario por archivo. |
| Tuberías de IA y realizaciones documentales | [docs/calidad/tuberias-ia](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/2c400a1c53657690c1767ec294f5d8d5b61aa67a/docs/calidad/tuberias-ia) | Incluye fuentes bajo proyectos, cambios y controles, además de paquetes históricos. Cada expediente conserva su estado de propuesta, prueba o recepción. |
| Continuación del 15 de septiembre y mapa | [continuacion-15-09-2026](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/2c400a1c53657690c1767ec294f5d8d5b61aa67a/docs/calidad/tuberias-ia/continuacion-15-09-2026) | Es un subconjunto de las tuberías. El mapa histórico permanece intacto y este inventario es un documento adicional, sin sustitución del mapa. |
| Ensayo de IA y observabilidad | [Sede del ensayo](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad) | Su proyecto `eio-candidato`, versión `0.1.0`, declara banco, inferencia y cálculo de costes. Su manifiesto no agrupa automáticamente todos los proyectos anidados. |
| Sistema conjunto | [Lenguaje, computación e IA gobernada](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/sistema-conjunto-lenguaje-computacion-ia-gobernada) | Conserva la orientación de conjunto y sus relaciones con los dominios. Este inventario no eleva la IA a fundamento rector del SV. |
| Suministro documental | [Model Context Protocol](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol) | Versiones conservadas por directorio; incluye `0.1.3` y preparación PDF `0.1.4`. Separar protocolo, lector, corpus, suministro efectivo y recepción. La V.1 añade el verificador de custodia documental. |
| Instrumentación compartida | [instrumentacion-rust](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/instrumentacion-rust) | `sv-instrumentacion 0.1.0`; manifiesto y bloqueo conservados. Declara `sysinfo 0.39.6`, `netstat2 0.11.2`, `serde_json 1.0.151` y `sha2 0.10.9`. La cobertura efectiva debe acreditarse en cada ejecución. |
| Cliente de acceso y controladores Astra | [gpt-6.1-sol/acceso](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6.1-sol/acceso) | `sv-comprobacion-acceso-openai 0.1.0`; conserva su denominación inicial de ruta. Las pruebas Astra tienen sede propia. El cliente depende de la instrumentación compartida, de módulos internos y de las bibliotecas fijadas en Cargo. |
| Catálogo, PDF, revisión doble y examen Astra | [gpt-6-astra](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra) | Controladores, realizaciones conservadas, cálculos, adjudicaciones y visores se identifican por prueba y edición. Una versión inicial no sustituye la recepción de una continuación posterior. |
| Visores egui | [Visor del catálogo](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/catalogo-a0-20261007/visor-egui) y visores de las demás pruebas bajo su edición | Consultar el manifiesto de cada edición: los HTML autosuficientes y sus derivados JavaScript/WebAssembly son productos de presentación. No sustituyen la adjudicación ni acreditan una inferencia adicional. |
| Safeguard y Árbitro | [gpt-oss-safeguard-120b](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b) | Se conservan realizaciones del Árbitro, consistencia, literalidad y retroalimentación A1–A3. Sus proyectos dependen de revisiones de `mistral.rs`, MCP y el núcleo del Lenguaje, además del entorno declarado. |
| Qwen y GPT-OSS | [Qwen](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen) y [OpenAI](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai) | Fuentes y controles históricos permanecen vinculados a sus modelos. Su conservación no declara servidores activos, pesos disponibles o candidatos admitidos. |
| Cálculos auxiliares conservados | [Criterio histórico de puntuación](https://github.com/juantoniolloretegea/SV-motor/tree/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/criterio-puntuacion-20261001), cálculo dimensional Qwen y evaluación PDQ | Son antecedentes identificados. Deben leerse con la norma y recepción de su fecha; no sustituyen los contratos posteriores de adjudicación. |

En el cliente se declaran, entre otras, `reqwest 0.12.28`, `jsonwebtoken 10.3.0`, `serde 1.0.228`, `serde_json 1.0.151`, `sha2 0.10.9` y `zeroize 1.8.2`. Los archivos `Cargo.lock` concretan la resolución de dependencias; las versiones de los manifiestos no demuestran qué funciones se invocaron realmente. La salvedad criptográfica C/ensamblador ya documentada permanece pendiente y no queda resuelta por esta publicación.

## 3. Recuento reproducible por archivo

El [inventario completo de rutas](INVENTARIO-FUENTES-V1.csv) recoge repositorio, ámbito, revisión, ruta, tamaño, identidad Git del contenido y enlace. Incluye fuentes `.rs`, `.html`, `.css`, `.js`, `.mjs`, además de archivos `.toml` y `.lock` de construcción o configuración. Incluye versiones y dependencias de terceros conservadas en esos ámbitos; no computa código embebido como archivos independientes.

| Ámbito | Rutas directas del criterio de recuento |
|---|---:|
| Lenguaje: espacio de trabajo `rust` | 69 |
| Lenguaje: `docs/calidad/tuberias-ia` | 249 |
| Motor: `laboratorio/ensayo-ia-y-observabilidad`, tras la V.1 | 938 |
| **Total de los tres ámbitos sin solapamiento** | **1.256** |

La continuación del 15 de septiembre está incluida en las 249 rutas y no se suma otra vez. En Motor, el mismo criterio de enumeración identificaba 498 rutas antes de esta incorporación; las 440 añadidas dan 938. Este criterio incluye también los TOML de configuración: no debe confundirse con un recuento restringido a archivos llamados exactamente `Cargo.toml` o `Cargo.lock`.

Los [paquetes históricos](PAQUETES-HISTORICOS-V1.csv) se enumeran aparte, con su identidad Git. Su contenido interno no se suma al recuento anterior ni se presenta como revisado íntegramente por esta V.1.

El [manifiesto de conservación](https://github.com/juantoniolloretegea/SV-motor/blob/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/FUENTES-PRESERVADAS-V1-20261007.json) distingue las huellas SHA-256 del antecedente y de la copia pública. **31 archivos tienen adaptación declarada de referencias del entorno o de direcciones de infraestructura**; no son copias idénticas del original ni registros brutos de ejecución. Los restantes 431 registros conservan identidad de contenido. Las adaptaciones no modifican las reglas de adjudicación.

## 4. Ramas y código no incorporado a `main`

La revisión consultó las 119 ramas de Lenguaje: `main` y 118 adicionales. De estas últimas, 104 eran antecesoras de `main` y 14 divergían. En Motor se identificaron `main` y `sv-auth-v0.2`; esta última conserva un commit documental propio. El [detalle de comparación](RAMAS-COMPARADAS-V1.csv) fija las referencias y la base empleada, evitando presentar las distancias históricas como valores permanentes.

Una rama divergente puede contener cambios latentes, documentación, antecedentes integrados por otra secuencia o desarrollos pendientes. **Divergencia no equivale a pérdida ni autoriza una integración automática.** Las solicitudes #59 y #56 están integradas aunque sus ramas preservadas mantienen una historia divergente.

El caso funcional pendiente más claro es [PR #89, diagnósticos ES/EN](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/89): continúa abierta y en borrador, con cabecera `45cf0d5fb4674500fc1f089418c9f362e177e0be`. Conserva `diagnostic_frontend.rs`, `diagnostic_validation.rs` y sus pruebas, ausentes del `main` inventariado. El mapa ya identificaba esta revisión como candidata no promovida. Está conservada en GitHub; no forma parte del núcleo recibido por el mero hecho de existir.

El mapa también remite a [SV-matematica-semantica-cuaternaria](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/tree/511f05857cbeda7474483dee654990e6064324c5). La rama `lab/playground-sv-permanente` fue comprobada en `511f05857cbeda7474483dee654990e6064324c5`; su `main`, en `d458277890e5953df08a2f55b7e8ef4a951a2e30`. La comparación registró 221 commits propios y 11 pendientes respecto a `main`. El mapa conserva su referencia histórica `86441ad4d375e31737dfcead0b1fd9cd52161883`. Son identidades distintas, que deben mantenerse explícitas.

## 5. Custodia, reconstrucción y mantenimiento

Se conservan [dos comprobaciones de recuperación en Rust](CUSTODIA-RECUPERACION-V1.json): la incorporación principal y su complemento. Incluyen rutas, bytes, SHA-256 y revisiones de GitHub. La comprobación es de custodia material; no constituye una auditoría externa ni una recepción científica del contenido.

Las copias secundarias prescindibles sólo se retiran cuando su identidad coincide con el contenido recuperado. Los originales distintos de una copia adaptada, las fuentes activas y las evidencias únicas quedan preservados. El [registro de conservación](CONSERVACION-V1.json) identifica las rutas de GitHub que sustentan esa retirada, sin datos de acceso ni información privada.

Para reconstruir un conjunto se debe fijar su revisión, recuperar sus fuentes y manifiestos, resolver las dependencias declaradas y verificar la construcción correspondiente. En particular:

1. Algunas pruebas unitarias del cliente remiten a respuestas SSE reservadas de las primeras conexiones. Hace falta preparar y verificar muestras públicas equivalentes para reproducir esas pruebas sin divulgar información operativa.
2. Las realizaciones históricas de Safeguard dependen de proyectos y entornos específicos. Deben resolverse esas referencias antes de afirmar una construcción autónoma.
3. Los controladores extraídos para un informe deben cotejarse con sus módulos y la estructura de su proyecto. La extracción documental no acredita por sí sola compilación independiente.
4. Los visores deben reconstruirse desde la edición correspondiente y contrastarse con el conjunto adjudicado y su manifiesto. La existencia de un HTML no prueba fidelidad visual o interacción correcta.
5. La eventual integración de una rama experimental requiere su propio análisis, contrato y recepción. Este inventario no la ejecuta ni la aprueba.

Las [limitaciones de esta conservación](https://github.com/juantoniolloretegea/SV-motor/blob/02d5aee5fafe71105e2a451273adba888e87308a/laboratorio/ensayo-ia-y-observabilidad/ALCANCE-PRESERVACION-V1-20261007.md) forman parte de la lectura del manifiesto. No se incluyen historiales personales, credenciales, claves reservadas de examen, expedientes económicos privados, utilidades administrativas ajenas al recorrido del SV ni cachés de compilación. Los registros de gasto conservan su sede privada independiente.

## 6. Relación con calidad y condición de continuación

Los [sucesos del SV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/2c400a1c53657690c1767ec294f5d8d5b61aa67a/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md), los [tiques técnicos](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/2c400a1c53657690c1767ec294f5d8d5b61aa67a/docs/calidad/Inventario-sv/tiques-tecnicos/TIQUES_TECNICOS.csv) y las actas competentes conservan la autoridad sobre el estado de cada realización. La V.1 es un inventario transversal de sedes, versiones y dependencias; no crea un registro paralelo de admisiones ni reemplaza esos instrumentos.

El mapa histórico `MAPA.html` conserva su identidad Git **`24e25dbf1255ddf25d5f1e7eefa8d89d4b12ed3e`**, correspondiente a 3.703.604 bytes. No se modifica su contenido, sus diagramas ni sus referencias. La carpeta de este inventario se encuentra dentro de `mapa`, junto a sus documentos existentes, sin trasladarlos.

La continuación técnica consiste en acreditar reconstrucción reproducible de los conjuntos necesarios, resolver las dependencias pendientes y devolver cada resultado a su sede y revisión competente. La reorganización definitiva del código deberá tratarse expresamente cuando corresponda; esta V.1 permite mantener la orientación y la custodia durante ese intervalo.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
