//! Mediación no eludible de efectos protegidos para R1-4.
//!
//! Esta unidad no ejecuta un efecto externo. Consume un `Permit` ya concedido,
//! vuelve a comprobar el mismo efecto y las ligaduras gobernantes vigentes y,
//! sólo entonces, forma un compromiso mediado no fabricable. La futura
//! ejecución material deberá consumir ese compromiso, no el permiso aislado.

use crate::authority::transitions::AuthorityContinuity;
use crate::authority::{AccumulationContract, EffectDescriptor};
use crate::control::{
    AuthorityHolderRef, AuthorityRef, CheckResult, ContextRef, EffectFamilyRef, EffectRef, FormRef,
    GovernedObjectRef, TransitionClass,
};
use crate::permission::Permit;

/// Compromiso mediado de un único permiso para un efecto protegido concreto.
///
/// El tipo no implementa `Clone` ni `Copy`, no ofrece constructor público y
/// conserva internamente el `Permit` consumido. Esta unidad tampoco expone una
/// operación que ejecute el efecto; el compromiso constituye la única salida
/// positiva de la frontera de mediación.
///
/// ```compile_fail
/// use sv_core::MediatedEffectCommitment;
/// let _ = MediatedEffectCommitment::new();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct MediatedEffectCommitment {
    permit: Permit,
}

impl MediatedEffectCommitment {
    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        self.permit.authority()
    }

    #[inline]
    pub fn authority_holder(&self) -> &AuthorityHolderRef {
        self.permit.authority_holder()
    }

    #[inline]
    pub fn authority_context(&self) -> &ContextRef {
        self.permit.authority_context()
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        self.permit.form()
    }

    #[inline]
    pub const fn transition_class(&self) -> TransitionClass {
        self.permit.transition_class()
    }

    #[inline]
    pub fn form_effect_family(&self) -> &EffectFamilyRef {
        self.permit.form_effect_family()
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        self.permit.effect()
    }

    #[inline]
    pub fn effect_reference(&self) -> &EffectRef {
        self.permit.effect_reference()
    }

    #[inline]
    pub fn governed_object(&self) -> &GovernedObjectRef {
        self.permit.governed_object()
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        self.permit.context()
    }

    #[inline]
    pub fn accumulation(&self) -> &AccumulationContract {
        self.permit.accumulation()
    }

    #[inline]
    pub const fn technical_result(&self) -> CheckResult {
        self.permit.technical_result()
    }
}

/// Fallo cerrado de la mediación de un permiso.
///
/// Ninguna variante pertenece a `Tri`, modifica autoridad o ejecuta el efecto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediationError {
    EffectMismatch {
        permitted: EffectRef,
        presented: EffectRef,
    },
    UnknownForm(FormRef),
    FormBindingChanged(FormRef),
    UnknownAuthority(AuthorityRef),
    AuthorityBindingChanged(AuthorityRef),
    EffectOutsideCurrentScope {
        authority: AuthorityRef,
        effect: EffectRef,
    },
    MissingRequirementSet {
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
    },
    RequirementBindingChanged {
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
    },
    NonAccreditedPermitState,
}

/// Media un permiso ya concedido contra el estado constituido vigente.
///
/// La operación consume el `Permit`: una misma instancia no puede atravesar
/// dos veces la frontera por clonación, copia o préstamo. Antes de formar el
/// compromiso comprueba el `EffectDescriptor` completo, la forma, la autoridad,
/// `E_max`, `D_a` y la instantánea gobernante de `Req` sellada al conceder el
/// permiso.
///
/// El resultado positivo sigue sin equivaler a ejecución material. Una unidad
/// posterior deberá consumir `MediatedEffectCommitment` en el punto donde el
/// adaptador externo pueda comprometer el efecto.
pub fn mediate_permit(
    continuity: &AuthorityContinuity,
    permit: Permit,
    effect: &EffectDescriptor,
) -> Result<MediatedEffectCommitment, MediationError> {
    if permit.effect() != effect {
        return Err(MediationError::EffectMismatch {
            permitted: permit.effect_reference().clone(),
            presented: effect.reference().clone(),
        });
    }

    let form = continuity
        .form(permit.form())
        .ok_or_else(|| MediationError::UnknownForm(permit.form().clone()))?;

    if !permit.matches_current_form(form) {
        return Err(MediationError::FormBindingChanged(permit.form().clone()));
    }

    let authority_reference = permit.required_authority().clone();
    let authority = continuity
        .authority(&authority_reference)
        .ok_or_else(|| MediationError::UnknownAuthority(authority_reference.clone()))?;

    if !permit.matches_current_authority(authority) {
        return Err(MediationError::AuthorityBindingChanged(authority_reference));
    }

    if !form.describes_effect(effect) || !authority.contains_effect_scope(effect) {
        return Err(MediationError::EffectOutsideCurrentScope {
            authority: authority.reference().clone(),
            effect: effect.reference().clone(),
        });
    }

    let requirements = continuity
        .requirement_set(
            permit.requirement_form(),
            permit.requirement_effect_family(),
            permit.requirement_context(),
        )
        .ok_or_else(|| MediationError::MissingRequirementSet {
            form: permit.requirement_form().clone(),
            effect_family: permit.requirement_effect_family().clone(),
            context: permit.requirement_context().clone(),
        })?;

    if !permit.matches_current_requirements(requirements) {
        return Err(MediationError::RequirementBindingChanged {
            form: permit.requirement_form().clone(),
            effect_family: permit.requirement_effect_family().clone(),
            context: permit.requirement_context().clone(),
        });
    }

    if permit.technical_result() != CheckResult::Accredited {
        return Err(MediationError::NonAccreditedPermitState);
    }

    Ok(MediatedEffectCommitment { permit })
}
