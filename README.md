# SV-lenguaje-de-computacion

**Actualización de esta presentación técnica:** 24 de agosto de 2026  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

---

## Lenguaje de computación del Sistema Vectorial SV

Este repositorio contiene la especificación pública y las realizaciones verificables del Lenguaje SV (SVP). Reúne la gramática, la representación intermedia (IR), la etapa frontal de referencia, el núcleo Rust, los adaptadores de ejecución, el catálogo de diagnósticos, la batería de conformidad y la documentación de calidad y trazabilidad.

Los fundamentos matemáticos y semánticos del Sistema Vectorial SV se mantienen en [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica). Este repositorio desarrolla su expresión como lenguaje de computación dentro del alcance definido por sus especificaciones.

### Accesos directos

- [Entorno público del Lenguaje SV](https://lenguaje-sv.itvia.online/) — ejecución Rust/WebAssembly local en el navegador.
- [Playground Python/Pyodide histórico](./docs/historico/PLAYGROUND_PYTHON_PYODIDE_2026_08_24.md) — antecedente conservado para trazabilidad.
- [Historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md).
- [Gramática superficial mínima v0.2](./GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md).
- [Representación intermedia y bienformación v0.3](./IR_CANONICA_BIENFORMACION_SV_v0_3.md).
- [Catálogo efectivo de errores v0.3](./docs/referencia/ERRORES_CANONICOS_SV_v0_3.md).
- [Documentación de calidad](./docs/calidad/README.md).

---

## Estado técnico

| Elemento | Versión o estado | Alcance |
|---|---|---|
| Gramática superficial mínima | **0.2** | Sintaxis vigente para admisibilidad, `resolve` y `Frame`. |
| Representación intermedia | **0.3** | Estructuras y reglas de bienformación vigentes. |
| Serializador canónico de referencia | **0.1.0** | Serialización JSON determinista de la implementación Python. |
| Proyección diferencial Rust | **0.1.0** | Observable compartido por los destinos Rust nativo y WebAssembly; no sustituye al serializador canónico completo. |
| Etapa frontal Python | **Referencia diferencial** | Conserva análisis, validación, descenso a IR, diagnósticos y oráculos de conformidad. |
| Núcleo Rust | **`sv_core`** | Una sola implementación compartida por el destino nativo y WebAssembly. |
| Entorno público | **Rust / WebAssembly** | <https://lenguaje-sv.itvia.online/>; ejecución local en el navegador. |
| Conformidad | **72/72** | 11 casos válidos y 61 inválidos en la batería comprometida. |
| Paridad de tres vías | **Ejecutada** | Referencia Python, Rust nativo y WebAssembly de navegador sobre el mismo corpus; WASI se conserva como evidencia complementaria. |
| Biblioteca estándar | **Pendiente** | Estado documentado en [`stdlib/README.md`](./stdlib/README.md). |

El artefacto WebAssembly utilizado por el entorno público corresponde al corte:

```text
fuente
20a1f95cbf1bdbfb4f16cd39335bd71ca1d1c606

sv_wasm.wasm
SHA-256
7b49228624f101dc8d863a2b4d631b7ed8eacb4ee4a29c2459d32f6b63aff5dc
```

La realización Rust/WebAssembly quedó integrada mediante la [PR #22](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/pull/22) y la [confirmación `befc666`](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/commit/befc666fabe54ecd541416610bf31ddfe776aa69). La identidad del artefacto y las pruebas asociadas permiten distinguir el despliegue público de la mera presentación web.

---

## Una sola semántica, varios destinos materiales

La arquitectura de R0 mantiene una sola implementación del núcleo semántico:

```text
archivo .svp
   ↓
sv_core::compile_svp
   ├── Rust nativo
   └── WebAssembly de navegador
```

El adaptador WebAssembly no contiene una segunda gramática, un segundo analizador ni reglas semánticas independientes. La interfaz JavaScript transporta bytes, invoca las funciones exportadas del módulo y presenta el observable devuelto.

En el entorno público:

```text
texto .svp
   ↓
sv_wasm
   ↓
sv_core::compile_svp
   ↓
proyección diferencial de R0
```

La distribución mediante Cloudflare no constituye autoridad semántica: entrega los activos estáticos del entorno público; la compilación se ejecuta localmente en el navegador.

---

## Referencia Python y conservación histórica

La implementación Python no se elimina por la aparición de Rust/WebAssembly. Permanece como referencia diferencial, conserva el catálogo diagnóstico y sostiene los oráculos comprometidos de la batería de conformidad.

El Playground público anterior, basado en Python/Pyodide, dejó de ser la puerta principal del Lenguaje al desplegarse el entorno Rust/WebAssembly. Su interfaz se conserva como [instantánea histórica de 24/08/2026](./docs/historico/PLAYGROUND_PYTHON_PYODIDE_2026_08_24.md).

La presentación README previa a este relevo también se conserva íntegramente en [`docs/historico/README_2026_08_24_PRE_ENTORNO_RUST_WASM.md`](./docs/historico/README_2026_08_24_PRE_ENTORNO_RUST_WASM.md).

---

## Conformidad y paridad ejecutada

La batería vigente contiene:

```text
casos válidos   = 11
casos inválidos = 61
total           = 72
```

La comprobación WebAssembly utiliza el mismo texto `.svp` que las otras realizaciones. Para los casos válidos se contrasta la referencia Python, los oráculos JSON comprometidos, el destino Rust nativo y el destino WebAssembly de navegador. Para los casos inválidos se exige rechazo en las tres vías.

La paridad exacta del código diagnóstico `E***` y del texto de los mensajes de error no forma parte del alcance acreditado por esta comprobación.

La ejecución de navegador asociada al corte indicado produjo:

```text
válidos admitidos    = 11/11
inválidos rechazados = 61/61
```

Dos reconstrucciones independientes del mismo corte produjeron bit a bit los mismos ejecutables nativo, WASI y WebAssembly de navegador; la variación observada se limitó al registro operativo del servidor HTTP de la prueba.

---

## Fronteras y límites

Las comprobaciones anteriores no deben ampliarse más allá de su evidencia material:

- la proyección diferencial no se presenta como serializador canónico completo;
- la conformidad 72/72 acredita el corpus comprometido, no una garantía integral del sistema;
- no se acredita paridad diagnóstica exacta `E***`;
- la ejecución en un navegador real no implica compatibilidad universal con todos los motores de navegador;
- un fallo técnico o una entrada no admitida no se convierten en `Tri.U`;
- el despliegue público no acredita por sí solo las Garantías I o II;
- R0 permanece abierto mientras no exista cierre integral expreso;
- R1–R4 no se consideran iniciados por el despliegue WebAssembly.

Permanecen además las deudas técnicas registradas en la documentación de calidad, entre ellas `ConflictOperator`/J2.3, la divergencia histórica de `E204`, `RG1`, las limitaciones de `CriticalityResult` y el desarrollo pendiente de la biblioteca estándar.

---

## Calidad, trazabilidad e historial

La documentación pública de calidad se encuentra en [`docs/calidad/`](./docs/calidad/). El [historial de versiones](./docs/calidad/HISTORIAL_VERSIONES_LENGUAJE_SV.md) distingue las versiones integradas, las piezas históricas, los desarrollos pendientes y los entornos públicos registrados.

El historial de Git conserva el detalle de cada modificación. Las instantáneas históricas permiten reconstruir el estado de la presentación pública sin mantener simultáneamente dos entornos como puerta vigente.

---

## Ecosistema SV

| Repositorio | Función |
|---|---|
| [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica) | fundamentos matemáticos y semánticos |
| [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion) | especificación y realizaciones del Lenguaje SV |
| [SV-motor](https://github.com/juantoniolloretegea/SV-motor) | infraestructura de ejecución e integración |
| [SVcustos-dataset](https://github.com/juantoniolloretegea/SVcustos-dataset) | conjuntos de datos y antecedentes de seguridad estructural |
| [SVperitus-dataset](https://github.com/juantoniolloretegea/SVperitus-dataset) | agentes especializados y conjuntos de datos asociados |
| [SV-banco-de-idiomas](https://github.com/juantoniolloretegea/SV-banco-de-idiomas) | infraestructura lingüística auxiliar |

---

## Archivo histórico de la presentación anterior

La presentación pública anterior, incluido el anexo histórico que contenía, se conserva íntegramente en [`docs/historico/README_2026_08_24_PRE_ENTORNO_RUST_WASM.md`](./docs/historico/README_2026_08_24_PRE_ENTORNO_RUST_WASM.md). Su conservación permite mantener la trazabilidad documental sin trasladar a la portada vigente material que ya no describe el estado técnico actual.
