//! Candidata RETP-154. Comparador exacto instalado de confianza para una
//! obligación específica. No decide autoridad, verdad profesional ni permiso.
use crate::audit_diagnostics::{Diagnostic, DiagnosticCause as Cause, DiagnosticStage as Stage};
use super::{RequirementCheck, RequirementClass, RequirementDescriptor, VerifierApplicability};
use crate::authority::transitions::AuthorityContinuity;
use crate::control::CheckResult;

pub const MAX_CHECK_BYTES: usize = 65536;

#[derive(Debug, PartialEq, Eq)]
pub enum ObservedCheckError { ForeignBinding, CoreRequirement, EmptyContract, TooLarge }

#[derive(Debug, PartialEq, Eq)]
pub struct ObservedCheckFailure { error: ObservedCheckError, diagnostic: Diagnostic }
impl ObservedCheckFailure {
    pub fn error(&self) -> &ObservedCheckError { &self.error }
    pub fn diagnostic(&self) -> Diagnostic { self.diagnostic }
    fn new(error: ObservedCheckError, cause: Cause, stage: Stage) -> Self {
        Self { error, diagnostic: Diagnostic::new(cause, stage) }
    }
    fn into_legacy(self) -> ObservedCheckError { self.error }
}

#[derive(Debug)]
pub struct PreparedExactCheck<'a> {
    continuity: &'a AuthorityContinuity,
    descriptor: &'a RequirementDescriptor,
    applicability: &'a VerifierApplicability,
    contract: Box<[u8]>,
    expected: Box<[u8]>,
}

#[derive(Debug)]
pub struct ObservedExactCheck<'a> {
    continuity: &'a AuthorityContinuity,
    check: RequirementCheck,
    contract: Box<[u8]>,
    expected: Box<[u8]>,
    observed: Option<Box<[u8]>>,
    diagnostic: Option<Diagnostic>,
}

impl<'a> PreparedExactCheck<'a> {
    /// No basta que una obligación sea Specific: el instalador competente
    /// debe haber fijado que su significado es esta igualdad exacta. Los
    /// bytes del contrato conservan esa declaración; no la interpretan.
    #[allow(dead_code)]
    pub(crate) fn prepare(
        continuity: &'a AuthorityContinuity, descriptor: &'a RequirementDescriptor,
        applicability: &'a VerifierApplicability, contract: &[u8], expected: &[u8],
    ) -> Result<Self, ObservedCheckError> {
        Self::prepare_diagnosed(continuity, descriptor, applicability, contract, expected)
            .map_err(ObservedCheckFailure::into_legacy)
    }

    #[allow(dead_code)]
    pub(crate) fn prepare_diagnosed(
        continuity: &'a AuthorityContinuity, descriptor: &'a RequirementDescriptor,
        applicability: &'a VerifierApplicability, contract: &[u8], expected: &[u8],
    ) -> Result<Self, ObservedCheckFailure> {
        let failure = |error, cause| ObservedCheckFailure::new(error, cause, Stage::PrepareCheck);
        let own = continuity.requirement_set(descriptor.form(), descriptor.effect_family(),
            descriptor.context()).and_then(|s| s.requirement(descriptor.reference()));
        let applicable = continuity.verifier_applicability(descriptor.reference(),
            applicability.verifier(), descriptor.context());
        if !own.is_some_and(|d| std::ptr::eq(d, descriptor))
            || !applicable.is_some_and(|v| std::ptr::eq(v, applicability))
            || !descriptor.accepts_applicability(applicability) {
            return Err(failure(ObservedCheckError::ForeignBinding, Cause::ForeignBinding));
        }
        if descriptor.class() != RequirementClass::Specific {
            return Err(failure(ObservedCheckError::CoreRequirement, Cause::CoreRequirement));
        }
        if contract.is_empty() { return Err(failure(ObservedCheckError::EmptyContract, Cause::EmptyContract)); }
        if contract.len() > MAX_CHECK_BYTES {
            return Err(failure(ObservedCheckError::TooLarge, Cause::ContractTooLarge));
        }
        if expected.len() > MAX_CHECK_BYTES {
            return Err(failure(ObservedCheckError::TooLarge, Cause::ExpectedTooLarge));
        }
        Ok(Self { continuity, descriptor, applicability,
            contract: contract.into(), expected: expected.into() })
    }

    /// La evidencia presente y vacía es distinta de evidencia no disponible.
    /// El resultado lo calcula este código; no es argumento del llamador.
    pub fn run(&self, observed: Option<&[u8]>)
        -> Result<ObservedExactCheck<'a>, ObservedCheckError>
    {
        self.run_diagnosed(observed).map_err(ObservedCheckFailure::into_legacy)
    }

    pub fn run_diagnosed(&self, observed: Option<&[u8]>)
        -> Result<ObservedExactCheck<'a>, ObservedCheckFailure>
    {
        if observed.is_some_and(|v| v.len() > MAX_CHECK_BYTES) {
            return Err(ObservedCheckFailure::new(ObservedCheckError::TooLarge,
                Cause::ObservedTooLarge, Stage::Compare));
        }
        let (result, diagnostic) = match observed {
            None => (CheckResult::NotVerifiable, Some(Diagnostic::new(Cause::MissingObservation, Stage::Compare))),
            Some(v) if v == &*self.expected => (CheckResult::Accredited, None),
            Some(_) => (CheckResult::Refuted, Some(Diagnostic::new(Cause::ExactMismatch, Stage::Compare))),
        };
        let d = self.descriptor;
        let a = self.applicability;
        let check = RequirementCheck {
            requirement: d.reference.clone(), form: d.form.clone(),
            effect_family: d.effect_family.clone(), context: d.context.clone(),
            verifier: a.verifier.clone(), verifier_family: a.verifier_family.clone(),
            applicability_rule: a.applicability_rule.clone(), result,
        };
        Ok(ObservedExactCheck { continuity: self.continuity, check,
            contract: self.contract.clone(), expected: self.expected.clone(),
            observed: observed.map(Into::into), diagnostic })
    }
}

impl ObservedExactCheck<'_> {
    pub fn diagnostic(&self) -> Option<Diagnostic> { self.diagnostic }
    pub fn result(&self) -> CheckResult { self.check.result() }
    pub fn contract(&self) -> &[u8] { &self.contract }
    pub fn expected(&self) -> &[u8] { &self.expected }
    pub fn observed(&self) -> Option<&[u8]> { self.observed.as_deref() }
    pub fn belongs_to(&self, continuity: &AuthorityContinuity) -> bool {
        std::ptr::eq(self.continuity, continuity)
    }
    // No salida de RequirementCheck/ResolvedRequirementResult sin su vínculo:
    // aún falta cerrar la recepción profesional y todos sus verificadores.
}
