# Vectores adversariales SEC.0 — V1

**Fecha:** 22/08/2026  
**Estado:** catálogo inicial independiente de la implementación  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Este documento conserva escenarios adversariales derivados de SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T de forma independiente del lenguaje de implementación.

Los vectores aquí definidos no constituyen una realización del Lenguaje SV, una ejecución de prueba ni una certificación material. Su finalidad es fijar condiciones falsables que puedan transformarse posteriormente en pruebas contra un sistema sometido a prueba (`SUT`) exacto.

La existencia de un vector en este catálogo **no constituye cobertura**. Mientras el vector no se ejecute de forma falsable contra el `SUT` exacto, con alcance causal acreditado y criterio esperado admisible, la propiedad correspondiente permanece `NO_PROBADO`.

Cada vector conserva al menos:

```text
precondición
→ alteración o fallo ejercido
→ evidencia mínima de alcance sobre el objetivo
→ resultado contractual esperado
```

## 2. Reglas de reutilización

1. El vector no presupone Python, Rust ni una plataforma concreta.
2. El resultado esperado procede de los contratos SEC.0, no de la conducta observada del `SUT`.
3. Una representación local de autoridad, continuidad, independencia, persistencia o atestación no sustituye la propiedad material correspondiente.
4. Una clase de prueba aplicable no puede desaparecer por mera declaración del ejecutor o del `SUT`.
5. Las pruebas locales pueden ejercitar una parte del vector, pero la acreditación material final queda reservada al sistema completo cuando la propiedad dependa de componentes externos al backend.
6. Si un observador altera la condición cuyo fallo se intenta medir, el resultado sólo es transferible al sistema realmente observado cuando esa instrumentación forma parte expresa de su identidad o existe equivalencia acreditada.
7. `ReachedFaults`, `Observed` y `Oracle` no pueden descansar únicamente en una afirmación, registro o código producido por el mismo componente sometido a la misma clase de fallo cuando ese fallo pueda falsear también dicha evidencia.
8. Cuando no pueda acreditarse de forma suficiente que el fallo alcanzó la dependencia objetivo, el caso no cubre la propiedad y queda `NO_EJECUTADO`, `NO_PROBADO` o `INCONCLUSO`, según corresponda.
9. Los controles positivos deben ejecutarse sobre la misma identidad de `SUT`, garantía y perfil material que los negativos cuya batería pretenden completar.
10. Un efecto positivo debe ser materialmente observable dentro del alcance declarado; un no-op, una mera ausencia de rechazo o un registro interno de «éxito» no bastan cuando el mismo fallo pueda falsearlos.

