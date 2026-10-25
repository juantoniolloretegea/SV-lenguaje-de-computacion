//! ES: Admisión de un mandato institucional exacto, configurado fuera de las
//! peticiones. La firma prueba procedencia respecto de la clave reconocida;
//! no acredita por sí sola legitimidad jurídica. Sin red, disco o inferencia.
//! EN: Admission of one exact institutional mandate, configured outside request
//! input. Signatures prove origin relative to the recognized key, not legal
//! legitimacy. No network, disk or inference. Persistence remains an R2 duty.
#![forbid(unsafe_code)]

pub(crate) mod plan;
#[cfg(test)]
mod tests;

use super::reception::{PreparedGenesisReception, ReceivedGenesis, MAX_RECEPTION_BYTES};
use super::ExternalGenesisPremise;
use crate::{
    CheckResult, EffectExecutor, ExecutionRequest, ProtectedDecisionContinuity,
    TracedExerciseConfirmation, TracedPermitDecision,
};
use ed25519_dalek::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fmt,
    sync::{Mutex, OnceLock},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub const MAX_AUDIT_ENTRIES: usize = 32;
const MAX_ADMISSIONS_PER_PROCESS: usize = 1024;
const MAX_LABEL_BYTES: usize = 256;
static ADMITTED: OnceLock<Mutex<BTreeSet<[u8; 32]>>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissionError {
    InvalidField,
    TooLarge,
    InvalidKey,
    SelfAccreditation,
    InvalidSignature,
    NotYetValid,
    Expired,
    ClockUnavailable,
    ClockRollback,
    AlreadyAdmitted,
    RegistryFull,
    RegistryUnavailable,
    AuditFull,
    Revoked,
    AlreadyDispatched,
    ScopeMismatch,
    CoreInvariant,
    CoreRejected,
    CoreExecutionRejected,
    AdapterIndeterminate,
}

/// ES: Datos de configuración privilegiada; nunca se obtienen de una petición.
/// EN: Privileged configuration data; never sourced from ordinary requests.
pub struct MandateDefinition<'a> {
    pub institution: &'a str,
    pub issuer: &'a str,
    pub domain: &'a str,
    pub context: &'a str,
    pub operation: &'a str,
    pub object: &'a str,
    pub edition: &'a str,
    pub mandate_id: &'a str,
    pub proposer_key: [u8; 32],
    pub valid_from: u64,
    pub valid_until: u64,
    pub equality_contract: &'a [u8],
    pub expected: &'a [u8],
}

/// Un mandato es dato propuesto hasta comprobar su procedencia y vigencia.
/// A mandate is proposal data until its origin and validity have been checked.
#[derive(Clone)]
pub struct InstitutionalMandate {
    labels: [Box<str>; 8],
    proposer: VerifyingKey,
    valid_from: u64,
    valid_until: u64,
    contract: Box<[u8]>,
    expected: Box<[u8]>,
}

fn label(value: &str) -> Result<(), AdmissionError> {
    if value.len() > MAX_LABEL_BYTES {
        return Err(AdmissionError::TooLarge);
    }
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        return Err(AdmissionError::InvalidField);
    }
    Ok(())
}

fn key(bytes: &[u8; 32]) -> Result<VerifyingKey, AdmissionError> {
    let k = VerifyingKey::from_bytes(bytes).map_err(|_| AdmissionError::InvalidKey)?;
    if k.is_weak() || k.to_edwards().compress().to_bytes() != *bytes {
        return Err(AdmissionError::InvalidKey);
    }
    Ok(k)
}

fn field(out: &mut Vec<u8>, bytes: &[u8]) {
    // ES: Longitud explícita; no concatenación ambigua ni normalización.
    // EN: Explicit lengths prevent ambiguous concatenation; no normalization.
    out.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
    out.extend_from_slice(bytes);
}

