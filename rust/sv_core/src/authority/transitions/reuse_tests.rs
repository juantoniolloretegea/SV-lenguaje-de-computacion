use super::*;
use crate::control::{
    AccumulationRuleRef, ApplicabilityRuleRef, CheckResult, ControlId, CoverageRuleRef,
    ExerciseRef, ReuseBindingKeyRef, ReuseBindingValueRef, ReuseRuleRef, VerifierFamilyRef,
    VerifierRef,
};
use crate::execution::{
    execute_mediated, EffectExecutor, ExecutionContinuity, ExecutionError, ExecutionRequest,
    ExerciseAttemptState,
};
use crate::mediation::{mediate_permit, MediatedEffectCommitment, MediationError};
use crate::permission::{decide_permit, PermitDecision, PermitDecisionError, PermitRejection};
use crate::requirements::initial::{
    ApplicabilityProposal, CoverageRuleProposal, InitialRequirementError, ReuseBindingProposal,
    ReuseRuleProposal,
};
use crate::requirements::{
    CoreRequirementKind, RequirementCheck, RequirementClass, RequirementSet,
};
use crate::requirements_bridge::{resolve_requirement_result, ResolvedRequirementResult};

fn id(value: &str) -> ControlId {
    ControlId::new(value).unwrap()
}

fn form_ref(value: &str) -> FormRef {
    FormRef::from_core_id(id(value))
}

fn effect_family_ref(value: &str) -> EffectFamilyRef {
    EffectFamilyRef::from_core_id(id(value))
}

fn effect_ref(value: &str) -> EffectRef {
    EffectRef::from_core_id(id(value))
}

fn context_ref(value: &str) -> ContextRef {
    ContextRef::from_core_id(id(value))
}

fn authority_ref(value: &str) -> AuthorityRef {
    AuthorityRef::from_core_id(id(value))
}

fn holder_ref(value: &str) -> AuthorityHolderRef {
    AuthorityHolderRef::from_core_id(id(value))
}

fn object_ref(value: &str) -> GovernedObjectRef {
    GovernedObjectRef::from_core_id(id(value))
}

fn requirement_ref(value: &str) -> RequirementRef {
    RequirementRef::from_core_id(id(value))
}

fn verifier_ref(value: &str) -> VerifierRef {
    VerifierRef::from_core_id(id(value))
}

fn verifier_family_ref(value: &str) -> VerifierFamilyRef {
    VerifierFamilyRef::from_core_id(id(value))
}

fn applicability_rule_ref(value: &str) -> ApplicabilityRuleRef {
    ApplicabilityRuleRef::from_core_id(id(value))
}

fn coverage_rule_ref(value: &str) -> CoverageRuleRef {
    CoverageRuleRef::from_core_id(id(value))
}

fn reuse_rule_ref(value: &str) -> ReuseRuleRef {
    ReuseRuleRef::from_core_id(id(value))
}

fn reuse_binding_key(value: &str) -> ReuseBindingKeyRef {
    ReuseBindingKeyRef::from_core_id(id(value))
}

fn reuse_binding_value(value: &str) -> ReuseBindingValueRef {
    ReuseBindingValueRef::from_core_id(id(value))
}

fn accumulation_rule_ref(value: &str) -> AccumulationRuleRef {
    AccumulationRuleRef::from_core_id(id(value))
}

fn mandatory_requirement_proposals(
    form: &str,
    family: &str,
    context: &str,
) -> Vec<RequirementProposal> {
    [
        ("form", CoreRequirementKind::FormValidity),
        ("authority", CoreRequirementKind::ApplicableAuthority),
        (
            "verifier",
            CoreRequirementKind::VerifierAdmissibilityAndApplicability,
        ),
        ("no-self", CoreRequirementKind::NoSelfAccreditation),
    ]
    .into_iter()
    .map(|(suffix, kind)| {
        RequirementProposal::new(
            requirement_ref(&format!("req:{form}:{suffix}")),
            RequirementClass::Core(kind),
            form_ref(form),
            effect_family_ref(family),
            context_ref(context),
            [verifier_family_ref("verifier-family:canonical")],
            applicability_rule_ref("applicability:canonical"),
        )
    })
    .collect()
}

