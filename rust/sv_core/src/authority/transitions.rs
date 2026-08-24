//! Transiciones de autoridad y génesis lógica de R1-2.
//!
//! Este módulo es descendiente de `authority` para que la única vía productiva
//! de construcción de `FormDescriptor` y `ConstitutedAuthority` de este corte
//! quede situada en la puerta T-0. Los adaptadores externos no reciben
//! constructores equivalentes.
//!
//! La continuidad representada aquí es lógica e intra-proceso. No acredita
//! continuidad material entre procesos, réplicas, restauraciones o estados
//! persistentes.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    AccumulationContract, ConstitutedAuthority, EffectDescriptor, EffectEnvelope, FormDescriptor,
    GovernedDomain, InvalidAuthorityScope,
};
use crate::control::{
    AuthorityHolderRef, AuthorityRef, ContextRef, ContinuityOccupancy, EffectFamilyRef, EffectRef,
    FormRef, GovernedObjectRef, TransitionClass,
};

/// Disposición de una clase T-* respecto de la constitución de autoridad en
/// R1-2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionDisposition {
    /// T-0: única vía productiva de autoridad materializada en este corte.
    GenesisOnly,
    /// T-I, T-V, T-H y T-E: no pueden constituir autoridad.
    NonAuthorizing,
    /// T-G, T-C y T-R: no aplican cambios hasta que R1-3 materialice `Req` y
    /// los resultados de comprobación aplicables.
    BlockedPendingRequirements,
}

/// Clasifica una transición sin ejecutar efectos ni modificar autoridad.
#[inline]
pub const fn transition_disposition(class: TransitionClass) -> TransitionDisposition {
    match class {
        TransitionClass::Genesis => TransitionDisposition::GenesisOnly,
        TransitionClass::Information
        | TransitionClass::Verification
        | TransitionClass::Enablement
        | TransitionClass::Exercise => TransitionDisposition::NonAuthorizing,
        TransitionClass::Governance
        | TransitionClass::Constitutive
        | TransitionClass::Recovery => TransitionDisposition::BlockedPendingRequirements,
    }
}

/// Premisa constituyente externa consumida por la puerta T-0.
///
/// El tipo es deliberadamente opaco y no ofrece constructor público. Su mera
/// existencia tampoco constituye autoridad SV: sólo satisface una premisa de
/// entrada del modelo lógico de T-0. La legitimidad material de su procedencia
/// queda fuera de R1.
///
/// ```compile_fail
/// use sv_core::ExternalGenesisPremise;
/// let _premise = ExternalGenesisPremise { consumed: false };
/// ```
#[derive(Debug)]
pub struct ExternalGenesisPremise {
    consumed: bool,
}

impl ExternalGenesisPremise {
    #[cfg(test)]
    fn for_test() -> Self {
        Self { consumed: false }
    }

    /// Indica si la premisa ya fue consumida por una génesis completada.
    #[inline]
    pub const fn is_consumed(&self) -> bool {
        self.consumed
    }
}

/// Descripción ordinaria de un efecto propuesto para la constitución inicial.
///
/// Una propuesta no es un `EffectDescriptor` constituido.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectProposal {
    reference: EffectRef,
    family: EffectFamilyRef,
    object: GovernedObjectRef,
    context: ContextRef,
}

impl EffectProposal {
    pub fn new(
        reference: EffectRef,
        family: EffectFamilyRef,
        object: GovernedObjectRef,
        context: ContextRef,
    ) -> Self {
        Self {
            reference,
            family,
            object,
            context,
        }
    }
}

/// Descripción ordinaria de una forma propuesta para el estado inicial.
///
/// La clase T-* queda fijada en la propuesta y no puede elegirse después por el
/// ejecutor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormProposal {
    reference: FormRef,
    transition_class: TransitionClass,
    effect_family: EffectFamilyRef,
    context_bindings: Vec<ContextRef>,
    required_authority: Option<AuthorityRef>,
    accumulation: AccumulationContract,
}

impl FormProposal {
    pub fn new(
        reference: FormRef,
        transition_class: TransitionClass,
        effect_family: EffectFamilyRef,
        context_bindings: impl IntoIterator<Item = ContextRef>,
        required_authority: Option<AuthorityRef>,
        accumulation: AccumulationContract,
    ) -> Self {
        Self {
            reference,
            transition_class,
            effect_family,
            context_bindings: context_bindings.into_iter().collect(),
            required_authority,
            accumulation,
        }
    }
}

