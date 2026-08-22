# Especificación arquitectónica del entorno de ejecución soberano mínimo del Lenguaje SV — v0

**Fecha:** 22/08/2026  
**Estado:** borrador arquitectónico v0  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Este documento define la frontera mínima entre el backend soberano del Lenguaje SV, orientado a Rust, y las dependencias materiales necesarias para sostener garantías SEC.0.

Su finalidad es impedir cinco reducciones incorrectas:

1. identificar el backend con el sistema completo;
2. atribuir al lenguaje de implementación garantías que dependen de almacenamiento, administración, recuperación, raíces de confianza, aislamiento, comunicaciones, presentación humana u otras dependencias externas al proceso;
3. convertir autoridad, forma, requisitos, presupuesto, modelo de fallos o límites de garantía en parámetros libres de ejecución;
4. heredar evidencia después de cambiar una capacidad, dependencia o perfil que pueda afectar causalmente a la garantía;
5. presentar una conformidad obtenida sobre un perfil estrecho como conformidad de un sistema posterior más amplio.

La especificación no abre todavía una implementación Rust ni selecciona plataforma material. No modifica semántica, gramática, IR, catálogo diagnóstico ni SEC.0-A/D/M/X/T.

## 2. Antecedentes y jerarquía

Esta especificación se interpreta conjuntamente con:

- `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`;
- `OBJETIVO_RUST_0_BACKEND_SOBERANO.md`;
- `MANIFIESTO_DE_ARQUITECTURA_DERECHOS_OBLIGACIONES_GARANTIAS_Y_FUNDAMENTOS_DEL_SISTEMA_VECTORIAL_SV_V1.md`;
- SEC.0-A, SEC.0-D, SEC.0-M, SEC.0-X y SEC.0-T;
- `ACTA_ARQUITECTONICA_ESTATUTO_LABORATORIO_BACKEND_SOBERANO_Y_DOBLE_GARANTIA_SV_2026_08_22.md`.

En caso de conflicto, esta pieza no rebaja ninguna obligación superior.

## 3. Unidad de análisis por garantía

La unidad de seguridad no es el proceso Rust. La unidad de análisis es la garantía concreta `G` y todas las dependencias capaces de falsificarla.

Para cada `G` deberá poder establecerse:

```text
G
→ propiedad contractual
→ punto de imposición
→ componentes capaces de falsificarla
→ TCB(G)
→ ThreatModel(G)
→ Evidence(G)
→ FailureLimit(G)
→ límites declarados
```

Una función, un tipo o una comprobación local no materializan por sí solos una garantía.

Todo componente cuyo compromiso pueda falsificar `G` deberá incluirse en `TCB(G)` o quedar excluido mediante evidencia suficiente dentro del modelo de fallos declarado. Una delimitación de alcance no puede utilizarse para ocultar precisamente un falsificador de la garantía afirmada.

## 4. Cadena de transformación y ejecución

La arquitectura deberá distinguir:

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

La etapa frontal de referencia puede continuar temporalmente en Python. El artefacto soberano final no deberá requerir Python para ejecutarse.

La aceptación de una etapa no acredita automáticamente la siguiente.

## 5. Núcleo Rust: obligaciones intra-proceso

Dentro de su frontera real, el núcleo soberano deberá preservar:

1. tipos y distinciones semánticas obligatorias;
2. `Tri` sin conversiones implícitas prohibidas;
3. correspondencia entre IR admitida y operaciones ejecutables;
4. separación entre información, evidencia, hecho constituido, autoridad, habilitación y ejercicio;
5. mediación interna de operaciones protegidas de su perímetro;
6. fallo cerrado ante `D-R` y `D-N`;
7. ligadura de decisiones a revisión y contexto pertinentes;
8. trazas suficientes dentro del alcance interno;
9. declaración de dependencias que el proceso no puede garantizar.

Estas propiedades son intra-proceso. **No constituyen por sí solas Garantía I material** cuando `G` dependa de componentes externos.

En particular, mediación interna no equivale a mediación completa; una traza emitida por el propio `SUT` no es necesariamente evidencia pública independiente; y el fallo cerrado lógico no demuestra la integridad o disponibilidad de verificadores externos.

Todo uso de `unsafe`, FFI o código nativo externo deberá incluirse en `TCB(G)` cuando pueda falsificar la garantía.

## 6. Autoridad, formas y génesis

La realización deberá conservar las distinciones de SEC.0-A. La capacidad técnica o administrativa no constituye por sí misma autoridad SV.