## 3. Vectores SEC.0-A — autoridad, constitución y génesis

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-A-01` | operación protegida dependiente de autoridad | aportar información o una verificación favorable e intentar obtener autoridad inexistente | el efecto solicitado depende de una autoridad que no existía antes de la comprobación | efecto bloqueado; la verificación no constituye autoridad |
| `V-A-02` | forma de transición previamente constituida | reclasificar localmente la forma o sustituir su descriptor para hacer admisible un efecto distinto | la ejecución alcanza el punto en que se decide la clase o familia de efectos | efecto bloqueado; la clase no es discrecional |
| `V-A-03` | autoridad con `E_max` y `D_a` delimitados | intentar ejercer un efecto u objeto fuera de la envolvente o dominio gobernado | el efecto propuesto queda materialmente fuera del alcance constituido | efecto bloqueado; no hay ampliación mediante T-I/T-V/T-H/T-E |
| `V-A-04` | autoridad válida para ejercer un efecto | intentar delegar o constituir autoridad derivada sin T-G/T-C válida | la operación crea o transporta facultad a un nuevo titular o ámbito | delegación bloqueada; capacidad de ejercicio no equivale a gobierno |
| `V-A-05` | forma repetible o acumulable | repetir o componer actos individualmente admisibles hasta producir un efecto global no constituido | la traza acumulada excede el contrato de singularidad, idempotencia o acumulación | nuevo ejercicio bloqueado antes del exceso |
| `V-A-06` | efecto protegido que sólo puede producirse mediante una forma constituida | presentar una forma inexistente, no constituida o materialmente distinta de su descriptor admitido | la operación alcanza el punto de selección de forma | efecto bloqueado; una representación técnica no constituye `F` |
| `V-A-07` | recuperación de una autoridad anterior | reconstruir un objeto técnico antiguo e intentar atribuirle autoridad nueva o más amplia | la autoridad resultante excede la autoridad preexistente o carece de continuidad acreditable | T-R bloqueada; recuperación no crea ni amplía autoridad |
| `V-A-08` | proceso, cuenta o componente con capacidad material para producir un efecto | ejercer esa capacidad sin autoridad SV aplicable | se acredita capacidad técnica para producir el efecto, pero ausencia de autoridad constituida correspondiente | efecto no legítimo; capacidad material no equivale a autoridad |
| `V-A-09` | `D_a` previamente gobernado | hacer que una T-E produzca un objeto nuevo y utilizar su mera producción para incorporarlo a `D_a` | el objeto no pertenecía al conjunto portador gobernado antes del ejercicio y la incorporación depende del propio T-E | el nuevo objeto no entra en `D_a` por ejecución ordinaria; la ampliación exige T-G o T-C |
| `V-A-10` | ejercicio automático que exige decidir `x ∈ D_a` antes del efecto | presentar un caso cuya pertenencia no pueda decidirse con las premisas constituidas disponibles | la decisión de pertenencia permanece sin base suficiente antes del compromiso | no hay T-E automática; corresponde bloqueo técnico conforme a SEC.0-D hasta gobierno o constitución suficiente |
| `V-A-11` | autoridad o autorización ligada a fase, ITI, régimen o constitución concretos | migrar, copiar o declarar compatible la autorización bajo otro contexto sin relación de compatibilidad previamente gobernada | la reutilización sólo resulta posible por la nueva compatibilidad introducida | la migración no transporta autoridad por sí sola; exige T-G o T-C cuando confiere capacidad de reutilización |
| `V-A-12` | agente, servicio o componente que carece de autoridad constituyente sobre sí mismo | hacer que su propia admisión, verificación, arranque o registro constituya la autoridad necesaria para gobernarse o ampliarse | el mismo sujeto o componente aporta la condición suficiente para legitimar el acto que crea su autoridad | constitución bloqueada; no existe acreditación propia válida |

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
| `V-D-08` | dos evidencias admitidas o verificadores aplicables producen resultados incompatibles | escoger el resultado favorable, una mayoría o una precedencia no constituida | la decisión depende del conflicto y no existe regla de resolución previamente constituida | `D-N`; el conflicto no se resuelve de forma oportunista |
| `V-D-09` | una obligación queda en `D-A` sobre estado o revisión concretos | modificar entre comprobación y efecto una dimensión material de `Req(F,e | C)` | el punto de compromiso recibe un estado distinto del acreditado | el `D-A` anterior no autoriza el efecto; nueva comprobación o `D-N` |
| `V-D-10` | resultado `D-A` persistido para reutilización posterior | restaurar o conservar el resultado y cambiar después el contexto, revisión, vigencia o dependencia ligada | el resultado persistido pertenece a un contexto distinto del vigente | el resultado almacenado no se reutiliza como acreditación actual; nueva comprobación o `D-N` |

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
| `V-M-09` | decisión negativa basada en ausencia | consultar un índice, vista o réplica de cobertura incompleta y concluir que no existe revocación, consumo u otro hecho autoritativo | se demuestra que la estructura consultada no cubre todo `AStore` pertinente | la ausencia no queda acreditada; `D-N` para el efecto dependiente |
| `V-M-10` | copia de seguridad, reducción o compactación del historial | eliminar una dependencia incluida en `PDep(d | C)` y usar el resultado para recuperar o decidir | la operación ya no permite distinguir estados semánticos materialmente distintos | la copia o reducción no acredita continuidad ni decisión para ese alcance |
| `V-M-11` | forma que requiere `Budget(F | C)` | proporcionar un presupuesto tautológico, por ejemplo «hasta agotar recursos», o sin cota materialmente decidible | no puede decidirse antes del exceso si el consumo permanece autorizado | ejercicio automático bloqueado |
| `V-M-12` | `Budget(F | C)` ya constituido y consumo próximo al límite | sustituir durante la ejecución el presupuesto por otro más amplio sin transición de gobierno o constitución válida | el mismo acto habría excedido el presupuesto previamente aplicable | el presupuesto anterior no se amplía por parámetro local; operación bloqueada |
| `V-M-13` | autoridad consumible protegida frente a clonación o retroceso bajo un fallo constituido | recrear un testigo local vacío o cambiar localmente la clase de fallo para volver a presentar la autoridad como no consumida | el segundo ejercicio comparte el mismo antecedente y el fallo relevante no ha cambiado legítimamente | segundo ejercicio bloqueado o no verificable |
| `V-M-14` | una única autoridad consumible y dos ejecutores concurrentes | lanzar dos consumos simultáneos sin clonación previa | ambas operaciones alcanzan materialmente el punto de compromiso de la misma unidad de autoridad | como máximo un efecto puede acreditarse; la carrera no duplica consumo |
| `V-M-15` | decisión cuya vigencia depende de una fuente temporal declarada | retroceder, sustituir o bifurcar la fuente temporal y reutilizar una decisión supuestamente vigente | la decisión temporal depende del reloj alterado y no existe evidencia suficiente de monotonicidad o continuidad frente al fallo | no se infiere vigencia por la marca temporal; nueva acreditación o `D-N` |
| `V-M-16` | recuperación de estado autoritativo | validar la legitimidad del estado recuperado únicamente mediante un testigo, clave, índice o metadato que puede retroceder o clonarse indistinguiblemente con ese mismo estado | el mismo fallo controla simultáneamente el estado y su única prueba de legitimidad | recuperación no acreditable; se requiere una dependencia suficiente frente al mismo fallo |

## 6. Vectores SEC.0-X — ejecución material, TCB, arranque, atestación e independencia

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-X-01` | garantía cuyo modelo incluye compromiso o sospecha de la raíz | proponer nueva raíz legitimada sólo por la raíz saliente | la raíz saliente es precisamente parte del fallo ensayado | recuperación bloqueada sin vía independiente frente al mismo fallo |
| `V-X-02` | garantía `G` con `TCB(G)` declarado | introducir, omitir o eliminar de la definición un componente capaz de falsificar `G` | puede demostrarse causalmente que el componente altera la verdad de `G` | la garantía no queda acreditada mientras el falsificador permanezca fuera de `TCB(G)` |
| `V-X-03` | operación dependiente de estado vivo o revisión actual | repetir una atestación antigua o suprimir la obligación de actualidad | la evidencia corresponde a un estado previo materialmente distinto | `D-N` o rechazo; la evidencia antigua no acredita el estado actual |
| `V-X-04` | compensación o testigo invocado como independiente | situarlo bajo la misma causa de compromiso que el componente observado | el mismo fallo puede falsear simultáneamente objeto y evidencia | la independencia no queda acreditada frente a ese fallo |
| `V-X-05` | decisión humana sobre objeto presentado | cambiar objeto, revisión o consecuencia entre presentación y firma | la firma queda ligada a una revisión distinta de la presentada | la autorización no se transfiere al objeto posterior |
| `V-X-06` | mediador que pretende controlar todo efecto protegido | utilizar una vía material alternativa capaz de producir el mismo efecto sin atravesar el control gobernado | se acredita de forma específica que la vía ensayada alcanza el recurso o efecto protegido sin pasar por el mediador | mediación incompleta; garantía fallida salvo forma alternativa gobernada con garantías propias |
| `V-X-07` | código fuente revisado y artefacto ejecutable | alterar cadena de construcción, dependencia, artefacto distribuido o artefacto cargado | el ejecutable efectivo puede diferir causalmente de la fuente acreditada | fuente pública por sí sola no acredita ejecutable |
| `V-X-08` | reserva de recursos para rechazo, registro o recuperación | agotar por la misma vía recursos ordinarios y los supuestamente reservados | el atacante puede impedir también la función de control | no existe aislamiento frente a ese fallo; garantía no acreditada |
| `V-X-09` | artefacto que arranca correctamente | asociarlo a estado persistente cuya continuidad, revocaciones o procedencia no son acreditables | el proceso arranca pero `PDep` o continuidad del estado no pueden justificarse | arranque no equivale a `AdmittedExecutionState`; efectos dependientes bloqueados |
| `V-X-10` | varias aprobaciones, firmas o testigos | hacer que compartan la misma dependencia capaz de falsearlos simultáneamente | se identifica una causa común de compromiso dentro del fallo ensayado | la pluralidad no acredita independencia frente a ese fallo |
| `V-X-11` | canal autenticado hacia un extremo identificado | sustituir o comprometer el proceso situado detrás del extremo sin romper la autenticación del canal | el canal sigue autenticado pero el proceso receptor no está en estado admitido | autenticación de canal no acredita integridad ni admisión del proceso receptor |
| `V-X-12` | observador utilizado para acreditar una garantía | someter observador y objeto a la misma causa de compromiso | la causa ensayada puede falsear simultáneamente estado y observación | la observación no acredita la garantía frente a ese fallo |
| `V-X-13` | recuperación ante una clase de fallo constituida | aportar una prueba de independencia válida sólo frente a otra clase de fallo | la evidencia no excluye la causa activa que comprometió la garantía | recuperación no acreditable; la independencia debe referirse al mismo fallo pertinente |
| `V-X-14` | garantía cuyo efecto final puede ser alterado desde fuera del proceso | utilizar una vía de host, hipervisor, acceso directo a memoria, gestión fuera de banda, depuración, inyección dinámica, parcheo en vivo, código nativo externo o mecanismo equivalente | la vía concreta ensayada puede modificar o evitar causalmente el punto de imposición de `G` | la vía entra en `TCB(G)` o la garantía no puede afirmarse frente a su compromiso |

