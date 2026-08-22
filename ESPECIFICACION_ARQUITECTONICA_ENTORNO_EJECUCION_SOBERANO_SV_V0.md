# Especificación arquitectónica del entorno de ejecución soberano mínimo del Lenguaje SV — v0

**Fecha:** 22/08/2026  
**Estado:** borrador arquitectónico v0  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Este documento define la frontera mínima que deberá separar el backend soberano del Lenguaje SV, orientado a Rust, de las dependencias materiales necesarias para sostener garantías SEC.0.

Su finalidad es impedir tres reducciones incorrectas:

1. identificar el backend con el sistema completo;
2. atribuir al lenguaje de implementación garantías que dependen de almacenamiento, administración, recuperación, raíces de confianza, aislamiento, comunicaciones, presentación humana u otras dependencias externas al proceso;
3. convertir autoridad, forma, requisitos, presupuesto, modelo de fallos o límites de garantía en parámetros libres que la propia ejecución pueda rebajar.

La especificación no abre todavía una implementación Rust y no selecciona sistema operativo, motor de almacenamiento, hipervisor, base de datos, mecanismo criptográfico, hardware, servicio de identidad ni plataforma de despliegue.

No modifica la semántica del Lenguaje SV, la gramática, la IR canónica, el catálogo diagnóstico ni los contratos SEC.0-A/D/M/X/T.

## 2. Antecedentes vigentes

Esta especificación se interpreta conjuntamente con:

- `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`, que fija el núcleo semántico y la condición de legitimidad del lenguaje;
- `OBJETIVO_RUST_0_BACKEND_SOBERANO.md`, que establece Rust como objetivo principal del backend soberano y excluye la dependencia de Python en destino;
- `MANIFIESTO_DE_ARQUITECTURA_DERECHOS_OBLIGACIONES_GARANTIAS_Y_FUNDAMENTOS_DEL_SISTEMA_VECTORIAL_SV_V1.md`, que fija la subordinación de capas y la separación entre laboratorio y producción;
- los contratos SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T;
- `ACTA_ARQUITECTONICA_ESTATUTO_LABORATORIO_BACKEND_SOBERANO_Y_DOBLE_GARANTIA_SV_2026_08_22.md`, que establece la doble garantía de construcción conforme y comprobación adversarial integral.

En caso de conflicto, esta pieza no puede utilizarse para rebajar una obligación doctrinal o contractual superior.

## 3. Principio de frontera

La unidad de seguridad no es el proceso Rust. La unidad de análisis es la **garantía concreta** y todas las dependencias capaces de falsificarla.

Para cada garantía `G` deberá poder establecerse:

```text
G
→ propiedad contractual
→ punto de imposición
→ componentes que pueden falsificarla
→ TCB(G)
→ ThreatModel(G)
→ Evidence(G)
→ FailureLimit(G)
→ límites declarados
```

Una propiedad no se considerará materializada porque exista una función, tipo, objeto inmutable, comprobación local o rama de código que la represente.

La propiedad sólo podrá atribuirse a una realización cuando, dentro del modelo de fallos declarado, las vías materiales capaces de producir el efecto protegido estén mediadas por mecanismos suficientes o queden expresamente fuera del alcance de la garantía.

## 4. Cadena de transformación y ejecución

La arquitectura futura deberá distinguir al menos las etapas siguientes:

```text
fuente SVP
→ etapa frontal
→ IR canónica admitida
→ backend soberano Rust
→ artefacto ejecutable
→ artefacto cargado
→ estado de ejecución admitido
→ entorno de ejecución soberano
→ efectos mediados
```

La etapa frontal de referencia puede continuar temporalmente en Python mientras permanezca vigente esa decisión arquitectónica. El artefacto soberano final no deberá requerir Python para su ejecución.

La aceptación de una etapa no acredita automáticamente la siguiente:

```text
fuente válida
≠ artefacto correcto
artefacto correcto
≠ artefacto cargado correcto
artefacto cargado correcto
≠ estado persistente legítimo
estado de ejecución admitido
≠ autoridad suficiente para todo efecto
```

## 5. Núcleo soberano en Rust

El backend y el entorno de ejecución en Rust deberán asumir únicamente las garantías que puedan imponer realmente dentro de su frontera.

Como mínimo, el núcleo soberano deberá poder preservar:

1. los tipos y distinciones semánticas obligatorias del lenguaje;
2. la representación irreductible de `Tri` sin conversiones implícitas prohibidas;
3. la correspondencia entre IR admitida y operaciones ejecutables;
4. la separación entre información, evidencia, hecho constituido, autoridad, habilitación y ejercicio cuando la realización materialice estas categorías;
5. la mediación interna de toda operación protegida que pertenezca a su perímetro;
6. el fallo cerrado ante resultados técnicos `D-R` y `D-N` cuando la operación dependa de esas comprobaciones;
7. la ligadura de cada decisión protegida a la revisión y contexto materialmente pertinentes;
8. la producción de trazas suficientes para reconstruir decisiones y efectos dentro del alcance declarado;
9. la identificación explícita de las dependencias que el propio proceso no puede garantizar.

Rust aporta una base favorable para seguridad de memoria, tipado, control explícito de recursos internos y construcción de artefactos autónomos. Estas propiedades no sustituyen la demostración de corrección semántica ni las garantías materiales externas.

Todo uso de `unsafe`, FFI, código nativo externo o mecanismo equivalente deberá incluirse en `TCB(G)` para toda garantía que pueda falsear.

## 6. Autoridad, constitución y formas protegidas

La realización deberá conservar las distinciones de SEC.0-A entre:

```text
información
evidencia admitida
hecho semántico constituido
autoridad
habilitación
ejercicio
```

La capacidad técnica de ejecutar código, administrar una máquina, escribir almacenamiento o poseer privilegios del sistema operativo no constituye por sí misma autoridad SV.

Una autoridad sólo podrá reconocerse como legítimamente constituida por las vías fijadas en SEC.0-A: T-0, T-C, T-G o T-R bajo sus condiciones respectivas. T-I, T-V, T-H y T-E no constituyen autoridad.

Toda forma concreta `F` deberá derivar de un descriptor semántico previamente constituido que fije, al menos, cuando sean aplicables:

- clase T-*;
- familia de efectos;
- ligaduras de contexto;
- autoridad previa necesaria;
- regla de acumulación.

La clase de transición, su familia de efectos y la autoridad exigida no podrán ser elegidas por el llamador, el ejecutor, el verificador, el monitor ni otro componente interesado en la aceptación del acto.

Introducir una forma nueva o modificar materialmente su clase o familia de efectos exige la transición constitutiva correspondiente. Delegar o modificar autoridad exige una T-G o T-C válida cuando así lo determine SEC.0-A; la mera no ampliación de un objeto técnico de permisos no demuestra por sí sola que exista derecho de gobierno.

## 7. Requisitos constituidos, aplicabilidad y fallo cerrado

Para toda forma sujeta a control, el entorno deberá preservar el conjunto constituido:

```text
Req(F,e | C) = N(F,e | C) ∪ S(F,e | C)
```

La pertenencia de una obligación a `Req`, su aplicabilidad y la posibilidad de omitirla no podrán decidirse localmente durante la ejecución para favorecer el acto.

Las obligaciones nucleares fijadas por SEC.0-D permanecerán no eludibles en las condiciones previstas por ese contrato. Una forma sujeta a control con `Req = ∅` no adquiere permiso por ausencia de requisitos.

El resultado técnico de cada obligación conserva exactamente la distinción:

```text
D-A — ACREDITADO
D-R — REFUTADO
D-N — NO_VERIFICABLE
```

Una forma sujeta a control sólo puede continuar cuando todas las obligaciones aplicables están en `D-A`. `D-R` y `D-N` bloquean el efecto protegido correspondiente.

`D-N` es un estado técnico de comprobación. No pertenece a `Tri` y no puede convertirse en `U`, éxito, advertencia tolerable ni permiso por reintento.

Una forma de emergencia, recuperación, mantenimiento o excepción deberá disponer de constitución, autoridad y `Req` propios. No puede utilizarse como vía para eludir un `D-R` o `D-N` de la forma ordinaria.

## 8. Políticas constituidas y prohibición de rebaja en ejecución

Las políticas que condicionan una garantía no podrán presentarse como opciones ordinarias modificables por la misma ejecución cuya validez depende de ellas.

Cuando sean aplicables, deberán quedar previamente constituidos o gobernados:

- `Budget(F | C)`;
- requisitos de actualidad o frescura;
- reglas de revocación y vigencia;
- reglas de acumulación;
- reglas de continuidad y recuperación;
- `ThreatModel(G)`;
- `FailureLimit(G)`;
- definición de `TCB(G)`;
- criterios de aplicabilidad de verificadores y clases de prueba.

