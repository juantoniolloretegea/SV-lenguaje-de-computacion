# Matriz de correspondencia y alcance causal de vectores SEC.0 — V1

**Fecha:** 22/08/2026  
**Estado:** referencia de diseño de pruebas; sin cobertura ejecutada  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Esta matriz relaciona cada vector de `VECTORES_ADVERSARIALES_SEC0_V1.md` con la propiedad contractual que pretende falsificar y con la evidencia mínima necesaria para acreditar que el fallo o mutación alcanzó la dependencia objetivo.

La matriz no acredita ninguna propiedad por sí sola. Mientras no exista ejecución sobre un `SUT` exacto conforme a SEC.0-T, todos los objetivos aquí enumerados permanecen `NO_PROBADO`.

La relación que deberá conservar cada ejecución es:

```text
propiedad contractual
→ vector
→ dependencia objetivo del SUT
→ alteración ejercida
→ ReachedFaults acreditado
→ Observed
→ Expected
→ Verdict
```

La mera coincidencia de nombres entre una propiedad y un vector no constituye alcance causal.

## 2. Reglas generales de alcance

1. `ReachedFaults` debe acreditar que la alteración llegó a la dependencia que materializa la propiedad concreta.
2. Un registro emitido por el mismo componente sometido al mismo fallo no basta cuando ese fallo pueda falsear simultáneamente objetivo y registro.
3. La observación de un efecto final no sustituye la prueba de que la alteración alcanzó el punto causal pretendido.
4. La evidencia obtenida sobre una vía material no se transfiere a otra vía salvo que se acredite que ambas comparten la misma dependencia causal relevante para el fallo ensayado; en otro caso, cada vía requiere `Reach` propio.
5. Cuando no exista observación suficiente, el caso permanece `NO_EJECUTADO`, `NO_PROBADO` o `INCONCLUSO`; nunca `PASS` por mera invocación.
6. `Oracle` y `Observed` deben permanecer distinguibles. El `SUT` no puede definir por sí solo el criterio que decide si su propia respuesta era correcta frente al mismo fallo.
7. Los controles positivos deben acreditar en `Observed` el cambio, emisión o consecuencia material del objeto o recurso protegido por `G`; un permiso, indicador interno, no-op, ausencia de rechazo o registro de éxito no bastan.

## 3. SEC.0-A

