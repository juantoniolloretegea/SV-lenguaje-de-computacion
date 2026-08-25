//! Constitución inicial de `Req` y `Applicable` para R1-3.
//!
//! Este submódulo sólo convierte propuestas en objetos constituidos cuando
//! recibe la capacidad interna emitida por la puerta T-0. No ejecuta
//! comprobaciones y no produce permiso ni efecto.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    CoreRequirementKind, RequirementClass, RequirementDescriptor, RequirementSet,
    VerifierApplicability,
};
use crate::authority::transitions::GenesisControlToken;
use crate::control::{
    ApplicabilityRuleRef, ContextRef, EffectFamilyRef, FormRef, RequirementRef,
    VerifierFamilyRef, VerifierRef,
};

/// Propuesta ordinaria de una obligación para la constitución inicial.
///
/// Una propuesta no es una obligación constituida y no puede agregarse como
/// resultado de comprobación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementProposal {
    reference: RequirementRef,
    class: RequirementClass,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    admissible_verifier_families: Vec<VerifierFamilyRef>,
    applicability_rule: ApplicabilityRuleRef,
}

impl RequirementProposal {
    pub fn new(
        reference: RequirementRef,
        class: RequirementClass,
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
        admissible_verifier_families: impl IntoIterator<Item = VerifierFamilyRef>,
        applicability_rule: ApplicabilityRuleRef,
    ) -> Self {
        Self {
            reference,
            class,
            form,
            effect_family,
            context,
            admissible_verifier_families: admissible_verifier_families.into_iter().collect(),
            applicability_rule,
        }
    }
}

/// Propuesta ordinaria de una relación `Applicable(V,q,C)` inicial.
///
/// La propuesta no acredita que `V` haya ejecutado una comprobación ni permite
/// al propio verificador constituir la relación fuera de T-0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicabilityProposal {
    verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    requirement: RequirementRef,
    context: ContextRef,
    applicability_rule: ApplicabilityRuleRef,
}