/// Descripción ordinaria de una autoridad propuesta para el estado inicial.
///
/// La propuesta no confiere autoridad y no puede convertirse en
/// `ConstitutedAuthority` fuera de la puerta T-0 de este módulo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityProposal {
    reference: AuthorityRef,
    holder: AuthorityHolderRef,
    context: ContextRef,
    effects: Vec<EffectProposal>,
    governed_objects: Vec<GovernedObjectRef>,
}

impl AuthorityProposal {
    pub fn new(
        reference: AuthorityRef,
        holder: AuthorityHolderRef,
        context: ContextRef,
        effects: impl IntoIterator<Item = EffectProposal>,
        governed_objects: impl IntoIterator<Item = GovernedObjectRef>,
    ) -> Self {
        Self {
            reference,
            holder,
            context,
            effects: effects.into_iter().collect(),
            governed_objects: governed_objects.into_iter().collect(),
        }
    }
}

/// Propuesta completa de estado inicial para T-0.
#[derive(Debug, PartialEq, Eq)]
pub struct GenesisPlan {
    forms: Vec<FormProposal>,
    authorities: Vec<AuthorityProposal>,
}

impl GenesisPlan {
    pub fn new(
        forms: impl IntoIterator<Item = FormProposal>,
        authorities: impl IntoIterator<Item = AuthorityProposal>,
    ) -> Self {
        Self {
            forms: forms.into_iter().collect(),
            authorities: authorities.into_iter().collect(),
        }
    }

    #[inline]
    pub fn form_count(&self) -> usize {
        self.forms.len()
    }

    #[inline]
    pub fn authority_count(&self) -> usize {
        self.authorities.len()
    }
}

/// Rechazo cerrado de una génesis lógica.
///
/// Ninguna variante pertenece a `Tri` ni constituye un valor semántico de
/// dominio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenesisError {
    AlreadyInhabited,
    PremiseAlreadyConsumed,
    EmptyInitialForms,
    EmptyInitialAuthorities,
    DuplicateFormRef(FormRef),
    DuplicateAuthorityRef(AuthorityRef),
    MissingRequiredAuthority {
        form: FormRef,
        authority: AuthorityRef,
    },
    InvalidAuthorityScope(InvalidAuthorityScope),
}

impl From<InvalidAuthorityScope> for GenesisError {
    fn from(error: InvalidAuthorityScope) -> Self {
        Self::InvalidAuthorityScope(error)
    }
}

/// Continuidad autoritativa lógica de R1.
///
/// El tipo no implementa `Clone`. Crear un valor `Uninhabited` representa un
/// estado lógico vacío; no demuestra que el proceso material sea una
/// continuidad autoritativa nueva.
#[derive(Debug, PartialEq, Eq)]
pub struct AuthorityContinuity {
    occupancy: ContinuityOccupancy,
    forms: BTreeMap<FormRef, FormDescriptor>,
    authorities: BTreeMap<AuthorityRef, ConstitutedAuthority>,
}

impl AuthorityContinuity {
    /// Crea únicamente el estado lógico vacío del modelo intra-proceso.
    #[inline]
    pub fn uninhabited() -> Self {
        Self {
            occupancy: ContinuityOccupancy::Uninhabited,
            forms: BTreeMap::new(),
            authorities: BTreeMap::new(),
        }
    }

    #[inline]
    pub const fn occupancy(&self) -> ContinuityOccupancy {
        self.occupancy
    }

    /// Disponibilidad lógica de T-0 dentro de este objeto de continuidad.
    #[inline]
    pub const fn t0_available(&self) -> bool {
        matches!(self.occupancy, ContinuityOccupancy::Uninhabited)
    }

    #[inline]
    pub fn form_count(&self) -> usize {
        self.forms.len()
    }

    #[inline]
    pub fn authority_count(&self) -> usize {
        self.authorities.len()
    }

    #[inline]
    pub fn form(&self, reference: &FormRef) -> Option<&FormDescriptor> {
        self.forms.get(reference)
    }

    #[inline]
    pub fn authority(&self, reference: &AuthorityRef) -> Option<&ConstitutedAuthority> {
        self.authorities.get(reference)
    }

