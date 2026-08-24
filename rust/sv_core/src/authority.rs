//! Formas constituidas y magnitudes de autoridad de R1-1.
//!
//! Este módulo representa descriptores de forma, efectos descritos,
//! envolventes máximas de efectos y dominios gobernados. No implementa todavía
//! transiciones T-*, habilitación, `Req`, permisos ni ejecución de efectos.
//!
//! Las estructuras que implican constitución no disponen de constructor
//! público. Identificar una forma, un titular o una autoridad no permite
//! fabricar el objeto constituido correspondiente.

use std::collections::BTreeSet;

use crate::control::{
    AccumulationRuleRef, AuthorityHolderRef, AuthorityRef, ContextRef, EffectFamilyRef, EffectRef,
    FormRef, GovernedObjectRef, TransitionClass,
};

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
/// Esta estructura permite comprobar alcance. No constituye autorización para
/// ejecutarlo y no dispone de constructor público.
#[derive(Debug, PartialEq, Eq)]
pub struct EffectDescriptor {
    reference: EffectRef,
    family: EffectFamilyRef,
    object: GovernedObjectRef,
    context: ContextRef,
}

impl EffectDescriptor {
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
/// Los campos quedan fijados en la constitución y no ofrecen mutadores
/// públicos. R1-2 deberá gobernar las vías que puedan producir descriptores
/// nuevos o modificar materialmente el conjunto de formas.
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

    /// Comprueba únicamente la pertenencia del efecto a la familia declarada
    /// por la forma. No decide autoridad, habilitación ni permiso.
    #[inline]
    pub fn describes_effect_family(&self, effect: &EffectDescriptor) -> bool {
        &self.effect_family == effect.family()
    }
}

/// Envolvente máxima de efectos `E_max(a | C)`.
///
/// La colección es cerrada después de la constitución. No existen métodos
/// públicos que añadan efectos por información, verificación, habilitación o
/// ejercicio ordinarios.
#[derive(Debug, PartialEq, Eq)]
pub struct EffectEnvelope {
    effects: BTreeSet<EffectRef>,
}

impl EffectEnvelope {
    fn constitute(effects: impl IntoIterator<Item = EffectRef>) -> Self {
        Self {
            effects: effects.into_iter().collect(),
        }
    }

    #[inline]
    pub fn contains(&self, effect: &EffectRef) -> bool {
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
    pub fn iter(&self) -> impl Iterator<Item = &EffectRef> {
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

/// Autoridad constituida y acotada para un titular y un contexto.
///
/// El objeto no implementa `Clone` ni expone un constructor público. Una
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
    fn constitute(
        reference: AuthorityRef,
        holder: AuthorityHolderRef,
        context: ContextRef,
        effects: impl IntoIterator<Item = EffectRef>,
        objects: impl IntoIterator<Item = GovernedObjectRef>,
    ) -> Self {
        Self {
            reference,
            holder,
            context,
            max_effects: EffectEnvelope::constitute(effects),
            governed_domain: GovernedDomain::constitute(objects),
        }
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
    pub fn contains_effect(&self, effect: &EffectRef) -> bool {
        self.max_effects.contains(effect)
    }

    #[inline]
    pub fn governs(&self, object: &GovernedObjectRef) -> bool {
        self.governed_domain.contains(object)
    }

    /// Comprueba que el efecto descrito pertenece simultáneamente al contexto,
    /// a `E_max` y a `D_a` de esta autoridad.
    ///
    /// Un resultado positivo acredita sólo alcance constituido. No equivale a
    /// habilitación ni a permiso de ejecución.
    #[inline]
    pub fn contains_effect_scope(&self, effect: &EffectDescriptor) -> bool {
        &self.context == effect.context()
            && self.contains_effect(effect.reference())
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
    fn form_descriptor_rejects_an_effect_from_another_family_by_scope_test() {
        let descriptor = FormDescriptor::constitute(
            form_ref("form:1"),
            TransitionClass::Exercise,
            effect_family_ref("family:write"),
            [context_ref("context:1")],
            None,
            AccumulationContract::NotApplicable,
        );
        let matching = effect_descriptor("effect:1", "family:write", "object:1", "context:1");
        let foreign = effect_descriptor("effect:2", "family:delete", "object:1", "context:1");

        assert!(descriptor.describes_effect_family(&matching));
        assert!(!descriptor.describes_effect_family(&foreign));
    }

    #[test]
    fn effect_envelope_is_exact_and_not_wildcarded() {
        let allowed = effect_ref("effect:allowed");
        let other = effect_ref("effect:other");
        let envelope = EffectEnvelope::constitute([allowed.clone()]);

        assert!(envelope.contains(&allowed));
        assert!(!envelope.contains(&other));
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
        let allowed = effect_ref("effect:allowed");
        let denied = effect_ref("effect:denied");
        let member = object_ref("object:member");
        let outsider = object_ref("object:outsider");

        let authority = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [allowed.clone()],
            [member.clone()],
        );

        assert_eq!(authority.reference().id().as_str(), "authority:1");
        assert_eq!(authority.holder().id().as_str(), "holder:1");
        assert_eq!(authority.context().id().as_str(), "context:1");
        assert!(authority.contains_effect(&allowed));
        assert!(!authority.contains_effect(&denied));
        assert!(authority.governs(&member));
        assert!(!authority.governs(&outsider));
    }

    #[test]
    fn effect_scope_requires_context_envelope_and_domain_together() {
        let authority = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [effect_ref("effect:allowed")],
            [object_ref("object:member")],
        );

        let inside = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:member",
            "context:1",
        );
        let wrong_effect = effect_descriptor(
            "effect:other",
            "family:write",
            "object:member",
            "context:1",
        );
        let wrong_object = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:outsider",
            "context:1",
        );
        let wrong_context = effect_descriptor(
            "effect:allowed",
            "family:write",
            "object:member",
            "context:2",
        );

        assert!(authority.contains_effect_scope(&inside));
        assert!(!authority.contains_effect_scope(&wrong_effect));
        assert!(!authority.contains_effect_scope(&wrong_object));
        assert!(!authority.contains_effect_scope(&wrong_context));
    }

    #[test]
    fn duplicate_members_do_not_expand_envelope_or_domain() {
        let effect = effect_ref("effect:1");
        let object = object_ref("object:1");

        let authority = ConstitutedAuthority::constitute(
            authority_ref("authority:1"),
            holder_ref("holder:1"),
            context_ref("context:1"),
            [effect.clone(), effect],
            [object.clone(), object],
        );

        assert_eq!(authority.max_effects().len(), 1);
        assert_eq!(authority.governed_domain().len(), 1);
    }
}
