//! Conflicto entre comprobaciones de una misma obligación en R1-3.
//!
//! La unidad 3A materializa el caso sin regla de resolución constituida. La
//! unidad 3B añade una regla cerrada de verificador decisivo cuya constitución
//! depende de T-0 y de una relación `Applicable(V,q,C)` ya validada. Ninguna de
//! las dos vías produce permiso, autoridad ni efecto protegido.

use std::collections::BTreeSet;

use crate::authority::transitions::GenesisControlToken;
use crate::control::{
    ApplicabilityRuleRef, CheckResult, ConflictResolutionRuleRef, ContextRef, EffectFamilyRef,
    FormRef, RequirementRef, VerifierFamilyRef, VerifierRef,
};
use crate::requirements::{RequirementCheck, RequirementDescriptor, VerifierApplicability};

/// Regla constituida de resolución de conflicto para una obligación.
///
/// Esta primera realización fija un único verificador decisivo. La regla no
/// tiene constructor público: su constitución productiva sólo se realiza desde
/// T-0 y deriva las ligaduras materiales de la obligación y de la relación de
/// aplicabilidad ya constituidas.
#[derive(Debug, PartialEq, Eq)]
pub struct ConflictResolutionRule {
    reference: ConflictResolutionRuleRef,
    requirement: RequirementRef,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    decisive_verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    applicability_rule: ApplicabilityRuleRef,
}

impl ConflictResolutionRule {
    pub(crate) fn constitute_from_genesis(
        _token: &GenesisControlToken,
        reference: ConflictResolutionRuleRef,
        descriptor: &RequirementDescriptor,
        applicability: &VerifierApplicability,
    ) -> Self {
        Self {
            reference,
            requirement: descriptor.reference().clone(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            decisive_verifier: applicability.verifier().clone(),
            verifier_family: applicability.verifier_family().clone(),
            applicability_rule: applicability.applicability_rule().clone(),
        }
    }

    #[inline]
    pub fn reference(&self) -> &ConflictResolutionRuleRef {
        &self.reference
    }

    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        &self.form
    }

    #[inline]
    pub fn effect_family(&self) -> &EffectFamilyRef {
        &self.effect_family
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }

    #[inline]
    pub fn decisive_verifier(&self) -> &VerifierRef {
        &self.decisive_verifier
    }

    #[inline]
    pub fn verifier_family(&self) -> &VerifierFamilyRef {
        &self.verifier_family
    }

    #[inline]
    pub fn applicability_rule(&self) -> &ApplicabilityRuleRef {
        &self.applicability_rule
    }

    #[inline]
    fn matches_descriptor(&self, descriptor: &RequirementDescriptor) -> bool {
        self.requirement == *descriptor.reference()
            && self.form == *descriptor.form()
            && self.effect_family == *descriptor.effect_family()
            && self.context == *descriptor.context()
            && self.applicability_rule == *descriptor.applicability_rule()
            && descriptor
                .admissible_verifier_families()
                .any(|family| family == &self.verifier_family)
    }

    #[inline]
    fn matches_decisive_check(
        &self,
        descriptor: &RequirementDescriptor,
        check: &RequirementCheck,
    ) -> bool {
        check.matches_descriptor(descriptor)
            && check.verifier() == &self.decisive_verifier
            && check.verifier_family() == &self.verifier_family
            && check.applicability_rule() == &self.applicability_rule
    }
}