fn controlled_plan(requirements: Vec<RequirementProposal>) -> GenesisPlan {
    let authority = authority_ref("authority:genesis");
    let context = "context:genesis";
    GenesisPlan::new(
        [FormProposal::new(
            form_ref("form:exercise"),
            TransitionClass::Exercise,
            effect_family_ref("family:write"),
            [context_ref(context)],
            Some(authority.clone()),
            AccumulationContract::SingleUse,
        )],
        [AuthorityProposal::new(
            authority,
            holder_ref("holder:root"),
            context_ref(context),
            [EffectProposal::new(
                effect_ref("effect:write-one"),
                effect_family_ref("family:write"),
                object_ref("object:one"),
                context_ref(context),
            )],
            [object_ref("object:one")],
        )],
    )
    .with_initial_control(requirements, [])
}

fn assert_rejected_genesis_is_atomic(
    continuity: &AuthorityContinuity,
    premise: &ExternalGenesisPremise,
) {
    assert_eq!(continuity.occupancy(), ContinuityOccupancy::Uninhabited);
    assert!(continuity.t0_available());
    assert!(!premise.is_consumed());
    assert_eq!(continuity.form_count(), 0);
    assert_eq!(continuity.authority_count(), 0);
    assert_eq!(continuity.requirement_set_count(), 0);
    assert_eq!(continuity.verifier_applicability_count(), 0);
}

#[test]
fn reuse_rule_is_constituted_by_t0_and_bound_to_requirement() {
    let form = "form:exercise";
    let context = "context:genesis";
    let requirement = requirement_ref("req:form:exercise:form");
    let rule_reference = reuse_rule_ref("reuse-rule:form");
    let key = reuse_binding_key("binding:regime");
    let value = reuse_binding_value("value:v1");
    let mut requirements = mandatory_requirement_proposals(form, "family:write", context);
    requirements[0] = requirements[0].clone().with_reuse_rule(ReuseRuleProposal::new(
        rule_reference.clone(),
        [ReuseBindingProposal::new(key.clone(), value.clone())],
    ));

    let mut continuity = AuthorityContinuity::uninhabited();
    let mut premise = ExternalGenesisPremise::for_test();
    continuity
        .apply_genesis(&mut premise, controlled_plan(requirements))
        .unwrap();

    let descriptor = continuity
        .requirement_set(
            &form_ref(form),
            &effect_family_ref("family:write"),
            &context_ref(context),
        )
        .and_then(|set| set.requirement(&requirement))
        .expect("T-0 debe constituir la obligación");
    let rule = descriptor
        .reuse_rule()
        .expect("T-0 debe ligar la regla de reutilización al descriptor");

    assert_eq!(rule.reference(), &rule_reference);
    assert_eq!(rule.requirement(), &requirement);
    assert_eq!(rule.bindings().collect::<Vec<_>>(), vec![(&key, &value)]);
    assert!(premise.is_consumed());
}

#[test]
fn empty_reuse_binding_set_rejects_t0_atomically() {
    let form = "form:exercise";
    let context = "context:genesis";
    let requirement = requirement_ref("req:form:exercise:form");
    let mut requirements = mandatory_requirement_proposals(form, "family:write", context);
    requirements[0] = requirements[0].clone().with_reuse_rule(ReuseRuleProposal::new(
        reuse_rule_ref("reuse-rule:empty"),
        [],
    ));

    let mut continuity = AuthorityContinuity::uninhabited();
    let mut premise = ExternalGenesisPremise::for_test();
    let result = continuity.apply_genesis(&mut premise, controlled_plan(requirements));

    assert_eq!(
        result,
        Err(GenesisError::InvalidInitialRequirements(
            InitialRequirementError::EmptyReuseBindingSet(requirement)
        ))
    );
    assert_rejected_genesis_is_atomic(&continuity, &premise);
}

