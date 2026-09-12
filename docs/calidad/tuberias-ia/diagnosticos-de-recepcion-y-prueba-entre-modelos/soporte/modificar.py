from pathlib import Path
s=Path('tmp/continuacion-157/candidata/rust/sv_core/src')
p=s/'authority/transitions/reception.rs';t=p.read_text()
t=t.replace('use super::{', 'use crate::audit_diagnostics::{Diagnostic, DiagnosticCause as Cause, DiagnosticStage as Stage};\nuse super::{',1)
a=t.index('fn bounds(');b=t.index('\nimpl PreparedGenesisReception',a)
t=t[:a]+'''/// Error original y detalle emitidos juntos. La prosa no decide la recepción.
#[derive(Debug, PartialEq, Eq)]
pub struct ReceptionFailure { error: ReceptionError, diagnostic: Diagnostic }
impl ReceptionFailure {
    pub fn error(&self) -> &ReceptionError { &self.error }
    pub fn diagnostic(&self) -> Diagnostic { self.diagnostic }
    fn new(error: ReceptionError, cause: Cause, stage: Stage) -> Self {
        Self { error, diagnostic: Diagnostic::new(cause, stage) }
    }
    fn into_legacy(self) -> ReceptionError { self.error }
}
fn bounds(parts: &[&[u8];3], stage: Stage) -> Result<(), ReceptionFailure> {
    // Conserva precedencia histórica: todos los vacíos antes de los excesos.
    for (index, part) in parts.iter().enumerate() {
        if part.is_empty() {
            let cause = [Cause::EmptyAct, Cause::EmptyIssuer, Cause::EmptyVersion][index];
            return Err(ReceptionFailure::new(ReceptionError::Empty, cause, stage));
        }
    }
    for (index, part) in parts.iter().enumerate() {
        if part.len() > MAX_RECEPTION_BYTES {
            let cause = [Cause::ActTooLarge, Cause::IssuerTooLarge, Cause::VersionTooLarge][index];
            return Err(ReceptionFailure::new(ReceptionError::TooLarge, cause, stage));
        }
    }
    Ok(())
}
''' + t[b:]
a=t.index('        bounds(&[act, issuer.as_bytes(), version.as_bytes()])?;')
t=t[:a]+'''        Self::prepare_diagnosed(premise, plan, act, issuer, version).map_err(ReceptionFailure::into_legacy)
    }

    #[allow(dead_code)]
    pub(crate) fn prepare_diagnosed(
        premise: ExternalGenesisPremise, plan: GenesisPlan,
        act: &[u8], issuer: &str, version: &str,
    ) -> Result<Self, ReceptionFailure> {
        bounds(&[act, issuer.as_bytes(), version.as_bytes()], Stage::PrepareReception)?;''' + t[a+len('        bounds(&[act, issuer.as_bytes(), version.as_bytes()])?;'):]
a=t.index('        if self.plan.is_none()');b=t.index('        Ok(ReceivedGenesis',a)
t=t[:a]+'''        self.receive_diagnosed(act, issuer, version).map_err(ReceptionFailure::into_legacy)
    }

    pub fn receive_diagnosed(&mut self, act: &[u8], issuer: &str, version: &str)
        -> Result<ReceivedGenesis, ReceptionFailure>
    {
        let failure = |error, cause| ReceptionFailure::new(error, cause, Stage::Receive);
        if self.plan.is_none() {
            return Err(failure(ReceptionError::AlreadyAttempted, Cause::AlreadyAttempted));
        }
        bounds(&[act, issuer.as_bytes(), version.as_bytes()], Stage::Receive)?;
        match (issuer != &*self.issuer, version != &*self.version) {
            (true, true) => return Err(failure(ReceptionError::DeclarationMismatch, Cause::DeclarationsMismatch)),
            (true, false) => return Err(failure(ReceptionError::DeclarationMismatch, Cause::IssuerMismatch)),
            (false, true) => return Err(failure(ReceptionError::DeclarationMismatch, Cause::VersionMismatch)),
            (false, false) => (),
        }
        if act != &*self.act { return Err(failure(ReceptionError::ActMismatch, Cause::ActMismatch)); }
        let plan = self.plan.take().ok_or_else(|| failure(ReceptionError::AlreadyAttempted, Cause::AlreadyAttempted))?;
        let mut continuity = AuthorityContinuity::uninhabited();
        continuity.apply_genesis(&mut self.premise, plan)
            .map_err(|e| failure(ReceptionError::Genesis(e), Cause::GenesisRejected))?;
''' +t[b:]
p.write_text(t)
p=s/'requirements/observed.rs';t=p.read_text()
t=t.replace('use super::{','use crate::audit_diagnostics::{Diagnostic, DiagnosticCause as Cause, DiagnosticStage as Stage};\nuse super::{',1)
i=t.index('\n#[derive(Debug)]')
t=t[:i]+'''
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
''' +t[i:]
t=t.replace('    observed: Option<Box<[u8]>>,','    observed: Option<Box<[u8]>>,\n    diagnostic: Option<Diagnostic>,')
a=t.index('        let own =')
t=t[:a]+'''        Self::prepare_diagnosed(continuity, descriptor, applicability, contract, expected)
            .map_err(ObservedCheckFailure::into_legacy)
    }

    #[allow(dead_code)]
    pub(crate) fn prepare_diagnosed(
        continuity: &'a AuthorityContinuity, descriptor: &'a RequirementDescriptor,
        applicability: &'a VerifierApplicability, contract: &[u8], expected: &[u8],
    ) -> Result<Self, ObservedCheckFailure> {
        let failure = |error, cause| ObservedCheckFailure::new(error, cause, Stage::PrepareCheck);
''' +t[a:]
t=t.replace('return Err(ObservedCheckError::ForeignBinding);','return Err(failure(ObservedCheckError::ForeignBinding, Cause::ForeignBinding));')
t=t.replace('return Err(ObservedCheckError::CoreRequirement);','return Err(failure(ObservedCheckError::CoreRequirement, Cause::CoreRequirement));')
t=t.replace('if contract.is_empty() { return Err(ObservedCheckError::EmptyContract); }','if contract.is_empty() { return Err(failure(ObservedCheckError::EmptyContract, Cause::EmptyContract)); }')
t=t.replace('''        if contract.len() > MAX_CHECK_BYTES || expected.len() > MAX_CHECK_BYTES {
            return Err(ObservedCheckError::TooLarge);
        }''','''        if contract.len() > MAX_CHECK_BYTES {
            return Err(failure(ObservedCheckError::TooLarge, Cause::ContractTooLarge));
        }
        if expected.len() > MAX_CHECK_BYTES {
            return Err(failure(ObservedCheckError::TooLarge, Cause::ExpectedTooLarge));
        }''')
a=t.index('        if observed.is_some_and');b=t.index('        let d = self.descriptor;',a)
t=t[:a]+'''        self.run_diagnosed(observed).map_err(ObservedCheckFailure::into_legacy)
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
''' +t[b:]
t=t.replace('observed: observed.map(Into::into) })','observed: observed.map(Into::into), diagnostic })')
t=t.replace("impl ObservedExactCheck<'_> {", "impl ObservedExactCheck<'_> {\n    pub fn diagnostic(&self) -> Option<Diagnostic> { self.diagnostic }")
p.write_text(t)
print('dos fronteras ampliadas; API previa conservada')
