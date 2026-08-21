# SEC.0-D — Contrato abstracto de diagnóstico y fallo cerrado

**Fecha:** 21/08/2026  
**Estado:** contrato arquitectónico cerrado  
**Ámbito:** Lenguaje SV — SEC.0-D  
**Antecedente:** `CONTRATO_ABSTRACTO_DE_AUTORIDAD_CONSTITUCION_Y_GENESIS_SEC0_A_V2_2026_08_21.md`

## 1. Objeto

SEC.0-D fija el contrato abstracto que determina qué resultados técnicos son admisibles cuando una operación sujeta a control exige acreditar determinadas condiciones y la comprobación las confirma, las refuta o no puede acreditarlas.

El contrato impide que la ausencia, insuficiencia o imposibilidad de verificación se transformen silenciosamente en éxito, autoridad o valor ternario del dominio.

SEC.0-D no modifica gramática, IR, validador, catálogo diagnóstico ni infraestructura de ejecución. Tampoco selecciona una plataforma material ni asigna códigos diagnósticos.

## 2. Relación con SEC.0-A

SEC.0-A distingue información, evidencia admitida, hecho semántico constituido, autoridad, habilitación y ejercicio, y determina las vías legítimas de constitución y transporte de autoridad.

SEC.0-D conserva esas distinciones y establece la consecuencia siguiente:

> Una comprobación técnica puede acreditar o impedir el ejercicio de una autoridad ya existente; no puede crear la autoridad cuya existencia pretende comprobar.

La clase de una transición, su autoridad aplicable y su dominio gobernado no pueden ser creados por el propio resultado diagnóstico.

## 3. Formas sujetas a control

Toda forma concreta `F`, distinta de T-0, queda sujeta a control cuando pueda:

- producir un efecto material protegido;
- modificar autoridad;
- modificar una envolvente máxima de efectos;
- modificar un dominio gobernado;
- alterar una habilitación relevante para un efecto protegido;
- ejecutar recuperación;
- producir un estado persistente del que dependa posteriormente una decisión protegida.

La pertenencia de `F` a este conjunto queda determinada por su constitución. No puede decidirla durante la ejecución el ejecutor, el interesado, el verificador ni el monitor.

Para toda forma sujeta a control, efecto `e` y contexto constitutivo `C`, debe existir un conjunto no vacío de obligaciones:

```text
Req(F,e | C) = N(F,e | C) ∪ S(F,e | C)
```

donde:

- `N(F,e | C)` es el núcleo no eludible de control;
- `S(F,e | C)` contiene las obligaciones específicas adicionales de la forma.

Se establece:

```text
Req(F,e | C) = ∅  ⇒  forma inválida para producir el efecto
```

y queda excluida la interpretación:

```text
Req(F,e | C) = ∅  ⇒  Permit = true
```

## 4. Núcleo no eludible

La aplicabilidad de cada obligación del núcleo queda determinada por la definición constituida de `F`. No puede decidirse localmente durante la ejecución.

Omitir una obligación del núcleo exige una modificación constitutiva de `F` mediante la transición que corresponda; no puede presentarse como un simple «no aplica» operativo.

Toda forma sujeta a control debe conservar, según su definición constituida, las obligaciones siguientes:

1. **Validez de la forma:** `F` existe, está constituida y es aplicable al acto.
2. **Autoridad aplicable:** la autoridad invocada existe, está vigente y habilita la clase de efecto pretendida.
3. **Dominio gobernado:** cuando SEC.0-A exija pertenencia a `D_a`, debe acreditarse `x ∈ D_a` antes del efecto.
4. **Verificación admisible:** cada regla de verificación utilizada está admitida y es aplicable al objeto y al contexto.
5. **Prohibición de acreditación propia:** ninguna regla de verificación puede crear o certificar en el mismo acto la autoridad necesaria para legitimar su propio uso.
6. **Vigencia o no revocación:** cuando la autoridad, la forma o el antecedente dependan de una condición de vigencia o revocación, ésta forma parte del núcleo.

