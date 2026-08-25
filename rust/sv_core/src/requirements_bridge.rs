//! Puente sellado entre la resolución intra-obligación y la agregación de R1-3.
//!
//! Una `RequirementCheck` describe una comprobación individual. Este módulo
//! impide que la agregación inter-obligaciones acepte directamente una
//! comprobación seleccionada. Sólo un `ResolvedRequirementResult` formado por
//! la vía de resolución puede entrar en la agregación gobernada.
//!
//! El resultado conserva la identidad de los verificadores participantes y la
//! referencia de la regla de cobertura constituida, cuando existe. Así una
//! variación de cobertura altera la ligadura material del sello.

use std::collections::BTreeSet;

use crate::control::{
    ApplicabilityRuleRef, CheckResult, ConflictResolutionRuleRef, ContextRef, CoverageRuleRef,
    EffectFamilyRef, FormRef, RequirementRef, VerifierFamilyRef, VerifierRef,
};
use crate::requirements::{RequirementCheck, RequirementClass, RequirementDescriptor, RequirementSet};
use crate::requirements_conflict::{resolve_requirement_checks, RequirementConflictError};

/// Resultado técnico ya resuelto de una obligación constituida.
///
/// No existe constructor público. El valor sólo puede formarse después de
/// atravesar `resolve_requirement_checks`, por lo que un `CheckResult` nominal
/// no basta para fabricar una entrada agregable.
///
/// ```compile_fail
/// use sv_core::{CheckResult, ResolvedRequirementResult};
/// let _ = ResolvedRequirementResult::new(CheckResult::Accredited);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct ResolvedRequirementResult {
    requirement: RequirementRef,
    class: RequirementClass,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    admissible_verifier_families: BTreeSet<VerifierFamilyRef>,
    applicability_rule: ApplicabilityRuleRef,
    conflict_resolution_rule: Option<ConflictResolutionRuleRef>,
    coverage_rule: Option<CoverageRuleRef>,
    participating_verifiers: BTreeSet<VerifierRef>,
    result: CheckResult,
}

impl ResolvedRequirementResult {
    fn from_governed_resolution(
        descriptor: &RequirementDescriptor,
        participating_verifiers: BTreeSet<VerifierRef>,
        result: CheckResult,
    ) -> Self {
        Self {
            requirement: descriptor.reference().clone(),
            class: descriptor.class(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            admissible_verifier_families: descriptor
                .admissible_verifier_families()
                .cloned()
                .collect(),
            applicability_rule: descriptor.applicability_rule().clone(),
            conflict_resolution_rule: descriptor
                .conflict_resolution_rule()
                .map(|rule| rule.reference().clone()),
            coverage_rule: descriptor
                .coverage_rule()
                .map(|rule| rule.reference().clone()),
            participating_verifiers,
            result,
        }
    }

    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    /// Verificadores cuyas comprobaciones formaron parte del conjunto
    /// efectivamente resuelto. Esta colección describe participación, no
    /// acredita por sí sola cobertura suficiente.
    #[inline]
    pub fn participating_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.participating_verifiers.iter()
    }

    #[inline]
    pub const fn result(&self) -> CheckResult {
        self.result
    }

    #[inline]
    fn matches_descriptor(&self, descriptor: &RequirementDescriptor) -> bool {
        self.requirement == *descriptor.reference()
            && self.class == descriptor.class()
            && self.form == *descriptor.form()
            && self.effect_family == *descriptor.effect_family()
            && self.context == *descriptor.context()
            && self.applicability_rule == *descriptor.applicability_rule()
            && self.admissible_verifier_families
                == descriptor
                    .admissible_verifier_families()
                    .cloned()
                    .collect::<BTreeSet<_>>()
            && self.conflict_resolution_rule
                == descriptor
                    .conflict_resolution_rule()
                    .map(|rule| rule.reference().clone())
            && self.coverage_rule
                == descriptor
                    .coverage_rule()
                    .map(|rule| rule.reference().clone())
    }
}

/// Resuelve el conjunto de comprobaciones suministrado para una obligación y
/// sella el resultado con la ligadura material de su descriptor constituido y
/// la identidad de los verificadores que participaron.
pub fn resolve_requirement_result(
    descriptor: &RequirementDescriptor,
    checks: &[&RequirementCheck],
) -> Result<ResolvedRequirementResult, RequirementConflictError> {
    let result = resolve_requirement_checks(descriptor, checks)?;
    let participating_verifiers = checks
        .iter()
        .map(|check| check.verifier().clone())
        .collect();
    Ok(ResolvedRequirementResult::from_governed_resolution(
        descriptor,
        participating_verifiers,
        result,
    ))
}

