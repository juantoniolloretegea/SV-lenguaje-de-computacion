//! Cobertura de comprobaciones exigibles para una obligación de R1-3.
//!
//! La aplicabilidad de un verificador no lo convierte por sí sola en obligatorio.
//! Este módulo representa una regla de cobertura previamente constituida y
//! evalúa, sin producir permiso ni autoridad, si todos los verificadores exigidos
//! por esa regla participaron en el resultado resuelto de la obligación.

use std::collections::BTreeSet;

use crate::authority::transitions::GenesisControlToken;
use crate::control::{
    ContextRef, CoverageRuleRef, EffectFamilyRef, FormRef, RequirementRef, VerifierRef,
};
use crate::requirements::RequirementDescriptor;
use crate::requirements_bridge::ResolvedRequirementResult;

/// Regla cerrada de cobertura para una obligación constituida.
///
/// La primera realización de R1-3 fija un conjunto no vacío de verificadores
/// concretos cuya participación resulta necesaria para acreditar cobertura
/// positiva. `Applicable(V,q,C)` y `required(V,q,C)` permanecen separados.
#[derive(Debug, PartialEq, Eq)]
pub struct CoverageRule {
    reference: CoverageRuleRef,
    requirement: RequirementRef,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    required_verifiers: BTreeSet<VerifierRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageRuleFormationError {
    EmptyRequiredVerifierSet,
    DuplicateRequiredVerifier(VerifierRef),
}

impl CoverageRule {
    pub(crate) fn constitute_from_genesis(
        _token: &GenesisControlToken,
        reference: CoverageRuleRef,
        descriptor: &RequirementDescriptor,
        required_verifiers: BTreeSet<VerifierRef>,
    ) -> Self {
        debug_assert!(!required_verifiers.is_empty());
        Self {
            reference,
            requirement: descriptor.reference().clone(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            required_verifiers,
        }
    }

    #[cfg(test)]
    pub(crate) fn constitute_for_test(
        reference: CoverageRuleRef,
        descriptor: &RequirementDescriptor,
        required_verifiers: impl IntoIterator<Item = VerifierRef>,
    ) -> Result<Self, CoverageRuleFormationError> {
        let mut required = BTreeSet::new();
        for verifier in required_verifiers {
            if !required.insert(verifier.clone()) {
                return Err(CoverageRuleFormationError::DuplicateRequiredVerifier(
                    verifier,
                ));
            }
        }
        if required.is_empty() {
            return Err(CoverageRuleFormationError::EmptyRequiredVerifierSet);
        }

        Ok(Self {
            reference,
            requirement: descriptor.reference().clone(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            required_verifiers: required,
        })
    }

    #[inline]
    pub fn reference(&self) -> &CoverageRuleRef {
        &self.reference
    }

    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub fn required_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.required_verifiers.iter()
    }

    #[inline]
    pub(crate) fn matches_descriptor(&self, descriptor: &RequirementDescriptor) -> bool {
        self.requirement == *descriptor.reference()
            && self.form == *descriptor.form()
            && self.effect_family == *descriptor.effect_family()
            && self.context == *descriptor.context()
    }
}

/// Resultado técnico de la evaluación de cobertura.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageDisposition {
    Complete,
    Incomplete,
}

/// Evidencia estructural de qué participantes exige la regla, cuáles estuvieron
/// presentes y cuáles faltaron.
///
/// Este objeto no es un `CheckResult`, no pertenece a `Tri` y no produce
/// permiso. En esta primera subunidad tampoco sustituye todavía al resultado
/// resuelto dentro de la agregación inter-obligaciones.
#[derive(Debug, PartialEq, Eq)]
pub struct CoverageAssessment {
    requirement: RequirementRef,
    rule: Option<CoverageRuleRef>,
    required_verifiers: BTreeSet<VerifierRef>,
    participating_verifiers: BTreeSet<VerifierRef>,
    missing_required_verifiers: BTreeSet<VerifierRef>,
    disposition: CoverageDisposition,
}

impl CoverageAssessment {
    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub fn rule(&self) -> Option<&CoverageRuleRef> {
        self.rule.as_ref()
    }

    #[inline]
    pub const fn disposition(&self) -> CoverageDisposition {
        self.disposition
    }

    #[inline]
    pub fn required_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.required_verifiers.iter()
    }

