# SEC.0-X — Contrato abstracto de ejecución material, conjunto técnico de confianza, arranque, atestación e independencia

**Fecha:** 21/08/2026  
**Estado:** contrato arquitectónico cerrado  
**Ámbito:** Lenguaje SV — SEC.0-X  
**Antecedentes:** SEC.0-A, SEC.0-D y SEC.0-M cerrados.

## 1. Objeto

SEC.0-X fija las condiciones materiales mínimas que debe satisfacer una realización del Lenguaje SV para que los contratos previamente establecidos de autoridad, diagnóstico, persistencia y continuidad no puedan ser falseados por componentes no declarados de la propia plataforma.

Este contrato no selecciona sistema operativo, procesador, lenguaje de implementación, hipervisor, motor de almacenamiento, mecanismo criptográfico, fabricante, servicio de identidad ni infraestructura de red. Tampoco modifica gramática, IR, validador ni catálogo diagnóstico.

## 2. Conjunto técnico de confianza por garantía

Para toda garantía material `G` debe poder definirse:

```text
TCB(G)
```

como el conjunto de componentes materiales, lógicos y operativos cuya conducta puede determinar si `G` se cumple.

Se establece:

```text
si el compromiso de c puede falsificar G
⇒ c ∈ TCB(G)
```

No puede excluirse un componente por considerarlo infraestructura auxiliar, por estar fuera del proceso principal o por una división meramente organizativa.

No se exige un único conjunto técnico de confianza universal. Pueden declararse conjuntos distintos para garantías distintas, siempre que sus dependencias sean explícitas.

## 3. Independencia material y modelo de fallos

La separación conceptual entre módulos, procesos, servicios, máquinas virtuales, contenedores o cuentas no demuestra independencia material.

Toda afirmación de independencia debe indicar respecto de qué clase de fallo o compromiso se sostiene. Dos componentes pueden ser independientes frente a un fallo de proceso y compartir, sin embargo, sistema operativo, administrador material, raíz criptográfica, servicio de identidad, cadena de construcción, almacenamiento, alimentación o red.

Toda garantía material debe declarar su modelo de fallos y sus límites. No se considera válida una garantía cuyo modelo excluya precisamente un componente capaz de falsearla dentro del alcance declarado.

## 4. Raíz de confianza

Toda cadena material de validación necesita una raíz:

```text
Root(G)
```

cuya legitimidad no dependa circularmente de aquello que pretende validar.

Una cadena de la forma:

```text
A valida B
B valida A
```

no es suficiente si no existe una raíz anterior que rompa la circularidad.

### 4.1. Sustitución de la raíz

Se distinguen dos situaciones:

1. **rotación ordinaria**, cuando el modelo de fallos constituido excluye el compromiso de la raíz saliente;
2. **recuperación frente a compromiso o sospecha de compromiso**, cuando la raíz saliente no puede actuar como única prueba de legitimidad de su sucesora.

La clasificación entre ambas situaciones queda determinada por `ThreatModel(G)` y `FailureLimit(G)` previamente constituidos. No puede elegirse durante el acto de sustitución.

Si el modelo de fallos incluye el compromiso o la sospecha de compromiso de `Root(G)`, la nueva raíz sólo puede admitirse mediante una raíz anterior distinta, una relación de recuperación independiente frente al mismo fallo o un procedimiento externo previamente gobernado.

Si no existe una vía independiente suficiente, la garantía dependiente deja de ser acreditable para las operaciones que la requieren.

## 5. Arranque y estado de ejecución admitido

Se distingue:

```text
BootCandidate
```

de:

```text
AdmittedExecutionState
```

El mero arranque de un ejecutable no lo convierte en estado de ejecución admitido.

Toda etapa capaz de sustituir, modificar o redirigir una etapa posterior y falsificar una garantía entra en el conjunto técnico de confianza correspondiente. El arranque correcto tampoco decide qué estado persistente constituye continuidad vigente; esa cuestión permanece gobernada por SEC.0-M.

## 6. Fuente, construcción y artefacto ejecutable

La publicidad y auditabilidad del código fuente no demuestran por sí solas que el artefacto cargado corresponda a ese código.

Se distinguen:

```text
Source
BuildInputs
BuildProcess
Artifact
LoadedArtifact
```

Toda herramienta o dependencia capaz de introducir conducta no presente en la fuente examinada pertenece a `TCB(G_build)`, salvo que exista una comprobación suficiente para excluirla.