/// Entrada estructural inválida al resolver varias comprobaciones de una misma
/// obligación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementConflictError {
    EmptyChecks,
    MixedRequirements {
        expected: RequirementRef,
        found: RequirementRef,
    },
    DuplicateVerifier(VerifierRef),
    BindingMismatch {
        requirement: RequirementRef,
        verifier: VerifierRef,
    },
    RuleBindingMismatch(ConflictResolutionRuleRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConflictObservation {
    requirement: RequirementRef,
    verifier: VerifierRef,
    result: CheckResult,
}

/// Resuelve varias comprobaciones selladas de una misma obligación cuando no
/// existe una regla de resolución previamente constituida.
///
/// Si todas coinciden, conserva el resultado común. Si existe incompatibilidad
/// entre resultados de verificadores nominalmente distintos, devuelve `D-N`.
/// La repetición del mismo verificador se rechaza y el orden de entrada no
/// influye en el resultado.
pub fn resolve_requirement_checks_without_rule(
    checks: &[&RequirementCheck],
) -> Result<CheckResult, RequirementConflictError> {
    resolve_observations(checks.iter().map(|check| ConflictObservation {
        requirement: check.requirement().clone(),
        verifier: check.verifier().clone(),
        result: check.result(),
    }))
}

/// Resuelve comprobaciones selladas contra el descriptor constituido de la
/// obligación y, si existe conflicto, aplica exclusivamente la regla de
/// resolución previamente constituida dentro de ese descriptor.
///
/// La función no recibe una regla elegida por el llamador. De este modo, la
/// precedencia no es un parámetro ordinario del acto de comprobación.
pub fn resolve_requirement_checks(
    descriptor: &RequirementDescriptor,
    checks: &[&RequirementCheck],
) -> Result<CheckResult, RequirementConflictError> {
    let Some(first) = checks.first() else {
        return Err(RequirementConflictError::EmptyChecks);
    };

    let mut verifiers = BTreeSet::new();
    let common_result = first.result();
    let mut conflict = false;

    for check in checks {
        if !check.matches_descriptor(descriptor) {
            return Err(RequirementConflictError::BindingMismatch {
                requirement: check.requirement().clone(),
                verifier: check.verifier().clone(),
            });
        }

        if !verifiers.insert(check.verifier().clone()) {
            return Err(RequirementConflictError::DuplicateVerifier(
                check.verifier().clone(),
            ));
        }

        if check.result() != common_result {
            conflict = true;
        }
    }

    if !conflict {
        return Ok(common_result);
    }

    let Some(rule) = descriptor.conflict_resolution_rule() else {
        return Ok(CheckResult::NotVerifiable);
    };

    if !rule.matches_descriptor(descriptor) {
        return Err(RequirementConflictError::RuleBindingMismatch(
            rule.reference().clone(),
        ));
    }

    let Some(decisive_check) = checks
        .iter()
        .copied()
        .find(|check| check.verifier() == rule.decisive_verifier())
    else {
        return Ok(CheckResult::NotVerifiable);
    };

    if !rule.matches_decisive_check(descriptor, decisive_check) {
        return Err(RequirementConflictError::RuleBindingMismatch(
            rule.reference().clone(),
        ));
    }

    Ok(decisive_check.result())
}

fn resolve_observations(
    observations: impl IntoIterator<Item = ConflictObservation>,
) -> Result<CheckResult, RequirementConflictError> {
    let mut observations = observations.into_iter();
    let Some(first) = observations.next() else {
        return Err(RequirementConflictError::EmptyChecks);
    };

    let expected_requirement = first.requirement.clone();
    let common_result = first.result;
    let mut conflict = false;
    let mut verifiers = BTreeSet::new();
    verifiers.insert(first.verifier);

    for observation in observations {
        if observation.requirement != expected_requirement {
            return Err(RequirementConflictError::MixedRequirements {
                expected: expected_requirement,
                found: observation.requirement,
            });
        }

        if !verifiers.insert(observation.verifier.clone()) {
            return Err(RequirementConflictError::DuplicateVerifier(
                observation.verifier,
            ));
        }

        if observation.result != common_result {
            conflict = true;
        }
    }

    if conflict {
        Ok(CheckResult::NotVerifiable)
    } else {
        Ok(common_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ControlId;
    use crate::requirements::{RequirementClass, RequirementDescriptor, VerifierApplicability};

    fn id(value: &str) -> ControlId {
        ControlId::new(value).unwrap()
    }

    fn requirement(value: &str) -> RequirementRef {
        RequirementRef::from_core_id(id(value))
    }

    fn form(value: &str) -> FormRef {
        FormRef::from_core_id(id(value))
    }

    fn family(value: &str) -> EffectFamilyRef {
        EffectFamilyRef::from_core_id(id(value))
    }

    fn context(value: &str) -> ContextRef {
        ContextRef::from_core_id(id(value))
    }

    fn verifier(value: &str) -> VerifierRef {
        VerifierRef::from_core_id(id(value))
    }

    fn verifier_family(value: &str) -> VerifierFamilyRef {
        VerifierFamilyRef::from_core_id(id(value))
    }

    fn applicability_rule(value: &str) -> ApplicabilityRuleRef {
        ApplicabilityRuleRef::from_core_id(id(value))
    }

    fn conflict_rule_ref(value: &str) -> ConflictResolutionRuleRef {
        ConflictResolutionRuleRef::from_core_id(id(value))
    }

    fn observation(
        requirement_value: &str,
        verifier_value: &str,
        result: CheckResult,
    ) -> ConflictObservation {
        ConflictObservation {
            requirement: requirement(requirement_value),
            verifier: verifier(verifier_value),
            result,
        }
    }

    fn descriptor(context_value: &str) -> RequirementDescriptor {
        RequirementDescriptor::constitute_for_test(
            requirement("req:1"),
            RequirementClass::Specific,
            form("form:1"),
            family("family:write"),
            context(context_value),
            [verifier_family("verifier-family:canonical")],
            applicability_rule("applicability:canonical"),
        )
        .unwrap()
    }

    fn applicable(
        descriptor: &RequirementDescriptor,
        verifier_value: &str,
    ) -> VerifierApplicability {
        VerifierApplicability::constitute_for_test(
            verifier(verifier_value),
            verifier_family("verifier-family:canonical"),
            descriptor.reference().clone(),
            descriptor.context().clone(),
            applicability_rule("applicability:canonical"),
        )
    }

    fn check(
        descriptor: &RequirementDescriptor,
        verifier_value: &str,
        result: CheckResult,
    ) -> RequirementCheck {
        let applicability = applicable(descriptor, verifier_value);
        RequirementCheck::constitute_for_test(descriptor, &applicability, result).unwrap()
    }

    fn attach_decisive_rule(
        descriptor: &mut RequirementDescriptor,
        verifier_value: &str,
    ) {
        let applicability = applicable(descriptor, verifier_value);
        let rule = ConflictResolutionRule {
            reference: conflict_rule_ref("conflict-rule:1"),
            requirement: descriptor.reference().clone(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            decisive_verifier: applicability.verifier().clone(),
            verifier_family: applicability.verifier_family().clone(),
            applicability_rule: applicability.applicability_rule().clone(),
        };
        descriptor.attach_conflict_resolution_rule_for_test(rule);
    }

    #[test]
    fn empty_check_set_is_rejected() {
        assert_eq!(
            resolve_observations([]),
            Err(RequirementConflictError::EmptyChecks)
        );
    }

    #[test]
    fn mixed_requirements_are_not_a_single_conflict() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:2", "verifier:2", CheckResult::Refuted),
            ]),
            Err(RequirementConflictError::MixedRequirements {
                expected: requirement("req:1"),
                found: requirement("req:2"),
            })
        );
    }

    #[test]
    fn repeated_verifier_is_rejected_instead_of_weighted() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:1", CheckResult::Accredited),
            ]),
            Err(RequirementConflictError::DuplicateVerifier(verifier(
                "verifier:1"
            )))
        );
    }

    #[test]
    fn homogeneous_accredited_checks_remain_accredited() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:2", CheckResult::Accredited),
            ]),
            Ok(CheckResult::Accredited)
        );
    }

    #[test]
    fn homogeneous_refuted_checks_remain_refuted() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Refuted),
                observation("req:1", "verifier:2", CheckResult::Refuted),
            ]),
            Ok(CheckResult::Refuted)
        );
    }

    #[test]
    fn homogeneous_not_verifiable_checks_remain_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::NotVerifiable),
                observation("req:1", "verifier:2", CheckResult::NotVerifiable),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn accredited_and_refuted_without_rule_become_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:2", CheckResult::Refuted),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn accredited_and_not_verifiable_without_rule_remain_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:2", CheckResult::NotVerifiable),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn refuted_and_not_verifiable_without_rule_become_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Refuted),
                observation("req:1", "verifier:2", CheckResult::NotVerifiable),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn conflict_result_is_independent_of_input_order() {
        let left = resolve_observations([
            observation("req:1", "verifier:1", CheckResult::Accredited),
            observation("req:1", "verifier:2", CheckResult::Refuted),
            observation("req:1", "verifier:3", CheckResult::NotVerifiable),
        ]);
        let right = resolve_observations([
            observation("req:1", "verifier:3", CheckResult::NotVerifiable),
            observation("req:1", "verifier:1", CheckResult::Accredited),
            observation("req:1", "verifier:2", CheckResult::Refuted),
        ]);

        assert_eq!(left, Ok(CheckResult::NotVerifiable));
        assert_eq!(right, left);
    }

    #[test]
    fn public_governed_path_rejects_foreign_material_binding() {
        let local = descriptor("context:1");
        let foreign = descriptor("context:other");
        let foreign_check = check(&foreign, "verifier:1", CheckResult::Accredited);

        assert_eq!(
            resolve_requirement_checks(&local, &[&foreign_check]),
            Err(RequirementConflictError::BindingMismatch {
                requirement: requirement("req:1"),
                verifier: verifier("verifier:1"),
            })
        );
    }

    #[test]
    fn public_governed_path_without_rule_keeps_conflict_at_dn() {
        let descriptor = descriptor("context:1");
        let a = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let r = check(&descriptor, "verifier:2", CheckResult::Refuted);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&a, &r]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn decisive_accredited_verifier_resolves_conflict_to_da() {
        let mut descriptor = descriptor("context:1");
        attach_decisive_rule(&mut descriptor, "verifier:1");
        let a = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let r = check(&descriptor, "verifier:2", CheckResult::Refuted);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&a, &r]),
            Ok(CheckResult::Accredited)
        );
    }

    #[test]
    fn decisive_refuted_verifier_resolves_conflict_to_dr() {
        let mut descriptor = descriptor("context:1");
        attach_decisive_rule(&mut descriptor, "verifier:2");
        let a = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let r = check(&descriptor, "verifier:2", CheckResult::Refuted);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&a, &r]),
            Ok(CheckResult::Refuted)
        );
    }

    #[test]
    fn decisive_dn_verifier_cannot_promote_conflict() {
        let mut descriptor = descriptor("context:1");
        attach_decisive_rule(&mut descriptor, "verifier:1");
        let n = check(&descriptor, "verifier:1", CheckResult::NotVerifiable);
        let a = check(&descriptor, "verifier:2", CheckResult::Accredited);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&n, &a]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn absent_decisive_verifier_leaves_conflict_at_dn() {
        let mut descriptor = descriptor("context:1");
        attach_decisive_rule(&mut descriptor, "verifier:decisive");
        let a = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let r = check(&descriptor, "verifier:2", CheckResult::Refuted);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&a, &r]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn majority_does_not_override_constituted_decisive_verifier() {
        let mut descriptor = descriptor("context:1");
        attach_decisive_rule(&mut descriptor, "verifier:3");
        let a1 = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let a2 = check(&descriptor, "verifier:2", CheckResult::Accredited);
        let r = check(&descriptor, "verifier:3", CheckResult::Refuted);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&a1, &a2, &r]),
            Ok(CheckResult::Refuted)
        );
    }

    #[test]
    fn repeated_verifier_with_incompatible_results_is_rejected_on_public_path() {
        let descriptor = descriptor("context:1");
        let a = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let r = check(&descriptor, "verifier:1", CheckResult::Refuted);

        assert_eq!(
            resolve_requirement_checks(&descriptor, &[&a, &r]),
            Err(RequirementConflictError::DuplicateVerifier(verifier(
                "verifier:1"
            )))
        );
    }
}
