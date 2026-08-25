//! Reutilización ligada de resultados técnicos cualificados de R1-3.
//!
//! Esta unidad no presume vigencia por cronología. Un resultado histórico sólo
//! puede conservarse cuando ya fue cualificado por la cobertura de 3D y una
//! regla constituida mantiene exactamente las ligaduras causales exigidas.
//! La ausencia o variación de esas ligaduras cierra en `D-N` para el acto
//! actual; nunca produce `Tri::U`, permiso ni autoridad.

use std::collections::{BTreeMap, BTreeSet};

use crate::authority::transitions::GenesisControlToken;
use crate::control::{
    ApplicabilityRuleRef, CheckResult, ConflictResolutionRuleRef, ContextRef, CoverageRuleRef,
    EffectFamilyRef, FormRef, RequirementRef, ReuseBindingKeyRef, ReuseBindingValueRef,
    ReuseRuleRef, VerifierFamilyRef, VerifierRef,
};
use crate::requirements::{RequirementClass, RequirementDescriptor};
use crate::requirements_bridge::ResolvedRequirementResult;
use crate::requirements_coverage::{
    assess_requirement_coverage, CoverageAssessmentError, CoverageDisposition,
};

/// Regla constituida de reutilización histórica para una obligación.
///
/// La primera realización conserva un conjunto exacto y no vacío de pares
/// dimensión/valor. Una dimensión identifica qué condición debe continuar; el
/// valor identifica el estado constituido que debe permanecer sin cambio.
#[derive(Debug, PartialEq, Eq)]
pub struct ReuseRule {
    reference: ReuseRuleRef,
    requirement: RequirementRef,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    exact_bindings: BTreeMap<ReuseBindingKeyRef, ReuseBindingValueRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReuseRuleFormationError {
    EmptyBindingSet,
    DuplicateBindingKey(ReuseBindingKeyRef),
}

impl ReuseRule {
    pub(crate) fn constitute_from_genesis(
        _token: &GenesisControlToken,
        reference: ReuseRuleRef,
        descriptor: &RequirementDescriptor,
        exact_bindings: BTreeMap<ReuseBindingKeyRef, ReuseBindingValueRef>,
    ) -> Self {
        debug_assert!(!exact_bindings.is_empty());
        Self {
            reference,
            requirement: descriptor.reference().clone(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            exact_bindings,
        }
    }

    #[cfg(test)]
    pub(crate) fn constitute_for_test(
        reference: ReuseRuleRef,
        descriptor: &RequirementDescriptor,
        bindings: impl IntoIterator<Item = (ReuseBindingKeyRef, ReuseBindingValueRef)>,
    ) -> Result<Self, ReuseRuleFormationError> {
        let mut exact_bindings = BTreeMap::new();
        for (key, value) in bindings {
            if exact_bindings.insert(key.clone(), value).is_some() {
                return Err(ReuseRuleFormationError::DuplicateBindingKey(key));
            }
        }
        if exact_bindings.is_empty() {
            return Err(ReuseRuleFormationError::EmptyBindingSet);
        }

        Ok(Self {
            reference,
            requirement: descriptor.reference().clone(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            exact_bindings,
        })
    }

    #[inline]
    pub fn reference(&self) -> &ReuseRuleRef {
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
    pub fn bindings(
        &self,
    ) -> impl Iterator<Item = (&ReuseBindingKeyRef, &ReuseBindingValueRef)> {
        self.exact_bindings.iter()
    }

    #[inline]
    pub(crate) fn matches_descriptor(&self, descriptor: &RequirementDescriptor) -> bool {
        self.requirement == *descriptor.reference()
            && self.form == *descriptor.form()
            && self.effect_family == *descriptor.effect_family()
            && self.context == *descriptor.context()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConflictRuleSnapshot {
    reference: ConflictResolutionRuleRef,
    decisive_verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    applicability_rule: ApplicabilityRuleRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CoverageRuleSnapshot {
    reference: CoverageRuleRef,
    required_verifiers: BTreeSet<VerifierRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReuseRuleSnapshot {
    reference: ReuseRuleRef,
    exact_bindings: BTreeMap<ReuseBindingKeyRef, ReuseBindingValueRef>,
}

/// Resultado de una obligación ya resuelto y cualificado por cobertura, sellado
/// para una posible reutilización posterior.
///
/// No existe constructor público. El único sellado productivo recalcula la
/// cualificación de 3D a partir de un `ResolvedRequirementResult` cuya ligadura
/// completa coincide con el descriptor constituido.
///
/// ```compile_fail
/// use sv_core::{CheckResult, HistoricalQualifiedRequirementResult};
/// let _ = HistoricalQualifiedRequirementResult::new(CheckResult::Accredited);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct HistoricalQualifiedRequirementResult {
    requirement: RequirementRef,
    class: RequirementClass,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    admissible_verifier_families: BTreeSet<VerifierFamilyRef>,
    applicability_rule: ApplicabilityRuleRef,
    conflict_rule: Option<ConflictRuleSnapshot>,
    coverage_rule: Option<CoverageRuleSnapshot>,
    reuse_rule: Option<ReuseRuleSnapshot>,
    participating_verifiers: BTreeSet<VerifierRef>,
    result: CheckResult,
}

impl HistoricalQualifiedRequirementResult {
    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub const fn result(&self) -> CheckResult {
        self.result
    }

    #[inline]
    pub fn participating_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.participating_verifiers.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoricalQualificationError {
    RequirementMismatch {
        expected: RequirementRef,
        found: RequirementRef,
    },
    ResolvedBindingMismatch(RequirementRef),
    Coverage(CoverageAssessmentError),
}

impl From<CoverageAssessmentError> for HistoricalQualificationError {
    fn from(error: CoverageAssessmentError) -> Self {
        Self::Coverage(error)
    }
}

/// Sella el resultado cualificado de un acto para una posible reutilización
/// posterior.
///
/// El valor histórico almacena el resultado posterior a 3D, no el resultado
/// resuelto bruto de 3C. Por tanto una acreditación con cobertura insuficiente
/// queda sellada como `D-N` y no puede reaparecer después como `D-A`.
pub fn seal_historical_qualified_result(
    descriptor: &RequirementDescriptor,
    resolved: &ResolvedRequirementResult,
) -> Result<HistoricalQualifiedRequirementResult, HistoricalQualificationError> {
    if resolved.requirement() != descriptor.reference() {
        return Err(HistoricalQualificationError::RequirementMismatch {
            expected: descriptor.reference().clone(),
            found: resolved.requirement().clone(),
        });
    }
    if !resolved.matches_descriptor(descriptor) {
        return Err(HistoricalQualificationError::ResolvedBindingMismatch(
            descriptor.reference().clone(),
        ));
    }

    let assessment = assess_requirement_coverage(descriptor, resolved)?;
    let result = match resolved.result() {
        CheckResult::Refuted => CheckResult::Refuted,
        CheckResult::Accredited if assessment.disposition() == CoverageDisposition::Complete => {
            CheckResult::Accredited
        }
        CheckResult::Accredited | CheckResult::NotVerifiable => CheckResult::NotVerifiable,
    };

    let conflict_rule = descriptor.conflict_resolution_rule().map(|rule| ConflictRuleSnapshot {
        reference: rule.reference().clone(),
        decisive_verifier: rule.decisive_verifier().clone(),
        verifier_family: rule.verifier_family().clone(),
        applicability_rule: rule.applicability_rule().clone(),
    });

    let coverage_rule = descriptor.coverage_rule().map(|rule| CoverageRuleSnapshot {
        reference: rule.reference().clone(),
        required_verifiers: rule.required_verifiers().cloned().collect(),
    });

    let reuse_rule = descriptor.reuse_rule().map(|rule| ReuseRuleSnapshot {
        reference: rule.reference().clone(),
        exact_bindings: rule
            .bindings()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    });

    Ok(HistoricalQualifiedRequirementResult {
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
        conflict_rule,
        coverage_rule,
        reuse_rule,
        participating_verifiers: resolved.participating_verifiers().cloned().collect(),
        result,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReuseDisposition {
    Reused,
    NotReusable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReuseRejectionReason {
    MissingHistoricalRule,
    MissingCurrentRule,
    MaterialBindingChanged,
    RuleOrBindingChanged,
}

/// Resultado trazable de intentar reutilizar un resultado histórico.
///
/// `NotReusable` siempre produce `D-N` para el acto actual. El resultado
/// histórico original permanece inmutable.
#[derive(Debug, PartialEq, Eq)]
pub struct ReuseAssessment {
    requirement: RequirementRef,
    disposition: ReuseDisposition,
    result: CheckResult,
    rejection_reason: Option<ReuseRejectionReason>,
}

impl ReuseAssessment {
    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub const fn disposition(&self) -> ReuseDisposition {
        self.disposition
    }

    #[inline]
    pub const fn result(&self) -> CheckResult {
        self.result
    }

    #[inline]
    pub const fn rejection_reason(&self) -> Option<ReuseRejectionReason> {
        self.rejection_reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReuseAssessmentError {
    RequirementMismatch {
        expected: RequirementRef,
        found: RequirementRef,
    },
}

fn current_conflict_snapshot(descriptor: &RequirementDescriptor) -> Option<ConflictRuleSnapshot> {
    descriptor.conflict_resolution_rule().map(|rule| ConflictRuleSnapshot {
        reference: rule.reference().clone(),
        decisive_verifier: rule.decisive_verifier().clone(),
        verifier_family: rule.verifier_family().clone(),
        applicability_rule: rule.applicability_rule().clone(),
    })
}

fn current_coverage_snapshot(descriptor: &RequirementDescriptor) -> Option<CoverageRuleSnapshot> {
    descriptor.coverage_rule().map(|rule| CoverageRuleSnapshot {
        reference: rule.reference().clone(),
        required_verifiers: rule.required_verifiers().cloned().collect(),
    })
}

fn current_reuse_snapshot(descriptor: &RequirementDescriptor) -> Option<ReuseRuleSnapshot> {
    descriptor.reuse_rule().map(|rule| ReuseRuleSnapshot {
        reference: rule.reference().clone(),
        exact_bindings: rule
            .bindings()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    })
}

/// Evalúa la reutilización histórica contra el descriptor constituido actual.
///
/// No recibe regla, ligaduras ni tiempo como parámetros libres. La regla y los
/// valores actuales se obtienen exclusivamente del descriptor. Si la
/// continuidad material no puede acreditarse, la salida del acto actual es
/// `D-N`; la función nunca promociona el resultado histórico.
///
/// ```compile_fail
/// use sv_core::{reuse_historical_requirement_result, RequirementDescriptor, ResolvedRequirementResult};
/// fn invalid(descriptor: &RequirementDescriptor, resolved: &ResolvedRequirementResult) {
///     let _ = reuse_historical_requirement_result(descriptor, resolved);
/// }
/// ```
pub fn reuse_historical_requirement_result(
    current: &RequirementDescriptor,
    historical: &HistoricalQualifiedRequirementResult,
) -> Result<ReuseAssessment, ReuseAssessmentError> {
    if historical.requirement != *current.reference() {
        return Err(ReuseAssessmentError::RequirementMismatch {
            expected: current.reference().clone(),
            found: historical.requirement.clone(),
        });
    }

    let current_reuse = current_reuse_snapshot(current);
    let Some(historical_reuse) = historical.reuse_rule.as_ref() else {
        return Ok(ReuseAssessment {
            requirement: current.reference().clone(),
            disposition: ReuseDisposition::NotReusable,
            result: CheckResult::NotVerifiable,
            rejection_reason: Some(ReuseRejectionReason::MissingHistoricalRule),
        });
    };
    let Some(current_reuse) = current_reuse else {
        return Ok(ReuseAssessment {
            requirement: current.reference().clone(),
            disposition: ReuseDisposition::NotReusable,
            result: CheckResult::NotVerifiable,
            rejection_reason: Some(ReuseRejectionReason::MissingCurrentRule),
        });
    };

    let current_families: BTreeSet<_> = current.admissible_verifier_families().cloned().collect();
    let material_binding_matches = historical.class == current.class()
        && historical.form == *current.form()
        && historical.effect_family == *current.effect_family()
        && historical.context == *current.context()
        && historical.admissible_verifier_families == current_families
        && historical.applicability_rule == *current.applicability_rule()
        && historical.conflict_rule == current_conflict_snapshot(current)
        && historical.coverage_rule == current_coverage_snapshot(current);

    if !material_binding_matches {
        return Ok(ReuseAssessment {
            requirement: current.reference().clone(),
            disposition: ReuseDisposition::NotReusable,
            result: CheckResult::NotVerifiable,
            rejection_reason: Some(ReuseRejectionReason::MaterialBindingChanged),
        });
    }

    if historical_reuse != &current_reuse {
        return Ok(ReuseAssessment {
            requirement: current.reference().clone(),
            disposition: ReuseDisposition::NotReusable,
            result: CheckResult::NotVerifiable,
            rejection_reason: Some(ReuseRejectionReason::RuleOrBindingChanged),
        });
    }

    Ok(ReuseAssessment {
        requirement: current.reference().clone(),
        disposition: ReuseDisposition::Reused,
        result: historical.result,
        rejection_reason: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::{ControlId, VerifierFamilyRef};
    use crate::requirements::{RequirementClass, RequirementDescriptor, VerifierApplicability};
    use crate::requirements_bridge::resolve_requirement_result;
    use crate::requirements_coverage::CoverageRule;
    use crate::RequirementCheck;

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

    fn applicability(value: &str) -> ApplicabilityRuleRef {
        ApplicabilityRuleRef::from_core_id(id(value))
    }

    fn coverage(value: &str) -> CoverageRuleRef {
        CoverageRuleRef::from_core_id(id(value))
    }

    fn reuse_rule_ref(value: &str) -> ReuseRuleRef {
        ReuseRuleRef::from_core_id(id(value))
    }

    fn binding_key(value: &str) -> ReuseBindingKeyRef {
        ReuseBindingKeyRef::from_core_id(id(value))
    }

    fn binding_value(value: &str) -> ReuseBindingValueRef {
        ReuseBindingValueRef::from_core_id(id(value))
    }

    fn descriptor(reference: &str, context_value: &str) -> RequirementDescriptor {
        RequirementDescriptor::constitute_for_test(
            requirement(reference),
            RequirementClass::Specific,
            form("form:1"),
            family("family:write"),
            context(context_value),
            [verifier_family("verifier-family:canonical")],
            applicability("applicability:canonical"),
        )
        .unwrap()
    }

    fn attach_coverage(descriptor: &mut RequirementDescriptor, verifier_value: &str) {
        let rule = CoverageRule::constitute_for_test(
            coverage("coverage:1"),
            descriptor,
            [verifier(verifier_value)],
        )
        .unwrap();
        descriptor.attach_coverage_rule_for_test(rule);
    }

    fn attach_reuse(
        descriptor: &mut RequirementDescriptor,
        rule_ref: &str,
        bindings: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) {
        let rule = ReuseRule::constitute_for_test(
            reuse_rule_ref(rule_ref),
            descriptor,
            bindings
                .into_iter()
                .map(|(key, value)| (binding_key(key), binding_value(value))),
        )
        .unwrap();
        descriptor.attach_reuse_rule_for_test(rule);
    }

    fn resolved(
        descriptor: &RequirementDescriptor,
        verifier_value: &str,
        result: CheckResult,
    ) -> ResolvedRequirementResult {
        let applicability_relation = VerifierApplicability::constitute_for_test(
            verifier(verifier_value),
            verifier_family("verifier-family:canonical"),
            descriptor.reference().clone(),
            descriptor.context().clone(),
            applicability("applicability:canonical"),
        );
        let check = RequirementCheck::constitute_for_test(
            descriptor,
            &applicability_relation,
            result,
        )
        .unwrap();
        resolve_requirement_result(descriptor, &[&check]).unwrap()
    }

    fn reusable_descriptor(reference: &str) -> RequirementDescriptor {
        let mut descriptor = descriptor(reference, "context:1");
        attach_coverage(&mut descriptor, "verifier:1");
        attach_reuse(
            &mut descriptor,
            "reuse:1",
            [("binding:regime", "value:v1"), ("binding:validity", "value:active")],
        );
        descriptor
    }

    #[test]
    fn reuse_rule_rejects_empty_binding_set() {
        let descriptor = descriptor("req:1", "context:1");
        assert_eq!(
            ReuseRule::constitute_for_test(reuse_rule_ref("reuse:1"), &descriptor, []),
            Err(ReuseRuleFormationError::EmptyBindingSet)
        );
    }

    #[test]
    fn reuse_rule_rejects_duplicate_binding_key() {
        let descriptor = descriptor("req:1", "context:1");
        assert_eq!(
            ReuseRule::constitute_for_test(
                reuse_rule_ref("reuse:1"),
                &descriptor,
                [
                    (binding_key("binding:regime"), binding_value("value:v1")),
                    (binding_key("binding:regime"), binding_value("value:v2")),
                ],
            ),
            Err(ReuseRuleFormationError::DuplicateBindingKey(binding_key(
                "binding:regime"
            )))
        );
    }

    #[test]
    fn incomplete_coverage_cannot_reappear_as_historical_da() {
        let mut descriptor = descriptor("req:1", "context:1");
        attach_coverage(&mut descriptor, "verifier:required");
        attach_reuse(
            &mut descriptor,
            "reuse:1",
            [("binding:regime", "value:v1")],
        );
        let raw = resolved(&descriptor, "verifier:other", CheckResult::Accredited);
        let historical = seal_historical_qualified_result(&descriptor, &raw).unwrap();

        assert_eq!(historical.result(), CheckResult::NotVerifiable);
        assert_eq!(
            reuse_historical_requirement_result(&descriptor, &historical)
                .unwrap()
                .result(),
            CheckResult::NotVerifiable
        );
    }

    #[test]
    fn exact_continuity_preserves_da() {
        let descriptor = reusable_descriptor("req:1");
        let raw = resolved(&descriptor, "verifier:1", CheckResult::Accredited);
        let historical = seal_historical_qualified_result(&descriptor, &raw).unwrap();
        let assessment = reuse_historical_requirement_result(&descriptor, &historical).unwrap();

        assert_eq!(assessment.disposition(), ReuseDisposition::Reused);
        assert_eq!(assessment.result(), CheckResult::Accredited);
    }

    #[test]
    fn exact_continuity_preserves_dr() {
        let descriptor = reusable_descriptor("req:1");
        let raw = resolved(&descriptor, "verifier:1", CheckResult::Refuted);
        let historical = seal_historical_qualified_result(&descriptor, &raw).unwrap();
        assert_eq!(
            reuse_historical_requirement_result(&descriptor, &historical)
                .unwrap()
                .result(),
            CheckResult::Refuted
        );
    }

    #[test]
    fn exact_continuity_preserves_dn_without_promotion() {
        let descriptor = reusable_descriptor("req:1");
        let raw = resolved(&descriptor, "verifier:1", CheckResult::NotVerifiable);
        let historical = seal_historical_qualified_result(&descriptor, &raw).unwrap();
        assert_eq!(
            reuse_historical_requirement_result(&descriptor, &historical)
                .unwrap()
                .result(),
            CheckResult::NotVerifiable
        );
    }

    #[test]
    fn missing_current_rule_closes_to_dn() {
        let historical_descriptor = reusable_descriptor("req:1");
        let raw = resolved(
            &historical_descriptor,
            "verifier:1",
            CheckResult::Accredited,
        );
        let historical =
            seal_historical_qualified_result(&historical_descriptor, &raw).unwrap();

        let mut current = descriptor("req:1", "context:1");
        attach_coverage(&mut current, "verifier:1");
        let assessment = reuse_historical_requirement_result(&current, &historical).unwrap();

        assert_eq!(assessment.disposition(), ReuseDisposition::NotReusable);
        assert_eq!(assessment.result(), CheckResult::NotVerifiable);
        assert_eq!(
            assessment.rejection_reason(),
            Some(ReuseRejectionReason::MissingCurrentRule)
        );
    }

    #[test]
    fn changed_single_binding_closes_to_dn() {
        let historical_descriptor = reusable_descriptor("req:1");
        let raw = resolved(
            &historical_descriptor,
            "verifier:1",
            CheckResult::Accredited,
        );
        let historical =
            seal_historical_qualified_result(&historical_descriptor, &raw).unwrap();

        let mut current = descriptor("req:1", "context:1");
        attach_coverage(&mut current, "verifier:1");
        attach_reuse(
            &mut current,
            "reuse:1",
            [("binding:regime", "value:v2"), ("binding:validity", "value:active")],
        );
        let assessment = reuse_historical_requirement_result(&current, &historical).unwrap();

        assert_eq!(assessment.result(), CheckResult::NotVerifiable);
        assert_eq!(
            assessment.rejection_reason(),
            Some(ReuseRejectionReason::RuleOrBindingChanged)
        );
    }

    #[test]
    fn same_rule_reference_with_changed_binding_is_not_reusable() {
        let historical_descriptor = reusable_descriptor("req:1");
        let raw = resolved(
            &historical_descriptor,
            "verifier:1",
            CheckResult::Accredited,
        );
        let historical =
            seal_historical_qualified_result(&historical_descriptor, &raw).unwrap();

        let mut current = descriptor("req:1", "context:1");
        attach_coverage(&mut current, "verifier:1");
        attach_reuse(
            &mut current,
            "reuse:1",
            [("binding:regime", "value:v1"), ("binding:validity", "value:revoked")],
        );

        assert_eq!(
            reuse_historical_requirement_result(&current, &historical)
                .unwrap()
                .result(),
            CheckResult::NotVerifiable
        );
    }

    #[test]
    fn changed_context_closes_to_dn() {
        let historical_descriptor = reusable_descriptor("req:1");
        let raw = resolved(
            &historical_descriptor,
            "verifier:1",
            CheckResult::Accredited,
        );
        let historical =
            seal_historical_qualified_result(&historical_descriptor, &raw).unwrap();

        let mut current = descriptor("req:1", "context:other");
        attach_coverage(&mut current, "verifier:1");
        attach_reuse(
            &mut current,
            "reuse:1",
            [("binding:regime", "value:v1"), ("binding:validity", "value:active")],
        );

        let assessment = reuse_historical_requirement_result(&current, &historical).unwrap();
        assert_eq!(assessment.result(), CheckResult::NotVerifiable);
        assert_eq!(
            assessment.rejection_reason(),
            Some(ReuseRejectionReason::MaterialBindingChanged)
        );
    }

    #[test]
    fn other_requirement_is_structural_error() {
        let descriptor = reusable_descriptor("req:1");
        let raw = resolved(&descriptor, "verifier:1", CheckResult::Accredited);
        let historical = seal_historical_qualified_result(&descriptor, &raw).unwrap();

        let current = reusable_descriptor("req:2");
        assert_eq!(
            reuse_historical_requirement_result(&current, &historical),
            Err(ReuseAssessmentError::RequirementMismatch {
                expected: requirement("req:2"),
                found: requirement("req:1"),
            })
        );
    }
}