    /// Puerta interna de T-0.
    ///
    /// Es `pub(crate)` porque R1-2 materializa la transición dentro de
    /// `sv_core`, pero no ofrece a los adaptadores una API que pueda acuñar por
    /// sí misma la premisa externa necesaria. La operación es transaccional:
    /// todo rechazo deja la continuidad y la premisa sin consumir.
    pub(crate) fn apply_genesis(
        &mut self,
        premise: &mut ExternalGenesisPremise,
        plan: GenesisPlan,
    ) -> Result<(), GenesisError> {
        if matches!(self.occupancy, ContinuityOccupancy::Inhabited) {
            return Err(GenesisError::AlreadyInhabited);
        }

        if premise.consumed {
            return Err(GenesisError::PremiseAlreadyConsumed);
        }

        if plan.forms.is_empty() {
            return Err(GenesisError::EmptyInitialForms);
        }

        if plan.authorities.is_empty() {
            return Err(GenesisError::EmptyInitialAuthorities);
        }

        let mut authority_refs = BTreeSet::new();
        for authority in &plan.authorities {
            if !authority_refs.insert(authority.reference.clone()) {
                return Err(GenesisError::DuplicateAuthorityRef(
                    authority.reference.clone(),
                ));
            }
        }

        let mut form_refs = BTreeSet::new();
        for form in &plan.forms {
            if !form_refs.insert(form.reference.clone()) {
                return Err(GenesisError::DuplicateFormRef(form.reference.clone()));
            }

            if let Some(required) = &form.required_authority {
                if !authority_refs.contains(required) {
                    return Err(GenesisError::MissingRequiredAuthority {
                        form: form.reference.clone(),
                        authority: required.clone(),
                    });
                }
            }
        }

        let mut authorities = BTreeMap::new();
        for proposal in plan.authorities {
            let governed_domain = GovernedDomain {
                objects: proposal.governed_objects.into_iter().collect(),
            };

            let mut effects = BTreeSet::new();
            for effect in proposal.effects {
                if effect.context != proposal.context {
                    return Err(GenesisError::InvalidAuthorityScope(
                        InvalidAuthorityScope::EffectOutsideAuthorityContext,
                    ));
                }

                if !governed_domain.objects.contains(&effect.object) {
                    return Err(GenesisError::InvalidAuthorityScope(
                        InvalidAuthorityScope::EffectOutsideGovernedDomain,
                    ));
                }

                effects.insert(EffectDescriptor {
                    reference: effect.reference,
                    family: effect.family,
                    object: effect.object,
                    context: effect.context,
                });
            }

            let reference = proposal.reference.clone();
            authorities.insert(
                reference,
                ConstitutedAuthority {
                    reference: proposal.reference,
                    holder: proposal.holder,
                    context: proposal.context,
                    max_effects: EffectEnvelope { effects },
                    governed_domain,
                },
            );
        }

        let mut forms = BTreeMap::new();
        for proposal in plan.forms {
            let reference = proposal.reference.clone();
            forms.insert(
                reference,
                FormDescriptor {
                    reference: proposal.reference,
                    transition_class: proposal.transition_class,
                    effect_family: proposal.effect_family,
                    context_bindings: proposal.context_bindings.into_iter().collect(),
                    required_authority: proposal.required_authority,
                    accumulation: proposal.accumulation,
                },
            );
        }

        self.forms = forms;
        self.authorities = authorities;
        self.occupancy = ContinuityOccupancy::Inhabited;
        premise.consumed = true;
        Ok(())
    }
}

