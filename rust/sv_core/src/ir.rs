use crate::{AdmissibilityState, Nat, Tri, GRAMMAR_VERSION, IR_VERSION, SERIALIZER_VERSION};

/// Nivel ontológico de un objeto de la IR canónica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrLevel {
    N0,
    N1,
    N2,
    N3,
    N4,
}

impl IrLevel {
    pub const fn label(self) -> &'static str {
        match self {
            Self::N0 => "N0",
            Self::N1 => "N1",
            Self::N2 => "N2",
            Self::N3 => "N3",
            Self::N4 => "N4",
        }
    }
}

/// Representación soberana de un programa IR 0.3.
///
/// Las versiones no son datos libres: derivan de las constantes canónicas del núcleo.
/// Las colecciones preservan el orden producido por la etapa frontal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrProgram {
    source_file: String,
    source_sha256: String,
    objects: Vec<IrObject>,
    operations: Vec<IrOperation>,
}

impl IrProgram {
    pub const fn grammar_version(&self) -> &'static str {
        GRAMMAR_VERSION
    }

    pub const fn ir_version(&self) -> &'static str {
        IR_VERSION
    }

    pub const fn serializer_version(&self) -> &'static str {
        SERIALIZER_VERSION
    }

    pub fn source_file(&self) -> &str {
        &self.source_file
    }

    pub fn source_sha256(&self) -> &str {
        &self.source_sha256
    }

    pub fn objects(&self) -> &[IrObject] {
        &self.objects
    }

    pub fn operations(&self) -> &[IrOperation] {
        &self.operations
    }
}

/// Objeto IR emitible por la etapa frontal 0.2 hacia IR 0.3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrObject {
    name: String,
    kind: IrObjectKind,
}

impl IrObject {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &IrObjectKind {
        &self.kind
    }

    pub const fn level(&self) -> IrLevel {
        self.kind.level()
    }

    pub const fn ir_type(&self) -> &'static str {
        self.kind.ir_type()
    }
}

/// Campos tipados y cerrados de los objetos que la etapa frontal vigente emite.
///
/// No existe una variante genérica de mapa de claves: el conjunto de tipos y campos
/// observables queda fijado por esta enumeración.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrObjectKind {
    Codomain {
        values: Vec<String>,
    },
    OutputSemantics {
        mappings: Vec<(String, String)>,
    },
    CellSpec {
        b: Nat,
        n: Nat,
        codomain: String,
        semantics: String,
        role: String,
    },
    CoupledSpec {
        cell: String,
        bridges: Vec<Nat>,
    },
    Connector {
        source_codomain: String,
        target_position: Nat,
        mapping: Vec<(String, Tri)>,
    },
    AdmissibilityTable {
        input_codomains: Vec<String>,
        output_codomain: String,
        table: Vec<(Vec<String>, String)>,
    },
    CaptureSpec {
        parameter_id: Nat,
        observation_domain: String,
        observation_space: String,
        failure_symbol: String,
        mapping: String,
    },
    AdmissibilitySpec {
        parameter_id: Nat,
        states: [AdmissibilityState; 3],
        rule: String,
    },
    Ternarizer {
        observation_space: String,
        partition_zero: String,
        partition_one: String,
        partition_u: String,
        mapping: String,
    },
    ResSpec {
        context: String,
        mechanism: String,
        mapping: String,
    },
    CellState {
        spec: String,
        vector: Vec<Tri>,
    },
    CoupledState {
        spec: String,
        base_vector: Vec<Tri>,
        updated_vector: Vec<Tri>,
    },
    CompositionGraph {
        nodes: Vec<String>,
        edges: Vec<(String, String, Nat, String)>,
        relation: String,
        regime: String,
    },
    SemanticRelation {
        kind: String,
        table: Option<String>,
        constraints: Option<Vec<String>>,
    },
    Pattern {
        kind: String,
        arity: Option<Nat>,
        constraints: Option<Vec<String>>,
    },
    Horizon {
        architecture: String,
        events: Vec<String>,
    },
    Frame {
        index: Nat,
        architecture: String,
        cell_states: Vec<String>,
        eval_results: Vec<String>,
        gate_results: Vec<String>,
        supervision: Vec<String>,
        criticalities: Vec<String>,
    },
    TransitionData {
        horizon_ref: String,
        events: Vec<(String, Tri)>,
        induced_parameters: Vec<(String, Nat, Tri)>,
        metadata: Option<Vec<String>>,
    },
    Trajectory {
        entries: Vec<(String, Option<String>)>,
    },
    Domain {
        parameters: Vec<String>,
        interface: String,
        horizon: String,
        capture_specs: Vec<String>,
        admissibility_specs: Vec<String>,
        ternarizers: Vec<String>,
        exogeneity_mask: String,
        silent_u: String,
        transduction_policy: String,
        u_policy: String,
        closure_criterion: String,
    },
    Agent {
        architecture: String,
        domain: String,
        query_engine: String,
    },
    QuerySpec {
        query_type: String,
        scope: String,
        restrictions: Vec<String>,
    },
}