`V-X-06` y `V-X-14` son familias de ataque. La ejecución sobre una vía concreta sólo aporta evidencia sobre esa vía y sobre las equivalencias que hayan sido acreditadas; no demuestra por enumeración la ausencia de todas las vías laterales posibles.

## 7. Vectores SEC.0-T — falsabilidad, aplicabilidad y evidencia

| ID | Precondición | Alteración adversarial | Evidencia mínima de alcance | Resultado esperado |
|---|---|---|---|---|
| `V-T-01` | caso que declara cubrir un invariante | ejecutar una mutación que no alcance la dependencia objetivo | una evidencia admisible de alcance no acredita modificación del objetivo | el caso no cubre el invariante; nunca `PASS` por mera ejecución |
| `V-T-02` | vector asociado a un invariante | cambiar únicamente la etiqueta de `Targets` sin modificar el fallo causal ensayado | el fallo ejercido no viola la propiedad correspondiente al nuevo identificador | no existe cobertura del invariante reetiquetado |
| `V-T-03` | prueba falsable con resultado esperado | hacer que el `SUT` produzca también el criterio de corrección o asignar un veredicto incompatible con `Expected` y `Observed` | `Oracle` depende del mismo componente sometido al fallo o `Verdict` no se deriva de la comparación | evidencia no admisible como `PASS` |
| `V-T-04` | prueba de carrera, orden, recursos o tiempo | instrumentar de forma que el observador elimine o transforme el fallo | la conducta cambia al introducir la instrumentación | el resultado sólo cubre el `SUT` efectivamente ensayado; no se transfiere sin equivalencia acreditada |
| `V-T-05` | `SUT` con capacidad material relevante | omitir esa capacidad de la descripción para excluir su clase de prueba | una observación suficiente demuestra que la capacidad existe | la clase sigue siendo aplicable |
| `V-T-06` | evidencia pública utilizada frente a un fallo | permitir que el mismo fallo reescriba `SUT` y evidencia | la causa ensayada controla ambos extremos | la evidencia puede conservar valor de laboratorio, pero no acredita conformidad pública frente a ese fallo |
| `V-T-07` | afirmación de conformidad sobre un conjunto de propiedades | ejecutar sólo un subconjunto y presentar el resultado como cobertura total | existen propiedades aplicables sin caso falsable ejecutado o en `NO_PROBADO`, `NO_EJECUTADO` o `INCONCLUSO` | la afirmación completa de conformidad no es admisible |
| `V-T-08` | existe un `FAIL` confirmado para una realización y alcance concretos | acumular ejecuciones posteriores satisfactorias sin cierre causal | la violación original sigue vigente y no existe cierre conforme a SEC.0-T | el `FAIL` permanece |
| `V-T-09` | una realización posee evidencia previa de conformidad para un perfil exacto | añadir persistencia, administración, recuperación, comunicaciones, privilegios, una nueva dependencia material o cualquier capacidad causalmente relevante y conservar el sello anterior | la nueva capacidad altera `Capabilities(SUT,G)`, `TCB(G)`, el perfil material o las clases aplicables | la evidencia anterior no se transfiere automáticamente; el nuevo `SUT` mantiene `NO_PROBADO` en las propiedades afectadas hasta nueva comprobación |

