# Gramática superficial mínima del lenguaje de computación del Sistema Vectorial SV — v0.1

## Especificación de la superficie declarativa del DSL canónico

**Autor:** Juan Antonio Lloret Egea
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)
**ISSN:** 2695-6411
**Licencia:** CC BY-NC-ND 4.0
**Fecha:** marzo 2026
**Estado:** Especificación técnica del lenguaje — v0.1
**Repositorio:** [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion)

© Juan Antonio Lloret Egea, marzo 2026. Todos los derechos reservados conforme a CC BY-NC-ND 4.0.

---

## Resumen

Este documento fija la gramática superficial mínima del lenguaje de computación del Sistema Vectorial SV en su versión 0.1. La gramática es la capa que traduce la intención del programador humano a la representación intermedia canónica (IR v0.2) ya publicada.

La gramática es declarativa, nominal y sin azúcar innecesario. Toda construcción escrita en esta superficie baja unívocamente a un objeto o una operación de la IR v0.2, sin pérdida semántica y sin ambigüedad. La gramática no inventa semántica nueva: es una superficie humana austera de la IR, no una teoría paralela.

El principio rector es que escribir un programa SV correcto sea natural y que escribir uno incorrecto sea imposible o doloroso.

---

## 1. Subordinación doctrinal

Este documento es doctrina derivada del corpus publicado del Sistema Vectorial SV. La autoridad normativa suprema reside en los *Fundamentos algebraico-semánticos del Sistema Vectorial SV* (Release 3).

La gramática se subordina a dos documentos ya publicados en el repositorio del lenguaje:

| Documento | Función |
|-----------|---------|
| Frontera normativa del lenguaje SV — v0 | Establece qué debe conocer, ejecutar, verificar y prohibir el lenguaje |
| IR canónica y sistema de bienformación — v0.2 | Fija la representación intermedia normalizada, los juicios de bienformación y los errores constitutivos |

La gramática no puede contradecir ninguno de estos documentos. En caso de discrepancia, prevalece la IR; si la discrepancia afecta a un invariante constitutivo, prevalecen los *Fundamentos*.

### Cadena de compilación

```
Gramática superficial (este documento) → IR canónica v0.2 → Backend Rust + Backend Python (referencia) → WASM (futuro)
```

---

## 2. Decisiones de diseño

### 2.1. Gramática declarativa y nominal

Toda declaración tiene nombre explícito, tipo explícito y campos obligatorios. No hay inferencia amplia de tipos, no hay infijo, no hay sobrecarga, no hay macros. Lo que el usuario escribe, lo declara; lo que no declara, no existe en su programa.

### 2.2. Resultados obtenidos por operadores, no por literales

La gramática v0.1 no permite construir a mano objetos de resultado (`EvalResult`, `GateResult`, `ResolutionRecord`, `QueryResult`, `SupervisionResult`). Estos se obtienen exclusivamente por invocación de operadores (`evaluate`, `gate`, `resolve`, `query`, `supervise`). Esta decisión impide falsificaciones sintácticas de objetos de nivel N2/N3 de la IR.

### 2.3. Exclusión de `max` y `min` en la superficie v0.1

El lowering de `max/min` como azúcar sobre `gate` con tabla trivial ya está cerrado en la IR v0.2 y reconocido en la Frontera normativa (Bloque A, punto A.9). Sin embargo, la superficie v0.1 no los expone. La razón es doble: no añaden capacidad expresiva (todo lo que `max` puede hacer, `gate` con la tabla adecuada lo hace) y su presencia en v0 induciría falsa apariencia de primitividad. `max/min` entrarán como azúcar derivado en una versión posterior, con lowering visible al usuario.

### 2.4. `Comp` como constructor austero

`Comp` es constructor organizativo de Grado C. Su axiomática fuerte sigue abierta. La gramática permite invocarlo, pero no cierra más semántica de la que el corpus ha cerrado. `compose(...)` recibe un grafo, relaciones semánticas previas y patrones compositivos declarados explícitamente.

### 2.5. Zona prohibida sintácticamente inalcanzable

La gramática no ofrece construcciones para escribir lo que la Frontera normativa (Bloque D) prohíbe. La zona prohibida no es una advertencia: es una imposibilidad sintáctica.

### 2.6. Invariantes constitutivos garantizados por el tipo, no por el usuario