impl IrObjectKind {
    pub const fn level(&self) -> IrLevel {
        match self {
            Self::Codomain { .. }
            | Self::OutputSemantics { .. }
            | Self::CellSpec { .. }
            | Self::CoupledSpec { .. }
            | Self::Connector { .. }
            | Self::AdmissibilityTable { .. }
            | Self::CaptureSpec { .. }
            | Self::AdmissibilitySpec { .. }
            | Self::Ternarizer { .. }
            | Self::ResSpec { .. }
            | Self::SemanticRelation { .. }
            | Self::Pattern { .. } => IrLevel::N0,
            Self::CellState { .. }
            | Self::CoupledState { .. }
            | Self::CompositionGraph { .. } => IrLevel::N1,
            Self::Horizon { .. }
            | Self::Frame { .. }
            | Self::TransitionData { .. }
            | Self::Trajectory { .. } => IrLevel::N3,
            Self::Domain { .. } | Self::Agent { .. } | Self::QuerySpec { .. } => IrLevel::N4,
        }
    }

    pub const fn ir_type(&self) -> &'static str {
        match self {
            Self::Codomain { .. } => "Codomain",
            Self::OutputSemantics { .. } => "OutputSemantics",
            Self::CellSpec { .. } => "CellSpec",
            Self::CoupledSpec { .. } => "CoupledSpec",
            Self::Connector { .. } => "Connector",
            Self::AdmissibilityTable { .. } => "AdmissibilityTable",
            Self::CaptureSpec { .. } => "CaptureSpec",
            Self::AdmissibilitySpec { .. } => "AdmissibilitySpec",
            Self::Ternarizer { .. } => "Ternarizer",
            Self::ResSpec { .. } => "ResSpec",
            Self::CellState { .. } => "CellState",
            Self::CoupledState { .. } => "CoupledState",
            Self::CompositionGraph { .. } => "CompositionGraph",
            Self::SemanticRelation { .. } => "SemanticRelation",
            Self::Pattern { .. } => "Pattern",
            Self::Horizon { .. } => "Horizon",
            Self::Frame { .. } => "Frame",
            Self::TransitionData { .. } => "TransitionData",
            Self::Trajectory { .. } => "Trajectory",
            Self::Domain { .. } => "Domain",
            Self::Agent { .. } => "Agent",
            Self::QuerySpec { .. } => "QuerySpec",
        }
    }

    /// `Frame.immutable` es constitutivamente verdadero en la IR vigente.
    pub const fn frame_is_immutable(&self) -> Option<bool> {
        match self {
            Self::Frame { .. } => Some(true),
            _ => None,
        }
    }

    /// `Trajectory.append_only` es constitutivamente verdadero en la IR vigente.
    pub const fn trajectory_is_append_only(&self) -> Option<bool> {
        match self {
            Self::Trajectory { .. } => Some(true),
            _ => None,
        }
    }
}

/// Operación IR emitible por la etapa frontal 0.2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrOperation {
    name: String,
    kind: IrOperationKind,
}

impl IrOperation {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &IrOperationKind {
        &self.kind
    }

    pub const fn op_type(&self) -> &'static str {
        self.kind.op_type()
    }

    pub const fn result_type(&self) -> &'static str {
        self.kind.result_type()
    }
}

