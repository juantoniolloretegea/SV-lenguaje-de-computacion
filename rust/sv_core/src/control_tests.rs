use core::any::TypeId;

use crate::{
    AdmittedEvidenceRef, AuthorityRef, CheckResult, ConstitutedFactRef, ContinuityOccupancy,
    ControlId, EnablementRef, ExerciseRef, InformationRef, TransitionClass, Tri,
};

#[test]
fn control_id_rejects_empty_or_blank_identifiers() {
    assert!(ControlId::new("").is_err());
    assert!(ControlId::new("   \t\n").is_err());
    assert_eq!(ControlId::new("control:1").unwrap().as_str(), "control:1");
}

#[test]
fn transition_classes_preserve_the_eight_contractual_labels() {
    let cases = [
        (TransitionClass::Information, "T-I"),
        (TransitionClass::Verification, "T-V"),
        (TransitionClass::Enablement, "T-H"),
        (TransitionClass::Exercise, "T-E"),
        (TransitionClass::Governance, "T-G"),
        (TransitionClass::Constitutive, "T-C"),
        (TransitionClass::Genesis, "T-0"),
        (TransitionClass::Recovery, "T-R"),
    ];

    for (class, label) in cases {
        assert_eq!(class.label(), label);
    }
}

#[test]
fn check_results_are_exactly_the_three_sec0d_results() {
    assert_eq!(CheckResult::Accredited.label(), "D-A");
    assert_eq!(CheckResult::Refuted.label(), "D-R");
    assert_eq!(CheckResult::NotVerifiable.label(), "D-N");
}

#[test]
fn technical_check_results_are_a_type_distinct_from_tri() {
    assert_ne!(TypeId::of::<CheckResult>(), TypeId::of::<Tri>());
}

#[test]
fn continuity_occupancy_has_distinct_uninhabited_and_inhabited_states() {
    assert_ne!(
        ContinuityOccupancy::Uninhabited,
        ContinuityOccupancy::Inhabited
    );
}

#[test]
fn protected_reference_categories_are_nominally_distinct() {
    let information = InformationRef::from_core_id(ControlId::new("x").unwrap());
    let evidence = AdmittedEvidenceRef::from_core_id(ControlId::new("x").unwrap());
    let fact = ConstitutedFactRef::from_core_id(ControlId::new("x").unwrap());
    let authority = AuthorityRef::from_core_id(ControlId::new("x").unwrap());
    let enablement = EnablementRef::from_core_id(ControlId::new("x").unwrap());
    let exercise = ExerciseRef::from_core_id(ControlId::new("x").unwrap());

    assert_eq!(information.id().as_str(), "x");
    assert_eq!(evidence.id().as_str(), "x");
    assert_eq!(fact.id().as_str(), "x");
    assert_eq!(authority.id().as_str(), "x");
    assert_eq!(enablement.id().as_str(), "x");
    assert_eq!(exercise.id().as_str(), "x");

    assert_ne!(TypeId::of::<InformationRef>(), TypeId::of::<AdmittedEvidenceRef>());
    assert_ne!(TypeId::of::<AdmittedEvidenceRef>(), TypeId::of::<ConstitutedFactRef>());
    assert_ne!(TypeId::of::<ConstitutedFactRef>(), TypeId::of::<AuthorityRef>());
    assert_ne!(TypeId::of::<AuthorityRef>(), TypeId::of::<EnablementRef>());
    assert_ne!(TypeId::of::<EnablementRef>(), TypeId::of::<ExerciseRef>());
}