En toda forma sujeta a control, las obligaciones 1, 2, 4 y 5 son siempre nucleares. La obligación 3 lo es cuando `F` opera sobre un dominio `D_a` exigido por SEC.0-A. La obligación 6 lo es cuando `F` invoca una autoridad, forma o antecedente sometido a vigencia o revocación.

Si la validez de `F`, la autoridad aplicable o la pertenencia a `D_a` exigida por SEC.0-A no pueden decidirse a partir de estado ya admitido, el resultado técnico correspondiente es `D-N`. No existe promoción silenciosa a T-G, T-C o T-R.

## 5. Resultados técnicos de comprobación

Para cada obligación `q` se distinguen exactamente tres resultados técnicos abstractos.

### 5.1. D-A — ACREDITADO

Existe evidencia suficiente, admisible y ligada al objeto correcto, evaluada mediante una regla de verificación aplicable, y esa evidencia satisface `q`.

### 5.2. D-R — REFUTADO

Existe evidencia admisible suficiente para establecer que `q` no se cumple.

### 5.3. D-N — NO_VERIFICABLE

No existe base suficiente para acreditar ni refutar `q` dentro del procedimiento exigido.

Entre sus causas posibles se encuentran:

- evidencia ausente o incompleta;
- procedencia de la evidencia no acreditable;
- verificador no disponible;
- verificador no admitido para la obligación;
- versión o contexto no identificables con seguridad suficiente;
- conflicto entre evidencias sin regla gobernada de resolución;
- dependencia técnica necesaria no comprobable;
- imposibilidad de ligar la evidencia al objeto exacto;
- fallo técnico que impide completar la comprobación.

`D-N` describe un estado técnico de comprobación. No es un valor de `Tri`.

## 6. Regla fundamental de fallo cerrado

Una forma sujeta a control sólo puede continuar si todas las obligaciones de `Req(F,e | C)` quedan en `D-A`.

```text
Permit(F,e | C) ⇔
    Req(F,e | C) ≠ ∅
    ∧ ∀q ∈ Req(F,e | C), Check(q) = D-A
```

Por tanto:

```text
∃q : Check(q) = D-R  ⇒  e no se ejecuta
∃q : Check(q) = D-N  ⇒  e no se ejecuta
```

La regla se aplica a T-E y también a toda T-G, T-C o T-R sujeta a control que produzca un efecto o modifique autoridad, gobierno, recuperación o estado protegido.

T-0 conserva el estatuto excepcional fijado por SEC.0-A y no se redefine en este contrato.

La diferencia entre `D-R` y `D-N` debe conservarse en la traza y en el diagnóstico.

## 7. Gobierno, recuperación y excepción

Una T-G, T-C, T-R, excepción gobernada o forma de emergencia no puede emplearse como vía para eludir un `D-R` o un `D-N` obtenido por la forma ordinaria.

Cada forma debe disponer de `Req` propio, conservar el núcleo no eludible y acreditar la autoridad específica que permite ejercerla.

Una forma alternativa puede constituir legítimamente un efecto distinto o más restringido. No puede emplearse para:

- ampliar silenciosamente `E_max`;
- ampliar silenciosamente `D_a`;
- suprimir obligaciones del núcleo;
- convertir un resultado no verificable en permiso.

Cuando una operación de emergencia necesite efectos distintos de los de la forma ordinaria bloqueada, esos efectos deben pertenecer a una forma distinta, previamente constituida, con `D_a`, `E_max`, autoridad y obligaciones propios.

La condición de emergencia no confiere por sí misma un conjunto mayor de poderes.

## 8. Alcance del fallo cerrado

El bloqueo se aplica al efecto cuya condición no ha podido acreditarse y a los efectos que dependan de él según relaciones previamente constituidas.

SEC.0-D no autoriza convertir la indisponibilidad de un verificador local en una paralización general si no existe una dependencia que lo exija.

