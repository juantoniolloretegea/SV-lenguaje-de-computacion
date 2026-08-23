use std::collections::BTreeSet;

/// Código diagnóstico canónico para violaciones del cierre de `Frame`.
pub const FRAME_CLOSURE_DIAGNOSTIC_CODE: &str = "E308";

/// Proyección resuelta mínima de la arquitectura necesaria para validar un `Frame`.
///
/// `nodes` contiene identidades de `CoupledSpec`, que son las identidades de nodo
/// relevantes para J-F0 y J-F1. El `CellSpec` subyacente no participa en esa clave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedArchitecture {
    name: String,
    nodes: Vec<String>,
}

impl ResolvedArchitecture {
    pub fn new(name: impl Into<String>, nodes: Vec<String>) -> Self {
        Self {
            name: name.into(),
            nodes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nodes(&self) -> &[String] {
        &self.nodes
    }
}

/// Proyección resuelta mínima de un `CoupledState` incluido en un `Frame`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCoupledState {
    name: String,
    coupled_spec: String,
}

impl ResolvedCoupledState {
    pub fn new(name: impl Into<String>, coupled_spec: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            coupled_spec: coupled_spec.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn coupled_spec(&self) -> &str {
        &self.coupled_spec
    }
}

/// Proyección resuelta mínima de un `EvalResult` incluido en un `Frame`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedEvalResult {
    name: String,
    source_state: String,
}

impl ResolvedEvalResult {
    pub fn new(name: impl Into<String>, source_state: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_state: source_state.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_state(&self) -> &str {
        &self.source_state
    }
}

/// Proyección resuelta mínima de un `GateResult` incluido en un `Frame`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedGateResult {
    name: String,
    eval_inputs: Vec<String>,
}

impl ResolvedGateResult {
    pub fn new(name: impl Into<String>, eval_inputs: Vec<String>) -> Self {
        Self {
            name: name.into(),
            eval_inputs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn eval_inputs(&self) -> &[String] {
        &self.eval_inputs
    }
}

/// Objetivo de supervisión ya resuelto para la comprobación de cierre de `Frame`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedSupervisionTarget {
    Cell(String),
    Composed(String),
    System(String),
}

impl ResolvedSupervisionTarget {
    pub fn cell(eval_result: impl Into<String>) -> Self {
        Self::Cell(eval_result.into())
    }

    pub fn composed(gate_result: impl Into<String>) -> Self {
        Self::Composed(gate_result.into())
    }

    pub fn system(architecture: impl Into<String>) -> Self {
        Self::System(architecture.into())
    }
}

/// Proyección resuelta mínima de un `SupervisionResult` incluido en un `Frame`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSupervisionResult {
    name: String,
    meta_eval: String,
    target: ResolvedSupervisionTarget,
}

impl ResolvedSupervisionResult {
    pub fn new(
        name: impl Into<String>,
        meta_eval: impl Into<String>,
        target: ResolvedSupervisionTarget,
    ) -> Self {
        Self {
            name: name.into(),
            meta_eval: meta_eval.into(),
            target,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn meta_eval(&self) -> &str {
        &self.meta_eval
    }

    pub fn target(&self) -> &ResolvedSupervisionTarget {
        &self.target
    }
}

/// Candidato ya resuelto que aporta a `Frame` únicamente las relaciones necesarias
/// para decidir J-F0…J-F5. No sustituye al AST ni a la IR canónica.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameCandidate {
    name: String,
    index: u64,
    architecture: ResolvedArchitecture,
    cell_states: Vec<ResolvedCoupledState>,
    eval_results: Vec<ResolvedEvalResult>,
    gate_results: Vec<ResolvedGateResult>,
    supervision: Vec<ResolvedSupervisionResult>,
    criticalities: Vec<String>,
}

impl FrameCandidate {
    pub fn new(name: impl Into<String>, index: u64, architecture: ResolvedArchitecture) -> Self {
        Self {
            name: name.into(),
            index,
            architecture,
            cell_states: Vec::new(),
            eval_results: Vec::new(),
            gate_results: Vec::new(),
            supervision: Vec::new(),
            criticalities: Vec::new(),
        }
    }

