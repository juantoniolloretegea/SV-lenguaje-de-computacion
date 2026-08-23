# SV-lenguaje-de-computacion

**Actualización de esta presentación técnica:** 23 de agosto de 2026  
**Versión del conjunto:** V.1  
**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0

---

## Lenguaje de computación del Sistema Vectorial SV

Este repositorio es la sede operativa y técnica del Lenguaje SV (SVP). Reúne la especificación pública de la superficie del lenguaje, su representación intermedia canónica, la etapa frontal de referencia, la batería de conformidad y la documentación necesaria para distinguir con precisión qué está definido, qué está materializado y qué permanece pendiente.

La sede doctrinal superior del Sistema Vectorial SV se mantiene en [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica). Este repositorio no sustituye aquella autoridad: materializa en lenguaje de computación un alcance gobernado de la doctrina y del álgebra del SV.

La cadena técnica es:

```text
doctrina y matemática
        ↓
especificación del lenguaje
        ↓
etapa frontal de referencia
        ↓
IR canónica
        ↓
diagnóstico y conformidad reproducible
```

---

## Estado técnico vigente

| Elemento | Estado | Enlace |
|---|---|---|
| Frontera normativa v0 | Contrato normativo de base | [FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md](./FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md) |
| Gramática superficial mínima v0.2 | Sucesora normativa de v0.1 para admisibilidad, `resolve` y coherencia de `Frame` | [GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md](./GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md) |
| IR canónica v0.3 | Sucesora normativa de v0.2 en el mismo radio | [IR_CANONICA_BIENFORMACION_SV_v0_3.md](./IR_CANONICA_BIENFORMACION_SV_v0_3.md) |
| Catálogo efectivo de errores v0.3 | Catálogo del frontend, con E110, E305 y E308 | [docs/referencia/ERRORES_CANONICOS_SV_v0_3.md](./docs/referencia/ERRORES_CANONICOS_SV_v0_3.md) |
| Etapa frontal de referencia | Analizador sintáctico, AST, validación, descenso a IR y serialización | [`src/`](./src/) |
| Batería de conformidad | 72 casos: 11 válidos y 61 inválidos | [`tests/conformance/`](./tests/conformance/) |
| Registro técnico de la actualización | Evidencia, alcance y límites de las tres correcciones | [docs/calidad/ACTUALIZACION_FRONTEND_REFERENCIA_2026_08_23.md](./docs/calidad/ACTUALIZACION_FRONTEND_REFERENCIA_2026_08_23.md) |
| Biblioteca estándar | Desarrollo separado | [`stdlib/`](./stdlib/) |

Las versiones v0.1 de la gramática y v0.2 de la IR se conservan como antecedentes públicos. No se reescriben retrospectivamente. Las versiones v0.2 y v0.3 incorporan por referencia todo lo no modificado y sustituyen únicamente las reglas que declaran de forma expresa.

La presentación pública anterior del repositorio, que contenía además un anexo arquitectónico extenso, se conserva sin alteración como [instantánea histórica de 19/08/2026](./docs/historico/README_2026_08_19.md). Cuando exista discrepancia sobre el estado actual del frontend, prevalece esta presentación vigente junto con las especificaciones versionadas enlazadas arriba.

---

## Tres precisiones constitutivas del frontend vigente

### 1. Fallo técnico o inadmisibilidad no son `Tri.U`

El alfabeto semántico permanece:

```text
Tri = {0, 1, U}
```

`U` es un valor semántico ternario y no un contenedor para fallos técnicos, ausencia de captura, inadmisibilidad o trabajo pendiente.

La admisibilidad superficial v0.2 utiliza exactamente:

```text
{Ok, Degraded, NotAdmitted}
```

`Bottom` sigue siendo el símbolo técnico de fallo de captura de `CaptureSpec`; no forma parte de `Tri` ni de los estados de `AdmissibilitySpec`. Una observación `NotAdmitted` no produce por defecto un nuevo valor ternario. Una observación admitida puede, en cambio, ser clasificada legítimamente como `U` por un `Ternarizer` cuando pertenece a su partición `B_U`.

### 2. `resolve` revisa una `U` constituida e identificable

La forma superficial vigente identifica el objetivo mediante estado y posición:

```svp
let RR1 = resolve((S1, 3), with: RS1,
                  context: ContextoClinico,
                  mechanism: RevisionExperto);
```

La posición es uno-basada, debe existir y debe contener efectivamente `U`. El contexto y el mecanismo de la instancia deben ser compatibles con el `ResSpec` nombrado. La revisión computacional no adquiere por sí sola autoridad para fabricar una clausura positiva de una `U` genuina.

### 3. `Frame` conserva cierre estructural y causal

La sintaxis de `Frame` no cambia, pero su bienformación v0.3 exige que las colecciones declaradas pertenezcan al mismo cierre estructural y causal:

