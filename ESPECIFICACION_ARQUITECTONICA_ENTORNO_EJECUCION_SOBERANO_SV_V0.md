# Especificación arquitectónica del entorno de ejecución soberano mínimo del Lenguaje SV — v0

**Fecha:** 22/08/2026  
**Estado:** borrador arquitectónico v0  
**Ámbito:** `SV-lenguaje-de-computacion`

## 1. Objeto

Este documento define la frontera mínima que deberá separar el backend soberano del Lenguaje SV, orientado a Rust, de las dependencias materiales necesarias para sostener garantías SEC.0.

Su finalidad es impedir dos reducciones incorrectas:

1. identificar el backend con el sistema completo;
2. atribuir al lenguaje de implementación garantías que dependen de almacenamiento, administración, recuperación, raíces de confianza, aislamiento, comunicaciones, presentación humana u otras dependencias externas al proceso.

La especificación no abre todavía una implementación Rust, no selecciona sistema operativo, motor de almacenamiento, hipervisor, base de datos, mecanismo criptográfico, hardware, servicio de identidad ni plataforma de despliegue.

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

La unidad de seguridad no es el proceso Rust. La unidad de seguridad es la **garantía concreta** y todas las dependencias capaces de falsificarla.

Para cada garantía `G` deberá poder establecerse:

```text
G
→ propiedad contractual
→ punto de imposición
→ componentes que pueden falsificarla
→ TCB(G)
→ modelo de fallos
→ evidencia necesaria
→ límites
```

Una propiedad no se considerará materializada porque exista una función, tipo, objeto inmutable, comprobación local o rama de código que la represente.

La propiedad sólo podrá atribuirse a una realización cuando, dentro del modelo de fallos declarado, no exista una vía material no gobernada capaz de producir el efecto que la propiedad pretende impedir.

## 4. Cadena de transformación y ejecución

La arquitectura futura deberá distinguir al menos las etapas siguientes:

```text
fuente SVP
→ etapa frontal
→ IR canónica admitida
→ backend soberano Rust
→ artefacto ejecutable
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
4. la separación entre información, evidencia, autoridad, habilitación y ejercicio cuando la realización materialice estas categorías;
5. la mediación interna de toda operación protegida que pertenezca a su perímetro;
6. el fallo cerrado ante resultados técnicos `D-R` y `D-N` cuando la operación dependa de esas comprobaciones;
7. la ligadura de cada decisión protegida a la revisión y contexto que resulten materialmente pertinentes;
8. la producción de trazas suficientes para reconstruir las decisiones y efectos dentro del alcance declarado;
9. la identificación explícita de las dependencias que el propio proceso no puede garantizar.

Rust aporta una base favorable para seguridad de memoria, control del entorno de ejecución, tipado y construcción de artefactos autónomos. Estas propiedades no sustituyen la demostración de corrección semántica ni las garantías materiales externas.

Todo uso de `unsafe`, FFI, código nativo externo o mecanismo equivalente deberá formar parte del conjunto técnico de confianza de las garantías que pueda falsificar.

## 6. Efecto protegido y mediación

Se denomina **efecto protegido** a todo efecto cuya producción dependa de autoridad, constitución, verificación, continuidad, consumo, recuperación o cualquier otra condición fijada por SEC.0.

Para una garantía de mediación completa deberá cumplirse:

```text
cualquier vía material capaz de producir el efecto protegido
→ atraviesa un punto gobernado equivalente
```

No basta con que la API ordinaria utilice el mediador.

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

## 7. Estado autoritativo y continuidad

El entorno soberano deberá distinguir entre:

```text
estado de proceso
estado derivado
estado persistente autoritativo
continuidad vigente
```

El backend Rust puede mantener y comprobar estructuras internas, pero no puede atribuir por sí solo resistencia material a retroceso o clonación cuando el estado del proceso y su almacenamiento puedan copiarse o restaurarse conjuntamente.

Las decisiones sobre revocación, consumo único, continuidad, acumulación persistente o recuperación deberán depender de una fuente o relación de continuidad cuya resistencia sea suficiente frente al fallo declarado.

Cuando esa propiedad no pueda imponerse desde el propio proceso, deberá declararse una dependencia material externa.

## 8. Consumo único y anti-retroceso

Una autoridad o autorización de un solo uso exige que un segundo ejercicio no pueda acreditarse después de clonación, restauración o bifurcación dentro del modelo de fallos declarado.

Un contador local, fichero local, variable del proceso, marca en memoria o registro almacenado en la misma imagen retrocedible no basta para una garantía fuerte de consumo único.

La realización deberá declarar qué mecanismo sostiene la unicidad y frente a qué fallos es independiente.

Si no existe un mecanismo suficiente, el comportamiento correcto es bloquear o declarar técnicamente no verificable el ejercicio que dependa de esa unicidad.

## 9. Recursos y aislamiento

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

## 10. Raíz de confianza, arranque y actualización

Toda garantía que dependa de la identidad o integridad del artefacto ejecutado deberá declarar:

```text
Root(G)
TCB(G)
ThreatModel(G)
Evidence(G)
FailureLimit(G)
```

El backend no puede convertir en raíz de confianza una variable o configuración cuya legitimidad dependa circularmente del mismo estado que pretende validar.

La arquitectura deberá distinguir:

- construcción;
- artefacto producido;
- artefacto distribuido;
- artefacto cargado;
- estado de ejecución admitido.

Los mecanismos de actualización y recuperación forman parte del perímetro cuando pueden sustituir cualquiera de esos elementos.

Si el modelo de fallos incluye compromiso o sospecha de la raíz saliente, la recuperación deberá depender de una vía suficientemente independiente frente al mismo fallo.

## 11. Construcción y correspondencia fuente-artefacto

El backend Rust no queda acreditado únicamente porque su código fuente sea revisable.

Toda afirmación sobre el artefacto ejecutado deberá considerar, cuando proceda:

- compilador;
- versión y configuración de compilación;
- dependencias;
- código generado;
- enlazador;
- bibliotecas nativas;
- scripts o herramientas capaces de modificar el resultado;
- procedimiento de distribución y carga.

Un componente podrá excluirse de `TCB(G_build)` sólo cuando exista evidencia suficiente para demostrar que no puede falsificar la garantía dentro del fallo declarado.

## 12. Atestación y evidencia de estado vivo

La atestación, si se adopta, será una fuente de evidencia y no una fuente de autoridad.

El entorno deberá poder distinguir evidencia histórica de evidencia actual cuando una forma dependa del estado vivo de la plataforma.

La exigencia de actualidad deberá derivarse de la forma o garantía constituida y no de una decisión local tomada durante la operación.

Cuando la atestación dependa de una raíz, servicio o componente externo, ese componente deberá quedar incluido en el conjunto técnico de confianza correspondiente.

## 13. Comunicaciones y efectos externos

Una comunicación autenticada no acredita por sí sola que el proceso del extremo se encuentre en un estado admitido.

Las garantías de comunicación deberán separar, cuando proceda:

- identidad del extremo;
- integridad del contenido;
- actualidad o no repetición;
- confidencialidad;
- identidad o estado del proceso receptor;
- correspondencia entre solicitud autorizada y efecto externo.

Cuando un efecto externo pueda haber ocurrido sin que el proceso local pueda determinarlo, no deberá repetirse automáticamente salvo que exista una regla gobernada de idempotencia, reconciliación, compensación o comprobación suficiente.

## 14. Presentación y autorización humanas

Cuando una operación privilegiada dependa de una decisión humana, la garantía deberá abarcar la cadena necesaria para que la persona actúe sobre la representación correcta del objeto y de su consecuencia.

La realización deberá conservar, cuando proceda, ligadura entre:

- objeto;
- revisión;
- representación;
- identidad del actor;
- acto de autorización;
- efecto finalmente ejecutado.

Una firma válida no demuestra por sí sola comprensión ni fidelidad de la presentación.

## 15. Frontera de garantías: backend frente a soporte material

La clasificación siguiente es orientativa y deberá concretarse por perfil de realización:

| Propiedad | Papel posible del backend Rust | Soporte material adicional normalmente necesario | Evidencia final |
|---|---|---|---|
| semántica de `Tri` y operadores puros | principal | no necesariamente | pruebas de conformidad y equivalencia |
| tipado y separación de estados internos | principal | no necesariamente | pruebas y revisión del artefacto |
| fallo cerrado lógico | principal | fuente de evidencia/verificación cuando sea externa | pruebas de decisión e integración |
| mediación dentro del proceso | principal | control de vías externas al proceso | ataque de perímetro completo |
| consumo acumulativo local | principal | persistencia si debe sobrevivir reinicios | pruebas de reinicio y continuidad |
| consumo único fuerte | parcial | mecanismo resistente a clonación/retroceso | ataque de clonación y restauración |
| revocación persistente | parcial | almacenamiento y continuidad suficientes | ataque de restauración y bifurcación |
| aislamiento de CPU/memoria/almacenamiento | parcial | sistema operativo, hipervisor, hardware o mecanismo equivalente | agotamiento adversarial |
| raíz de confianza | consumidor de la raíz | raíz externa o materialmente anterior suficiente | ataque de sustitución/recuperación |
| correspondencia fuente-artefacto | productor parcial | cadena de construcción y verificación independiente | reproducción o evidencia equivalente |
| atestación | consumidor y verificador | raíz y mecanismo de atestación | repetición, sustitución y frescura |
| presentación humana fiel | parcial | interfaz y camino de presentación confiables | cambio entre presentación y firma |
| efecto externo no repetido | parcial | soporte del sistema externo o reconciliación | fallo entre emisión y confirmación |

La tabla no atribuye automáticamente una garantía a ninguna tecnología concreta.

## 16. Identidad de la realización sometida a prueba

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

## 17. Interfaz para comprobación adversarial

La futura realización deberá permitir construir pruebas sin introducir una segunda semántica de referencia.

La infraestructura de prueba deberá poder obtener o conservar, según el vector:

```text
InjectedFaults
ReachedFaults
Expected
Observed
Verdict
Artifacts
```

La instrumentación deberá declararse y no podrá considerarse transparente cuando altere orden, concurrencia, tiempo, recursos, persistencia o privilegios relevantes para el fallo ensayado.

El catálogo `tests/sec0/VECTORES_ADVERSARIALES_SEC0_V1.md` constituirá la referencia portable de escenarios una vez incorporado al árbol principal.

## 18. Perfiles de realización

La arquitectura podrá admitir perfiles materiales distintos siempre que no compartan una denominación de garantía más fuerte que las propiedades que realmente sostienen.

Un perfil deberá declarar:

- garantías ofrecidas;
- componentes incluidos;
- dependencias externas;
- modelo de fallos;
- límites;
- propiedades no acreditadas.

Una realización local de laboratorio puede ser útil sin ofrecer resistencia a clonación, retroceso, compromiso administrativo o sustitución de artefactos. Esas limitaciones deberán permanecer explícitas y no podrán heredarse silenciosamente hacia un perfil soberano.

## 19. Condiciones previas a la implementación Rust

Antes de atribuir a un módulo Rust una garantía SEC.0 concreta deberá estar identificada, al menos:

1. la propiedad contractual que implementa;
2. el punto exacto donde se impone;
3. las vías materiales capaces de evitar ese punto;
4. las dependencias que entran en `TCB(G)`;
5. el modelo de fallos dentro del cual se formula la garantía;
6. qué parte puede imponerse dentro del proceso y qué parte requiere soporte externo;
7. el vector adversarial que podrá falsarla cuando exista una realización comprobable.

Estas condiciones no obligan a resolver desde el inicio todas las garantías materiales. Sí impiden presentar una representación local como si ya fuera una garantía material completa.

## 20. Secuencia arquitectónica de realización

La materialización podrá avanzar por capas sin atribuir a cada capa el sello final del sistema:

### Fase R0 — núcleo semántico soberano

- tipos y operaciones puras exigidas por la frontera normativa;
- correspondencia IR → representación Rust;
- semántica determinista y trazable;
- pruebas de conformidad local.

### Fase R1 — mediación y decisiones protegidas

- frontera explícita de efectos protegidos;
- fallo cerrado;
- ligaduras de revisión y contexto;
- trazas de decisión;
- interfaces para autoridad, evidencia y estado autoritativo.

### Fase R2 — persistencia y continuidad material

- estado autoritativo;
- revocación;
- acumulación persistente;
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

## 21. Doble garantía de cierre

Una realización soberana sólo podrá aspirar al cierre dentro de un alcance declarado cuando concurran:

### Garantía I — construcción conforme

La arquitectura y la realización conservan los contratos aplicables y disponen de mecanismos suficientes para imponerlos dentro del modelo de fallos declarado.

### Garantía II — resistencia adversarial integral

El sistema completo ha sido sometido a ataques capaces de alcanzar las dependencias materiales que pueden falsificar esas garantías, sin quedar una violación pendiente dentro del alcance ensayado.

La superación de pruebas locales del backend no sustituye la segunda garantía.

## 22. No garantías

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

## 23. Cierre de v0

El entorno soberano del Lenguaje SV deberá construirse alrededor de garantías y puntos de imposición, no alrededor de la mera elección de un lenguaje de programación.

Rust queda fijado como base principal del backend soberano, pero las garantías que excedan la frontera del proceso deberán depender de mecanismos materiales expresamente declarados y sometidos al mismo régimen de trazabilidad, fallo cerrado y comprobación adversarial que el resto del sistema.

La siguiente evolución de esta especificación deberá concretar las interfaces mínimas entre el núcleo Rust y las dependencias externas sin seleccionar prematuramente tecnologías de plataforma.