Los invariantes constitutivos 8 (inmutabilidad del polígono/frame) y 9 (trayectoria append-only) son propiedades del tipo que el compilador/verificador garantiza. No aparecen como campos en la gramática. El usuario no escribe `immutable: True` ni `append_only: True` porque no existe la alternativa.

### 2.7. Derivación automática de `n` desde `b`

La célula se declara con `b` y el compilador deriva `n = b²`. La superficie no permite escribir `n` directamente, lo que hace imposible la incoherencia `n ≠ b²`. La precondición `b ≥ 3` se verifica en lowering (error E si se viola).

---

## 3. Alcance

### 3.1. Lo que la gramática v0.1 cubre

| Nivel IR | Objetos cubiertos |
|----------|-------------------|
| N0 — Definición | `Codomain`, `OutputSemantics`, `CellSpec`, `CoupledSpec`, `Connector`, `AdmissibilityTable`, `CaptureSpec`, `AdmissibilitySpec`, `Ternarizer`, `ResSpec` |
| N1 — Estado | `CellState`, `CoupledState`, `CompositionGraph` (con `Edge` enriquecido) |
| N2 — Resultado | `EvalResult`, `GateResult`, `ResolutionRecord`, `SupervisionResult`, `QueryResult` (todos obtenidos por operador, nunca por literal) |
| N3 — Evolución | `Horizon`, `Frame`, `TransitionData`, `TrajectoryEntry`, `Trajectory` |
| N4 — Uso | `Domain`, `Agent`, `QuerySpec` |
| Composición | `SemanticRelation`, `Pattern` (declaraciones mínimas nominales) |
| Operadores | `evaluate`, `gate`, `resolve`, `query`, `supervise`, `compose` |

### 3.2. Lo que la gramática v0.1 no cubre

No se incluyen en esta versión: azúcar para `max/min`, notación infija para compuertas, generación implícita de `Frame`, atajos para `CoverageReport` o `ArchitectureView`, authoring superficial completo de `ConflictOperator`, análisis discreto como sintaxis de primera clase, ni ninguna construcción del Bloque C de la Frontera normativa.

---

## 4. Nomenclatura

La gramática adopta íntegramente la tabla de nomenclatura desambiguada de la Frontera normativa v0 (§3). Las palabras clave de la gramática coinciden con los nombres de esa tabla. No se permite ningún alias que reintroduzca colisiones.

| Palabra clave | Objeto IR | Símbolo del corpus |
|---------------|-----------|-------------------|
| `Tri` | Tipo ternario | Σ = {0, 1, U} |
| `Zero`, `One`, `U` | Literales ternarios | 0, 1, U |
| `codomain` | Codomain | Ωᵢ / Kᵢ |
| `output_semantics` | OutputSemantics | κᵢ |
| `cellspec` | CellSpec | Cᵢ (componentes de definición) |
| `coupledspec` | CoupledSpec | Ĉᵢ = (Cᵢ, Bᵢ) |
| `connector` | Connector | φⱼ→ᵢ⁽ᵏ⁾ |
| `admissibility_table` | AdmissibilityTable | 𝒯 |
| `ternarizer` | Ternarizer | τⱼ |
| `graph` | CompositionGraph | 𝒢 = (V, E, Φ) |
| `evaluate` | χᵢ (factorizado) | evaluate + classify |
| `gate` | ⊗_gate | Compuerta jerárquica |
| `resolve` | Res | Función de resolución |
| `supervise` | ▷ | Supervisión meta |
| `compose` | Comp | Constructor organizativo |
| `query` | 𝒬_ω | Operador de consulta |

---

## 5. Gramática formal

### 5.0. Estructura general y primitivas

```ebnf
program                ::= { declaration | command } ;

declaration            ::= codomain_decl
                         | output_semantics_decl
                         | cellspec_decl
                         | coupledspec_decl
                         | connector_decl
                         | table_decl
                         | capture_decl
                         | admissibility_decl
                         | ternarizer_decl
                         | resspec_decl
                         | cellstate_decl
                         | coupledstate_decl
                         | graph_decl
                         | semantic_relation_decl
                         | pattern_decl
                         | horizon_decl
                         | frame_decl
                         | transitiondata_decl
                         | trajectory_decl
                         | domain_decl
                         | agent_decl
                         | queryspec_decl ;

command                ::= eval_cmd
                         | gate_cmd
                         | resolve_cmd
                         | query_cmd
                         | supervise_cmd
                         | compose_cmd
                         | projection_cmd ;

tri_literal            ::= "Zero" | "One" | "U" ;
identifier             ::= letter { letter | digit | "_" } ;
nat                    ::= digit { digit } ;
string_literal         ::= '"' { character } '"' ;

list<T>                ::= "[" [ T { "," T } ] "]" ;
pair<A,B>              ::= "(" A "," B ")" ;
tuple_literal          ::= "(" identifier { "," identifier } ")" ;
```