#[test]
fn duplicate_reuse_binding_key_rejects_t0_atomically() {
    let form = "form:exercise";
    let context = "context:genesis";
    let requirement = requirement_ref("req:form:exercise:form");
    let key = reuse_binding_key("binding:regime");
    let mut requirements = mandatory_requirement_proposals(form, "family:write", context);
    requirements[0] = requirements[0].clone().with_reuse_rule(ReuseRuleProposal::new(
        reuse_rule_ref("reuse-rule:duplicate-key"),
        [
            ReuseBindingProposal::new(key.clone(), reuse_binding_value("value:v1")),
            ReuseBindingProposal::new(key.clone(), reuse_binding_value("value:v2")),
        ],
    ));

    let mut continuity = AuthorityContinuity::uninhabited();
    let mut premise = ExternalGenesisPremise::for_test();
    let result = continuity.apply_genesis(&mut premise, controlled_plan(requirements));

    assert_eq!(
        result,
        Err(GenesisError::InvalidInitialRequirements(
            InitialRequirementError::DuplicateReuseBindingKey { requirement, key }
        ))
    );
    assert_rejected_genesis_is_atomic(&continuity, &premise);
}

#[test]
fn duplicate_reuse_rule_reference_across_requirements_rejects_t0_atomically() {
    let form = "form:exercise";
    let context = "context:genesis";
    let shared = reuse_rule_ref("reuse-rule:shared");
    let mut requirements = mandatory_requirement_proposals(form, "family:write", context);
    requirements[0] = requirements[0].clone().with_reuse_rule(ReuseRuleProposal::new(
        shared.clone(),
        [ReuseBindingProposal::new(
            reuse_binding_key("binding:one"),
            reuse_binding_value("value:one"),
        )],
    ));
    requirements[1] = requirements[1].clone().with_reuse_rule(ReuseRuleProposal::new(
        shared.clone(),
        [ReuseBindingProposal::new(
            reuse_binding_key("binding:two"),
            reuse_binding_value("value:two"),
        )],
    ));

    let mut continuity = AuthorityContinuity::uninhabited();
    let mut premise = ExternalGenesisPremise::for_test();
    let result = continuity.apply_genesis(&mut premise, controlled_plan(requirements));

    assert_eq!(
        result,
        Err(GenesisError::InvalidInitialRequirements(
            InitialRequirementError::DuplicateReuseRuleRef(shared)
        ))
    );
    assert_rejected_genesis_is_atomic(&continuity, &premise);
}

fn permission_verifier(requirement: &RequirementRef, suffix: &str) -> VerifierRef {
    verifier_ref(&format!("verifier:{}:{suffix}", requirement.id().as_str()))
}

fn permission_plan() -> GenesisPlan {
    permission_plan_with(
        TransitionClass::Exercise,
        AccumulationContract::SingleUse,
    )
}

