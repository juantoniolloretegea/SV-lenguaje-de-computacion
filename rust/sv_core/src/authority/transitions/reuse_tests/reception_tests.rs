use super::*;
use crate::authority::transitions::reception::{
    PreparedGenesisReception, ReceptionError, MAX_RECEPTION_BYTES,
};
use crate::requirements::observed::{ObservedCheckError, PreparedExactCheck, MAX_CHECK_BYTES};

const ACT: &[u8] = b"ACTO SINTETICO: holder:permit; ensayo; ninguna facultad profesional";
const ISSUER: &str = "emisor:ensayo";
const VERSION: &str = "ensayo:1";
const CONTRACT: &[u8] = b"ensayo: igualdad exacta de bytes; sin significado profesional";

fn reception(plan: GenesisPlan) -> PreparedGenesisReception {
    PreparedGenesisReception::prepare(
        ExternalGenesisPremise::for_test(),
        plan,
        ACT,
        ISSUER,
        VERSION,
    )
    .unwrap()
}

fn observed_continuity() -> AuthorityContinuity {
    let mut plan = permission_plan();
    let q = requirement_ref("req:exact");
    plan.requirements.push(RequirementProposal::new(
        q.clone(),
        RequirementClass::Specific,
        form_ref("form:permit"),
        effect_family_ref("family:write"),
        context_ref("context:permit"),
        [verifier_family_ref("verifier-family:permit")],
        applicability_rule_ref("applicability:permit"),
    ));
    plan.applicabilities.push(ApplicabilityProposal::new(
        verifier_ref("verifier:exact"),
        verifier_family_ref("verifier-family:permit"),
        q,
        context_ref("context:permit"),
        applicability_rule_ref("applicability:permit"),
    ));
    let mut c = AuthorityContinuity::uninhabited();
    c.apply_genesis(&mut ExternalGenesisPremise::for_test(), plan)
        .unwrap();
    c
}

fn binding(
    c: &AuthorityContinuity,
) -> (
    &crate::requirements::RequirementDescriptor,
    &crate::requirements::VerifierApplicability,
) {
    let q = requirement_ref("req:exact");
    let d = c
        .requirement_set(
            &form_ref("form:permit"),
            &effect_family_ref("family:write"),
            &context_ref("context:permit"),
        )
        .unwrap()
        .requirement(&q)
        .unwrap();
    let a = c
        .verifier_applicability(
            &q,
            &verifier_ref("verifier:exact"),
            &context_ref("context:permit"),
        )
        .unwrap();
    (d, a)
}

#[test]
fn rg01_receives_fixed_act_and_actual_t0_plan() {
    let mut r = reception(permission_plan());
    let received = r.receive(ACT, ISSUER, VERSION).unwrap();
    assert_eq!(received.act(), ACT);
    assert_eq!(received.declared_issuer(), ISSUER);
    assert_eq!(received.version(), VERSION);
    assert!(!received.continuity().t0_available());
    assert_eq!(received.continuity().form_count(), 2);
    assert_eq!(received.continuity().authority_count(), 2);
    assert_eq!(received.continuity().requirement_set_count(), 1);
}

#[test]
fn rg02_rejects_changed_act_before_consuming_preparation() {
    let mut r = reception(permission_plan());
    assert!(matches!(
        r.receive(b"ACTO cambiado", ISSUER, VERSION),
        Err(ReceptionError::ActMismatch)
    ));
    assert!(r.receive(ACT, ISSUER, VERSION).is_ok());
}

#[test]
fn rg03_rejects_changed_declarations_and_preserves_original() {
    let mut r = reception(permission_plan());
    assert!(matches!(
        r.receive(ACT, "emisor:otro", VERSION),
        Err(ReceptionError::DeclarationMismatch)
    ));
    assert!(matches!(
        r.receive(ACT, ISSUER, "ensayo:2"),
        Err(ReceptionError::DeclarationMismatch)
    ));
    assert!(r.receive(ACT, ISSUER, VERSION).is_ok());
}

#[test]
fn rg04_does_not_produce_second_continuity() {
    let mut r = reception(permission_plan());
    assert!(r.receive(ACT, ISSUER, VERSION).is_ok());
    assert!(matches!(
        r.receive(ACT, ISSUER, VERSION),
        Err(ReceptionError::AlreadyAttempted)
    ));
}