impl Default for AuthorityContinuity {
    fn default() -> Self {
        Self::uninhabited()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::{ControlId, TransitionClass};

    fn id(value: &str) -> ControlId {
        ControlId::new(value).unwrap()
    }

    fn form_ref(value: &str) -> FormRef {
        FormRef::from_core_id(id(value))
    }

    fn effect_family_ref(value: &str) -> EffectFamilyRef {
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

    fn valid_plan() -> GenesisPlan {
        let authority = authority_ref("authority:genesis");
        GenesisPlan::new(
            [FormProposal::new(
                form_ref("form:exercise"),
                TransitionClass::Exercise,
                effect_family_ref("family:write"),
                [context_ref("context:genesis")],
                Some(authority.clone()),
                AccumulationContract::SingleUse,
            )],
            [AuthorityProposal::new(
                authority,
                holder_ref("holder:root"),
                context_ref("context:genesis"),
                [EffectProposal::new(
                    effect_ref("effect:write-one"),
                    effect_family_ref("family:write"),
                    object_ref("object:one"),
                    context_ref("context:genesis"),
                )],
                [object_ref("object:one")],
            )],
        )
    }

    #[test]
    fn valid_t0_constitutes_initial_state_and_consumes_genesis() {
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        continuity.apply_genesis(&mut premise, valid_plan()).unwrap();

        assert_eq!(continuity.occupancy(), ContinuityOccupancy::Inhabited);
        assert!(!continuity.t0_available());
        assert!(premise.is_consumed());
        assert_eq!(continuity.form_count(), 1);
        assert_eq!(continuity.authority_count(), 1);
        assert!(continuity.form(&form_ref("form:exercise")).is_some());
        assert!(continuity
            .authority(&authority_ref("authority:genesis"))
            .is_some());
    }

    #[test]
    fn second_t0_on_the_same_continuity_is_rejected() {
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut first = ExternalGenesisPremise::for_test();
        continuity.apply_genesis(&mut first, valid_plan()).unwrap();

        let mut second = ExternalGenesisPremise::for_test();
        let result = continuity.apply_genesis(&mut second, valid_plan());

        assert_eq!(result, Err(GenesisError::AlreadyInhabited));
        assert!(!second.is_consumed());
        assert_eq!(continuity.form_count(), 1);
        assert_eq!(continuity.authority_count(), 1);
    }

    #[test]
    fn rejected_empty_genesis_does_not_consume_t0_or_premise() {
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        let result = continuity.apply_genesis(&mut premise, GenesisPlan::new([], []));

        assert_eq!(result, Err(GenesisError::EmptyInitialForms));
        assert_eq!(continuity.occupancy(), ContinuityOccupancy::Uninhabited);
        assert!(continuity.t0_available());
        assert!(!premise.is_consumed());
        assert_eq!(continuity.form_count(), 0);
        assert_eq!(continuity.authority_count(), 0);
    }

    #[test]
    fn consumed_external_premise_cannot_seed_another_genesis() {
        let mut first_continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();
        first_continuity
            .apply_genesis(&mut premise, valid_plan())
            .unwrap();

        let mut second_continuity = AuthorityContinuity::uninhabited();
        let result = second_continuity.apply_genesis(&mut premise, valid_plan());

        assert_eq!(result, Err(GenesisError::PremiseAlreadyConsumed));
        assert!(second_continuity.t0_available());
    }

    #[test]
    fn duplicate_form_reference_is_rejected_atomically() {
        let authority = authority_ref("authority:genesis");
        let duplicated = form_ref("form:duplicate");
        let form = || {
            FormProposal::new(
                duplicated.clone(),
                TransitionClass::Exercise,
                effect_family_ref("family:write"),
                [context_ref("context:genesis")],
                Some(authority.clone()),
                AccumulationContract::SingleUse,
            )
        };
        let plan = GenesisPlan::new(
            [form(), form()],
            [AuthorityProposal::new(
                authority,
                holder_ref("holder:root"),
                context_ref("context:genesis"),
                [],
                [object_ref("object:one")],
            )],
        );
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        let result = continuity.apply_genesis(&mut premise, plan);

        assert_eq!(result, Err(GenesisError::DuplicateFormRef(duplicated)));
        assert!(continuity.t0_available());
        assert!(!premise.is_consumed());
    }

    #[test]
    fn duplicate_authority_reference_is_rejected_atomically() {
        let duplicated = authority_ref("authority:duplicate");
        let authority = || {
            AuthorityProposal::new(
                duplicated.clone(),
                holder_ref("holder:root"),
                context_ref("context:genesis"),
                [],
                [object_ref("object:one")],
            )
        };
        let plan = GenesisPlan::new(
            [FormProposal::new(
                form_ref("form:exercise"),
                TransitionClass::Exercise,
                effect_family_ref("family:write"),
                [context_ref("context:genesis")],
                Some(duplicated.clone()),
                AccumulationContract::SingleUse,
            )],
            [authority(), authority()],
        );
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        let result = continuity.apply_genesis(&mut premise, plan);

        assert_eq!(
            result,
            Err(GenesisError::DuplicateAuthorityRef(duplicated))
        );
        assert!(continuity.t0_available());
        assert!(!premise.is_consumed());
    }

    #[test]
    fn missing_required_authority_is_rejected() {
        let missing = authority_ref("authority:missing");
        let plan = GenesisPlan::new(
            [FormProposal::new(
                form_ref("form:exercise"),
                TransitionClass::Exercise,
                effect_family_ref("family:write"),
                [context_ref("context:genesis")],
                Some(missing.clone()),
                AccumulationContract::SingleUse,
            )],
            [AuthorityProposal::new(
                authority_ref("authority:other"),
                holder_ref("holder:root"),
                context_ref("context:genesis"),
                [],
                [object_ref("object:one")],
            )],
        );
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        let result = continuity.apply_genesis(&mut premise, plan);

        assert_eq!(
            result,
            Err(GenesisError::MissingRequiredAuthority {
                form: form_ref("form:exercise"),
                authority: missing,
            })
        );
        assert!(continuity.t0_available());
        assert!(!premise.is_consumed());
    }

    #[test]
    fn authority_effect_outside_context_is_rejected_without_partial_state() {
        let authority = authority_ref("authority:genesis");
        let plan = GenesisPlan::new(
            [FormProposal::new(
                form_ref("form:exercise"),
                TransitionClass::Exercise,
                effect_family_ref("family:write"),
                [context_ref("context:genesis")],
                Some(authority.clone()),
                AccumulationContract::SingleUse,
            )],
            [AuthorityProposal::new(
                authority,
                holder_ref("holder:root"),
                context_ref("context:genesis"),
                [EffectProposal::new(
                    effect_ref("effect:write-one"),
                    effect_family_ref("family:write"),
                    object_ref("object:one"),
                    context_ref("context:foreign"),
                )],
                [object_ref("object:one")],
            )],
        );
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        let result = continuity.apply_genesis(&mut premise, plan);

        assert_eq!(
            result,
            Err(GenesisError::InvalidAuthorityScope(
                InvalidAuthorityScope::EffectOutsideAuthorityContext
            ))
        );
        assert!(continuity.t0_available());
        assert_eq!(continuity.form_count(), 0);
        assert_eq!(continuity.authority_count(), 0);
        assert!(!premise.is_consumed());
    }

    #[test]
    fn authority_effect_outside_domain_is_rejected_without_partial_state() {
        let authority = authority_ref("authority:genesis");
        let plan = GenesisPlan::new(
            [FormProposal::new(
                form_ref("form:exercise"),
                TransitionClass::Exercise,
                effect_family_ref("family:write"),
                [context_ref("context:genesis")],
                Some(authority.clone()),
                AccumulationContract::SingleUse,
            )],
            [AuthorityProposal::new(
                authority,
                holder_ref("holder:root"),
                context_ref("context:genesis"),
                [EffectProposal::new(
                    effect_ref("effect:write-one"),
                    effect_family_ref("family:write"),
                    object_ref("object:outside"),
                    context_ref("context:genesis"),
                )],
                [object_ref("object:inside")],
            )],
        );
        let mut continuity = AuthorityContinuity::uninhabited();
        let mut premise = ExternalGenesisPremise::for_test();

        let result = continuity.apply_genesis(&mut premise, plan);

        assert_eq!(
            result,
            Err(GenesisError::InvalidAuthorityScope(
                InvalidAuthorityScope::EffectOutsideGovernedDomain
            ))
        );
        assert!(continuity.t0_available());
        assert_eq!(continuity.form_count(), 0);
        assert_eq!(continuity.authority_count(), 0);
        assert!(!premise.is_consumed());
    }

    #[test]
    fn information_verification_enablement_and_exercise_are_non_authorizing() {
        for class in [
            TransitionClass::Information,
            TransitionClass::Verification,
            TransitionClass::Enablement,
            TransitionClass::Exercise,
        ] {
            assert_eq!(
                transition_disposition(class),
                TransitionDisposition::NonAuthorizing
            );
        }
    }

    #[test]
    fn governance_constitutive_and_recovery_are_blocked_until_requirements_exist() {
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

    #[test]
    fn only_genesis_has_productive_disposition_in_r1_2() {
        assert_eq!(
            transition_disposition(TransitionClass::Genesis),
            TransitionDisposition::GenesisOnly
        );
    }
}