fn permission_plan_with(
    transition_class: TransitionClass,
    accumulation: AccumulationContract,
) -> GenesisPlan {
    let form = form_ref("form:permit");
    let family = effect_family_ref("family:write");
    let context = context_ref("context:permit");
    let authority = authority_ref("authority:permit");
    let verifier_family = verifier_family_ref("verifier-family:permit");
    let applicability_rule = applicability_rule_ref("applicability:permit");

    let mandatory = [
        ("req:permit:form", CoreRequirementKind::FormValidity),
        ("req:permit:authority", CoreRequirementKind::ApplicableAuthority),
        (
            "req:permit:verifier",
            CoreRequirementKind::VerifierAdmissibilityAndApplicability,
        ),
        ("req:permit:no-self", CoreRequirementKind::NoSelfAccreditation),
    ];

    let mut requirements = Vec::new();
    let mut applicabilities = Vec::new();
    for (reference, kind) in mandatory {
        let requirement = requirement_ref(reference);
        let primary = permission_verifier(&requirement, "primary");
        let alternate = permission_verifier(&requirement, "alternate");
        requirements.push(
            RequirementProposal::new(
                requirement.clone(),
                RequirementClass::Core(kind),
                form.clone(),
                family.clone(),
                context.clone(),
                [verifier_family.clone()],
                applicability_rule.clone(),
            )
            .with_coverage_rule(CoverageRuleProposal::new(
                coverage_rule_ref(&format!("coverage:{reference}")),
                [primary.clone()],
            ))
            .with_reuse_rule(ReuseRuleProposal::new(
                reuse_rule_ref(&format!("reuse:{reference}")),
                [ReuseBindingProposal::new(
                    reuse_binding_key("binding:permit:regime"),
                    reuse_binding_value("value:permit:v1"),
                )],
            )),
        );
        applicabilities.push(ApplicabilityProposal::new(
            primary,
            verifier_family.clone(),
            requirement.clone(),
            context.clone(),
            applicability_rule.clone(),
        ));
        applicabilities.push(ApplicabilityProposal::new(
            alternate,
            verifier_family.clone(),
            requirement,
            context.clone(),
            applicability_rule.clone(),
        ));
    }

    GenesisPlan::new(
        [
            FormProposal::new(
                form,
                transition_class,
                family.clone(),
                [context.clone()],
                Some(authority.clone()),
                accumulation,
            ),
            FormProposal::new(
                form_ref("form:permit:free"),
                TransitionClass::Information,
                family.clone(),
                [context.clone()],
                None,
                AccumulationContract::NotApplicable,
            ),
        ],
        [
            AuthorityProposal::new(
                authority,
                holder_ref("holder:permit"),
                context.clone(),
                [
                    EffectProposal::new(
                        effect_ref("effect:permit:allowed"),
                        family.clone(),
                        object_ref("object:permit"),
                        context.clone(),
                    ),
                    EffectProposal::new(
                        effect_ref("effect:permit:wrong-family"),
                        effect_family_ref("family:delete"),
                        object_ref("object:permit"),
                        context.clone(),
                    ),
                ],
                [object_ref("object:permit")],
            ),
            AuthorityProposal::new(
                authority_ref("authority:permit:other"),
                holder_ref("holder:permit:other"),
                context.clone(),
                [EffectProposal::new(
                    effect_ref("effect:permit:foreign"),
                    family,
                    object_ref("object:permit:foreign"),
                    context.clone(),
                )],
                [object_ref("object:permit:foreign")],
            ),
        ],
    )
    .with_initial_control(requirements, applicabilities)
}

fn permission_continuity() -> AuthorityContinuity {
    permission_continuity_with(
        TransitionClass::Exercise,
        AccumulationContract::SingleUse,
    )
}

fn permission_continuity_with(
    transition_class: TransitionClass,
    accumulation: AccumulationContract,
) -> AuthorityContinuity {
    let mut continuity = AuthorityContinuity::uninhabited();
    let mut premise = ExternalGenesisPremise::for_test();
    continuity
        .apply_genesis(
            &mut premise,
            permission_plan_with(transition_class, accumulation),
        )
        .unwrap();
    assert!(premise.is_consumed());
    continuity
}

fn permission_requirement_set(continuity: &AuthorityContinuity) -> &RequirementSet {
    continuity
        .requirement_set(
            &form_ref("form:permit"),
            &effect_family_ref("family:write"),
            &context_ref("context:permit"),
        )
        .unwrap()
}

fn permission_results(
    continuity: &AuthorityContinuity,
    first_result: CheckResult,
    use_alternate_for_first: bool,
) -> Vec<ResolvedRequirementResult> {
    let set = permission_requirement_set(continuity);
    set.iter()
        .enumerate()
        .map(|(index, descriptor)| {
            let suffix = if index == 0 && use_alternate_for_first {
                "alternate"
            } else {
                "primary"
            };
            let verifier = permission_verifier(descriptor.reference(), suffix);
            let applicability = continuity
                .verifier_applicability(descriptor.reference(), &verifier, descriptor.context())
                .unwrap();
            let result = if index == 0 {
                first_result
            } else {
                CheckResult::Accredited
            };
            let check = RequirementCheck::constitute_for_test(descriptor, applicability, result)
                .unwrap();
            resolve_requirement_result(descriptor, &[&check]).unwrap()
        })
        .collect()
}

