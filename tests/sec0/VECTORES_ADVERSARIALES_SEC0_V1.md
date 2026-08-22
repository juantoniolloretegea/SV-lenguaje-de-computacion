# Vectores adversariales SEC.0 — V1

**Fecha:** 22/08/2026  
**Estado:** catálogo inicial independiente de la implementación  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Este documento conserva escenarios adversariales derivados de SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T de forma independiente del lenguaje de implementación.

Los vectores aquí definidos no constituyen una realización del Lenguaje SV ni una certificación material. Su finalidad es servir como referencia estable para construir pruebas contra cualquier realización posterior, incluido el backend soberano en Rust y, cuando exista una realización suficientemente completa, el sistema material integrado.

Cada vector debe conservar cuatro elementos:

```text
precondición
→ alteración o fallo ejercido
→ evidencia de alcance sobre el objetivo
→ resultado contractual esperado
```

Una ejecución sólo puede utilizarse como cobertura si puede acreditarse que el fallo o la mutación alcanzó el objetivo declarado y que el criterio esperado no depende circularmente del componente sometido al mismo fallo.

## 2. Reglas de reutilización

1. El vector no presupone Python, Rust ni una plataforma concreta.
2. El resultado esperado procede de los contratos SEC.0, no de la conducta observada del sistema sometido a prueba.
3. Una representación local de autoridad, continuidad, independencia, persistencia o atestación no sustituye la propiedad material correspondiente.
4. Una clase de prueba aplicable no puede desaparecer por mera declaración del ejecutor o del sistema sometido a prueba.
5. Las pruebas locales pueden ejercitar una parte del vector, pero la acreditación material final queda reservada a la realización completa cuando el contrato dependa de componentes externos al backend.
6. Si un observador altera la condición cuyo fallo se intenta medir, el resultado sólo es transferible al sistema realmente observado cuando esa instrumentación forma parte expresa de su definición.
7. Los controles positivos son necesarios para comprobar que el fallo cerrado no se materializa como bloqueo indiscriminado de operaciones legítimas.

## 3. Vectores SEC.0-A — autoridad, constitución y génesis

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-A-01` | operación protegida dependiente de autoridad | aportar información o una verificación favorable e intentar obtener autoridad inexistente | el efecto solicitado depende de una autoridad que no existía antes de la comprobación | efecto bloqueado; la verificación no constituye autoridad |
| `V-A-02` | forma de transición previamente constituida | reclasificar localmente la forma o sustituir su descriptor para hacer admisible un efecto distinto | la ejecución alcanza el punto en que se decide la clase o familia de efectos | efecto bloqueado; la clase no es discrecional |
| `V-A-03` | autoridad con `E_max` y `D_a` delimitados | introducir un efecto u objeto fuera de la envolvente o dominio gobernado | el efecto propuesto queda materialmente fuera del alcance constituido | efecto bloqueado; no hay ampliación mediante T-I/T-V/T-H/T-E |
| `V-A-04` | autoridad válida para ejercer un efecto | intentar delegar o constituir autoridad derivada sin T-G/T-C válida | la operación crea o transporta facultad a un nuevo titular o ámbito | delegación bloqueada; capacidad de ejercicio no equivale a gobierno |
| `V-A-05` | autoridad consumible o acumulable | repetir o componer actos individualmente admisibles hasta producir un efecto global no constituido | la traza acumulada excede el contrato de singularidad, idempotencia o acumulación | nuevo ejercicio bloqueado antes del exceso |
| `V-A-06` | efecto protegido que sólo puede producirse mediante una forma constituida | presentar una forma inexistente, no constituida o materialmente distinta de su descriptor admitido | la operación alcanza el punto de selección de forma | efecto bloqueado; una representación técnica no constituye `F` |
| `V-A-07` | recuperación de una autoridad anterior | reconstruir un objeto técnico antiguo e intentar atribuirle autoridad nueva o más amplia | la autoridad resultante excede la autoridad preexistente o carece de continuidad acreditable | T-R bloqueada; recuperación no crea ni amplía autoridad |
| `V-A-08` | proceso, cuenta o componente con capacidad material para producir un efecto | ejercer esa capacidad sin autoridad SV aplicable | se acredita que el actor puede técnicamente producir el efecto pero no posee la autoridad constituida correspondiente | efecto no legítimo; capacidad material no equivale a autoridad |

## 4. Vectores SEC.0-D — diagnóstico y fallo cerrado

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-D-01` | forma sujeta a control | eliminar una obligación nuclear o presentar `Req = ∅` | la decisión protegida se evalúa con el requisito ausente | `D-N` o invalidez de forma; nunca permiso |
| `V-D-02` | obligación aplicable no acreditable | forzar indisponibilidad, evidencia incompleta o verificador no admisible | la obligación necesaria queda efectivamente sin base suficiente | `D-N`; efecto bloqueado; nunca `U` ni éxito |
| `V-D-03` | obligación materialmente refutada | retirar o inutilizar el verificador después de existir `D-R` | la refutación sigue siendo aplicable al mismo objeto y contexto | `D-R` se conserva; no se degrada a éxito ni a ausencia de prueba |
| `V-D-04` | verificador cuya legitimidad es necesaria | hacer que el mismo acto o componente produzca la acreditación suficiente para autorizarse a sí mismo | la decisión usa esa acreditación propia como condición de legitimidad | `D-N`; no existe acreditación propia válida |
| `V-D-05` | comprobación previa ligada a revisión, contexto o vigencia | reutilizarla después de cambiar una dimensión material | el cambio afecta una ligadura declarada de la comprobación | resultado anterior no reutilizable; nueva comprobación o `D-N` |
| `V-D-06` | forma ordinaria bloqueada por `D-N` o `D-R` | invocar emergencia, mantenimiento o excepción para producir el mismo efecto sin núcleo propio | la vía alternativa alcanza el mismo efecto protegido | bloqueo salvo forma alternativa previamente constituida con autoridad y requisitos propios |
| `V-D-07` | obligación con familia de verificadores previamente admitida | sustituir oportunistamente el verificador por otro no constituido o no aplicable para obtener un resultado favorable | la decisión utiliza el verificador sustituto como fundamento | `D-N`; disponibilidad o resultado favorable no demuestran aplicabilidad |