La arquitectura de recursos, redundancia, disponibilidad y denegación de servicio queda reservada a fases posteriores.

## 9. No equivalencias obligatorias

Quedan prohibidas las identificaciones siguientes:

```text
NO_VERIFICABLE ≠ U
NO_VERIFICABLE ≠ REFUTADO
NO_VERIFICABLE ≠ ACREDITADO
ausencia de evidencia ≠ evidencia de ausencia
error técnico ≠ U
reintento ≠ acreditación
firma válida ≠ contenido verificado
verificador disponible ≠ verificador admitido
resultado almacenado ≠ resultado todavía aplicable
```

Una `U` genuina pertenece a la semántica ternaria del dominio y exige las condiciones propias de ese dominio. SEC.0-D no crea una vía alternativa hacia `U`.

## 10. Selección y aplicabilidad de verificadores

Cada obligación `q` debe declarar qué familia de reglas de verificación es admisible para acreditarla.

Un verificador concreto `V` sólo puede intervenir cuando se satisfacen las condiciones de aplicabilidad constituidas para `q`:

```text
Applicable(V, q, C) = true
```

La mera ejecución de `V` no demuestra su aplicabilidad.

Si la aplicabilidad no puede acreditarse:

```text
Check(q) = D-N
```

salvo que otra regla admisible e independiente permita completar legítimamente la comprobación.

Un verificador no puede acreditar, mediante el mismo acto cuya legitimidad depende de esa acreditación, que él mismo es el verificador autorizado.

## 11. Conflicto entre evidencias o verificadores

Si dos evidencias admisibles o dos verificadores aplicables producen resultados incompatibles, sólo puede obtenerse `D-A` o `D-R` mediante una regla de resolución previamente constituida.

Si no existe tal regla:

```text
Check(q) = D-N
```

No es lícito elegir el resultado favorable, usar una precedencia no constituida, escoger por mera cronología, resolver por mayoría no autorizada ni degradar la discrepancia a advertencia para continuar.

## 12. Agregación de obligaciones

Para una forma con varias obligaciones se define un resultado técnico agregado:

```text
D-R  si al menos una obligación está REFUTADA;
D-N  si ninguna está REFUTADA y al menos una es NO_VERIFICABLE;
D-A  sólo si todas están ACREDITADAS.
```

Esta agregación describe el estado del control y no constituye una operación ternaria del dominio.

La precedencia de `D-R` sobre `D-N` sirve únicamente para conservar la existencia de una refutación material. En ambos casos el efecto protegido queda bloqueado.

## 13. Evidencia almacenada y reutilización

Un resultado de verificación previamente obtenido sólo puede reutilizarse si la regla correspondiente declara explícitamente sus ligaduras de validez.

Como mínimo, deben quedar ligadas las dimensiones que puedan alterar el resultado:

- objeto exacto;
- operación o familia de operaciones;
- contexto constitutivo;
- versión o régimen aplicable;
- antecedente de autoridad relevante;
- evidencia utilizada;
- regla de verificación;
- condición de vigencia cuando proceda.

Si no puede acreditarse que esas ligaduras continúan siendo válidas:

```text
resultado previo ⇒ no reutilizable
Check(q) = D-N
```

La mera existencia de una comprobación histórica no la convierte en comprobación vigente.

## 14. Cobertura parcial

La acreditación de una parte de un objeto, una muestra, una parte estructural o un subconjunto de obligaciones no acredita el conjunto salvo que exista una regla constituida que permita esa inferencia.

```text
Check(subconjunto) = D-A
```

no implica:

```text
Check(totalidad) = D-A
```

sin una regla de cobertura suficiente.

Cuando la cobertura requerida no pueda establecerse, el resultado es `D-N`.

## 15. Reintentos, vigencia y sustitución de verificadores

Un reintento idéntico no transforma por sí mismo `D-N` en `D-A`.