| Vector | Propiedad contractual | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-A-01` | información o verificación no constituyen autoridad | la operación protegida alcanza la decisión de autoridad utilizando una entrada que sólo aporta información/evidencia y carece de autoridad previa | `NO_PROBADO` |
| `V-A-02` | clase T-* no discrecional | la ejecución alcanza el punto donde una clase o familia de efectos alterada intenta sustituir el descriptor constituido | `NO_PROBADO` |
| `V-A-03` | `E_max` y `D_a` no se amplían por ejercicio | el efecto propuesto llega al control de pertenencia y se acredita que queda fuera de la envolvente o dominio previamente constituido | `NO_PROBADO` |
| `V-A-04` | ejercer no confiere derecho de gobierno | la operación llega al punto que crearía o transportaría autoridad derivada sin una T-G/T-C aplicable | `NO_PROBADO` |
| `V-A-05` | repetición respeta contrato de acumulación | la traza acumulada alcanza la primera operación cuyo agregado excedería el contrato constituido | `NO_PROBADO` |
| `V-A-06` | una forma debe estar constituida | el efecto alcanza el selector de forma con una identidad inexistente o materialmente distinta de la constituida | `NO_PROBADO` |
| `V-A-07` | T-R no crea ni amplía autoridad | la recuperación llega al punto de restablecimiento con una autoridad mayor o sin continuidad acreditable | `NO_PROBADO` |
| `V-A-08` | capacidad material no equivale a autoridad | un actor técnicamente capaz alcanza el punto de compromiso sin autoridad SV aplicable | `NO_PROBADO` |
| `V-A-09` | T-E no amplía `D_a` generando nuevos miembros | un objeto generado por T-E llega al mecanismo que decide su incorporación a `D_a` y se acredita que no pertenecía al portador gobernado antes del acto | `NO_PROBADO` |
| `V-A-10` | T-E automática exige pertenencia decidible | el punto previo al efecto recibe un caso para el que `x ∈ D_a` no puede acreditarse ni refutarse con estado admitido | `NO_PROBADO` |
| `V-A-11` | compatibilidad o migración que habilita reutilización requiere gobierno | una autorización ligada a un contexto alcanza un efecto bajo otro contexto únicamente gracias a una compatibilidad no constituida | `NO_PROBADO` |
| `V-A-12` | prohibición de autoconstitución | el mismo agente o componente alcanza el acto constitutivo usando como única base su propia admisión, verificación, arranque o registro | `NO_PROBADO` |
| `V-A-13` | T-0 no puede repetirse sobre una continuidad autoritativa ya habitada | una instancia con identidad local nueva alcanza un `AStore`, `PDep` o continuidad que ya admitió autoridad o una instancia previa e intenta escribir o ampliar autoridad invocando T-0 | `NO_PROBADO` |

## 4. SEC.0-D

| Vector | Propiedad contractual | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-D-01` | `Req` no vacío y núcleo no eludible | la decisión protegida se evalúa con una obligación nuclear ausente o con `Req = ∅` | `NO_PROBADO` |
| `V-D-02` | `D-N` bloquea y no se convierte en `U` o éxito | una obligación necesaria entra realmente en estado no verificable y ese resultado llega al punto de autorización | `NO_PROBADO` |
| `V-D-03` | `D-R` no desaparece por indisponibilidad posterior | la refutación aplicable llega a la decisión después de retirar o inutilizar el verificador | `NO_PROBADO` |
| `V-D-04` | prohibición de acreditación propia | la decisión utiliza como condición de legitimidad una acreditación producida por el mismo acto cuya validez depende de ella | `NO_PROBADO` |
| `V-D-05` | comprobación ligada a contexto y revisión | el resultado anterior llega a una nueva decisión después de cambiar una ligadura material | `NO_PROBADO` |
| `V-D-06` | emergencia o excepción no eluden núcleo | una vía alternativa alcanza el mismo efecto protegido sin autoridad o `Req` propios | `NO_PROBADO` |
| `V-D-07` | aplicabilidad de verificador constituida | un verificador no aplicable o no constituido llega a fundamentar la obligación concreta | `NO_PROBADO` |
| `V-D-08` | conflicto sin regla de resolución produce `D-N` | dos evidencias o verificadores incompatibles llegan a la agregación y no existe regla constituida de resolución | `NO_PROBADO` |
| `V-D-09` | continuidad comprobación → efecto | el estado o revisión cambia materialmente después de `D-A` y antes del punto de compromiso | `NO_PROBADO` |
| `V-D-10` | un `D-A` persistido no se independiza del contexto | el resultado almacenado se reutiliza después de restauración o cambio material de una ligadura | `NO_PROBADO` |

## 5. SEC.0-M