## 8. Controles positivos mínimos

Los controles positivos comprueban que una realización no satisface el régimen de seguridad mediante bloqueo indiscriminado. **No pueden combinarse resultados de perfiles diferentes para construir una conformidad única.**

### `V-P-01` — ejercicio protegido legítimo

- **Identidad:** mismo `SUT`, garantía `G`, perfil material, configuración y dependencias relevantes que las pruebas negativas de la batería correspondiente.
- **Precondición:** forma constituida, autoridad vigente y aplicable, efecto dentro de `E_max` y `D_a`, `Req(F,e | C)` no vacío con todas sus obligaciones en `D-A`, presupuesto y continuidad acreditados cuando procedan.
- **Ejercicio:** solicitar el efecto exactamente dentro de la envolvente constituida.
- **Alcance y observación:** debe acreditarse que la solicitud alcanza el punto de compromiso y que el efecto contractual observable se produce. Un no-op, un código interno de éxito o un registro del mismo componente sometido al fallo no bastan cuando puedan ser falseados por éste.
- **Resultado:** el efecto legítimo se produce conforme a su semántica y ligaduras; el fallo cerrado no se convierte en rechazo indiscriminado.

### `V-P-02` — recuperación o actualización legítima

- **Identidad:** mismo `SUT`, garantía `G`, perfil material y dependencias relevantes que los ataques de recuperación o actualización asociados.
- **Precondición:** forma de gobierno o recuperación previamente constituida, autoridad específica, requisitos acreditados y vía independiente frente al fallo cuando éste la exija.
- **Ejercicio:** realizar la transición dentro del alcance autorizado.
- **Alcance y observación:** debe acreditarse la transición efectiva y el estado resultante mediante evidencia admisible para el fallo ensayado.
- **Resultado:** la transición legítima se produce sin ampliar silenciosamente autoridad, reducir el modelo de fallos ni rebajar las garantías aplicables.

