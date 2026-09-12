//! Candidata RETP-154: recepción posterior a admisión externa.
//! No produce la premisa opaca ni autentica al emisor declarado.
use crate::audit_diagnostics::{Diagnostic, DiagnosticCause as Cause, DiagnosticStage as Stage};
use super::{AuthorityContinuity, ExternalGenesisPremise, GenesisError, GenesisPlan};

pub const MAX_RECEPTION_BYTES: usize = 65536;

#[derive(Debug, PartialEq, Eq)]
pub enum ReceptionError {
    Empty,
    TooLarge,
    DeclarationMismatch,
    ActMismatch,
    AlreadyAttempted,
    Genesis(GenesisError),
}

#[derive(Debug)]
pub struct PreparedGenesisReception {
    premise: ExternalGenesisPremise,
    plan: Option<GenesisPlan>,
    act: Box<[u8]>,
    issuer: Box<str>,
    version: Box<str>,
}

#[derive(Debug)]
pub struct ReceivedGenesis {
    continuity: AuthorityContinuity,
    act: Box<[u8]>,
    issuer: Box<str>,
    version: Box<str>,
}

/// Error original y detalle emitidos juntos. La prosa no decide la recepción.
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

impl PreparedGenesisReception {
    /// Instalación interna de confianza; jamás llamada con un plan decidido
    /// por la propuesta recibida. La premisa sigue requiriendo su productor.
    #[allow(dead_code)]
    pub(crate) fn prepare(
        premise: ExternalGenesisPremise, plan: GenesisPlan,
        act: &[u8], issuer: &str, version: &str,
    ) -> Result<Self, ReceptionError> {
        Self::prepare_diagnosed(premise, plan, act, issuer, version).map_err(ReceptionFailure::into_legacy)
    }

    #[allow(dead_code)]
    pub(crate) fn prepare_diagnosed(
        premise: ExternalGenesisPremise, plan: GenesisPlan,
        act: &[u8], issuer: &str, version: &str,
    ) -> Result<Self, ReceptionFailure> {
        bounds(&[act, issuer.as_bytes(), version.as_bytes()], Stage::PrepareReception)?;
        Ok(Self { premise, plan: Some(plan), act: act.into(),
            issuer: issuer.into(), version: version.into() })
    }

    /// Sólo recibe datos; no acepta un plan, premisa o resultado alternativos.
    /// Discordancia permite reintento. Llegar a T-0 consume este preparado
    /// incluso si T-0 rechaza el plan; ese error se conserva y no se repara.
    pub fn receive(&mut self, act: &[u8], issuer: &str, version: &str)
        -> Result<ReceivedGenesis, ReceptionError>
    {
        self.receive_diagnosed(act, issuer, version).map_err(ReceptionFailure::into_legacy)
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
        Ok(ReceivedGenesis { continuity, act: self.act.clone(),
            issuer: self.issuer.clone(), version: self.version.clone() })
    }
}

impl ReceivedGenesis {
    pub fn continuity(&self) -> &AuthorityContinuity { &self.continuity }
    pub fn act(&self) -> &[u8] { &self.act }
    pub fn declared_issuer(&self) -> &str { &self.issuer }
    pub fn version(&self) -> &str { &self.version }
}