Un `D-N` tampoco borra ni rebaja un `D-R` anterior que siga siendo aplicable. Si existe una refutación vigente sobre una obligación, el resultado continúa siendo `D-R` hasta que una transición legítima modifique el hecho subyacente, la vigencia de la evidencia o la regla que determina su aplicabilidad.

Las obligaciones nucleares relativas a validez de la forma, autoridad, vigencia, revocación, aplicabilidad del verificador y prohibición de acreditación propia no pueden pasar de `D-N` a `D-A` por mera sustitución del verificador.

El cambio de verificador sólo es admisible cuando una regla previamente constituida autoriza esa sustitución y permite acreditar que el nuevo verificador satisface las mismas ligaduras exigidas.

En obligaciones de vigencia o revocación, la indisponibilidad de la fuente o del verificador que consulta el estado de autoridad no permite presumir vigencia. Un verificador sustituto sólo puede producir `D-A` si acredita continuidad con la misma fuente de autoridad o con un estado equivalente cuya equivalencia y continuidad estén previamente gobernadas.

Si esa continuidad no puede acreditarse, el resultado permanece en `D-N`.

## 16. Intervención humana

Una decisión humana no convierte automáticamente `D-N` en `D-A`.

Una persona con autoridad suficiente puede aportar evidencia, realizar una comprobación admitida, ejercer una autoridad de gobierno ya constituida, iniciar una transición legítima de gobierno o constitución o ejercer una excepción previamente constituida.

No puede:

- declarar retrospectivamente que una comprobación inexistente sí ocurrió;
- convertir ausencia de evidencia en evidencia;
- usar una firma genérica para suprimir obligaciones;
- presentar como T-E ordinaria una excepción que materialmente cambia el contrato de autoridad.

Una excepción humana es un acto distinto y debe quedar trazada como tal.

## 17. Modos de emergencia

Una forma de emergencia debe existir previamente como forma gobernada, con:

- autoridad específica;
- ámbito delimitado;
- `D_a` y `E_max` propios;
- núcleo no eludible de control;
- obligaciones adicionales propias;
- traza diferenciada;
- reglas de retorno al régimen ordinario.

No puede equivaler a:

```text
si no se puede verificar, continuar
```

La forma de emergencia puede añadir obligaciones y restringir su dominio o su envolvente. No puede suprimir el núcleo ni convertir en paso un `D-N` o `D-R` de ese núcleo.

Tampoco puede constituirse como un conjunto mayor que la forma ordinaria bloqueada por el solo hecho de ser de emergencia. Los efectos distintos que necesite deben pertenecer a otra forma previamente constituida, con autoridad, dominio y envolvente propios.

## 18. Obligación diagnóstica mínima

Todo bloqueo de un efecto protegido debe poder explicar, como mínimo:

1. qué efecto se intentaba realizar;
2. qué obligación de `Req(F,e | C)` resultó relevante;
3. si el resultado fue `D-R` o `D-N`;
4. qué regla de verificación se aplicó o por qué no pudo aplicarse;
5. qué evidencia fue utilizada o qué evidencia faltó;
6. qué ligadura de contexto impidió la reutilización, cuando proceda;
7. qué transición o nueva evidencia podría cambiar legítimamente el estado, si está definida.

Estas obligaciones son abstractas. SEC.0-D no asigna códigos, nombres de campos de IR ni formato de mensajes.

## 19. Invariantes

### D2-01 — Perímetro constituido de control
La pertenencia de una forma al conjunto sujeto a control queda constituida antes del acto y no puede elegirla el ejecutor o el interesado.

### D2-02 — Requisitos no vacíos
Toda forma sujeta a control debe poseer un `Req` no vacío que incluya el núcleo no eludible aplicable.

### D2-03 — Única vía de paso controlado
Una forma sujeta a control sólo puede producir su efecto cuando todas sus obligaciones exigibles están acreditadas.

### D2-04 — No verificación no es éxito
`D-N` bloquea el efecto protegido.