## 9. Vectores integrales prioritarios

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

### `V-I-06` — raíz comprometida y aprobación humana bajo causa común

- **Combinación:** A + X + T.
- **Ataque:** utilizar una aprobación humana como supuesto testigo independiente de recuperación mientras la presentación, identidad, canal o evidencia de firma dependen de la misma raíz o causa de compromiso ensayada.
- **Alcance:** se demuestra que el fallo puede controlar simultáneamente la raíz comprometida y aquello que la persona ve, firma o acredita.
- **Resultado:** la aprobación no constituye independencia frente a ese fallo; recuperación o sustitución permanecen no acreditables.

## 10. Matriz de vectorización y estado probatorio

La tabla siguiente es una **matriz de correspondencia**, no una matriz de cobertura. En el estado actual no existe ejecución soberana de estos vectores; por tanto, las clases enumeradas permanecen `NO_PROBADO` respecto de una futura realización hasta que se satisfaga SEC.0-T.

| Origen | Clase mínima o condición falsable | Vector(es) de referencia | Estado probatorio actual |
|---|---|---|---|
| A | creación de autoridad por información/verificación | `V-A-01` | `NO_PROBADO` |
| A | ampliación de `E_max` mediante ejercicio ordinario | `V-A-03` | `NO_PROBADO` |
| A | ampliación de `D_a` mediante T-E | `V-A-09` | `NO_PROBADO` |
| A | reclasificación interesada | `V-A-02` | `NO_PROBADO` |
| A | forma no constituida | `V-A-06` | `NO_PROBADO` |
| A | repetición sin contrato de acumulación | `V-A-05` | `NO_PROBADO` |
| A | delegación fuera de gobierno válido | `V-A-04` | `NO_PROBADO` |
| A | recuperación que inventa o amplía autoridad | `V-A-07` | `NO_PROBADO` |
| A | capacidad material confundida con autoridad | `V-A-08` | `NO_PROBADO` |
| A | dominio no decidible para T-E automática | `V-A-10` | `NO_PROBADO` |
| A | migración o compatibilidad que amplía reutilización | `V-A-11` | `NO_PROBADO` |
| A | autoconstitución del agente o componente | `V-A-12` | `NO_PROBADO` |
| D | requisitos vacíos u omisión nuclear | `V-D-01` | `NO_PROBADO` |
| D | `D-N` convertido en éxito o `U` | `V-D-02` | `NO_PROBADO` |
| D | eliminación de `D-R` por indisponibilidad | `V-D-03` | `NO_PROBADO` |
| D | acreditación propia | `V-D-04` | `NO_PROBADO` |
| D | comprobación caducada o cambio de contexto | `V-D-05`, `V-D-10` | `NO_PROBADO` |
| D | excepción o emergencia que omite núcleo | `V-D-06` | `NO_PROBADO` |
| D | sustitución oportunista de verificador | `V-D-07` | `NO_PROBADO` |
| D | evidencias contradictorias sin regla constituida | `V-D-08` | `NO_PROBADO` |
| D/M | cambio entre comprobación y efecto | `V-D-09` | `NO_PROBADO` |
| M | vista no autoritativa como fuente decisoria | `V-M-01` | `NO_PROBADO` |
| M | presupuesto ausente o tautológico | `V-M-02`, `V-M-11` | `NO_PROBADO` |
| M | exceso acumulado | `V-M-03` | `NO_PROBADO` |
| M | saturación de atención humana | `V-M-04` | `NO_PROBADO` |
| M | clonación y doble consumo | `V-M-05` | `NO_PROBADO` |
| M | retroceso de revocación/consumo | `V-M-06` | `NO_PROBADO` |
| M | bifurcación o selección por puntero/índice | `V-M-07` | `NO_PROBADO` |
| M | efecto externo incierto y reejecución ciega | `V-M-08` | `NO_PROBADO` |
| M | índice incompleto y falsa ausencia | `V-M-09` | `NO_PROBADO` |
| M | copia/reducción que elimina `PDep` | `V-M-10` | `NO_PROBADO` |
| M | presupuesto ampliado durante la ejecución | `V-M-12` | `NO_PROBADO` |
| M | recreación de testigo o reetiquetado del fallo | `V-M-13` | `NO_PROBADO` |
| M | doble consumo concurrente | `V-M-14` | `NO_PROBADO` |
| M | retroceso o sustitución de fuente temporal | `V-M-15` | `NO_PROBADO` |
| M | recuperación circular del estado | `V-M-16` | `NO_PROBADO` |
| X | rotación de raíz comprometida | `V-X-01` | `NO_PROBADO` |
| X | `TCB(G)` incompleto o reescrito | `V-X-02` | `NO_PROBADO` |
| X | atestación antigua u omisión de actualidad | `V-X-03` | `NO_PROBADO` |
| X | compensación con causa de fallo común | `V-X-04` | `NO_PROBADO` |
| X | cambio entre presentación y firma | `V-X-05` | `NO_PROBADO` |
| X | vía privilegiada o lateral que evita mediación | `V-X-06`, `V-X-14` | `NO_PROBADO` |
| X | cadena de construcción o ejecutable alterado | `V-X-07` | `NO_PROBADO` |
| X | aislamiento ficticio de recursos | `V-X-08` | `NO_PROBADO` |
| X | arranque sobre estado persistente ilegítimo | `V-X-09` | `NO_PROBADO` |
| X | falsa independencia de aprobaciones | `V-X-10` | `NO_PROBADO` |
| X | canal autenticado hacia proceso no admitido | `V-X-11` | `NO_PROBADO` |
| X | observador y observado bajo causa común | `V-X-12` | `NO_PROBADO` |
| X | independencia referida al fallo equivocado | `V-X-13` | `NO_PROBADO` |
| A/X | raíz y aprobación humana bajo el mismo fallo | `V-I-06` | `NO_PROBADO` |
| T | mutación sin alcance causal | `V-T-01` | `NO_PROBADO` |
| T | reetiquetado de `Targets` | `V-T-02` | `NO_PROBADO` |
| T | oráculo circular o veredicto libre | `V-T-03` | `NO_PROBADO` |
| T | instrumentación que transforma el fallo | `V-T-04` | `NO_PROBADO` |
| T | capacidad material omitida para reducir aplicabilidad | `V-T-05` | `NO_PROBADO` |
| T | evidencia pública controlada por el mismo fallo | `V-T-06` | `NO_PROBADO` |
| T | cobertura parcial presentada como total | `V-T-07` | `NO_PROBADO` |
| T | lavado de un `FAIL` por ejecuciones posteriores | `V-T-08` | `NO_PROBADO` |
| T | cambio de perfil o capacidad con herencia del sello anterior | `V-T-09` | `NO_PROBADO` |
| T | operaciones legítimas que deben avanzar | `V-P-01`, `V-P-02` | `NO_PROBADO` |

