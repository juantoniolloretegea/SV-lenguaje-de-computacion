# IR canónica y sistema de bienformación del Lenguaje SV — v0.3

## Sucesora normativa de v0.2 para admisibilidad, resolución identificada y cierre relacional de `Frame`

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 23 de agosto de 2026  
**Estado:** Especificación técnica pública — v0.3

**Precisiones posteriores:** N0-01 (§6.1), N0-02 (§6.2), N0-03 (§6.3) N0-04 (§6.4) y BridgeSet (§6.5), con alcance y diagnóstico expresos

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
J-K0  Codomain contiene al menos un miembro y no contiene miembros repetidos.
J-K1  Cada CellSpec enlaza una interpretación única para cada miembro de su Codomain, sin claves ajenas.
J-J0  La proyección de un programa admitido no contiene miembros homónimos dentro de ningún objeto JSON.
J-B0  CoupledSpec.bridges no repite posiciones como valores Nat.
J-H0  Horizon.architecture resuelve un CompositionGraph declarado y bien formado.
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

### 6.1. `Codomain` como conjunto representado

Para toda declaración `K : Codomain` con representación secuencial `K.values`:

```text
K.values ≠ []
card(K.values) = card(set(K.values))
```

La secuencia conserva el orden declarado para la representación y la serialización, pero no convierte por sí sola al codominio en un orden total semántico. Un miembro repetido produce rechazo `E004 — InvalidCodomain`; el frontend no elimina duplicados, no ordena los miembros y no repara el programa en silencio.

El identificador `E101 — EmptyCodomain` de la tabla histórica de IR v0.2 no se reutiliza: en el catálogo efectivo vigente `E101` identifica `VectorLengthMismatch`. N0-01 fija `E004` como identidad observable única para la invalidez estructural de `Codomain`, sin reescribir el antecedente histórico ni cerrar la deuda distinta `E111 — UnorderedCodomain`.

---

<a id="relacion-n0-02"></a>

### 6.2. Relación total y unívoca `CellSpec–OutputSemantics–Codomain`

Para cada `C : CellSpec`, sean `K = C.codomain` y `S = C.semantics` las declaraciones resueltas por identidad y tipo. Se exige:

```text
keys = [key | (key, description) ∈ S.mappings]
card(keys) = card(set(keys))
set(keys) = set(K.values)
```

Esta regla concreta J1.1 de IR v0.2: cada símbolo del codominio tiene exactamente una interpretación declarada. Rechaza una semántica vacía sobre codominio no vacío, claves ausentes, ajenas o repetidas, incluso si la repetición conserva el mismo texto. No exige textos descriptivos distintos, no juzga su contenido y no exige el mismo orden de declaración de claves y miembros. La validación no completa, deduplica ni reordena las declaraciones; conserva las reglas de representación y serialización existentes.

La comprobación se realiza sobre cada `CellSpec`, después de resolver sus referencias, antes de producir la proyección observable. Las referencias adelantadas y las distribuidas entre unidades ES/EN se resuelven sobre el programa completo o ensamblado. Si varias celdas comparten una semántica, cada relación debe satisfacer la regla por separado; otra declaración semántica no suple sus claves ausentes.

La invalidez de esta relación produce `E115 — InvalidOutputSemantics`, fase `validate`, capa efectiva 1 (Estado), con identidad de la celda, semántica y codominio implicados. `E102 — MissingOutputSemantics` conserva su alcance efectivo para referencia semántica ausente o de tipo incorrecto. El rechazo no produce IR ni un valor `U`.

N0-02 cierra esta relación constituida, sin añadir a `OutputSemantics` una referencia propia de codominio. La validación global de nombres homónimos de JSON, incluidas declaraciones sin vínculo con una `CellSpec`, se resuelve separadamente en N0-03 (§6.3). La Gramática v0.2, el esquema de IR v0.3 y el serializador v0.1.0 se conservan: se refuerza la admisión de entradas, no se modifica la forma de salida de los programas conformes. Los programas que violan J-K1 dejan de admitirse.

---

<a id="proyeccion-n0-03"></a>

### 6.3. Unicidad local de miembros y estabilidad de proyección (N0-03)