## 5. Vectores SEC.0-M — memoria, persistencia, recursos y continuidad

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-M-01` | decisión dependiente de estado persistente autoritativo | inutilizar `AStore` y responder desde una vista, caché o índice no autoritativo | la decisión se intenta cerrar sin acceso a la fuente acreditada | `D-N`; la vista no sustituye a la fuente |
| `V-M-02` | forma repetible, recursiva, expansiva o expuesta | ejecutar sin política de consumo constituida | la forma entra realmente en régimen repetible o acumulable | ejercicio automático bloqueado |
| `V-M-03` | presupuesto constituido | repartir un exceso total en llamadas individualmente inferiores al límite | la suma acumulada supera la cota autorizada | la operación que produce el exceso queda bloqueada |
| `V-M-04` | forma capaz de generar decisiones humanas privilegiadas | saturar atención humana mediante acumulación de solicitudes | la cola o demanda supera la política constituida | nuevas solicitudes privilegiadas quedan limitadas o bloqueadas sin ampliar autoridad automática |
| `V-M-05` | autorización de un solo uso | clonar la realización antes del consumo e intentar ejercer desde dos copias | ambas copias derivan del mismo antecedente de autoridad | como máximo un efecto puede acreditarse; el segundo debe bloquearse o quedar no verificable |
| `V-M-06` | revocación, consumo o acumulación persistente | restaurar una copia anterior autoconsistente | la restauración elimina materialmente un hecho posterior relevante | la copia no recupera vigencia por sí sola; autoridad o continuidad quedan bloqueadas hasta acreditación suficiente |
| `V-M-07` | dos continuaciones localmente válidas | seleccionar por `HEAD`, marca temporal, contador local, tamaño o velocidad de respuesta | existen al menos dos candidatos compatibles localmente | la continuidad vigente no se decide por criterio no constituido; `D-N` cuando no pueda acreditarse |
| `V-M-08` | efecto externo no necesariamente idempotente | interrumpir después de la emisión y antes de conocer si el efecto ocurrió; reiniciar y reejecutar | no existe evidencia suficiente para distinguir éxito de ausencia | no se repite automáticamente; reconciliación gobernada o estado no verificable |
| `V-M-09` | decisión negativa basada en ausencia | consultar un índice, vista o réplica cuya cobertura es incompleta y concluir que no existe revocación, consumo u otro hecho autoritativo | se demuestra que la estructura consultada no cubre todo `AStore` pertinente | la ausencia no queda acreditada; `D-N` para el efecto dependiente |
| `V-M-10` | copia de seguridad, reducción o compactación del historial | eliminar una dependencia incluida en `PDep(d | C)` y usar el resultado para recuperar o decidir | la operación ya no permite distinguir estados semánticos materialmente distintos | la copia o reducción no acredita continuidad ni decisión para ese alcance |
| `V-M-11` | forma que requiere `Budget(F | C)` | proporcionar un presupuesto tautológico, por ejemplo «hasta agotar recursos», o sin cota materialmente decidible | no puede decidirse antes del exceso si el consumo permanece autorizado | ejercicio automático bloqueado |
| `V-M-12` | `Budget(F | C)` ya constituido y consumo acumulado próximo al límite | sustituir durante la ejecución el presupuesto por otro más amplio sin la transición de gobierno o constitución correspondiente | el mismo acto habría excedido el presupuesto previamente aplicable | el presupuesto anterior no se amplía por parámetro local; operación bloqueada |
| `V-M-13` | autoridad consumible protegida frente a clonación o retroceso bajo un fallo constituido | recrear un testigo local vacío o cambiar localmente la clase de fallo para volver a presentar la autoridad como no consumida | el segundo ejercicio comparte el mismo antecedente de autoridad y el fallo relevante no ha cambiado legítimamente | segundo ejercicio bloqueado o no verificable; recrear representación o reetiquetar el fallo no restaura unicidad |

## 6. Vectores SEC.0-X — ejecución material, TCB, arranque, atestación e independencia

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-X-01` | garantía cuyo modelo incluye compromiso o sospecha de la raíz | proponer nueva raíz legitimada sólo por la raíz saliente | la raíz saliente es precisamente parte del fallo ensayado | recuperación bloqueada sin vía independiente frente al mismo fallo |
| `V-X-02` | garantía `G` con `TCB(G)` declarado | introducir, omitir o eliminar de la definición un componente capaz de falsificar `G` | puede demostrarse causalmente que el componente altera la verdad de `G` | la garantía no queda acreditada mientras el falsificador permanezca fuera de `TCB(G)` |
| `V-X-03` | operación dependiente de estado vivo o revisión actual | repetir una atestación antigua o suprimir la obligación de actualidad | la evidencia corresponde a un estado previo materialmente distinto | `D-N` o rechazo; la evidencia antigua no acredita el estado actual |
| `V-X-04` | compensación o testigo invocado como independiente | situarlo bajo la misma causa de compromiso que el componente observado | el mismo fallo puede falsear simultáneamente objeto y evidencia | la independencia no queda acreditada frente a ese fallo |
| `V-X-05` | decisión humana sobre objeto presentado | cambiar objeto, revisión o consecuencia entre presentación y firma | la firma queda ligada a una revisión distinta de la presentada | la autorización no se transfiere al objeto posterior |
| `V-X-06` | mediador que pretende controlar todo efecto protegido | utilizar vía administrativa, mantenimiento, depuración, recuperación o escritura directa | la vía alternativa produce el mismo efecto sin atravesar el control ordinario | mediación incompleta; garantía fallida salvo que la vía alternativa esté gobernada con garantías propias |
| `V-X-07` | código fuente revisado y artefacto ejecutable | alterar cadena de construcción, dependencia o artefacto cargado | el binario ejecutado puede diferir causalmente de la fuente acreditada | fuente pública por sí sola no acredita ejecutable |
| `V-X-08` | reserva de recursos para rechazo, registro o recuperación | agotar por la misma vía tanto recursos ordinarios como los supuestamente reservados | el atacante puede impedir también la función de control | no existe aislamiento frente a ese fallo; garantía no acreditada |
| `V-X-09` | artefacto que arranca correctamente | asociarlo a un estado persistente cuya continuidad, revocaciones o procedencia no son acreditables | el proceso arranca pero `PDep` o continuidad del estado no pueden justificarse | arranque no equivale a `AdmittedExecutionState`; efectos dependientes bloqueados |
| `V-X-10` | varias aprobaciones, firmas o testigos | hacer que compartan la misma dependencia capaz de falsearlos simultáneamente | se identifica una causa común de compromiso dentro del fallo ensayado | la pluralidad no acredita independencia frente a ese fallo |
| `V-X-11` | canal autenticado hacia un extremo identificado | sustituir o comprometer el proceso situado detrás del extremo sin romper la autenticación del canal | el canal sigue autenticado pero el proceso receptor no está en estado admitido | autenticación de canal no acredita integridad ni admisión del proceso receptor |
| `V-X-12` | observador utilizado para acreditar una garantía | someter observador y objeto a la misma causa de compromiso | la causa ensayada puede falsear simultáneamente estado y observación | la observación no acredita la garantía frente a ese fallo |
| `V-X-13` | recuperación ante una clase de fallo constituida | aportar una prueba de independencia válida sólo frente a otra clase de fallo | la evidencia no excluye la causa activa que comprometió la garantía | recuperación no acreditable; la independencia debe referirse al mismo fallo pertinente |