Esa comprobación sólo permite la exclusión cuando es independiente respecto de la misma clase de fallo frente a la que se pretende excluir la herramienta. Una segunda construcción dentro de la misma cadena comprometible, o una huella producida por el mismo proceso, no constituyen por sí solas independencia suficiente.

La capacidad técnica para sustituir un artefacto no confiere autoridad para actualizarlo.

## 7. Mediación material

Cuando una garantía afirma que toda operación protegida atraviesa un componente de mediación, deben cumplirse:

1. mediación completa;
2. resistencia a modificación no autorizada por los sujetos sometidos a la política;
3. base de comprobación declarada para justificar que el mediador aplica la política correspondiente.

Si la decisión real depende de un intérprete semántico, motor de reglas, clasificador o componente de dominio, dicho componente pertenece al conjunto técnico de confianza de esa garantía.

Las vías de administración, depuración, recuperación, actualización, escritura directa o acceso privilegiado que puedan producir el mismo efecto protegido forman parte del mismo perímetro o deben constituirse como formas alternativas gobernadas con garantías explícitas.

## 8. Atestación y actualidad de la evidencia

Una atestación se considera evidencia acerca de propiedades declaradas de una plataforma o estado de ejecución:

```text
Attest(P, S, n)
```

No crea autoridad y sólo puede satisfacer obligaciones previamente constituidas.

La necesidad de actualidad de la evidencia no puede decidirse localmente. Cuando una forma dependa del estado vivo de la plataforma, del arranque vigente, de la composición actual del conjunto técnico de confianza o de otra propiedad mutable, la actualidad forma parte del núcleo de comprobación de la forma.

Si el elemento de actualidad exigido no puede acreditarse, corresponde el resultado técnico `D-N`.

## 9. Persistencia, retroceso y clonación

Una respuesta lógica de escritura no acredita por sí sola una garantía material de persistencia superior a la que la plataforma pueda justificar.

El testigo utilizado para impedir retroceso o doble consumo debe poseer independencia material suficiente respecto del estado que protege, dentro del modelo de fallos declarado.

No basta que un contador, restricción o testimonio viva dentro de la misma copia que pueda clonarse o retroceder conjuntamente.

## 10. Aislamiento de recursos

Las reservas de CPU, memoria, almacenamiento, entrada/salida u otros recursos sólo se consideran aisladas frente a las clases de fallo para las que la plataforma pueda imponer materialmente esa separación.

Si un mismo sujeto o fallo puede agotar tanto los recursos ordinarios como los reservados para rechazo, revocación, diagnóstico o recuperación, no existe aislamiento frente a ese fallo.

## 11. Canales y extremos

La autenticación de un canal no demuestra por sí sola que el proceso situado detrás del extremo se encuentre en un estado de ejecución admitido.

Toda garantía de comunicación debe distinguir, cuando proceda:

- identidad del extremo;
- integridad del contenido;
- actualidad o no repetición;
- confidencialidad;
- relación entre el extremo lógico y el proceso material que recibe o emite.

## 12. Presentación humana y firma

Cuando una decisión humana privilegiada dependa de lo mostrado por una interfaz, el camino desde el objeto autorizado hasta su representación forma parte del conjunto técnico de confianza de esa garantía.

La firma sólo puede atribuirse al objeto material relevante cuando exista ligadura verificable entre:

- objeto o consecuencia;
- representación mostrada;
- identidad de la persona;
- acto de firma;
- versión o estado aplicable;
- revisión material común a presentación y firma.

Si el objeto o cualquiera de sus ligaduras relevantes cambia después de mostrarse y antes de firmarse, la presentación anterior deja de acreditar el objeto posterior.

SEC.0-X no demuestra comprensión humana ni ausencia de fatiga; sí exige que la plataforma no falsee materialmente aquello sobre lo que la persona decide.

## 13. Independencia de quórums y firmas múltiples

La pluralidad numérica no demuestra independencia.

Para afirmar independencia frente a un fallo deben examinarse las dependencias comunes relevantes: servicio de identidad, interfaz humana, administrador, raíz criptográfica, soporte físico, cadena de construcción, almacenamiento o canal de control, entre otras.

Si una dependencia común puede falsificar todas las aprobaciones, la pluralidad no constituye independencia frente a ese fallo.

## 14. Claves, secretos y privilegios materiales

Una clave o secreto puede permitir ejercer autoridad, pero su posesión técnica no redefine al titular de esa autoridad.

Si una clave destinada a demostrar unicidad puede copiarse indistinguiblemente junto con una réplica, no basta por sí sola para demostrar identidad única de implantación ni consumo único.

