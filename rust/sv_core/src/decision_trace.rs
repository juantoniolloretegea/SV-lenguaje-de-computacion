//! Ligadura decisión–efecto y traza determinista de R1-5.
//!
//! Este módulo no vuelve a decidir autoridad, requisitos, permiso, mediación o
//! ejecución. Envuelve las fronteras ya cerradas de R1-3/R1-4 y conserva una
//! relación causal canónica dentro de una continuidad lógica intra-proceso.
//!
//! Las funciones productivas crudas de decisión, mediación y ejecución quedan
//! internas al crate. La superficie pública conforme de R1-5 atraviesa esta
//! envolvente para que una decisión gobernada no pueda omitir su traza.

use std::collections::{BTreeMap, BTreeSet};

use crate::authority::transitions::AuthorityContinuity;
use crate::authority::EffectDescriptor;
use crate::control::{
    ApplicabilityRuleRef, AuthorityRef, CheckResult, ConflictResolutionRuleRef, ContextRef,
    CoverageRuleRef, EffectFamilyRef, ExerciseRef, FormRef, RequirementRef, ReuseBindingKeyRef,
    ReuseBindingValueRef, ReuseRuleRef, VerifierFamilyRef, VerifierRef,
};
use crate::execution::{
    execute_mediated, EffectExecutor, ExecutionContinuity, ExecutionError, ExecutionRequest,
    ExerciseAttemptState, ExerciseConfirmation, ExerciseTraceEntry,
};
use crate::mediation::{mediate_permit, MediatedEffectCommitment, MediationError};
use crate::permission::{
    decide_permit, Permit, PermitDecision, PermitDecisionError, PermitRejection,
};
use crate::requirements::{RequirementClass, RequirementDescriptor, RequirementSet};
use crate::requirements_bridge::ResolvedRequirementResult;

/// Referencia opaca de una decisión trazada dentro de una continuidad lógica.
///
/// La referencia se forma internamente mediante un ordinal estructural decimal.
/// No representa tiempo, vigencia, autoridad ni identidad durable entre procesos.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecisionTraceRef(String);

impl DecisionTraceRef {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictRuleTrace {
    reference: ConflictResolutionRuleRef,
    decisive_verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    applicability_rule: ApplicabilityRuleRef,
}

impl ConflictRuleTrace {
    #[inline]
    pub fn reference(&self) -> &ConflictResolutionRuleRef {
        &self.reference
    }

    #[inline]
    pub fn decisive_verifier(&self) -> &VerifierRef {
        &self.decisive_verifier
    }

    #[inline]
    pub fn verifier_family(&self) -> &VerifierFamilyRef {
        &self.verifier_family
    }