fn permission_effect<'a>(
    continuity: &'a AuthorityContinuity,
    authority: &AuthorityRef,
    effect: &str,
) -> &'a EffectDescriptor {
    continuity
        .authority(authority)
        .unwrap()
        .max_effects()
        .iter()
        .find(|candidate| candidate.reference().id().as_str() == effect)
        .unwrap()
}

fn granted_permission<'a>(
    continuity: &'a AuthorityContinuity,
    effect: &'a EffectDescriptor,
) -> crate::permission::Permit {
    let results = permission_results(continuity, CheckResult::Accredited, false);
    let decision = decide_permit(continuity, &form_ref("form:permit"), effect, &results).unwrap();
    let PermitDecision::Granted(permit) = decision else {
        panic!("la decisión debía conceder el permiso de prueba");
    };
    permit
}

fn mediated_commitment(continuity: &AuthorityContinuity) -> MediatedEffectCommitment {
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(continuity, &authority, "effect:permit:allowed");
    let permit = granted_permission(continuity, effect);
    mediate_permit(continuity, permit, effect).unwrap()
}

#[derive(Default)]
struct TestExecutor {
    calls: usize,
    fail: bool,
    last_exercise: Option<ExerciseRef>,
    last_effect: Option<EffectRef>,
}

impl EffectExecutor for TestExecutor {
    type Error = &'static str;

    fn execute(&mut self, request: &ExecutionRequest<'_>) -> Result<(), Self::Error> {
        self.calls += 1;
        self.last_exercise = Some(request.exercise().clone());
        self.last_effect = Some(request.effect_reference().clone());
        if self.fail {
            Err("adapter-error")
        } else {
            Ok(())
        }
    }
}

#[test]
fn complete_governed_da_forms_a_sealed_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit");
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:allowed");
    let results = permission_results(&continuity, CheckResult::Accredited, false);

    let decision = decide_permit(&continuity, &form, effect, &results).unwrap();
    let PermitDecision::Granted(permit) = decision else {
        panic!("la decisión debía conceder un permiso sellado");
    };

    assert_eq!(permit.authority(), &authority);
    assert_eq!(permit.required_authority(), &authority);
    assert_eq!(permit.authority_holder(), &holder_ref("holder:permit"));
    assert_eq!(permit.authority_context(), &context_ref("context:permit"));
    assert_eq!(permit.form(), &form);
    assert_eq!(permit.transition_class(), TransitionClass::Exercise);
    assert_eq!(permit.form_effect_family(), &effect_family_ref("family:write"));
    assert_eq!(permit.effect_reference(), &effect_ref("effect:permit:allowed"));
    assert_eq!(permit.governed_object(), &object_ref("object:permit"));
    assert_eq!(permit.context(), &context_ref("context:permit"));
    assert_eq!(permit.requirement_form(), &form);
    assert_eq!(
        permit.requirement_effect_family(),
        &effect_family_ref("family:write")
    );
    assert_eq!(permit.requirement_context(), &context_ref("context:permit"));
    assert_eq!(permit.technical_result(), CheckResult::Accredited);
    assert_eq!(permit.accumulation(), &AccumulationContract::SingleUse);
}

#[test]
fn refuted_requirements_never_form_positive_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit");
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:allowed");
    let results = permission_results(&continuity, CheckResult::Refuted, false);

    assert_eq!(
        decide_permit(&continuity, &form, effect, &results),
        Ok(PermitDecision::NotGranted(
            PermitRejection::RefutedRequirements
        ))
    );
}