#[test]
fn rg05_preserves_genesis_error_without_repair() {
    let mut r = reception(GenesisPlan::new([], []));
    assert!(matches!(
        r.receive(ACT, ISSUER, VERSION),
        Err(ReceptionError::Genesis(GenesisError::EmptyInitialForms))
    ));
    assert!(matches!(
        r.receive(ACT, ISSUER, VERSION),
        Err(ReceptionError::AlreadyAttempted)
    ));
}

#[test]
fn rg06_bounds_before_copy_and_admits_boundary() {
    let mut r = reception(permission_plan());
    assert!(matches!(
        r.receive(&[], ISSUER, VERSION),
        Err(ReceptionError::Empty)
    ));
    assert!(matches!(
        r.receive(&vec![b'x'; MAX_RECEPTION_BYTES + 1], ISSUER, VERSION),
        Err(ReceptionError::TooLarge)
    ));
    assert!(r.receive(ACT, ISSUER, VERSION).is_ok());
    assert!(matches!(
        PreparedGenesisReception::prepare(
            ExternalGenesisPremise::for_test(),
            permission_plan(),
            ACT,
            "",
            VERSION
        ),
        Err(ReceptionError::Empty)
    ));
    let boundary = vec![b'x'; MAX_RECEPTION_BYTES];
    let mut r = PreparedGenesisReception::prepare(
        ExternalGenesisPremise::for_test(),
        permission_plan(),
        &boundary,
        ISSUER,
        VERSION,
    )
    .unwrap();
    assert_eq!(
        r.receive(&boundary, ISSUER, VERSION).unwrap().act(),
        boundary
    );
}

#[test]
fn oc01_computes_result_and_keeps_exact_evidence() {
    let c = observed_continuity();
    let (d, a) = binding(&c);
    let p = PreparedExactCheck::prepare(&c, d, a, CONTRACT, b"original\0 1").unwrap();
    let result = p.run(Some(b"original\0 1")).unwrap();
    assert_eq!(result.result(), CheckResult::Accredited);
    assert_eq!(result.contract(), CONTRACT);
    assert_eq!(result.expected(), b"original\0 1");
    assert_eq!(result.observed(), Some(&b"original\0 1"[..]));
    assert!(result.belongs_to(&c));
}

#[test]
fn oc02_refutes_changed_bytes_without_normalization() {
    let c = observed_continuity();
    let (d, a) = binding(&c);
    let p = PreparedExactCheck::prepare(&c, d, a, CONTRACT, b"no: 1").unwrap();
    for v in [&b"no:1"[..], &b"si: 1"[..], &b"NO: 1"[..]] {
        let result = p.run(Some(v)).unwrap();
        assert_eq!(result.result(), CheckResult::Refuted);
        assert_eq!(result.observed(), Some(v));
    }
}

#[test]
fn oc03_missing_is_not_empty_or_tri_u() {
    let c = observed_continuity();
    let (d, a) = binding(&c);
    let p = PreparedExactCheck::prepare(&c, d, a, CONTRACT, b"").unwrap();
    assert_eq!(p.run(None).unwrap().result(), CheckResult::NotVerifiable);
    assert_eq!(p.run(None).unwrap().observed(), None);
    assert_eq!(p.run(Some(b"")).unwrap().result(), CheckResult::Accredited);
    assert_eq!(p.run(Some(b"")).unwrap().observed(), Some(&b""[..]));
}

#[test]
fn oc04_rejects_foreign_descriptor_with_equal_content() {
    let c = observed_continuity();
    let other = observed_continuity();
    let (d, a) = binding(&c);
    let (foreign, _) = binding(&other);
    assert_eq!(d, foreign);
    assert!(matches!(
        PreparedExactCheck::prepare(&c, foreign, a, CONTRACT, b"1"),
        Err(ObservedCheckError::ForeignBinding)
    ));
    assert!(PreparedExactCheck::prepare(&c, d, a, CONTRACT, b"1").is_ok());
}