### 5.1. Enumeraciones cerradas

```ebnf
role_literal               ::= "Base" | "Supervisor" | "Composite" ;

regime_literal             ::= "Simple" | "General" ;

semantic_relation_kind     ::= "DeclaredRelation" ;

pattern_kind               ::= "DeclaredPattern" ;

query_type_literal         ::= "PointEvaluation"
                             | "TrajectoryState"
                             | "FrameComparison"
                             | "CoverageState"
                             | "PendingU"
                             | "GlobalCriticality" ;

scope_literal              ::= "Cell"
                             | "Pair"
                             | "Architecture"
                             | "Trajectory" ;

supervisable_literal       ::= "CellTarget" "(" identifier ")"
                             | "ComposedTarget" "(" identifier ")"
                             | "SystemTarget" "(" identifier ")" ;
```

Nota sobre `semantic_relation_kind` y `pattern_kind`: en v0.1 se admite únicamente la variante `DeclaredRelation` y `DeclaredPattern`. No se cierra un catálogo fuerte de clases de relación o patrón porque el corpus doctrinal no lo ha congelado como enumeración cerrada. El mecanismo de extensión quedará definido en versiones posteriores.

### 5.2. Nivel 0 — Definición

```ebnf
codomain_decl              ::= "codomain" identifier "="
                               "{" identifier { "," identifier } "}" ";" ;

output_semantics_decl      ::= "output_semantics" identifier "{"
                               { identifier "->" string_literal ";" }
                               "}" ;

cellspec_decl              ::= "cellspec" identifier "{"
                               "b" ":" nat ";"
                               "codomain" ":" identifier ";"
                               "semantics" ":" identifier ";"
                               "role" ":" role_literal ";"
                               "}" ;

coupledspec_decl           ::= "coupledspec" identifier "{"
                               "cell" ":" identifier ";"
                               "bridges" ":" list<nat> ";"
                               "}" ;

connector_decl             ::= "connector" identifier "{"
                               "source_codomain" ":" identifier ";"
                               "target_position" ":" nat ";"
                               "mapping" ":" "{"
                                   { identifier "->" tri_literal ";" }
                               "}" ";" ;

table_decl                 ::= "admissibility_table" identifier "{"
                               "input_codomains" ":" list<identifier> ";"
                               "output_codomain" ":" identifier ";"
                               "table" ":" "{"
                                   { tuple_literal "->" identifier ";" }
                               "}" ";" ;

capture_decl               ::= "capture_spec" identifier "{"
                               "parameter_id" ":" nat ";"
                               "observation_domain" ":" identifier ";"
                               "observation_space" ":" identifier ";"
                               "failure_symbol" ":" "Bottom" ";"
                               "mapping" ":" identifier ";"
                               "}" ;

admissibility_decl         ::= "admissibility_spec" identifier "{"
                               "parameter_id" ":" nat ";"
                               "states" ":" "{Ok, Degraded, Failed, U}" ";"
                               "rule" ":" identifier ";"
                               "}" ;

ternarizer_decl            ::= "ternarizer" identifier "{"
                               "observation_space" ":" identifier ";"
                               "partition_zero" ":" identifier ";"
                               "partition_one" ":" identifier ";"
                               "partition_u" ":" identifier ";"
                               "mapping" ":" identifier ";"
                               "}" ;

resspec_decl               ::= "res_spec" identifier "{"
                               "context" ":" identifier ";"
                               "mechanism" ":" identifier ";"
                               "mapping" ":" identifier ";"
                               "}" ;
```

Precondiciones verificadas en lowering para `cellspec`:
- `b ≥ 3` (error si se viola).
- `n` se deriva automáticamente como `b²`. El usuario no escribe `n`.

### 5.3. Nivel 1 — Estado