    #[inline]
    pub fn applicability_rule(&self) -> &ApplicabilityRuleRef {
        &self.applicability_rule
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageRuleTrace {
    reference: CoverageRuleRef,
    required_verifiers: BTreeSet<VerifierRef>,
}

impl CoverageRuleTrace {
    #[inline]
    pub fn reference(&self) -> &CoverageRuleRef {
        &self.reference
    }

    #[inline]
    pub fn required_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.required_verifiers.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReuseRuleTrace {
    reference: ReuseRuleRef,
    exact_bindings: BTreeMap<ReuseBindingKeyRef, ReuseBindingValueRef>,
}

impl ReuseRuleTrace {
    #[inline]
    pub fn reference(&self) -> &ReuseRuleRef {
        &self.reference
    }

    #[inline]
    pub fn bindings(
        &self,
    ) -> impl Iterator<Item = (&ReuseBindingKeyRef, &ReuseBindingValueRef)> {
        self.exact_bindings.iter()
    }
}

/// Instantánea canónica de una obligación y su resultado ya resuelto.
///
/// Conserva la ligadura material del descriptor vigente al producir el
/// resultado, las reglas constituidas y el conjunto de verificadores que
/// participaron. No es una `RequirementCheck` ni una capacidad de decisión.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementDecisionTrace {
    requirement: RequirementRef,
    class: RequirementClass,
    form: FormRef,
    effect_family: EffectFamilyRef,
    context: ContextRef,
    admissible_verifier_families: BTreeSet<VerifierFamilyRef>,
    applicability_rule: ApplicabilityRuleRef,
    conflict_rule: Option<ConflictRuleTrace>,
    coverage_rule: Option<CoverageRuleTrace>,
    reuse_rule: Option<ReuseRuleTrace>,
    participating_verifiers: BTreeSet<VerifierRef>,
    result: CheckResult,
}

impl RequirementDecisionTrace {
    fn from_resolved(
        descriptor: &RequirementDescriptor,
        resolved: &ResolvedRequirementResult,
    ) -> Self {
        let conflict_rule = descriptor.conflict_resolution_rule().map(|rule| ConflictRuleTrace {
            reference: rule.reference().clone(),
            decisive_verifier: rule.decisive_verifier().clone(),
            verifier_family: rule.verifier_family().clone(),
            applicability_rule: rule.applicability_rule().clone(),
        });
        let coverage_rule = descriptor.coverage_rule().map(|rule| CoverageRuleTrace {
            reference: rule.reference().clone(),
            required_verifiers: rule.required_verifiers().cloned().collect(),
        });
        let reuse_rule = descriptor.reuse_rule().map(|rule| ReuseRuleTrace {
            reference: rule.reference().clone(),
            exact_bindings: rule
                .bindings()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        });

        Self {
            requirement: descriptor.reference().clone(),
            class: descriptor.class(),
            form: descriptor.form().clone(),
            effect_family: descriptor.effect_family().clone(),
            context: descriptor.context().clone(),
            admissible_verifier_families: descriptor
                .admissible_verifier_families()
                .cloned()
                .collect(),
            applicability_rule: descriptor.applicability_rule().clone(),
            conflict_rule,
            coverage_rule,
            reuse_rule,
            participating_verifiers: resolved.participating_verifiers().cloned().collect(),
            result: resolved.result(),
        }
    }

    #[inline]
    pub fn requirement(&self) -> &RequirementRef {
        &self.requirement
    }

    #[inline]
    pub const fn class(&self) -> RequirementClass {
        self.class
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        &self.form
    }

    #[inline]
    pub fn effect_family(&self) -> &EffectFamilyRef {
        &self.effect_family
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }

    #[inline]
    pub fn admissible_verifier_families(&self) -> impl Iterator<Item = &VerifierFamilyRef> {
        self.admissible_verifier_families.iter()
    }

    #[inline]
    pub fn applicability_rule(&self) -> &ApplicabilityRuleRef {
        &self.applicability_rule
    }

    #[inline]
    pub fn conflict_rule(&self) -> Option<&ConflictRuleTrace> {
        self.conflict_rule.as_ref()
    }

    #[inline]
    pub fn coverage_rule(&self) -> Option<&CoverageRuleTrace> {
        self.coverage_rule.as_ref()
    }

    #[inline]
    pub fn reuse_rule(&self) -> Option<&ReuseRuleTrace> {
        self.reuse_rule.as_ref()
    }

    #[inline]
    pub fn participating_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.participating_verifiers.iter()
    }

    #[inline]
    pub const fn result(&self) -> CheckResult {
        self.result
    }
}

/// Disposición de permiso conservada por la traza.
///
/// Un bloqueo no es un permiso negativo ni una autoridad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracedPermitDisposition {
    Granted,
    BlockedRefuted,
    BlockedNotVerifiable,
}

/// Traza causal sellada de una decisión protegida que alcanzó resultado
/// gobernado.
///
/// El objeto es clonable porque copiar evidencia local no crea autoridad ni
/// capacidad. No ofrece conversión a `Permit`, mediación, ejecución o `Tri`.
///
/// ```compile_fail
/// use sv_core::{DecisionTrace, Permit};
/// fn promote(trace: DecisionTrace) -> Permit { trace.into() }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionTrace {
    reference: DecisionTraceRef,
    form: FormRef,
    authority: AuthorityRef,
    effect: EffectDescriptor,
    context: ContextRef,
    requirements: BTreeMap<RequirementRef, RequirementDecisionTrace>,
    aggregate: CheckResult,
    permit_disposition: TracedPermitDisposition,
}

