//! Constitución inicial de `Req`, `Applicable`, reglas de conflicto y cobertura para R1-3.
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
    ApplicabilityRuleRef, ConflictResolutionRuleRef, ContextRef, CoverageRuleRef,
    EffectFamilyRef, FormRef, RequirementRef, VerifierFamilyRef, VerifierRef,
};
use crate::requirements_conflict::ConflictResolutionRule;
use crate::requirements_coverage::CoverageRule;

/// Propuesta ordinaria de una regla de resolución de conflicto para una
/// obligación.
///
/// La propuesta sólo identifica la regla y el verificador que pretende quedar
/// fijado como decisivo. Las restantes ligaduras se derivan de la obligación y
/// de `Applicable(V,q,C)` durante T-0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictResolutionRuleProposal {
    reference: ConflictResolutionRuleRef,
    decisive_verifier: VerifierRef,
}

impl ConflictResolutionRuleProposal {
    pub fn new(reference: ConflictResolutionRuleRef, decisive_verifier: VerifierRef) -> Self {
        Self {
            reference,
            decisive_verifier,
        }
    }
}

/// Propuesta ordinaria de una regla de cobertura para una obligación.
///
/// La propuesta identifica la regla y el conjunto concreto de verificadores que
/// pretende hacer exigibles. Las ligaduras materiales se derivan de la
/// obligación y cada verificador deberá disponer de `Applicable(V,q,C)` antes
/// de que T-0 constituya la regla.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageRuleProposal {
    reference: CoverageRuleRef,
    required_verifiers: Vec<VerifierRef>,
}

impl CoverageRuleProposal {
    pub fn new(
        reference: CoverageRuleRef,
        required_verifiers: impl IntoIterator<Item = VerifierRef>,
    ) -> Self {
        Self {
            reference,
            required_verifiers: required_verifiers.into_iter().collect(),
        }
    }
}

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
    conflict_resolution_rule: Option<ConflictResolutionRuleProposal>,
    coverage_rule: Option<CoverageRuleProposal>,
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
            conflict_resolution_rule: None,
            coverage_rule: None,
        }
    }

    /// Adjunta una propuesta de regla de conflicto a esta obligación.
    ///
    /// La adjunción no constituye la regla. La conversión sólo puede producirse
    /// dentro de T-0 y exige que el verificador decisivo sea ya aplicable a la
    /// obligación y al contexto constituidos.
    pub fn with_conflict_resolution_rule(
        mut self,
        rule: ConflictResolutionRuleProposal,
    ) -> Self {
        self.conflict_resolution_rule = Some(rule);
        self
    }

    /// Adjunta una propuesta de regla de cobertura a esta obligación.
    ///
    /// La adjunción no constituye la regla. T-0 deberá acreditar que cada
    /// verificador requerido ya dispone de una relación `Applicable(V,q,C)`
    /// válida para la misma obligación y contexto.
    pub fn with_coverage_rule(mut self, rule: CoverageRuleProposal) -> Self {
        self.coverage_rule = Some(rule);
        self
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
    DuplicateConflictResolutionRuleRef(ConflictResolutionRuleRef),
    UnknownRequirementForConflictRule(RequirementRef),
    ConflictResolverNotApplicable {
        requirement: RequirementRef,
        verifier: VerifierRef,
    },
    DuplicateCoverageRuleRef(CoverageRuleRef),
    EmptyCoverageRequiredVerifierSet(RequirementRef),
    DuplicateCoverageRequiredVerifier {
        requirement: RequirementRef,
        verifier: VerifierRef,
    },
    UnknownRequirementForCoverageRule(RequirementRef),
    CoverageVerifierNotApplicable {
        requirement: RequirementRef,
        verifier: VerifierRef,
    },
}