| Vector | Propiedad contractual | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-M-01` | una vista no sustituye `AStore` | la decisión autoritativa se intenta cerrar usando la vista mientras la fuente acreditada no está disponible | `NO_PROBADO` |
| `V-M-02` | formas aplicables requieren `Budget(F | C)` | una forma repetible, recursiva, expansiva o expuesta llega al ejercicio automático sin política constituida | `NO_PROBADO` |
| `V-M-03` | presupuesto acumulativo | la operación que produce el primer exceso acumulado alcanza el punto de admisión | `NO_PROBADO` |
| `V-M-04` | atención humana finita | la demanda acumulada alcanza el límite constituido de atención privilegiada | `NO_PROBADO` |
| `V-M-05` | clonación no duplica consumo | dos copias derivadas del mismo antecedente alcanzan el intento de consumir la misma autorización | `NO_PROBADO` |
| `V-M-06` | restauración no resucita revocación o consumo | la copia antigua alcanza una decisión que depende de un hecho posterior eliminado por la restauración | `NO_PROBADO` |
| `V-M-07` | continuidad no se decide por criterio local no gobernado | al menos dos continuaciones localmente válidas alcanzan el selector de continuidad | `NO_PROBADO` |
| `V-M-08` | efecto externo incierto no se reejecuta a ciegas | el fallo ocurre después de la emisión y antes de poder distinguir materialmente éxito de ausencia; la recuperación alcanza el punto de reejecución | `NO_PROBADO` |
| `V-M-09` | ausencia exige cobertura suficiente | una decisión negativa se intenta cerrar desde una estructura cuya cobertura incompleta está acreditada | `NO_PROBADO` |
| `V-M-10` | reducción conserva `PDep` | el artefacto reducido o copia de seguridad alcanza una decisión que exige una dependencia eliminada | `NO_PROBADO` |
| `V-M-11` | presupuesto no tautológico | una política sin cota decidible alcanza la admisión de ejercicio automático | `NO_PROBADO` |
| `V-M-12` | presupuesto no se amplía localmente | la ejecución sustituye `Budget(F | C)` y alcanza una operación que habría excedido la política previa | `NO_PROBADO` |
| `V-M-13` | testigo recreado o fallo reetiquetado no restaura consumo | un segundo intento alcanza el punto de consumo con el mismo antecedente después de recrear representación o cambiar etiqueta de fallo | `NO_PROBADO` |
| `V-M-14` | doble consumo concurrente | dos ejecuciones simultáneas alcanzan materialmente el punto de compromiso de la misma unidad consumible | `NO_PROBADO` |
| `V-M-15` | fuente temporal no autoritativa por mera marca | una decisión dependiente del tiempo alcanza el punto de vigencia usando una fuente retrocedida, sustituida o bifurcada | `NO_PROBADO` |
| `V-M-16` | recuperación no circular | el estado y su única prueba de legitimidad quedan simultáneamente bajo el mismo retroceso, clonación o sustitución y alcanzan la decisión de recuperación | `NO_PROBADO` |

## 6. SEC.0-X

| Vector | Propiedad contractual | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-X-01` | raíz comprometida no legitima sola a su sucesora | la raíz saliente sometida al fallo alcanza el punto que admitiría la nueva raíz como única prueba | `NO_PROBADO` |
| `V-X-02` | cierre de `TCB(G)` | un componente omitido o retirado de `TCB(G)` modifica causalmente la verdad de `G` | `NO_PROBADO` |
| `V-X-03` | actualidad de atestación | evidencia antigua alcanza una decisión dependiente del estado vivo después de un cambio material | `NO_PROBADO` |
| `V-X-04` | independencia relativa al mismo fallo | el fallo ensayado puede alterar simultáneamente el objeto y el supuesto testigo independiente | `NO_PROBADO` |
| `V-X-05` | fidelidad presentación-firma | el objeto o revisión cambia entre presentación y acto de autorización y el efecto posterior usa la autorización anterior | `NO_PROBADO` |
| `V-X-06` | mediación completa | la vía material concreta ensayada alcanza el recurso o efecto protegido sin atravesar el punto gobernado equivalente | `NO_PROBADO` |
| `V-X-07` | fuente revisada no acredita artefacto ejecutado | una alteración de construcción, distribución o carga alcanza el artefacto efectivo y cambia conducta relevante para `G` | `NO_PROBADO` |
| `V-X-08` | aislamiento material de recursos | la misma causa de agotamiento impide tanto trabajo ordinario como la función de control que se declaraba reservada | `NO_PROBADO` |
| `V-X-09` | arranque no equivale a estado admitido | el proceso arranca sobre un estado persistente cuya continuidad o procedencia no puede acreditarse y alcanza una operación dependiente | `NO_PROBADO` |
| `V-X-10` | pluralidad no implica independencia | una dependencia común bajo el fallo ensayado alcanza simultáneamente todas las aprobaciones o testigos | `NO_PROBADO` |
| `V-X-11` | canal autenticado no acredita proceso | el canal sigue autenticado mientras el proceso efectivo del extremo cambia a un estado no admitido | `NO_PROBADO` |
| `V-X-12` | observador limitado por su dominio de fallo | la causa ensayada alcanza simultáneamente objeto observado y mecanismo de observación | `NO_PROBADO` |
| `V-X-13` | independencia debe corresponder al fallo pertinente | la prueba aportada excluye otra clase de fallo pero no la causa activa que falsifica `G` | `NO_PROBADO` |
| `V-X-14` | dependencias externas capaces de evitar el punto de imposición entran en `TCB(G)` | la vía externa concreta modifica causalmente el recurso final o evita el control cuya completitud se afirma | `NO_PROBADO` |

`V-X-06` y `V-X-14` exigen alcance por vía concreta. Una ejecución sobre una vía sólo puede transferirse a otra si se acredita que ambas comparten la misma dependencia causal relevante para el fallo ensayado; de no ser así, cada vía requiere `Reach` propio.