## 11. Qué cuenta como alcance causal

Para una ejecución concreta debe establecerse una relación verificable:

```text
Target contractual
→ dependencia material o lógica que lo implementa
→ fallo o mutación introducido
→ evidencia de que la alteración alcanzó esa dependencia
→ observación del efecto relevante
```

No basta:

- citar el identificador del vector;
- afirmar que una ruta «fue probada»;
- disponer de un registro interno del mismo componente sometido al fallo;
- demostrar alcance sobre una vía y transferirlo a otras vías no equivalentes;
- observar un resultado final sin acreditar que la mutación llegó al punto causal que pretendía atacar.

Para familias como `V-X-06` y `V-X-14`, cada vía material pertinente —administrativa, recuperación, host, hipervisor, depuración, acceso directo, inyección dinámica u otra— requiere alcance propio o una equivalencia acreditada capaz de detectar diferencias causales.

Cuando el propio `SUT` genere un registro útil para `ReachedFaults`, dicho registro sólo podrá formar parte de la evidencia si el modelo de fallos permite justificar que la misma causa ensayada no puede falsificar simultáneamente el objetivo y ese registro. En caso contrario debe añadirse observación suficiente desde otro dominio de fallo o conservarse el estado `INCONCLUSO`/`NO_PROBADO`.

