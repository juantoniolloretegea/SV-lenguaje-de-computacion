//! Decisión sellada de permiso para R1-4.
//!
//! La unidad 1 materializa la frontera de decisión. La unidad 2 refuerza el
//! sello con una instantánea de las ligaduras gobernantes que deberán seguir
//! vigentes en el punto de mediación. Ninguna de las dos unidades ejecuta por
//! sí misma un efecto protegido.
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
    ApplicabilityRuleRef, AuthorityHolderRef, AuthorityRef, CheckResult, ConflictResolutionRuleRef,
    ContextRef, CoverageRuleRef, EffectFamilyRef, EffectRef, FormRef, GovernedObjectRef,
    RequirementRef, ReuseBindingKeyRef, ReuseBindingValueRef, ReuseRuleRef, TransitionClass,
    VerifierFamilyRef, VerifierRef,
};
use crate::requirements::{RequirementClass, RequirementDescriptor, RequirementSet};
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
    context_bindings: BTreeSet<ContextRef>,
    selected_context: ContextRef,
    required_authority: AuthorityRef,
    accumulation: AccumulationContract,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitConflictRuleSnapshot {
    reference: ConflictResolutionRuleRef,
    decisive_verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    applicability_rule: ApplicabilityRuleRef,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitCoverageRuleSnapshot {
    reference: CoverageRuleRef,
    required_verifiers: BTreeSet<VerifierRef>,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitReuseRuleSnapshot {
    reference: ReuseRuleRef,
    exact_bindings: BTreeMap<ReuseBindingKeyRef, ReuseBindingValueRef>,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitRequirementSnapshot {
    class: RequirementClass,
    admissible_verifier_families: BTreeSet<VerifierFamilyRef>,
    applicability_rule: ApplicabilityRuleRef,
    conflict_rule: Option<PermitConflictRuleSnapshot>,
    coverage_rule: Option<PermitCoverageRuleSnapshot>,
    reuse_rule: Option<PermitReuseRuleSnapshot>,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitRequirementSetSnapshot {
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    requirements: BTreeMap<RequirementRef, PermitRequirementSnapshot>,
}

#[derive(Debug, PartialEq, Eq)]
struct PermitApplicabilitySnapshot {
    verifier_family: VerifierFamilyRef,
    context: ContextRef,
    applicability_rule: ApplicabilityRuleRef,
}

fn requirement_snapshot(descriptor: &RequirementDescriptor) -> PermitRequirementSnapshot {
    PermitRequirementSnapshot {
        class: descriptor.class(),
        admissible_verifier_families: descriptor
            .admissible_verifier_families()
            .cloned()
            .collect(),
        applicability_rule: descriptor.applicability_rule().clone(),
        conflict_rule: descriptor.conflict_resolution_rule().map(|rule| {
            PermitConflictRuleSnapshot {
                reference: rule.reference().clone(),
                decisive_verifier: rule.decisive_verifier().clone(),
                verifier_family: rule.verifier_family().clone(),
                applicability_rule: rule.applicability_rule().clone(),
            }
        }),
        coverage_rule: descriptor.coverage_rule().map(|rule| PermitCoverageRuleSnapshot {
            reference: rule.reference().clone(),
            required_verifiers: rule.required_verifiers().cloned().collect(),
        }),
        reuse_rule: descriptor.reuse_rule().map(|rule| PermitReuseRuleSnapshot {
            reference: rule.reference().clone(),
            exact_bindings: rule
                .bindings()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        }),
    }
}

fn requirement_set_snapshot(requirements: &RequirementSet) -> PermitRequirementSetSnapshot {
    PermitRequirementSetSnapshot {
        form: requirements.form().clone(),
        effect_family: requirements.effect_family().clone(),
        context: requirements.context().clone(),
        requirements: requirements
            .iter()
            .map(|descriptor| (descriptor.reference().clone(), requirement_snapshot(descriptor)))
            .collect(),
    }
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
/// El sello conserva además la descripción gobernante de `Req`, las relaciones
/// `Applicable(V,q,C)` de los verificadores que participaron y la forma que
/// deberá seguir coincidiendo en la mediación. Conservar esa instantánea no
/// ejecuta el efecto ni convierte el permiso en autoridad nueva.
#[derive(Debug, PartialEq, Eq)]
pub struct Permit {
    authority: PermitAuthorityBinding,
    form: PermitFormBinding,
    effect: EffectDescriptor,
    requirements: PermitRequirementSetSnapshot,
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
        &self.requirements.form
    }

    #[inline]
    pub fn requirement_effect_family(&self) -> &EffectFamilyRef {
        &self.requirements.effect_family
    }

    #[inline]
    pub fn requirement_context(&self) -> &ContextRef {
        &self.requirements.context
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
    pub(crate) fn matches_current_requirements(&self, requirements: &RequirementSet) -> bool {
        self.requirements == requirement_set_snapshot(requirements)
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
/// participantes. `D-R` y `D-N` producen ausencia cerrada de permiso positivo.
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
                requirements: requirement_set_snapshot(requirements),
                applicabilities,
                technical_result,
            }))
        }
    }
}
