# Gramática superficial mínima del Lenguaje SV — v0.2

## Sucesora normativa de v0.1 para admisibilidad, `resolve` y coherencia de `Frame`

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 23 de agosto de 2026  
**Estado:** Especificación técnica pública — v0.2

---

## 1. Estatuto y relación con v0.1

La versión 0.2 conserva íntegramente la gramática v0.1 salvo en las producciones y restricciones que este documento sustituye de forma expresa.

Por tanto:

```text
Gramática v0.2
= Gramática v0.1
+ correcciones de este documento
```

Las construcciones no mencionadas aquí mantienen su sintaxis y su estatuto anteriores. La versión 0.1 se conserva como antecedente histórico y no se reescribe retrospectivamente.

La v0.2 baja a la IR canónica v0.3.

---

## 2. Objetivos de la revisión

Esta versión corrige tres fronteras observables del frontend:

1. separa los estados de admisibilidad técnica del valor semántico `Tri.U`;
2. obliga a que `resolve` identifique una ocurrencia constituida de `U` mediante estado y posición;
3. conserva la sintaxis de `Frame`, pero somete sus colecciones derivadas a las reglas de cierre estructural y causal de IR v0.3.

No introduce un cuarto valor ternario ni un cuarto impacto semántico independiente.

---

## 3. Admisibilidad técnica

### 3.1. Estados cerrados

La producción v0.1:

```text
{Ok, Degraded, Failed, U}
```

queda sustituida por el conjunto cerrado:

```text
{Ok, Degraded, NotAdmitted}
```

El orden superficial de los tres identificadores no tiene significado semántico; el conjunto debe contener exactamente una vez cada estado.

Producción normativa:

```ebnf
admissibility_state        ::= "Ok" | "Degraded" | "NotAdmitted" ;

admissibility_decl         ::= "admissibility_spec" identifier "{"
                               "parameter_id" ":" nat ";"
                               "states" ":" "{" admissibility_state ","
                                                admissibility_state ","
                                                admissibility_state "}" ";"
                               "rule" ":" identifier ";"
                               "}" ;
```

Reglas de bienformación:

- `parameter_id > 0`;
- `states` contiene exactamente `Ok`, `Degraded` y `NotAdmitted`;
- `rule` no puede estar vacío.

El incumplimiento se diagnostica como:

```text
E110 — InvalidAdmissibilitySpec
```

### 3.2. Separación respecto de `Tri`

`Failed` deja de formar parte de `AdmissibilitySpec`. El fallo de captura continúa representado por el símbolo técnico `Bottom` de `CaptureSpec`.

No existen coerciones automáticas:

```text
Bottom       ↛ Tri
NotAdmitted  ↛ Tri
fallo técnico ↛ Tri.U
```

Una observación admitida puede producir legítimamente `Tri.U` únicamente a través de un `Ternarizer` cuya partición `partition_u` la clasifique en la región semántica correspondiente.

---

## 4. Objetivo explícito de `resolve`

### 4.1. `ResolutionTarget`

Se introduce una forma superficial cerrada para identificar el objeto revisado:

```ebnf
resolution_target          ::= "(" identifier "," nat ")" ;
```

Su significado es:

```text
ResolutionTarget = (EvaluableStateRef, position)
```

La posición es uno-basada.

### 4.2. Nueva producción de `resolve`

La producción v0.1 que aceptaba el literal abstracto `U` queda sustituida por:

```ebnf
resolve_cmd                ::= "let" identifier "=" "resolve" "("
                               resolution_target ","
                               "with" ":" identifier ","
                               "context" ":" identifier ","
                               "mechanism" ":" identifier
                               ")" ";" ;
```

Ejemplo:

```svp
let RR1 = resolve((S1, 3),
                  with: RS1,
                  context: ContextoClinico,
                  mechanism: RevisionExperto);
```

Reglas de bienformación:

1. el estado referenciado debe ser un `CellState` o `CoupledState` evaluable;
2. la posición debe existir;
3. el valor efectivo de la posición debe ser `U`;
4. `with` debe referir un `ResSpec`;
5. por defecto, la instancia `(context, mechanism)` debe coincidir exactamente con `(ResSpec.context, ResSpec.mechanism)`.

El incumplimiento de estas condiciones se diagnostica como:

```text
E305 — UnsafeUResolution
```

`resolve` representa revisión de una `U` constituida. La mera ejecución de la revisión no confiere por sí sola una clausura positiva de esa `U`.

---

## 5. Proyección de resultados de `resolve`

La forma general de proyección no cambia:

```ebnf
projection_cmd             ::= "let" identifier "=" identifier "." identifier ";" ;
```

Para un `ResolutionRecord` v0.3 se reconocen los campos:

```text
target
previous
reviewed_to
resolved_to
context_ref
mechanism_ref
```

Ejemplo válido:

```svp
let valor_resuelto = RR1.resolved_to;
```

La existencia del campo `resolved_to` no significa que el programa pueda fabricar una clausura positiva.

---

## 6. `Frame`: sintaxis conservada, bienformación reforzada

La producción superficial de `Frame` se conserva:

```ebnf
frame_decl                 ::= "frame" identifier "{"
                               "index" ":" nat ";"
                               "architecture" ":" identifier ";"
                               "cell_states" ":" list<identifier> ";"
                               "eval_results" ":" list<identifier> ";"
                               "gate_results" ":" list<identifier> ";"
                               "supervision" ":" list<identifier> ";"
                               "criticalities" ":" list<identifier> ";"
                               "}" ;
```

La IR v0.3 exige, sin imponer exhaustividad:

- `architecture` referencia un `CompositionGraph`;
- existe como máximo un `CoupledState` por nodo de esa arquitectura;
- dos nodos distintos pueden compartir el mismo `CellSpec` mediante `CoupledSpec` distintos;
- cada evaluación incluida evalúa un estado del propio `Frame`;
- no hay dos evaluaciones materiales de la misma fuente dentro del mismo `Frame`;
- cada compuerta incluida depende sólo de evaluaciones incluidas;
- cada supervisión incluida conserva su meta-evaluación y objetivo dentro del mismo cierre;
- un `SystemTarget` de supervisión debe coincidir con `Frame.architecture`;
- mientras no exista productor superficial constituido de `CriticalityResult`, `criticalities = []`.

Las violaciones de este cierre se diagnostican como:

```text
E308 — FrameClosureViolation
```

---

## 7. Versionado observable

La etapa frontal v0.2 debe emitir en la cabecera canónica:

```json
{
  "grammar_version": "0.2",
  "ir_version": "0.3",
  "serializer_version": "0.1.0"
}
```

---

## 8. Elementos que no cambian

Esta versión no introduce:

- nuevos literales de `Tri`;
- `max` o `min` en la superficie;
- authoring superficial completo de `ConflictOperator`;
- primitivas de tiempo, reloj o UTC;
- `deployment_profile` como construcción del Lenguaje;
- TCB, raíz de confianza o atestación como tipos de IR;
- productor superficial de `CriticalityResult`;
- habilitación de `PendingU`.

La deuda de `ConflictOperator` en régimen `General` permanece visible y no queda resuelta por E308 ni por ninguna de las tres correcciones anteriores.

---

## 9. Compatibilidad

Un programa v0.1 que utilice:

```text
states: {Ok, Degraded, Failed, U}
```

o:

```text
resolve(U, ...)
```

no es conforme con la gramática v0.2.

Las demás construcciones v0.1 permanecen compatibles mientras satisfagan los juicios de bienformación de IR v0.3.

---

## 10. Dictamen técnico

La gramática v0.2 mantiene la superficie austera del Lenguaje SV y corrige exclusivamente las fronteras necesarias para impedir que un fallo técnico se convierta en semántica ternaria, que una revisión opere sobre un `U` abstracto sin identidad y que un `Frame` pueda declarar resultados ajenos a su propio cierre estructural o causal.