impl DecisionTrace {
    #[inline]
    pub fn reference(&self) -> &DecisionTraceRef {
        &self.reference
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        &self.form
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        &self.authority
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        &self.effect
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }

    #[inline]
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    #[inline]
    pub fn requirement(&self, reference: &RequirementRef) -> Option<&RequirementDecisionTrace> {
        self.requirements.get(reference)
    }

    #[inline]
    pub fn requirements(&self) -> impl Iterator<Item = &RequirementDecisionTrace> {
        self.requirements.values()
    }

    #[inline]
    pub const fn aggregate(&self) -> CheckResult {
        self.aggregate
    }

    #[inline]
    pub const fn permit_disposition(&self) -> TracedPermitDisposition {
        self.permit_disposition
    }
}

/// Fallo estructural al ensamblar una traza desde una decisión ya gobernada.
/// Ninguna variante es `D-N` ni `Tri.U`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceAssemblyError {
    UnknownForm(FormRef),
    FormWithoutAuthority(FormRef),
    UnknownAuthority(AuthorityRef),
    MissingRequirementSet {
        form: FormRef,
        effect_family: EffectFamilyRef,
        context: ContextRef,
    },
    UnexpectedRequirementResult(RequirementRef),
    DuplicateRequirementResult(RequirementRef),
    RequirementBindingMismatch(RequirementRef),
    MissingRequirementResult(RequirementRef),
    DecisionBindingMismatch,
    DuplicateDecisionTraceRef(DecisionTraceRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TracedDecisionError {
    Decision(PermitDecisionError),
    Trace(TraceAssemblyError),
}

impl From<PermitDecisionError> for TracedDecisionError {
    fn from(error: PermitDecisionError) -> Self {
        Self::Decision(error)
    }
}

impl From<TraceAssemblyError> for TracedDecisionError {
    fn from(error: TraceAssemblyError) -> Self {
        Self::Trace(error)
    }
}

/// Permiso gobernado unido de forma lineal a la referencia de su traza.
///
/// No implementa `Clone` ni expone el `Permit` interno por valor.
#[derive(Debug, PartialEq, Eq)]
pub struct TracedPermit {
    permit: Permit,
    decision_trace: DecisionTraceRef,
}

impl TracedPermit {
    #[inline]
    pub fn decision_trace(&self) -> &DecisionTraceRef {
        &self.decision_trace
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        self.permit.authority()
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        self.permit.form()
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        self.permit.effect()
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        self.permit.context()
    }

    #[inline]
    pub const fn technical_result(&self) -> CheckResult {
        self.permit.technical_result()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TracedBlockedDecision {
    rejection: PermitRejection,
    decision_trace: DecisionTraceRef,
}

impl TracedBlockedDecision {
    #[inline]
    pub const fn rejection(&self) -> PermitRejection {
        self.rejection
    }

    #[inline]
    pub fn decision_trace(&self) -> &DecisionTraceRef {
        &self.decision_trace
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TracedPermitDecision {
    Granted(TracedPermit),
    NotGranted(TracedBlockedDecision),
}

impl TracedPermitDecision {
    #[inline]
    pub fn decision_trace(&self) -> &DecisionTraceRef {
        match self {
            Self::Granted(permit) => permit.decision_trace(),
            Self::NotGranted(blocked) => blocked.decision_trace(),
        }
    }
}

/// Continuidad pública de R1-5 para decisión, mediación, ejecución y traza.
///
/// Consume `AuthorityContinuity` y conserva internamente la traza de ejercicios
/// ya materializada por R1-4. No expone acceso mutable a la continuidad
/// autoritativa ni a los registros fuente.
#[derive(Debug, PartialEq, Eq)]
pub struct ProtectedDecisionContinuity {
    execution: ExecutionContinuity,
    next_decision_ordinal: String,
    decisions: BTreeMap<DecisionTraceRef, DecisionTrace>,
    mediated: BTreeSet<DecisionTraceRef>,
    exercise_links: BTreeMap<String, DecisionTraceRef>,
}

impl ProtectedDecisionContinuity {
    pub fn from_authority(authority: AuthorityContinuity) -> Self {
        Self {
            execution: ExecutionContinuity::from_authority(authority),
            next_decision_ordinal: "1".to_owned(),
            decisions: BTreeMap::new(),
            mediated: BTreeSet::new(),
            exercise_links: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityContinuity {
        self.execution.authority()
    }

    #[inline]
    pub fn decision_trace_count(&self) -> usize {
        self.decisions.len()
    }

    #[inline]
    pub fn decision_trace(&self, reference: &DecisionTraceRef) -> Option<&DecisionTrace> {
        self.decisions.get(reference)
    }

    #[inline]
    pub fn decision_traces(&self) -> impl Iterator<Item = &DecisionTrace> {
        self.decisions.values()
    }

    #[inline]
    pub fn is_mediated(&self, reference: &DecisionTraceRef) -> bool {
        self.mediated.contains(reference)
    }

    #[inline]
    pub fn exercise_event_count(&self) -> usize {
        self.execution.exercise_event_count()
    }

    #[inline]
    pub fn exercise_events(&self) -> impl Iterator<Item = &ExerciseTraceEntry> {
        self.execution.exercise_events()
    }

    #[inline]
    pub fn exercise_state(&self, exercise: &ExerciseRef) -> Option<ExerciseAttemptState> {
        self.execution.exercise_state(exercise)
    }

    pub fn exercise_decision_ref(&self, exercise: &ExerciseRef) -> Option<&DecisionTraceRef> {
        self.exercise_links.get(exercise.id().as_str())
    }

    pub fn exercise_decision_trace(&self, exercise: &ExerciseRef) -> Option<&DecisionTrace> {
        self.exercise_decision_ref(exercise)
            .and_then(|reference| self.decision_trace(reference))
    }

    fn candidate_decision_ref(&self) -> DecisionTraceRef {
        DecisionTraceRef(format!("decision:{}", self.next_decision_ordinal))
    }

    fn append_decision(&mut self, trace: DecisionTrace) -> Result<(), TraceAssemblyError> {
        let reference = trace.reference().clone();
        if self.decisions.insert(reference.clone(), trace).is_some() {
            return Err(TraceAssemblyError::DuplicateDecisionTraceRef(reference));
        }
        self.next_decision_ordinal = decimal_successor(&self.next_decision_ordinal);
        Ok(())
    }

    fn next_exercise_key(&self) -> String {
        let mut ordinal = "0".to_owned();
        for event in self.execution.exercise_events() {
            if event.state() == ExerciseAttemptState::DispatchCommitted {
                ordinal = decimal_successor(&ordinal);
            }
        }
        format!("exercise:{}", decimal_successor(&ordinal))
    }

    fn dispatch_count(&self) -> usize {
        self.execution
            .exercise_events()
            .filter(|entry| entry.state() == ExerciseAttemptState::DispatchCommitted)
            .count()
    }
}

fn decimal_successor(value: &str) -> String {
    let mut bytes = value.as_bytes().to_vec();
    let mut carry = true;
    for digit in bytes.iter_mut().rev() {
        if !carry {
            break;
        }
        if *digit == b'9' {
            *digit = b'0';
        } else {
            *digit += 1;
            carry = false;
        }
    }
    if carry {
        bytes.insert(0, b'1');
    }
    String::from_utf8(bytes).expect("el sucesor decimal interno conserva ASCII")
}

fn decision_result(decision: &PermitDecision) -> (CheckResult, TracedPermitDisposition) {
    match decision {
        PermitDecision::Granted(permit) => (
            permit.technical_result(),
            TracedPermitDisposition::Granted,
        ),
        PermitDecision::NotGranted(PermitRejection::RefutedRequirements) => (
            CheckResult::Refuted,
            TracedPermitDisposition::BlockedRefuted,
        ),
        PermitDecision::NotGranted(PermitRejection::NotVerifiableRequirements) => (
            CheckResult::NotVerifiable,
            TracedPermitDisposition::BlockedNotVerifiable,
        ),
    }
}

fn assemble_trace(
    reference: DecisionTraceRef,
    continuity: &AuthorityContinuity,
    form_reference: &FormRef,
    effect: &EffectDescriptor,
    resolved_results: &[ResolvedRequirementResult],
    decision: &PermitDecision,
) -> Result<DecisionTrace, TraceAssemblyError> {
    let form = continuity
        .form(form_reference)
        .ok_or_else(|| TraceAssemblyError::UnknownForm(form_reference.clone()))?;
    let authority_reference = form
        .requires_authority()
        .ok_or_else(|| TraceAssemblyError::FormWithoutAuthority(form_reference.clone()))?;
    let authority = continuity
        .authority(authority_reference)
        .ok_or_else(|| TraceAssemblyError::UnknownAuthority(authority_reference.clone()))?;
    let requirements = continuity
        .requirement_set(form.reference(), effect.family(), effect.context())
        .ok_or_else(|| TraceAssemblyError::MissingRequirementSet {
            form: form.reference().clone(),
            effect_family: effect.family().clone(),
            context: effect.context().clone(),
        })?;

    let mut traced_requirements = BTreeMap::new();
    for resolved in resolved_results {
        let requirement = resolved.requirement().clone();
        let descriptor = requirements.requirement(&requirement).ok_or_else(|| {
            TraceAssemblyError::UnexpectedRequirementResult(requirement.clone())
        })?;
        if !resolved.matches_descriptor(descriptor) {
            return Err(TraceAssemblyError::RequirementBindingMismatch(requirement));
        }
        if traced_requirements
            .insert(
                requirement.clone(),
                RequirementDecisionTrace::from_resolved(descriptor, resolved),
            )
            .is_some()
        {
            return Err(TraceAssemblyError::DuplicateRequirementResult(requirement));
        }
    }
    for descriptor in requirements.iter() {
        if !traced_requirements.contains_key(descriptor.reference()) {
            return Err(TraceAssemblyError::MissingRequirementResult(
                descriptor.reference().clone(),
            ));
        }
    }

    let (aggregate, permit_disposition) = decision_result(decision);
    if let PermitDecision::Granted(permit) = decision {
        if permit.form() != form.reference()
            || permit.authority() != authority.reference()
            || permit.effect() != effect
            || permit.context() != effect.context()
            || aggregate != CheckResult::Accredited
        {
            return Err(TraceAssemblyError::DecisionBindingMismatch);
        }
    }

    Ok(DecisionTrace {
        reference,
        form: form.reference().clone(),
        authority: authority.reference().clone(),
        effect: effect.clone(),
        context: effect.context().clone(),
        requirements: traced_requirements,
        aggregate,
        permit_disposition,
    })
}

/// Única entrada pública de decisión protegida de R1-5.
///
/// La operación reutiliza `decide_permit` internamente y sólo después de que la
/// decisión gobernada exista ensambla y registra su traza canónica. Una decisión
/// D-R o D-N queda trazada sin fabricar un `Permit` negativo.
pub fn decide_permit_traced(
    continuity: &mut ProtectedDecisionContinuity,
    form_reference: &FormRef,
    effect: &EffectDescriptor,
    resolved_results: &[ResolvedRequirementResult],
) -> Result<TracedPermitDecision, TracedDecisionError> {
    let decision = decide_permit(
        continuity.authority(),
        form_reference,
        effect,
        resolved_results,
    )?;
    let reference = continuity.candidate_decision_ref();
    let trace = assemble_trace(
        reference.clone(),
        continuity.authority(),
        form_reference,
        effect,
        resolved_results,
        &decision,
    )?;
    continuity.append_decision(trace)?;

    Ok(match decision {
        PermitDecision::Granted(permit) => TracedPermitDecision::Granted(TracedPermit {
            permit,
            decision_trace: reference,
        }),
        PermitDecision::NotGranted(rejection) => {
            TracedPermitDecision::NotGranted(TracedBlockedDecision {
                rejection,
                decision_trace: reference,
            })
        }
    })
}

fn trace_matches_permit(trace: &DecisionTrace, permit: &Permit) -> bool {
    trace.permit_disposition() == TracedPermitDisposition::Granted
        && trace.aggregate() == CheckResult::Accredited
        && trace.form() == permit.form()
        && trace.authority() == permit.authority()
        && trace.effect() == permit.effect()
        && trace.context() == permit.context()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TracedMediationError {
    UnknownDecisionTrace(DecisionTraceRef),
    DecisionNotGranted(DecisionTraceRef),
    DecisionBindingMismatch(DecisionTraceRef),
    Mediation(MediationError),
}

/// Compromiso mediado ligado a la decisión trazada que lo originó.
/// No implementa `Clone` ni expone el compromiso interno por valor.
#[derive(Debug, PartialEq, Eq)]
pub struct TracedMediatedCommitment {
    commitment: MediatedEffectCommitment,
    decision_trace: DecisionTraceRef,
}

impl TracedMediatedCommitment {
    #[inline]
    pub fn decision_trace(&self) -> &DecisionTraceRef {
        &self.decision_trace
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        self.commitment.authority()
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        self.commitment.form()
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        self.commitment.effect()
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        self.commitment.context()
    }
}

pub fn mediate_traced_permit(
    continuity: &mut ProtectedDecisionContinuity,
    traced_permit: TracedPermit,
    effect: &EffectDescriptor,
) -> Result<TracedMediatedCommitment, TracedMediationError> {
    let reference = traced_permit.decision_trace.clone();
    let trace = continuity
        .decision_trace(&reference)
        .ok_or_else(|| TracedMediationError::UnknownDecisionTrace(reference.clone()))?;
    if trace.permit_disposition() != TracedPermitDisposition::Granted {
        return Err(TracedMediationError::DecisionNotGranted(reference));
    }
    if !trace_matches_permit(trace, &traced_permit.permit) {
        return Err(TracedMediationError::DecisionBindingMismatch(reference));
    }

    let commitment = mediate_permit(
        continuity.authority(),
        traced_permit.permit,
        effect,
    )
    .map_err(TracedMediationError::Mediation)?;
    continuity.mediated.insert(reference.clone());

    Ok(TracedMediatedCommitment {
        commitment,
        decision_trace: reference,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TracedExerciseConfirmation {
    confirmation: ExerciseConfirmation,
    decision_trace: DecisionTraceRef,
}

impl TracedExerciseConfirmation {
    #[inline]
    pub fn decision_trace(&self) -> &DecisionTraceRef {
        &self.decision_trace
    }

    #[inline]
    pub fn exercise(&self) -> &ExerciseRef {
        self.confirmation.exercise()
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        self.confirmation.authority()
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        self.confirmation.form()
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        self.confirmation.effect()
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        self.confirmation.context()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TracedExecutionError<E> {
    UnknownDecisionTrace(DecisionTraceRef),
    DecisionNotMediated(DecisionTraceRef),
    DecisionBindingMismatch(DecisionTraceRef),
    ExerciseReservationConflict(String),
    Execution {
        decision_trace: DecisionTraceRef,
        error: ExecutionError<E>,
    },
}

fn trace_matches_commitment(
    trace: &DecisionTrace,
    commitment: &MediatedEffectCommitment,
) -> bool {
    trace.permit_disposition() == TracedPermitDisposition::Granted
        && trace.aggregate() == CheckResult::Accredited
        && trace.form() == commitment.form()
        && trace.authority() == commitment.authority()
        && trace.effect() == commitment.effect()
        && trace.context() == commitment.context()
}

/// Única entrada pública de ejecución protegida de R1-5.
///
/// Antes de delegar en R1-4 reserva de forma determinista el enlace causal del
/// próximo `ExerciseRef`. Si la ejecución falla antes del despacho la reserva se
/// elimina; si el adaptador interrumpe la llamada después de `DispatchCommitted`,
/// el enlace ya existe y no depende de una actualización posterior al efecto.
pub fn execute_traced_mediated<A: EffectExecutor>(
    continuity: &mut ProtectedDecisionContinuity,
    traced_commitment: TracedMediatedCommitment,
    adapter: &mut A,
) -> Result<TracedExerciseConfirmation, TracedExecutionError<A::Error>> {
    let reference = traced_commitment.decision_trace.clone();
    let trace = continuity
        .decision_trace(&reference)
        .ok_or_else(|| TracedExecutionError::UnknownDecisionTrace(reference.clone()))?;
    if !continuity.is_mediated(&reference) {
        return Err(TracedExecutionError::DecisionNotMediated(reference));
    }
    if !trace_matches_commitment(trace, &traced_commitment.commitment) {
        return Err(TracedExecutionError::DecisionBindingMismatch(reference));
    }

    let exercise_key = continuity.next_exercise_key();
    if continuity.exercise_links.contains_key(&exercise_key) {
        return Err(TracedExecutionError::ExerciseReservationConflict(exercise_key));
    }
    continuity
        .exercise_links
        .insert(exercise_key.clone(), reference.clone());
    let dispatches_before = continuity.dispatch_count();

    match execute_mediated(
        &mut continuity.execution,
        traced_commitment.commitment,
        adapter,
    ) {
        Ok(confirmation) => {
            let actual = confirmation.exercise().id().as_str();
            if actual != exercise_key {
                return Err(TracedExecutionError::ExerciseReservationConflict(
                    actual.to_owned(),
                ));
            }
            Ok(TracedExerciseConfirmation {
                confirmation,
                decision_trace: reference,
            })
        }
        Err(error) => {
            if continuity.dispatch_count() == dispatches_before {
                continuity.exercise_links.remove(&exercise_key);
            }
            Err(TracedExecutionError::Execution {
                decision_trace: reference,
                error,
            })
        }
    }
}

/// La solicitud cruda de R1-4 no se expone como vía pública de ejecución de
/// R1-5. El tipo sigue siendo visible para implementar el puerto, pero sólo la
/// envolvente trazada puede producir una instancia real durante el despacho.
pub trait TracedEffectExecutor: EffectExecutor {}

impl<T: EffectExecutor> TracedEffectExecutor for T {}

/// Firma de solo lectura útil para adaptadores que quieran expresar
/// explícitamente que reciben una solicitud gobernada ya sellada.
pub type TracedExecutionRequest<'a> = ExecutionRequest<'a>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_successor_is_not_machine_word_bounded() {
        assert_eq!(decimal_successor("9"), "10");
        assert_eq!(
            decimal_successor("999999999999999999999999999999999999"),
            "1000000000000000000000000000000000000"
        );
    }

    #[test]
    fn empty_execution_continuity_predicts_first_exercise_without_a_clock() {
        let continuity = ProtectedDecisionContinuity::from_authority(
            AuthorityContinuity::uninhabited(),
        );
        assert_eq!(continuity.next_exercise_key(), "exercise:1");
    }

    #[test]
    fn decision_references_are_structural_ordinals() {
        let mut continuity = ProtectedDecisionContinuity::from_authority(
            AuthorityContinuity::uninhabited(),
        );
        assert_eq!(continuity.candidate_decision_ref().as_str(), "decision:1");
        continuity.next_decision_ordinal = decimal_successor(&continuity.next_decision_ordinal);
        assert_eq!(continuity.candidate_decision_ref().as_str(), "decision:2");
    }
}
