# Adenda técnica de vigencia de la Frontera Normativa del Lenguaje SV — C01–C03

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 24 de agosto de 2026  
**Estado:** Especificación técnica pública — adenda de vigencia

---

## 1. Objeto y estatuto

La presente adenda reconcilia la `FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md`, publicada en marzo de 2026, con las revisiones versionadas posteriormente incorporadas al Lenguaje SV mediante:

- Gramática superficial mínima v0.2;
- IR canónica y sistema de bienformación v0.3;
- correcciones C01, C02 y C03;
- realización Rust integrada dentro del alcance ya comprobado.

La Frontera Normativa v0 se conserva íntegramente como antecedente histórico y normativo de su fecha. Esta adenda no la reescribe retrospectivamente. Dentro de los puntos expresamente enumerados aquí, las reglas posteriores sustituyen las formulaciones incompatibles de v0.

Fuera de ese radio, la presente adenda no altera por sí sola las cláusulas de la Frontera Normativa v0 ni introduce capacidades nuevas.

Regla de lectura:

```text
Frontera Normativa v0
+ sustituciones expresas de esta adenda
+ Gramática 0.2 / IR 0.3 en su radio versionado
= frontera pública vigente para C01–C03
```

No existe una corrección C04.

---

## 2. C01 — Captura, admisibilidad y `Tri.U`

### 2.1. Cláusula sustituida

El apartado B.6 de la Frontera Normativa v0 utilizaba el conjunto:

```text
R = {ok, degradado, fallido, U}
```

y asociaba el fallo de captura o la insuficiencia de admisibilidad con un resultado `U`.

Esa formulación queda sustituida, dentro del Lenguaje vigente, por la separación explícita entre captura, admisibilidad técnica y valor ternario.

### 2.2. Estados vigentes

`CaptureSpec` conserva un símbolo técnico de fallo:

```text
Bottom
```

`AdmissibilitySpec` admite exactamente:

```text
AdmissibilityState = {Ok, Degraded, NotAdmitted}
```

El alfabeto semántico permanece:

```text
Tri = {Zero, One, U}
```

No existen coerciones automáticas entre esas capas:

```text
Bottom        ↛ Tri
NotAdmitted   ↛ Tri
fallo técnico ↛ Tri
```

Por tanto, un fallo de captura, una entrada no admitida o una excepción técnica no constituyen por sí mismos `Tri.U`.

### 2.3. Única vía ordinaria hacia `Tri.U` desde una observación admitida

Una observación positivamente admitida puede producir legítimamente `Tri.U` únicamente mediante un `Ternarizer` declarado cuando la observación pertenece a su partición `B_U` aplicable.

La cadena vigente se interpreta, en su radio implementado, como separación de fases:

```text
captura
→ resultado técnico de captura
→ admisibilidad
→ observación admitida
→ Ternarizer(B0 | B1 | B_U)
→ Tri
```

La ausencia de una observación admitida no fabrica una posición ternaria ni reescribe un estado previo.

### 2.4. Diagnóstico

La declaración inválida de los estados de admisibilidad se rechaza mediante:

```text
E110 — InvalidAdmissibilitySpec
```

---

## 3. C02 — Resolución identificada de una `U`

### 3.1. Sustitución de la forma abstracta de v0

La formulación abstracta de la Frontera Normativa v0:

```text
Res : {U} × Context × Mechanism → {0, 1, U}
```

permanece únicamente como antecedente conceptual. La superficie y la IR vigentes exigen identificar la ocurrencia concreta de `U` que se revisa.

Se define:

```text
ResolutionTarget = (EvaluableStateRef, position)
```

con posición uno-basada.

La forma superficial vigente es:

```svp
let RR = resolve((estado, posicion),
                 with: ResSpecRef,
                 context: ContextRef,
                 mechanism: MechanismRef);
```

### 3.2. Condiciones de bienformación

La operación exige:

1. que el estado referenciado sea evaluable;
2. que la posición exista;
3. que el valor efectivo de esa posición sea `U`;
4. que `with` refiera un `ResSpec`;
5. que, por defecto, la instancia `(context, mechanism)` sea compatible por igualdad exacta con `(ResSpec.context, ResSpec.mechanism)`.

La ausencia de una relación ampliada expresamente constituida no implica compatibilidad universal.

### 3.3. Revisión y clausura

La revisión de una `U` constituida no confiere por sí sola autoridad suficiente para clausurarla en `0` o `1`.

Debe permanecer representable:

```text
U → revisión → U
```

La existencia de `resolve` no autoriza a fabricar una clausura positiva ni a identificar automáticamente `reviewed_to` con `resolved_to`.

### 3.4. Diagnóstico

Las violaciones del objetivo o de la compatibilidad de la instancia de revisión se diagnostican mediante:

```text
E305 — UnsafeUResolution
```

---

## 4. C03 — Coherencia estructural y causal de `Frame`

### 4.1. Alcance de la actualización