Una autoridad sólo puede reconocerse por T-0, T-C, T-G o T-R bajo sus condiciones respectivas. T-I, T-V, T-H y T-E no constituyen autoridad.

### 6.1. T-0 sólo en la génesis inicial de la instancia

T-0 sólo puede constituir el primer estado legítimo de autoridad de una instancia durante su génesis inicial admitida, antes de que dicha instancia entre en un estado ordinario de ejecución protegido.

Un perfil material o de garantías puede formar parte de esa configuración inicial. **Crear, sustituir o ampliar un perfil después de la génesis de la instancia no constituye una nueva T-0**: el acto deberá clasificarse por su efecto real como T-G o T-C, o como T-R si únicamente restaura autoridad preexistente bajo continuidad legítima.

T-0 no es una operación ordinaria disponible después de entrar en un estado de ejecución admitido. Un procedimiento denominado `bootstrap`, `init`, `genesis` o equivalente no adquiere estatuto T-0 por su nombre.

Después de la génesis, todo acto que conceda, amplíe, delegue, sustituya o restaure autoridad debe satisfacer autoridad previa y `Req` aplicables.

Etiquetar como T-0 una fabricación de autoridad en ejecución no la legitima y el efecto debe quedar bloqueado conforme a SEC.0-A/D.

### 6.2. Forma constituida

Toda forma `F` deberá derivar de un descriptor previamente constituido que fije, cuando corresponda:

- clase T-*;
- familia de efectos;
- ligaduras de contexto;
- autoridad previa necesaria;
- regla de acumulación.

La clase y autoridad exigidas no pueden ser elegidas por un componente interesado en la aceptación del acto.

## 7. Requisitos y fallo cerrado

Para toda forma sujeta a control se preserva:

```text
Req(F,e | C) = N(F,e | C) ∪ S(F,e | C)
```

La aplicabilidad de obligaciones no puede rebajarse durante la ejecución.

Se mantienen exactamente:

```text
D-A — ACREDITADO
D-R — REFUTADO
D-N — NO_VERIFICABLE
```

Sólo puede continuar una forma cuando todas las obligaciones aplicables están en `D-A`. `D-R` y `D-N` bloquean el efecto correspondiente. `D-N` no es `U` ni éxito.

Una forma de emergencia, recuperación, mantenimiento o excepción requiere constitución, autoridad y `Req` propios.

Si evidencias o verificadores aplicables son incompatibles y no existe regla de resolución previamente constituida, corresponde `D-N`.

## 8. Políticas constituidas y rebaja por renombrado

Cuando sean aplicables, deberán estar previamente constituidos o gobernados:

- `Budget(F | C)`;
- requisitos de actualidad;
- revocación y vigencia;
- acumulación;
- continuidad y recuperación;
- `ThreatModel(G)`;
- `FailureLimit(G)`;
- `TCB(G)`;
- criterios de aplicabilidad.

No pueden rebajarse como parámetros ordinarios del acto.

### 8.1. Cambio de garantía

Reducir `ThreatModel(G)`, `TCB(G)`, `FailureLimit(G)`, dependencias o requisitos y denominar al resultado `G'` no conserva la identidad de `G` ni su evidencia.

Todo cambio causalmente relevante requiere una transición de gobierno o constitución válida apoyada en autoridad previamente constituida. La nueva identidad no legitima el cambio por sí sola.

Si el sujeto beneficiado por la rebaja necesita precisamente la garantía modificada para acreditar su propia autoridad de cambio, existe acreditación circular y el acto no puede fundarse en esa misma base. Cuando el modelo de fallos exija independencia, la autoridad o procedimiento de gobierno deberá ser suficientemente independiente frente a la misma clase de fallo.

Una `G'` legítimamente más estrecha deberá declarar su alcance menor y no heredará el sello ni la evidencia de `G`. Las propiedades afectadas permanecen `NO_PROBADO` hasta nueva comprobación.

## 9. Efecto protegido y mediación completa

Un efecto protegido es todo efecto dependiente de autoridad, constitución, verificación, continuidad, consumo, recuperación u otra condición SEC.0.

Para afirmar mediación completa:

```text
cualquier vía material capaz de producir el efecto protegido
→ atraviesa un punto gobernado equivalente
```

No basta la interfaz ordinaria.

El análisis deberá incluir cualquier vía administrativa, de mantenimiento, recuperación, actualización, depuración, carga o extensión dinámica, escritura directa, código externo, infraestructura anfitriona, virtualización, gestión material o mecanismo equivalente que pueda modificar el recurso final o evitar el punto de imposición.

