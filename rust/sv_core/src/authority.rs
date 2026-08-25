//! Formas constituidas y magnitudes de autoridad de R1.
//!
//! R1-1 fija descriptores de forma, efectos descritos, envolventes máximas y
//! dominios gobernados. R1-2 añade, como submódulo descendiente, la única
//! puerta productiva de constitución disponible en ese corte: T-0 bajo premisa
//! externa opaca y continuidad lógica no habitada.
//!
//! Los constructores brutos conservados en este archivo sólo existen bajo
//! `cfg(test)`. El código de producción no recibe constructores alternativos
//! que permitan fabricar formas o autoridad al margen de la puerta gobernada.

use std::collections::BTreeSet;

use crate::control::{
    AccumulationRuleRef, AuthorityHolderRef, AuthorityRef, ContextRef, EffectFamilyRef, EffectRef,
    FormRef, GovernedObjectRef, TransitionClass,
};

pub mod transitions;

/// Contrato de acumulación fijado por el descriptor de una forma.
///
/// R1-1 registra la clase del contrato, pero no ejecuta todavía agregadores ni
/// predicados de traza.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccumulationContract {
    NotApplicable,
    SingleUse,
    Idempotent,
    GovernedAggregator(AccumulationRuleRef),
    DecidableTracePredicate(AccumulationRuleRef),
}

/// Efecto concreto descrito por identidad, familia, objeto y contexto.
///
/// La identidad nominal del efecto no sustituye las restantes ligaduras. Dos
/// descriptores que reutilicen la misma `EffectRef` con familia, objeto o
/// contexto distintos no representan el mismo alcance constituido.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectDescriptor {
    reference: EffectRef,
    family: EffectFamilyRef,
    object: GovernedObjectRef,
    context: ContextRef,
}

impl EffectDescriptor {
    #[cfg(test)]
    fn constitute(
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

    #[inline]
    pub fn reference(&self) -> &EffectRef {
        &self.reference
    }

    #[inline]
    pub fn family(&self) -> &EffectFamilyRef {
        &self.family
    }

    #[inline]
    pub fn object(&self) -> &GovernedObjectRef {
        &self.object
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }
}

/// Descriptor semántico constituido de una forma concreta de transición.
///
/// Los campos no ofrecen mutadores públicos. R1-2 materializa la génesis
/// inicial; las modificaciones posteriores de formas continúan reservadas a
/// las transiciones autorizantes que correspondan.
#[derive(Debug, PartialEq, Eq)]
pub struct FormDescriptor {
    reference: FormRef,
    transition_class: TransitionClass,
    effect_family: EffectFamilyRef,
    context_bindings: BTreeSet<ContextRef>,
    required_authority: Option<AuthorityRef>,
    accumulation: AccumulationContract,
}

impl FormDescriptor {
    #[cfg(test)]
    fn constitute(
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

    #[inline]
    pub fn reference(&self) -> &FormRef {
        &self.reference
    }

    #[inline]
    pub const fn transition_class(&self) -> TransitionClass {
        self.transition_class
    }

    #[inline]
    pub fn effect_family(&self) -> &EffectFamilyRef {
        &self.effect_family
    }

    #[inline]
    pub fn context_bindings(&self) -> impl Iterator<Item = &ContextRef> {
        self.context_bindings.iter()
    }

    #[inline]
    pub fn requires_authority(&self) -> Option<&AuthorityRef> {
        self.required_authority.as_ref()
    }

    #[inline]
    pub fn accumulation(&self) -> &AccumulationContract {
        &self.accumulation
    }

    /// Comprueba las dimensiones del efecto que R1-1 puede ligar desde la
    /// forma: familia y contexto constituido.
    ///
    /// Un resultado positivo no decide autoridad, habilitación, `Req` ni
    /// permiso.
    #[inline]
    pub fn describes_effect(&self, effect: &EffectDescriptor) -> bool {
        &self.effect_family == effect.family() && self.context_bindings.contains(effect.context())
    }
}

/// Envolvente máxima de efectos `E_max(a | C)`.
///
/// La colección conserva la descripción completa de cada efecto constituido:
/// identidad, familia, objeto y contexto. No depende de que `EffectRef` sea
/// globalmente única fuera de esta frontera.
///
/// La envolvente queda cerrada después de la constitución. No existen métodos
/// públicos que añadan efectos por información, verificación, habilitación o
/// ejercicio ordinarios.
#[derive(Debug, PartialEq, Eq)]
pub struct EffectEnvelope {
    effects: BTreeSet<EffectDescriptor>,
}

impl EffectEnvelope {
    #[cfg(test)]
    fn constitute(effects: impl IntoIterator<Item = EffectDescriptor>) -> Self {
        Self {
            effects: effects.into_iter().collect(),
        }
    }

