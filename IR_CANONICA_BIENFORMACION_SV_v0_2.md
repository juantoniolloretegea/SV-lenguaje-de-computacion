# Especificación de la IR canónica y del sistema de bienformación del lenguaje SV — v0.2

## Representación intermedia normalizada, juicios de legalidad formal y errores constitutivos

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** [0000-0002-6634-3351](https://orcid.org/0000-0002-6634-3351)  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** marzo 2026  
**Estado:** Especificación técnica del lenguaje — v0.2  
**Repositorio:** [SV-lenguaje-de-computacion](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion)

© Juan Antonio Lloret Egea, marzo 2026. Todos los derechos reservados conforme a CC BY-NC-ND 4.0.

---

## Resumen

Este documento define la representación intermedia canónica (IR) del lenguaje de computación del Sistema Vectorial SV y el sistema de juicios de bienformación que la gobiernan. Su propósito es traducir la Frontera normativa v0 a una máquina formal de legalidad: qué objetos existen en la IR, qué invariantes verifican, qué transiciones son lícitas y qué errores formales debe emitir el sistema cuando se violan.

La IR no es la sintaxis del usuario ni el backend de ejecución. Es la capa intermedia normalizada, serializable y auditable que toda construcción del lenguaje debe atravesar antes de alcanzar cualquier backend. Su diseño garantiza que la conformidad cruzada entre implementaciones se verifica sobre una representación común, no sobre convenciones sintácticas de superficie.

---

## 1. Alcance y posición

### 1.1. Qué es la IR

La IR canónica es la representación normalizada de todo programa válido del lenguaje SV. Toda construcción del DSL se traduce (lowering) a la IR antes de llegar a Rust, Python o cualquier otro backend. La IR es el objeto que se audita, se serializa, se reproduce y se compara entre implementaciones.

### 1.2. Qué no es la IR

La IR no es la sintaxis del usuario (que pertenece a la gramática superficial, aún no definida). No es el código objeto del backend (que pertenece a la fase de compilación). No es un formato de intercambio de datos clínicos (que pertenece al dominio).

### 1.3. Subordinación

Este documento es doctrina técnica derivada de la Frontera normativa del lenguaje SV v0 y, a través de ella, del corpus doctrinal completo del Sistema Vectorial SV (Fundamentos + Documentos I–VI). En caso de discrepancia, prevalece la Frontera normativa; en caso de discrepancia entre la Frontera y el corpus doctrinal, prevalecen los Fundamentos.

---

## 2. Niveles ontológicos de la IR

La IR separa cinco niveles ontológicos que no pueden mezclarse. Todo objeto de la IR pertenece a exactamente un nivel.

| Nivel | Qué contiene | Ejemplo |
|-------|-------------|---------|
| N0 — Definición | Especificaciones inmutables de estructura | CellSpec, Connector, AdmissibilityTable, Ternarizer, CaptureSpec, AdmissibilitySpec, ResSpec |
| N1 — Estado | Configuraciones evaluables con datos concretos | CellState, CoupledState, CompositionGraph |
| N2 — Resultado | Salidas deterministas de evaluación | EvalResult, GateResult, SupervisionResult, CriticalityResult, ResolutionRecord |
| N3 — Evolución | Estructuras de reevaluación y trayectoria | Frame, TransitionData, Trajectory, Horizon |
| N4 — Uso | Especialización, consulta y análisis | Domain, Agent, QuerySpec, QueryContext, QueryResult, AnalyticView |

La regla cardinal es: un objeto de nivel superior puede referenciar objetos de niveles inferiores, pero nunca al revés. Un CellSpec no puede referenciar un Frame. Un Frame puede referenciar un EvalResult. Una Trajectory puede referenciar Frames.

---

## 3. Objetos canónicos de la IR

### Nivel 0 — Definición

**CellSpec** — Especificación inmutable de una célula composable.

    CellSpec = {
        n         : SquareNat,          -- n = b², b >= 3
        b         : Nat,                -- base estructural
        codomain  : Codomain,           -- espacio de salidas posibles
        semantics : OutputSemantics,    -- interpretación del codominio
        role      : Role                -- rol en la arquitectura
    }

**CoupledSpec** — Especificación de una célula acoplable (extiende CellSpec).

    CoupledSpec = {
        cell    : CellSpec,
        bridges : BridgeSet             -- B ⊆ {1, …, n}
    }

**Connector** — Conector de transmisión intercelular.

    Connector = {
        source_codomain : Codomain,     -- K de la célula transmisora
        target_position : Nat,          -- posición puente en la receptora
        mapping : Codomain -> Tri       -- φ : K → Σ
    }

**AdmissibilityTable** — Tabla de admisibilidad de una compuerta.

    AdmissibilityTable = {
        input_codomains : [Codomain],       -- K₁, K₂, ..., Kₖ
        output_codomain : Codomain,         -- K_comp
        table : [Codomain] -> Codomain      -- 𝒯 : K₁ × ... × Kₖ → K_comp
    }

**CaptureSpec** — Especificación formal del sensor o procedimiento de captura (Doc IV, ϕⱼ).

    CaptureSpec = {
        parameter_id      : Nat,                  -- Pⱼ
        observation_domain : ObservationDomain,    -- Wⱼ
        observation_space  : ObservationSpace,     -- Oⱼ
        failure_symbol     : Bottom,               -- ⊥
        mapping : ObservationDomain -> ObservationSpaceExt  -- ϕⱼ : Wⱼ → Oⱼ^⊥
    }

**AdmissibilitySpec** — Especificación formal del estado de admisibilidad observacional (Doc IV, R).

    AdmissibilitySpec = {
        parameter_id : Nat,
        states : {Ok, Degraded, Failed, U},        -- R = {ok, degradado, fallido, U}
        rule   : (ObservationSpaceExt) -> AdmissibilityState
    }

**Ternarizer** — Función de ternarización (Doc IV, τⱼ).

    Ternarizer = {
        observation_space : ObservationSpace,      -- Oⱼ
        partition_zero    : Set,                   -- B₀
        partition_one     : Set,                   -- B₁
        partition_u       : Set,                   -- B_U
        mapping : ObservationSpace -> Tri          -- τⱼ : Oⱼ → Σ
    }

**ResSpec** — Especificación del mecanismo de resolución de U.

    ResSpec = {
        context   : Context,            -- 𝒳 : contexto de evidencia
        mechanism : Mechanism,          -- ℛ : mecanismo de revisión
        mapping   : (U, Context, Mechanism) -> Tri
    }

---

### Nivel 1 — Estado

**CellState** — Estado concreto de una célula con vector ternario.

    CellState = {
        spec   : CellSpec,
        vector : Vec<Tri>               -- v ∈ Σⁿ, longitud = spec.n
    }

**CoupledState** — Estado de una célula acoplable con vector base y actualizado.

    CoupledState = {
        spec           : CoupledSpec,
        base_vector    : Vec<Tri>,      -- xᵢ⁽⁰⁾
        updated_vector : Vec<Tri>       -- x̃ᵢ (tras transmisiones)
    }

**Edge** — Arista enriquecida del grafo de composición.

    Edge = {
        source    : NodeId,
        target    : NodeId,
        position  : Nat,                -- k ∈ BridgeSet del target
        connector : Connector           -- φ asociado a esta arista
    }

**CompositionGraph** — Grafo dirigido acíclico de células acopladas.

    CompositionGraph = {
        nodes    : [CoupledSpec],       -- V
        edges    : [Edge],              -- E (cada arista lleva su conector)
        relation : SemanticRelation,    -- R(𝒜)
        regime   : Regime               -- Simple | General
    }

**ConflictOperator** — Operador de conflicto para régimen general.

    ConflictOperator = {
        target_cell     : NodeId,
        target_position : Nat,          -- k ∈ Bᵢ
        family          : ConflictFamily,   -- Consensus | Precedence | LocalSovereignty
        mapping         : (Tri, [Tri]) -> Tri   -- Ψ : Σ × Σʳ → Σ
    }

---

### Nivel 2 — Resultado

**EvalResult** — Resultado de evaluación de una célula.

    EvalResult = {
        source_state    : CellStateRef,    -- enlace al CellState que lo produjo
        counts          : (Nat, Nat, Nat), -- (N₀, N₁, Nᵤ)
        threshold       : Nat,             -- T(n)
        classification  : Classification,  -- APTO | NO_APTO | INDETERMINADO
        criticality     : Int,             -- Γ(v)
        deltas          : (Int, Int)       -- (δ₀, δ₁)
    }

**GateResult** — Resultado de una compuerta.

    GateResult = {
        inputs : [EvalResult],
        table  : AdmissibilityTable,
        output : Codomain                  -- 𝒯(y₁, y₂, ..., yₖ)
    }

**SupervisionResult** — Resultado de supervisión meta.

    SupervisionResult = {
        meta_eval : EvalResult,
        target    : Supervisable,
        verdict   : SupervisionVerdict     -- Accept | Tense | Veto
    }

**Supervisable** — Unión etiquetada del objeto supervisado.

    Supervisable =
        | CellTarget(EvalResult)           -- nivel (a): supervisa una célula
        | ComposedTarget(GateResult)       -- nivel (b): supervisa un resultado compuesto
        | SystemTarget(CompositionGraph)   -- nivel (c): supervisa un sistema completo

**CriticalityResult** — Resultado de criticidad condicionada al puente.

    CriticalityResult = {
        source_state  : CellStateRef,
        bridge_k      : Nat,
        delta_gamma_k : BoundedInt         -- ΔΓₖ ∈ {0, 1}
    }

**ResolutionRecord** — Registro de una resolución de U.

    ResolutionRecord = {
        parameter   : Nat,
        previous    : Tri,                 -- siempre U
        resolved_to : Tri,                 -- 0 o 1 (o U si persiste)
        context     : Context,
        mechanism   : Mechanism
    }

---

### Nivel 3 — Evolución

**Horizon** — Horizonte de sucesos de una arquitectura.

    Horizon = {
        architecture : ArchitectureId,
        events       : [EventType]         -- ℋ(𝒜) = {ε₁, ..., εₘ}
    }

**Frame** — Evaluación completa inmutable del sistema.

    Frame = {
        index          : Nat,              -- índice ordinal (no temporal)
        architecture   : ArchitectureId,
        cell_states    : [CoupledState],
        eval_results   : [EvalResult],
        gate_results   : [GateResult],
        supervision    : [SupervisionResult],
        criticalities  : [CriticalityResult],
        immutable      : True              -- invariante constitutivo
    }

**TransitionData** — Dato de transición entre frames.

    TransitionData = {
        horizon_ref        : HorizonRef,            -- referencia al horizonte de la arquitectura
        events             : [(EventType, Tri)],    -- νₙ = {(εᵢ, τᵢ)}
        induced_parameters : [(NodeId, Nat, Tri)],  -- lista de (célula, posición, nuevo valor)
        metadata           : AuditMetadata
    }

**TrajectoryEntry** — Entrada de la trayectoria (frame con transición opcional).

    TrajectoryEntry = {
        frame      : Frame,
        transition : TransitionData?       -- presente salvo en la última entrada
    }

**Trajectory** — Secuencia append-only de frames y transiciones.

    Trajectory = {
        entries     : [TrajectoryEntry],
        append_only : True                 -- invariante constitutivo
    }

    Invariantes de alternancia:
    - entries no puede estar vacío (al menos un frame inicial).
    - Toda entrada que no sea la última DEBE llevar transition.
    - La última entrada NO PUEDE llevar transition.
    - Si una entrada lleva transition, DEBE existir una entrada siguiente.

---

### Nivel 4 — Uso

**Domain** — Dominio especializado.

    Domain = {
        parameters          : [ParameterInstance],
        interface           : Interface,            -- ℐ(𝒜)
        horizon             : Horizon,
        capture_specs       : [CaptureSpec],        -- ϕⱼ por parámetro exógeno
        admissibility_specs : [AdmissibilitySpec],  -- reglas de admisibilidad
        ternarizers         : [Ternarizer],         -- τⱼ por parámetro exógeno
        exogeneity_mask     : ExogeneityMask,       -- E(C)
        silent_u            : SilentU,              -- U_s(𝒜)
        transduction_policy : TransductionPolicy,
        u_policy            : UPolicy,
        closure_criterion   : ClosureCriterion
    }

**Agent** — Agente especializado tipado por dominio.

    Agent = {
        architecture : CompositionGraph,
        domain       : Domain,
        query_engine : QueryEngine
    }

**QuerySpec** — Especificación de una consulta.

    QuerySpec = {
        query_type   : QueryType,
        scope        : Scope,
        restrictions : [Restriction]
    }

**QueryContext** — Contexto de ejecución de una consulta.

    QueryContext =
        | PointEval(Frame)                          -- evaluación puntual
        | TrajectoryView(Trajectory)                -- estado de trayectoria
        | FrameComparison(Frame, Frame)             -- comparación entre frames
        | ArchitectureView(CompositionGraph, [EvalResult], [GateResult], [SupervisionResult])
        | CoverageReport(Domain, Interface, SilentU)

**QueryResult** — Resultado de una consulta.

    QueryResult = {
        response      : TypedResponse,
        justification : Justification,     -- J : cadena trazable
        metadata      : QueryMetadata      -- M : criticidad, cobertura, U pendientes
    }

**AnalyticView** — Vista analítica sobre una trayectoria.

    AnalyticView = {
        source_trajectory : TrajectoryId,
        sequences         : [CanonicalSequence],
        encoding          : EncodingDeclaration    -- Injective | Lossy(projection_name)
    }

---

## 4. Juicios de bienformación

Los juicios se organizan por capas. Un juicio de capa superior presupone que los juicios de capas inferiores ya se han verificado.

### Capa 0 — Juicios sobre valores primitivos

**J0.1** — Un valor de tipo Tri es bien formado si pertenece a {Zero, One, U} y no ha sido producido por coerción implícita desde otro tipo.

**J0.2** — Un entero n es un SquareNat bien formado si existe b en N con b >= 3 tal que n = b².

### Capa 1 — Juicios sobre definiciones

**J1.1** — Un CellSpec es bien formado si:
- n es SquareNat bien formado (J0.2);
- b >= 3 y n = b² (coherencia obligatoria entre n y b almacenados);
- codomain es finito, explícito y no vacío;
- semantics documenta la interpretación de cada elemento del codomain;
- role pertenece a un conjunto declarado de roles.

**J1.2** — Un CoupledSpec es bien formado si:
- cell es CellSpec bien formado (J1.1);
- bridges es subconjunto de {1, ..., n};
- si bridges es vacío, la célula es composable pero no participa en transmisión en serie.

**J1.3** — Un Connector es bien formado si:
- source_codomain coincide con el codomain de la célula transmisora;
- target_position pertenece al BridgeSet de la célula receptora;
- mapping produce valores en Tri para todo elemento del source_codomain.

**J1.4** — Una AdmissibilityTable es bien formada si:
- está completamente definida para toda combinación de entradas;
- es determinista: cada combinación produce exactamente una salida;
- la asimetría, si existe, está documentada;
- existe una relación semántica previa R(A) declarada antes de la tabla.

**J1.5** — Un Ternarizer es bien formado si:
- la partición (B_0, B_1, B_U) cubre todo el espacio observacional sin solapamiento;
- mapping produce valores en Tri;
- la regla conservadora se cumple: inadmisibilidad produce U.

**J1.6** — Un ResSpec es bien formado si:
- context y mechanism son no vacíos y explícitos;
- mapping produce valores en Tri;
- no hay resolución de U sin contexto declarado.

**J1.7** — Un CaptureSpec es bien formado si:
- observation_domain está tipado y documentado;
- mapping produce valores en ObservationSpaceExt (incluyendo ⊥);
- si mapping produce ⊥, el parámetro receptor recibe U.

**J1.8** — Un AdmissibilitySpec es bien formado si:
- states = {Ok, Degraded, Failed, U};
- rule es determinista para toda entrada;
- si el resultado es Failed o U, el parámetro receptor recibe U.

### Capa 2 — Juicios sobre estados

**J2.1** — Un CellState es bien formado si:
- spec es CellSpec bien formado (J1.1);
- vector tiene longitud exactamente spec.n;
- cada componente del vector es Tri bien formado (J0.1).

**J2.2** — Un CoupledState es bien formado si:
- spec es CoupledSpec bien formado (J1.2);
- base_vector y updated_vector tienen longitud spec.cell.n;
- updated_vector difiere de base_vector solo en posiciones de bridges;
- los valores en posiciones de bridges provienen de un Connector bien formado (J1.3).

**J2.3** — Un CompositionGraph es bien formado si:
- todos los nodos son CoupledSpec bien formados (J1.2);
- toda arista (Edge) satisface: position pertenece al BridgeSet del target, y connector es bien formado (J1.3);
- el grafo es acíclico (existe orden topológico);
- relation es una SemanticRelation declarada explícitamente;
- en régimen simple: cada par (target, position) recibe a lo sumo una arista;
- en régimen general: cada par (target, position) con concurrencia tiene ConflictOperator declarado.

### Capa 3 — Juicios sobre resultados

**J3.1** — Un EvalResult es bien formado respecto de un CellState si:
- source_state referencia un CellState bien formado (J2.1);
- counts = (N_0, N_1, N_U) satisface N_0 + N_1 + N_U = source_state.spec.n;
- threshold = floor(7 * source_state.spec.n / 9);
- classification sigue la regla de precedencia desfavorable;
- criticality = N_U - min(delta_0, delta_1);
- los valores son deterministas y reproducibles a partir del vector.

**J3.2** — Un GateResult es bien formado si:
- inputs son EvalResults bien formados (J3.1);
- table es AdmissibilityTable bien formada (J1.4);
- output es determinista: table aplicada a inputs produce siempre el mismo output.

**J3.3** — Un SupervisionResult es bien formado si:
- meta_eval proviene de una célula de segundo orden (sus parámetros miden propiedades del proceso, no del dominio);
- target es un Supervisable bien etiquetado (CellTarget, ComposedTarget o SystemTarget);
- verdict es determinista dado meta_eval;
- si verdict = Veto, el resultado del objeto supervisado queda anulado.

**J3.4** — Un ResolutionRecord es bien formado si:
- previous = U;
- resolved_to pertenece a Tri;
- context y mechanism son no vacíos;
- el registro es inmutable una vez creado.

### Capa 4 — Juicios sobre evolución

**J4.1** — Un Frame es bien formado si:
- todos los CoupledStates son bien formados (J2.2);
- todos los EvalResults son bien formados respecto de sus CellStates (J3.1);
- el frame es inmutable: ninguna operación posterior puede modificarlo.

**J4.2** — Una Trajectory es bien formada si:
- entries no está vacío;
- cada Frame en entries es bien formado (J4.1);
- cada TransitionData es bien formada (J4.3), con horizon_ref coherente con la arquitectura;
- toda entrada que no sea la última lleva transition;
- la última entrada no lleva transition;
- si una entrada lleva transition, existe una entrada siguiente;
- la secuencia es append-only: no se elimina ni modifica ningún elemento previo.

**J4.3** — Un TransitionData es bien formado si:
- horizon_ref referencia un Horizon bien formado y perteneciente a la arquitectura;
- todo evento referenciado pertenece al Horizon referenciado por horizon_ref;
- todo valor de estado del evento pertenece a Tri;
- induced_parameters especifica al menos un cambio de parámetro;
- el dato es suficiente para reconstruir el operador inducido a partir de induced_parameters.

### Capa 5 — Juicios sobre uso

**J5.1** — Un QuerySpec es bien formado respecto de un QueryContext si satisface CQ1–CQ6:
- CQ1: la respuesta debe venir acompañada de justificación reconstruible;
- CQ2: la justificación no puede contener pasos opacos;
- CQ3: la consulta no modifica la trayectoria;
- CQ4: la respuesta declara de qué porción de la interfaz depende;
- CQ5: la consulta es compatible con la trayectoria ya constituida;
- CQ6: la consulta declara criticidad y U que afectan a su cierre;
- el query_type es coherente con el tipo de QueryContext (PointEval requiere Frame, TrajectoryView requiere Trajectory, etc.).

**J5.2** — Un AnalyticView es bien formado si:
- las secuencias se extraen canónicamente de una trayectoria bien formada (J4.2);
- la codificación (encoding) declara explícitamente si es inyectiva o con pérdida;
- si es con pérdida, está marcada como proyección declarada y no como equivalente.

---

## 5. Errores formales obligatorios

Toda violación de un juicio de bienformación produce un error formal. Los errores son constitutivos del lenguaje: no son warnings, no son sugerencias, no se pueden silenciar.

### Errores de Capa 0

| Código | Nombre | Juicio violado |
|--------|--------|----------------|
| E001 | InvalidTriCoercion | J0.1 — conversión implícita de Tri a otro tipo |
| E002 | InvalidCellSize | J0.2 — n no es b² con b >= 3 |

### Errores de Capa 1

| Código | Nombre | Juicio violado |
|--------|--------|----------------|
| E101 | EmptyCodomain | J1.1 — codomain vacío o no documentado |
| E102 | MissingOutputSemantics | J1.1 — OutputSemantics ausente |
| E103 | IllegalBridgePosition | J1.2 — posición puente fuera de {1, ..., n} |
| E104 | InvalidConnectorCodomain | J1.3 — mapping no produce Tri para alguna entrada |
| E105 | IncompleteAdmissibilityTable | J1.4 — tabla no definida para alguna combinación |
| E106 | MissingSemanticRelation | J1.4 — composición sin relación semántica previa |
| E107 | InvalidTernarizerPartition | J1.5 — partición no cubre o tiene solapamiento |
| E108 | MissingResContext | J1.6 — resolución de U sin contexto o mecanismo |
| E109 | InvalidCaptureSpec | J1.7 — captura mal tipada o sin manejo de fallo |
| E110 | InvalidAdmissibilitySpec | J1.8 — regla de admisibilidad no determinista |
| E111 | UnorderedCodomain | Lowering 6.1 — codomain sin orden total explícito para max/min |

### Errores de Capa 2

| Código | Nombre | Juicio violado |
|--------|--------|----------------|
| E201 | VectorLengthMismatch | J2.1 — longitud del vector no coincide con spec.n |
| E202 | IllegalBridgeUpdate | J2.2 — posición no puente recibe transmisión |
| E203 | CyclicCompositionGraph | J2.3 — ciclo detectado en el grafo |
| E204 | MissingConflictOperator | J2.3 — concurrencia sin operador de conflicto |
| E205 | UndeclaredRegime | J2.3 — grafo no declara régimen |
| E206 | EdgeConnectorMismatch | J2.3 — arista con conector incompatible |

### Errores de Capa 3

| Código | Nombre | Juicio violado |
|--------|--------|----------------|
| E301 | InconsistentCounts | J3.1 — N₀ + N₁ + Nᵤ distinto de n |
| E302 | WrongThreshold | J3.1 — umbral no coincide con floor(7n/9) |
| E303 | ClassificationPrecedenceViolation | J3.1 — no respeta precedencia desfavorable |
| E304 | NonDeterministicGate | J3.2 — tabla produce salidas distintas para misma entrada |
| E305 | UnsafeUResolution | J3.4 — resolución sin ResolutionRecord bien formado |
| E306 | UntaggedSupervisable | J3.3 — objeto supervisado sin etiqueta de nivel |

### Errores de Capa 4

| Código | Nombre | Juicio violado |
|--------|--------|----------------|
| E401 | FrameMutationAttempt | J4.1 — intento de modificar frame construido |
| E402 | NonAppendOnlyTrajectory | J4.2 — intento de eliminar o modificar entrada previa |
| E403 | UndeclaredHorizonEvent | J4.3 — evento no pertenece al horizonte |
| E404 | BrokenAlternation | J4.2 — entrada no final sin transición, o última con transición |
| E405 | EmptyTrajectory | J4.2 — trayectoria sin ninguna entrada |
| E406 | InsufficientTransitionData | J4.3 — induced_parameters vacío |

### Errores de Capa 5

| Código | Nombre | Juicio violado |
|--------|--------|----------------|
| E501 | OpaqueJustification | J5.1 / CQ2 — justificación no trazable |
| E502 | QueryMutatesTrajectory | J5.1 / CQ3 — consulta modifica trayectoria |
| E503 | StrongConclusionUnderInsufficientCoverage | J5.1 / CQ6 — 0 o 1 fuerte sin cobertura |
| E504 | UndeclaredLossyEncoding | J5.2 — codificación con pérdida no declarada |
| E505 | IllegalCompClosure | Bloque D — Comp tratado como operador cerrado |
| E506 | AutomatedDesignDelegation | Bloque D — proceso automático modifica diseño sin humano |
| E507 | QueryContextMismatch | J5.1 — tipo de consulta incoherente con tipo de contexto |

---

## 6. Operaciones de lowering (DSL a IR)

El lowering traduce construcciones del DSL a objetos de la IR sin pérdida semántica ilícita.

### 6.1. max/min bajan a gate con tabla trivial

Toda invocación de `max(Ci, Cj)` se traduce a:

    gate(Ci, Cj, AdmissibilityTable_trivial_max)

Toda invocación de `min(Ci, Cj)` se traduce a:

    gate(Ci, Cj, AdmissibilityTable_trivial_min)

Donde cada tabla trivial implementa la comparación ordinal correspondiente sobre un codomain con orden total explícito y documentado.

Precondición obligatoria: el codomain de ambas células admite orden total explícito y documentado. Si no se cumple, el lowering emite error E111.

La relación semántica previa (dominancia homogénea como subtipo de habilitación cruzada) se genera automáticamente pero queda registrada en la IR.

### 6.2. evaluate + classify bajan como factorización de chi

La evaluación se traduce a dos fases distinguibles en la IR:

    evaluate(CellState) -> (N_0, N_1, N_U)           -- fase vectorial
    classify(N_0, N_1, N_U, T(n)) -> Classification  -- fase normativa

La IR conserva ambas fases como operaciones separadas para trazabilidad.

### 6.3. Res baja con contexto y mecanismo obligatorios

Toda resolución de U se traduce a:

    resolve(U, Context, Mechanism) -> (Tri, ResolutionRecord)

La IR no acepta resolución sin ResolutionRecord bien formado (J3.4).

### 6.4. query baja como operador tipado con contexto

Toda consulta se traduce a:

    query(QuerySpec, Agent, QueryContext) -> QueryResult = (r, J, M)

La IR verifica CQ1–CQ6 y la coherencia entre query_type y QueryContext (J5.1) antes de ejecutar.

### 6.5. Comp baja como constructor, no como operador cerrado

Toda construcción con Comp se traduce a:

    compose(CompositionGraph, [SemanticRelation], [Pattern]) -> Architecture

La IR registra que Comp es constructor organizativo de Grado C. No se le aplican juicios de firma cerrada ni proposiciones fuertes. Si una futura versión del corpus cierra la axiomática de Comp, la IR se actualizará.

---

## 7. Condiciones de serialización y trazabilidad

### 7.1. Serialización

Todo objeto de la IR debe ser serializable a un formato determinista y reproducible. La serialización debe preservar:

- todos los campos obligatorios del objeto;
- los juicios de bienformación que el objeto ha superado;
- los errores formales emitidos, si los hubo;
- los metadatos de auditoría (origen, fecha, versión del lenguaje).

### 7.2. Reproducibilidad

Dado un Frame inicial y una secuencia de TransitionData con contenido suficiente (induced_parameters completos), todo Frame posterior debe ser recalculable de forma determinista a partir de la IR serializada.

### 7.3. Conformidad cruzada

La IR es el nivel en el que se verifica la conformidad cruzada entre backends. Si Rust y Python producen IRs distintas para la misma entrada, al menos uno de los dos es incorrecto. La IR, no la sintaxis ni el código objeto, es el contrato de fidelidad.

---

## 8. Lo que la IR no puede fingir

La IR debe declarar expresamente que no cierra las siguientes cuestiones, porque el corpus doctrinal no las ha cerrado:

1. **Convergencia general de trayectorias.** No existe norma formal de convergencia o divergencia. La IR no puede prometer estabilización.

2. **Teoría multiagente plena.** La composición entre agentes se admite solo bajo relación semántica declarada. La IR no modela interacciones multiagente arbitrarias.

3. **Axiomática completa de transductores.** La IR verifica las condiciones mínimas (partición, codominio ternario, regla conservadora), pero no impone una axiomática cerrada de transductores bien formados.

4. **Operador de entrenamiento.** No forma parte de la IR v0.2.

5. **Equivalencia semántica de codificaciones con pérdida.** La IR las marca como proyecciones declaradas, nunca como representaciones equivalentes.

6. **Firma cerrada de Comp.** La IR lo registra como constructor organizativo. Si la axiomática se cierra en el futuro, la IR se actualizará.

7. **Formalización completa del objeto supervisable para nivel (b).** La IR acepta SupervisionResult con Supervisable etiquetado (CellTarget, ComposedTarget, SystemTarget), pero no cierra la definición formal del nivel (b) más allá de lo que el Doc II documenta.

---

## Referencias

[R1] Juan Antonio Lloret Egea. *Frontera normativa del lenguaje de computación del Sistema Vectorial SV — v0.* marzo 2026. https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/FRONTERA_NORMATIVA_LENGUAJE_SV_v0.md

[R2] Juan Antonio Lloret Egea. *Fundamentos algebraico-semánticos del Sistema Vectorial SV.* v1.0.0, Release 3. ITVIA, 2026. https://www.itvia.online/pub/fundamentos-algebraico-semanticos-del-sistema-vectorial-sv/release/3

[R3] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — I. Transmisión en serie por parámetro puente.* v1, Release 4. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv/release/4

[R4] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — II. Gramática general de composición.* v1.0, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--ii-gramatica-general-de-composicion/release/1

[R5] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — III. Horizonte de sucesos y reevaluación discreta.* v1, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--iii-horizonte-de-sucesos-y-reevaluacion-discreta/release/1

[R6] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — IV. Transducción al alfabeto ternario e interfaz paramétrica del sistema.* v1, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--iv-transduccion-al-alfabeto-ternario-e-interfaz-parametrica-del-sistema/release/1

[R7] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — V. Invariantes, agentes especializados y operador de consulta del sistema.* v2, Release 2. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--v-invariantes-agentes-especializados-y-operador-de-consulta-del-sistema/release/2

[R8] Juan Antonio Lloret Egea. *Álgebra de composición intercelular del marco SV — VI. Análisis discreto, representaciones y herramientas de secuencias del sistema.* v1, Release 1. ITVIA, 2026. https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv--vi-analisis-discreto-representaciones-y-herramientas-de-secuencias-del-sistema/release/1

---

*Especificación técnica del lenguaje de computación del Sistema Vectorial SV. ISSN 2695-6411.*  
*Juan Antonio Lloret Egea | ORCID 0000-0002-6634-3351 | CC BY-NC-ND 4.0*