Un acto no podrá aumentar localmente su presupuesto, suprimir una obligación de actualidad, reducir el modelo de fallos, eliminar un falsificador de `TCB(G)` ni declarar no aplicable una capacidad materialmente presente para obtener un resultado favorable.

Toda modificación legítima de estas condiciones deberá seguir la transición de gobierno o constitución que corresponda y producir una nueva identidad de garantía o realización cuando la diferencia pueda afectar causalmente a la evidencia previa.

## 9. Efecto protegido y mediación

Se denomina **efecto protegido** a todo efecto cuya producción dependa de autoridad, constitución, verificación, continuidad, consumo, recuperación o cualquier otra condición fijada por SEC.0.

Para una garantía de mediación completa deberá cumplirse:

```text
cualquier vía material capaz de producir el efecto protegido
→ atraviesa un punto gobernado equivalente
```

No basta con que la interfaz ordinaria utilice el mediador.

Deben incluirse en el análisis, cuando puedan producir el mismo efecto:

- interfaces administrativas;
- mantenimiento;
- recuperación;
- actualización;
- depuración;
- escritura directa;
- herramientas operativas;
- procesos privilegiados;
- restauración de estado;
- mecanismos externos capaces de modificar el recurso final.

Una vía alternativa puede existir, pero deberá constituirse como forma gobernada con autoridad, requisitos y garantías propios.

## 10. Estado autoritativo, dependencias persistentes y continuidad

El entorno soberano deberá distinguir entre:

```text
estado de proceso
estado derivado
estado persistente autoritativo
continuidad vigente
```

Toda decisión protegida que deba sobrevivir a reinicio o recuperación deberá poder reconstruir y acreditar las dependencias suficientes de `PDep(d | C)` según SEC.0-M.

Una vista, caché, índice o resumen no autoritativo puede ayudar a localizar información, pero no sustituye por sí solo a `AStore`. Si una estructura derivada determina una decisión autoritativa, deberá asumir expresamente las obligaciones de una fuente autoritativa para ese alcance.

Las afirmaciones negativas basadas en ausencia requieren cobertura acreditada. La ausencia de un elemento en un índice incompleto no demuestra su ausencia del estado autoritativo.

El backend Rust puede mantener y comprobar estructuras internas, pero no puede atribuir por sí solo resistencia material a retroceso o clonación cuando el estado del proceso y su almacenamiento puedan copiarse o restaurarse conjuntamente.

Las decisiones sobre revocación, consumo único, continuidad, acumulación persistente o recuperación deberán depender de una fuente o relación de continuidad cuya resistencia sea suficiente frente al fallo declarado.

## 11. Ligadura entre comprobación y efecto

Una acreditación sólo puede utilizarse mientras continúe siendo aplicable al objeto, revisión, contexto y vigencia de los que dependió.

En el punto material de compromiso del efecto deberá poder acreditarse que las ligaduras relevantes siguen siendo válidas o que una regla previamente constituida demuestra que los cambios intermedios no afectan a `Req(F,e | C)`.

Si esa continuidad no puede acreditarse, corresponde `D-N` y deberá repetirse la comprobación sobre un estado aplicable antes de producir el efecto.

La especificación no impone una técnica concreta de coordinación; exige la propiedad de continuidad entre comprobación y efecto.

## 12. Consumo único y anti-retroceso

Una autoridad o autorización de un solo uso exige que un segundo ejercicio no pueda acreditarse después de clonación, restauración o bifurcación dentro del modelo de fallos declarado.

Un contador local, fichero local, variable del proceso, marca en memoria o registro almacenado en la misma imagen retrocedible no basta para una garantía fuerte de consumo único.

La realización deberá declarar qué mecanismo sostiene la unicidad y frente a qué fallos es independiente.

Si no existe un mecanismo suficiente, el comportamiento correcto es bloquear o declarar técnicamente no verificable el ejercicio que dependa de esa unicidad.

## 13. Recursos, presupuesto y aislamiento

Para toda forma repetible, recursiva, expansiva, abierta a entrada no confiable o capaz de generar actos humanos privilegiados deberá existir, cuando lo exija SEC.0-M, un `Budget(F | C)` previamente constituido.

