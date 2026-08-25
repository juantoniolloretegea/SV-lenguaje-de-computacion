//! Requisitos, aplicabilidad y resultados técnicos de R1-3.
//!
//! Esta primera unidad materializa estructuras cerradas para `Req(F,e | C)` y
//! `Applicable(V,q,C)`. No ofrece constructores públicos para constituir esas
//! relaciones durante el acto de comprobación, no produce `Permit` y no
//! ejecuta efectos protegidos.
//!
//! Un valor `CheckResult::Accredited` aislado sigue siendo sólo un resultado
//! técnico nominal. La agregación pública de este módulo acepta únicamente
//! comprobaciones ligadas a una obligación y a una relación de aplicabilidad
//! ya constituida.

use std::collections::{BTreeMap, BTreeSet};

use crate::control::{
    ApplicabilityRuleRef, CheckResult, ContextRef, EffectFamilyRef, FormRef, RequirementRef,
    VerifierFamilyRef, VerifierRef,
};

/// Obligación nuclear de SEC.0-D.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoreRequirementKind {
    FormValidity,
    ApplicableAuthority,
    GovernedDomainMembership,
    VerifierAdmissibilityAndApplicability,
    NoSelfAccreditation,
    ValidityOrNonRevocation,
}

/// Clasificación de una obligación dentro de `Req = N ∪ S`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementClass {
    Core(CoreRequirementKind),
    Specific,
}

/// Descriptor inmutable de una obligación ligada a forma, familia de efectos y
/// contexto.
///
/// La primera unidad de R1-3 no expone un constructor productivo. La existencia
/// de una referencia nominal no basta para fabricar este objeto constituido.
#[derive(Debug, PartialEq, Eq)]
pub struct RequirementDescriptor {
    reference: RequirementRef,
    class: RequirementClass,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    admissible_verifier_families: BTreeSet<VerifierFamilyRef>,
    applicability_rule: ApplicabilityRuleRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidRequirementDescriptor {
    NoAdmissibleVerifierFamily,
}

impl RequirementDescriptor {
    #[cfg(test)]
    fn constitute_for_test(
        reference: RequirementRef,
        class: RequirementClass,
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
        admissible_verifier_families: impl IntoIterator<Item = VerifierFamilyRef>,
        applicability_rule: ApplicabilityRuleRef,
    ) -> Result<Self, InvalidRequirementDescriptor> {
        let admissible_verifier_families: BTreeSet<_> =
            admissible_verifier_families.into_iter().collect();
        if admissible_verifier_families.is_empty() {
            return Err(InvalidRequirementDescriptor::NoAdmissibleVerifierFamily);
        }

        Ok(Self {
            reference,
            class,
            form,
            effect_family,
            context,
            admissible_verifier_families,
            applicability_rule,
        })
    }

    #[inline]
    pub fn reference(&self) -> &RequirementRef {
        &self.reference
    }

    #[inline]
    pub const fn class(&self) -> RequirementClass {
        self.class
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
    pub fn admissible_verifier_families(&self) -> impl Iterator<Item = &VerifierFamilyRef> {
        self.admissible_verifier_families.iter()
    }

    #[inline]
    pub fn applicability_rule(&self) -> &ApplicabilityRuleRef {
        &self.applicability_rule
    }

    #[inline]
    pub fn accepts_applicability(&self, applicability: &VerifierApplicability) -> bool {
        applicability.requirement == self.reference
            && applicability.context == self.context
            && applicability.applicability_rule == self.applicability_rule
            && self
                .admissible_verifier_families
                .contains(&applicability.verifier_family)
    }
}

/// Relación constituida `Applicable(V,q,C)`.
///
/// No es un booleano suministrable por el verificador ni por el beneficiario
/// del efecto. La primera unidad no ofrece constructor público para esta
/// relación.
#[derive(Debug, PartialEq, Eq)]
pub struct VerifierApplicability {
    verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    requirement: RequirementRef,
    context: ContextRef,
    applicability_rule: ApplicabilityRuleRef,
}

impl VerifierApplicability {
    #[cfg(test)]
    fn constitute_for_test(
        verifier: VerifierRef,
        verifier_family: VerifierFamilyRef,
        requirement: RequirementRef,
        context: ContextRef,
        applicability_rule: ApplicabilityRuleRef,
    ) -> Self {
        Self {
            verifier,
            verifier_family,
            requirement,
            context,
            applicability_rule,
        }
    }