Toda declaración `S : OutputSemantics` debe tener claves distintas, también cuando ninguna `CellSpec` la referencia:

```text
keys = [key | (key, description) ∈ S.mappings]
card(keys) = card(set(keys))
```

Una clave repetida produce `E115 — InvalidOutputSemantics`, incluso si los textos coinciden. Esta precisión extiende la sede de comprobación de la misma infracción de multiplicidad ya diagnosticada por N0-02; no reutiliza el código para un significado distinto. Sin una celda vinculante, el diagnóstico identifica la semántica y sus claves repetidas, sin fabricar identidades de celda o codominio. La comprobación complementaria se aplica después de las validaciones existentes, antes de exponer el programa admitido o descender a mapas; conserva los rechazos relacionales y su precedencia de N0-02.

La unicidad es local a cada mapa. Distintas declaraciones pueden usar la misma clave; sus identidades y referencias siguen siendo independientes. No se deduplican claves, no se fusionan mapas ni se exige cobertura de un codominio no referenciado. Una semántica no enlazada y sin repeticiones conserva su admisibilidad estructural previa, incluida la forma vacía; eso no acredita su suficiencia para una operación o una celda futura.

En el esquema emitido vigente, las claves variables se encuentran en `OutputSemantics.mappings` y `Connector.mapping`. El segundo conserva su comprobación de unicidad y cobertura anterior. Las filas de tablas, las secuencias y los campos textuales permanecen respectivamente filas, secuencias y texto; no se reinterpretan como mapas. Los nombres de los demás campos JSON son fijos y distintos dentro de cada objeto del esquema.

Para la proyección JSON `P` de todo programa admitido del corpus comprobado, el recorrido `P → parse_JSON → serialize_JSON → parse_JSON` debe conservar miembros, nombres, valores, tipos, orden observable y representación numérica sin conversión a coma flotante. El lector de prueba rechaza homónimos a cualquier profundidad antes de formar mapas. La identidad de nombres se compara tras decodificar escapes JSON, sin normalización adicional. Se permiten diferencias de espacios y escapes equivalentes según el contrato del observador; esta propiedad no demuestra igualdad literal entre emisores.

J-J0 se exige a todos los programas admitidos por las rutas constituidas. La revisión del esquema y la guarda de sus dos mapas variables fundamentan esa obligación; las pruebas acreditan el corpus y las variantes declaradas. No se afirma una verificación formal mecanizada universal. No se introduce un importador de IR ni una equivalencia completa entre AST, IR y doctrina: `equivalence_json` conserva su condición de proyección diferencial. Gramática 0.2, esquema IR 0.3 y serializador 0.1.0 mantienen sus versiones; la entrada que viola la unicidad deja de admitirse. La deuda CRLF de DFL-008 sigue separada.

---

<a id="arquitectura-n0-04"></a>

### 6.4. Resolución de la arquitectura de `Horizon` (N0-04)

IR v0.2, nivel 3, declara literalmente `Horizon.architecture : ArchitectureId`; el nivel 4 declara `Agent.architecture : CompositionGraph`. Se precisa aquí la resolución de aquella identidad en la superficie y la IR vigentes:

```text
Para todo H : Horizon admitido:
resolve(H.architecture) : CompositionGraph
Para todo A : Agent admitido, con D = resolve(A.domain) y H = resolve(D.horizon):
A.architecture = H.architecture
```

Aquí `resolve` designa resolución de referencias en el juicio de bienformación, no la operación de revisión de U de la DSL. La referencia se resuelve por identidad exacta en el espacio global del programa completo o ensamblado. Debe existir un objeto declarado del tipo `CompositionGraph`, que a su vez supere sus comprobaciones de bienformación. No bastan una cadena no vacía, dos nombres iguales sin referente, un objeto de otro tipo ni un resultado de operación. Dos grafos de idéntica estructura y nombres distintos no son intercambiables por inferencia.

La obligación alcanza horizontes sin consumidores y referencias adelantadas o entre unidades ES/EN. Las validaciones globales preceden a la admisión del programa; no se exige que el grafo aparezca antes del horizonte ni se introduce resolución parcial por unidad. Se conserva la validación previa de la relación `Agent–Domain–Horizon`: un programa sólo se admite si también supera J-H0, por lo que la igualdad del agente queda ligada al mismo grafo real.

