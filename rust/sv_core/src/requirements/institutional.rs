//! ES: Seis comprobaciones nucleares y comparación específica del mandato v1.
//! Reutiliza resolución y cobertura R1; no acepta resultados del solicitante.
//! EN: Six core checks plus the v1 mandate's specific comparison. Reuses R1
//! resolution and coverage; requesters cannot supply check outcomes.
#![forbid(unsafe_code)]

use super::observed::PreparedExactCheck;
use super::{CoreRequirementKind, RequirementCheck, RequirementClass};
use crate::authority::transitions::institutional::{plan, AdmissionError, ValidatedInvocation};
use crate::{CheckResult, EffectDescriptor, FormRef, ResolvedRequirementResult, TransitionClass};

pub(crate) fn measure(
    evidence: &ValidatedInvocation<'_>,
    form: &FormRef,
    effect: &EffectDescriptor,
) -> Result<Vec<ResolvedRequirementResult>, AdmissionError> {
    let c = evidence.continuity();
    let f = c.form(form).ok_or(AdmissionError::CoreInvariant)?;
    let authority_ref = f
        .requires_authority()
        .ok_or(AdmissionError::CoreInvariant)?;
    let a = c
        .authority(authority_ref)
        .ok_or(AdmissionError::CoreInvariant)?;
    let set = c
        .requirement_set(form, effect.family(), effect.context())
        .ok_or(AdmissionError::CoreInvariant)?;
    if set.len() != 7 {
        return Err(AdmissionError::CoreInvariant);
    }
    let mut results = Vec::with_capacity(7);
    for (index, d) in set.iter().enumerate() {
        // ES: Orden de los identificadores fijado por el único productor del plan.
        // EN: Identifier ordering is fixed by the sole plan producer.
        let v = plan::verifier(index)?;
        let applicability = c
            .verifier_applicability(d.reference(), &v, effect.context())
            .ok_or(AdmissionError::CoreInvariant)?;
        let all_applicable = set.iter().enumerate().all(|(i, descriptor)| {
            plan::verifier(i)
                .ok()
                .and_then(|v| {
                    c.verifier_applicability(descriptor.reference(), &v, effect.context())
                })
                .is_some_and(|value| descriptor.accepts_applicability(value))
        });
        let check = match d.class() {
            RequirementClass::Specific if index == 6 => PreparedExactCheck::prepare(
                c,
                d,
                applicability,
                evidence.contract(),
                evidence.expected(),
            )
            .and_then(|p| p.run(evidence.observed()))
            .map_err(|_| AdmissionError::CoreInvariant)?
            .into_check(),
            RequirementClass::Core(kind) if plan::CORE.get(index) == Some(&kind) => {
                let satisfied = match kind {
                    CoreRequirementKind::FormValidity => {
                        f.transition_class() == TransitionClass::Exercise
                            && f.describes_effect(effect)
                    }
                    CoreRequirementKind::ApplicableAuthority => {
                        a.reference() == authority_ref && a.contains_effect_scope(effect)
                    }
                    CoreRequirementKind::GovernedDomainMembership => a.governs(effect.object()),
                    CoreRequirementKind::VerifierAdmissibilityAndApplicability => all_applicable,
                    CoreRequirementKind::NoSelfAccreditation => evidence.independent_keys(),
                    CoreRequirementKind::ValidityOrNonRevocation => evidence.current(),
                };
                RequirementCheck {
                    requirement: d.reference.clone(),
                    form: d.form.clone(),
                    effect_family: d.effect_family.clone(),
                    context: d.context.clone(),
                    verifier: applicability.verifier.clone(),
                    verifier_family: applicability.verifier_family.clone(),
                    applicability_rule: applicability.applicability_rule.clone(),
                    result: if satisfied {
                        CheckResult::Accredited
                    } else {
                        CheckResult::Refuted
                    },
                }
            }
            _ => return Err(AdmissionError::CoreInvariant),
        };
        results.push(
            crate::requirements_bridge::resolve_requirement_result(d, &[&check])
                .map_err(|_| AdmissionError::CoreInvariant)?,
        );
    }
    Ok(results)
}