## 7. Vectores SEC.0-T — falsabilidad, aplicabilidad y evidencia

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-T-01` | caso que declara cubrir un invariante | ejecutar una mutación que no alcance la dependencia objetivo | la sonda o evidencia de alcance no acredita modificación del objetivo | el caso no cubre el invariante; `NO_EJECUTADO` o el estado técnico que corresponda; nunca `PASS` |
| `V-T-02` | vector asociado a un invariante | cambiar únicamente la etiqueta de `Targets` sin modificar el fallo causal ensayado | el fallo ejercido no viola la propiedad correspondiente al nuevo identificador | no existe cobertura del invariante reetiquetado |
| `V-T-03` | prueba falsable con resultado esperado | hacer que el SUT produzca también el criterio de corrección o asignar un veredicto incompatible con `Expected` y `Observed` | `Oracle` depende del mismo componente sometido al fallo o `Verdict` no se deriva de `Expected` y `Observed` | evidencia no admisible como `PASS`; el criterio debe ser no circular frente al mismo fallo y el veredicto debe derivarse del resultado observado |
| `V-T-04` | prueba de carrera, orden, recursos o tiempo | instrumentar de forma que el observador elimine el fallo | la conducta cambia al introducir la instrumentación | el resultado sólo cubre el SUT aumentado; no se transfiere a la realización ordinaria |
| `V-T-05` | SUT con capacidad material relevante | omitir esa capacidad de la descripción para excluir su clase de prueba | una observación independiente demuestra que la capacidad existe | la clase sigue siendo aplicable |
| `V-T-06` | evidencia pública utilizada frente a un fallo | permitir que el mismo fallo reescriba SUT y evidencia | la causa ensayada controla ambos extremos | la evidencia conserva valor de laboratorio, pero no constituye evidencia pública independiente frente a ese fallo |
| `V-T-07` | afirmación de conformidad sobre un conjunto de propiedades | ejecutar sólo un subconjunto y presentar el resultado como cobertura total | existen propiedades aplicables sin caso falsable ejecutado o con estado `NO_PROBADO`, `NO_EJECUTADO` o `INCONCLUSO` | la afirmación completa de conformidad no es admisible |
| `V-T-08` | existe un `FAIL` confirmado para una realización y alcance concretos | acumular ejecuciones posteriores satisfactorias sin corregir y cerrar el fallo conforme a SEC.0-T | la violación original sigue siendo válida y no existe cierre causal acreditado | el `FAIL` permanece; no se elimina por mayoría o acumulación estadística de `PASS` |

## 8. Controles positivos mínimos

Los controles positivos comprueban que una realización no satisface el régimen de seguridad mediante bloqueo indiscriminado.

### `V-P-01` — ejercicio protegido legítimo

- **Precondición:** forma constituida, autoridad vigente y aplicable, efecto dentro de `E_max` y `D_a`, `Req(F,e | C)` no vacío con todas sus obligaciones en `D-A`, presupuesto y continuidad acreditados cuando procedan.
- **Ejercicio:** solicitar el efecto exactamente dentro de la envolvente constituida.
- **Resultado:** el efecto puede avanzar conforme a su semántica; el fallo cerrado no convierte una operación legítima en rechazo arbitrario.

### `V-P-02` — recuperación o actualización legítima

- **Precondición:** forma de gobierno o recuperación previamente constituida, autoridad específica, requisitos acreditados y vía independiente frente al fallo cuando éste la exija.
- **Ejercicio:** realizar la transición dentro del alcance autorizado.
- **Resultado:** la transición puede avanzar sin ampliar silenciosamente autoridad ni rebajar las garantías aplicables.

## 9. Vectores integrales prioritarios

Los siguientes escenarios deben conservarse además de sus reducciones locales porque combinan obligaciones de varios contratos:

### `V-I-01` — diagnóstico no verificable y vía de emergencia

- **Combinación:** A + D.
- **Ataque:** provocar `D-N` en la forma ordinaria e intentar producir el efecto mediante una forma extraordinaria sin autoridad o requisitos propios.
- **Resultado:** el efecto permanece bloqueado.

### `V-I-02` — clonación, consumo y retroceso

- **Combinación:** A + M + X.
- **Ataque:** clonar el estado antes del consumo, ejercer una copia, restaurar o activar la otra e intentar un segundo efecto.
- **Resultado:** el sistema no puede acreditar dos ejercicios de una misma autoridad de un solo uso.

### `V-I-03` — raíz comprometida y recuperación

- **Combinación:** D + M + X.
- **Ataque:** comprometer la raíz, restaurar un estado antiguo y proponer una nueva raíz mediante evidencia controlada por la anterior.
- **Resultado:** recuperación bloqueada hasta disponer de una vía acreditable independiente frente al mismo fallo.

### `V-I-04` — agotamiento y funciones de control

- **Combinación:** D + M + X.
- **Ataque:** agotar recursos ordinarios y comprobar si el mismo agotamiento impide rechazo, registro mínimo, revocación o recuperación.
- **Resultado:** no puede producirse éxito, ampliación de autoridad ni omisión de controles; cualquier garantía de reserva exige aislamiento frente al fallo declarado.

### `V-I-05` — actualización y mediación

- **Combinación:** A + X + T.
- **Ataque:** sustituir artefacto, configuración o componente mediante una vía de actualización con capacidad para evitar el mediador ordinario.
- **Resultado:** la actualización debe estar gobernada y el nuevo estado debe volver a satisfacer las garantías aplicables; de lo contrario no existe conformidad transferible.

## 10. Cobertura de las clases mínimas de SEC.0-T

El catálogo V1 cubre las clases mínimas que SEC.0-T exige poder ensayar cuando resulten aplicables: génesis y ampliación de autoridad, dominio y formas; delegación y recuperación; requisitos, verificadores, `D-R` y `D-N`; persistencia, cobertura negativa, bifurcación, reducción, presupuestos, agotamiento y efectos externos; construcción, arranque, mediación, atestación, independencia, presentación, canales, administración y raíces; falsabilidad, aplicabilidad, instrumentación y evidencia.

La existencia de un vector no acredita que una realización concreta lo haya ejecutado. La cobertura sólo nace de una ejecución falsable con alcance acreditado sobre el SUT exacto.

## 11. Condición para convertir un vector en prueba ejecutable

Un vector de este catálogo sólo debe transformarse en una prueba concreta cuando exista un SUT identificable y un mecanismo suficiente para declarar y conservar, conforme a SEC.0-T:

```text
TestRun
SUT
TestCase
Targets
ThreatModel
InitialState
InjectedFaults
ReachedFaults
Oracle
Observer
Expected
Observed
Verdict
Artifacts
```

`ReachedFaults` debe acreditar que la alteración alcanzó la dependencia que pretendía afectar. `Verdict` debe derivarse de `Expected` y `Observed`; no constituye un dato libre. `Oracle` no puede depender circularmente del componente sometido a la misma clase de fallo.

Cuando una propiedad dependa de almacenamiento, recuperación, infraestructura, raíz de confianza, administración, aislamiento u otra dependencia externa al proceso, la ejecución local del backend sólo proporciona evidencia parcial. La acreditación final de esa propiedad requiere ejercer el vector sobre el sistema completo dentro del modelo de fallos declarado.

## 12. Vigencia

Este catálogo es independiente de la implementación y puede ampliarse con nuevas regresiones o escenarios siempre que cada incorporación derive de una obligación contractual vigente y conserve un criterio falsable.

El historial del repositorio conserva las realizaciones experimentales anteriores. Su existencia histórica no les atribuye estatuto de backend, entorno de ejecución ni mecanismo material de seguridad.