## 7. SEC.0-T

| Vector | Propiedad contractual | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-T-01` | una inyección sin alcance no cubre | la mutación se ejecuta pero una evidencia admisible demuestra que no llegó a la dependencia objetivo | `NO_PROBADO` |
| `V-T-02` | `Targets` no se reetiqueta | el mismo fallo se asocia nominalmente a otro objetivo sin cambiar la dependencia realmente alterada | `NO_PROBADO` |
| `V-T-03` | oráculo no circular y veredicto derivado | la misma causa puede controlar resultado y criterio, o el veredicto contradice `Expected`/`Observed` | `NO_PROBADO` |
| `V-T-04` | instrumentación declarada | la instrumentación altera una condición causal y transforma o elimina el fallo | `NO_PROBADO` |
| `V-T-05` | aplicabilidad deriva de capacidad efectiva | una capacidad material observable existe aunque la descripción del perfil la omita | `NO_PROBADO` |
| `V-T-06` | evidencia pública protegida frente al mismo fallo | el fallo ensayado puede reescribir simultáneamente `SUT` y artefacto de evidencia invocado públicamente | `NO_PROBADO` |
| `V-T-07` | cobertura parcial no es conformidad completa | existe al menos una propiedad aplicable sin ejecución falsable suficiente y la afirmación intenta abarcarla | `NO_PROBADO` |
| `V-T-08` | `FAIL` no se lava por acumulación | la violación original permanece causalmente abierta y se intenta neutralizar con ejecuciones posteriores satisfactorias | `NO_PROBADO` |
| `V-T-09` | cambio causal de perfil invalida transferencia automática | una nueva capacidad o dependencia altera `Capabilities(SUT,G)`, `TCB(G)` o clases aplicables y se reutiliza evidencia de la identidad anterior | `NO_PROBADO` |

## 8. Controles positivos

| Vector | Propiedad | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-P-01` | una operación legítima no debe quedar bloqueada indiscriminadamente | la misma identidad de `SUT/G/perfil` alcanza el punto de compromiso y `Observed` acredita el cambio, emisión o consecuencia material del objeto o recurso protegido por `G`; un permiso, indicador interno o registro no sustituyen ese efecto | `NO_PROBADO` |
| `V-P-02` | una recuperación o actualización legítima debe poder avanzar | la transición autorizada alcanza materialmente el estado resultante esperado bajo la misma identidad y garantías declaradas | `NO_PROBADO` |

No puede utilizarse un control positivo de un perfil para completar la afirmación de otro perfil.

## 9. Escenarios integrales

| Vector | Interacción | Qué cuenta como `Reach` mínimo | Estado actual |
|---|---|---|---|
| `V-I-01` | `D-N` + vía de emergencia | la vía alternativa alcanza el mismo efecto protegido después del bloqueo ordinario | `NO_PROBADO` |
| `V-I-02` | clonación + consumo + retroceso | al menos dos continuaciones del mismo antecedente alcanzan el intento de producir un segundo efecto | `NO_PROBADO` |
| `V-I-03` | raíz comprometida + recuperación | la causa de compromiso controla la raíz saliente y la evidencia usada para proponer la recuperación | `NO_PROBADO` |
| `V-I-04` | agotamiento + funciones de control | el mismo agotamiento alcanza trabajo ordinario y función de rechazo, registro, revocación o recuperación | `NO_PROBADO` |
| `V-I-05` | actualización + mediación | la vía de actualización alcanza artefacto, configuración o recurso protegido evitando el control ordinario | `NO_PROBADO` |
| `V-I-06` | raíz comprometida + aprobación humana | la misma causa de fallo alcanza la raíz y el camino que determina lo presentado, firmado o acreditado por la persona | `NO_PROBADO` |

## 10. Condición de vigencia

Esta matriz deberá evolucionar junto con el catálogo. Un nuevo vector no podrá considerarse suficientemente especificado hasta que identifique una condición de alcance capaz de discriminar entre:

```text
fallo invocado pero no alcanzado
≠
fallo materialmente alcanzado
```

La futura ejecución deberá conservar la evidencia concreta utilizada para establecer esa distinción. Ninguna fila de esta matriz constituye cobertura mientras no exista dicha ejecución.
