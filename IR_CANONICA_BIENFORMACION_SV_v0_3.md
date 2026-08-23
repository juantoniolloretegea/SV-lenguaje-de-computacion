# IR canónica y sistema de bienformación del Lenguaje SV — v0.3

## Sucesora normativa de v0.2 para admisibilidad, resolución identificada y cierre relacional de `Frame`

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 23 de agosto de 2026  
**Estado:** Especificación técnica pública — v0.3

---

## 1. Estatuto y relación con v0.2

La versión 0.3 conserva la IR v0.2 salvo en las definiciones y juicios que este documento sustituye de forma expresa.

```text
IR v0.3
= IR v0.2
+ correcciones declaradas aquí
```

La IR v0.2 se conserva como antecedente histórico. La gramática superficial correspondiente a esta versión es la v0.2.

---

## 2. Frontera entre captura, admisibilidad y `Tri`

### 2.1. Estados de admisibilidad

`AdmissibilitySpec` usa exactamente:

```text
AdmissibilityState = {Ok, Degraded, NotAdmitted}
```

`Failed` y `U` dejan de pertenecer a ese conjunto.

`Bottom` permanece como símbolo técnico de fallo de `CaptureSpec` y queda separado tanto de `AdmissibilityState` como de `Tri`.

### 2.2. Regla constitutiva

No existe coerción automática:

```text
Bottom       ↛ 0 | 1 | U
NotAdmitted  ↛ 0 | 1 | U
fallo técnico ↛ Tri
```

`Ok` y `Degraded` representan observaciones positivamente admitidas bajo la regla declarada. Una observación admitida puede alcanzar `Tri.U` sólo por la vía semántica de un `Ternarizer` cuando la observación pertenece a la partición `B_U` aplicable.

La ausencia de nueva ternarización no rellena una posición con `U`, no reescribe un estado anterior y no constituye un vector ternario incompleto.

### 2.3. Diagnóstico

La declaración inválida de `AdmissibilitySpec` se rechaza mediante:

```text
E110 — InvalidAdmissibilitySpec
```

---

## 3. Resolución identificada de una `U`

### 3.1. `ResolutionTarget`

Se define:

```text
ResolutionTarget = (EvaluableStateRef, position)
```

con las condiciones:

- `EvaluableStateRef` referencia un `CellState` o `CoupledState`;
- `position` es uno-basada;
- la posición existe en el vector efectivo;
- el valor efectivo de esa posición es `U`.

Un literal abstracto `U` no es un objetivo suficiente de `resolve`.

### 3.2. Instancia de revisión

La operación canónica de superficie baja a una entrada equivalente a:

```json
{
  "target": {"state": "S1", "position": 3},
  "with_spec": "RS1",
  "context_instance": "ContextoClinico",
  "mechanism_instance": "RevisionExperto"
}
```

Por defecto, la compatibilidad con `ResSpec` exige igualdad exacta:

```text
context_instance   = ResSpec.context
mechanism_instance = ResSpec.mechanism
```

La ausencia de una relación ampliada expresamente constituida no equivale a compatibilidad universal.

### 3.3. Revisión y clausura son estatutos distintos

La revisión computacional aporta material de revisión. No constituye por sí misma autoridad suficiente para cerrar una `U` genuina en `0` o `1`.

El circuito autorable del lenguaje no dispone de una construcción que fabrique una referencia de clausura positiva. Por ello, la existencia de `resolve` no debe interpretarse como una autorización para reescribir el objetivo revisado.

Debe permanecer representable:

```text
U → revisión → U
```

### 3.4. Esquema conceptual de `ResolutionRecord`

La versión 0.3 reconoce, para proyección superficial, los campos:

```text
target
previous
reviewed_to
resolved_to
context_ref
mechanism_ref
```

`previous` conserva el estado ternario previo pertinente; `target` conserva la identidad de la ocurrencia revisada. `reviewed_to` expresa el resultado de la revisión cuando proceda; `resolved_to` expresa el estatuto de clausura reconocido, sin que ambos se identifiquen por defecto.

### 3.5. Diagnóstico

Se utiliza:

```text
E305 — UnsafeUResolution
```

para objetivo no evaluable, posición fuera de rango, posición distinta de `U` o instancia incompatible con su `ResSpec`.

---

## 4. Cierre estructural y causal de `Frame`

### 4.1. Arquitectura

Todo `Frame` debe referir un `CompositionGraph` existente mediante `architecture`.

Sea:

```text
N_F = nodes(Frame.architecture)
```

Cada `CoupledState` incluido en `Frame.cell_states` debe corresponder a un nodo de `N_F`.

### 4.2. Identidad de estado por nodo

Dentro de un `Frame`:

```text
como máximo un CoupledState por nodo de arquitectura
```

La identidad se determina por el `CoupledSpec` que constituye el nodo, no por el `CellSpec` subyacente. Por tanto, dos nodos distintos pueden compartir el mismo `CellSpec` sin colisionar.

No se permiten referencias duplicadas al mismo `CoupledState`.