impl InstitutionalMandate {
    pub fn new(d: MandateDefinition<'_>) -> Result<Self, AdmissionError> {
        let labels = [
            d.institution,
            d.issuer,
            d.domain,
            d.context,
            d.operation,
            d.object,
            d.edition,
            d.mandate_id,
        ];
        for v in labels {
            label(v)?;
        }
        if d.valid_from >= d.valid_until || d.equality_contract.is_empty() {
            return Err(AdmissionError::InvalidField);
        }
        let size = labels
            .iter()
            .map(|v| v.len() + 8)
            .sum::<usize>()
            .checked_add(d.equality_contract.len())
            .and_then(|n| n.checked_add(d.expected.len()))
            .and_then(|n| n.checked_add(128))
            .ok_or(AdmissionError::TooLarge)?;
        if size > MAX_RECEPTION_BYTES {
            return Err(AdmissionError::TooLarge);
        }
        Ok(Self {
            labels: labels.map(Into::into),
            proposer: key(&d.proposer_key)?,
            valid_from: d.valid_from,
            valid_until: d.valid_until,
            contract: d.equality_contract.into(),
            expected: d.expected.into(),
        })
    }

    /// ES: Formato binario v1, exacto y separado de petición y revocación.
    /// EN: Exact v1 binary format, domain-separated from request and revocation.
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut out = b"SV-INSTITUTIONAL-MANDATE-1\0".to_vec();
        for v in &self.labels {
            field(&mut out, v.as_bytes());
        }
        out.extend_from_slice(self.proposer.as_bytes());
        out.extend_from_slice(&self.valid_from.to_be_bytes());
        out.extend_from_slice(&self.valid_until.to_be_bytes());
        field(&mut out, &self.contract);
        field(&mut out, &self.expected);
        out
    }
    pub fn fingerprint(&self) -> [u8; 32] {
        Sha256::digest(self.signing_bytes()).into()
    }
    pub fn revocation_bytes(&self) -> Vec<u8> {
        let mut out = b"SV-INSTITUTIONAL-REVOCATION-1\0".to_vec();
        out.extend_from_slice(&self.fingerprint());
        out
    }
}

impl fmt::Debug for InstitutionalMandate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstitutionalMandate")
            .finish_non_exhaustive()
    }
}

/// ES: La instalación reconoce una clave y UN mandato exacto. Esta API es
/// privilegiada: su acceso y persistencia corresponden al soporte institucional.
/// EN: Installation recognizes a key and ONE exact mandate. This is a privileged
/// API: access control and persistence belong to the institutional host.
pub struct InstitutionalInstallation {
    mandate: Option<InstitutionalMandate>,
    issuer: VerifyingKey,
    audit: Vec<AuditEntry>,
    input_rejections: InputRejections,
}

