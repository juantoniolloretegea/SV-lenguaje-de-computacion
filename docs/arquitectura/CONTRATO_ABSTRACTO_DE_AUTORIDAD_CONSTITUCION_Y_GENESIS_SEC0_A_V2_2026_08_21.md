# SEC.0-A V2 — Contrato abstracto de autoridad, constitución y génesis

**Fecha:** 21/08/2026  
**Estado:** contrato arquitectónico cerrado  
**Ámbito:** Lenguaje SV — SEC.0-A  
**Vigencia:** para T-0 y sus referencias derivadas, este contrato debe leerse conjuntamente con [`ADENDA_CORRECTIVA_SEC0_A_UNIDAD_DE_GENESIS_Y_CONTINUIDAD_AUTORITATIVA_2026_08_22.md`](./ADENDA_CORRECTIVA_SEC0_A_UNIDAD_DE_GENESIS_Y_CONTINUIDAD_AUTORITATIVA_2026_08_22.md), que prevalece ante cualquier lectura incompatible.

## 1. Objeto

SEC.0-A fija el contrato abstracto que determina qué puede constituir, transportar o ejercer autoridad en SV; mediante qué clases de transición puede nacer legítimamente esa autoridad; y qué vías ordinarias deben ser incapaces de fabricarla o ampliarla.

El contrato no modifica gramática, IR, validador, catálogo diagnóstico ni infraestructura de ejecución. Tampoco selecciona una plataforma material. Su función es establecer propiedades que deberán conservar las fases posteriores.

Dos condiciones estructurales rigen el contrato:

1. la separación conceptual entre componentes no demuestra aislamiento material;
2. la invariancia de una tabla de concesiones no demuestra la invariancia de la envolvente máxima de efectos.

## 2. Distinciones fundamentales

Se distinguen seis clases que no pueden identificarse entre sí:

1. **Información:** contenido recibido, leído, calculado o inferido.
2. **Evidencia admitida:** información que ha superado una regla de verificación y admisión previamente autorizada.
3. **Hecho semántico constituido:** objeto que pertenece legítimamente al dominio declarado.
4. **Autoridad:** facultad conferida por una transición legítima para producir una familia delimitada de efectos.
5. **Habilitación:** condición actual que permite ejercer una autoridad ya existente.
6. **Ejercicio:** realización material de un efecto autorizado.

Quedan prohibidas las siguientes identificaciones:

- información ⇒ autoridad;
- verificación ⇒ concesión;
- constitución semántica ⇒ permiso;
- capacidad técnica ⇒ autoridad;
- ejecución material ⇒ titularidad;
- firma criptográfica ⇒ comprensión humana;
- copia o reconstrucción desde una representación codificada ⇒ legitimidad;
- historial local válido ⇒ continuidad vigente única.

## 3. Magnitudes de autoridad

Para cada autoridad `a` y contexto constitutivo `C` se distinguen cuatro magnitudes.

### 3.1. Autoridad constitutiva

Declara quién puede realizar qué clase de operación y bajo qué condiciones estructurales. No se identifica con el objeto que la representa materialmente.

### 3.2. Envolvente máxima de efectos

Se denota por:

```text
E_max(a | C)
```

Es el conjunto máximo de efectos —o de trazas de efectos cuando la acumulación sea material— que `a` puede producir legítimamente mientras no exista una transición de gobierno o constitución que modifique esa autoridad.

Si `Enabled(a,I)` representa los efectos habilitados bajo información admisible `I`, debe cumplirse:

```text
Enabled(a,I) ⊆ E_max(a | C)
```

para toda información admisible `I`.

Cuando una concesión depende de información mutable, su envolvente máxima debe quedar delimitada con independencia de esa información. Una concesión no puede presentarse como estrecha por el mero hecho de que, en el estado actual, sólo habilite pocos efectos.

### 3.3. Dominio gobernado de autoridad

El ejercicio automático mediante T-E exige un dominio `D_a` que satisfaga conjuntamente:

1. la pertenencia `x ∈ D_a` puede decidirse antes del ejercicio a partir de estado constitutivo o de gobierno previamente admitido;
2. T-I, T-V, T-H y T-E no pueden ampliar por sí solas la definición ni la extensión autorizada de `D_a`;
3. toda ampliación de `D_a` requiere T-G o T-C, salvo su constitución inicial por T-0;
4. la relación entre `D_a` y `E_max(a | C)` permite decidir antes de ejecutar si el efecto propuesto queda dentro de la envolvente autorizada;
5. no basta una definición tautológica que acepte, sin límite gobernado, todo objeto o efecto que pueda aparecer en el futuro.