- como máximo un `CoupledState` por nodo de arquitectura;
- nodos distintos pueden compartir el mismo `CellSpec` mediante `CoupledSpec` distintos;
- cada `EvalResult` del `Frame` debe evaluar uno de sus estados y no puede duplicar materialmente la misma fuente;
- cada `GateResult` debe depender sólo de evaluaciones del mismo `Frame`;
- `supervision` sólo puede referir evaluaciones, compuestos o arquitectura pertenecientes al mismo `Frame`;
- mientras la superficie no disponga de un productor constituido de `CriticalityResult`, `criticalities` debe permanecer vacío.

Estas reglas exigen coherencia de lo declarado, no exhaustividad: un `Frame` no tiene que contener todas las evaluaciones, compuertas o supervisiones posibles.

---

## Cadena de procesamiento de referencia

```text
archivo .svp
   ↓
análisis léxico
   ↓
análisis sintáctico
   ↓
AST
   ↓
validación de bienformación
   ↓
IR canónica v0.3
   ↓
JSON canónico · serializador 0.1.0
```

La etapa frontal no constituye por sí sola una infraestructura soberana de ejecución del SV. Producir IR conforme no acredita ejecución material, persistencia resistente, raíz de confianza ni resistencia adversarial del sistema completo.

Los módulos principales son:

| Archivo | Función |
|---|---|
| `src/svp_lexer.py` | análisis léxico |
| `src/svp_parser.py` | análisis sintáctico y AST |
| `src/svp_ast.py` | tipos sintácticos |
| `src/svp_validator.py` | bienformación materializada |
| `src/svp_ir.py` | descenso a IR v0.3 |
| `src/svp_serialize.py` | JSON canónico determinista |
| `src/svp_errors.py` | catálogo diagnóstico efectivo |
| `src/svp_main.py` | interfaz de línea de órdenes |

---

## Conformidad reproducible

La batería vigente contiene:

```text
casos válidos   = 11
casos inválidos = 61
total           = 72
```

Se ejecuta mediante:

```bash
python tests/run_conformance.py
```

Los casos válidos comparan la IR emitida con JSON canónicos comprometidos en el repositorio. Los casos inválidos deben terminar con el código diagnóstico esperado. El flujo de integración continua ejecuta la conformidad contra esos oráculos y comprueba además que la ejecución no los modifique.

Los nuevos diagnósticos del radio v0.2/v0.3 son:

```text
E110  InvalidAdmissibilitySpec
E305  UnsafeUResolution
E308  FrameClosureViolation
```

La evidencia de conformidad no equivale a una garantía material del backend o del sistema completo.

---

## Deuda técnica que permanece visible

La actualización no abre un cuarto impacto semántico.

```text
ConflictOperator / J2.3 = obligación normativa conservada
cobertura superficial completa de régimen General = pendiente
E204 canónico de IR y E204 efectivo del frontend = divergencia conocida
```

En el catálogo efectivo, `E204` continúa significando `QueryMissingContext`; no materializa el `E204 — MissingConflictOperator` de la IR histórica. Esta divergencia se conserva de forma explícita hasta su resolución gobernada y no autoriza a fabricar `ConflictOperator` implícitamente.

También permanecen fuera del núcleo actual del Lenguaje objetos de gobierno, plataforma y despliegue como TCB, raíces de confianza, atestación, continuidad autoritativa y perfiles materiales. Su importancia arquitectónica no demuestra necesidad de convertirlos en sintaxis o IR.

---

## Límites de esta versión

- `max` y `min` no son construcciones superficiales.
- `PendingU` permanece reconocido pero no habilitado en la superficie vigente.
- no existe authoring superficial completo de `ConflictOperator` para régimen `General`;
- `CriticalityResult` no dispone todavía de productor superficial;
- la etapa frontal no es un backend soberano;
- este repositorio no declara por el mero resultado de la batería una garantía integral de construcción ni de resistencia adversarial del sistema completo.

---

## Ecosistema SV

| Repositorio | Función |
|---|---|
| [SV-matematica-semantica](https://github.com/juantoniolloretegea/SV-matematica-semantica) | sede doctrinal y matemática |
| [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion) | especificación y frontend del Lenguaje SV |
| [SV-motor](https://github.com/juantoniolloretegea/SV-motor) | infraestructura de ejecución e integración subordinada a la doctrina |
| [SVcustos-dataset](https://github.com/juantoniolloretegea/SVcustos-dataset) | conjuntos de datos y antecedentes de seguridad estructural |
| [SVperitus-dataset](https://github.com/juantoniolloretegea/SVperitus-dataset) | agentes especializados y conjuntos de datos asociados |
| [SV-banco-de-idiomas](https://github.com/juantoniolloretegea/SV-banco-de-idiomas) | infraestructura lingüística auxiliar |

---

*Documento técnico público del Lenguaje SV. Describe el estado verificable del repositorio y no sustituye la doctrina matemática superior del Sistema Vectorial SV.*