El presupuesto deberá declarar los recursos relevantes y una cota o criterio de admisión materialmente comprobable. No podrá sustituirse durante la ejecución por un presupuesto más amplio para permitir un acto que ya habría excedido la política constituida.

El entorno Rust puede contabilizar consumo lógico, pero la imposición material de límites de CPU, memoria, almacenamiento, entrada/salida, procesos u otros recursos puede requerir mecanismos externos al proceso.

Para cada recurso relevante deberá distinguirse:

```text
medición
contabilidad
límite lógico
imposición material
reserva de control
```

No se considerará aislada una reserva cuando el mismo fallo o sujeto pueda agotar simultáneamente el recurso ordinario y la capacidad necesaria para rechazar, registrar, revocar, detener o recuperar.

La atención humana, cuando sea aplicable, se mantiene como recurso finito de la arquitectura y no se transforma en capacidad ilimitada por automatización.

## 14. Raíz de confianza, arranque, actualización y recuperación

Toda garantía que dependa de la identidad o integridad del artefacto ejecutado deberá declarar:

```text
Root(G)
TCB(G)
ThreatModel(G)
Evidence(G)
FailureLimit(G)
```

La definición de estas magnitudes deberá estar protegida frente a la misma clase de fallo para la que se invocan. No podrán reescribirse durante el acto para excluir un componente capaz de falsificar la garantía.

El backend no puede convertir en raíz de confianza una variable o configuración cuya legitimidad dependa circularmente del mismo estado que pretende validar.

La arquitectura deberá distinguir:

- construcción;
- artefacto producido;
- artefacto distribuido;
- artefacto cargado;
- estado de ejecución admitido.

Los mecanismos de actualización y recuperación forman parte del perímetro cuando pueden sustituir cualquiera de esos elementos.

Si el modelo de fallos incluye compromiso o sospecha de la raíz saliente, la recuperación deberá depender de una vía suficientemente independiente frente al mismo fallo. La recuperación deberá partir además de una autoridad y regla previamente constituidas; no puede inventar autoridad después del compromiso.

## 15. Construcción y correspondencia fuente-artefacto

El backend Rust no queda acreditado únicamente porque su código fuente sea revisable.

Toda afirmación sobre el artefacto ejecutado deberá considerar, cuando proceda:

- compilador;
- versión y configuración de compilación;
- dependencias;
- código generado;
- enlazador;
- bibliotecas nativas;
- guiones o herramientas capaces de modificar el resultado;
- procedimiento de distribución y carga.

Un componente podrá excluirse de `TCB(G_build)` sólo cuando exista evidencia suficiente para demostrar que no puede falsificar la garantía dentro del fallo declarado.

## 16. Atestación y evidencia de estado vivo

La atestación, si se adopta, será una fuente de evidencia y no una fuente de autoridad.

El entorno deberá poder distinguir evidencia histórica de evidencia actual cuando una forma dependa del estado vivo de la plataforma.

La exigencia de actualidad deberá derivarse de la forma o garantía constituida y no de una decisión local tomada durante la operación.

Cuando la atestación dependa de una raíz, servicio o componente externo, ese componente deberá quedar incluido en el conjunto técnico de confianza correspondiente.

## 17. Comunicaciones y efectos externos

Una comunicación autenticada no acredita por sí sola que el proceso del extremo se encuentre en un estado admitido.

Las garantías de comunicación deberán separar, cuando proceda:

- identidad del extremo;
- integridad del contenido;
- actualidad o no repetición;
- confidencialidad;
- identidad o estado del proceso receptor;
- correspondencia entre solicitud autorizada y efecto externo.

Cuando un efecto externo pueda haber ocurrido sin que el proceso local pueda determinarlo, no deberá repetirse automáticamente salvo que exista una regla gobernada de idempotencia, reconciliación, compensación o comprobación suficiente.

## 18. Presentación y autorización humanas

Cuando una operación privilegiada dependa de una decisión humana, la garantía deberá abarcar la cadena necesaria para que la persona actúe sobre la representación correcta del objeto y de su consecuencia.

La realización deberá conservar, cuando proceda, ligadura entre:

- objeto;
- revisión;
- representación;
- identidad del actor;
- acto de autorización;
- efecto finalmente ejecutado.

Una firma válida no demuestra por sí sola comprensión ni fidelidad de la presentación.

## 19. Frontera de garantías: backend frente a soporte material

