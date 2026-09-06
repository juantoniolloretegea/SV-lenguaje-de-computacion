# SV-lenguaje-de-computacion

**Actualización de esta presentación técnica:** 5 de septiembre de 2026  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

---

## Lenguaje de computación del Sistema Vectorial SV

Este repositorio contiene la especificación pública y las realizaciones verificables del Lenguaje SV (SVP): gramática, representación intermedia (IR), perfiles fuente, etapa frontal de referencia, núcleo Rust, destinos nativo y WebAssembly, pruebas de conformidad, documentación arquitectónica y registros de calidad.

Los fundamentos matemáticos y semánticos del Sistema Vectorial SV se mantienen en [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica). Este repositorio desarrolla su expresión como lenguaje de computación dentro del alcance acreditado por sus especificaciones y pruebas.

### Accesos directos

- **[Puerta operativa obligatoria para agentes](./AGENTS.md)** — exige identificar el corte, leer el rector y detener cualquier cambio que lo contradiga o pretenda completarlo por inferencia.
- **[Pilares y restricciones de diseño del Lenguaje de Computación SV](./docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md)** — pieza rectora: fija lo que el núcleo debe preservar y rechazar, y lo que tiene prohibido decidir por el dominio o por el agente.
- [Entorno público del Lenguaje SV](https://lenguaje-sv.itvia.online/) — compilación Rust/WebAssembly local en el navegador.
- [Gramática superficial mínima v0.2](./GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md).
- [Especificación normativa de perfiles fuente SVP-ES / SVP-EN](./ESPECIFICACION_NORMATIVA_PERFILES_FUENTE_SVP_ES_EN_v1_2026_08_29.md).
- [Representación intermedia y bienformación v0.3](./IR_CANONICA_BIENFORMACION_SV_v0_3.md).
- [Cierre técnico de R1](./docs/arquitectura/ACTA_TECNICA_CIERRE_R1_2026_08_25.md).
- [Apertura de R2](./docs/arquitectura/ACTA_TECNICA_APERTURA_R2_PERSISTENCIA_Y_CONTINUIDAD_MATERIAL_2026_08_25.md).
- [Cierre correctivo B2 y restauración de continuidad](./docs/calidad/ACTA_TECNICA_DE_CONFORMIDAD_CIERRE_CORRECTIVO_B2_Y_RESTAURACION_CONTINUIDAD_2026_08_29.md).
- [Historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md).
- [Documentación de calidad](./docs/calidad/README.md).

---

## Estado técnico vigente

| Elemento | Versión o estado | Alcance acreditado |
|---|---|---|
| Gramática canónica | **0.2** | Gramática común aplicada tras resolver el perfil fuente explícito. |
| Perfiles fuente | **SVP-ES · SVP-EN** | Dos representaciones fuente cerradas que convergen sobre una misma identidad canónica. |
| Perfil léxico | **`svp-grammar-0.2-lex-es-1`** | Repertorio de identificadores; es independiente de los perfiles fuente. |
| Representación intermedia | **0.3** | IR canónica común, independiente del idioma de fuente. |
| Pilares de diseño | **RECTOR_DE_DISENO** | El núcleo valida y preserva contratos constituidos; no elige células, tamaños, asignaciones ni cobertura de agentes. |
| Serializador canónico de referencia | **0.1.0** | JSON determinista de la implementación Python de referencia. |
| Proyección diferencial Rust | **0.1.0** | Observable compartido por Rust nativo y WebAssembly; no sustituye al serializador canónico completo. |
| Núcleo Rust | **`sv_core`** | Implementación compartida por los destinos nativo y WebAssembly. |
| Entorno público | **Rust / WebAssembly** | <https://lenguaje-sv.itvia.online/>. |
| Conformidad vigente | **80/80** | 12 casos válidos y 68 inválidos, incluido N0-01. |
| `sv_core` | **210/210 + N0-01 3/3** | Suite interna y pruebas de integración específicas de unicidad de `Codomain`. |
| Dominios cerrados | **5/5 + 6/6 navegador** | Regresiones permanentes y sondas DG-01/02/03 en SVP-ES y SVP-EN. |
| `sv_wasm` | **2/2** | Adaptador WebAssembly sobre el mismo núcleo. |
| Documentación ejecutable `sv_core` | **17/17** | Pruebas de documentación Rust. |
| R0 | **CERRADO** | Incluido el perímetro correctivo abierto por DFL-007. |
| R1 | **CERRADO Y REVALIDADO** | Autoridad, mediación, decisiones protegidas y trazas intra-proceso sobre la base R0 corregida. |
| R2 | **ABIERTO** | Persistencia y continuidad material; levantada la suspensión específica causada por DFL-007. |
| R3–R4 | **NO INICIADOS** | Conservan sus fases propias. |
| Garantía I / Garantía II | **NO_PROBADO** | Ningún cierre anterior acredita estas garantías. |

El levantamiento de la suspensión de R2 no constituye cierre de R2 ni prueba propiedades materiales que pertenecen a esa fase.

---

## Una gramática canónica y dos perfiles fuente

La arquitectura vigente distingue el perfil léxico de los perfiles fuente:

```text
bytes UTF-8 de la unidad
        ↓
perfil léxico común de identificadores
        ↓
perfil fuente explícito SVP-ES o SVP-EN
        ↓
identidad canónica de forma constitutiva
        ↓
Gramática canónica 0.2
        ↓
IR canónica 0.3
        ↓
semántica única del Lenguaje SV
```

`SVP-ES` y `SVP-EN` no crean dos gramáticas, dos representaciones intermedias ni dos semánticas. La realización vigente contiene **154 identidades canónicas**, **297 grafías distintas** y **11 formas compartidas**.

La selección de perfil fuente es explícita. No existe autodetección ni caída silenciosa entre perfiles. La canonicalización no traduce identificadores del usuario, cadenas, comentarios, datos ni nombres de archivo, y la huella `source_sha256` se calcula sobre los bytes UTF-8 originales.

---

## Identidad de la realización WebAssembly publicada

El corte de realización integrado es:

```text
main de realización
c1acf943a7a44ce81080881e59283de8a2019606
```

La identidad del WebAssembly desplegado es:

```text
sv_wasm.wasm
bytes   = 378956
SHA-256 = 95c7d1e0313567ef099c6e426a7fcee8ff4a5ac8adb670265f859f1bf03caab3
```

La distribución estática utilizada para el despliegue manual queda identificada por:

```text
SV_LENGUAJE_PRODUCCION_B2_CLOUDFLARE_2026-08-29_FINAL_CONFORMIDAD.zip
bytes   = 167503
SHA-256 = 566200f97bfea86a0b7ce7c4919bac9d5367a67b8cba719eef1c573942d696f5
```

La distribución contiene una única representación Base64 comprimida del módulo. La aplicación comprueba identidad y tamaño antes de utilizar el WebAssembly.

Cloudflare constituye la capa de distribución del entorno público, no una autoridad semántica independiente. La compilación del texto `.svp` se realiza localmente en el navegador mediante el mismo `sv_core` utilizado por el destino Rust nativo.

---

## Conformidad y corrección DFL-007

La ampliación de la verificación durante B2 descubrió tres dominios cerrados que Rust trataba como palabras abiertas:

```text
SemanticRelation.kind
Pattern.kind
Graph.regime
```

La corrección vigente exige:

```text
SemanticRelation.kind = DeclaredRelation
Pattern.kind          = DeclaredPattern
Graph.regime          ∈ {Simple, General}
```

La comprobación se aplica sobre la identidad canónica común, por lo que una única regla protege SVP-ES y SVP-EN. Las sondas de regresión quedan incorporadas de forma permanente en Rust y en la prueba WebAssembly de navegador.

La deuda distinta relativa a `ConflictOperator` y a la concurrencia bajo régimen `General` permanece abierta. No forma parte del cierre de DG-03.

Evidencia principal:

- [PR #55](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/55);
- [R0 Rust — ejecución 33271992363](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992363);
- [R0-8 — ejecución 33271992371](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992371);
- [R0 WebAssembly y navegador — ejecución 33271992457](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/actions/runs/33271992457).

---

## Referencia Python y varios destinos materiales

Python permanece como referencia diferencial y conserva los oráculos y parte del contrato diagnóstico histórico. Rust constituye la realización soberana compartida por el destino nativo y WebAssembly.

```text
fuente .svp
   ↓
perfil fuente explícito
   ↓
sv_core
   ├── Rust nativo
   └── WebAssembly de navegador
```

El adaptador WebAssembly no introduce una segunda gramática ni reglas semánticas independientes. JavaScript transporta bytes, selecciona de forma explícita el perfil solicitado, invoca las exportaciones del módulo y presenta el observable resultante.

El Playground Python/Pyodide anterior se conserva como [instantánea histórica](./docs/historico/PLAYGROUND_PYTHON_PYODIDE_2026_08_24.md).

---

## Límites vigentes

Las comprobaciones publicadas deben leerse dentro de su alcance material:

- la proyección diferencial Rust 0.1.0 no es el serializador canónico completo;
- no se acredita paridad textual exacta de todos los códigos y mensajes diagnósticos entre realizaciones;
- una ejecución en Chromium no acredita compatibilidad universal con todos los motores de navegador;
- una entrada no admitida o un fallo técnico no se convierten en `Tri.U`;
- el despliegue público no prueba por sí solo ninguna garantía de fase;
- `ConflictOperator`/J2.3 para concurrencia en régimen `General` permanece pendiente;
- R2 sigue abierto y debe acreditar sus propias propiedades de persistencia y continuidad material;
- Garantía I y Garantía II permanecen `NO_PROBADO`.

La deuda técnica vigente se mantiene en [`docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md`](./docs/calidad/REGISTRO_DEUDA_VIVA_DEL_FRENTE_FINAL_DEL_LENGUAJE_SV.md).

---

## Calidad, trazabilidad e historial

La documentación pública de Calidad se encuentra en [`docs/calidad/`](./docs/calidad/). El [historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md) distingue versiones normativas, realizaciones, entornos públicos, correcciones y estados de continuidad.

El historial de Git conserva el detalle mecánico de los cambios. Los registros de Calidad concentran los hitos materiales y no sustituyen las pruebas ni las especificaciones que fundamentan cada afirmación.

La verificación externa independiente del corte final se registrará mediante un acta separada cuando se complete; no se presume por la existencia del presente cierre de conformidad.

---

## Ecosistema SV

| Repositorio | Función |
|---|---|
| [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica) | fundamentos matemáticos y semánticos |
| [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion) | especificación y realizaciones del Lenguaje SV |
| [SV-motor](https://github.com/juantoniolloretegea/SV-motor) | infraestructura de ejecución e integración |
| [SVcustos-dataset](https://github.com/juantoniolloretegea/SVcustos-dataset) | conjuntos de datos y sede pública de realizaciones Beta cuando corresponda |
| [SVperitus-dataset](https://github.com/juantoniolloretegea/SVperitus-dataset) | agentes especializados y conjuntos de datos asociados |
| [SV-banco-de-idiomas](https://github.com/juantoniolloretegea/SV-banco-de-idiomas) | infraestructura lingüística auxiliar |

---

## Archivo histórico

La presentación pública anterior al entorno Rust/WebAssembly se conserva íntegramente en [`docs/historico/README_2026_08_24_PRE_ENTORNO_RUST_WASM.md`](./docs/historico/README_2026_08_24_PRE_ENTORNO_RUST_WASM.md).