La lista es no exhaustiva. El criterio es causal: si una dependencia puede falsificar `G`, entra en `TCB(G)` salvo exclusión acreditada.

## 10. Estado autoritativo y recuperación no circular

El entorno distinguirá:

```text
estado de proceso
estado derivado
estado persistente autoritativo
continuidad vigente
```

Las decisiones que sobrevivan a reinicio o recuperación deberán reconstruir y acreditar `PDep(d | C)`.

Una vista o índice no autoritativo no sustituye por sí solo a `AStore`.

La recuperación no puede fundarse únicamente en una prueba que retroceda o se clone indistinguiblemente junto con el estado protegido.

### 10.1. Estado y clave bajo el mismo fallo

El estado recuperado no puede ser la única fuente de la clave, raíz, testigo o regla que acredita la legitimidad de ese mismo estado frente al fallo considerado.

Si el mismo fallo puede restaurar simultáneamente el estado y su única prueba de legitimidad, la recuperación permanece no acreditable para las decisiones dependientes.

## 11. Ligadura entre comprobación y efecto

Una acreditación sólo puede utilizarse mientras siga siendo aplicable al objeto, revisión, contexto y vigencia de los que dependió.

En el punto de compromiso debe acreditarse continuidad suficiente entre comprobación y efecto. Si cambia una dimensión material, el `D-A` anterior no basta: se requiere nueva comprobación o corresponde `D-N`.

Persistir un `D-A` no lo independiza de su contexto.

## 12. Consumo único y concurrencia

Una autorización de un solo uso debe impedir que un segundo ejercicio quede acreditado después de clonación, restauración, bifurcación o carrera concurrente dentro del modelo declarado.

Un contador local dentro de la misma imagen retrocedible no basta para una garantía fuerte.

Si dos ejecutores alcanzan simultáneamente el punto de compromiso de la misma autoridad consumible, como máximo un efecto puede quedar acreditado.

## 13. Recursos, presupuesto, tiempo y aislamiento

Cuando SEC.0-M lo exija, `Budget(F | C)` deberá estar constituido y permitir decidir el régimen admisible antes del exceso. No podrá sustituirse durante la ejecución por otro más amplio.

El backend puede contabilizar consumo lógico, pero la imposición material de límites puede requerir soporte externo.

La atención humana sigue siendo un recurso finito.

Toda decisión dependiente del tiempo deberá declarar su fuente temporal. Una marca temporal mayor no demuestra continuidad ni vigencia. Si la fuente temporal puede ser alterada bajo el fallo declarado, deberá formar parte de las dependencias y de `TCB(G)` cuando pueda falsificar la garantía.

## 14. Raíz, arranque, actualización y recuperación

Toda garantía dependiente de la identidad o integridad del artefacto ejecutado deberá declarar:

```text
Root(G)
TCB(G)
ThreatModel(G)
Evidence(G)
FailureLimit(G)
```

Estas magnitudes no pueden reescribirse durante el acto para excluir falsificadores.

Si el modelo incluye compromiso o sospecha de la raíz saliente, la recuperación requiere una vía suficientemente independiente frente al mismo fallo.

La recuperación parte de autoridad y reglas previamente constituidas; no inventa autoridad después del compromiso.

El arranque correcto no acredita por sí solo la legitimidad del estado persistente.

## 15. Construcción y artefacto ejecutado

El código fuente revisable no acredita por sí solo el artefacto cargado.

Toda afirmación sobre el ejecutable deberá considerar compilador, configuración de construcción, dependencias, enlazado, bibliotecas externas y procedimiento de distribución y carga cuando puedan afectar causalmente a `G`.

Una diferencia causalmente relevante entre fuente, artefacto producido, distribuido o cargado cambia la identidad probatoria de la realización.

## 16. Atestación y estado vivo

La atestación es evidencia, no autoridad.

La actualidad exigida deriva de la forma o garantía constituida y no de una decisión local.

Una evidencia antigua no acredita el estado vivo actual cuando la diferencia sea material para `G`.

## 17. Comunicaciones y efectos externos

Un canal autenticado no acredita por sí solo que el proceso del extremo esté en un estado admitido.

Las garantías de comunicación separarán identidad, integridad, actualidad, confidencialidad, estado del proceso y correspondencia entre solicitud y efecto cuando proceda.

Si un efecto externo pudo ocurrir y no puede determinarse, no deberá repetirse automáticamente sin una regla gobernada suficiente.