```ebnf
cellstate_decl             ::= "cellstate" identifier "{"
                               "spec" ":" identifier ";"
                               "vector" ":" list<tri_literal> ";"
                               "}" ;

coupledstate_decl          ::= "coupledstate" identifier "{"
                               "spec" ":" identifier ";"
                               "base_vector" ":" list<tri_literal> ";"
                               "updated_vector" ":" list<tri_literal> ";"
                               "}" ;

edge_literal               ::= "edge" "("
                               "source" ":" identifier ","
                               "target" ":" identifier ","
                               "position" ":" nat ","
                               "connector" ":" identifier
                               ")" ;

graph_decl                 ::= "graph" identifier "{"
                               "nodes" ":" list<identifier> ";"
                               "edges" ":" list<edge_literal> ";"
                               "relation" ":" identifier ";"
                               "regime" ":" regime_literal ";"
                               [ "conflicts" ":" list<identifier> ";" ]
                               "}" ;
```

Precondiciones verificadas en lowering para `cellstate`:
- La longitud del vector debe ser igual a `b²` del `CellSpec` referenciado por `spec`.
- Cada elemento del vector debe ser un `tri_literal` válido.

Precondiciones verificadas en lowering para `graph`:
- El campo `relation` debe apuntar a un identificador de `SemanticRelation` declarado. Sin relación semántica previa, no hay composición (invariante constitutivo 6).
- El grafo no puede contener ciclos (Bloque D.5 de la Frontera).

### 5.4. Composición — Relaciones y patrones

```ebnf
semantic_relation_decl     ::= "semantic_relation" identifier "{"
                               "kind" ":" semantic_relation_kind ";"
                               [ "table" ":" identifier ";" ]
                               [ "constraints" ":" list<identifier> ";" ]
                               "}" ;

pattern_decl               ::= "pattern" identifier "{"
                               "kind" ":" pattern_kind ";"
                               [ "arity" ":" nat ";" ]
                               [ "constraints" ":" list<identifier> ";" ]
                               "}" ;
```

Estas declaraciones son nominales y austeras. No cierran más semántica que la necesaria para alimentar `compose(...)`. La teoría fuerte de relaciones y patrones queda abierta para versiones posteriores, conforme al Grado C de `Comp` en el corpus.

### 5.5. Nivel 3 — Evolución

```ebnf
horizon_decl               ::= "horizon" identifier "{"
                               "architecture" ":" identifier ";"
                               "events" ":" list<identifier> ";"
                               "}" ;

frame_decl                 ::= "frame" identifier "{"
                               "index" ":" nat ";"
                               "architecture" ":" identifier ";"
                               "cell_states" ":" list<identifier> ";"
                               "eval_results" ":" list<identifier> ";"
                               "gate_results" ":" list<identifier> ";"
                               "supervision" ":" list<identifier> ";"
                               "criticalities" ":" list<identifier> ";"
                               "}" ;

transitiondata_decl        ::= "transition_data" identifier "{"
                               "horizon_ref" ":" identifier ";"
                               "events" ":" list<identifier> ";"
                               "induced_parameters" ":" list<induced_param_literal> ";"
                               [ "metadata" ":" list<identifier> ";" ]
                               "}" ;

induced_param_literal      ::= "(" identifier "," nat "," tri_literal ")" ;

trajectory_entry           ::= "entry" "("
                               "frame" ":" identifier
                               [ "," "transition" ":" identifier ]
                               ")" ;

trajectory_decl            ::= "trajectory" identifier "{"
                               "entries" ":" list<trajectory_entry> ";"
                               "}" ;
```

Invariantes constitutivos garantizados por el tipo (no escritos por el usuario):
- **Frame:** inmutable una vez construido (invariante 8). El compilador rechaza toda operación que intente modificar un frame existente.
- **Trajectory:** append-only (invariante 9). El compilador solo permite añadir entradas al final; no permite reescritura, eliminación ni reordenación.
- **TrajectoryEntry:** respeta las 4 invariantes de alternancia definidas en la IR v0.2.

### 5.6. Nivel 4 — Uso