    pub fn with_cell_states(mut self, cell_states: Vec<ResolvedCoupledState>) -> Self {
        self.cell_states = cell_states;
        self
    }

    pub fn with_eval_results(mut self, eval_results: Vec<ResolvedEvalResult>) -> Self {
        self.eval_results = eval_results;
        self
    }

    pub fn with_gate_results(mut self, gate_results: Vec<ResolvedGateResult>) -> Self {
        self.gate_results = gate_results;
        self
    }

    pub fn with_supervision(mut self, supervision: Vec<ResolvedSupervisionResult>) -> Self {
        self.supervision = supervision;
        self
    }

    pub fn with_criticalities(mut self, criticalities: Vec<String>) -> Self {
        self.criticalities = criticalities;
        self
    }
}

/// `Frame` constituido tras comprobar su cierre estructural y causal.
///
/// Conserva las colecciones declaradas como referencias, en su orden de entrada.
/// No impone exhaustividad y no expone mutadores posteriores a la constitución.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    name: String,
    index: u64,
    architecture: String,
    cell_states: Vec<String>,
    eval_results: Vec<String>,
    gate_results: Vec<String>,
    supervision: Vec<String>,
    criticalities: Vec<String>,
}

impl Frame {
    pub fn from_candidate(candidate: FrameCandidate) -> Result<Self, FrameClosureViolation> {
        let FrameCandidate {
            name,
            index,
            architecture,
            cell_states,
            eval_results,
            gate_results,
            supervision,
            criticalities,
        } = candidate;

        if !criticalities.is_empty() {
            return Err(FrameClosureViolation::CriticalitiesNotProducible {
                declared: criticalities,
            });
        }

        let architecture_nodes: BTreeSet<String> = architecture.nodes.iter().cloned().collect();

        let mut seen_state_refs = BTreeSet::new();
        let mut seen_nodes = BTreeSet::new();
        for state in &cell_states {
            if !seen_state_refs.insert(state.name.clone()) {
                return Err(FrameClosureViolation::DuplicateStateReference {
                    state: state.name.clone(),
                });
            }
            if !architecture_nodes.contains(&state.coupled_spec) {
                return Err(FrameClosureViolation::StateOutsideArchitecture {
                    state: state.name.clone(),
                    node: state.coupled_spec.clone(),
                    architecture: architecture.name.clone(),
                });
            }
            if !seen_nodes.insert(state.coupled_spec.clone()) {
                return Err(FrameClosureViolation::MultipleStatesForNode {
                    node: state.coupled_spec.clone(),
                });
            }
        }

        let frame_states: BTreeSet<String> = cell_states.iter().map(|state| state.name.clone()).collect();
        let mut seen_eval_sources = BTreeSet::new();
        for eval in &eval_results {
            if !frame_states.contains(&eval.source_state) {
                return Err(FrameClosureViolation::EvalSourceOutsideFrame {
                    eval: eval.name.clone(),
                    source_state: eval.source_state.clone(),
                });
            }
            if !seen_eval_sources.insert(eval.source_state.clone()) {
                return Err(FrameClosureViolation::DuplicateEvalSource {
                    source_state: eval.source_state.clone(),
                });
            }
        }

        let frame_evals: BTreeSet<String> = eval_results.iter().map(|eval| eval.name.clone()).collect();
        for gate in &gate_results {
            for input in &gate.eval_inputs {
                if !frame_evals.contains(input) {
                    return Err(FrameClosureViolation::GateInputOutsideFrame {
                        gate: gate.name.clone(),
                        eval_input: input.clone(),
                    });
                }
            }
        }

        let frame_gates: BTreeSet<String> = gate_results.iter().map(|gate| gate.name.clone()).collect();
        for supervision_result in &supervision {
            if !frame_evals.contains(&supervision_result.meta_eval) {
                return Err(FrameClosureViolation::SupervisionMetaEvalOutsideFrame {
                    supervision: supervision_result.name.clone(),
                    meta_eval: supervision_result.meta_eval.clone(),
                });
            }

            match &supervision_result.target {
                ResolvedSupervisionTarget::Cell(eval) => {
                    if !frame_evals.contains(eval) {
                        return Err(FrameClosureViolation::SupervisionCellTargetOutsideFrame {
                            supervision: supervision_result.name.clone(),
                            target: eval.clone(),
                        });
                    }
                }
                ResolvedSupervisionTarget::Composed(gate) => {
                    if !frame_gates.contains(gate) {
                        return Err(FrameClosureViolation::SupervisionComposedTargetOutsideFrame {
                            supervision: supervision_result.name.clone(),
                            target: gate.clone(),
                        });
                    }
                }
                ResolvedSupervisionTarget::System(system) => {
                    if system != &architecture.name {
                        return Err(FrameClosureViolation::SupervisionSystemTargetMismatch {
                            supervision: supervision_result.name.clone(),
                            target: system.clone(),
                            architecture: architecture.name.clone(),
                        });
                    }
                }
            }
        }

        Ok(Self {
            name,
            index,
            architecture: architecture.name,
            cell_states: cell_states.into_iter().map(|state| state.name).collect(),
            eval_results: eval_results.into_iter().map(|eval| eval.name).collect(),
            gate_results: gate_results.into_iter().map(|gate| gate.name).collect(),
            supervision: supervision.into_iter().map(|result| result.name).collect(),
            criticalities,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn architecture(&self) -> &str {
        &self.architecture
    }

    pub fn cell_states(&self) -> &[String] {
        &self.cell_states
    }

    pub fn eval_results(&self) -> &[String] {
        &self.eval_results
    }

    pub fn gate_results(&self) -> &[String] {
        &self.gate_results
    }

    pub fn supervision(&self) -> &[String] {
        &self.supervision
    }

    pub fn criticalities(&self) -> &[String] {
        &self.criticalities
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameClosureViolation {
    CriticalitiesNotProducible {
        declared: Vec<String>,
    },
    DuplicateStateReference {
        state: String,
    },
    StateOutsideArchitecture {
        state: String,
        node: String,
        architecture: String,
    },
    MultipleStatesForNode {
        node: String,
    },
    EvalSourceOutsideFrame {
        eval: String,
        source_state: String,
    },
    DuplicateEvalSource {
        source_state: String,
    },
    GateInputOutsideFrame {
        gate: String,
        eval_input: String,
    },
    SupervisionMetaEvalOutsideFrame {
        supervision: String,
        meta_eval: String,
    },
    SupervisionCellTargetOutsideFrame {
        supervision: String,
        target: String,
    },
    SupervisionComposedTargetOutsideFrame {
        supervision: String,
        target: String,
    },
    SupervisionSystemTargetMismatch {
        supervision: String,
        target: String,
        architecture: String,
    },
}

impl FrameClosureViolation {
    pub const fn diagnostic_code(&self) -> &'static str {
        FRAME_CLOSURE_DIAGNOSTIC_CODE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_candidate() -> FrameCandidate {
        FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into(), "N2".into()]),
        )
        .with_cell_states(vec![
            ResolvedCoupledState::new("S1", "N1"),
            ResolvedCoupledState::new("S2", "N2"),
        ])
        .with_eval_results(vec![
            ResolvedEvalResult::new("E1", "S1"),
            ResolvedEvalResult::new("E2", "S2"),
        ])
        .with_gate_results(vec![ResolvedGateResult::new(
            "G1",
            vec!["E1".into(), "E2".into()],
        )])
        .with_supervision(vec![ResolvedSupervisionResult::new(
            "SUP1",
            "E2",
            ResolvedSupervisionTarget::composed("G1"),
        )])
    }

    #[test]
    fn valid_frame_preserves_declared_references() {
        let frame = Frame::from_candidate(complete_candidate()).expect("Frame válido");

        assert_eq!(frame.name(), "F0");
        assert_eq!(frame.index(), 0);
        assert_eq!(frame.architecture(), "A0");
        assert_eq!(frame.cell_states(), &["S1".to_string(), "S2".to_string()]);
        assert_eq!(frame.eval_results(), &["E1".to_string(), "E2".to_string()]);
        assert_eq!(frame.gate_results(), &["G1".to_string()]);
        assert_eq!(frame.supervision(), &["SUP1".to_string()]);
        assert!(frame.criticalities().is_empty());
    }

    #[test]
    fn frame_is_not_exhaustive() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into(), "N2".into(), "N3".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")]);

        assert!(Frame::from_candidate(candidate).is_ok());
    }

    #[test]
    fn distinct_architecture_nodes_accept_distinct_states() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into(), "N2".into()]),
        )
        .with_cell_states(vec![
            ResolvedCoupledState::new("S1", "N1"),
            ResolvedCoupledState::new("S2", "N2"),
        ]);

        assert!(Frame::from_candidate(candidate).is_ok());
    }

    #[test]
    fn duplicate_state_reference_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![
            ResolvedCoupledState::new("S1", "N1"),
            ResolvedCoupledState::new("S1", "N1"),
        ]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::DuplicateStateReference {
                state: "S1".into()
            })
        );
    }

    #[test]
    fn state_outside_architecture_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S2", "N2")]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::StateOutsideArchitecture {
                state: "S2".into(),
                node: "N2".into(),
                architecture: "A0".into(),
            })
        );
    }

    #[test]
    fn multiple_states_for_same_node_are_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![
            ResolvedCoupledState::new("S1", "N1"),
            ResolvedCoupledState::new("S2", "N1"),
        ]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::MultipleStatesForNode { node: "N1".into() })
        );
    }

    #[test]
    fn eval_source_outside_frame_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![ResolvedEvalResult::new("E1", "S2")]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::EvalSourceOutsideFrame {
                eval: "E1".into(),
                source_state: "S2".into(),
            })
        );
    }

    #[test]
    fn duplicate_eval_source_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![
            ResolvedEvalResult::new("E1", "S1"),
            ResolvedEvalResult::new("E2", "S1"),
        ]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::DuplicateEvalSource {
                source_state: "S1".into()
            })
        );
    }

    #[test]
    fn gate_input_outside_frame_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
        .with_gate_results(vec![ResolvedGateResult::new(
            "G1",
            vec!["E1".into(), "E2".into()],
        )]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::GateInputOutsideFrame {
                gate: "G1".into(),
                eval_input: "E2".into(),
            })
        );
    }

    #[test]
    fn supervision_meta_eval_outside_frame_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
        .with_supervision(vec![ResolvedSupervisionResult::new(
            "SUP1",
            "E2",
            ResolvedSupervisionTarget::cell("E1"),
        )]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::SupervisionMetaEvalOutsideFrame {
                supervision: "SUP1".into(),
                meta_eval: "E2".into(),
            })
        );
    }

    #[test]
    fn supervision_cell_target_outside_frame_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
        .with_supervision(vec![ResolvedSupervisionResult::new(
            "SUP1",
            "E1",
            ResolvedSupervisionTarget::cell("E2"),
        )]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::SupervisionCellTargetOutsideFrame {
                supervision: "SUP1".into(),
                target: "E2".into(),
            })
        );
    }

    #[test]
    fn supervision_composed_target_outside_frame_is_rejected() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
        .with_supervision(vec![ResolvedSupervisionResult::new(
            "SUP1",
            "E1",
            ResolvedSupervisionTarget::composed("G1"),
        )]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::SupervisionComposedTargetOutsideFrame {
                supervision: "SUP1".into(),
                target: "G1".into(),
            })
        );
    }

    #[test]
    fn supervision_system_target_must_match_architecture() {
        let candidate = FrameCandidate::new(
            "F0",
            0,
            ResolvedArchitecture::new("A0", vec!["N1".into()]),
        )
        .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
        .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
        .with_supervision(vec![ResolvedSupervisionResult::new(
            "SUP1",
            "E1",
            ResolvedSupervisionTarget::system("A1"),
        )]);

        assert_eq!(
            Frame::from_candidate(candidate),
            Err(FrameClosureViolation::SupervisionSystemTargetMismatch {
                supervision: "SUP1".into(),
                target: "A1".into(),
                architecture: "A0".into(),
            })
        );
    }

    #[test]
    fn non_empty_criticalities_are_rejected() {
        let candidate = complete_candidate().with_criticalities(vec!["C1".into()]);

        let error = Frame::from_candidate(candidate).expect_err("debe rechazar criticidades");
        assert_eq!(error.diagnostic_code(), "E308");
        assert_eq!(
            error,
            FrameClosureViolation::CriticalitiesNotProducible {
                declared: vec!["C1".into()]
            }
        );
    }
}