    #[inline]
    pub fn participating_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.participating_verifiers.iter()
    }

    #[inline]
    pub fn missing_required_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.missing_required_verifiers.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageAssessmentError {
    ResultRequirementMismatch {
        expected: RequirementRef,
        found: RequirementRef,
    },
    RuleBindingMismatch(CoverageRuleRef),
}

/// Evalúa la cobertura del conjunto efectivamente resuelto para una obligación.
///
/// La ausencia de regla no se interpreta como cobertura vacía: produce
/// `Incomplete`. Los verificadores participantes que no son exigidos por la
/// regla no añaden peso, voto ni autoridad.
pub fn assess_requirement_coverage(
    descriptor: &RequirementDescriptor,
    rule: Option<&CoverageRule>,
    resolved: &ResolvedRequirementResult,
) -> Result<CoverageAssessment, CoverageAssessmentError> {
    if resolved.requirement() != descriptor.reference() {
        return Err(CoverageAssessmentError::ResultRequirementMismatch {
            expected: descriptor.reference().clone(),
            found: resolved.requirement().clone(),
        });
    }

    if let Some(rule) = rule {
        if !rule.matches_descriptor(descriptor) {
            return Err(CoverageAssessmentError::RuleBindingMismatch(
                rule.reference().clone(),
            ));
        }
    }

    let participating_verifiers: BTreeSet<_> = resolved
        .participating_verifiers()
        .cloned()
        .collect();
    let required_verifiers: BTreeSet<_> = rule
        .map(|rule| rule.required_verifiers().cloned().collect())
        .unwrap_or_default();
    let missing_required_verifiers: BTreeSet<_> = required_verifiers
        .difference(&participating_verifiers)
        .cloned()
        .collect();

    let disposition = if rule.is_some() && missing_required_verifiers.is_empty() {
        CoverageDisposition::Complete
    } else {
        CoverageDisposition::Incomplete
    };

    Ok(CoverageAssessment {
        requirement: descriptor.reference().clone(),
        rule: rule.map(|rule| rule.reference().clone()),
        required_verifiers,
        participating_verifiers,
        missing_required_verifiers,
        disposition,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::{ApplicabilityRuleRef, CheckResult, ControlId, VerifierFamilyRef};
    use crate::requirements::{
        RequirementCheck, RequirementClass, RequirementDescriptor, VerifierApplicability,
    };
    use crate::requirements_bridge::resolve_requirement_result;

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

    fn coverage_rule(value: &str) -> CoverageRuleRef {
        CoverageRuleRef::from_core_id(id(value))
    }

    fn descriptor(reference: &str, context_value: &str) -> RequirementDescriptor {
        RequirementDescriptor::constitute_for_test(
            requirement(reference),
            RequirementClass::Specific,
            form("form:1"),
            family("family:write"),
            context(context_value),
            [verifier_family("verifier-family:canonical")],
            applicability_rule("applicability:canonical"),
        )
        .unwrap()
    }

    fn check(
        descriptor: &RequirementDescriptor,
        verifier_value: &str,
        result: CheckResult,
    ) -> RequirementCheck {
        let applicability = VerifierApplicability::constitute_for_test(
            verifier(verifier_value),
            verifier_family("verifier-family:canonical"),
            descriptor.reference().clone(),
            descriptor.context().clone(),
            applicability_rule("applicability:canonical"),
        );
        RequirementCheck::constitute_for_test(descriptor, &applicability, result).unwrap()
    }

    fn resolved(
        descriptor: &RequirementDescriptor,
        verifiers: &[(&str, CheckResult)],
    ) -> ResolvedRequirementResult {
        let checks: Vec<_> = verifiers
            .iter()
            .map(|(value, result)| check(descriptor, value, *result))
            .collect();
        let refs: Vec<_> = checks.iter().collect();
        resolve_requirement_result(descriptor, &refs).unwrap()
    }

    #[test]
    fn coverage_rule_rejects_empty_required_set() {
        let descriptor = descriptor("req:1", "context:1");
        assert_eq!(
            CoverageRule::constitute_for_test(coverage_rule("coverage:1"), &descriptor, []),
            Err(CoverageRuleFormationError::EmptyRequiredVerifierSet)
        );
    }

    #[test]
    fn coverage_rule_rejects_duplicate_required_verifier() {
        let descriptor = descriptor("req:1", "context:1");
        let repeated = verifier("verifier:1");
        assert_eq!(
            CoverageRule::constitute_for_test(
                coverage_rule("coverage:1"),
                &descriptor,
                [repeated.clone(), repeated.clone()],
            ),
            Err(CoverageRuleFormationError::DuplicateRequiredVerifier(repeated))
        );
    }

    #[test]
    fn absence_of_rule_is_not_empty_positive_coverage() {
        let descriptor = descriptor("req:1", "context:1");
        let resolved = resolved(&descriptor, &[("verifier:1", CheckResult::Accredited)]);
        let assessment = assess_requirement_coverage(&descriptor, None, &resolved).unwrap();

        assert_eq!(assessment.disposition(), CoverageDisposition::Incomplete);
        assert_eq!(assessment.rule(), None);
        assert_eq!(assessment.required_verifiers().count(), 0);
    }

    #[test]
    fn all_required_participants_complete_coverage() {
        let descriptor = descriptor("req:1", "context:1");
        let rule = CoverageRule::constitute_for_test(
            coverage_rule("coverage:1"),
            &descriptor,
            [verifier("verifier:1"), verifier("verifier:2")],
        )
        .unwrap();
        let resolved = resolved(
            &descriptor,
            &[
                ("verifier:1", CheckResult::Accredited),
                ("verifier:2", CheckResult::Accredited),
            ],
        );

        let assessment = assess_requirement_coverage(&descriptor, Some(&rule), &resolved).unwrap();
        assert_eq!(assessment.disposition(), CoverageDisposition::Complete);
        assert_eq!(assessment.missing_required_verifiers().count(), 0);
    }

    #[test]
    fn missing_required_participant_is_explicit() {
        let descriptor = descriptor("req:1", "context:1");
        let rule = CoverageRule::constitute_for_test(
            coverage_rule("coverage:1"),
            &descriptor,
            [verifier("verifier:1"), verifier("verifier:2")],
        )
        .unwrap();
        let resolved = resolved(&descriptor, &[("verifier:1", CheckResult::Accredited)]);

        let assessment = assess_requirement_coverage(&descriptor, Some(&rule), &resolved).unwrap();
        assert_eq!(assessment.disposition(), CoverageDisposition::Incomplete);
        assert_eq!(
            assessment.missing_required_verifiers().collect::<Vec<_>>(),
            vec![&verifier("verifier:2")]
        );
    }

    #[test]
    fn extra_participant_does_not_replace_a_missing_required_one() {
        let descriptor = descriptor("req:1", "context:1");
        let rule = CoverageRule::constitute_for_test(
            coverage_rule("coverage:1"),
            &descriptor,
            [verifier("verifier:required")],
        )
        .unwrap();
        let resolved = resolved(&descriptor, &[("verifier:extra", CheckResult::Accredited)]);

        let assessment = assess_requirement_coverage(&descriptor, Some(&rule), &resolved).unwrap();
        assert_eq!(assessment.disposition(), CoverageDisposition::Incomplete);
        assert!(assessment
            .missing_required_verifiers()
            .any(|value| value == &verifier("verifier:required")));
    }

    #[test]
    fn rule_bound_to_other_context_is_rejected() {
        let local = descriptor("req:1", "context:1");
        let foreign = descriptor("req:1", "context:other");
        let rule = CoverageRule::constitute_for_test(
            coverage_rule("coverage:foreign"),
            &foreign,
            [verifier("verifier:1")],
        )
        .unwrap();
        let resolved = resolved(&local, &[("verifier:1", CheckResult::Accredited)]);

        assert_eq!(
            assess_requirement_coverage(&local, Some(&rule), &resolved),
            Err(CoverageAssessmentError::RuleBindingMismatch(
                coverage_rule("coverage:foreign")
            ))
        );
    }

    #[test]
    fn result_from_other_requirement_is_rejected() {
        let local = descriptor("req:1", "context:1");
        let foreign = descriptor("req:2", "context:1");
        let resolved = resolved(&foreign, &[("verifier:1", CheckResult::Accredited)]);

        assert_eq!(
            assess_requirement_coverage(&local, None, &resolved),
            Err(CoverageAssessmentError::ResultRequirementMismatch {
                expected: requirement("req:1"),
                found: requirement("req:2"),
            })
        );
    }
}