Una regla generativa o inductiva sólo puede operar sobre un conjunto portador previamente gobernado cuya extensión no pueda ampliarse mediante T-I, T-V, T-H ni T-E. La mera relación «descendiente de un miembro de `D_a`» no incorpora por sí sola nuevos objetos a `D_a`; esa incorporación exige T-G o T-C.

### 3.4. Habilitación

La habilitación puede variar con evidencia, estado o condiciones operativas, pero no puede ensanchar `E_max`.

### 3.5. Ejercicio y acumulación

La corrección de un acto aislado no basta cuando su repetición o composición puede producir un efecto global distinto. Toda forma de T-E repetible o acumulable debe declarar un contrato suficiente de acumulación: singularidad, carácter idempotente, función `Agg_F` o predicado decidible equivalente sobre la traza.

Sin ese contrato no existe ejercicio automático repetido válido.

## 4. Clases abstractas de transición

Las clases siguientes son semánticas; no constituyen tipos de IR.

### T-I — Transición informativa

Puede modificar información ordinaria. No puede crear, ampliar o transportar autoridad; modificar la regla de admisión; ampliar `E_max`; crear compatibilidad entre constituciones; ni seleccionar por sí sola la continuidad vigente.

### T-V — Transición de verificación y admisión

Puede convertir información en evidencia admitida conforme a una regla previamente autorizada. No puede crear su propio verificador, ampliar autoridad por el hecho de verificar, modificar silenciosamente la regla de admisión ni acreditar por sí sola a la fuente evaluada.

### T-H — Transición de habilitación

Puede activar o desactivar el ejercicio de una autoridad ya existente conforme a condiciones previamente fijadas. Debe conservar:

```text
Enabled(a) ⊆ E_max(a)
```

### T-E — Transición de ejercicio

Realiza un efecto ya habilitado. El ejecutor material puede ser distinto del titular, pero no adquiere por ello la autoridad del titular.

T-E automática sólo es admisible cuando puede decidirse antes del ejercicio que el efecto pertenece a `E_max` dentro de un `D_a` gobernado. Si esa pertenencia no puede decidirse con las premisas autorizadas, la operación requiere una transición de gobierno o constitución adecuada.

### T-G — Transición de gobierno

Puede conceder, revocar, delegar o modificar autoridad dentro del régimen constitutivo que la habilita. También pertenecen a esta clase las modificaciones de reglas de verificación, relaciones de compatibilidad, asociaciones entre persona, función y ámbito, y actualizaciones de predicados que alteren la envolvente efectiva de una concesión.

### T-C — Transición constitutiva

Modifica la constitución en un eje que altera las condiciones de validez de la autoridad: fase, ITI aplicable, régimen constitutivo, límites estructurales u otras condiciones equivalentes.

### T-0 — Transición de génesis

Constituye el primer estado legítimo de autoridad de una instancia. No puede probar su propia legitimidad desde el sistema que todavía está naciendo. Su validez depende de premisas externas expresas sobre identidad, autoridad constituyente, integridad suficiente del procedimiento y correspondencia entre el objeto constituido y el ejecutado.

### T-R — Transición de recuperación

Restablece autoridad preexistente bajo una continuidad legítima. No puede crear autoridad nueva, ampliar la envolvente anterior, elegir silenciosamente entre continuidades incompatibles ni reinterpretar un historial antiguo bajo una constitución posterior sin compatibilidad autorizada.

## 5. Clasificación no discrecional

La clase T-* de un acto no puede ser elegida por el sujeto, el ejecutor, el monitor ni el componente interesado en su aceptación.

Toda forma concreta de transición debe haber sido constituida previamente mediante un descriptor semántico de efecto `F` que fije, al menos:

- clase T-*;
- familia de efectos;
- ligaduras de contexto;
- autoridad previa necesaria;
- regla de acumulación cuando corresponda.

El conjunto inicial de formas se establece por T-0. Introducir una forma nueva o modificar materialmente su clase o familia de efectos exige T-C. T-G puede conceder, revocar o parametrizar autoridad sobre formas ya constituidas, pero no reclasificar una ejecución para convertir un cambio de autoridad en T-I, T-V, T-H o T-E.

