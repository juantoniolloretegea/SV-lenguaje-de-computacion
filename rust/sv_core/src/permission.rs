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
        &self.form.context
    }

    #[inline]
    pub fn requirement_form(&self) -> &FormRef {
        &self.requirement_form
    }

    #[inline]
    pub fn requirement_effect_family(&self) -> &EffectFamilyRef {
        &self.requirement_effect_family
    }

    #[inline]
    pub fn requirement_context(&self) -> &ContextRef {
        &self.requirement_context
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
