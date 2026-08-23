use crate::resolution::resolved::ResolvedTargetState;
use crate::resolution::{review_u, ResSpec, UnsafeUResolution, U_RESOLUTION_DIAGNOSTIC_CODE};
use crate::{Nat, Tri};

fn nat(value: &str) -> Nat {
    Nat::from_decimal(value).expect("natural válido")
}

fn spec() -> ResSpec {
    ResSpec::new("RS1", "ContextoA", "MecanismoA", "MapeoA")
}

#[test]
fn cell_target_u_is_reviewable_and_closure_remains_u() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::Zero, Tri::One, Tri::U, Tri::Zero]);
    let record = review_u(&state, nat("3"), &spec(), "ContextoA", "MecanismoA", Some(Tri::Zero))
        .expect("la U constituida debe ser revisable");

    assert_eq!(record.target().state(), "S1");
    assert_eq!(record.target().position().as_decimal(), "3");
    assert_eq!(record.previous(), Tri::U);
    assert_eq!(record.reviewed_to(), Some(Tri::Zero));
    assert_eq!(record.resolved_to(), Tri::U);
    assert_eq!(record.context_ref(), "ContextoA");
    assert_eq!(record.mechanism_ref(), "MecanismoA");
}

#[test]
fn coupled_target_uses_the_effective_updated_vector() {
    let state = ResolvedTargetState::coupled("CS1", vec![Tri::Zero, Tri::U, Tri::One]);
    let record = review_u(&state, nat("2"), &spec(), "ContextoA", "MecanismoA", Some(Tri::One))
        .expect("la U del vector efectivo debe ser revisable");

    assert_eq!(record.target().state(), "CS1");
    assert_eq!(record.previous(), Tri::U);
    assert_eq!(record.reviewed_to(), Some(Tri::One));
    assert_eq!(record.resolved_to(), Tri::U);
}

#[test]
fn non_evaluable_target_is_rejected_with_e305() {
    let state = ResolvedTargetState::other("F1", "Frame");
    let error = review_u(&state, nat("1"), &spec(), "ContextoA", "MecanismoA", None)
        .expect_err("un Frame no es un objetivo evaluable de resolve");

    assert_eq!(error.diagnostic_code(), U_RESOLUTION_DIAGNOSTIC_CODE);
    assert_eq!(error, UnsafeUResolution::TargetNotEvaluable {
        state: "F1".to_owned(),
        kind: "Frame".to_owned(),
    });
}

#[test]
fn zero_position_is_rejected_as_not_one_based() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::U]);
    let error = review_u(&state, nat("0"), &spec(), "ContextoA", "MecanismoA", None)
        .expect_err("la posición cero no es válida");

    assert_eq!(error.diagnostic_code(), "E305");
    assert!(matches!(error, UnsafeUResolution::PositionOutOfRange { .. }));
}

#[test]
fn position_beyond_effective_vector_is_rejected() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::U, Tri::Zero]);
    let error = review_u(&state, nat("3"), &spec(), "ContextoA", "MecanismoA", None)
        .expect_err("la posición fuera de rango debe rechazarse");

    assert_eq!(error, UnsafeUResolution::PositionOutOfRange {
        state: "S1".to_owned(),
        position: nat("3"),
        vector_len: 2,
    });
}

#[test]
fn arbitrary_precision_position_cannot_overflow_into_a_valid_index() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::U]);
    let huge = nat("184467440737095516160000000000000000000");
    let error = review_u(&state, huge.clone(), &spec(), "ContextoA", "MecanismoA", None)
        .expect_err("un natural enorme debe quedar fuera de rango sin estrechamiento");

    assert_eq!(error, UnsafeUResolution::PositionOutOfRange {
        state: "S1".to_owned(),
        position: huge,
        vector_len: 1,
    });
}

#[test]
fn target_value_must_be_a_constituted_u() {
    for found in [Tri::Zero, Tri::One] {
        let state = ResolvedTargetState::cell("S1", vec![found]);
        let error = review_u(&state, nat("1"), &spec(), "ContextoA", "MecanismoA", None)
            .expect_err("resolve no puede operar sobre 0 o 1");

        assert_eq!(error, UnsafeUResolution::TargetIsNotU {
            state: "S1".to_owned(),
            position: nat("1"),
            found,
        });
    }
}

#[test]
fn context_instance_must_match_res_spec_exactly() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::U]);
    let error = review_u(&state, nat("1"), &spec(), "ContextoB", "MecanismoA", None)
        .expect_err("el contexto incompatible debe rechazarse");

    assert_eq!(error, UnsafeUResolution::ContextMismatch {
        spec: "RS1".to_owned(),
        expected: "ContextoA".to_owned(),
        actual: "ContextoB".to_owned(),
    });
}

#[test]
fn mechanism_instance_must_match_res_spec_exactly() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::U]);
    let error = review_u(&state, nat("1"), &spec(), "ContextoA", "MecanismoB", None)
        .expect_err("el mecanismo incompatible debe rechazarse");

    assert_eq!(error, UnsafeUResolution::MechanismMismatch {
        spec: "RS1".to_owned(),
        expected: "MecanismoA".to_owned(),
        actual: "MecanismoB".to_owned(),
    });
}

#[test]
fn review_material_never_becomes_positive_closure_by_execution_alone() {
    let state = ResolvedTargetState::cell("S1", vec![Tri::U]);

    for reviewed_to in [None, Some(Tri::Zero), Some(Tri::One), Some(Tri::U)] {
        let record = review_u(&state, nat("1"), &spec(), "ContextoA", "MecanismoA", reviewed_to)
            .expect("la revisión es válida");
        assert_eq!(record.reviewed_to(), reviewed_to);
        assert_eq!(record.resolved_to(), Tri::U);
    }
}

#[test]
fn review_does_not_mutate_the_effective_state() {
    let before = vec![Tri::Zero, Tri::U, Tri::One];
    let state = ResolvedTargetState::cell("S1", before.clone());
    let _ = review_u(&state, nat("2"), &spec(), "ContextoA", "MecanismoA", Some(Tri::One))
        .expect("la revisión es válida");

    assert_eq!(state.effective_vector(), Some(before.as_slice()));
}

#[test]
fn res_spec_preserves_opaque_contract_references() {
    let rs = spec();
    assert_eq!(rs.name(), "RS1");
    assert_eq!(rs.context(), "ContextoA");
    assert_eq!(rs.mechanism(), "MecanismoA");
    assert_eq!(rs.mapping(), "MapeoA");
}