impl ApplicabilityProposal {
    pub fn new(
        verifier: VerifierRef,
        verifier_family: VerifierFamilyRef,
        requirement: RequirementRef,
        context: ContextRef,
        applicability_rule: ApplicabilityRuleRef,
    ) -> Self {
        Self {
            verifier,
            verifier_family,
            requirement,
            context,
            applicability_rule,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RequirementBinding {
    pub(crate) form: FormRef,
    pub(crate) effect_family: EffectFamilyRef,
    pub(crate) context: ContextRef,
}

impl RequirementBinding {
    pub(crate) fn new(form: FormRef, effect_family: EffectFamilyRef, context: ContextRef) -> Self {
        Self {
            form,
            effect_family,
            context,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ApplicabilityKey {
    requirement: RequirementRef,
    verifier: VerifierRef,
    context: ContextRef,
}

/// Rechazos de la constitución inicial de R1-3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitialRequirementError {
    MissingRequirementSet {
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
    },
    RequirementForUncontrolledBinding {
        requirement: RequirementRef,
    },
    DuplicateRequirementRef(RequirementRef),
    NoAdmissibleVerifierFamily(RequirementRef),
    MissingMandatoryCore {
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
        kind: CoreRequirementKind,
    },
    UnknownRequirementForApplicability(RequirementRef),
    ApplicabilityMismatch {
        requirement: RequirementRef,
        verifier: VerifierRef,
    },
    DuplicateApplicability {
        requirement: RequirementRef,
        verifier: VerifierRef,
        context: ContextRef,
    },
}

/// Estado inicial constituido de requisitos y relaciones de aplicabilidad.
///
/// No tiene constructor público. Sólo T-0 puede obtenerlo a partir de
/// propuestas mediante `constitute_initial`.
#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct InitialRequirementState {
    sets: BTreeMap<RequirementBinding, RequirementSet>,
    applicabilities: BTreeMap<ApplicabilityKey, VerifierApplicability>,
}

impl InitialRequirementState {
    pub(crate) fn requirement_set(
        &self,
        form: &FormRef,
        effect_family: &EffectFamilyRef,
        context: &ContextRef,
    ) -> Option<&RequirementSet> {
        self.sets.get(&RequirementBinding::new(
            form.clone(),
            effect_family.clone(),
            context.clone(),
        ))
    }

    pub(crate) fn applicability(
        &self,
        requirement: &RequirementRef,
        verifier: &VerifierRef,
        context: &ContextRef,
    ) -> Option<&VerifierApplicability> {
        self.applicabilities.get(&ApplicabilityKey {
            requirement: requirement.clone(),
            verifier: verifier.clone(),
            context: context.clone(),
        })
    }

    pub(crate) fn requirement_set_count(&self) -> usize {
        self.sets.len()
    }

    pub(crate) fn applicability_count(&self) -> usize {
        self.applicabilities.len()
    }
}

const MANDATORY_CORE: [CoreRequirementKind; 4] = [
    CoreRequirementKind::FormValidity,
    CoreRequirementKind::ApplicableAuthority,
    CoreRequirementKind::VerifierAdmissibilityAndApplicability,
    CoreRequirementKind::NoSelfAccreditation,
];

/// Constituye atómicamente el régimen inicial de requisitos.
///
/// La capacidad `GenesisControlToken` no tiene constructor accesible fuera de
/// la puerta T-0. Su presencia evita que otra operación interna convierta una
/// propuesta en objeto constituido por mera llamada a esta función.
pub(crate) fn constitute_initial(
    _token: &GenesisControlToken,
    controlled_bindings: impl IntoIterator<Item = RequirementBinding>,
    proposals: impl IntoIterator<Item = RequirementProposal>,
    applicability_proposals: impl IntoIterator<Item = ApplicabilityProposal>,
) -> Result<InitialRequirementState, InitialRequirementError> {
    let controlled: BTreeSet<_> = controlled_bindings.into_iter().collect();
    let mut grouped: BTreeMap<RequirementBinding, Vec<RequirementDescriptor>> = BTreeMap::new();
    let mut reference_to_binding: BTreeMap<RequirementRef, RequirementBinding> = BTreeMap::new();

    for proposal in proposals {
        let binding = RequirementBinding::new(
            proposal.form.clone(),
            proposal.effect_family.clone(),
            proposal.context.clone(),
        );

        if !controlled.contains(&binding) {
            return Err(InitialRequirementError::RequirementForUncontrolledBinding {
                requirement: proposal.reference,
            });
        }

        if reference_to_binding
            .insert(proposal.reference.clone(), binding.clone())
            .is_some()
        {
            return Err(InitialRequirementError::DuplicateRequirementRef(
                proposal.reference,
            ));
        }

        let families: BTreeSet<_> = proposal.admissible_verifier_families.into_iter().collect();
        if families.is_empty() {
            return Err(InitialRequirementError::NoAdmissibleVerifierFamily(
                proposal.reference,
            ));
        }

        grouped.entry(binding).or_default().push(RequirementDescriptor {
            reference: proposal.reference,
            class: proposal.class,
            form: proposal.form,
            effect_family: proposal.effect_family,
            context: proposal.context,
            admissible_verifier_families: families,
            applicability_rule: proposal.applicability_rule,
        });
    }

    let mut sets = BTreeMap::new();
    for binding in &controlled {
        let descriptors = grouped.remove(binding).unwrap_or_default();
        if descriptors.is_empty() {
            return Err(InitialRequirementError::MissingRequirementSet {
                form: binding.form.clone(),
                effect_family: binding.effect_family.clone(),
                context: binding.context.clone(),
            });
        }

        let mut requirements = BTreeMap::new();
        for descriptor in descriptors {
            requirements.insert(descriptor.reference.clone(), descriptor);
        }

        for mandatory in MANDATORY_CORE {
            if !requirements
                .values()
                .any(|descriptor| descriptor.class == RequirementClass::Core(mandatory))
            {
                return Err(InitialRequirementError::MissingMandatoryCore {
                    form: binding.form.clone(),
                    effect_family: binding.effect_family.clone(),
                    context: binding.context.clone(),
                    kind: mandatory,
                });
            }
        }

        sets.insert(
            binding.clone(),
            RequirementSet {
                form: binding.form.clone(),
                effect_family: binding.effect_family.clone(),
                context: binding.context.clone(),
                requirements,
            },
        );
    }

    let mut applicabilities = BTreeMap::new();
    for proposal in applicability_proposals {
        let Some(binding) = reference_to_binding.get(&proposal.requirement) else {
            return Err(InitialRequirementError::UnknownRequirementForApplicability(
                proposal.requirement,
            ));
        };
        let descriptor = sets
            .get(binding)
            .and_then(|set| set.requirement(&proposal.requirement))
            .expect("la referencia de obligación ya fue validada en el régimen inicial");

        let applicability = VerifierApplicability {
            verifier: proposal.verifier.clone(),
            verifier_family: proposal.verifier_family,
            requirement: proposal.requirement.clone(),
            context: proposal.context.clone(),
            applicability_rule: proposal.applicability_rule,
        };

        if !descriptor.accepts_applicability(&applicability) {
            return Err(InitialRequirementError::ApplicabilityMismatch {
                requirement: proposal.requirement,
                verifier: proposal.verifier,
            });
        }

        let key = ApplicabilityKey {
            requirement: applicability.requirement.clone(),
            verifier: applicability.verifier.clone(),
            context: applicability.context.clone(),
        };
        if applicabilities.insert(key.clone(), applicability).is_some() {
            return Err(InitialRequirementError::DuplicateApplicability {
                requirement: key.requirement,
                verifier: key.verifier,
                context: key.context,
            });
        }
    }

    Ok(InitialRequirementState {
        sets,
        applicabilities,
    })
}
