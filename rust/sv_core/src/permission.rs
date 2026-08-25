//! Decisión sellada de permiso para R1-4.
//!
//! La unidad 1 materializa la frontera de decisión. La unidad 2 refuerza el
//! sello con las ligaduras gobernantes que deberán seguir vigentes en el punto
//! de mediación. Ninguna de las dos unidades ejecuta por sí misma un efecto
//! protegido.
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

use std::collections::{BTreeMap, BTreeSet};

use crate::authority::transitions::AuthorityContinuity;
use crate::authority::{AccumulationContract, ConstitutedAuthority, EffectDescriptor, FormDescriptor};
use crate::control::{
    ApplicabilityRuleRef, AuthorityHolderRef, AuthorityRef, CheckResult, ContextRef,
    EffectFamilyRef, EffectRef, FormRef, GovernedObjectRef, RequirementRef, TransitionClass,
    VerifierFamilyRef, VerifierRef,
};
use crate::requirements::RequirementSet;
use crate::requirements_bridge::ResolvedRequirementResult;
use crate::requirements_coverage::{
    aggregate_covered_requirement_results, CoveredAggregationError,
};
use crate::requirements_reuse::{
    reuse_historical_requirement_result, seal_historical_qualified_result,
    HistoricalQualificationError, HistoricalQualifiedRequirementResult, ReuseDisposition,
    ReuseRejectionReason,
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
    context_bindings: BTreeSet<ContextRef>,
    selected_context: ContextRef,
    required_authority: AuthorityRef,
    accumulation: AccumulationContract,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitApplicabilitySnapshot {
    verifier_family: VerifierFamilyRef,
    context: ContextRef,
    applicability_rule: ApplicabilityRuleRef,
}

fn historical_results(
    requirements: &RequirementSet,
    resolved_results: &[ResolvedRequirementResult],
) -> Result<
    BTreeMap<RequirementRef, HistoricalQualifiedRequirementResult>,
    PermitDecisionError,
> {
    let mut historical = BTreeMap::new();

    for result in resolved_results {
        let requirement = result.requirement();
        let descriptor = requirements.requirement(requirement).ok_or_else(|| {
            PermitDecisionError::MissingRequirementForHistoricalSeal(requirement.clone())
        })?;
        let sealed = seal_historical_qualified_result(descriptor, result)
            .map_err(PermitDecisionError::HistoricalQualification)?;
        historical.insert(requirement.clone(), sealed);
    }

    Ok(historical)
}

fn applicability_snapshots(
    continuity: &AuthorityContinuity,
    requirements: &RequirementSet,
    resolved_results: &[ResolvedRequirementResult],
) -> Result<
    BTreeMap<(RequirementRef, VerifierRef), PermitApplicabilitySnapshot>,
    PermitDecisionError,
> {
    let mut snapshots = BTreeMap::new();

    for result in resolved_results {
        let requirement = result.requirement();
        let descriptor = requirements.requirement(requirement).ok_or_else(|| {
            PermitDecisionError::MissingRequirementForApplicability(requirement.clone())
        })?;

        for verifier in result.participating_verifiers() {
            let applicability = continuity
                .verifier_applicability(requirement, verifier, descriptor.context())
                .ok_or_else(|| PermitDecisionError::MissingConstitutedApplicability {
                    requirement: requirement.clone(),
                    verifier: verifier.clone(),
                    context: descriptor.context().clone(),
                })?;

            snapshots.insert(
                (requirement.clone(), verifier.clone()),
                PermitApplicabilitySnapshot {
                    verifier_family: applicability.verifier_family().clone(),
                    context: applicability.context().clone(),
                    applicability_rule: applicability.applicability_rule().clone(),
                },
            );
        }
    }

    Ok(snapshots)
}

/// Permiso positivo sellado para un acto protegido concreto.
///
/// El tipo no implementa `Clone` ni ofrece constructor público. Sólo
/// `decide_permit` puede formarlo después de recuperar del estado constituido la
/// forma, la autoridad y `Req`, y de obtener un resultado técnico final `D-A`
/// mediante la agregación gobernada de R1-3.
///
/// Para la mediación se conservan resultados históricos ya cualificados por 3D
/// y las relaciones `Applicable(V,q,C)` de los verificadores participantes. El
/// sellado histórico no presume vigencia: la unidad 2 deberá reutilizar cada
/// resultado mediante la regla constituida de 3E.
#[derive(Debug, PartialEq, Eq)]
pub struct Permit {
    authority: PermitAuthorityBinding,
    form: PermitFormBinding,
    effect: EffectDescriptor,
    historical_results: BTreeMap<RequirementRef, HistoricalQualifiedRequirementResult>,
    applicabilities: BTreeMap<(RequirementRef, VerifierRef), PermitApplicabilitySnapshot>,
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
    pub fn authority_context(&self) -> &ContextRef {
        &self.authority.context
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
    pub fn form_effect_family(&self) -> &EffectFamilyRef {
        &self.form.effect_family
    }

    #[inline]
    pub fn required_authority(&self) -> &AuthorityRef {
        &self.form.required_authority
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
        &self.form.selected_context
    }

    #[inline]
    pub fn requirement_form(&self) -> &FormRef {
        &self.form.reference
    }

    #[inline]
    pub fn requirement_effect_family(&self) -> &EffectFamilyRef {
        self.effect.family()
    }

    #[inline]
    pub fn requirement_context(&self) -> &ContextRef {
        self.effect.context()
    }

    #[inline]
    pub const fn technical_result(&self) -> CheckResult {
        self.technical_result
    }

    #[inline]
    pub fn accumulation(&self) -> &AccumulationContract {
        &self.form.accumulation
    }

    #[inline]
    pub(crate) fn matches_current_form(&self, form: &FormDescriptor) -> bool {
        self.form.reference == *form.reference()
            && self.form.transition_class == form.transition_class()
            && self.form.effect_family == *form.effect_family()
            && self.form.context_bindings == form.context_bindings().cloned().collect()
            && form.context_bindings().any(|context| context == &self.form.selected_context)
            && form.requires_authority() == Some(&self.form.required_authority)
            && self.form.accumulation == *form.accumulation()
    }

    #[inline]
    pub(crate) fn matches_current_authority(&self, authority: &ConstitutedAuthority) -> bool {
        self.authority.reference == *authority.reference()
            && self.authority.holder == *authority.holder()
            && self.authority.context == *authority.context()
    }

    #[inline]
    pub(crate) fn historical_requirement_count(&self) -> usize {
        self.historical_results.len()
    }

    #[inline]
    pub(crate) fn has_historical_requirement(&self, requirement: &RequirementRef) -> bool {
        self.historical_results.contains_key(requirement)
    }

    pub(crate) fn first_non_reusable_requirement(
        &self,
        requirements: &RequirementSet,
    ) -> Option<(RequirementRef, Option<ReuseRejectionReason>)> {
        for descriptor in requirements.iter() {
            let Some(historical) = self.historical_results.get(descriptor.reference()) else {
                return Some((descriptor.reference().clone(), None));
            };

            let assessment = match reuse_historical_requirement_result(descriptor, historical) {
                Ok(assessment) => assessment,
                Err(_) => return Some((descriptor.reference().clone(), None)),
            };

            if assessment.disposition() != ReuseDisposition::Reused
                || assessment.result() != CheckResult::Accredited
            {
                return Some((descriptor.reference().clone(), assessment.rejection_reason()));
            }
        }

        None
    }

    pub(crate) fn first_changed_applicability(
        &self,
        continuity: &AuthorityContinuity,
    ) -> Option<(RequirementRef, VerifierRef, ContextRef)> {
        for ((requirement, verifier), snapshot) in &self.applicabilities {
            let Some(current) = continuity.verifier_applicability(
                requirement,
                verifier,
                &snapshot.context,
            ) else {
                return Some((
                    requirement.clone(),
                    verifier.clone(),
                    snapshot.context.clone(),
                ));
            };

            if current.verifier_family() != &snapshot.verifier_family
                || current.requirement() != requirement
                || current.context() != &snapshot.context
                || current.applicability_rule() != &snapshot.applicability_rule
            {
                return Some((
                    requirement.clone(),
                    verifier.clone(),
                    snapshot.context.clone(),
                ));
            }
        }

        None
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
    MissingRequirementForHistoricalSeal(RequirementRef),
    HistoricalQualification(HistoricalQualificationError),
    MissingRequirementForApplicability(RequirementRef),
    MissingConstitutedApplicability {
        requirement: RequirementRef,
        verifier: VerifierRef,
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
/// forma, familia, contexto, autoridad requerida, alcance constituido del
/// efecto y las relaciones de aplicabilidad constituidas de los verificadores
/// participantes. Los resultados por obligación se sellan como resultados
/// históricos cualificados para que la mediación posterior deba pasar por 3E.
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
        CheckResult::Accredited => {
            let historical_results = historical_results(requirements, resolved_results)?;
            let applicabilities =
                applicability_snapshots(continuity, requirements, resolved_results)?;

            Ok(PermitDecision::Granted(Permit {
                authority: PermitAuthorityBinding {
                    reference: authority.reference().clone(),
                    holder: authority.holder().clone(),
                    context: authority.context().clone(),
                },
                form: PermitFormBinding {
                    reference: form.reference().clone(),
                    transition_class: form.transition_class(),
                    effect_family: form.effect_family().clone(),
                    context_bindings: form.context_bindings().cloned().collect(),
                    selected_context: effect.context().clone(),
                    required_authority: required_authority.clone(),
                    accumulation: form.accumulation().clone(),
                },
                effect: effect.clone(),
                historical_results,
                applicabilities,
                technical_result,
            }))
        }
    }
}