```ebnf
domain_decl                ::= "domain" identifier "{"
                               "parameters" ":" list<identifier> ";"
                               "interface" ":" identifier ";"
                               "horizon" ":" identifier ";"
                               "capture_specs" ":" list<identifier> ";"
                               "admissibility_specs" ":" list<identifier> ";"
                               "ternarizers" ":" list<identifier> ";"
                               "exogeneity_mask" ":" identifier ";"
                               "silent_u" ":" identifier ";"
                               "transduction_policy" ":" identifier ";"
                               "u_policy" ":" identifier ";"
                               "closure_criterion" ":" identifier ";"
                               "}" ;

agent_decl                 ::= "agent" identifier "{"
                               "architecture" ":" identifier ";"
                               "domain" ":" identifier ";"
                               "query_engine" ":" identifier ";"
                               "}" ;

queryspec_decl             ::= "query_spec" identifier "{"
                               "query_type" ":" query_type_literal ";"
                               "scope" ":" scope_literal ";"
                               "restrictions" ":" list<identifier> ";"
                               "}" ;
```

### 5.7. Operadores

```ebnf
eval_cmd                   ::= "let" identifier "=" "evaluate" "(" identifier ")" ";" ;

gate_cmd                   ::= "let" identifier "=" "gate" "("
                               list<identifier> "," "using" ":" identifier
                               ")" ";" ;

resolve_cmd                ::= "let" identifier "=" "resolve" "("
                               "U" ","
                               "with" ":" identifier ","
                               "context" ":" identifier ","
                               "mechanism" ":" identifier
                               ")" ";" ;

query_context_literal      ::= "PointEval" "(" identifier ")"
                             | "TrajectoryView" "(" identifier ")"
                             | "FrameComparison" "(" identifier "," identifier ")"
                             | "ArchitectureView" "(" identifier "," list<identifier> ","
                                                       list<identifier> "," list<identifier> ")"
                             | "CoverageReport" "(" identifier "," identifier ","
                                                     identifier ")" ;

query_cmd                  ::= "let" identifier "=" "query" "("
                               identifier ","
                               "by" ":" identifier ","
                               "in" ":" query_context_literal
                               ")" ";" ;

supervise_cmd              ::= "let" identifier "=" "supervise" "("
                               identifier ","
                               "target" ":" supervisable_literal
                               ")" ";" ;

compose_cmd                ::= "let" identifier "=" "compose" "("
                               identifier ","
                               "relations" ":" list<identifier> ","
                               "patterns" ":" list<identifier>
                               ")" ";" ;

projection_cmd             ::= "let" identifier "=" identifier "." identifier ";" ;
```

Restricciones semánticas verificadas en lowering:

- **`evaluate`:** el argumento debe ser un identificador de `CellState` o `CoupledState`. El operador ejecuta internamente la factorización `evaluate + classify` cerrada por la IR v0.2. El `EvalResult` resultante expone conteos (`N₀`, `N₁`, `Nᵤ`), umbral (`T(n)`), clasificación y `source_state`.
- **`gate`:** todos los identificadores de la lista deben ser `EvalResult`. No se aceptan otros tipos. La tabla referida por `using` debe ser un `AdmissibilityTable` declarado.
- **`resolve`:** devuelve un `ResolutionRecord`. El valor ternario resuelto se obtiene por proyección explícita: `let v = RR1.value;`. No devuelve un par implícito ni un tipo compuesto anónimo.
- **`query`:** el constructor de `QueryContext` debe ser explícito (una de las cinco variantes). No se acepta un identificador opaco como contexto.
- **`supervise`:** el `target` debe ser un constructor explícito de `Supervisable` (`CellTarget`, `ComposedTarget` o `SystemTarget`). No se acepta un identificador opaco.
- **`compose`:** las listas de `relations` y `patterns` deben contener identificadores de `SemanticRelation` y `Pattern` declarados previamente. No se acepta `compose` sin relaciones ni patrones.
- **`projection`:** permite acceder a campos de objetos de resultado. Es la única forma de extraer valores internos (como `ResolutionRecord.value`).

---

## 6. Lowering unívoco

Cada forma superficial baja a exactamente un objeto o una operación de la IR v0.2. No hay ambigüedad.

### 6.1. Declaraciones → Objetos IR

