//! Decisión sellada de permiso para R1-4.
//!
//! Esta primera unidad materializa únicamente la frontera de decisión. No
//! ejecuta efectos protegidos ni hace productiva ninguna clase T-*.
//!
//! Un resultado técnico nominal no puede convertirse en permiso:
//!
//! ```compile_fail
//! use sv_core::{CheckResult, Permit};
//! let _permit: Permit = CheckResult::Accredited.into();
//! ```
//!
//! Tampoco existe un constructor público para fabricar el objeto sellado:
//!
//! ```compile_fail
//! use sv_core::Permit;
//! let _ = Permit::new();
//! ```

use crate::authority::transitions::AuthorityContinuity;
use crate::authority::{AccumulationContract, EffectDescriptor};
use crate::control::{
    AuthorityHolderRef, AuthorityRef, CheckResult, ContextRef, EffectFamilyRef, EffectRef, FormRef,
    GovernedObjectRef, TransitionClass,
};
use crate::requirements_bridge::ResolvedRequirementResult;
use crate::requirements_coverage::{
    aggregate_covered_requirement_results, CoveredAggregationError,
};

#[derive(Debug, PartialEq, Eq)]
struct PermitAuthorityBinding {
    reference: AuthorityRef,
    holder: AuthorityHolderRef,
    context: ContextRef,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitFormBinding {
    reference: FormRef,
    transition_class: TransitionClass,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    required_authority: AuthorityRef,
    accumulation: AccumulationContract,
}

/// Permiso positivo sellado para un acto protegido concreto.
///
/// El tipo no implementa `Clone` ni ofrece constructor público. Sólo
/// `decide_permit` puede formarlo después de recuperar del estado constituido la
/// forma, la autoridad y `Req`, y de obtener un resultado técnico final `D-A`
/// mediante la agregación gobernada de R1-3.
///
/// Esta unidad no ofrece todavía una operación que consuma el permiso para
/// ejecutar un efecto. La mediación productiva pertenece a una unidad posterior
/// de R1-4.
#[derive(Debug, PartialEq, Eq)]
pub struct Permit {
    authority: PermitAuthorityBinding,
    form: PermitFormBinding,
    effect: EffectDescriptor,
    requirement_form: FormRef,
    requirement_effect_family: EffectFamilyRef,
    requirement_context: ContextRef,
    technical_result: CheckResult,
}

impl Permit {
    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        &self.authority.reference
    }

    #[inline]
    pub fn authority_holder(&self) -> &AuthorityHolderRef {
        &self.authority.holder
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        &self.form.reference
    }

    #[inline]
    pub const fn transition_class(&self) -> TransitionClass {
        self.form.transition_class
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        &self.effect
    }

    #[inline]
    pub fn effect_reference(&self) -> &EffectRef {
        self.effect.reference()
    }

    #[inline]
    pub fn governed_object(&self) -> &GovernedObjectRef {
        self.effect.object()
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.form.context
    }

    #[inline]
    pub const fn technical_result(&self) -> CheckResult {
        self.technical_result
    }

    #[inline]
    pub fn accumulation(&self) -> &AccumulationContract {
        &self.form.accumulation
    }
}

/// Ausencia cerrada de permiso positivo por el resultado técnico final de
/// R1-3. Estas variantes no son `CheckResult` y no pertenecen a `Tri`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermitRejection {
    RefutedRequirements,
    NotVerifiableRequirements,
}

/// Resultado cerrado de la decisión de permiso.
///
/// `NotGranted` no es una autoridad negativa, no muta el estado y no ejecuta
/// efectos. `Granted` contiene el único objeto `Permit` que puede producir esta
/// unidad.
#[derive(Debug, PartialEq, Eq)]
pub enum PermitDecision {
    Granted(Permit),
    NotGranted(PermitRejection),
}

/// Entrada estructural inválida o ligadura incompatible durante la decisión.
/// Ninguna variante constituye `Tri.U` ni un resultado técnico de requisito.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermitDecisionError {
    UnknownForm(FormRef),
    FormWithoutRequiredAuthority(FormRef),
    UnknownRequiredAuthority(AuthorityRef),
    FormEffectMismatch {
        form: FormRef,
        effect: EffectRef,
    },
    EffectOutsideAuthorityScope {
        authority: AuthorityRef,
        effect: EffectRef,
    },
    MissingRequirementSet {
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
    },
    InvalidGovernedResult(CoveredAggregationError),
}