La comprobación complementaria de J-H0 se realiza después de las validaciones anteriores, incluida N0-03, para conservar la precedencia de los rechazos existentes. Si un programa viola varias obligaciones, el primer rechazo no certifica el cumplimiento de las restantes. La referencia ausente o de tipo incorrecto utiliza `E006 — UndeclaredReference` en Python y el rechazo textual de referencia tipada ya existente en Rust. La incompatibilidad de arquitectura del agente conserva `E402` en Python y su rechazo textual anterior en Rust. Se mantienen sus diferencias documentadas bajo DFL-001; no se atribuye un código estructurado común a Rust.

La corrección no crea una arquitectura, no completa el dominio ni interpreta los campos opacos de `Domain` o `Agent`. No decide multiplicidad de `Horizon.events`, coherencia causal adicional entre horizontes y frames ni K1-T. Gramática 0.2, esquema IR 0.3 y serializador 0.1.0 mantienen sus versiones: la identidad ya estaba representada; se comprueba su referencia. No se modifican emisores ni se admite una IR o U como sustituto del rechazo.

---

<a id="bridgeset-j-b0"></a>
### 6.5. Unicidad de posiciones de `BridgeSet` (J-B0)

IR v0.2, definición de `CoupledSpec` y J1.2, declara `bridges : BridgeSet`, subconjunto de `{1,…,n}`. La lista superficial representa ese conjunto: dos ocurrencias de la misma posición no constituyen dos puentes distintos. Para admitir una especificación acoplable:

```text
J-B0: para todo i != j, bridges[i] != bridges[j] como valores Nat.
```

Se conserva la comprobación de rango `1 ≤ posición ≤ n`. El conjunto vacío sigue admitido con el límite de J1.2: no participa en transmisión en serie. Las posiciones válidas conservan su orden declarado en la IR y su proyección. No se ordena ni deduplica la entrada, ni se repara una infracción. `03` y `3` denotan el mismo Nat según la representación numérica vigente; su coexistencia se rechaza.

El juicio se aplica al programa completo, incluidas unidades SVP-ES/SVP-EN ensambladas con referencias adelantadas. Se conserva la precedencia de la referencia tipada a `CellSpec` y de los rechazos de rango; la unicidad se comprueba después de recorrer el rango completo de esa lista. Un primer rechazo no acredita las obligaciones restantes.

La realización emite el rechazo textual controlado `CoupledSpec <id>: posición puente repetida: <Nat>`. No se asigna por analogía un código nuevo: el catálogo efectivo y su concordancia permanecen bajo DFL-001. El caso del corpus se identifica por la obligación `J1.2/BridgeSet`, separada del observable. Gramática 0.2, esquema IR 0.3 y proyección 0.1.0 conservan sus versiones; el cambio impone una propiedad ya constituida, sin añadir un campo ni una operación.

RETP-083 y la [radiografía §15](docs/arquitectura/N0_RADIOGRAFIA_DE_OBJETOS_INVARIANTES_Y_ORACULOS_DEL_NUCLEO_SV_2026_09_04.md#cierre-bridgeset-20260907) delimitan evidencia y relevo. La retirada Python de RETP-082 conserva en Git sus antecedentes; las menciones a ambas realizaciones en los cierres anteriores describen esos cortes históricos.

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

La conformidad vigente de SV dispone de una batería de 92 casos:

```text
14 válidos
78 inválidos
92 total
```

Los casos válidos comparan directamente la proyección nativa con los esperados comprometidos mediante el observador de pares JSON ordenados. Los inválidos exigen rechazo controlado y el texto esperado para su obligación; no se afirma que el destino emita todos los códigos del catálogo (DFL-001, RETP-082). La batería incluye contraejemplos específicos para:

- estados de admisibilidad heredados;
- `Codomain` con un miembro repetido;
- `CoupledSpec.bridges` con una posición repetida;
- semántica de `CellSpec` vacía, incompleta, con clave ajena o repetida;
- semántica repetida sin celda vinculante y control de la guarda previa de claves de `Connector`;
- horizonte con arquitectura inexistente o de tipo incorrecto y agente con dos arquitecturas reales distintas;
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