Si el efecto real queda fuera del descriptor de la forma, el acto no es una instancia válida de ella.

## 6. Vías legítimas de constitución de autoridad

Una autoridad puede llegar legítimamente a existir únicamente por:

1. T-0 de génesis;
2. T-C válidamente autorizada;
3. T-G válidamente autorizada;
4. T-R que restaure sin ampliar una autoridad anterior y cuya continuidad sea acreditable.

T-I, T-V, T-H y T-E nunca constituyen autoridad.

Rige además la prohibición de circularidad:

> Ningún sujeto, agente, programa, verificador, servicio o proceso puede fabricar la autoridad que necesita para validar el acto mediante el cual adquiere esa misma autoridad.

Por tanto, un verificador no se acredita a sí mismo; un agente de seguridad no legitima su propia génesis; un agente especializado no adquiere gobierno constitutivo por el mero hecho de estar constituido; y una IA o servicio auxiliar no puede ser origen autónomo de una transición constitutiva.

## 7. Transporte, delegación y compatibilidad

Transportar autoridad no equivale a copiar un objeto.

Una delegación ordinaria debe satisfacer:

```text
E_max(a_delegada) ⊆ E_max(a_origen)
```

Si el resultado es más amplio, existe una nueva concesión y debe tratarse como T-G o T-C.

Una relación de compatibilidad que permita reutilizar una autorización bajo otra fase, ITI, régimen de gobierno o constitución es ella misma un acto que confiere capacidad de reutilización. No puede surgir por mera copia, reconstrucción desde una representación codificada, dato externo o regla informativa.

El privilegio del sistema operativo, del hipervisor o de una cuenta técnica no constituye por sí mismo autoridad SV.

## 8. Ligaduras de contexto

Cada acto de autoridad debe quedar ligado a las dimensiones materialmente pertinentes. Según la operación, podrán incluir:

- titular;
- ejecutor, si es distinto;
- operación;
- ámbito;
- objeto;
- estado;
- candidato o alternativa de decisión;
- antecedente histórico;
- fase del agente;
- versión de ITI;
- régimen de gobierno;
- constitución de referencia;
- decisión previa de la que dependa;
- entorno de ejecución cuando afecte a la validez.

La ausencia de una dimensión sólo es admisible si puede demostrarse que su variación no amplía ni reutiliza indebidamente la autoridad.

## 9. Verificación gobernada

El resultado de verificar es evidencia, no autoridad.

La regla que determina quién o qué puede verificar debe estar autorizada antes del hecho concreto. Una fuente no puede aportar simultáneamente el contenido y la regla suficiente que la acredita, salvo que esa regla ya hubiera sido incorporada por una transición de gobierno anterior.

Cambiar un perfil de verificación de modo que cambie qué contenidos pueden habilitar efectos es T-G o T-C, no T-I ni T-V.

Si la verificación no puede realizarse, SEC.0-A prohíbe convertir esa imposibilidad en éxito. El tratamiento exacto corresponde a SEC.0-D.

## 10. Concesiones dependientes de información mutable

Una concesión puede usar un predicado dinámico únicamente si la envolvente máxima de sus efectos queda fijada con independencia de la información que alimente ese predicado.

Se distinguen tres regímenes:

1. **Predicado dentro de una envolvente fija:** la información selecciona elementos de un dominio ya gobernado; cambia habilitación, no autoridad.
2. **Predicado que define la propia envolvente:** si una actualización introduce nuevos objetos dentro del dominio sobre el que existe facultad material, esa actualización es T-G o T-C.
3. **Envolvente no acotable de forma decidible para ejercicio automático:** la autoridad puede existir como capacidad gobernable, pero no habilita T-E automática hasta que una transición de gobierno o constitución delimite suficientemente el efecto o la familia de efectos.

## 11. Composición y acumulación

No pueden utilizarse ni la composición de actos no autorizantes ni la acumulación de ejercicios autorizados para producir un efecto fuera de la envolvente.

Toda forma de T-E que pueda repetirse o interactuar materialmente con ejercicios anteriores debe declarar desde su constitución uno de estos contratos:

- singularidad o consumo;
- carácter idempotente demostrado;
- función `Agg_F(e_1,...,e_n)`;
- predicado decidible sobre la traza.