#[test]
fn oc05_rejects_foreign_applicability_with_equal_content() {
    let c = observed_continuity();
    let other = observed_continuity();
    let (d, a) = binding(&c);
    let (_, foreign) = binding(&other);
    assert_eq!(a, foreign);
    assert!(matches!(
        PreparedExactCheck::prepare(&c, d, foreign, CONTRACT, b"1"),
        Err(ObservedCheckError::ForeignBinding)
    ));
}

#[test]
fn oc06_equality_cannot_accredit_core_authority() {
    let c = observed_continuity();
    let q = requirement_ref("req:permit:authority");
    let d = c
        .requirement_set(
            &form_ref("form:permit"),
            &effect_family_ref("family:write"),
            &context_ref("context:permit"),
        )
        .unwrap()
        .requirement(&q)
        .unwrap();
    let v = permission_verifier(&q, "primary");
    let a = c
        .verifier_applicability(&q, &v, &context_ref("context:permit"))
        .unwrap();
    assert!(matches!(
        PreparedExactCheck::prepare(&c, d, a, CONTRACT, b"acreditado"),
        Err(ObservedCheckError::CoreRequirement)
    ));
}

#[test]
fn oc07_bounds_contract_reference_and_observation() {
    let c = observed_continuity();
    let (d, a) = binding(&c);
    let large = vec![0; MAX_CHECK_BYTES + 1];
    assert!(matches!(
        PreparedExactCheck::prepare(&c, d, a, b"", b"1"),
        Err(ObservedCheckError::EmptyContract)
    ));
    assert!(matches!(
        PreparedExactCheck::prepare(&c, d, a, &large, b"1"),
        Err(ObservedCheckError::TooLarge)
    ));
    assert!(matches!(
        PreparedExactCheck::prepare(&c, d, a, CONTRACT, &large),
        Err(ObservedCheckError::TooLarge)
    ));
    let edge = vec![0; MAX_CHECK_BYTES];
    let p = PreparedExactCheck::prepare(&c, d, a, CONTRACT, &edge).unwrap();
    assert!(matches!(
        p.run(Some(&large)),
        Err(ObservedCheckError::TooLarge)
    ));
    assert_eq!(
        p.run(Some(&edge)).unwrap().result(),
        CheckResult::Accredited
    );
}

#[test]
fn oc08_result_does_not_claim_other_continuity() {
    let c = observed_continuity();
    let other = observed_continuity();
    let (d, a) = binding(&c);
    let p = PreparedExactCheck::prepare(&c, d, a, CONTRACT, b"1").unwrap();
    let result = p.run(Some(b"1")).unwrap();
    assert!(!result.belongs_to(&other));
    assert!(result.belongs_to(&c));
}

#[test]
fn rg07_debug_does_not_disclose_received_material() {
    let confidential = b"sentinel-personal-act-2591";
    let mut prepared = PreparedGenesisReception::prepare(
        ExternalGenesisPremise::for_test(),
        permission_plan(),
        confidential,
        "sentinel-issuer-2591",
        "sentinel-version-2591",
    )
    .unwrap();
    let before = format!("{prepared:?}");
    assert!(!before.contains("sentinel-issuer-2591"));
    assert!(!before.contains(&format!("{confidential:?}")));
    let received = prepared
        .receive(
            confidential,
            "sentinel-issuer-2591",
            "sentinel-version-2591",
        )
        .unwrap();
    let after = format!("{received:?}");
    assert!(!after.contains("sentinel-issuer-2591"));
    assert!(!after.contains(&format!("{confidential:?}")));
}

#[test]
fn oc09_debug_does_not_disclose_evidence() {
    let c = observed_continuity();
    let (d, a) = binding(&c);
    let sentinel = b"sentinel-evidence-2591";
    let prepared = PreparedExactCheck::prepare(&c, d, a, sentinel, sentinel).unwrap();
    let evidence = prepared.run(Some(sentinel)).unwrap();
    let encoded = format!("{sentinel:?}");
    assert!(!format!("{prepared:?}").contains(&encoded));
    assert!(!format!("{evidence:?}").contains(&encoded));
}

