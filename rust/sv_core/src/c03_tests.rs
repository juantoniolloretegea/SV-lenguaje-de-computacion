use crate::frame::resolved::{
    FrameCandidate, ResolvedArchitecture, ResolvedCoupledState, ResolvedEvalResult,
    ResolvedGateResult, ResolvedSupervisionResult, ResolvedSupervisionTarget,
};
use crate::frame::{Frame, FrameClosureViolation};
use crate::Nat;

fn candidate_with_full_declared_chain() -> FrameCandidate {
    FrameCandidate::new(
        "F_C03",
        Nat::from_u64(7),
        ResolvedArchitecture::new(
            "A_C03",
            vec!["N1".into(), "N2".into(), "N3".into()],
        ),
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

fn assert_e308(error: FrameClosureViolation) {
    assert_eq!(error.diagnostic_code(), "E308");
}

#[test]
fn c03_accepts_a_coherent_declared_subset_without_exhaustivity() {
    let frame = Frame::from_candidate(candidate_with_full_declared_chain())
        .expect("C03 debe aceptar el cierre coherente declarado");

    assert_eq!(frame.architecture(), "A_C03");
    assert_eq!(frame.cell_states(), &["S1".to_string(), "S2".to_string()]);
    assert_eq!(frame.eval_results(), &["E1".to_string(), "E2".to_string()]);
    assert_eq!(frame.gate_results(), &["G1".to_string()]);
    assert_eq!(frame.supervision(), &["SUP1".to_string()]);
    assert!(frame.criticalities().is_empty());

    // N3 pertenece a la arquitectura, pero C03 no impone exhaustividad.
    assert!(!frame.cell_states().iter().any(|state| state == "S3"));
}

#[test]
fn c03_rejects_an_evaluation_whose_source_escapes_the_frame() {
    let candidate = FrameCandidate::new(
        "F_C03",
        Nat::from_u64(0),
        ResolvedArchitecture::new("A_C03", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
    .with_eval_results(vec![ResolvedEvalResult::new("E_EXT", "S_EXT")]);

    let error = Frame::from_candidate(candidate).expect_err("debe rechazar fuente externa");
    assert!(matches!(
        error,
        FrameClosureViolation::EvalSourceOutsideFrame { .. }
    ));
    assert_e308(error);
}

#[test]
fn c03_rejects_a_gate_or_supervision_that_escapes_the_causal_closure() {
    let gate_error = Frame::from_candidate(
        candidate_with_full_declared_chain().with_gate_results(vec![ResolvedGateResult::new(
            "G_EXT",
            vec!["E1".into(), "E_EXT".into()],
        )]),
    )
    .expect_err("debe rechazar entrada de compuerta externa");
    assert!(matches!(
        gate_error,
        FrameClosureViolation::GateInputOutsideFrame { .. }
    ));
    assert_e308(gate_error);

    let supervision_error = Frame::from_candidate(
        candidate_with_full_declared_chain().with_supervision(vec![ResolvedSupervisionResult::new(
            "SUP_EXT",
            "E1",
            ResolvedSupervisionTarget::system("A_EXT"),
        )]),
    )
    .expect_err("debe rechazar objetivo sistémico externo");
    assert!(matches!(
        supervision_error,
        FrameClosureViolation::SupervisionSystemTargetMismatch { .. }
    ));
    assert_e308(supervision_error);
}

#[test]
fn c03_rejects_criticalities_while_no_surface_producer_exists() {
    let error = Frame::from_candidate(
        candidate_with_full_declared_chain().with_criticalities(vec!["CR1".into()]),
    )
    .expect_err("debe rechazar criticidad no producible");

    assert!(matches!(
        error,
        FrameClosureViolation::CriticalitiesNotProducible { .. }
    ));
    assert_e308(error);
}