#[test]
fn not_verifiable_requirements_never_form_positive_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit");
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:allowed");
    let results = permission_results(&continuity, CheckResult::NotVerifiable, false);

    assert_eq!(
        decide_permit(&continuity, &form, effect, &results),
        Ok(PermitDecision::NotGranted(
            PermitRejection::NotVerifiableRequirements
        ))
    );
}

#[test]
fn accredited_but_incomplete_coverage_never_forms_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit");
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:allowed");
    let results = permission_results(&continuity, CheckResult::Accredited, true);

    assert_eq!(
        decide_permit(&continuity, &form, effect, &results),
        Ok(PermitDecision::NotGranted(
            PermitRejection::NotVerifiableRequirements
        ))
    );
}

#[test]
fn form_effect_mismatch_is_closed_before_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit");
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:wrong-family");
    let results = permission_results(&continuity, CheckResult::Accredited, false);

    assert_eq!(
        decide_permit(&continuity, &form, effect, &results),
        Err(PermitDecisionError::FormEffectMismatch {
            form,
            effect: effect_ref("effect:permit:wrong-family"),
        })
    );
}

#[test]
fn effect_outside_required_authority_scope_is_closed_before_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit");
    let main_authority = authority_ref("authority:permit");
    let other_authority = authority_ref("authority:permit:other");
    let effect = permission_effect(&continuity, &other_authority, "effect:permit:foreign");
    let results = permission_results(&continuity, CheckResult::Accredited, false);

    assert_eq!(
        decide_permit(&continuity, &form, effect, &results),
        Err(PermitDecisionError::EffectOutsideAuthorityScope {
            authority: main_authority,
            effect: effect_ref("effect:permit:foreign"),
        })
    );
}

#[test]
fn form_without_required_authority_cannot_form_permit() {
    let continuity = permission_continuity();
    let form = form_ref("form:permit:free");
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:allowed");

    assert_eq!(
        decide_permit(&continuity, &form, effect, &[]),
        Err(PermitDecisionError::FormWithoutRequiredAuthority(form))
    );
}

#[test]
fn valid_permit_is_consumed_into_exact_mediated_commitment() {
    let continuity = permission_continuity();
    let authority = authority_ref("authority:permit");
    let effect = permission_effect(&continuity, &authority, "effect:permit:allowed");
    let permit = granted_permission(&continuity, effect);

    let commitment = mediate_permit(&continuity, permit, effect).unwrap();

    assert_eq!(commitment.authority(), &authority);
    assert_eq!(commitment.authority_holder(), &holder_ref("holder:permit"));
    assert_eq!(commitment.form(), &form_ref("form:permit"));
    assert_eq!(commitment.transition_class(), TransitionClass::Exercise);
    assert_eq!(commitment.effect_reference(), effect.reference());
    assert_eq!(commitment.governed_object(), effect.object());
    assert_eq!(commitment.context(), effect.context());
    assert_eq!(commitment.technical_result(), CheckResult::Accredited);
    assert_eq!(commitment.accumulation(), &AccumulationContract::SingleUse);
}

#[test]
fn different_effect_cannot_cross_mediation_with_valid_permit() {
    let continuity = permission_continuity();
    let authority = authority_ref("authority:permit");
    let other_authority = authority_ref("authority:permit:other");
    let allowed = permission_effect(&continuity, &authority, "effect:permit:allowed");
    let foreign = permission_effect(&continuity, &other_authority, "effect:permit:foreign");
    let permit = granted_permission(&continuity, allowed);

    assert_eq!(
        mediate_permit(&continuity, permit, foreign),
        Err(MediationError::EffectMismatch {
            permitted: effect_ref("effect:permit:allowed"),
            presented: effect_ref("effect:permit:foreign"),
        })
    );
}

