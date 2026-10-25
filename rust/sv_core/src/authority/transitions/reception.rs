//! Recepción posterior a admisión externa, según RETP-154.
//!
//! ES: Conserva el acto recibido junto a la única continuidad trazada que nace
//! del plan fijado. No produce la premisa ni autentica al emisor declarado.
//! EN: Keeps the received act with the single traced continuity created from
//! the fixed plan. It neither produces the premise nor authenticates its issuer.
//! No persistencia, red ni interpretación de instrucciones del contenido.

#![forbid(unsafe_code)]

use super::{AuthorityContinuity, ExternalGenesisPremise, GenesisError, GenesisPlan};
use crate::{
    decide_permit_traced, execute_traced_mediated, mediate_traced_permit, EffectDescriptor,
    EffectExecutor, FormRef, ProtectedDecisionContinuity, ResolvedRequirementResult,
    TracedDecisionError, TracedExecutionError, TracedExerciseConfirmation,
    TracedMediatedCommitment, TracedMediationError, TracedPermit, TracedPermitDecision,
};
use std::fmt;

/// Cota por campo en esta recepción; no es una cuota del proceso ni del plan.
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

struct Preparation {
    premise: ExternalGenesisPremise,
    plan: GenesisPlan,
    evidence: ReceivedAct,
}

struct ReceivedAct {
    act: Box<[u8]>,
    issuer: Box<str>,
    version: Box<str>,
}

pub struct PreparedGenesisReception {
    preparation: Option<Preparation>,
}

pub struct ReceivedGenesis {
    continuity: ProtectedDecisionContinuity,
    evidence: ReceivedAct,
}

// ES: Debug no es un canal de exportación documental o de identidad.
// EN: Debug is not a document or identity export channel.
impl fmt::Debug for PreparedGenesisReception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreparedGenesisReception")
            .field("attempted", &self.preparation.is_none())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for ReceivedGenesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceivedGenesis")
            .field("decision_count", &self.continuity.decision_trace_count())
            .field("event_count", &self.continuity.exercise_event_count())
            .finish_non_exhaustive()
    }
}

fn bounds(parts: &[&[u8]]) -> Result<(), ReceptionError> {
    if parts.iter().any(|v| v.is_empty()) {
        return Err(ReceptionError::Empty);
    }
    if parts.iter().any(|v| v.len() > MAX_RECEPTION_BYTES) {
        return Err(ReceptionError::TooLarge);
    }
    Ok(())
}

impl PreparedGenesisReception {
    /// ES: Instalación interna; el plan no procede de la propuesta no fiable.
    /// El productor material de la premisa sigue siendo una dependencia.
    /// EN: Internal installation; the plan does not come from the untrusted
    /// proposal. The material producer of the premise remains a dependency.
    #[allow(dead_code)] // Integración institucional pendiente; sin constructor externo alternativo.
    pub(crate) fn prepare(
        premise: ExternalGenesisPremise,
        plan: GenesisPlan,
        act: &[u8],
        issuer: &str,
        version: &str,
    ) -> Result<Self, ReceptionError> {
        bounds(&[act, issuer.as_bytes(), version.as_bytes()])?;
        Ok(Self {
            preparation: Some(Preparation {
                premise,
                plan,
                evidence: ReceivedAct {
                    act: act.into(),
                    issuer: issuer.into(),
                    version: version.into(),
                },
            }),
        })
    }

    /// ES: Rechazar una sustitución conserva el preparado. Entrar en T-0 lo
    /// consume incluso si el plan resulta inválido; nunca se repara el acto.
    /// EN: Rejecting substitution preserves preparation. Entering T-0 consumes
    /// it even on an invalid plan; the received act is never repaired.
    pub fn receive(
        &mut self,
        act: &[u8],
        issuer: &str,
        version: &str,
    ) -> Result<ReceivedGenesis, ReceptionError> {
        let prepared = self
            .preparation
            .as_ref()
            .ok_or(ReceptionError::AlreadyAttempted)?;
        bounds(&[act, issuer.as_bytes(), version.as_bytes()])?;
        if issuer != &*prepared.evidence.issuer || version != &*prepared.evidence.version {
            return Err(ReceptionError::DeclarationMismatch);
        }
        if act != &*prepared.evidence.act {
            return Err(ReceptionError::ActMismatch);
        }
        let mut prepared = self
            .preparation
            .take()
            .ok_or(ReceptionError::AlreadyAttempted)?;
        let mut authority = AuthorityContinuity::uninhabited();
        authority
            .apply_genesis(&mut prepared.premise, prepared.plan)
            .map_err(ReceptionError::Genesis)?;
        Ok(ReceivedGenesis {
            continuity: ProtectedDecisionContinuity::from_authority(authority),
            evidence: prepared.evidence,
        })
    }
}

impl ReceivedGenesis {
    pub fn continuity(&self) -> &AuthorityContinuity {
        self.continuity.authority()
    }
    pub fn act(&self) -> &[u8] {
        &self.evidence.act
    }
    pub fn declared_issuer(&self) -> &str {
        &self.evidence.issuer
    }
    pub fn version(&self) -> &str {
        &self.evidence.version
    }
    pub fn trace(&self) -> &ProtectedDecisionContinuity {
        &self.continuity
    }

    /// ES: Delega en la misma regla nuclear; no permite sustituir la continuidad
    /// ni extraerla separada del acto. EN: Delegates to the same core rule and
    /// permits neither replacing the continuity nor detaching it from the act.
    pub fn decide(
        &mut self,
        form: &FormRef,
        effect: &EffectDescriptor,
        results: &[ResolvedRequirementResult],
    ) -> Result<TracedPermitDecision, TracedDecisionError> {
        decide_permit_traced(&mut self.continuity, form, effect, results)
    }

    pub fn mediate(
        &mut self,
        permit: TracedPermit,
        effect: &EffectDescriptor,
    ) -> Result<TracedMediatedCommitment, TracedMediationError> {
        mediate_traced_permit(&mut self.continuity, permit, effect)
    }

    pub fn execute<A: EffectExecutor>(
        &mut self,
        commitment: TracedMediatedCommitment,
        adapter: &mut A,
    ) -> Result<TracedExerciseConfirmation, TracedExecutionError<A::Error>> {
        execute_traced_mediated(&mut self.continuity, commitment, adapter)
    }
}

// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