    #[inline]
    pub fn contains(&self, effect: &EffectDescriptor) -> bool {
        self.effects.contains(effect)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &EffectDescriptor> {
        self.effects.iter()
    }
}

/// Dominio gobernado `D_a` sobre el que puede decidirse pertenencia antes del
/// ejercicio.
///
/// R1-1 usa un conjunto finito e inmutable como primera realización material
/// de pertenencia decidible. No incorpora reglas generativas ni ampliaciones
/// por información ordinaria.
#[derive(Debug, PartialEq, Eq)]
pub struct GovernedDomain {
    objects: BTreeSet<GovernedObjectRef>,
}

impl GovernedDomain {
    #[cfg(test)]
    fn constitute(objects: impl IntoIterator<Item = GovernedObjectRef>) -> Self {
        Self {
            objects: objects.into_iter().collect(),
        }
    }

    #[inline]
    pub fn contains(&self, object: &GovernedObjectRef) -> bool {
        self.objects.contains(object)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &GovernedObjectRef> {
        self.objects.iter()
    }
}

/// Incoherencia al comprobar la constitución de alcance de una autoridad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidAuthorityScope {
    EffectOutsideAuthorityContext,
    EffectOutsideGovernedDomain,
}

/// Autoridad constituida y acotada para un titular y un contexto.
///
/// El objeto no implementa `Clone` ni expone un constructor ordinario. Una
/// `AuthorityRef` copiable continúa siendo sólo una referencia nominal y no
/// sustituye este objeto constituido.
#[derive(Debug, PartialEq, Eq)]
pub struct ConstitutedAuthority {
    reference: AuthorityRef,
    holder: AuthorityHolderRef,
    context: ContextRef,
    max_effects: EffectEnvelope,
    governed_domain: GovernedDomain,
}

impl ConstitutedAuthority {
    #[cfg(test)]
    fn constitute(
        reference: AuthorityRef,
        holder: AuthorityHolderRef,
        context: ContextRef,
        effects: impl IntoIterator<Item = EffectDescriptor>,
        objects: impl IntoIterator<Item = GovernedObjectRef>,
    ) -> Result<Self, InvalidAuthorityScope> {
        let governed_domain = GovernedDomain::constitute(objects);
        let effects: Vec<_> = effects.into_iter().collect();

        if effects.iter().any(|effect| effect.context() != &context) {
            return Err(InvalidAuthorityScope::EffectOutsideAuthorityContext);
        }

        if effects
            .iter()
            .any(|effect| !governed_domain.contains(effect.object()))
        {
            return Err(InvalidAuthorityScope::EffectOutsideGovernedDomain);
        }

        Ok(Self {
            reference,
            holder,
            context,
            max_effects: EffectEnvelope::constitute(effects),
            governed_domain,
        })
    }

    #[inline]
    pub fn reference(&self) -> &AuthorityRef {
        &self.reference
    }

    #[inline]
    pub fn holder(&self) -> &AuthorityHolderRef {
        &self.holder
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }

    #[inline]
    pub fn max_effects(&self) -> &EffectEnvelope {
        &self.max_effects
    }

    #[inline]
    pub fn governed_domain(&self) -> &GovernedDomain {
        &self.governed_domain
    }

    #[inline]
    pub fn contains_effect(&self, effect: &EffectDescriptor) -> bool {
        self.max_effects.contains(effect)
    }

    #[inline]
    pub fn governs(&self, object: &GovernedObjectRef) -> bool {
        self.governed_domain.contains(object)
    }