/// Entrada estructural inválida en la agregación de obligaciones de esta
/// unidad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedAggregationError {
    EmptyRequirementSet,
    UnexpectedResult(RequirementRef),
    DuplicateResult(RequirementRef),
    BindingMismatch(RequirementRef),
    MissingResult(RequirementRef),
}

/// Valida y agrega exactamente un resultado resuelto y sellado por cada
/// obligación de un `RequirementSet` completo.
///
/// Esta función permanece interna tras 3D. La agregación pública exige además
/// cualificación de cobertura. Aquí se conserva la validación estructural de 3C
/// y su regresión independiente.
pub(crate) fn aggregate_resolved_requirement_results(
    requirements: &RequirementSet,
    results: &[ResolvedRequirementResult],
) -> Result<CheckResult, ResolvedAggregationError> {
    if requirements.is_empty() {
        return Err(ResolvedAggregationError::EmptyRequirementSet);
    }

    let mut seen = BTreeSet::new();
    let mut saw_refuted = false;
    let mut saw_not_verifiable = false;

    for resolved in results {
        let reference = resolved.requirement().clone();
        let descriptor = requirements
            .requirement(&reference)
            .ok_or_else(|| ResolvedAggregationError::UnexpectedResult(reference.clone()))?;

        if !resolved.matches_descriptor(descriptor) {
            return Err(ResolvedAggregationError::BindingMismatch(reference));
        }

        if !seen.insert(resolved.requirement().clone()) {
            return Err(ResolvedAggregationError::DuplicateResult(
                resolved.requirement().clone(),
            ));
        }

        match resolved.result() {
            CheckResult::Accredited => {}
            CheckResult::Refuted => saw_refuted = true,
            CheckResult::NotVerifiable => saw_not_verifiable = true,
        }
    }

    for descriptor in requirements.iter() {
        if !seen.contains(descriptor.reference()) {
            return Err(ResolvedAggregationError::MissingResult(
                descriptor.reference().clone(),
            ));
        }
    }

    if saw_refuted {
        Ok(CheckResult::Refuted)
    } else if saw_not_verifiable {
        Ok(CheckResult::NotVerifiable)
    } else {
        Ok(CheckResult::Accredited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ControlId;
    use crate::requirements::{
        CoreRequirementKind, RequirementCheck, RequirementClass, RequirementDescriptor,
        RequirementSet, VerifierApplicability,
    };

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

    fn descriptor(reference: &str, class: RequirementClass) -> RequirementDescriptor {
        RequirementDescriptor::constitute_for_test(
            requirement(reference),
            class,
            form("form:1"),
            family("family:write"),
            context("context:1"),
            [verifier_family("verifier-family:canonical")],
            applicability_rule("applicability:canonical"),
        )
        .unwrap()
    }

    fn mandatory_descriptors() -> Vec<RequirementDescriptor> {
        [
            ("req:form", CoreRequirementKind::FormValidity),
            ("req:authority", CoreRequirementKind::ApplicableAuthority),
            (
                "req:verifier",
                CoreRequirementKind::VerifierAdmissibilityAndApplicability,
            ),
            ("req:no-self", CoreRequirementKind::NoSelfAccreditation),
        ]
        .into_iter()
        .map(|(reference, kind)| descriptor(reference, RequirementClass::Core(kind)))
        .collect()
    }

    fn set() -> RequirementSet {
        RequirementSet::constitute_for_test(
            form("form:1"),
            family("family:write"),
            context("context:1"),
            mandatory_descriptors(),
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
        verifier_value: &str,
        result: CheckResult,
    ) -> ResolvedRequirementResult {
        let check = check(descriptor, verifier_value, result);
        resolve_requirement_result(descriptor, &[&check]).unwrap()
    }

    #[test]
    fn homogeneous_single_check_is_sealed_with_its_descriptor_and_participant() {
        let descriptor = descriptor("req:1", RequirementClass::Specific);
        let resolved = resolved(&descriptor, "verifier:1", CheckResult::Accredited);

        assert_eq!(resolved.requirement(), descriptor.reference());
        assert_eq!(resolved.result(), CheckResult::Accredited);
        assert_eq!(
            resolved.participating_verifiers().collect::<Vec<_>>(),
            vec![&verifier("verifier:1")]
        );
        assert!(resolved.matches_descriptor(&descriptor));
    }

    #[test]
    fn conflict_without_rule_is_sealed_as_dn_and_preserves_participants() {
        let descriptor = descriptor("req:1", RequirementClass::Specific);
        let left = check(&descriptor, "verifier:1", CheckResult::Accredited);
        let right = check(&descriptor, "verifier:2", CheckResult::Refuted);

        let resolved = resolve_requirement_result(&descriptor, &[&left, &right]).unwrap();

        assert_eq!(resolved.result(), CheckResult::NotVerifiable);
        assert_eq!(resolved.participating_verifiers().count(), 2);
        assert!(resolved
            .participating_verifiers()
            .any(|value| value == &verifier("verifier:1")));
        assert!(resolved
            .participating_verifiers()
            .any(|value| value == &verifier("verifier:2")));
    }

    #[test]
    fn complete_accreditation_aggregates_to_da() {
        let set = set();
        let results: Vec<_> = set
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                resolved(
                    descriptor,
                    &format!("verifier:{index}"),
                    CheckResult::Accredited,
                )
            })
            .collect();

        assert_eq!(
            aggregate_resolved_requirement_results(&set, &results),
            Ok(CheckResult::Accredited)
        );
    }

    #[test]
    fn refutation_precedes_dn_after_resolution() {
        let set = set();
        let results: Vec<_> = set
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                let result = match index {
                    0 => CheckResult::NotVerifiable,
                    1 => CheckResult::Refuted,
                    _ => CheckResult::Accredited,
                };
                resolved(descriptor, &format!("verifier:{index}"), result)
            })
            .collect();

        assert_eq!(
            aggregate_resolved_requirement_results(&set, &results),
            Ok(CheckResult::Refuted)
        );
    }

    #[test]
    fn dn_is_preserved_without_refutation() {
        let set = set();
        let results: Vec<_> = set
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                let result = if index == 0 {
                    CheckResult::NotVerifiable
                } else {
                    CheckResult::Accredited
                };
                resolved(descriptor, &format!("verifier:{index}"), result)
            })
            .collect();

        assert_eq!(
            aggregate_resolved_requirement_results(&set, &results),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn aggregation_requires_complete_requirement_coverage() {
        let set = set();
        let first = set.iter().next().unwrap();
        let result = resolved(first, "verifier:1", CheckResult::Accredited);

        assert!(matches!(
            aggregate_resolved_requirement_results(&set, &[result]),
            Err(ResolvedAggregationError::MissingResult(_))
        ));
    }

    #[test]
    fn aggregation_rejects_unexpected_requirement_result() {
        let set = set();
        let foreign = descriptor("req:foreign", RequirementClass::Specific);
        let result = resolved(&foreign, "verifier:foreign", CheckResult::Accredited);

        assert_eq!(
            aggregate_resolved_requirement_results(&set, &[result]),
            Err(ResolvedAggregationError::UnexpectedResult(requirement(
                "req:foreign"
            )))
        );
    }

    #[test]
    fn aggregation_rejects_duplicate_requirement_result() {
        let set = set();
        let mut results: Vec<_> = set
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                resolved(
                    descriptor,
                    &format!("verifier:{index}"),
                    CheckResult::Accredited,
                )
            })
            .collect();
        let first = set.iter().next().unwrap();
        results.push(resolved(
            first,
            "verifier:duplicate",
            CheckResult::Accredited,
        ));

        assert_eq!(
            aggregate_resolved_requirement_results(&set, &results),
            Err(ResolvedAggregationError::DuplicateResult(
                first.reference().clone()
            ))
        );
    }

    #[test]
    fn aggregation_rejects_foreign_material_binding() {
        let set = set();
        let first = set.iter().next().unwrap();
        let foreign = RequirementDescriptor::constitute_for_test(
            first.reference().clone(),
            first.class(),
            form("form:other"),
            family("family:write"),
            context("context:1"),
            [verifier_family("verifier-family:canonical")],
            applicability_rule("applicability:canonical"),
        )
        .unwrap();
        let result = resolved(&foreign, "verifier:foreign", CheckResult::Accredited);

        assert_eq!(
            aggregate_resolved_requirement_results(&set, &[result]),
            Err(ResolvedAggregationError::BindingMismatch(
                first.reference().clone()
            ))
        );
    }

    #[test]
    fn aggregation_is_independent_of_result_order() {
        let set = set();
        let mut results: Vec<_> = set
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                let result = if index == 0 {
                    CheckResult::NotVerifiable
                } else {
                    CheckResult::Accredited
                };
                resolved(descriptor, &format!("verifier:{index}"), result)
            })
            .collect();

        let first = aggregate_resolved_requirement_results(&set, &results).unwrap();
        results.reverse();
        let second = aggregate_resolved_requirement_results(&set, &results).unwrap();

        assert_eq!(first, second);
        assert_eq!(first, CheckResult::NotVerifiable);
    }
}
