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
use crate::requirements::{RequirementClass, RequirementDescriptor};
use crate::requirements_bridge::{ResolvedCheckObservation, ResolvedRequirementResult};
use crate::requirements_reuse::{
    seal_historical_qualified_result, HistoricalQualificationError,
};

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

/// Resultado individual de una comprobación que participó en la resolución de
/// una obligación.
///
/// Es una instantánea de trazabilidad, no una `RequirementCheck` reutilizable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndividualCheckTrace {
    verifier: VerifierRef,
    verifier_family: VerifierFamilyRef,
    applicability_rule: ApplicabilityRuleRef,
    result: CheckResult,
}

impl IndividualCheckTrace {
    fn from_observation(observation: &ResolvedCheckObservation) -> Self {
        Self {
            verifier: observation.verifier().clone(),
            verifier_family: observation.verifier_family().clone(),
            applicability_rule: observation.applicability_rule().clone(),
            result: observation.result(),
        }
    }

    #[inline]
    pub fn verifier(&self) -> &VerifierRef {
        &self.verifier
    }

    #[inline]
    pub fn verifier_family(&self) -> &VerifierFamilyRef {
        &self.verifier_family
    }

    #[inline]
    pub fn applicability_rule(&self) -> &ApplicabilityRuleRef {
        &self.applicability_rule
    }

    #[inline]
    pub const fn result(&self) -> CheckResult {
        self.result
    }
}

/// Instantánea canónica de una obligación y de la cadena que produjo su
/// resultado gobernado.
///
/// Conserva las comprobaciones individuales, el resultado de resolución
/// 3A/3B/3C y el resultado ya cualificado por cobertura 3D. Una acreditación
/// resuelta con cobertura incompleta queda por tanto trazada explícitamente como
/// `D-N` en `qualified_result`.
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
    checks: BTreeMap<VerifierRef, IndividualCheckTrace>,
    resolved_result: CheckResult,
    qualified_result: CheckResult,
}

impl RequirementDecisionTrace {
    fn from_resolved(
        descriptor: &RequirementDescriptor,
        resolved: &ResolvedRequirementResult,
    ) -> Result<Self, HistoricalQualificationError> {
        let qualified = seal_historical_qualified_result(descriptor, resolved)?.result();
        let conflict_rule = descriptor
            .conflict_resolution_rule()
            .map(|rule| ConflictRuleTrace {
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
        let checks = resolved
            .check_observations()
            .map(|observation| {
                (
                    observation.verifier().clone(),
                    IndividualCheckTrace::from_observation(observation),
                )
            })
            .collect();

        Ok(Self {
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
            checks,
            resolved_result: resolved.result(),
            qualified_result: qualified,
        })
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
    pub fn check(&self, verifier: &VerifierRef) -> Option<&IndividualCheckTrace> {
        self.checks.get(verifier)
    }

    #[inline]
    pub fn checks(&self) -> impl Iterator<Item = &IndividualCheckTrace> {
        self.checks.values()
    }

    #[inline]
    pub fn participating_verifiers(&self) -> impl Iterator<Item = &VerifierRef> {
        self.checks.keys()
    }

    #[inline]
    pub const fn resolved_result(&self) -> CheckResult {
        self.resolved_result
    }

    #[inline]
    pub const fn qualified_result(&self) -> CheckResult {
        self.qualified_result
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
    HistoricalQualification(HistoricalQualificationError),
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
        let trace = RequirementDecisionTrace::from_resolved(descriptor, resolved)
            .map_err(TraceAssemblyError::HistoricalQualification)?;
        if traced_requirements
            .insert(requirement.clone(), trace)
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

    let commitment = mediate_permit(continuity.authority(), traced_permit.permit, effect)
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

/// Error del adaptador interno que enlaza el `ExerciseRef` real con su decisión
/// antes de delegar en el ejecutor material.
#[derive(Debug, PartialEq, Eq)]
pub enum TracedAdapterError<E> {
    TraceLinkConflict {
        exercise: ExerciseRef,
        existing: DecisionTraceRef,
        presented: DecisionTraceRef,
    },
    Adapter(E),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TracedExecutionError<E> {
    UnknownDecisionTrace(DecisionTraceRef),
    DecisionNotMediated(DecisionTraceRef),
    DecisionBindingMismatch(DecisionTraceRef),
    Execution {
        decision_trace: DecisionTraceRef,
        error: ExecutionError<TracedAdapterError<E>>,
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

struct LinkingExecutor<'a, A: EffectExecutor> {
    inner: &'a mut A,
    links: &'a mut BTreeMap<String, DecisionTraceRef>,
    decision_trace: DecisionTraceRef,
}

impl<A: EffectExecutor> EffectExecutor for LinkingExecutor<'_, A> {
    type Error = TracedAdapterError<A::Error>;

    fn execute(&mut self, request: &ExecutionRequest<'_>) -> Result<(), Self::Error> {
        let key = request.exercise().id().as_str().to_owned();
        if let Some(existing) = self.links.get(&key) {
            if existing != &self.decision_trace {
                return Err(TracedAdapterError::TraceLinkConflict {
                    exercise: request.exercise().clone(),
                    existing: existing.clone(),
                    presented: self.decision_trace.clone(),
                });
            }
        } else {
            self.links.insert(key, self.decision_trace.clone());
        }
        self.inner.execute(request).map_err(TracedAdapterError::Adapter)
    }
}

/// Única entrada pública de ejecución protegida de R1-5.
///
/// La operación no predice `ExerciseRef`. El enlace causal se registra usando
/// la referencia real que R1-4 ya ha inscrito como `DispatchCommitted`, dentro
/// del adaptador interno de enlace y antes de delegar al ejecutor material. Por
/// ello, si el ejecutor entra en pánico después de recibir la solicitud, el
/// vínculo `ExerciseRef → DecisionTraceRef` ya existe.
pub fn execute_traced_mediated<A: EffectExecutor>(
    continuity: &mut ProtectedDecisionContinuity,
    traced_commitment: TracedMediatedCommitment,
    adapter: &mut A,
) -> Result<TracedExerciseConfirmation, TracedExecutionError<A::Error>> {
    let reference = traced_commitment.decision_trace.clone();
    {
        let trace = continuity
            .decision_trace(&reference)
            .ok_or_else(|| TracedExecutionError::UnknownDecisionTrace(reference.clone()))?;
        if !continuity.is_mediated(&reference) {
            return Err(TracedExecutionError::DecisionNotMediated(reference));
        }
        if !trace_matches_commitment(trace, &traced_commitment.commitment) {
            return Err(TracedExecutionError::DecisionBindingMismatch(reference));
        }
    }

    let result = {
        let execution = &mut continuity.execution;
        let links = &mut continuity.exercise_links;
        let mut linking = LinkingExecutor {
            inner: adapter,
            links,
            decision_trace: reference.clone(),
        };
        execute_mediated(execution, traced_commitment.commitment, &mut linking)
    };

    match result {
        Ok(confirmation) => Ok(TracedExerciseConfirmation {
            confirmation,
            decision_trace: reference,
        }),
        Err(error) => Err(TracedExecutionError::Execution {
            decision_trace: reference,
            error,
        }),
    }
}

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
    fn decision_references_are_structural_ordinals() {
        let mut continuity = ProtectedDecisionContinuity::from_authority(
            AuthorityContinuity::uninhabited(),
        );
        assert_eq!(continuity.candidate_decision_ref().as_str(), "decision:1");
        continuity.next_decision_ordinal = decimal_successor(&continuity.next_decision_ordinal);
        assert_eq!(continuity.candidate_decision_ref().as_str(), "decision:2");
    }
}
