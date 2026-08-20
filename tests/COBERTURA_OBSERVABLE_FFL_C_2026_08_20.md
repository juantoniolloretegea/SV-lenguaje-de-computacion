# Cobertura observable de FFL-C — 20/08/2026

## 1. Objeto

Este documento presenta un inventario de la evidencia persistida en `tests/` para la etapa frontal del compilador y distingue entre código diagnóstico existente, emisión alcanzable desde un archivo `.svp` y cobertura explícita mediante casos de prueba.

No modifica la gramática, la IR, el validador ni el catálogo diagnóstico.

## 2. Estado material de la batería

Tras incorporar `supervise_systemtarget_valido.svp`, la carpeta de conformidad contiene:

- 10 casos válidos, cada uno con su archivo `.expected.json`;
- 48 casos inválidos con código esperado declarado en `tests/run_conformance.py`.

La ejecución completa de este estado debe verificarse antes de declarar un nuevo resultado agregado de conformidad.

Los 48 casos inválidos cubren explícitamente 37 de los 47 códigos del catálogo efectivo. La diferencia entre número de casos y número de códigos se debe a que varios diagnósticos disponen de más de un caso discriminante.

## 3. Supervisable y E006

La cobertura persistida de `Supervisable` comprende:

- `CellTarget(EvalResult)` y `ComposedTarget(GateResult)` en `supervise_targets.svp`;
- `SystemTarget(CompositionGraph)` en `supervise_systemtarget_valido.svp`;
- incompatibilidad de tipo de los tres constructores mediante E006;
- referencia inexistente de `CellTarget` mediante E006.

`tests/run_e006_characterization.py` comprueba de forma separada que E006 se emite actualmente en dos situaciones observables distintas:

1. referencia inexistente;
2. referencia existente de tipo incompatible.

Esta caracterización no cambia el significado declarado del diagnóstico ni resuelve la deuda de precisión de su nombre y mensaje.

## 4. Códigos sin caso inválido de extremo a extremo

Diez códigos del catálogo efectivo no disponen de un caso `.svp` inválido que los emita de forma directa en `tests/run_conformance.py`.

| Código | Situación observable actual | Tratamiento en FFL-C |
|---|---|---|
| `E003 — NSquaredViolation` | `n` se deriva de `b²`; no existe una entrada superficial que declare un `n` contradictorio | No inventar un caso inalcanzable |
| `E004 — EmptyCodomain` | existe una comprobación en el validador, pero la gramática exige al menos un valor antes de alcanzar esa comprobación | Distinguir existencia de la comprobación y alcanzabilidad desde `.svp` |
| `E008 — ConnectorTargetNotTri` | el código no constituye la salida efectiva del subcaso vigente; el destino no ternario queda protegido por E104 | Mantener la distinción registrada |
| `E106 — MissingSemanticRelation` | la relación del grafo es obligatoria y una referencia ausente o incompatible sigue otra ruta diagnóstica | No atribuir emisión directa inexistente |
| `E111 — UnorderedCodomain` | la superficie v0.1 no representa el orden requerido para producir este diagnóstico; `max` y `min` están prohibidos por otra ruta | No ampliar la superficie para fabricar cobertura |
| `E201 — LiteralResultForbidden` | no existe producción superficial para construir directamente los objetos de resultado afectados | Considerarlo no alcanzable en la superficie vigente |
| `E203 — GateMissingTable` | `gate` exige sintácticamente `using`; la ausencia no alcanza una emisión específica de E203 | No confundir presencia en el catálogo con emisión observable |
| `E301 — FrameMutationForbidden` | no existe operación superficial de mutación de `Frame` | La inmutabilidad se preserva por la forma del lenguaje, sin caso de mutación |
| `E302 — TrajectoryMutationForbidden` | no existe operación superficial para modificar, eliminar o reordenar una trayectoria ya declarada | La propiedad se preserva por la forma del lenguaje, sin caso de mutación |
| `E501 — SerializationNonDeterministic` | el serializador no emite E501; el determinismo se comprueba positivamente mediante repetición y comparación de salidas | Mantener separadas propiedad comprobada y emisión diagnóstica |

## 5. Consecuencia para la lectura de cobertura

La cifra total de casos superados no debe interpretarse como cobertura exhaustiva de todos los códigos ni de todos los juicios de la IR. La evidencia debe leerse en tres niveles separados:

1. **caso persistido:** existe una entrada concreta y una expectativa verificable;
2. **emisión observable:** el procesamiento completo puede alcanzar el diagnóstico indicado;
3. **propiedad estructural:** una obligación puede estar preservada porque la superficie no ofrece la operación que permitiría vulnerarla, sin que por ello exista un diagnóstico alcanzable.

FFL-C debe cerrar únicamente cuando estos tres niveles queden descritos sin atribuir cobertura donde sólo existe imposibilidad estructural o una ruta diagnóstica distinta.