#[test]
fn mediated_t_e_executes_once_and_records_append_only_trace() {
    let continuity = permission_continuity();
    let commitment = mediated_commitment(&continuity);
    let mut execution = ExecutionContinuity::from_authority(continuity);
    let mut adapter = TestExecutor::default();

    let confirmation = execute_mediated(&mut execution, commitment, &mut adapter).unwrap();

    assert_eq!(adapter.calls, 1);
    assert_eq!(adapter.last_exercise.as_ref(), Some(confirmation.exercise()));
    assert_eq!(adapter.last_effect.as_ref(), Some(confirmation.effect().reference()));
    assert_eq!(confirmation.authority(), &authority_ref("authority:permit"));
    assert_eq!(confirmation.form(), &form_ref("form:permit"));
    assert_eq!(confirmation.effect().reference(), &effect_ref("effect:permit:allowed"));
    assert_eq!(confirmation.context(), &context_ref("context:permit"));
    assert_eq!(confirmation.exercise().id().as_str(), "exercise:1");
    assert_eq!(execution.exercise_event_count(), 2);

    let events = execution.exercise_events().collect::<Vec<_>>();
    assert_eq!(events[0].exercise(), confirmation.exercise());
    assert_eq!(events[0].state(), ExerciseAttemptState::DispatchCommitted);
    assert_eq!(events[1].exercise(), confirmation.exercise());
    assert_eq!(events[1].state(), ExerciseAttemptState::Confirmed);
    assert_eq!(
        execution.exercise_state(confirmation.exercise()),
        Some(ExerciseAttemptState::Confirmed)
    );
}

#[test]
fn adapter_error_after_dispatch_is_indeterminate_not_non_execution() {
    let continuity = permission_continuity();
    let commitment = mediated_commitment(&continuity);
    let mut execution = ExecutionContinuity::from_authority(continuity);
    let mut adapter = TestExecutor {
        fail: true,
        ..Default::default()
    };

    let result = execute_mediated(&mut execution, commitment, &mut adapter);
    let Err(ExecutionError::AdapterIndeterminate { exercise, error }) = result else {
        panic!("el error del adaptador debía quedar como ejecución indeterminada");
    };

    assert_eq!(error, "adapter-error");
    assert_eq!(adapter.calls, 1);
    assert_eq!(execution.exercise_event_count(), 2);
    assert_eq!(
        execution.exercise_state(&exercise),
        Some(ExerciseAttemptState::Indeterminate)
    );
    assert_eq!(
        execution.exercise_events().map(|entry| entry.state()).collect::<Vec<_>>(),
        vec![
            ExerciseAttemptState::DispatchCommitted,
            ExerciseAttemptState::Indeterminate,
        ]
    );
}

#[test]
fn single_use_blocks_second_dispatch_after_confirmed_exercise() {
    let continuity = permission_continuity();
    let first_commitment = mediated_commitment(&continuity);
    let mut execution = ExecutionContinuity::from_authority(continuity);
    let mut adapter = TestExecutor::default();

    let first = execute_mediated(&mut execution, first_commitment, &mut adapter).unwrap();
    let second_commitment = mediated_commitment(execution.authority());

    assert_eq!(
        execute_mediated(&mut execution, second_commitment, &mut adapter),
        Err(ExecutionError::SingleUseAlreadyDispatched(
            first.exercise().clone()
        ))
    );
    assert_eq!(adapter.calls, 1);
    assert_eq!(execution.exercise_event_count(), 2);
}

#[test]
fn single_use_blocks_second_dispatch_after_indeterminate_attempt() {
    let continuity = permission_continuity();
    let first_commitment = mediated_commitment(&continuity);
    let mut execution = ExecutionContinuity::from_authority(continuity);
    let mut failing = TestExecutor {
        fail: true,
        ..Default::default()
    };

    let first_result = execute_mediated(&mut execution, first_commitment, &mut failing);
    let Err(ExecutionError::AdapterIndeterminate { exercise, .. }) = first_result else {
        panic!("el primer intento debía quedar indeterminado");
    };

    let second_commitment = mediated_commitment(execution.authority());
    let mut succeeding = TestExecutor::default();
    assert_eq!(
        execute_mediated(&mut execution, second_commitment, &mut succeeding),
        Err(ExecutionError::SingleUseAlreadyDispatched(exercise))
    );
    assert_eq!(succeeding.calls, 0);
    assert_eq!(execution.exercise_event_count(), 2);
}

