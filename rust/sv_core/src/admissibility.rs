use crate::Nat;

/// Código diagnóstico canónico de una declaración inválida de `AdmissibilitySpec`.
pub const ADMISSIBILITY_DIAGNOSTIC_CODE: &str = "E110";

/// Estado técnico de admisibilidad.
///
/// Este tipo es deliberadamente distinto de `Tri`. En particular,
/// `NotAdmitted` no constituye `Tri::U`.
///
/// ```compile_fail
/// use sv_core::{AdmissibilityState, Tri};
///
/// let _: Tri = AdmissibilityState::NotAdmitted.into();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdmissibilityState {
    Ok,
    Degraded,
    NotAdmitted,
}

impl AdmissibilityState {
    /// Representación textual correspondiente a la gramática superficial 0.2.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ok => "Ok",
            Self::Degraded => "Degraded",
            Self::NotAdmitted => "NotAdmitted",
        }
    }

    /// Indica si el estado representa una observación positivamente admitida.
    ///
    /// Esta propiedad no ternariza la observación: sólo `Ok` y `Degraded`
    /// permiten continuar hacia una ternarización semántica posterior.
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Ok | Self::Degraded)
    }
}

impl TryFrom<&str> for AdmissibilityState {
    type Error = InvalidAdmissibilitySpec;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Ok" => Ok(Self::Ok),
            "Degraded" => Ok(Self::Degraded),
            "NotAdmitted" => Ok(Self::NotAdmitted),
            other => Err(InvalidAdmissibilitySpec::InvalidStateLabel(
                other.to_owned(),
            )),
        }
    }
}

impl TryFrom<String> for AdmissibilityState {
    type Error = InvalidAdmissibilitySpec;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// Resultado material de captura antes de cualquier decisión de admisibilidad.
///
/// `Bottom` representa un fallo técnico de captura. No pertenece a `Tri` y no
/// dispone de conversión automática a ningún valor ternario.
///
/// ```compile_fail
/// use sv_core::{CaptureOutcome, Tri};
///
/// let failure = CaptureOutcome::<()>::Bottom;
/// let _: Tri = failure.into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureOutcome<T> {
    Observation(T),
    Bottom,
}

/// Especificación de admisibilidad constituida en el núcleo Rust.
///
/// El conjunto de estados no se recibe como una colección abierta: queda
/// cerrado por el propio tipo `AdmissibilityState` a `Ok`, `Degraded` y
/// `NotAdmitted`. La validación de sintaxis completa y la resolución nominal
/// permanecen en etapas posteriores de R0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissibilitySpec {
    name: String,
    parameter_id: Nat,
    rule: String,
}

impl AdmissibilitySpec {
    pub fn new(
        name: impl Into<String>,
        parameter_id: Nat,
        rule: impl Into<String>,
    ) -> Result<Self, InvalidAdmissibilitySpec> {
        if parameter_id.as_decimal() == "0" {
            return Err(InvalidAdmissibilitySpec::NonPositiveParameterId);
        }

        let rule = rule.into();
        if rule.is_empty() {
            return Err(InvalidAdmissibilitySpec::MissingRule);
        }

        Ok(Self {
            name: name.into(),
            parameter_id,
            rule,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameter_id(&self) -> &Nat {
        &self.parameter_id
    }

    pub fn rule(&self) -> &str {
        &self.rule
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidAdmissibilitySpec {
    InvalidStateLabel(String),
    NonPositiveParameterId,
    MissingRule,
}

impl InvalidAdmissibilitySpec {
    pub const fn diagnostic_code(&self) -> &'static str {
        ADMISSIBILITY_DIAGNOSTIC_CODE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tri;

    #[test]
    fn admissibility_state_is_exactly_the_closed_v0_2_set() {
        assert_eq!(AdmissibilityState::try_from("Ok"), Ok(AdmissibilityState::Ok));
        assert_eq!(
            AdmissibilityState::try_from("Degraded"),
            Ok(AdmissibilityState::Degraded)
        );
        assert_eq!(
            AdmissibilityState::try_from("NotAdmitted"),
            Ok(AdmissibilityState::NotAdmitted)
        );

        for legacy_or_foreign in ["Failed", "U", "Zero", "One", "Bottom", ""] {
            let error = AdmissibilityState::try_from(legacy_or_foreign)
                .expect_err("el estado no pertenece a AdmissibilityState");
            assert_eq!(error.diagnostic_code(), "E110");
        }
    }

    #[test]
    fn admissibility_labels_are_exact() {
        assert_eq!(AdmissibilityState::Ok.label(), "Ok");
        assert_eq!(AdmissibilityState::Degraded.label(), "Degraded");
        assert_eq!(AdmissibilityState::NotAdmitted.label(), "NotAdmitted");
    }

    #[test]
    fn only_ok_and_degraded_are_positively_admitted() {
        assert!(AdmissibilityState::Ok.is_admitted());
        assert!(AdmissibilityState::Degraded.is_admitted());
        assert!(!AdmissibilityState::NotAdmitted.is_admitted());
    }

    #[test]
    fn admissibility_spec_rejects_zero_parameter_id() {
        let error = AdmissibilitySpec::new("A0", Nat::from_u64(0), "Rule")
            .expect_err("parameter_id debe ser positivo");
        assert_eq!(error, InvalidAdmissibilitySpec::NonPositiveParameterId);
        assert_eq!(error.diagnostic_code(), "E110");
    }

    #[test]
    fn admissibility_spec_rejects_missing_rule() {
        let error = AdmissibilitySpec::new("A0", Nat::from_u64(1), "")
            .expect_err("rule no puede faltar");
        assert_eq!(error, InvalidAdmissibilitySpec::MissingRule);
        assert_eq!(error.diagnostic_code(), "E110");
    }

    #[test]
    fn valid_admissibility_spec_preserves_identity_parameter_and_rule() {
        let spec = AdmissibilitySpec::new("A0", Nat::from_u64(7), "Rule7")
            .expect("AdmissibilitySpec válido");
        assert_eq!(spec.name(), "A0");
        assert_eq!(spec.parameter_id().as_decimal(), "7");
        assert_eq!(spec.rule(), "Rule7");
    }

    #[test]
    fn technical_capture_failure_remains_outside_tri() {
        let failure: CaptureOutcome<&str> = CaptureOutcome::Bottom;
        assert!(matches!(failure, CaptureOutcome::Bottom));
        assert_eq!(Tri::U.label(), "U");
    }

    #[test]
    fn admitted_observation_does_not_itself_create_tri() {
        let observation = CaptureOutcome::Observation("dato");
        assert_eq!(observation, CaptureOutcome::Observation("dato"));
        assert!(AdmissibilityState::Ok.is_admitted());
        assert!(AdmissibilityState::Degraded.is_admitted());
    }
}