impl From<CoveredAggregationError> for PermitDecisionError {
    fn from(error: CoveredAggregationError) -> Self {
        Self::InvalidGovernedResult(error)
    }
}

/// Decide si puede formarse un permiso positivo para un efecto protegido.
///
/// La operación no recibe `CheckResult`, autoridad aplicable, pertenencia a
/// `E_max`, pertenencia a `D_a` ni cobertura como parámetros elegibles por el
/// llamador. Recupera forma, autoridad y `Req` de la continuidad constituida y
/// vuelve a obtener el resultado final de R1-3 para la ligadura exacta del acto.
///
/// Un `D-A` final es necesario, pero sólo produce `Permit` si además coinciden
/// forma, familia, contexto, autoridad requerida y alcance constituido del
/// efecto. `D-R` y `D-N` producen ausencia cerrada de permiso positivo.
pub fn decide_permit(
    continuity: &AuthorityContinuity,
    form_reference: &FormRef,
    effect: &EffectDescriptor,
    resolved_results: &[ResolvedRequirementResult],
) -> Result<PermitDecision, PermitDecisionError> {
    let form = continuity
        .form(form_reference)
        .ok_or_else(|| PermitDecisionError::UnknownForm(form_reference.clone()))?;

    let required_authority = form.requires_authority().ok_or_else(|| {
        PermitDecisionError::FormWithoutRequiredAuthority(form.reference().clone())
    })?;

    let authority = continuity.authority(required_authority).ok_or_else(|| {
        PermitDecisionError::UnknownRequiredAuthority(required_authority.clone())
    })?;

    if !form.describes_effect(effect) {
        return Err(PermitDecisionError::FormEffectMismatch {
            form: form.reference().clone(),
            effect: effect.reference().clone(),
        });
    }

    if !authority.contains_effect_scope(effect) {
        return Err(PermitDecisionError::EffectOutsideAuthorityScope {
            authority: authority.reference().clone(),
            effect: effect.reference().clone(),
        });
    }

    let requirements = continuity
        .requirement_set(form.reference(), effect.family(), effect.context())
        .ok_or_else(|| PermitDecisionError::MissingRequirementSet {
            form: form.reference().clone(),
            effect_family: effect.family().clone(),
            context: effect.context().clone(),
        })?;

    let technical_result = aggregate_covered_requirement_results(requirements, resolved_results)?;

    match technical_result {
        CheckResult::Refuted => Ok(PermitDecision::NotGranted(
            PermitRejection::RefutedRequirements,
        )),
        CheckResult::NotVerifiable => Ok(PermitDecision::NotGranted(
            PermitRejection::NotVerifiableRequirements,
        )),
        CheckResult::Accredited => Ok(PermitDecision::Granted(Permit {
            authority: PermitAuthorityBinding {
                reference: authority.reference().clone(),
                holder: authority.holder().clone(),
                context: authority.context().clone(),
            },
            form: PermitFormBinding {
                reference: form.reference().clone(),
                transition_class: form.transition_class(),
                effect_family: form.effect_family().clone(),
                context: effect.context().clone(),
                required_authority: required_authority.clone(),
                accumulation: form.accumulation().clone(),
            },
            effect: effect.clone(),
            requirement_form: requirements.form().clone(),
            requirement_effect_family: requirements.effect_family().clone(),
            requirement_context: requirements.context().clone(),
            technical_result,
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::transitions::{
        transition_disposition, AuthorityProposal, EffectProposal, ExternalGenesisPremise,
        FormProposal, GenesisPlan, TransitionDisposition,
    };
    use crate::control::{
        ApplicabilityRuleRef, ControlId, CoverageRuleRef, RequirementRef, VerifierFamilyRef,
        VerifierRef,
    };
    use crate::requirements::initial::{
        ApplicabilityProposal, CoverageRuleProposal, RequirementProposal,
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

    fn family_ref(value: &str) -> EffectFamilyRef {
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

    fn verifier_for(requirement: &RequirementRef, suffix: &str) -> VerifierRef {
        verifier_ref(&format!("verifier:{}:{suffix}", requirement.id().as_str()))
    }

    fn controlled_continuity() -> AuthorityContinuity {
        let form = form_ref("form:exercise");
        let family = family_ref("family:write");
        let context = context_ref("context:1");
        let authority = authority_ref("authority:main");
        let verifier_family = verifier_family_ref("verifier-family:canonical");
        let applicability_rule = applicability_rule_ref("applicability:canonical");

        let mandatory = [
            ("req:form", CoreRequirementKind::FormValidity),
            ("req:authority", CoreRequirementKind::ApplicableAuthority),
            (
                "req:verifier",
                CoreRequirementKind::VerifierAdmissibilityAndApplicability,
            ),
            ("req:no-self", CoreRequirementKind::NoSelfAccreditation),
        ];

        let mut requirements = Vec::new();
        let mut applicabilities = Vec::new();
        for (reference, kind) in mandatory {
            let requirement = requirement_ref(reference);
            let primary = verifier_for(&requirement, "primary");
            let alternate = verifier_for(&requirement, "alternate");
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

        let plan = GenesisPlan::new(
            [
                FormProposal::new(
                    form.clone(),
                    TransitionClass::Exercise,
                    family.clone(),
                    [context.clone()],
                    Some(authority.clone()),
                    AccumulationContract::SingleUse,
                ),
                FormProposal::new(
                    form_ref("form:free"),
                    TransitionClass::Information,
                    family.clone(),
                    [context.clone()],
                    None,
                    AccumulationContract::NotApplicable,
                ),
            ],
            [
                AuthorityProposal::new(
                    authority.clone(),
                    holder_ref("holder:main"),
                    context.clone(),
                    [
                        EffectProposal::new(
                            effect_ref("effect:allowed"),
                            family.clone(),
                            object_ref("object:main"),
                            context.clone(),
                        ),
                        EffectProposal::new(
                            effect_ref("effect:wrong-family"),
                            family_ref("family:delete"),
                            object_ref("object:main"),
                            context.clone(),
                        ),
                    ],
                    [object_ref("object:main")],
                ),
                AuthorityProposal::new(
                    authority_ref("authority:other"),
                    holder_ref("holder:other"),
                    context.clone(),
                    [EffectProposal::new(
                        effect_ref("effect:foreign"),
                        family.clone(),
                        object_ref("object:foreign"),
                        context.clone(),
                    )],
                    [object_ref("object:foreign")],
                ),
            ],
        )
        .with_initial_control(requirements, applicabilities);

        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();
        continuity.apply_genesis(&mut premise, plan).unwrap();
        continuity
    }

    fn requirement_set(continuity: &AuthorityContinuity) -> &RequirementSet {
        continuity
            .requirement_set(
                &form_ref("form:exercise"),
                &family_ref("family:write"),
                &context_ref("context:1"),
            )
            .unwrap()
    }

    fn resolved_results(
        continuity: &AuthorityContinuity,
        first_result: CheckResult,
        use_alternate_for_first: bool,
    ) -> Vec<ResolvedRequirementResult> {
        let set = requirement_set(continuity);
        set.iter()
            .enumerate()
            .map(|(index, descriptor)| {
                let suffix = if index == 0 && use_alternate_for_first {
                    "alternate"
                } else {
                    "primary"
                };
                let verifier = verifier_for(descriptor.reference(), suffix);
                let applicability = continuity
                    .verifier_applicability(
                        descriptor.reference(),
                        &verifier,
                        descriptor.context(),
                    )
                    .unwrap();
                let result = if index == 0 {
                    first_result
                } else {
                    CheckResult::Accredited
                };
                let check =
                    RequirementCheck::constitute_for_test(descriptor, applicability, result)
                        .unwrap();
                resolve_requirement_result(descriptor, &[&check]).unwrap()
            })
            .collect()
    }

    fn authority_effect<'a>(
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

    #[test]
    fn complete_governed_da_forms_a_sealed_permit() {
        let continuity = controlled_continuity();
        let form = form_ref("form:exercise");
        let authority = authority_ref("authority:main");
        let effect = authority_effect(&continuity, &authority, "effect:allowed");
        let results = resolved_results(&continuity, CheckResult::Accredited, false);

        let decision = decide_permit(&continuity, &form, effect, &results).unwrap();
        let PermitDecision::Granted(permit) = decision else {
            panic!("la decisión debía conceder un permiso sellado");
        };

        assert_eq!(permit.authority(), &authority);
        assert_eq!(permit.authority_holder(), &holder_ref("holder:main"));
        assert_eq!(permit.form(), &form);
        assert_eq!(permit.transition_class(), TransitionClass::Exercise);
        assert_eq!(permit.effect_reference(), &effect_ref("effect:allowed"));
        assert_eq!(permit.governed_object(), &object_ref("object:main"));
        assert_eq!(permit.context(), &context_ref("context:1"));
        assert_eq!(permit.technical_result(), CheckResult::Accredited);
        assert_eq!(permit.accumulation(), &AccumulationContract::SingleUse);
    }

    #[test]
    fn refuted_requirements_never_form_positive_permit() {
        let continuity = controlled_continuity();
        let form = form_ref("form:exercise");
        let authority = authority_ref("authority:main");
        let effect = authority_effect(&continuity, &authority, "effect:allowed");
        let results = resolved_results(&continuity, CheckResult::Refuted, false);

        assert_eq!(
            decide_permit(&continuity, &form, effect, &results),
            Ok(PermitDecision::NotGranted(
                PermitRejection::RefutedRequirements
            ))
        );
    }

    #[test]
    fn not_verifiable_requirements_never_form_positive_permit() {
        let continuity = controlled_continuity();
        let form = form_ref("form:exercise");
        let authority = authority_ref("authority:main");
        let effect = authority_effect(&continuity, &authority, "effect:allowed");
        let results = resolved_results(&continuity, CheckResult::NotVerifiable, false);

        assert_eq!(
            decide_permit(&continuity, &form, effect, &results),
            Ok(PermitDecision::NotGranted(
                PermitRejection::NotVerifiableRequirements
            ))
        );
    }

    #[test]
    fn accredited_subset_without_required_coverage_never_forms_permit() {
        let continuity = controlled_continuity();
        let form = form_ref("form:exercise");
        let authority = authority_ref("authority:main");
        let effect = authority_effect(&continuity, &authority, "effect:allowed");
        let results = resolved_results(&continuity, CheckResult::Accredited, true);

        assert_eq!(
            decide_permit(&continuity, &form, effect, &results),
            Ok(PermitDecision::NotGranted(
                PermitRejection::NotVerifiableRequirements
            ))
        );
    }

    #[test]
    fn form_effect_mismatch_is_a_closed_error() {
        let continuity = controlled_continuity();
        let form = form_ref("form:exercise");
        let authority = authority_ref("authority:main");
        let effect = authority_effect(&continuity, &authority, "effect:wrong-family");
        let results = resolved_results(&continuity, CheckResult::Accredited, false);

        assert_eq!(
            decide_permit(&continuity, &form, effect, &results),
            Err(PermitDecisionError::FormEffectMismatch {
                form,
                effect: effect_ref("effect:wrong-family"),
            })
        );
    }

    #[test]
    fn effect_outside_required_authority_scope_is_a_closed_error() {
        let continuity = controlled_continuity();
        let form = form_ref("form:exercise");
        let main_authority = authority_ref("authority:main");
        let other_authority = authority_ref("authority:other");
        let effect = authority_effect(&continuity, &other_authority, "effect:foreign");
        let results = resolved_results(&continuity, CheckResult::Accredited, false);

        assert_eq!(
            decide_permit(&continuity, &form, effect, &results),
            Err(PermitDecisionError::EffectOutsideAuthorityScope {
                authority: main_authority,
                effect: effect_ref("effect:foreign"),
            })
        );
    }

    #[test]
    fn form_without_required_authority_cannot_form_permit() {
        let continuity = controlled_continuity();
        let form = form_ref("form:free");
        let authority = authority_ref("authority:main");
        let effect = authority_effect(&continuity, &authority, "effect:allowed");

        assert_eq!(
            decide_permit(&continuity, &form, effect, &[]),
            Err(PermitDecisionError::FormWithoutRequiredAuthority(form))
        );
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
}