La clasificación siguiente es orientativa y deberá concretarse por perfil de realización:

| Propiedad | Papel posible del backend Rust | Soporte material adicional normalmente necesario | Evidencia final |
|---|---|---|---|
| semántica de `Tri` y operadores puros | principal | no necesariamente | pruebas de conformidad y equivalencia |
| tipado y separación de estados internos | principal | no necesariamente | pruebas y revisión del artefacto |
| fallo cerrado lógico | principal | fuente de evidencia o verificación cuando sea externa | pruebas de decisión e integración |
| mediación dentro del proceso | principal | control de vías externas al proceso | ataque del perímetro completo |
| consumo acumulativo local | principal | persistencia si debe sobrevivir reinicios | pruebas de reinicio y continuidad |
| consumo único fuerte | parcial | mecanismo resistente a clonación o retroceso | ataque de clonación y restauración |
| revocación persistente | parcial | almacenamiento y continuidad suficientes | ataque de restauración y bifurcación |
| aislamiento de CPU, memoria o almacenamiento | parcial | sistema operativo, hipervisor, hardware o mecanismo equivalente | agotamiento adversarial |
| raíz de confianza | consumidor de la raíz | raíz externa o materialmente anterior suficiente | ataque de sustitución y recuperación |
| correspondencia fuente-artefacto | productor parcial | cadena de construcción y verificación independiente | reproducción o evidencia equivalente |
| atestación | consumidor y verificador | raíz y mecanismo de atestación | repetición, sustitución y actualidad |
| presentación humana fiel | parcial | interfaz y camino de presentación confiables | cambio entre presentación y firma |
| efecto externo no repetido | parcial | soporte del sistema externo o reconciliación | fallo entre emisión y confirmación |

La tabla no atribuye automáticamente una garantía a ninguna tecnología concreta.

## 20. Identidad de la realización sometida a prueba

Toda realización que aspire a una afirmación de conformidad deberá poder identificarse de forma suficiente.

Como mínimo, según el alcance, deberán poder ligarse:

```text
versión de fuente
IR o entrada admitida
versión del backend
artefacto ejecutable
dependencias relevantes
configuración
definición de garantías
estado inicial
perfil material
versión de la batería
instrumentación
```

Una diferencia capaz de afectar causalmente a una garantía impide transferir automáticamente evidencia entre realizaciones.

## 21. Interfaz para comprobación adversarial

La futura realización deberá permitir construir pruebas sin introducir una segunda semántica de referencia.

Toda ejecución que pretenda aportar cobertura conforme a SEC.0-T deberá poder conservar, según el alcance, la traza:

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

La mera correspondencia nominal de `Targets` no constituye cobertura. El fallo o mutación deberá ser materialmente ejercitable y `ReachedFaults` deberá acreditar que alcanzó la dependencia objetivo.

`Oracle` no podrá depender circularmente del componente sometido a la misma clase de fallo. `Verdict` deberá derivarse de `Expected` y `Observed`.

Se conservarán al menos los estados de prueba establecidos por SEC.0-T:

```text
PASS
FAIL
NO_EJECUTADO
NO_PROBADO
INCONCLUSO
```

`PASS` exige caso falsable ejecutado, alcance acreditado y resultado esperado obtenido. `FAIL` registra una violación o resultado incompatible. `NO_EJECUTADO`, `NO_PROBADO` e `INCONCLUSO` no constituyen cobertura.

Los estados de prueba permanecen separados de `Tri` y de `D-A`, `D-R` y `D-N`.

La instrumentación deberá declararse y no podrá considerarse transparente cuando altere orden, concurrencia, tiempo, recursos, persistencia o privilegios relevantes para el fallo ensayado.

La aplicabilidad de una clase de prueba derivará de las capacidades y garantías efectivamente presentes en el `SUT`; no podrá reducirse mediante una declaración unilateral de «no aplicable».

El catálogo `tests/sec0/VECTORES_ADVERSARIALES_SEC0_V1.md`, una vez incorporado al árbol principal, constituirá la referencia independiente de la implementación para los escenarios conservados.

## 22. Perfiles de realización

La arquitectura podrá admitir perfiles materiales distintos siempre que no compartan una denominación de garantía más fuerte que las propiedades que realmente sostienen.

Un perfil deberá declarar:

- garantías ofrecidas;
- componentes incluidos;
- dependencias externas;
- modelo de fallos;
- límites;
- propiedades no acreditadas.