### D2-05 — Refutación no es ausencia
`D-R` debe conservarse como resultado distinto de `D-N`.

### D2-06 — Separación respecto de `Tri`
Ningún resultado D-A/D-R/D-N modifica por sí mismo el valor ternario del dominio.

### D2-07 — Aplicabilidad acreditable
Un resultado sólo vale si la regla de verificación era aplicable a la obligación, al objeto y al contexto.

### D2-08 — Prohibición de acreditación propia
Un verificador no puede constituir la autoridad que necesita para validar su propio uso.

### D2-09 — Conflicto sin regla
Evidencias incompatibles sin regla gobernada de resolución producen `D-N`.

### D2-10 — Reutilización ligada
Una comprobación almacenada sólo puede reutilizarse mientras sus ligaduras de validez permanezcan acreditadas.

### D2-11 — Cobertura no extensiva por defecto
Acreditar una parte no acredita el conjunto sin una regla de cobertura.

### D2-12 — Reintento no creador
Repetir una comprobación sin nueva base no crea acreditación.

### D2-13 — Excepción no silenciosa
Una excepción gobernada debe aparecer como acto distinto, no como éxito ordinario.

### D2-14 — Fallo cerrado de alcance mínimo
El efecto dependiente queda bloqueado; la indisponibilidad no se extiende a efectos independientes sin una dependencia constituida.

### D2-15 — Diagnóstico trazable
Todo bloqueo material debe conservar causa técnica suficiente para distinguir refutación, imposibilidad de comprobación y cambio legítimo de contexto.

### D2-16 — Diagnóstico sin autoridad
El resultado diagnóstico informa y gobierna el paso según una regla ya constituida; no crea autoridad por sí mismo.

### D2-17 — Gobierno y recuperación no eluden el núcleo
Toda T-G, T-C o T-R sujeta a control posee requisitos propios y conserva el núcleo no eludible.

### D2-18 — Emergencia no amplificadora
Una forma de emergencia no puede constituirse como conjunto mayor que la forma ordinaria bloqueada por el solo hecho de ser de emergencia. Los efectos distintos que necesite deben pertenecer a otra forma previamente constituida, con autoridad, dominio y envolvente propios.

### D2-19 — Refutación persistente
Un `D-N` posterior no rebaja un `D-R` anterior que siga siendo aplicable.

### D2-20 — Sustitución gobernada de verificadores
El cambio de verificador no puede transformar por sí mismo una obligación nuclear no verificable en acreditada. Requiere una regla constituida de sustitución y la acreditación de sus ligaduras. Para vigencia o revocación, el sustituto debe conservar continuidad acreditada con el estado de autoridad correspondiente.

## 20. Casos de comprobación

Los casos siguientes delimitan el comportamiento exigido por el contrato.

### 20.1. Verificador no disponible
Si una operación requiere una obligación y el verificador admitido no está disponible, el resultado es `D-N` y el efecto queda bloqueado.

### 20.2. Verificador favorable no admitido
Un resultado favorable emitido por un verificador no admitido no acredita la obligación ni modifica un resultado aplicable emitido por una regla válida.

### 20.3. Evidencia contradictoria
Dos evidencias admisibles incompatibles, sin regla constituida de resolución, producen `D-N`.

### 20.4. Comprobación histórica bajo contexto distinto
Una comprobación previa no se reutiliza si no pueden acreditarse sus ligaduras respecto del contexto actual.

### 20.5. Cobertura parcial
La comprobación favorable de una muestra o parte estructural no acredita el conjunto sin una regla constituida de cobertura.

### 20.6. Intervención humana genérica
Una persona no puede convertir `D-N` en `D-A` mediante una aceptación genérica. Sólo puede actuar mediante evidencia o autoridad previamente constituidas.

### 20.7. Eliminación local de una obligación
El ejecutor no puede suprimir una obligación de `Req(F,e | C)` durante la ejecución.