## 18. Presentación y autorización humanas

Cuando una operación privilegiada dependa de una decisión humana, deberá conservarse la ligadura entre objeto, revisión, representación, identidad del actor, acto de autorización y efecto ejecutado.

Una firma válida no demuestra por sí sola comprensión ni fidelidad de presentación.

Una aprobación humana no constituye independencia frente a un fallo que pueda falsear simultáneamente la raíz y la presentación, identidad, canal o evidencia de firma utilizada para obtenerla.

## 19. Frontera de garantías

| Propiedad | Papel posible del backend Rust | Soporte material adicional normalmente necesario | Evidencia final |
|---|---|---|---|
| semántica de `Tri` y operadores puros | principal | no necesariamente | conformidad y equivalencia |
| tipado y estados internos | principal | no necesariamente | pruebas y revisión |
| fallo cerrado lógico | principal | verificación externa cuando corresponda | decisión e integración |
| mediación dentro del proceso | principal | control de vías externas | comprobación del perímetro |
| consumo acumulativo local | principal | persistencia si sobrevive reinicios | reinicio y continuidad |
| consumo único fuerte | parcial | soporte resistente a clonación, retroceso y concurrencia | comprobación material |
| revocación persistente | parcial | almacenamiento y continuidad | restauración y bifurcación |
| aislamiento de recursos | parcial | plataforma capaz de imponerlo | agotamiento adversarial |
| raíz de confianza | consumidor | raíz suficiente | sustitución y recuperación |
| fuente-artefacto | productor parcial | cadena de construcción | evidencia de correspondencia |
| atestación | consumidor/verificador | raíz y mecanismo externo | actualidad y sustitución |
| presentación humana | parcial | interfaz y camino confiables | cambio presentación-acto |
| efecto externo | parcial | reconciliación o soporte externo | fallo entre emisión y confirmación |

La tabla no atribuye automáticamente ninguna garantía ni constituye Garantía I por sí sola.

## 20. Identidad exacta del SUT

Toda afirmación de conformidad deberá ligar, según el alcance:

```text
versión de fuente
IR o entrada admitida
versión del backend
artefacto ejecutable y cargado
dependencias relevantes
configuración
definición de garantías
Capabilities(SUT,G)
TCB(G)
ThreatModel(G)
FailureLimit(G)
estado inicial
datos de prueba
perfil material
versión de la batería
instrumentación
```

Una diferencia capaz de afectar causalmente a `G` impide transferir automáticamente evidencia.

### 20.1. Cambio de capacidades

Añadir o retirar persistencia, recuperación, administración, comunicaciones, privilegios, virtualización, una raíz, una vía de actualización o cualquier dependencia causalmente relevante puede cambiar `Capabilities(SUT,G)`, `TCB(G)` y las clases aplicables aunque el binario no cambie.

Cuando ocurra, cambia la identidad probatoria para las propiedades afectadas. La evidencia previa no se hereda por identidad de binario, nombre de perfil ni similitud de configuración. Esas propiedades vuelven a `NO_PROBADO` hasta nueva comprobación.

## 21. Interfaz adversarial

Toda ejecución que pretenda aportar cobertura conforme a SEC.0-T deberá conservar:

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

La correspondencia nominal de `Targets` no es cobertura. `ReachedFaults` debe acreditar alcance causal.

Un registro del propio `SUT` no basta cuando la misma clase de fallo pueda falsear simultáneamente objetivo y registro.

`Oracle` no puede ser circular frente al mismo fallo. `Verdict` deriva de `Expected` y `Observed`.

Se mantienen:

```text
PASS
FAIL
NO_EJECUTADO
NO_PROBADO
INCONCLUSO
```

`NO_EJECUTADO`, `NO_PROBADO` e `INCONCLUSO` no cubren. Un `FAIL` confirmado sólo desaparece mediante cierre causal conforme a SEC.0-T.

La aplicabilidad deriva de las capacidades efectivamente presentes. No puede reducirse por declaración unilateral.

La evidencia pública deberá estar protegida frente a la misma clase de fallo para la que se invoca.

El catálogo `tests/sec0/VECTORES_ADVERSARIALES_SEC0_V1.md` será una referencia de escenarios falsables; su mera existencia documental no constituye cobertura.

## 22. Perfiles de realización

Un perfil deberá declarar garantías, capacidades, dependencias, modelo de fallos, límites y propiedades no acreditadas.