| Forma superficial | Objeto IR (nivel) |
|-------------------|-------------------|
| `codomain` | `Codomain` (N0) |
| `output_semantics` | `OutputSemantics` (N0) |
| `cellspec` | `CellSpec` (N0), con `n := b²` derivado |
| `coupledspec` | `CoupledSpec` (N0) |
| `connector` | `Connector` (N0) |
| `admissibility_table` | `AdmissibilityTable` (N0) |
| `capture_spec` | `CaptureSpec` (N0) |
| `admissibility_spec` | `AdmissibilitySpec` (N0) |
| `ternarizer` | `Ternarizer` (N0) |
| `res_spec` | `ResSpec` (N0) |
| `cellstate` | `CellState` (N1) |
| `coupledstate` | `CoupledState` (N1) |
| `graph` | `CompositionGraph` (N1), con `Edge` enriquecido |
| `semantic_relation` | Entrada de relación para `Comp` |
| `pattern` | Entrada de patrón para `Comp` |
| `horizon` | `Horizon` (N3) |
| `frame` | `Frame` (N3), inmutable por tipo |
| `transition_data` | `TransitionData` (N3), con `horizon_ref` e `induced_parameters` |
| `trajectory` | `Trajectory` (N3), append-only por tipo |
| `domain` | `Domain` (N4) |
| `agent` | `Agent` (N4) |
| `query_spec` | `QuerySpec` (N4) |

### 6.2. Operadores → Resultados IR

| Operador superficial | Resultado IR (nivel) |
|---------------------|---------------------|
| `evaluate(S)` | `EvalResult` (N2) |
| `gate([...], using: T)` | `GateResult` (N2) |
| `resolve(U, with: R, context: X, mechanism: M)` | `ResolutionRecord` (N2) |
| `query(Q, by: A, in: QC)` | `QueryResult` (N2) |
| `supervise(E, target: T)` | `SupervisionResult` (N2) |
| `compose(G, relations: [...], patterns: [...])` | `Architecture` |

### 6.3. Lowering interno de `evaluate`

El operador `evaluate` ejecuta internamente la factorización `evaluate + classify` cerrada por la IR v0.2:

1. **evaluate:** lectura de conteos `N₀(v)`, `N₁(v)`, `Nᵤ(v)` sobre el vector del estado.
2. **classify:** aplicación de `T(n) = ⌊7n/9⌋` con precedencia desfavorable (si `N₁ ≥ T(n)` → NO_APTO; si `N₀ ≥ T(n)` → APTO; otro caso → INDETERMINADO).
3. **Γ:** cálculo de criticidad `Γ(v) = Nᵤ(v) − min(δ₀(v), δ₁(v))`.

El `EvalResult` resultante contiene: conteos, umbral, clasificación, criticidad y `source_state`.

### 6.4. Lowering interno de `max/min` (reservado para versiones futuras)

Cuando `max/min` se incorporen como azúcar en una versión posterior, su lowering será:

- `max(E1, E2)` → `gate([E1, E2], using: T_max)` donde `T_max` es la tabla trivial de dominancia homogénea con orden documentado.
- `min(E1, E2)` → `gate([E1, E2], using: T_min)` donde `T_min` es la tabla trivial dual.

Este lowering ya está cerrado en la IR v0.2. La superficie v0.1 simplemente no lo expone aún.

---

## 7. Prohibiciones sintácticas

La gramática prohíbe de raíz las siguientes construcciones. No son advertencias: son imposibilidades del parser.

### 7.1. Prohibiciones derivadas del Bloque D de la Frontera normativa

| Prohibición | Mecanismo de la gramática |
|-------------|--------------------------|
| Coerción implícita de U | No existe literal `null`, `None`, `NaN`, `true`, `false`, entero ni string como sustituto de `Tri`. El tipo ternario solo admite `Zero`, `One`, `U`. |
| Composición sin relación semántica previa | `graph` exige campo `relation` obligatorio. `compose` exige lista no vacía de `relations`. |
| Comp como operador cerrado | `compose` es constructor que recibe grafo, relaciones y patrones. No tiene firma de operador fuerte. |
| Mutación de Frame | No existe operación superficial que modifique un frame existente. |
| Mutación de Trajectory | No existe operación que elimine, reordene o modifique entradas de una trayectoria. |
| `query` sin contexto tipado | El campo `in` de `query` exige un constructor explícito de `QueryContext`, no un identificador opaco. |

### 7.2. Prohibiciones adicionales de la superficie v0.1