La definición histórica de `Frame` de la Frontera Normativa v0 se conserva como antecedente. La IR v0.3 añade reglas de coherencia relacional para los objetos efectivamente declarados en cada `Frame`.

Estas reglas no imponen exhaustividad.

### 4.2. Reglas vigentes

Todo `Frame` bien formado debe cumplir, dentro del radio actualmente representable:

- `architecture` referencia un `CompositionGraph` existente;
- cada `CoupledState` incluido corresponde a un nodo de esa arquitectura;
- existe como máximo un `CoupledState` por nodo de arquitectura;
- nodos distintos pueden compartir un mismo `CellSpec` mediante `CoupledSpec` distintos;
- cada `EvalResult` incluido refiere un estado del mismo `Frame`;
- no se duplican dos evaluaciones materiales de la misma fuente dentro del mismo `Frame`;
- cada `GateResult` incluido depende únicamente de evaluaciones incluidas en el mismo `Frame`;
- cada `SupervisionResult` conserva su meta-evaluación y su objetivo dentro del mismo cierre;
- un `SystemTarget` de supervisión debe coincidir con `Frame.architecture`;
- mientras no exista un productor superficial constituido de `CriticalityResult`, se exige:

```text
Frame.criticalities = []
```

### 4.3. No exhaustividad

La bienformación exige coherencia de lo declarado. No exige producir estados, evaluaciones, compuertas, supervisiones o criticidades que la superficie vigente no haya constituido.

### 4.4. Diagnóstico

Las violaciones de estas reglas se diagnostican mediante:

```text
E308 — FrameClosureViolation
```

---

## 5. Operadores y deuda explícita

La presencia histórica de un operador en la Frontera Normativa v0 no implica que la superficie, la IR o la realización vigente dispongan de una implementación completa de ese operador.

En particular, permanece como deuda explícita:

```text
ConflictOperator / Ψ / J2.3
```

No se infiere una realización de `ConflictOperator` a partir de esta adenda ni de las correcciones C01–C03.

Del mismo modo, la ausencia actual de un productor superficial constituido de `CriticalityResult` no se corrige por declaración documental.

---

## 6. Versionado observable

Las versiones vigentes permanecen:

```text
grammar_version    = 0.2
ir_version         = 0.3
serializer_version = 0.1.0
```

La presente reconciliación no modifica esos números de versión porque no introduce una nueva gramática ni una nueva IR: fija la precedencia documental de revisiones ya incorporadas.

La realización Rust utiliza una proyección diferencial 0.1.0 para las comprobaciones de correspondencia. Esa proyección no se identifica por esta adenda con el serializador canónico completo de la referencia Python.

---

## 7. Tabla de precedencia material

| Materia | Formulación v0 | Regla vigente dentro del alcance posterior | Fuente posterior |
|---|---|---|---|
| Admisibilidad | `{ok, degradado, fallido, U}` | `{Ok, Degraded, NotAdmitted}`; fallo técnico separado de `Tri` | C01 · Gramática 0.2 · IR 0.3 |
| Fallo de captura | Podía desembocar en `U` por la propia cadena | `Bottom ↛ Tri`; fallo técnico `↛ Tri` | C01 · IR 0.3 |
| Ternarización | Cadena B.6 histórica | Sólo una observación admitida puede alcanzar `Tri` mediante el `Ternarizer` declarado | C01 · IR 0.3 |
| Resolución | `Res({U}, Context, Mechanism)` abstracta | `ResolutionTarget=(estado, posición)` sobre una `U` constituida, con `ResSpec`, contexto y mecanismo | C02 · Gramática 0.2 · IR 0.3 |
| Clausura tras revisión | No distinguida con suficiente precisión | revisión y clausura positiva son estatutos distintos | C02 · IR 0.3 |
| `Frame` | Definición previa a C03 | coherencia estructural y causal de lo declarado, sin exhaustividad | C03 · IR 0.3 |
| `ConflictOperator` | Reconocido en la frontera v0 | deuda explícita; no se presume implementado | deuda vigente |

---

## 8. Alcance y no efectos

Esta adenda:

- corrige una contradicción documental de vigencia entre la Frontera Normativa v0 y las revisiones C01–C03 ya incorporadas;
- no reescribe retrospectivamente el documento v0;
- no altera `Tri`;
- no crea un cuarto valor;
- no introduce C04;
- no amplía la sintaxis ni la IR;
- no modifica el código Rust ni la implementación Python;
- no acredita paridad diagnóstica exacta entre realizaciones;
- no completa `ConflictOperator`;
- no acredita las Garantías I o II;
- no declara cerrado R0 ni las fases posteriores de realización.

---

## 9. Regla de vigencia

Para C01, C02 y C03, cualquier lectura de la Frontera Normativa v0 que entre en conflicto con la presente adenda, la Gramática 0.2 o la IR 0.3 debe considerarse sustituida dentro del alcance expresamente versionado.

La Frontera Normativa v0 permanece disponible para reconstruir la evolución del Lenguaje y continúa siendo aplicable en las materias no sustituidas por documentos posteriores de igual o mayor especificidad.