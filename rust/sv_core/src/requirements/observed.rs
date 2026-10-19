//! RETP-154. Comparador exacto instalado de confianza para una
//! obligación específica. No decide autoridad, verdad profesional ni permiso.
#![forbid(unsafe_code)]

use super::{RequirementCheck, RequirementClass, RequirementDescriptor, VerifierApplicability};
use crate::authority::transitions::AuthorityContinuity;
use crate::control::CheckResult;
use std::{fmt, sync::Arc};

pub const MAX_CHECK_BYTES: usize = 65536;

#[derive(Debug, PartialEq, Eq)]
pub enum ObservedCheckError {
    ForeignBinding,
    CoreRequirement,
    EmptyContract,
    TooLarge,
}

pub struct PreparedExactCheck<'a> {
    continuity: &'a AuthorityContinuity,
    descriptor: &'a RequirementDescriptor,
    applicability: &'a VerifierApplicability,
    contract: Arc<[u8]>,
    expected: Arc<[u8]>,
}

pub struct ObservedExactCheck<'a> {
    continuity: &'a AuthorityContinuity,
    check: RequirementCheck,
    contract: Arc<[u8]>,
    expected: Arc<[u8]>,
    observed: Option<Box<[u8]>>,
}

// ES: Las ejecuciones comparten una referencia inmutable; no duplican contrato
// y patrón por observación. Debug omite contenido, identidades y requisitos.
// EN: Runs share immutable reference data instead of copying the contract and
// expected bytes per observation. Debug omits content, identities and requirements.
impl fmt::Debug for PreparedExactCheck<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreparedExactCheck").finish_non_exhaustive()
    }
}

impl fmt::Debug for ObservedExactCheck<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObservedExactCheck")
            .field("result", &self.result())
            .field("observed", &self.observed.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a> PreparedExactCheck<'a> {
    /// No basta que una obligación sea Specific: el instalador competente
    /// debe haber fijado que su significado es esta igualdad exacta. Los
    /// bytes del contrato conservan esa declaración; no la interpretan.
    /// EN: The trusted installer must establish that this specific obligation
    /// means exact byte equality; storing contract text does not interpret it.
    #[allow(dead_code)] // Productor institucional pendiente; no se abre al proponente.
    pub(crate) fn prepare(
        continuity: &'a AuthorityContinuity,
        descriptor: &'a RequirementDescriptor,
        applicability: &'a VerifierApplicability,
        contract: &[u8],
        expected: &[u8],
    ) -> Result<Self, ObservedCheckError> {
        let own = continuity
            .requirement_set(
                descriptor.form(),
                descriptor.effect_family(),
                descriptor.context(),
            )
            .and_then(|s| s.requirement(descriptor.reference()));
        let applicable = continuity.verifier_applicability(
            descriptor.reference(),
            applicability.verifier(),
            descriptor.context(),
        );
        if !own.is_some_and(|d| std::ptr::eq(d, descriptor))
            || !applicable.is_some_and(|v| std::ptr::eq(v, applicability))
            || !descriptor.accepts_applicability(applicability)
        {
            return Err(ObservedCheckError::ForeignBinding);
        }
        if descriptor.class() != RequirementClass::Specific {
            return Err(ObservedCheckError::CoreRequirement);
        }
        if contract.is_empty() {
            return Err(ObservedCheckError::EmptyContract);
        }
        if contract.len() > MAX_CHECK_BYTES || expected.len() > MAX_CHECK_BYTES {
            return Err(ObservedCheckError::TooLarge);
        }
        Ok(Self {
            continuity,
            descriptor,
            applicability,
            contract: contract.into(),
            expected: expected.into(),
        })
    }

    /// La evidencia presente y vacía es distinta de evidencia no disponible.
    /// El resultado lo calcula este código; no es argumento del llamador.
    /// EN: Present empty evidence differs from unavailable evidence. This code
    /// computes the result; callers cannot supply an accreditation value.
    pub fn run(
        &self,
        observed: Option<&[u8]>,
    ) -> Result<ObservedExactCheck<'a>, ObservedCheckError> {
        if observed.is_some_and(|v| v.len() > MAX_CHECK_BYTES) {
            return Err(ObservedCheckError::TooLarge);
        }
        let result = match observed {
            None => CheckResult::NotVerifiable,
            Some(v) if v == &*self.expected => CheckResult::Accredited,
            Some(_) => CheckResult::Refuted,
        };
        let d = self.descriptor;
        let a = self.applicability;
        let check = RequirementCheck {
            requirement: d.reference.clone(),
            form: d.form.clone(),
            effect_family: d.effect_family.clone(),
            context: d.context.clone(),
            verifier: a.verifier.clone(),
            verifier_family: a.verifier_family.clone(),
            applicability_rule: a.applicability_rule.clone(),
            result,
        };
        Ok(ObservedExactCheck {
            continuity: self.continuity,
            check,
            contract: self.contract.clone(),
            expected: self.expected.clone(),
            observed: observed.map(Into::into),
        })
    }
}

impl ObservedExactCheck<'_> {
    pub fn result(&self) -> CheckResult {
        self.check.result()
    }
    pub fn contract(&self) -> &[u8] {
        &self.contract
    }
    pub fn expected(&self) -> &[u8] {
        &self.expected
    }
    pub fn observed(&self) -> Option<&[u8]> {
        self.observed.as_deref()
    }
    pub fn belongs_to(&self, continuity: &AuthorityContinuity) -> bool {
        std::ptr::eq(self.continuity, continuity)
    }
    // No salida de RequirementCheck/ResolvedRequirementResult sin su vínculo:
    // aún falta cerrar la recepción profesional y todos sus verificadores.
}