impl InstitutionalInstallation {
    pub fn configure(
        mandate: InstitutionalMandate,
        recognized_key: [u8; 32],
    ) -> Result<Self, AdmissionError> {
        let issuer = key(&recognized_key)?;
        if issuer == mandate.proposer {
            return Err(AdmissionError::SelfAccreditation);
        }
        Ok(Self {
            mandate: Some(mandate),
            issuer,
            audit: Vec::new(),
            input_rejections: InputRejections::default(),
        })
    }
    pub fn audit(&self) -> &[AuditEntry] {
        &self.audit
    }
    pub fn input_rejections(&self) -> &InputRejections {
        &self.input_rejections
    }
    pub fn admit(
        &mut self,
        signature: [u8; 64],
    ) -> Result<AdmittedInstitutionalOperation, AdmissionError> {
        self.admit_at(signature, wall_time(), Instant::now())
    }
    fn admit_at(
        &mut self,
        signature: [u8; 64],
        now: Result<u64, AdmissionError>,
        tick: Instant,
    ) -> Result<AdmittedInstitutionalOperation, AdmissionError> {
        if self.audit.len() >= MAX_AUDIT_ENTRIES {
            return Err(AdmissionError::AuditFull);
        }
        let result = self.admit_inner(signature, now, tick);
        if matches!(&result, Err(AdmissionError::InvalidSignature)) {
            self.input_rejections
                .record(AdmissionError::InvalidSignature, now.ok());
            return result;
        }
        let outcome = match &result {
            Ok(_) => AuditOutcome::Admitted,
            Err(e) => AuditOutcome::Rejected(*e),
        };
        self.audit.push(AuditEntry {
            at: now.ok(),
            outcome,
            request: None,
            signature,
        });
        result.map(|mut value| {
            value.audit = std::mem::take(&mut self.audit);
            value.input_rejections = self.input_rejections;
            value
        })
    }
    fn admit_inner(
        &mut self,
        signature: [u8; 64],
        now: Result<u64, AdmissionError>,
        tick: Instant,
    ) -> Result<AdmittedInstitutionalOperation, AdmissionError> {
        let m = self
            .mandate
            .as_ref()
            .ok_or(AdmissionError::AlreadyAdmitted)?;
        let bytes = m.signing_bytes();
        self.issuer
            .verify_strict(&bytes, &Signature::from_bytes(&signature))
            .map_err(|_| AdmissionError::InvalidSignature)?;
        let now = now?;
        validity(m, now)?;
        let deadline = tick
            .checked_add(Duration::from_secs(m.valid_until - now))
            .ok_or(AdmissionError::InvalidField)?;
        let mut identity = bytes.clone();
        identity.extend_from_slice(self.issuer.as_bytes());
        let identity: [u8; 32] = Sha256::digest(identity).into();
        let mut admitted = ADMITTED
            .get_or_init(|| Mutex::new(BTreeSet::new()))
            .lock()
            .map_err(|_| AdmissionError::RegistryUnavailable)?;
        if admitted.contains(&identity) {
            return Err(AdmissionError::AlreadyAdmitted);
        }
        if admitted.len() >= MAX_ADMISSIONS_PER_PROCESS {
            return Err(AdmissionError::RegistryFull);
        }
        // ES: La reserva es terminal incluso ante rechazo posterior. No se elimina
        // para simular recuperación. EN: Terminal reservation, including failure.
        admitted.insert(identity);
        drop(admitted);
        let m = self.mandate.take().ok_or(AdmissionError::AlreadyAdmitted)?;
        let plan = plan::make_plan(&m)?;
        // ES: Único productor material: firma, reconocimiento y alcance ya cotejados.
        // EN: Sole material producer after signature, recognition and scope checks.
        let premise = ExternalGenesisPremise { consumed: false };
        let mut prepared =
            PreparedGenesisReception::prepare(premise, plan, &bytes, &m.labels[1], &m.labels[6])
                .map_err(|_| AdmissionError::CoreInvariant)?;
        let received = prepared
            .receive(&bytes, &m.labels[1], &m.labels[6])
            .map_err(|_| AdmissionError::CoreInvariant)?;
        Ok(AdmittedInstitutionalOperation {
            mandate: m,
            issuer: self.issuer,
            received,
            admitted_at: now,
            last_seen: now,
            deadline,
            revoked: None,
            dispatched: false,
            audit: Vec::new(),
            input_rejections: InputRejections::default(),
        })
    }
}

fn wall_time() -> Result<u64, AdmissionError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_secs())
        .map_err(|_| AdmissionError::ClockUnavailable)
}
fn validity(m: &InstitutionalMandate, now: u64) -> Result<(), AdmissionError> {
    if now < m.valid_from {
        return Err(AdmissionError::NotYetValid);
    }
    if now >= m.valid_until {
        return Err(AdmissionError::Expired);
    }
    Ok(())
}

/// Petición ordinaria: datos sin capacidad de modificar el mandato.
/// Ordinary request: data with no ability to modify the mandate.
pub struct InstitutionalInvocation<'a> {
    pub domain: &'a str,
    pub context: &'a str,
    pub operation: &'a str,
    pub object: &'a str,
    pub nonce: [u8; 16],
    pub observed: Option<&'a [u8]>,
}
impl InstitutionalInvocation<'_> {
    pub fn signing_bytes(&self, mandate: &InstitutionalMandate) -> Result<Vec<u8>, AdmissionError> {
        for v in [self.domain, self.context, self.operation, self.object] {
            label(v)?;
        }
        if self.observed.is_some_and(|v| v.len() > MAX_RECEPTION_BYTES) {
            return Err(AdmissionError::TooLarge);
        }
        let mut out = b"SV-INSTITUTIONAL-INVOCATION-1\0".to_vec();
        out.extend_from_slice(&mandate.fingerprint());
        for v in [self.domain, self.context, self.operation, self.object] {
            field(&mut out, v.as_bytes());
        }
        out.extend_from_slice(&self.nonce);
        out.push(u8::from(self.observed.is_some()));
        if let Some(bytes) = self.observed {
            field(&mut out, bytes);
        }
        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditOutcome {
    Admitted,
    Rejected(AdmissionError),
    Blocked(CheckResult),
    DispatchCommitted,
    Confirmed,
    Indeterminate,
}
pub struct AuditEntry {
    at: Option<u64>,
    outcome: AuditOutcome,
    request: Option<Box<[u8]>>,
    signature: [u8; 64],
}
impl AuditEntry {
    pub fn at(&self) -> Option<u64> {
        self.at
    }
    pub fn outcome(&self) -> AuditOutcome {
        self.outcome
    }
    /// Consulta explícita para custodia autorizada; no se publica mediante Debug.
    /// Explicit authorized evidence retrieval, never exposed through Debug.
    pub fn request_bytes(&self) -> Option<&[u8]> {
        self.request.as_deref()
    }
    pub fn signature(&self) -> &[u8; 64] {
        &self.signature
    }
}
impl fmt::Debug for AuditEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuditEntry")
            .field("outcome", &self.outcome)
            .finish_non_exhaustive()
    }
}