| Prohibición | Razón |
|-------------|-------|
| Literales directos de `EvalResult`, `GateResult`, `ResolutionRecord`, `QueryResult`, `SupervisionResult` | Los resultados solo se obtienen por operadores. |
| `max` / `min` como palabras clave | No existen en v0.1. Se reservan para azúcar futuro. |
| `gate` con argumentos que no sean `EvalResult` | Endurece la superficie y elimina ambigüedad de tipo en el lowering. |
| `supervise` con `target` opaco | Exige constructor explícito de `Supervisable`. |
| Alias que colisionen con la nomenclatura desambiguada de la Frontera | No se permiten redefiniciones de `Tri`, `Codomain`, `OutputSemantics`, `Role`, `BridgeSet`, `Connector`, `Ternarizer`, `AdmissibilityTable`, `InducedTransitionOp`, `VisualCode`, `Influence`. |

---

## 8. Ejemplo mínimo de superficie

```svp
-- Declaración de codominio y semántica
codomain K3 = { APTO, NO_APTO, INDETERMINADO };

output_semantics Klin {
  APTO -> "Clasificación fuerte favorable";
  NO_APTO -> "Clasificación fuerte desfavorable";
  INDETERMINADO -> "Sin clasificación fuerte";
}

-- Declaración de célula (b=3, n derivado como 9)
cellspec C1 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}

-- Célula acoplable con posición puente en 3
coupledspec CC1 {
  cell: C1;
  bridges: [3];
}

-- Conector de transmisión
connector Phi1 {
  source_codomain: K3;
  target_position: 3;
  mapping: {
    APTO -> Zero;
    NO_APTO -> One;
    INDETERMINADO -> U;
  }
}

-- Estado de la célula
cellstate S1 {
  spec: C1;
  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}

-- Evaluación (factoriza internamente evaluate + classify + Γ)
let E1 = evaluate(S1);

-- Segunda célula y evaluación
cellspec C2 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}

cellstate S2 {
  spec: C2;
  vector: [Zero, Zero, Zero, One, Zero, Zero, Zero, One, Zero];
}

let E2 = evaluate(S2);

-- Compuerta con tabla de admisibilidad
admissibility_table T1 {
  input_codomains: [K3, K3];
  output_codomain: K3;
  table: {
    (APTO, APTO) -> APTO;
    (APTO, NO_APTO) -> NO_APTO;
    (APTO, INDETERMINADO) -> INDETERMINADO;
    (NO_APTO, APTO) -> NO_APTO;
    (NO_APTO, NO_APTO) -> NO_APTO;
    (NO_APTO, INDETERMINADO) -> NO_APTO;
    (INDETERMINADO, APTO) -> INDETERMINADO;
    (INDETERMINADO, NO_APTO) -> NO_APTO;
    (INDETERMINADO, INDETERMINADO) -> INDETERMINADO;
  }
}

let G1 = gate([E1, E2], using: T1);

-- Supervisión meta
cellspec Meta {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Supervisor;
}

cellstate SMeta {
  spec: Meta;
  vector: [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero];
}

let EMeta = evaluate(SMeta);
let SV1 = supervise(EMeta, target: ComposedTarget(G1));

-- Resolución de U con trazabilidad
res_spec RS1 {
  context: ContextoClinico;
  mechanism: RevisionExperto;
  mapping: MapeoResolucion;
}

let RR1 = resolve(U, with: RS1, context: Ctx1, mechanism: M1);
let valor_resuelto = RR1.value;
```

---

## 9. Adversarial de cierre

### 9.1. ¿La gramática preserva la nomenclatura desambiguada?

Sí. Todas las palabras clave coinciden con la tabla de la Frontera normativa v0 (§3). No se introduce ningún nombre nuevo que colisione con los nombres del corpus. Los nombres de operadores (`evaluate`, `gate`, `resolve`, `query`, `supervise`, `compose`) son verbos que no colisionan con los sustantivos de la nomenclatura.

### 9.2. ¿La zona prohibida es sintácticamente inalcanzable?

Sí. No existe producción gramatical para: coerción implícita de U, composición sin relación, literales de resultado, mutación de frame o trayectoria, `max/min` como operadores, ni `query` sin contexto explícito.

### 9.3. ¿El lowering es unívoco?

Sí. Cada forma superficial baja a exactamente un objeto o una operación IR. Los tres puntos que presentaban ambigüedad potencial han sido cerrados: `resolve` devuelve `ResolutionRecord` (con proyección explícita para el valor), `gate` solo acepta `EvalResult`, y `supervise` exige constructor explícito de `Supervisable`.

