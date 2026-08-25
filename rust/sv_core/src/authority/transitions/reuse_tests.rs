use super::*;
use crate::control::{
    ApplicabilityRuleRef, ControlId, ReuseBindingKeyRef, ReuseBindingValueRef, ReuseRuleRef,
    VerifierFamilyRef,
};
use crate::requirements::initial::{
    InitialRequirementError, ReuseBindingProposal, ReuseRuleProposal,
};
use crate::requirements::{CoreRequirementKind, RequirementClass};

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

fn verifier_family_ref(value: &str) -> VerifierFamilyRef {
    VerifierFamilyRef::from_core_id(id(value))
}

fn applicability_rule_ref(value: &str) -> ApplicabilityRuleRef {
    ApplicabilityRuleRef::from_core_id(id(value))
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