/// ES: Estado admitido sin constructor ni acceso mutable a la continuidad.
/// EN: Admitted state without a constructor or mutable continuity access.
///
/// ```compile_fail
/// use sv_core::authority::transitions::institutional::AdmittedInstitutionalOperation;
/// let _ = AdmittedInstitutionalOperation::new();
/// ```
/// ```compile_fail
/// use sv_core::authority::transitions::institutional::ValidatedInvocation;
/// ```
pub struct AdmittedInstitutionalOperation {
    mandate: InstitutionalMandate,
    issuer: VerifyingKey,
    received: ReceivedGenesis,
    admitted_at: u64,
    last_seen: u64,
    deadline: Instant,
    revoked: Option<([u8; 64], Option<u64>)>,
    dispatched: bool,
    audit: Vec<AuditEntry>,
    input_rejections: InputRejections,
}

/// ES: Resumen acotado de entradas no autenticadas o mal formadas. No conserva
/// contenido ajeno ni consume la capacidad reservada a actuaciones autenticadas.
/// EN: Bounded summary of unauthenticated or malformed input, without payloads
/// or consumption of the capacity reserved for authenticated activity.
#[derive(Debug, Default, Clone, Copy)]
pub struct InputRejections {
    count: u64,
    last_at: Option<u64>,
    last_reason: Option<AdmissionError>,
}
impl InputRejections {
    pub fn count(&self) -> u64 {
        self.count
    }
    pub fn last_at(&self) -> Option<u64> {
        self.last_at
    }
    pub fn last_reason(&self) -> Option<AdmissionError> {
        self.last_reason
    }
    fn record(&mut self, reason: AdmissionError, at: Option<u64>) {
        self.count = self.count.saturating_add(1);
        self.last_at = at;
        self.last_reason = Some(reason);
    }
}
impl fmt::Debug for AdmittedInstitutionalOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedInstitutionalOperation")
            .field("revoked", &self.revoked.is_some())
            .field("dispatched", &self.dispatched)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterFailure {
    Unavailable,
    Rejected,
    Indeterminate,
}
/// ES: El adaptador recibe exactamente la evidencia comprobada y el efecto
/// mediado. Su confinamiento material requiere R3. EN: Receives the exact
/// checked evidence and mediated effect; material confinement still needs R3.
pub trait InstitutionalExecutor {
    fn execute(
        &mut self,
        request: &ExecutionRequest<'_>,
        evidence: &[u8],
    ) -> Result<(), AdapterFailure>;
}
struct BoundExecutor<'a, A> {
    adapter: &'a mut A,
    evidence: &'a [u8],
}
impl<A: InstitutionalExecutor> EffectExecutor for BoundExecutor<'_, A> {
    type Error = AdapterFailure;
    fn execute(&mut self, request: &ExecutionRequest<'_>) -> Result<(), Self::Error> {
        self.adapter.execute(request, self.evidence)
    }
}

pub enum InstitutionalOutcome {
    Confirmed(Box<TracedExerciseConfirmation>),
    Blocked(CheckResult),
}
impl fmt::Debug for InstitutionalOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmed(_) => f.write_str("Confirmed"),
            Self::Blocked(r) => f.debug_tuple("Blocked").field(r).finish(),
        }
    }
}