fn received_operation() -> (
    crate::authority::transitions::reception::ReceivedGenesis,
    EffectDescriptor,
    crate::TracedPermit,
) {
    let mut received = reception(permission_plan())
        .receive(ACT, ISSUER, VERSION)
        .unwrap();
    let effect = permission_effect(
        received.continuity(),
        &authority_ref("authority:permit"),
        "effect:permit:allowed",
    )
    .clone();
    let results = permission_results(received.continuity(), CheckResult::Accredited, false);
    let crate::TracedPermitDecision::Granted(permit) = received
        .decide(&form_ref("form:permit"), &effect, &results)
        .unwrap()
    else {
        panic!("el control sintético debe producir un permiso");
    };
    (received, effect, permit)
}

// ES: Recorrido real posterior a una admisión supuesta sólo por el banco.
// EN: Actual downstream path, with admission assumed only by the test fixture.
#[test]
fn rg08_receipt_keeps_act_with_decision_and_execution() {
    let (mut received, effect, permit) = received_operation();
    let decision = permit.decision_trace().clone();
    let commitment = received.mediate(permit, &effect).unwrap();
    let mut executor = TestExecutor::default();
    let confirmation = received.execute(commitment, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
    assert_eq!(confirmation.decision_trace(), &decision);
    assert_eq!(
        received
            .trace()
            .exercise_decision_ref(confirmation.exercise()),
        Some(&decision)
    );
    assert_eq!(received.trace().decision_trace_count(), 1);
    assert_eq!(received.trace().exercise_event_count(), 2);
    assert_eq!(received.act(), ACT);
    assert_eq!(received.declared_issuer(), ISSUER);
    assert_eq!(received.version(), VERSION);
}

#[test]
fn rg09_same_act_does_not_merge_receipts_or_consume_own_permit() {
    let (_, _, foreign) = received_operation();
    let (mut received, effect, own) = received_operation();
    assert!(matches!(
        received.mediate(foreign, &effect),
        Err(crate::TracedMediationError::ForeignContinuity)
    ));
    let commitment = received.mediate(own, &effect).unwrap();
    let mut executor = TestExecutor::default();
    received.execute(commitment, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
}

#[test]
fn rg10_foreign_commitment_never_reaches_adapter() {
    let (mut other, effect, permit) = received_operation();
    let foreign = other.mediate(permit, &effect).unwrap();
    drop(other);
    let (mut received, effect, permit) = received_operation();
    let own = received.mediate(permit, &effect).unwrap();
    let mut executor = TestExecutor::default();
    assert!(matches!(
        received.execute(foreign, &mut executor),
        Err(crate::TracedExecutionError::ForeignContinuity)
    ));
    assert_eq!(executor.calls, 0);
    assert_eq!(received.trace().exercise_event_count(), 0);
    received.execute(own, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
}

#[test]
fn rg11_uncertain_effect_keeps_cause_and_cannot_be_retried() {
    let (mut received, effect, permit) = received_operation();
    let first_decision = permit.decision_trace().clone();
    let commitment = received.mediate(permit, &effect).unwrap();
    let mut failing = TestExecutor {
        fail: true,
        ..Default::default()
    };
    let Err(crate::TracedExecutionError::Execution {
        decision_trace,
        error: ExecutionError::AdapterIndeterminate { exercise, error },
    }) = received.execute(commitment, &mut failing)
    else {
        panic!("el fallo posterior al despacho debe quedar indeterminado");
    };
    assert_eq!(error, crate::TracedAdapterError::Adapter("adapter-error"));
    assert_eq!(decision_trace, first_decision);
    assert_eq!(
        received.trace().exercise_state(&exercise),
        Some(ExerciseAttemptState::Indeterminate)
    );
    let before = received
        .trace()
        .exercise_events()
        .cloned()
        .collect::<Vec<_>>();
    let results = permission_results(received.continuity(), CheckResult::Accredited, false);
    let crate::TracedPermitDecision::Granted(permit) = received
        .decide(&form_ref("form:permit"), &effect, &results)
        .unwrap()
    else {
        panic!("la segunda decisión debe alcanzar la barrera de consumo");
    };
    let second_decision = permit.decision_trace().clone();
    let retry = received.mediate(permit, &effect).unwrap();
    assert_eq!(
        received.execute(retry, &mut failing),
        Err(crate::TracedExecutionError::Execution {
            decision_trace: second_decision,
            error: ExecutionError::SingleUseAlreadyDispatched(exercise.clone()),
        })
    );
    assert_eq!(failing.calls, 1);
    assert_eq!(
        received
            .trace()
            .exercise_events()
            .cloned()
            .collect::<Vec<_>>(),
        before
    );
    assert_eq!(
        received.trace().exercise_decision_ref(&exercise),
        Some(&first_decision)
    );
    assert_eq!(received.act(), ACT);
}

#[test]
fn rg12_received_act_does_not_replace_required_checks() {
    let mut received = reception(permission_plan())
        .receive(ACT, ISSUER, VERSION)
        .unwrap();
    let effect = permission_effect(
        received.continuity(),
        &authority_ref("authority:permit"),
        "effect:permit:allowed",
    )
    .clone();
    let result = received.decide(&form_ref("form:permit"), &effect, &[]);
    assert!(!matches!(
        result,
        Ok(crate::TracedPermitDecision::Granted(_))
    ));
    assert_eq!(received.trace().exercise_event_count(), 0);
}

#[test]
fn rg13_consumed_premise_is_not_reconstituted() {
    let mut premise = ExternalGenesisPremise::for_test();
    AuthorityContinuity::uninhabited()
        .apply_genesis(&mut premise, permission_plan())
        .unwrap();
    assert!(premise.is_consumed());
    let mut prepared =
        PreparedGenesisReception::prepare(premise, permission_plan(), ACT, ISSUER, VERSION)
            .unwrap();
    assert!(matches!(
        prepared.receive(ACT, ISSUER, VERSION),
        Err(ReceptionError::Genesis(
            GenesisError::PremiseAlreadyConsumed
        ))
    ));
    assert!(matches!(
        prepared.receive(ACT, ISSUER, VERSION),
        Err(ReceptionError::AlreadyAttempted)
    ));
}

#[test]
fn rg14_caller_cannot_replace_prepared_bytes_through_buffers() {
    let mut act = ACT.to_vec();
    let mut issuer = ISSUER.to_string();
    let mut version = VERSION.to_string();
    let mut prepared = PreparedGenesisReception::prepare(
        ExternalGenesisPremise::for_test(),
        permission_plan(),
        &act,
        &issuer,
        &version,
    )
    .unwrap();
    act.fill(b'X');
    issuer.clear();
    version.clear();
    let received = prepared.receive(ACT, ISSUER, VERSION).unwrap();
    assert_eq!(received.act(), ACT);
    assert_eq!(received.declared_issuer(), ISSUER);
    assert_eq!(received.version(), VERSION);
}

#[test]
fn rg15_each_received_field_has_its_own_bound() {
    let large = "x".repeat(MAX_RECEPTION_BYTES + 1);
    for (act, issuer, version) in [
        (large.as_bytes(), ISSUER, VERSION),
        (ACT, large.as_str(), VERSION),
        (ACT, ISSUER, large.as_str()),
    ] {
        assert!(matches!(
            PreparedGenesisReception::prepare(
                ExternalGenesisPremise::for_test(),
                permission_plan(),
                act,
                issuer,
                version
            ),
            Err(ReceptionError::TooLarge)
        ));
        let mut prepared = reception(permission_plan());
        assert!(matches!(
            prepared.receive(act, issuer, version),
            Err(ReceptionError::TooLarge)
        ));
        assert!(prepared.receive(ACT, ISSUER, VERSION).is_ok());
    }
}

#[test]
fn oc10_later_input_mutation_cannot_change_retained_evidence() {
    let c = observed_continuity();
    let (d, a) = binding(&c);
    let mut contract = CONTRACT.to_vec();
    let mut expected = b"dato original".to_vec();
    let prepared = PreparedExactCheck::prepare(&c, d, a, &contract, &expected).unwrap();
    contract.clear();
    expected.fill(b'X');
    let mut input = b"dato original".to_vec();
    let evidence = prepared.run(Some(&input)).unwrap();
    input.fill(b'Y');
    assert_eq!(evidence.result(), CheckResult::Accredited);
    assert_eq!(evidence.contract(), CONTRACT);
    assert_eq!(evidence.expected(), b"dato original");
    assert_eq!(evidence.observed(), Some(&b"dato original"[..]));
    assert_eq!(
        prepared.run(Some(&input)).unwrap().result(),
        CheckResult::Refuted
    );
}
