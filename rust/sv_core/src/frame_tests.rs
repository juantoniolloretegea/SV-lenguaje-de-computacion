use crate::frame::resolved::{
    FrameCandidate, ResolvedArchitecture, ResolvedCoupledState, ResolvedEvalResult,
    ResolvedGateResult, ResolvedSupervisionResult, ResolvedSupervisionTarget,
};
use crate::frame::{Frame, FrameClosureViolation, FRAME_CLOSURE_DIAGNOSTIC_CODE};
use crate::Nat;

fn complete_candidate() -> FrameCandidate {
    FrameCandidate::new(
        "F0",
        Nat::from_decimal("184467440737095516160000000000000000000")
            .expect("índice natural válido"),
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

fn base_candidate() -> FrameCandidate {
    FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
    .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
}

fn assert_e308(error: &FrameClosureViolation) {
    assert_eq!(error.diagnostic_code(), FRAME_CLOSURE_DIAGNOSTIC_CODE);
    assert_eq!(error.diagnostic_code(), "E308");
}

#[test]
fn valid_frame_preserves_declared_references_and_unbounded_nat_index() {
    let frame = Frame::from_candidate(complete_candidate()).expect("Frame válido");

    assert_eq!(frame.name(), "F0");
    assert_eq!(
        frame.index().as_decimal(),
        "184467440737095516160000000000000000000"
    );
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
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into(), "N2".into(), "N3".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")]);

    assert!(Frame::from_candidate(candidate).is_ok());
}

#[test]
fn distinct_architecture_nodes_accept_distinct_states() {
    let candidate = FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
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
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![
        ResolvedCoupledState::new("S1", "N1"),
        ResolvedCoupledState::new("S1", "N1"),
    ]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar duplicado");
    assert_eq!(
        error,
        FrameClosureViolation::DuplicateStateReference {
            state: "S1".into()
        }
    );
    assert_e308(&error);
}

#[test]
fn state_outside_architecture_is_rejected() {
    let candidate = FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S2", "N2")]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar estado externo");
    assert!(matches!(
        error,
        FrameClosureViolation::StateOutsideArchitecture { .. }
    ));
    assert_e308(&error);
}

#[test]
fn multiple_states_for_same_node_are_rejected() {
    let candidate = FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![
        ResolvedCoupledState::new("S1", "N1"),
        ResolvedCoupledState::new("S2", "N1"),
    ]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar dos estados por nodo");
    assert_eq!(
        error,
        FrameClosureViolation::MultipleStatesForNode { node: "N1".into() }
    );
    assert_e308(&error);
}

#[test]
fn eval_source_outside_frame_is_rejected() {
    let candidate = FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
    .with_eval_results(vec![ResolvedEvalResult::new("E1", "S2")]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar evaluación externa");
    assert!(matches!(
        error,
        FrameClosureViolation::EvalSourceOutsideFrame { .. }
    ));
    assert_e308(&error);
}

#[test]
fn duplicate_eval_source_is_rejected() {
    let candidate = FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
    .with_eval_results(vec![
        ResolvedEvalResult::new("E1", "S1"),
        ResolvedEvalResult::new("E2", "S1"),
    ]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar fuente duplicada");
    assert_eq!(
        error,
        FrameClosureViolation::DuplicateEvalSource {
            source_state: "S1".into()
        }
    );
    assert_e308(&error);
}

#[test]
fn gate_input_outside_frame_is_rejected() {
    let candidate = FrameCandidate::new(
        "F0",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
    .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
    .with_gate_results(vec![ResolvedGateResult::new(
        "G1",
        vec!["E1".into(), "E2".into()],
    )]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar entrada externa");
    assert!(matches!(
        error,
        FrameClosureViolation::GateInputOutsideFrame { .. }
    ));
    assert_e308(&error);
}

#[test]
fn supervision_meta_eval_outside_frame_is_rejected() {
    let candidate = complete_candidate().with_supervision(vec![ResolvedSupervisionResult::new(
        "SUP1",
        "E3",
        ResolvedSupervisionTarget::composed("G1"),
    )]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar meta-evaluación externa");
    assert!(matches!(
        error,
        FrameClosureViolation::SupervisionMetaEvalOutsideFrame { .. }
    ));
    assert_e308(&error);
}

#[test]
fn supervision_cell_target_outside_frame_is_rejected() {
    let candidate = complete_candidate().with_supervision(vec![ResolvedSupervisionResult::new(
        "SUP1",
        "E1",
        ResolvedSupervisionTarget::cell("E3"),
    )]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar objetivo celular externo");
    assert!(matches!(
        error,
        FrameClosureViolation::SupervisionCellTargetOutsideFrame { .. }
    ));
    assert_e308(&error);
}

#[test]
fn supervision_composed_target_outside_frame_is_rejected() {
    let candidate = complete_candidate().with_supervision(vec![ResolvedSupervisionResult::new(
        "SUP1",
        "E1",
        ResolvedSupervisionTarget::composed("G2"),
    )]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar objetivo compuesto externo");
    assert!(matches!(
        error,
        FrameClosureViolation::SupervisionComposedTargetOutsideFrame { .. }
    ));
    assert_e308(&error);
}

#[test]
fn supervision_system_target_must_match_architecture() {
    let candidate = complete_candidate().with_supervision(vec![ResolvedSupervisionResult::new(
        "SUP1",
        "E1",
        ResolvedSupervisionTarget::system("A1"),
    )]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar arquitectura distinta");
    assert!(matches!(
        error,
        FrameClosureViolation::SupervisionSystemTargetMismatch { .. }
    ));
    assert_e308(&error);
}

#[test]
fn non_empty_criticalities_are_rejected() {
    let candidate = complete_candidate().with_criticalities(vec!["CR1".into()]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar criticidad no producible");
    assert_eq!(
        error,
        FrameClosureViolation::CriticalitiesNotProducible {
            declared: vec!["CR1".into()]
        }
    );
    assert_e308(&error);
}

#[test]
fn supervision_cell_target_inside_frame_is_valid() {
    let candidate = base_candidate().with_supervision(vec![ResolvedSupervisionResult::new(
        "SUP1",
        "E1",
        ResolvedSupervisionTarget::cell("E1"),
    )]);

    assert!(Frame::from_candidate(candidate).is_ok());
}

#[test]
fn supervision_system_target_matching_architecture_is_valid() {
    let candidate = base_candidate().with_supervision(vec![ResolvedSupervisionResult::new(
        "SUP1",
        "E1",
        ResolvedSupervisionTarget::system("A0"),
    )]);

    assert!(Frame::from_candidate(candidate).is_ok());
}