### 4.3. Evaluaciones

Para cada `EvalResult` declarado en `Frame.eval_results`:

- su `source_state` debe pertenecer a `Frame.cell_states`;
- no puede existir una segunda evaluación material de la misma fuente dentro del mismo `Frame`.

Esta regla expresa identidad y cierre, no exhaustividad.

### 4.4. Compuertas

Para cada `GateResult` declarado en `Frame.gate_results`, todas sus evaluaciones de entrada deben pertenecer a `Frame.eval_results`.

No se admite una compuerta cuyo resultado dependa parcialmente de evaluaciones externas al `Frame`.

### 4.5. Supervisión

Para cada `SupervisionResult` declarado en `Frame.supervision`:

- `meta_eval` pertenece a `Frame.eval_results`;
- `CellTarget(x)` exige `x ∈ Frame.eval_results`;
- `ComposedTarget(x)` exige `x ∈ Frame.gate_results`;
- `SystemTarget(x)` exige `x = Frame.architecture`.

### 4.6. Criticidades

La superficie vigente no posee un productor constituido de `CriticalityResult`.

Por ello, mientras esa condición se mantenga:

```text
Frame.criticalities = []
```

No se infiere que la criticidad no exista en la doctrina; se afirma únicamente que la superficie actual no puede declarar honestamente resultados que no sabe producir.

### 4.7. No exhaustividad

La bienformación de `Frame` exige coherencia de lo declarado. No exige:

- un estado para todos los nodos posibles fuera del alcance declarado;
- una evaluación para cada estado;
- una compuerta para cada combinación;
- una supervisión para cada resultado;
- un cálculo de criticidad inexistente en la superficie.

### 4.8. Diagnóstico

Las violaciones anteriores se diagnostican como:

```text
E308 — FrameClosureViolation
```

---

## 5. Versionado de la IR emitida

La cabecera canónica de `IRProgram` es:

```json
{
  "ir_version": "0.3",
  "grammar_version": "0.2",
  "serializer_version": "0.1.0"
}
```

La versión del serializador no cambia porque las reglas de ordenación y canonicalización JSON permanecen estables.

---

## 6. Juicios nuevos o reforzados

Esta versión añade o refuerza, en el radio implementado, los siguientes juicios:

```text
J-A0  AdmissibilitySpec usa exactamente Ok/Degraded/NotAdmitted.
J-A1  Fallo técnico o NotAdmitted no fabrican Tri.
J-R0  resolve identifica un estado evaluable y una posición real.
J-R1  la posición objetivo contiene U.
J-R2  la instancia de revisión es compatible con su ResSpec.
J-R3  revisión y clausura positiva no se identifican automáticamente.
J-F0  Frame sólo contiene estados de su arquitectura.
J-F1  existe como máximo un estado por nodo de arquitectura.
J-F2  EvalResult queda ligado a un estado del mismo Frame y sin duplicación de fuente.
J-F3  GateResult sólo depende de evaluaciones del mismo Frame.
J-F4  SupervisionResult conserva meta-evaluación y objetivo dentro del mismo cierre.
J-F5  CriticalityResult no producible no puede declararse.
```

Estos nombres sirven para lectura técnica de v0.3 y no renumeran retrospectivamente los juicios de v0.2.

---

## 7. Elementos no modificados

La versión 0.3 no introduce ni resuelve:

- un cuarto valor de `Tri`;
- un cuarto impacto semántico independiente;
- authoring superficial completo de `ConflictOperator`;
- la deuda de régimen `General` de J2.3;
- primitivas de tiempo o reloj;
- `deployment_profile` como objeto de IR;
- TCB, raíces, atestación o continuidad autoritativa como tipos del núcleo;
- ejecución soberana de las operaciones;
- productor superficial de `CriticalityResult`.

La divergencia histórica del identificador `E204` permanece documentada en el catálogo efectivo v0.3.

---

## 8. Evidencia de conformidad

La implementación de referencia correspondiente a esta versión dispone de una batería de 72 casos:

```text
11 válidos
61 inválidos
72 total
```

Los casos válidos comparan la salida contra IR canónica comprometida. Los inválidos exigen el código diagnóstico declarado. La batería incluye contraejemplos específicos para:

- estados de admisibilidad heredados;
- objetivo de `resolve` fuera de rango o distinto de `U`;
- instancia de revisión incompatible;
- estados de `Frame` ajenos a la arquitectura;
- evaluaciones externas o duplicadas;
- compuertas y supervisiones externas;
- criticidad no producible;
- dos nodos distintos que comparten legítimamente un mismo `CellSpec`.

---

## 9. Dictamen técnico

IR v0.3 preserva la terna canónica y refuerza tres fronteras: la admisibilidad técnica no se convierte en semántica, `resolve` sólo puede revisar una ocurrencia constituida e identificable de `U`, y `Frame` sólo puede declarar resultados pertenecientes a su propio cierre estructural y causal. Las obligaciones externas a ese radio permanecen externas o como deuda explícita y no se incorporan a la IR por inercia.
