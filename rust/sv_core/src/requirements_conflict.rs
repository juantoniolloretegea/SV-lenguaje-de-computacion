//! Conflicto entre comprobaciones de una misma obligación en R1-3.
//!
//! Esta subunidad materializa únicamente el caso sin regla de resolución
//! previamente constituida. La superficie pública acepta `RequirementCheck`
//! ya selladas; no acepta resultados técnicos crudos ni constituye permiso.

use std::collections::BTreeSet;

use crate::control::{CheckResult, RequirementRef, VerifierRef};
use crate::requirements::RequirementCheck;

/// Entrada estructural inválida al resolver varias comprobaciones de una misma
/// obligación sin regla de resolución constituida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementConflictError {
    EmptyChecks,
    MixedRequirements {
        expected: RequirementRef,
        found: RequirementRef,
    },
    DuplicateVerifier(VerifierRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConflictObservation {
    requirement: RequirementRef,
    verifier: VerifierRef,
    result: CheckResult,
}

/// Resuelve varias comprobaciones selladas de una misma obligación cuando no
/// existe una regla de resolución previamente constituida.
///
/// Si todas coinciden, conserva el resultado común. Si existe incompatibilidad
/// entre resultados de verificadores nominalmente distintos, devuelve `D-N`.
/// La repetición del mismo verificador se rechaza y el orden de entrada no
/// influye en el resultado.
pub fn resolve_requirement_checks_without_rule(
    checks: &[&RequirementCheck],
) -> Result<CheckResult, RequirementConflictError> {
    resolve_observations(checks.iter().map(|check| ConflictObservation {
        requirement: check.requirement().clone(),
        verifier: check.verifier().clone(),
        result: check.result(),
    }))
}

fn resolve_observations(
    observations: impl IntoIterator<Item = ConflictObservation>,
) -> Result<CheckResult, RequirementConflictError> {
    let mut observations = observations.into_iter();
    let Some(first) = observations.next() else {
        return Err(RequirementConflictError::EmptyChecks);
    };

    let expected_requirement = first.requirement.clone();
    let common_result = first.result;
    let mut conflict = false;
    let mut verifiers = BTreeSet::new();
    verifiers.insert(first.verifier);

    for observation in observations {
        if observation.requirement != expected_requirement {
            return Err(RequirementConflictError::MixedRequirements {
                expected: expected_requirement,
                found: observation.requirement,
            });
        }

        if !verifiers.insert(observation.verifier.clone()) {
            return Err(RequirementConflictError::DuplicateVerifier(
                observation.verifier,
            ));
        }

        if observation.result != common_result {
            conflict = true;
        }
    }

    if conflict {
        Ok(CheckResult::NotVerifiable)
    } else {
        Ok(common_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ControlId;

    fn requirement(value: &str) -> RequirementRef {
        RequirementRef::from_core_id(ControlId::new(value).unwrap())
    }

    fn verifier(value: &str) -> VerifierRef {
        VerifierRef::from_core_id(ControlId::new(value).unwrap())
    }

    fn observation(
        requirement_value: &str,
        verifier_value: &str,
        result: CheckResult,
    ) -> ConflictObservation {
        ConflictObservation {
            requirement: requirement(requirement_value),
            verifier: verifier(verifier_value),
            result,
        }
    }

    #[test]
    fn empty_check_set_is_rejected() {
        assert_eq!(
            resolve_observations([]),
            Err(RequirementConflictError::EmptyChecks)
        );
    }

    #[test]
    fn mixed_requirements_are_not_a_single_conflict() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:2", "verifier:2", CheckResult::Refuted),
            ]),
            Err(RequirementConflictError::MixedRequirements {
                expected: requirement("req:1"),
                found: requirement("req:2"),
            })
        );
    }

    #[test]
    fn repeated_verifier_is_rejected_instead_of_weighted() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:1", CheckResult::Accredited),
            ]),
            Err(RequirementConflictError::DuplicateVerifier(verifier(
                "verifier:1"
            )))
        );
    }

    #[test]
    fn homogeneous_accredited_checks_remain_accredited() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:2", CheckResult::Accredited),
            ]),
            Ok(CheckResult::Accredited)
        );
    }

    #[test]
    fn homogeneous_refuted_checks_remain_refuted() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Refuted),
                observation("req:1", "verifier:2", CheckResult::Refuted),
            ]),
            Ok(CheckResult::Refuted)
        );
    }

    #[test]
    fn homogeneous_not_verifiable_checks_remain_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::NotVerifiable),
                observation("req:1", "verifier:2", CheckResult::NotVerifiable),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn accredited_and_refuted_without_rule_become_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:2", CheckResult::Refuted),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn accredited_and_not_verifiable_without_rule_remain_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Accredited),
                observation("req:1", "verifier:2", CheckResult::NotVerifiable),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn refuted_and_not_verifiable_without_rule_become_not_verifiable() {
        assert_eq!(
            resolve_observations([
                observation("req:1", "verifier:1", CheckResult::Refuted),
                observation("req:1", "verifier:2", CheckResult::NotVerifiable),
            ]),
            Ok(CheckResult::NotVerifiable)
        );
    }

    #[test]
    fn conflict_result_is_independent_of_input_order() {
        let left = resolve_observations([
            observation("req:1", "verifier:1", CheckResult::Accredited),
            observation("req:1", "verifier:2", CheckResult::Refuted),
            observation("req:1", "verifier:3", CheckResult::NotVerifiable),
        ]);
        let right = resolve_observations([
            observation("req:1", "verifier:3", CheckResult::NotVerifiable),
            observation("req:1", "verifier:1", CheckResult::Accredited),
            observation("req:1", "verifier:2", CheckResult::Refuted),
        ]);

        assert_eq!(left, Ok(CheckResult::NotVerifiable));
        assert_eq!(right, left);
    }
}