#[test]
fn idempotent_requires_new_governed_attempt_and_new_exercise_ref() {
    let continuity = permission_continuity_with(
        TransitionClass::Exercise,
        AccumulationContract::Idempotent,
    );
    let first_commitment = mediated_commitment(&continuity);
    let mut execution = ExecutionContinuity::from_authority(continuity);
    let mut adapter = TestExecutor::default();

    let first = execute_mediated(&mut execution, first_commitment, &mut adapter).unwrap();
    let second_commitment = mediated_commitment(execution.authority());
    let second = execute_mediated(&mut execution, second_commitment, &mut adapter).unwrap();

    assert_eq!(adapter.calls, 2);
    assert_ne!(first.exercise(), second.exercise());
    assert_eq!(first.exercise().id().as_str(), "exercise:1");
    assert_eq!(second.exercise().id().as_str(), "exercise:2");
    assert_eq!(execution.exercise_event_count(), 4);
}

#[test]
fn governed_accumulation_contracts_remain_closed_before_adapter() {
    let cases = [
        AccumulationContract::GovernedAggregator(accumulation_rule_ref("accumulation:aggregate")),
        AccumulationContract::DecidableTracePredicate(accumulation_rule_ref("accumulation:trace")),
    ];

    for accumulation in cases {
        let continuity = permission_continuity_with(TransitionClass::Exercise, accumulation.clone());
        let commitment = mediated_commitment(&continuity);
        let mut execution = ExecutionContinuity::from_authority(continuity);
        let mut adapter = TestExecutor::default();
        let result = execute_mediated(&mut execution, commitment, &mut adapter);

        match accumulation {
            AccumulationContract::GovernedAggregator(rule) => assert_eq!(
                result,
                Err(ExecutionError::GovernedAggregatorUnavailable(rule))
            ),
            AccumulationContract::DecidableTracePredicate(rule) => assert_eq!(
                result,
                Err(ExecutionError::TracePredicateUnavailable(rule))
            ),
            _ => unreachable!(),
        }
        assert_eq!(adapter.calls, 0);
        assert_eq!(execution.exercise_event_count(), 0);
    }
}

#[test]
fn non_exercise_commitment_is_rejected_before_adapter_dispatch() {
    let continuity = permission_continuity_with(
        TransitionClass::Information,
        AccumulationContract::NotApplicable,
    );
    let commitment = mediated_commitment(&continuity);
    let mut execution = ExecutionContinuity::from_authority(continuity);
    let mut adapter = TestExecutor::default();

    assert_eq!(
        execute_mediated(&mut execution, commitment, &mut adapter),
        Err(ExecutionError::UnsupportedTransitionClass(
            TransitionClass::Information
        ))
    );
    assert_eq!(adapter.calls, 0);
    assert_eq!(execution.exercise_event_count(), 0);
}

#[test]
fn r1_4_unit_three_does_not_make_authorizing_classes_productive() {
    for class in [
        TransitionClass::Governance,
        TransitionClass::Constitutive,
        TransitionClass::Recovery,
    ] {
        assert_eq!(
            transition_disposition(class),
            TransitionDisposition::BlockedPendingRequirements
        );
    }
}

#[test]
fn r1_4_unit_two_does_not_make_authorizing_classes_productive() {
    for class in [
        TransitionClass::Governance,
        TransitionClass::Constitutive,
        TransitionClass::Recovery,
    ] {
        assert_eq!(
            transition_disposition(class),
            TransitionDisposition::BlockedPendingRequirements
        );
    }
}

#[test]
fn r1_4_unit_one_does_not_make_authorizing_classes_productive() {
    for class in [
        TransitionClass::Governance,
        TransitionClass::Constitutive,
        TransitionClass::Recovery,
    ] {
        assert_eq!(
            transition_disposition(class),
            TransitionDisposition::BlockedPendingRequirements
        );
    }
}