impl AdmittedInstitutionalOperation {
    pub fn input_rejections(&self) -> &InputRejections {
        &self.input_rejections
    }
    pub fn mandate(&self) -> &InstitutionalMandate {
        &self.mandate
    }
    pub fn trace(&self) -> &ProtectedDecisionContinuity {
        self.received.trace()
    }
    pub fn audit(&self) -> &[AuditEntry] {
        &self.audit
    }
    pub fn revocation(&self) -> Option<&([u8; 64], Option<u64>)> {
        self.revoked.as_ref()
    }
    pub fn revoke(&mut self, signature: [u8; 64]) -> Result<(), AdmissionError> {
        self.issuer
            .verify_strict(
                &self.mandate.revocation_bytes(),
                &Signature::from_bytes(&signature),
            )
            .map_err(|_| AdmissionError::InvalidSignature)?;
        // ES: No depende del espacio del registro ordinario; no admite deshacer.
        // EN: Independent of ordinary audit capacity; cannot be undone.
        if self.revoked.is_none() {
            self.revoked = Some((signature, wall_time().ok()));
        }
        Ok(())
    }
    pub fn perform<A: InstitutionalExecutor>(
        &mut self,
        request: &InstitutionalInvocation<'_>,
        signature: [u8; 64],
        adapter: &mut A,
    ) -> Result<InstitutionalOutcome, AdmissionError> {
        self.perform_with_clock(request, signature, adapter, || {
            (wall_time(), Instant::now())
        })
    }
    #[cfg(test)]
    fn perform_at<A: InstitutionalExecutor>(
        &mut self,
        request: &InstitutionalInvocation<'_>,
        signature: [u8; 64],
        adapter: &mut A,
        now: Result<u64, AdmissionError>,
        tick: Instant,
    ) -> Result<InstitutionalOutcome, AdmissionError> {
        self.perform_with_clock(request, signature, adapter, || (now, tick))
    }
    fn perform_with_clock<
        A: InstitutionalExecutor,
        F: FnMut() -> (Result<u64, AdmissionError>, Instant),
    >(
        &mut self,
        request: &InstitutionalInvocation<'_>,
        signature: [u8; 64],
        adapter: &mut A,
        mut clock: F,
    ) -> Result<InstitutionalOutcome, AdmissionError> {
        let (now, tick) = clock();
        let bytes = match request.signing_bytes(&self.mandate).and_then(|bytes| {
            self.mandate
                .proposer
                .verify_strict(&bytes, &Signature::from_bytes(&signature))
                .map_err(|_| AdmissionError::InvalidSignature)?;
            Ok(bytes)
        }) {
            Ok(bytes) => bytes,
            Err(error) => {
                self.input_rejections.record(error, now.ok());
                return Err(error);
            }
        };
        if self.audit.len() >= MAX_AUDIT_ENTRIES {
            return Err(AdmissionError::AuditFull);
        }
        let index = self.audit.len();
        self.audit.push(AuditEntry {
            at: now.ok(),
            outcome: AuditOutcome::Rejected(AdmissionError::CoreInvariant),
            request: Some(bytes.into_boxed_slice()),
            signature,
        });
        let result = self.perform_inner(request, adapter, index, &mut clock, (now, tick));
        self.audit[index].outcome = match &result {
            Ok(InstitutionalOutcome::Confirmed(_)) => AuditOutcome::Confirmed,
            Ok(InstitutionalOutcome::Blocked(r)) => AuditOutcome::Blocked(*r),
            Err(AdmissionError::AdapterIndeterminate) => AuditOutcome::Indeterminate,
            Err(e) => AuditOutcome::Rejected(*e),
        };
        result
    }
    // ES: La segunda muestra impide despachar con una vigencia agotada al comprobar.
    // EN: The second sample prevents dispatch after validity expires during checks.
    fn perform_inner<
        A: InstitutionalExecutor,
        F: FnMut() -> (Result<u64, AdmissionError>, Instant),
    >(
        &mut self,
        r: &InstitutionalInvocation<'_>,
        adapter: &mut A,
        audit_index: usize,
        clock: &mut F,
        moment: (Result<u64, AdmissionError>, Instant),
    ) -> Result<InstitutionalOutcome, AdmissionError> {
        let (now, tick) = moment;
        let now = self.ensure_current(now, tick)?;
        if [r.domain, r.context, r.operation, r.object]
            != [
                &*self.mandate.labels[2],
                &*self.mandate.labels[3],
                &*self.mandate.labels[4],
                &*self.mandate.labels[5],
            ]
        {
            return Err(AdmissionError::ScopeMismatch);
        }
        let (form, effect) = plan::operation(self.received.continuity())?;
        let evidence = ValidatedInvocation {
            operation: self,
            observed: r.observed,
            now,
            tick,
        };
        let results = crate::requirements::institutional::measure(&evidence, &form, &effect)?;
        match self
            .received
            .decide(&form, &effect, &results)
            .map_err(|_| AdmissionError::CoreInvariant)?
        {
            TracedPermitDecision::NotGranted(blocked) => {
                Ok(InstitutionalOutcome::Blocked(match blocked.rejection() {
                    crate::PermitRejection::RefutedRequirements => CheckResult::Refuted,
                    crate::PermitRejection::NotVerifiableRequirements => CheckResult::NotVerifiable,
                }))
            }
            TracedPermitDecision::Granted(permit) => {
                let (now, tick) = clock();
                self.ensure_current(now, tick)?;
                let commitment = self
                    .received
                    .mediate(permit, &effect)
                    .map_err(|_| AdmissionError::CoreRejected)?;
                // ES: Terminal antes de invocar el adaptador, incluso ante error/pánico.
                // EN: Terminal before calling the adapter, including error/unwind.
                self.dispatched = true;
                self.audit[audit_index].outcome = AuditOutcome::DispatchCommitted;
                let evidence = r.observed.ok_or(AdmissionError::CoreInvariant)?;
                self.received
                    .execute(commitment, &mut BoundExecutor { adapter, evidence })
                    .map(|confirmation| InstitutionalOutcome::Confirmed(Box::new(confirmation)))
                    .map_err(|error| match error {
                        crate::TracedExecutionError::Execution {
                            error:
                                crate::ExecutionError::AdapterIndeterminate {
                                    error: crate::TracedAdapterError::Adapter(_),
                                    ..
                                },
                            ..
                        } => AdmissionError::AdapterIndeterminate,
                        _ => AdmissionError::CoreExecutionRejected,
                    })
            }
        }
    }
    fn ensure_current(
        &mut self,
        now: Result<u64, AdmissionError>,
        tick: Instant,
    ) -> Result<u64, AdmissionError> {
        if self.dispatched {
            return Err(AdmissionError::AlreadyDispatched);
        }
        let now = now?;
        if now < self.last_seen || now < self.admitted_at {
            return Err(AdmissionError::ClockRollback);
        }
        self.last_seen = now;
        if self.revoked.is_some() {
            return Err(AdmissionError::Revoked);
        }
        validity(&self.mandate, now)?;
        if tick >= self.deadline {
            return Err(AdmissionError::Expired);
        }
        Ok(now)
    }
}

// ES: Evidencia interna no construible por el solicitante ni serializable.
// EN: Internal evidence, neither requester-constructible nor serializable.
pub(crate) struct ValidatedInvocation<'a> {
    operation: &'a AdmittedInstitutionalOperation,
    observed: Option<&'a [u8]>,
    now: u64,
    tick: Instant,
}
impl ValidatedInvocation<'_> {
    pub(crate) fn continuity(&self) -> &super::AuthorityContinuity {
        self.operation.received.continuity()
    }
    pub(crate) fn contract(&self) -> &[u8] {
        &self.operation.mandate.contract
    }
    pub(crate) fn expected(&self) -> &[u8] {
        &self.operation.mandate.expected
    }
    pub(crate) fn observed(&self) -> Option<&[u8]> {
        self.observed
    }
    pub(crate) fn independent_keys(&self) -> bool {
        self.operation.issuer != self.operation.mandate.proposer
    }
    pub(crate) fn current(&self) -> bool {
        let o = self.operation;
        !o.dispatched
            && o.revoked.is_none()
            && self.now >= o.last_seen
            && self.now >= o.admitted_at
            && validity(&o.mandate, self.now).is_ok()
            && self.tick < o.deadline
    }
}

// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
