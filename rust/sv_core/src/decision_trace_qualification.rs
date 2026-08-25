//! Cualificación explícita del resultado de obligación conservado por R1-5.
//!
//! `ResolvedRequirementResult` conserva el resultado resuelto por 3A/3B/3C.
//! La cobertura de 3D puede degradar una acreditación nominal a `D-N`. La traza
//! de R1-5 conserva ambas piezas causales —resultado resuelto, regla de cobertura
//! y participantes— y esta función deriva de forma canónica el resultado ya
//! cualificado, sin reabrir la resolución ni inventar un estado nuevo.

use std::collections::BTreeSet;

use crate::control::{CheckResult, VerifierRef};
use crate::decision_trace::RequirementDecisionTrace;

/// Devuelve el resultado de una obligación después de aplicar la cualificación
/// de cobertura representada en su propia traza.
///
/// La regla coincide con R1-3/3D:
///
/// - `D-R` se conserva;
/// - `D-N` se conserva;
/// - `D-A` sólo permanece `D-A` si existe regla de cobertura y todos los
///   verificadores requeridos participaron;
/// - en otro caso, `D-A` queda cualificado como `D-N`.
///
/// La función no produce `Tri`, permiso, autoridad ni capacidad de ejecución.
pub fn qualified_requirement_trace_result(trace: &RequirementDecisionTrace) -> CheckResult {
    qualify(
        trace.result(),
        trace
            .coverage_rule()
            .map(|rule| rule.required_verifiers().cloned().collect()),
        trace.participating_verifiers().cloned().collect(),
    )
}

fn qualify(
    resolved: CheckResult,
    required: Option<BTreeSet<VerifierRef>>,
    participating: BTreeSet<VerifierRef>,
) -> CheckResult {
    match resolved {
        CheckResult::Refuted => CheckResult::Refuted,
        CheckResult::NotVerifiable => CheckResult::NotVerifiable,
        CheckResult::Accredited => match required {
            Some(required) if required.is_subset(&participating) => CheckResult::Accredited,
            Some(_) | None => CheckResult::NotVerifiable,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ControlId;

    fn verifier(value: &str) -> VerifierRef {
        VerifierRef::from_core_id(ControlId::new(value).unwrap())
    }

    #[test]
    fn refutation_is_preserved_by_trace_qualification() {
        assert_eq!(
            qualify(CheckResult::Refuted, None, BTreeSet::new()),
            CheckResult::Refuted
        );
    }

    #[test]
    fn nominal_accreditation_without_coverage_is_traced_as_dn() {
        assert_eq!(
            qualify(CheckResult::Accredited, None, BTreeSet::new()),
            CheckResult::NotVerifiable
        );
    }

    #[test]
    fn incomplete_coverage_degrades_nominal_da_to_dn() {
        assert_eq!(
            qualify(
                CheckResult::Accredited,
                Some([verifier("v:1"), verifier("v:2")].into_iter().collect()),
                [verifier("v:1")].into_iter().collect(),
            ),
            CheckResult::NotVerifiable
        );
    }

    #[test]
    fn complete_coverage_preserves_da() {
        assert_eq!(
            qualify(
                CheckResult::Accredited,
                Some([verifier("v:1"), verifier("v:2")].into_iter().collect()),
                [verifier("v:2"), verifier("v:1")].into_iter().collect(),
            ),
            CheckResult::Accredited
        );
    }
}