### 20.8. Confusión con `U`
La imposibilidad de verificar procedencia u otra obligación técnica produce `D-N`, no `U`.

### 20.9. Propagación indebida del bloqueo
La indisponibilidad de un verificador no justifica bloquear efectos independientes si no existe una dependencia constituida.

### 20.10. Requisitos vacíos
Una forma con `Req = ∅` es inválida para producir el efecto; la vaciedad nunca produce permiso.

### 20.11. Gobierno posterior a un bloqueo
Una T-G destinada a modificar `D_a` después de un `D-N` debe satisfacer su propio `Req` y acreditar su propia autoridad.

### 20.12. Revocación y posterior indisponibilidad
Una refutación vigente no desaparece porque posteriormente deje de estar disponible el verificador ordinario.

### 20.13. Emergencia con control insuficiente
Una forma de emergencia que omita el núcleo no eludible es inválida para el efecto protegido.

## 21. Premisas materiales no demostradas

SEC.0-D exige que la acreditación utilizada para permitir un efecto siga siendo aplicable en el momento material de ese efecto.

El contrato no demuestra la indivisibilidad material entre comprobación y ejecución ni impide por sí mismo una modificación concurrente del estado entre ambas.

Por tanto:

```text
Check(q) = D-A en t0
```

no basta si en el instante de ejecución `t1` no puede acreditarse que las ligaduras relevantes siguen vigentes.

La realización material de esa continuidad —mediante una operación indivisible, bloqueo, versión de estado, testimonio equivalente u otro mecanismo— pertenece a las fases de seguridad material y continuidad.

Hasta que esa garantía exista, una acreditación caducada o cuya vigencia en el momento del efecto no pueda establecerse debe tratarse como `D-N`, no como permiso vigente.

## 22. Deudas reservadas

Quedan fuera de SEC.0-D:

- disponibilidad de evidencias y verificadores;
- almacenamiento y caducidad material de resultados;
- consumo de recursos y denegación de servicio;
- persistencia y continuidad;
- integridad material del verificador;
- aislamiento y cadena de construcción;
- materialización de las comprobaciones en pruebas integrales;
- tipos concretos, sintaxis, IR, códigos diagnósticos, mensajes e interfaz de programación de verificadores.

Estas materias pertenecen a las fases posteriores que correspondan y no se consideran resueltas por este contrato.

## 23. Criterios de suficiencia del contrato

SEC.0-D se considera suficiente cuando quedan satisfechas las condiciones siguientes:

1. el conjunto de formas sujetas a control queda constituido antes del acto;
2. `Req = ∅` nunca produce permiso;
3. toda forma controlada conserva un núcleo no eludible;
4. T-E, T-G, T-C y T-R sujetas a control sólo avanzan con acreditación completa;
5. `D-R` y `D-N` bloquean sin confundirse;
6. `D-N` no se transforma en `U` ni en éxito;
7. la aplicabilidad del verificador está gobernada y la acreditación propia queda prohibida;
8. la reutilización de verificaciones exige ligaduras vigentes;
9. la cobertura parcial no se extiende sin regla;
10. las excepciones humanas o de emergencia no suprimen el núcleo ni amplían implícitamente autoridad, dominio o envolvente;
11. un `D-R` aplicable no puede sustituirse por un `D-N` posterior para alterar el resultado;
12. el fallo cerrado se mantiene en el alcance mínimo exigido por las dependencias constituidas;
13. el diagnóstico permite reconstruir por qué un efecto no avanzó;
14. la falta de decidibilidad de `F`, de la autoridad aplicable o de `x ∈ D_a` exigido por SEC.0-A produce `D-N`;
15. las premisas materiales y las materias posteriores permanecen separadas y no se utilizan como prueba de SEC.0-D.

## 24. Estado

Las condiciones anteriores quedan fijadas como contrato arquitectónico de SEC.0-D.

SEC.0-D queda cerrado como **contrato abstracto de diagnóstico y fallo cerrado**.