No puede construirse una conformidad agregando resultados positivos de un perfil y negativos de otro. La afirmación debe sostenerse sobre una misma identidad probatoria de `SUT`, garantía y perfil.

La expresión «SEC.0 conforme» sin identificar garantías, perfil, capacidades y límites no basta para una afirmación de doble garantía.

## 23. Condiciones previas a la implementación Rust

Antes de atribuir a un módulo Rust una garantía SEC.0 deberán estar identificados:

1. propiedad contractual;
2. forma, autoridad y requisitos aplicables;
3. punto de imposición;
4. vías capaces de evitarlo;
5. `TCB(G)`;
6. modelo de fallos;
7. frontera entre proceso y soporte externo;
8. vector falsable;
9. criterio de observación y alcance causal.

No es necesario resolver inicialmente todas las garantías materiales. Sí queda prohibido presentar una representación local como garantía material completa.

## 24. Secuencia de realización

### R0 — núcleo semántico soberano

Tipos, operaciones puras, correspondencia IR → Rust, semántica determinista y conformidad local.

### R1 — autoridad, mediación y decisiones protegidas

Formas constituidas, T-0 restringida a la génesis inicial de instancia, autoridad aplicable, `Req`, fallo cerrado, ligaduras y trazas.

### R2 — persistencia y continuidad material

`AStore`, `PDep`, revocación, presupuestos, tiempo cuando proceda, recuperación no circular, bifurcación y consumo único si el perfil lo ofrece.

### R3 — confianza de plataforma

Construcción, artefacto, raíz, actualización, atestación, aislamiento y dependencias materiales capaces de falsificar garantías.

### R4 — integración adversarial

Aplicación de vectores al sistema completo, fallos compuestos, comprobación de vías materiales, reducción causal y regresiones permanentes.

La secuencia no implica que todos los perfiles deban ofrecer todas las garantías.

## 25. Doble garantía ligada a identidad exacta

Una realización sólo podrá aspirar al cierre para una identidad exacta de `SUT`, garantía y perfil cuando concurran:

### Garantía I — construcción conforme

La arquitectura y realización conservan los contratos aplicables y disponen de mecanismos suficientes para imponerlos dentro del modelo de fallos, incluidas las dependencias materiales necesarias para `G`.

Las propiedades intra-proceso del §5 no bastan por sí solas cuando `G` depende del exterior del proceso.

### Garantía II — resistencia adversarial integral

El sistema completo correspondiente a la misma identidad ha sido sometido a pruebas capaces de alcanzar las dependencias que pueden falsificar `G`, sin violaciones pendientes dentro del alcance ensayado.

La existencia documental de vectores no satisface Garantía II. Una propiedad aplicable en `NO_PROBADO`, `NO_EJECUTADO` o `INCONCLUSO` impide la afirmación completa en el alcance afectado.

### 25.1. No herencia del doble sello

El doble sello no se hereda automáticamente después de añadir o retirar persistencia, recuperación, administración, comunicaciones, privilegios, dependencias materiales, ni después de cambiar `TCB(G)`, `ThreatModel(G)`, `FailureLimit(G)` o `Capabilities(SUT,G)`.

Cuando el cambio sea causalmente relevante, la realización resultante constituye una nueva identidad probatoria para las propiedades afectadas y deberá volver a satisfacer Garantía I y Garantía II en ese alcance.

La superación de pruebas sobre un perfil de laboratorio no acredita un perfil posterior con capacidades adicionales.

## 26. No garantías

Esta especificación no promete seguridad absoluta, ausencia de vulnerabilidades desconocidas, disponibilidad perfecta, independencia física universal, ejecución exactamente una vez frente a sistemas externos sin soporte suficiente, integridad de componentes no acreditados, comprensión humana ni resistencia ilimitada frente a control físico total.

Rust tampoco elimina por sí solo fallos de diseño, lógica o cadena de suministro.

## 27. Cierre de v0

El entorno soberano del Lenguaje SV deberá construirse alrededor de garantías, autoridad constituida, identidad exacta de realización y puntos materiales de imposición.

T-0 queda restringida a la génesis inicial de la instancia; una garantía no puede rebajarse por renombrado; un estado recuperado no acredita circularmente su propia legitimidad; una capacidad añadida puede cambiar la identidad probatoria del sistema; y ningún doble sello se transfiere automáticamente a una realización distinta de la que fue construida y comprobada.

La siguiente evolución deberá concretar las interfaces mínimas entre el núcleo Rust y las dependencias externas sin seleccionar prematuramente tecnologías de plataforma.