Los límites concretos de recursos, frecuencia, duración y disponibilidad pertenecen a SEC.0-M. SEC.0-A fija únicamente la obligación de que la acumulación no pueda quedar fuera del contrato por ausencia de definición.

## 12. Actos reservados a personas

SEC.0-A protege únicamente los actos que la doctrina o el régimen de gobierno declaren expresamente reservados a una persona.

En esos actos:

- una IA puede informar, calcular, proponer o preparar;
- un proceso puede ejecutar materialmente después de una autorización válida;
- ningún procedimiento computacional puede sintetizar el acto humano reservado;
- una automatización no puede convertirse en titular autónomo de autoridad constitutiva;
- la ausencia del acto humano exigido no puede sustituirse por plausibilidad, mayoría algorítmica ni funcionamiento degradado.

Una autorización humana general sólo puede cubrir operaciones repetitivas dentro de una envolvente previamente delimitada.

## 13. Firma humana, presentación y quórum

Una firma criptográfica acredita, bajo sus premisas, el uso de una credencial sobre un objeto. No demuestra por sí sola que la persona comprendiera la consecuencia presentada.

Toda garantía que dependa de un acto humano efectivo queda condicionada a que la persona identificada actúe sobre una representación fiel del objeto y del alcance relevante de la decisión.

El número de firmantes tampoco demuestra independencia. Un quórum que comparta identidad, interfaz, resumen o cadena técnica crítica puede constituir una sola dependencia material o cognitiva.

## 14. Conjunto técnico de confianza

SEC.0-A no demuestra la integridad del soporte físico, los microprogramas, el sistema operativo, el compilador, el visor de firma ni los mecanismos de almacenamiento.

Cada garantía material queda condicionada a un conjunto técnico de confianza efectivo:

```text
TCB(g) = {componentes cuya integridad es necesaria para que g sea cierta}
```

La separación funcional no demuestra que dos garantías tengan conjuntos técnicos de confianza independientes.

El monitor de referencia debe ser completo respecto de una política de mediación deliberadamente acotada. Si para decidir necesita interpretar semántica de dominio, el componente que realiza esa interpretación pasa a formar parte del conjunto técnico de confianza de la garantía correspondiente.

## 15. Continuidad y recuperación

Persistir un objeto, copiarlo, indexarlo o marcarlo como actual no le confiere autoridad.

Un historial localmente válido tampoco demuestra por sí mismo qué continuidad debe conservar autoridad cuando existen réplicas o bifurcaciones.

La recuperación exige una autoridad o mecanismo de recuperación previamente constituido. Si el procedimiento de recuperación crea autoridad nueva, amplía la anterior o decide entre continuidades incompatibles, deja de ser una mera T-R y requiere T-G o T-C.

La resolución material de bifurcaciones, continuidad, almacenamiento y recuperación corresponde a SEC.0-M/X.

## 16. Agentes especializados, seguridad e IA

Un agente especializado, incluido un eventual agente de seguridad:

- no adquiere gobierno constitutivo por su propia constitución;
- no puede acreditar su propia génesis;
- no puede ampliar `E_max` mediante conocimiento nuevo;
- no convierte observación en evidencia independiente cuando observa desde el mismo conjunto técnico comprometido;
- sólo puede ejercer automáticamente efectos comprendidos en una envolvente previamente autorizada.

Las IA y los servicios auxiliares pueden producir información, análisis y propuestas. La capacidad intelectual no constituye autoridad.

## 17. Invariantes de SEC.0-A V2

