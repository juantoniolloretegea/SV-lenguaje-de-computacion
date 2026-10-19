//! ES: Proyección cerrada de mandato v1 a T-0. Ninguna petición aporta el plan.
//! EN: Closed v1 mandate-to-T-0 projection. Requests cannot supply this plan.
use super::{AdmissionError, InstitutionalMandate};
use crate::authority::transitions::{
    AuthorityContinuity, AuthorityProposal, EffectProposal, FormProposal, GenesisPlan,
};
use crate::authority::{AccumulationContract, EffectDescriptor};
use crate::control::*;
use crate::requirements::initial::{
    ApplicabilityProposal, CoverageRuleProposal, RequirementProposal, ReuseBindingProposal,
    ReuseRuleProposal,
};
use crate::requirements::{CoreRequirementKind, RequirementClass};

pub(crate) const CORE: [CoreRequirementKind; 6] = [
    CoreRequirementKind::FormValidity,
    CoreRequirementKind::ApplicableAuthority,
    CoreRequirementKind::GovernedDomainMembership,
    CoreRequirementKind::VerifierAdmissibilityAndApplicability,
    CoreRequirementKind::NoSelfAccreditation,
    CoreRequirementKind::ValidityOrNonRevocation,
];
pub(crate) fn id(value: &str) -> Result<ControlId, AdmissionError> {
    ControlId::new(value).map_err(|_| AdmissionError::InvalidField)
}
pub(crate) fn form_ref() -> Result<FormRef, AdmissionError> {
    Ok(FormRef::from_core_id(id("institutional:form:v1")?))
}
pub(crate) fn authority_ref() -> Result<AuthorityRef, AdmissionError> {
    Ok(AuthorityRef::from_core_id(id(
        "institutional:authority:v1",
    )?))
}
pub(crate) fn verifier(index: usize) -> Result<VerifierRef, AdmissionError> {
    Ok(VerifierRef::from_core_id(id(&format!(
        "institutional:verifier:{index}:v1"
    ))?))
}
pub(super) fn make_plan(m: &InstitutionalMandate) -> Result<GenesisPlan, AdmissionError> {
    let form = form_ref()?;
    let authority = authority_ref()?;
    let family = EffectFamilyRef::from_core_id(id(&m.labels[4])?);
    let context = ContextRef::from_core_id(id(&m.labels[3])?);
    let object = GovernedObjectRef::from_core_id(id(&m.labels[5])?);
    let vf = VerifierFamilyRef::from_core_id(id("institutional:verifiers:v1")?);
    let rule = ApplicabilityRuleRef::from_core_id(id("institutional:applicability:v1")?);
    // ES: La mediación conserva el mandato exacto, incluida su vigencia y edición.
    // No habilita reutilización entre peticiones ni evita comprobar la revocación.
    // EN: Mediation retains the exact mandate, including validity and edition.
    // This allows neither cross-request reuse nor skipping revocation checks.
    let fingerprint = m
        .fingerprint()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let binding = ReuseBindingProposal::new(
        ReuseBindingKeyRef::from_core_id(id("institutional:mandate:sha256")?),
        ReuseBindingValueRef::from_core_id(id(&fingerprint)?),
    );
    let mut requirements = Vec::with_capacity(7);
    let mut applicabilities = Vec::with_capacity(7);
    for (index, class) in CORE
        .into_iter()
        .map(RequirementClass::Core)
        .chain(std::iter::once(RequirementClass::Specific))
        .enumerate()
    {
        let q = RequirementRef::from_core_id(id(&format!("institutional:requirement:{index}:v1"))?);
        let v = verifier(index)?;
        requirements.push(
            RequirementProposal::new(
                q.clone(),
                class,
                form.clone(),
                family.clone(),
                context.clone(),
                [vf.clone()],
                rule.clone(),
            )
            .with_coverage_rule(CoverageRuleProposal::new(
                CoverageRuleRef::from_core_id(id(&format!("institutional:coverage:{index}:v1"))?),
                [v.clone()],
            ))
            .with_reuse_rule(ReuseRuleProposal::new(
                ReuseRuleRef::from_core_id(id(&format!("institutional:reuse:{index}:v1"))?),
                [binding.clone()],
            )),
        );
        applicabilities.push(ApplicabilityProposal::new(
            v,
            vf.clone(),
            q,
            context.clone(),
            rule.clone(),
        ));
    }
    Ok(GenesisPlan::new(
        [FormProposal::new(
            form,
            TransitionClass::Exercise,
            family.clone(),
            [context.clone()],
            Some(authority.clone()),
            AccumulationContract::SingleUse,
        )],
        [AuthorityProposal::new(
            authority,
            AuthorityHolderRef::from_core_id(id(&m.labels[1])?),
            context.clone(),
            [EffectProposal::new(
                EffectRef::from_core_id(id(&m.labels[4])?),
                family,
                object.clone(),
                context,
            )],
            [object],
        )],
    )
    .with_initial_control(requirements, applicabilities))
}
pub(super) fn operation(
    c: &AuthorityContinuity,
) -> Result<(FormRef, EffectDescriptor), AdmissionError> {
    let a = c
        .authority(&authority_ref()?)
        .ok_or(AdmissionError::CoreInvariant)?;
    if a.max_effects().len() != 1 {
        return Err(AdmissionError::CoreInvariant);
    }
    Ok((
        form_ref()?,
        a.max_effects()
            .iter()
            .next()
            .ok_or(AdmissionError::CoreInvariant)?
            .clone(),
    ))
}