    #[inline]
    pub fn verifier(&self) -> &VerifierRef {
        &self.verifier
    }

    #[inline]
    pub fn verifier_family(&self) -> &VerifierFamilyRef {
        &self.verifier_family
    }

    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }

    #[inline]
    pub fn applicability_rule(&self) -> &ApplicabilityRuleRef {
        &self.applicability_rule
    }
}

/// Conjunto constituido de obligaciones aplicables a una forma, familia de
/// efectos y contexto.
///
/// La construcción productiva queda deliberadamente fuera de esta primera
/// unidad. El tipo conserva el invariante de no vacuidad y el núcleo obligatorio
/// cuando se constituye en las pruebas de realización.
#[derive(Debug, PartialEq, Eq)]
pub struct RequirementSet {
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    requirements: BTreeMap<RequirementRef, RequirementDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidRequirementSet {
    Empty,
    DuplicateRequirementRef(RequirementRef),
    InconsistentForm(RequirementRef),
    InconsistentEffectFamily(RequirementRef),
    InconsistentContext(RequirementRef),
    MissingMandatoryCore(CoreRequirementKind),
}

impl RequirementSet {
    #[cfg(test)]
    fn constitute_for_test(
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
        descriptors: impl IntoIterator<Item = RequirementDescriptor>,
    ) -> Result<Self, InvalidRequirementSet> {
        let mut requirements = BTreeMap::new();

        for descriptor in descriptors {
            if descriptor.form != form {
                return Err(InvalidRequirementSet::InconsistentForm(
                    descriptor.reference.clone(),
                ));
            }
            if descriptor.effect_family != effect_family {
                return Err(InvalidRequirementSet::InconsistentEffectFamily(
                    descriptor.reference.clone(),
                ));
            }
            if descriptor.context != context {
                return Err(InvalidRequirementSet::InconsistentContext(
                    descriptor.reference.clone(),
                ));
            }

            let reference = descriptor.reference.clone();
            if requirements.insert(reference.clone(), descriptor).is_some() {
                return Err(InvalidRequirementSet::DuplicateRequirementRef(reference));
            }
        }

        if requirements.is_empty() {
            return Err(InvalidRequirementSet::Empty);
        }

        for mandatory in [
            CoreRequirementKind::FormValidity,
            CoreRequirementKind::ApplicableAuthority,
            CoreRequirementKind::VerifierAdmissibilityAndApplicability,
            CoreRequirementKind::NoSelfAccreditation,
        ] {
            let present = requirements.values().any(|descriptor| {
                descriptor.class == RequirementClass::Core(mandatory)
            });
            if !present {
                return Err(InvalidRequirementSet::MissingMandatoryCore(mandatory));
            }
        }

        Ok(Self {
            form,
            effect_family,
            context,
            requirements,
        })
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
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    #[inline]
    pub fn requirement(&self, reference: &RequirementRef) -> Option<&RequirementDescriptor> {
        self.requirements.get(reference)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &RequirementDescriptor> {
        self.requirements.values()
    }
}

/// Comprobación ligada a una obligación y a un verificador cuya aplicabilidad
/// coincide con el descriptor constituido de esa obligación.
///
/// El tipo no tiene constructor público. Un `CheckResult` nominal no puede
/// convertirse por sí solo en una `RequirementCheck` agregable.
#[derive(Debug, PartialEq, Eq)]
pub struct RequirementCheck {
    requirement: RequirementRef,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    applicability_rule: ApplicabilityRuleRef,
    result: CheckResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckFormationError {
    ApplicabilityMismatch,
}

impl RequirementCheck {
    #[cfg(test)]
    fn constitute_for_test(
        descriptor: &RequirementDescriptor,
        applicability: &VerifierApplicability,
        result: CheckResult,
    ) -> Result<Self, CheckFormationError> {
        if !descriptor.accepts_applicability(applicability) {
            return Err(CheckFormationError::ApplicabilityMismatch);
        }

        Ok(Self {
            requirement: descriptor.reference.clone(),
            form: descriptor.form.clone(),
            effect_family: descriptor.effect_family.clone(),
            context: descriptor.context.clone(),
            verifier: applicability.verifier.clone(),
            verifier_family: applicability.verifier_family.clone(),
            applicability_rule: applicability.applicability_rule.clone(),
            result,
        })
    }

    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub fn verifier(&self) -> &VerifierRef {
        &self.verifier
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
    pub const fn result(&self) -> CheckResult {
        self.result
    }

    #[inline]
    fn matches_descriptor(&self, descriptor: &RequirementDescriptor) -> bool {
        self.requirement == descriptor.reference
            && self.form == descriptor.form
            && self.effect_family == descriptor.effect_family
            && self.context == descriptor.context
            && descriptor
                .admissible_verifier_families
                .contains(&self.verifier_family)
            && self.applicability_rule == descriptor.applicability_rule
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckAggregationError {
    EmptyRequirementSet,
    UnexpectedCheck(RequirementRef),
    DuplicateCheck(RequirementRef),
    BindingMismatch(RequirementRef),
    MissingCheck(RequirementRef),
}

/// Agrega comprobaciones ligadas a un `RequirementSet` completo.
///
/// La función no acepta `CheckResult` crudos. Exige una comprobación sellada
/// por obligación, verifica cobertura exacta y conserva la precedencia
/// contractual `D-R > D-N > D-A`.
pub fn aggregate_requirement_checks(
    requirements: &RequirementSet,
    checks: &[RequirementCheck],
) -> Result<CheckResult, CheckAggregationError> {
    if requirements.is_empty() {
        return Err(CheckAggregationError::EmptyRequirementSet);
    }

    let mut seen = BTreeSet::new();
    let mut saw_refuted = false;
    let mut saw_not_verifiable = false;

    for check in checks {
        let reference = check.requirement.clone();
        let descriptor = requirements
            .requirement(&reference)
            .ok_or_else(|| CheckAggregationError::UnexpectedCheck(reference.clone()))?;

        if !check.matches_descriptor(descriptor) {
            return Err(CheckAggregationError::BindingMismatch(reference));
        }

        if !seen.insert(check.requirement.clone()) {
            return Err(CheckAggregationError::DuplicateCheck(
                check.requirement.clone(),
            ));
        }

        match check.result {
            CheckResult::Accredited => {}
            CheckResult::Refuted => saw_refuted = true,
            CheckResult::NotVerifiable => saw_not_verifiable = true,
        }
    }

    for reference in requirements.requirements.keys() {
        if !seen.contains(reference) {
            return Err(CheckAggregationError::MissingCheck(reference.clone()));
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

    fn id(value: &str) -> ControlId {
        ControlId::new(value).unwrap()
    }

    fn requirement_ref(value: &str) -> RequirementRef {
        RequirementRef::from_core_id(id(value))
    }

    fn form_ref(value: &str) -> FormRef {
        FormRef::from_core_id(id(value))
    }

    fn effect_family_ref(value: &str) -> EffectFamilyRef {
        EffectFamilyRef::from_core_id(id(value))
    }

    fn context_ref(value: &str) -> ContextRef {
        ContextRef::from_core_id(id(value))
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

    fn descriptor(
        reference: &str,
        class: RequirementClass,
        form: &str,
        family: &str,
        context: &str,
    ) -> RequirementDescriptor {
        RequirementDescriptor::constitute_for_test(
            requirement_ref(reference),
            class,
            form_ref(form),
            effect_family_ref(family),
            context_ref(context),
            [verifier_family_ref("verifier-family:canonical")],
            applicability_rule_ref("applicability:canonical"),
        )
        .unwrap()
    }

    fn mandatory_descriptors() -> Vec<RequirementDescriptor> {
        [
            (
                "req:form",
                CoreRequirementKind::FormValidity,
            ),
            (
                "req:authority",
                CoreRequirementKind::ApplicableAuthority,
            ),
            (
                "req:verifier",
                CoreRequirementKind::VerifierAdmissibilityAndApplicability,
            ),
            (
                "req:no-self",
                CoreRequirementKind::NoSelfAccreditation,
            ),
        ]
        .into_iter()
        .map(|(reference, core)| {
            descriptor(
                reference,
                RequirementClass::Core(core),
                "form:1",
                "family:write",
                "context:1",
            )
        })
        .collect()
    }

    fn requirement_set() -> RequirementSet {
        RequirementSet::constitute_for_test(
            form_ref("form:1"),
            effect_family_ref("family:write"),
            context_ref("context:1"),
            mandatory_descriptors(),
        )
        .unwrap()
    }

    fn applicable_to(descriptor: &RequirementDescriptor, verifier: &str) -> VerifierApplicability {
        VerifierApplicability::constitute_for_test(
            verifier_ref(verifier),
            verifier_family_ref("verifier-family:canonical"),
            descriptor.reference().clone(),
            context_ref("context:1"),
            applicability_rule_ref("applicability:canonical"),
        )
    }

    fn check(
        descriptor: &RequirementDescriptor,
        verifier: &str,
        result: CheckResult,
    ) -> RequirementCheck {
        let applicability = applicable_to(descriptor, verifier);
        RequirementCheck::constitute_for_test(descriptor, &applicability, result).unwrap()
    }

    #[test]
    fn requirement_set_rejects_empty_req() {
        assert_eq!(
            RequirementSet::constitute_for_test(
                form_ref("form:1"),
                effect_family_ref("family:write"),
                context_ref("context:1"),
                [],
            ),
            Err(InvalidRequirementSet::Empty)
        );
    }

    #[test]
    fn requirement_set_requires_the_non_eludible_core() {
        let descriptors = vec![descriptor(
            "req:form",
            RequirementClass::Core(CoreRequirementKind::FormValidity),
            "form:1",
            "family:write",
            "context:1",
        )];

        assert_eq!(
            RequirementSet::constitute_for_test(
                form_ref("form:1"),
                effect_family_ref("family:write"),
                context_ref("context:1"),
                descriptors,
            ),
            Err(InvalidRequirementSet::MissingMandatoryCore(
                CoreRequirementKind::ApplicableAuthority
            ))
        );
    }

    #[test]
    fn requirement_set_rejects_reused_reference() {
        let mut descriptors = mandatory_descriptors();
        descriptors.push(descriptor(
            "req:form",
            RequirementClass::Specific,
            "form:1",
            "family:write",
            "context:1",
        ));

        assert_eq!(
            RequirementSet::constitute_for_test(
                form_ref("form:1"),
                effect_family_ref("family:write"),
                context_ref("context:1"),
                descriptors,
            ),
            Err(InvalidRequirementSet::DuplicateRequirementRef(requirement_ref(
                "req:form"
            )))
        );
    }

    #[test]
    fn applicability_must_match_requirement_context_family_and_rule() {
        let descriptor = descriptor(
            "req:1",
            RequirementClass::Specific,
            "form:1",
            "family:write",
            "context:1",
        );
        let matching = applicable_to(&descriptor, "verifier:1");
        assert!(descriptor.accepts_applicability(&matching));

        let wrong_requirement = VerifierApplicability::constitute_for_test(
            verifier_ref("verifier:1"),
            verifier_family_ref("verifier-family:canonical"),
            requirement_ref("req:other"),
            context_ref("context:1"),
            applicability_rule_ref("applicability:canonical"),
        );
        assert!(!descriptor.accepts_applicability(&wrong_requirement));

        let wrong_context = VerifierApplicability::constitute_for_test(
            verifier_ref("verifier:1"),
            verifier_family_ref("verifier-family:canonical"),
            descriptor.reference().clone(),
            context_ref("context:other"),
            applicability_rule_ref("applicability:canonical"),
        );
        assert!(!descriptor.accepts_applicability(&wrong_context));

        let wrong_family = VerifierApplicability::constitute_for_test(
            verifier_ref("verifier:1"),
            verifier_family_ref("verifier-family:other"),
            descriptor.reference().clone(),
            context_ref("context:1"),
            applicability_rule_ref("applicability:canonical"),
        );
        assert!(!descriptor.accepts_applicability(&wrong_family));

        let wrong_rule = VerifierApplicability::constitute_for_test(
            verifier_ref("verifier:1"),
            verifier_family_ref("verifier-family:canonical"),
            descriptor.reference().clone(),
            context_ref("context:1"),
            applicability_rule_ref("applicability:other"),
        );
        assert!(!descriptor.accepts_applicability(&wrong_rule));
    }

    #[test]
    fn nominal_da_cannot_form_an_aggregable_check_without_applicability() {
        let descriptor = descriptor(
            "req:1",
            RequirementClass::Specific,
            "form:1",
            "family:write",
            "context:1",
        );
        let wrong = VerifierApplicability::constitute_for_test(
            verifier_ref("verifier:1"),
            verifier_family_ref("verifier-family:other"),
            descriptor.reference().clone(),
            context_ref("context:1"),
            applicability_rule_ref("applicability:canonical"),
        );

        assert_eq!(
            RequirementCheck::constitute_for_test(&descriptor, &wrong, CheckResult::Accredited),
            Err(CheckFormationError::ApplicabilityMismatch)
        );
    }

    #[test]
    fn all_accredited_aggregates_to_da() {
        let set = requirement_set();
        let checks: Vec<_> = set
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                check(
                    descriptor,
                    &format!("verifier:{index}"),
                    CheckResult::Accredited,
                )
            })
            .collect();

        assert_eq!(
            aggregate_requirement_checks(&set, &checks),
            Ok(CheckResult::Accredited)
        );
    }

    #[test]
    fn refutation_precedes_not_verifiable() {
        let set = requirement_set();
        let mut checks = Vec::new();
        for (index, descriptor) in set.iter().enumerate() {
            let result = match index {
                0 => CheckResult::NotVerifiable,
                1 => CheckResult::Refuted,
                _ => CheckResult::Accredited,
            };
            checks.push(check(descriptor, &format!("verifier:{index}"), result));
        }

        assert_eq!(
            aggregate_requirement_checks(&set, &checks),
            Ok(CheckResult::Refuted)
        );
    }

    #[test]
    fn dn_is_preserved_when_there_is_no_refutation() {
        let set = requirement_set();
        let mut checks = Vec::new();
        for (index, descriptor) in set.iter().enumerate() {
            let result = if index == 0 {
                CheckResult::NotVerifiable
            } else {
                CheckResult::Accredited
            };
            checks.push(check(descriptor, &format!("verifier:{index}"), result));
        }

        assert_eq!(
            aggregate_requirement_checks(&set, &checks),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn aggregation_requires_complete_coverage() {
        let set = requirement_set();
        let first = set.iter().next().unwrap();
        let checks = [check(first, "verifier:1", CheckResult::Accredited)];

        assert!(matches!(
            aggregate_requirement_checks(&set, &checks),
            Err(CheckAggregationError::MissingCheck(_))
        ));
    }

    #[test]
    fn aggregation_rejects_duplicate_check_for_same_requirement() {
        let set = requirement_set();
        let mut checks = Vec::new();
        for (index, descriptor) in set.iter().enumerate() {
            checks.push(check(
                descriptor,
                &format!("verifier:{index}"),
                CheckResult::Accredited,
            ));
        }
        let first = set.iter().next().unwrap();
        checks.push(check(
            first,
            "verifier:duplicate",
            CheckResult::Accredited,
        ));

        assert_eq!(
            aggregate_requirement_checks(&set, &checks),
            Err(CheckAggregationError::DuplicateCheck(
                first.reference().clone()
            ))
        );
    }

    #[test]
    fn aggregation_rejects_same_reference_with_different_material_binding() {
        let set = requirement_set();
        let first = set.iter().next().unwrap();
        let foreign = descriptor(
            first.reference().id().as_str(),
            first.class(),
            "form:other",
            "family:write",
            "context:1",
        );
        let foreign_check = check(&foreign, "verifier:foreign", CheckResult::Accredited);

        assert_eq!(
            aggregate_requirement_checks(&set, &[foreign_check]),
            Err(CheckAggregationError::BindingMismatch(
                first.reference().clone()
            ))
        );
    }
}