Toda cuenta o proceso con capacidad material para alterar ejecutables, almacenamiento, configuración, tiempo, claves o componentes de control pertenece al conjunto técnico de confianza de las garantías que pueda falsificar, o bien debe declararse que la garantía no resiste su compromiso.

## 15. Actualización, recuperación y mantenimiento

Los mecanismos de actualización, recuperación, depuración y mantenimiento no quedan fuera de los contratos SEC.0-A, SEC.0-D y SEC.0-M por ser extraordinarios.

Si pueden producir efectos protegidos, pertenecen al perímetro correspondiente.

Una forma alternativa de mantenimiento con garantías menores no puede conservar la misma denominación ni el mismo alcance de garantía que la forma ordinaria. La equivalencia funcional no implica equivalencia de seguridad.

## 16. Observación y prueba material

Un registro, contador, sensor u observador sólo puede acreditar aquello que no pueda falsearse por la misma causa de compromiso que pretende detectar.

La observación externa no es obligatoria en todos los casos; lo es cuando una garantía concreta necesita distinguir un fallo que el propio estado observado no puede detectar desde dentro.

## 17. Definición protegida de garantías

Toda garantía concreta debe identificar:

```text
Guarantee G
Root(G)
TCB(G)
ThreatModel(G)
Evidence(G)
FailureLimit(G)
```

Esta definición no es un registro informativo libremente editable cuando se utiliza para decisiones protegidas. Forma parte de las dependencias persistentes correspondientes y su modificación requiere la transición de gobierno o constitución aplicable.

Si existe un componente cuyo compromiso puede falsificar `G` y dicho componente ha sido omitido de `TCB(G)`, la garantía no satisface este contrato aunque su registro sea internamente coherente.

## 18. Invariantes SEC.0-X

1. **X2-01 — Cierre del conjunto técnico de confianza.** Todo componente cuyo compromiso pueda falsificar una garantía pertenece al conjunto técnico de confianza de esa garantía.
2. **X2-02 — Separación conceptual no prueba independencia.**
3. **X2-03 — Independencia relativa al fallo.**
4. **X2-04 — Modelo de fallos explícito.**
5. **X2-05 — Raíz no circular.**
6. **X2-06 — Sustitución de raíz no autolegitimada.**
7. **X2-07 — Arranque admitido.**
8. **X2-08 — Cadena de arranque cerrada.**
9. **X2-09 — Fuente pública no acredita ejecutable.**
10. **X2-10 — Construcción declarada.**
11. **X2-11 — Compensación independiente.**
12. **X2-12 — Actualización con autoridad.**
13. **X2-13 — Mediación completa.**
14. **X2-14 — Mediador no ocultamente ampliado.**
15. **X2-15 — Atestación no crea autoridad.**
16. **X2-16 — Actualidad de atestación constituida cuando sea necesaria.**
17. **X2-17 — Atestación de alcance acotado.**
18. **X2-18 — Persistencia material declarada.**
19. **X2-19 — Testigo contra retroceso independiente.**
20. **X2-20 — Aislamiento demostrado.**
21. **X2-21 — Canal autenticado no acredita extremo íntegro.**
22. **X2-22 — Fidelidad de presentación y firma.**
23. **X2-23 — Pluralidad no implica independencia.**
24. **X2-24 — Clave copiable no demuestra unicidad.**
25. **X2-25 — Administración material declarada.**
26. **X2-26 — Mantenimiento dentro del perímetro.**
27. **X2-27 — Observador limitado por su dominio de fallo.**
28. **X2-28 — Degradación explícita.**
29. **X2-29 — Garantía material protegida.**

## 19. Límites del contrato

SEC.0-X no demuestra por sí mismo:

- infalibilidad del soporte físico;
- ausencia de vulnerabilidades desconocidas;
- corrección absoluta de todo compilador o sistema operativo;
- imposibilidad absoluta de extracción de claves;
- disponibilidad perfecta;
- independencia física universal;
- comprensión humana;
- ausencia de coacción;
- ausencia de fallos de fabricación;
- seguridad frente a cualquier atacante con acceso físico ilimitado.

Estas limitaciones deben declararse en el modelo de fallos de cada realización.

## 20. Continuidad

SEC.0-X cierra el contrato abstracto de condiciones materiales necesarias para sostener las garantías de SEC.0-A, SEC.0-D y SEC.0-M.

La traducción de los contratos A, D, M y X a pruebas integrales, la combinación sistemática de fallos y la comprobación de que una realización no cambia su significado corresponden a SEC.0-T.

SEC.0-X no autoriza todavía la selección de una plataforma concreta ni la introducción de tipos, sintaxis o códigos diagnósticos nuevos.