    /// Comprueba que el efecto descrito pertenece simultáneamente a `E_max`,
    /// a `D_a` y al contexto de esta autoridad.
    ///
    /// Un resultado positivo acredita sólo alcance constituido. No equivale a
    /// habilitación, cumplimiento de `Req`, permiso ni ejecución.
    #[inline]
    pub fn contains_effect_scope(&self, effect: &EffectDescriptor) -> bool {
        &self.context == effect.context()
            && self.contains_effect(effect)
            && self.governs(effect.object())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ControlId;

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

    fn effect_descriptor(
        effect: &str,
        family: &str,
        object: &str,
        context: &str,
    ) -> EffectDescriptor {
        EffectDescriptor::constitute(
            effect_ref(effect),
            effect_family_ref(family),
            object_ref(object),
            context_ref(context),
        )
    }

    #[test]
    fn form_descriptor_freezes_the_constituted_dimensions() {
        let required = authority_ref("authority:prior");
        let descriptor = FormDescriptor::constitute(
            form_ref("form:1"),
            TransitionClass::Exercise,
            effect_family_ref("family:write"),
            [context_ref("context:a"), context_ref("context:b")],
            Some(required.clone()),
            AccumulationContract::SingleUse,
        );

        assert_eq!(descriptor.reference().id().as_str(), "form:1");
        assert_eq!(descriptor.transition_class(), TransitionClass::Exercise);
        assert_eq!(descriptor.effect_family().id().as_str(), "family:write");
        assert_eq!(descriptor.context_bindings().count(), 2);
        assert_eq!(descriptor.requires_authority(), Some(&required));
        assert_eq!(descriptor.accumulation(), &AccumulationContract::SingleUse);
    }

    #[test]
    fn form_scope_requires_family_and_context_together() {
        let descriptor = FormDescriptor::constitute(
            form_ref("form:1"),
            TransitionClass::Exercise,
            effect_family_ref("family:write"),
            [context_ref("context:1")],
            None,
            AccumulationContract::NotApplicable,
        );
        let matching = effect_descriptor("effect:1", "family:write", "object:1", "context:1");
        let wrong_family = effect_descriptor("effect:2", "family:delete", "object:1", "context:1");
        let wrong_context = effect_descriptor("effect:3", "family:write", "object:1", "context:2");

        assert!(descriptor.describes_effect(&matching));
        assert!(!descriptor.describes_effect(&wrong_family));
        assert!(!descriptor.describes_effect(&wrong_context));
    }

    #[test]
    fn effect_envelope_uses_the_complete_effect_scope() {
        let allowed = effect_descriptor("effect:1", "family:write", "object:1", "context:1");
        let same_reference_other_family =
            effect_descriptor("effect:1", "family:delete", "object:1", "context:1");
        let same_reference_other_object =
            effect_descriptor("effect:1", "family:write", "object:2", "context:1");
        let same_reference_other_context =
            effect_descriptor("effect:1", "family:write", "object:1", "context:2");
        let envelope = EffectEnvelope::constitute([allowed.clone()]);

        assert!(envelope.contains(&allowed));
        assert!(!envelope.contains(&same_reference_other_family));
        assert!(!envelope.contains(&same_reference_other_object));
        assert!(!envelope.contains(&same_reference_other_context));
        assert_eq!(envelope.len(), 1);
    }

    #[test]
    fn governed_domain_membership_is_decidable_before_exercise() {
        let member = object_ref("object:member");
        let outsider = object_ref("object:outsider");
        let domain = GovernedDomain::constitute([member.clone()]);

        assert!(domain.contains(&member));
        assert!(!domain.contains(&outsider));
        assert_eq!(domain.len(), 1);
    }

    #[test]
    fn constituted_authority_binds_holder_context_envelope_and_domain() {
        let member = object_ref("object:member");
        let allowed = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:member",
            "context:1",
        );
        let denied = effect_descriptor(
            "effect:denied",
            "family:write",
            "object:member",
            "context:1",
        );

        let authority = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [allowed.clone()],
            [member.clone()],
        )
        .unwrap();

        assert_eq!(authority.reference().id().as_str(), "authority:1");
        assert_eq!(authority.holder().id().as_str(), "holder:1");
        assert_eq!(authority.context().id().as_str(), "context:1");
        assert!(authority.contains_effect(&allowed));
        assert!(!authority.contains_effect(&denied));
        assert!(authority.governs(&member));
    }

    #[test]
    fn authority_rejects_effect_outside_its_context() {
        let result = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [effect_descriptor(
                "effect:1",
                "family:write",
                "object:1",
                "context:2",
            )],
            [object_ref("object:1")],
        );

        assert_eq!(
            result,
            Err(InvalidAuthorityScope::EffectOutsideAuthorityContext)
        );
    }

    #[test]
    fn authority_rejects_effect_outside_its_governed_domain() {
        let result = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [effect_descriptor(
                "effect:1",
                "family:write",
                "object:outsider",
                "context:1",
            )],
            [object_ref("object:member")],
        );

        assert_eq!(
            result,
            Err(InvalidAuthorityScope::EffectOutsideGovernedDomain)
        );
    }

    #[test]
    fn effect_scope_requires_complete_constituted_effect_and_domain() {
        let inside = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:member",
            "context:1",
        );
        let authority = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [inside.clone()],
            [object_ref("object:member")],
        )
        .unwrap();

        let same_reference_wrong_family = effect_descriptor(
            "effect:allowed",
            "family:delete",
            "object:member",
            "context:1",
        );
        let same_reference_wrong_object = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:outsider",
            "context:1",
        );
        let same_reference_wrong_context = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:member",
            "context:2",
        );

        assert!(authority.contains_effect_scope(&inside));
        assert!(!authority.contains_effect_scope(&same_reference_wrong_family));
        assert!(!authority.contains_effect_scope(&same_reference_wrong_object));
        assert!(!authority.contains_effect_scope(&same_reference_wrong_context));
    }

    #[test]
    fn duplicate_members_do_not_expand_envelope_or_domain() {
        let effect = effect_descriptor("effect:1", "family:write", "object:1", "context:1");
        let object = object_ref("object:1");

        let authority = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [effect.clone(), effect],
            [object.clone(), object],
        )
        .unwrap();

        assert_eq!(authority.max_effects().len(), 1);
        assert_eq!(authority.governed_domain().len(), 1);
    }
}