### 9.4. ¿La gramática cierra más de lo debido?

No. `Comp` se mantiene como constructor austero con `SemanticRelation` y `Pattern` nominales. `semantic_relation_kind` y `pattern_kind` tienen una sola variante cada uno (`DeclaredRelation`, `DeclaredPattern`), lo que refleja que el corpus no ha congelado un catálogo fuerte. `supervise` entra en v0.1 con las tres variantes de `Supervisable` que la IR define, sin añadir ninguna.

### 9.5. ¿Faltan objetos necesarios?

No. `TransitionData` tiene su propia declaración. `SemanticRelation` y `Pattern` son declarables. Todas las enumeraciones tienen producción EBNF explícita. `CellSpec` se declara solo con `b`.

### 9.6. ¿Los 12 invariantes constitutivos se preservan?

| Invariante | Preservado por |
|------------|----------------|
| 1. Alfabeto {0, 1, U} | `tri_literal` cerrado a tres valores |
| 2. U es estado epistémico | Sin coerción a null/NaN/None/int |
| 3. n = b², b ≥ 3 | `cellspec` solo declara `b`; `n` derivado en lowering |
| 4. T(n) = ⌊7n/9⌋ | Aplicado en lowering de `evaluate`, no parametrizable |
| 5. Clasificación determinista | Factorizada dentro de `evaluate` |
| 6. Composición depende de R | `graph` y `compose` exigen relación obligatoria |
| 7. Operadores lícitos de ℱ_SV | Solo `evaluate`, `gate`, `resolve`, `query`, `supervise`, `compose` |
| 8. Polígono/frame inmutable | Frame inmutable por tipo, sin campo editable |
| 9. Trayectoria append-only | Trajectory append-only por tipo, sin operación de modificación |
| 10. IA no soberana | Sin delegación automática de diseño en la gramática |
| 11. Cero estadística | Sin construcciones probabilísticas ni estadísticas |
| 12. Todo auditable | Toda operación produce resultado nombrado y trazable |

---

## 10. Relación con los demás documentos del repositorio

| Documento | Relación con esta gramática |
|-----------|---------------------------|
| Frontera normativa v0 | La gramática implementa los Bloques A y B como superficie declarativa e impide el Bloque D por diseño sintáctico |
| IR canónica v0.2 | La gramática baja unívocamente a la IR. Cada forma superficial tiene exactamente una traducción IR |
| Consenso de Lenguajes | La gramática satisface C1 (tipo ternario semántico), C2 (funciones puras), C3 (precondiciones tipadas), C7 (auditabilidad de U) y C8 (conformidad cruzada por diseño) |
| Fundamentos (R3) | Todos los invariantes constitutivos se preservan |

---

## Referencias

[R1] Juan Antonio Lloret Egea. *Fundamentos algebraico-semánticos del Sistema Vectorial SV.* v1.0.0, Release 3. ITVIA, 2026. https://www.itvia.online/pub/fundamentos-algebraico-semanticos-del-sistema-vectorial-sv/release/3

[R2] Juan Antonio Lloret Egea. *Frontera normativa del lenguaje de computación del Sistema Vectorial SV — v0.* Marzo 2026. https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md

[R3] Juan Antonio Lloret Egea. *IR canónica y sistema de bienformación del lenguaje SV — v0.2.* Marzo 2026. https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/IR_CANONICA_BIENFORMACION_SV_v0_2.md

[R4] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — I. Transmisión en serie por parámetro puente.* v1, Release 4. ITVIA, 2026.

[R5] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — II. Gramática general de composición.* v1.0, Release 1. ITVIA, 2026.

[R6] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — III. Horizonte de sucesos y reevaluación discreta.* v1, Release 1. ITVIA, 2026.

[R7] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — IV. Transducción al alfabeto ternario e interfaz paramétrica del sistema.* v1, Release 1. ITVIA, 2026.

[R8] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — V. Invariantes, agentes especializados y operador de consulta del sistema.* v2, Release 2. ITVIA, 2026.

[R9] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — VI. Análisis discreto, representaciones y herramientas de secuencias del sistema.* v1, Release 1. ITVIA, 2026.

---

*Especificación técnica del lenguaje de computación del Sistema Vectorial SV. ISSN 2695-6411.*
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0*