## 12. Condición para convertir un vector en prueba ejecutable

Un vector sólo debe transformarse en una prueba concreta cuando exista un `SUT` identificable y un mecanismo suficiente para declarar y conservar, conforme a SEC.0-T:

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

`ReachedFaults` acredita el alcance efectivo sobre la dependencia objetivo. `Verdict` debe derivarse de `Expected` y `Observed`; no constituye un dato libre. `Oracle` no puede depender circularmente del componente sometido a la misma clase de fallo.

La identidad de `SUT` incluye las capacidades, garantías, perfil material, configuración y dependencias relevantes. No pueden agregarse resultados de identidades diferentes para obtener una conformidad que ninguna de ellas haya satisfecho por separado.

Cuando una propiedad dependa de almacenamiento, recuperación, infraestructura, raíz de confianza, administración, aislamiento u otra dependencia externa al proceso, la ejecución local del backend sólo proporciona evidencia parcial. La acreditación final requiere ejercer el vector sobre el sistema completo dentro del modelo de fallos declarado.

## 13. Evolución de capacidades y transferencia de evidencia

Añadir, retirar o sustituir una capacidad materialmente relevante puede cambiar `ApplicableClass(c | SUT,G)` aunque el binario permanezca idéntico.

En particular, incorporar persistencia, recuperación, una interfaz administrativa, un hipervisor, una nueva vía de actualización, un canal externo, una fuente temporal, una raíz o una dependencia capaz de producir efectos protegidos crea una diferencia de `SUT` cuando pueda afectar causalmente a una garantía.

La evidencia anterior no se transfiere por identidad de binario. Las propiedades afectadas permanecen `NO_PROBADO` hasta nueva comprobación sobre la identidad resultante.

## 14. Vigencia

Este catálogo es independiente de la implementación y puede ampliarse con nuevas regresiones o escenarios siempre que cada incorporación derive de una obligación contractual vigente y conserve un criterio falsable.

El historial del repositorio conserva las realizaciones experimentales anteriores. Su existencia histórica no les atribuye estatuto de backend, entorno de ejecución ni mecanismo material de seguridad.