- **A2-01 — Separación de información y autoridad.** T-I, T-V, T-H y T-E no crean autoridad constitutiva.
- **A2-02 — Vías legítimas de constitución.** Toda autoridad nueva procede de T-0, T-G, T-C o de una T-R no amplificadora.
- **A2-03 — No autoconstitución.** Ningún sujeto puede producir mediante una vía ordinaria la autoridad necesaria para validar su propia constitución, verificación o elevación.
- **A2-04 — Envolvente independiente de información.** `E_max` no cambia por información ordinaria, evidencia o habilitación.
- **A2-05 — Cierre efectivo de concesiones dependientes de información.** Toda T-E automática opera sobre un `D_a` gobernado y decidible antes del ejercicio; ampliar `D_a` exige T-G o T-C.
- **A2-06 — Verificación gobernada.** La regla de admisión no puede autoconstituirse ni cambiar por efecto de la información que evalúa.
- **A2-07 — Delegación no amplificadora.** El transporte ordinario no aumenta la envolvente ni transforma al ejecutor en titular.
- **A2-08 — Ligadura suficiente.** La autoridad queda ligada a todas las dimensiones cuya variación pueda producir reutilización o ampliación indebida.
- **A2-09 — Compatibilidad con autoridad.** Toda compatibilidad, migración o reutilización que habilite autoridad en otro contexto requiere gobierno.
- **A2-10 — Composición no amplificadora.** Toda T-E repetible o acumulable declara un contrato suficiente de acumulación.
- **A2-11 — Gobierno concurrente.** La validez respeta fase, ITI y régimen de gobierno; el cambio de cualquiera de ellos no autoriza reutilización silenciosa.
- **A2-12 — Reserva humana acotada.** Los actos expresamente reservados a una persona no pueden ser sintetizados por procedimiento computacional.
- **A2-13 — Historia local no constituye vigencia.** Persistencia, copia, índice, historial local o marca de actualidad no crean continuidad vigente.
- **A2-14 — Conjunto técnico de confianza declarado.** Toda garantía material debe identificar los componentes de cuya integridad depende.
- **A2-15 — Agente no legislador por defecto.** Un agente especializado o de seguridad no adquiere gobierno constitutivo por su capacidad técnica.
- **A2-16 — Firma no equivale a comprensión.** La firma criptográfica no demuestra por sí sola deliberación humana.
- **A2-17 — Clasificación constituida, no elegida.** La clase T-* pertenece a una forma previamente constituida y no puede ser elegida o modificada durante el acto por el ejecutante.

## 18. Casos integrales de comprobación

El contrato debe conservar sus invariantes, como mínimo, ante los casos siguientes:

1. concesión dependiente de una base de conocimiento mutable cuyo dominio efectivo pretende crecer mediante información ordinaria;
2. fuente que aporta simultáneamente contenido y regla suficiente para declararse verificada;
3. reutilización de una autorización antigua bajo un régimen de gobierno posterior;
4. repetición de actos individualmente válidos cuya acumulación produce un efecto global no permitido;
5. firma válida sobre una presentación que oculta la consecuencia material;
6. bifurcación con dos historiales localmente válidos y un índice que pretende decidir por sí solo cuál conserva autoridad;
7. copia del código y de la estructura de una instancia sin génesis ni continuidad legítimas;
8. componente que pretende registrar como transición informativa un efecto que materialmente amplía autoridad.

## 19. Premisas externas explícitas

SEC.0-A no afirma haber demostrado:

1. integridad del soporte físico, los microprogramas, el compilador, el sistema operativo o el almacenamiento;
2. correspondencia entre el proceso medido y el que realmente se ejecuta;
3. fidelidad del visor mediante el que una persona conoce lo que firma;
4. deliberación humana efectiva;
5. independencia material o cognitiva de un quórum;
6. unicidad global de continuidad entre réplicas o bifurcaciones;
7. disponibilidad ante agotamiento de recursos;
8. frescura cronológica cuando una política dependa del tiempo;
9. recuperación material cuando los actores ordinarios estén comprometidos.

## 20. Cuestiones fuera del alcance de SEC.0-A

Permanecen asignadas a fases posteriores:

- SEC.0-D: estado `NO_VERIFICABLE`, rechazo explícito, ausencia de conversión silenciosa del fallo en éxito y obligaciones diagnósticas;
- SEC.0-M: persistencia autoritativa, índices, bifurcaciones, recursos, disponibilidad, denegación de servicio, atención humana como recurso y recuperación material;
- SEC.0-X: arranque, soporte físico, microprogramas, sistema operativo, compilador, cadena de construcción, atestación, monitor material, aislamiento y plataforma de referencia;
- SEC.0-T: comprobaciones adaptativas, casos integrales y correspondencia entre contrato y realización;
- fases posteriores: tipos de IR, cambios de gramática, diagnósticos concretos, funciones de usuarios, modelo estable de responsabilidad, licencias, distribución reconocida e infraestructura general de ejecución.

## 21. Cierre

SEC.0-A queda cerrado como **contrato abstracto de autoridad, constitución y génesis**.

El cierre fija propiedades que deberán conservar las fases posteriores. No acredita por sí mismo una realización material ni autoriza cambios de IR, gramática, diagnósticos o infraestructura de ejecución.

SEC.0-D permanece separado y sólo puede abrirse mediante decisión expresa.