/// Contexto cerrado de la operación `query`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrQueryContext {
    PointEval {
        reference: String,
    },
    TrajectoryView {
        reference: String,
    },
    FrameComparison {
        references: [String; 2],
    },
    ArchitectureView {
        architecture: String,
        cells: Vec<String>,
        evals: Vec<String>,
        gates: Vec<String>,
    },
    CoverageReport {
        references: [String; 3],
    },
}

impl IrQueryContext {
    pub const fn variant_label(&self) -> &'static str {
        match self {
            Self::PointEval { .. } => "PointEval",
            Self::TrajectoryView { .. } => "TrajectoryView",
            Self::FrameComparison { .. } => "FrameComparison",
            Self::ArchitectureView { .. } => "ArchitectureView",
            Self::CoverageReport { .. } => "CoverageReport",
        }
    }
}

/// Objetivo cerrado de la operación `supervise`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrSupervisableTarget {
    Cell {
        reference: String,
    },
    Composed {
        reference: String,
    },
    System {
        reference: String,
    },
}

impl IrSupervisableTarget {
    pub const fn variant_label(&self) -> &'static str {
        match self {
            Self::Cell { .. } => "CellTarget",
            Self::Composed { .. } => "ComposedTarget",
            Self::System { .. } => "SystemTarget",
        }
    }
}

/// Campos tipados y cerrados de las operaciones IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrOperationKind {
    Evaluate {
        state: String,
    },
    Gate {
        eval_results: Vec<String>,
        table: String,
    },
    Resolve {
        target_state: String,
        target_position: Nat,
        with_spec: String,
        context_instance: String,
        mechanism_instance: String,
    },
    Query {
        spec: String,
        by: String,
        context: IrQueryContext,
    },
    Supervise {
        meta_eval: String,
        target: IrSupervisableTarget,
    },
    Compose {
        graph: String,
        relations: Vec<String>,
        patterns: Vec<String>,
    },
    Projection {
        source: String,
        field: String,
    },
}

impl IrOperationKind {
    pub const fn op_type(&self) -> &'static str {
        match self {
            Self::Evaluate { .. } => "evaluate",
            Self::Gate { .. } => "gate",
            Self::Resolve { .. } => "resolve",
            Self::Query { .. } => "query",
            Self::Supervise { .. } => "supervise",
            Self::Compose { .. } => "compose",
            Self::Projection { .. } => "projection",
        }
    }

    pub const fn result_type(&self) -> &'static str {
        match self {
            Self::Evaluate { .. } => "EvalResult",
            Self::Gate { .. } => "GateResult",
            Self::Resolve { .. } => "ResolutionRecord",
            Self::Query { .. } => "QueryResult",
            Self::Supervise { .. } => "SupervisionResult",
            Self::Compose { .. } => "Architecture",
            Self::Projection { .. } => "Projected",
        }
    }
}

/// Frontera interna de constitución de la representación IR.
///
/// R0-6 materializa la representación antes de enlazar el descenso Rust completo.
/// Por ello, los adaptadores externos pueden inspeccionar un `IrProgram` ya
/// constituido, pero no fabricar uno ni declarar unilateralmente objetos u operaciones
/// como IR soberana.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "R0-6 materializa la representación IR antes de enlazar el descenso Rust posterior"
    )
)]
pub(crate) mod construction {
    use super::{IrObject, IrObjectKind, IrOperation, IrOperationKind, IrProgram};

    pub(crate) fn program(
        source_file: impl Into<String>,
        source_sha256: impl Into<String>,
        objects: Vec<IrObject>,
        operations: Vec<IrOperation>,
    ) -> IrProgram {
        IrProgram {
            source_file: source_file.into(),
            source_sha256: source_sha256.into(),
            objects,
            operations,
        }
    }

    pub(crate) fn object(name: impl Into<String>, kind: IrObjectKind) -> IrObject {
        IrObject {
            name: name.into(),
            kind,
        }
    }

    pub(crate) fn operation(name: impl Into<String>, kind: IrOperationKind) -> IrOperation {
        IrOperation {
            name: name.into(),
            kind,
        }
    }
}