/// Estado inicial constituido de requisitos, aplicabilidad y reglas gobernadas.
///
/// No tiene constructor público. Sólo T-0 puede obtenerlo a partir de
/// propuestas mediante `constitute_initial`.
#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct InitialRequirementState {
    sets: BTreeMap<RequirementBinding, RequirementSet>,
    applicabilities: BTreeMap<ApplicabilityKey, VerifierApplicability>,
    coverage_rules: BTreeMap<RequirementRef, CoverageRule>,
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

    pub(crate) fn coverage_rule(&self, requirement: &RequirementRef) -> Option<&CoverageRule> {
        self.coverage_rules.get(requirement)
    }

    pub(crate) fn requirement_set_count(&self) -> usize {
        self.sets.len()
    }

    pub(crate) fn applicability_count(&self) -> usize {
        self.applicabilities.len()
    }

    pub(crate) fn coverage_rule_count(&self) -> usize {
        self.coverage_rules.len()
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
    token: &GenesisControlToken,
    controlled_bindings: impl IntoIterator<Item = RequirementBinding>,
    proposals: impl IntoIterator<Item = RequirementProposal>,
    applicability_proposals: impl IntoIterator<Item = ApplicabilityProposal>,
) -> Result<InitialRequirementState, InitialRequirementError> {
    let controlled: BTreeSet<_> = controlled_bindings.into_iter().collect();
    let mut grouped: BTreeMap<RequirementBinding, Vec<RequirementDescriptor>> = BTreeMap::new();
    let mut reference_to_binding: BTreeMap<RequirementRef, RequirementBinding> = BTreeMap::new();
    let mut pending_conflict_rules: BTreeMap<RequirementRef, ConflictResolutionRuleProposal> =
        BTreeMap::new();
    let mut conflict_rule_refs = BTreeSet::new();
    let mut pending_coverage_rules: BTreeMap<RequirementRef, CoverageRuleProposal> =
        BTreeMap::new();
    let mut coverage_rule_refs = BTreeSet::new();

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

        if let Some(rule) = proposal.conflict_resolution_rule {
            if !conflict_rule_refs.insert(rule.reference.clone()) {
                return Err(InitialRequirementError::DuplicateConflictResolutionRuleRef(
                    rule.reference,
                ));
            }
            pending_conflict_rules.insert(proposal.reference.clone(), rule);
        }

        if let Some(rule) = proposal.coverage_rule {
            if !coverage_rule_refs.insert(rule.reference.clone()) {
                return Err(InitialRequirementError::DuplicateCoverageRuleRef(
                    rule.reference,
                ));
            }
            if rule.required_verifiers.is_empty() {
                return Err(InitialRequirementError::EmptyCoverageRequiredVerifierSet(
                    proposal.reference,
                ));
            }
            let mut seen = BTreeSet::new();
            for verifier in &rule.required_verifiers {
                if !seen.insert(verifier.clone()) {
                    return Err(InitialRequirementError::DuplicateCoverageRequiredVerifier {
                        requirement: proposal.reference,
                        verifier: verifier.clone(),
                    });
                }
            }
            pending_coverage_rules.insert(proposal.reference.clone(), rule);
        }

        grouped.entry(binding).or_default().push(RequirementDescriptor {
            reference: proposal.reference,
            class: proposal.class,
            form: proposal.form,
            effect_family: proposal.effect_family,
            context: proposal.context,
            admissible_verifier_families: families,
            applicability_rule: proposal.applicability_rule,
            conflict_resolution_rule: None,
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
        let Some(descriptor) = sets
            .get(binding)
            .and_then(|set| set.requirement(&proposal.requirement))
        else {
            return Err(InitialRequirementError::UnknownRequirementForApplicability(
                proposal.requirement,
            ));
        };

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

    for (requirement, proposal) in pending_conflict_rules {
        let Some(binding) = reference_to_binding.get(&requirement).cloned() else {
            return Err(InitialRequirementError::UnknownRequirementForConflictRule(
                requirement,
            ));
        };

        let Some(descriptor) = sets
            .get(&binding)
            .and_then(|set| set.requirement(&requirement))
        else {
            return Err(InitialRequirementError::UnknownRequirementForConflictRule(
                requirement,
            ));
        };

        let key = ApplicabilityKey {
            requirement: requirement.clone(),
            verifier: proposal.decisive_verifier.clone(),
            context: descriptor.context().clone(),
        };

        let Some(applicability) = applicabilities.get(&key) else {
            return Err(InitialRequirementError::ConflictResolverNotApplicable {
                requirement,
                verifier: proposal.decisive_verifier,
            });
        };

        let rule = ConflictResolutionRule::constitute_from_genesis(
            token,
            proposal.reference,
            descriptor,
            applicability,
        );

        let Some(descriptor_mut) = sets
            .get_mut(&binding)
            .and_then(|set| set.requirements.get_mut(&requirement))
        else {
            return Err(InitialRequirementError::UnknownRequirementForConflictRule(
                requirement,
            ));
        };
        descriptor_mut.conflict_resolution_rule = Some(rule);
    }

    let mut coverage_rules = BTreeMap::new();
    for (requirement, proposal) in pending_coverage_rules {
        let Some(binding) = reference_to_binding.get(&requirement).cloned() else {
            return Err(InitialRequirementError::UnknownRequirementForCoverageRule(
                requirement,
            ));
        };
        let Some(descriptor) = sets
            .get(&binding)
            .and_then(|set| set.requirement(&requirement))
        else {
            return Err(InitialRequirementError::UnknownRequirementForCoverageRule(
                requirement,
            ));
        };

        let mut required_verifiers = BTreeSet::new();
        for verifier in proposal.required_verifiers {
            let key = ApplicabilityKey {
                requirement: requirement.clone(),
                verifier: verifier.clone(),
                context: descriptor.context().clone(),
            };
            if !applicabilities.contains_key(&key) {
                return Err(InitialRequirementError::CoverageVerifierNotApplicable {
                    requirement,
                    verifier,
                });
            }
            required_verifiers.insert(verifier);
        }

        let rule = CoverageRule::constitute_from_genesis(
            token,
            proposal.reference,
            descriptor,
            required_verifiers,
        );
        coverage_rules.insert(requirement, rule);
    }

    Ok(InitialRequirementState {
        sets,
        applicabilities,
        coverage_rules,
    })
}