Una realización local de laboratorio puede ser útil sin ofrecer resistencia a clonación, retroceso, compromiso administrativo o sustitución de artefactos. Esas limitaciones deberán permanecer explícitas y no podrán heredarse silenciosamente hacia un perfil soberano.

## 23. Condiciones previas a la implementación Rust

Antes de atribuir a un módulo Rust una garantía SEC.0 concreta deberá estar identificada, al menos:

1. la propiedad contractual que implementa;
2. la forma constituida, autoridad y requisitos de los que dependa;
3. el punto exacto donde se impone;
4. las vías materiales capaces de evitar ese punto;
5. las dependencias que entran en `TCB(G)`;
6. el modelo de fallos dentro del cual se formula la garantía;
7. qué parte puede imponerse dentro del proceso y qué parte requiere soporte externo;
8. el vector adversarial que podrá falsarla cuando exista una realización comprobable.

Estas condiciones no obligan a resolver desde el inicio todas las garantías materiales. Sí impiden presentar una representación local como si ya fuera una garantía material completa.

## 24. Secuencia arquitectónica de realización

La materialización podrá avanzar por capas sin atribuir a cada capa el sello final del sistema:

### Fase R0 — núcleo semántico soberano

- tipos y operaciones puras exigidas por la frontera normativa;
- correspondencia IR → representación Rust;
- semántica determinista y trazable;
- pruebas de conformidad local.

### Fase R1 — autoridad, mediación y decisiones protegidas

- formas constituidas y clasificación T-* no discrecional;
- frontera explícita de efectos protegidos;
- autoridad y `Req(F,e | C)` aplicables;
- fallo cerrado;
- ligaduras de revisión y contexto;
- trazas de decisión;
- interfaces para evidencia y estado autoritativo.

### Fase R2 — persistencia y continuidad material

- estado autoritativo y `PDep`;
- revocación;
- presupuestos y acumulación persistente;
- recuperación;
- bifurcación;
- consumo único cuando el perfil pretenda ofrecerlo.

### Fase R3 — confianza de plataforma

- construcción y artefacto;
- raíz de confianza;
- actualización;
- atestación cuando proceda;
- aislamiento material de recursos;
- vías administrativas y de mantenimiento.

### Fase R4 — integración adversarial

- aplicación de vectores SEC.0 al sistema completo;
- escenarios integrales A/D/M/X;
- fallos compuestos;
- regresiones permanentes;
- delimitación final de garantías y límites.

Esta secuencia no implica que todos los perfiles deban ofrecer todas las garantías materiales.

## 25. Doble garantía de cierre

Una realización soberana sólo podrá aspirar al cierre dentro de un alcance declarado cuando concurran:

### Garantía I — construcción conforme

La arquitectura y la realización conservan los contratos aplicables y disponen de mecanismos suficientes para imponerlos dentro del modelo de fallos declarado.

### Garantía II — resistencia adversarial integral

El sistema completo ha sido sometido a ataques capaces de alcanzar las dependencias materiales que pueden falsificar esas garantías, sin quedar una violación pendiente dentro del alcance ensayado.

La superación de pruebas locales del backend no sustituye la segunda garantía.

## 26. No garantías

Esta especificación no promete:

- seguridad absoluta;
- ausencia de vulnerabilidades desconocidas;
- disponibilidad perfecta;
- independencia física universal;
- ejecución exactamente una vez frente a sistemas externos que no la soporten;
- integridad de un sistema operativo, hipervisor o hardware no acreditados;
- que Rust elimine por sí solo fallos de diseño, lógica o cadena de suministro;
- comprensión humana;
- resistencia ilimitada frente a un atacante con control físico total.

## 27. Cierre de v0

El entorno soberano del Lenguaje SV deberá construirse alrededor de garantías, autoridad constituida y puntos de imposición, no alrededor de la mera elección de un lenguaje de programación.

Rust queda fijado como base principal del backend soberano, pero las garantías que excedan la frontera del proceso deberán depender de mecanismos materiales expresamente declarados y sometidos al mismo régimen de trazabilidad, fallo cerrado y comprobación adversarial que el resto del sistema.

La siguiente evolución de esta especificación deberá concretar las interfaces mínimas entre el núcleo Rust y las dependencias externas sin seleccionar prematuramente tecnologías de plataforma.
