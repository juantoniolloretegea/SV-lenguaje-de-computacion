use sv_core::{
    Frame, FrameCandidate, ResolvedArchitecture, ResolvedCoupledState, ResolvedEvalResult,
    ResolvedSupervisionResult, ResolvedSupervisionTarget,
};

fn base_candidate() -> FrameCandidate {
    FrameCandidate::new(
        "F0",
        0,
        ResolvedArchitecture::new("A0", vec!["N1".into()]),
    )
    .with_cell_states(vec![ResolvedCoupledState::new("S1", "N1")])
    .with_eval_results(vec![ResolvedEvalResult::new("E1", "S1")])
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
