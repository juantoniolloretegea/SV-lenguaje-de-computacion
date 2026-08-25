//! Ejecución gobernada de T-E para R1-4.
//!
//! Esta unidad consume exclusivamente `MediatedEffectCommitment`. No vuelve a
//! decidir autoridad, requisitos o permiso. El adaptador recibe una solicitud
//! sellada no fabricable y no puede convertir su resultado técnico en `Tri` ni
//! en un resultado D-*.
//!
//! La traza de ejercicio aquí representada es lógica e intra-proceso. No
//! acredita persistencia durable, atomicidad con el mundo externo ni
//! correspondencia material independiente.

use crate::authority::transitions::AuthorityContinuity;
use crate::authority::{AccumulationContract, EffectDescriptor};
use crate::control::{
    AccumulationRuleRef, AuthorityHolderRef, AuthorityRef, CheckResult, ContextRef, ControlId,
    EffectFamilyRef, EffectRef, ExerciseRef, FormRef, GovernedObjectRef, TransitionClass,
};
use crate::mediation::{
    revalidate_mediated_commitment, MediatedEffectCommitment, MediationError,
};

/// Estado técnico append-only de un intento de ejercicio.
///
/// `DispatchCommitted` significa que el núcleo ya consumió el compromiso y
/// registró el intento inmediatamente antes de invocar el adaptador. Si no
/// aparece un evento terminal posterior, el intento debe tratarse de forma
/// conservadora como potencialmente ejecutado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseAttemptState {
    DispatchCommitted,
    Confirmed,
    Indeterminate,
}

/// Entrada inmutable de la traza lógica de ejercicios.
///
/// La entrada puede consultarse y copiarse como evidencia técnica local, pero
/// sus campos no son fabricables mediante un constructor público.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExerciseTraceEntry {
    exercise: ExerciseRef,
    authority: AuthorityRef,
    form: FormRef,
    effect: EffectDescriptor,
    context: ContextRef,
    accumulation: AccumulationContract,
    state: ExerciseAttemptState,
}

impl ExerciseTraceEntry {
    #[inline]
    pub fn exercise(&self) -> &ExerciseRef {
        &self.exercise
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        &self.authority
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        &self.form
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
    pub fn accumulation(&self) -> &AccumulationContract {
        &self.accumulation
    }

    #[inline]
    pub const fn state(&self) -> ExerciseAttemptState {
        self.state
    }

    fn same_scope(&self, commitment: &MediatedEffectCommitment) -> bool {
        self.authority == *commitment.authority()
            && self.form == *commitment.form()
            && self.effect == *commitment.effect()
            && self.context == *commitment.context()
    }
}

/// Continuidad lógica de ejecución de R1-4.
///
/// El tipo consume `AuthorityContinuity` y no ofrece acceso mutable a ella. De
/// este modo, en este corte forma, autoridad, `E_max`, `D_a` y `Req` permanecen
/// inmutables mientras sólo T-E adquiere productividad. La futura habilitación
/// de T-G, T-C o T-R deberá introducir su propia revalidación o versión
/// gobernante antes de coexistir con la ejecución.
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutionContinuity {
    authority: AuthorityContinuity,
    next_exercise_ordinal: String,
    trace: Vec<ExerciseTraceEntry>,
}

impl ExecutionContinuity {
    /// Transfiere una continuidad autoritativa a la frontera de ejecución.
    ///
    /// La operación consume el objeto original. No demuestra identidad durable
    /// entre procesos ni crea autoridad nueva.
    pub fn from_authority(authority: AuthorityContinuity) -> Self {
        Self {
            authority,
            next_exercise_ordinal: "1".to_owned(),
            trace: Vec::new(),
        }
    }

    /// Vista inmutable del estado autoritativo usado por decisión y mediación.
    #[inline]
    pub fn authority(&self) -> &AuthorityContinuity {
        &self.authority
    }

    #[inline]
    pub fn exercise_event_count(&self) -> usize {
        self.trace.len()
    }

    #[inline]
    pub fn exercise_events(&self) -> impl Iterator<Item = &ExerciseTraceEntry> {
        self.trace.iter()
    }

    /// Estado terminal o pendiente conocido de un intento concreto.
    ///
    /// La búsqueda sigue la cadena causal append-only del mismo `ExerciseRef`;
    /// no implementa una regla cronológica «el último gana» entre ejercicios.
    pub fn exercise_state(&self, exercise: &ExerciseRef) -> Option<ExerciseAttemptState> {
        self.trace
            .iter()
            .rev()
            .find(|entry| entry.exercise() == exercise)
            .map(ExerciseTraceEntry::state)
    }

    fn next_exercise_ref(&mut self) -> ExerciseRef {
        let ordinal = self.next_exercise_ordinal.clone();
        self.next_exercise_ordinal = decimal_successor(&ordinal);
        let id = ControlId::new(format!("exercise:{ordinal}"))
            .expect("el ordinal interno siempre produce un identificador no vacío");
        ExerciseRef::from_core_id(id)
    }

    fn append_event(
        &mut self,
        exercise: &ExerciseRef,
        commitment: &MediatedEffectCommitment,
        state: ExerciseAttemptState,
    ) {
        self.trace.push(ExerciseTraceEntry {
            exercise: exercise.clone(),
            authority: commitment.authority().clone(),
            form: commitment.form().clone(),
            effect: commitment.effect().clone(),
            context: commitment.context().clone(),
            accumulation: commitment.accumulation().clone(),
            state,
        });
    }

    fn prior_dispatch_in_scope(
        &self,
        commitment: &MediatedEffectCommitment,
    ) -> Option<&ExerciseRef> {
        self.trace
            .iter()
            .find(|entry| {
                entry.state() == ExerciseAttemptState::DispatchCommitted
                    && entry.same_scope(commitment)
            })
            .map(ExerciseTraceEntry::exercise)
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

/// Solicitud sellada entregada al adaptador.
///
/// No existe constructor público. Obtener referencias desde esta solicitud no
/// permite producir otro compromiso ni otra solicitud ejecutable.
///
/// ```compile_fail
/// use sv_core::ExecutionRequest;
/// let _ = ExecutionRequest::new();
/// ```
#[derive(Debug)]
pub struct ExecutionRequest<'a> {
    exercise: &'a ExerciseRef,
    commitment: &'a MediatedEffectCommitment,
}

impl<'a> ExecutionRequest<'a> {
    #[inline]
    pub fn exercise(&self) -> &ExerciseRef {
        self.exercise
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        self.commitment.authority()
    }

    #[inline]
    pub fn authority_holder(&self) -> &AuthorityHolderRef {
        self.commitment.authority_holder()
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        self.commitment.form()
    }

    #[inline]
    pub const fn transition_class(&self) -> TransitionClass {
        self.commitment.transition_class()
    }

    #[inline]
    pub fn form_effect_family(&self) -> &EffectFamilyRef {
        self.commitment.form_effect_family()
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        self.commitment.effect()
    }

    #[inline]
    pub fn effect_reference(&self) -> &EffectRef {
        self.commitment.effect_reference()
    }

    #[inline]
    pub fn governed_object(&self) -> &GovernedObjectRef {
        self.commitment.governed_object()
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        self.commitment.context()
    }

    #[inline]
    pub fn accumulation(&self) -> &AccumulationContract {
        self.commitment.accumulation()
    }
}

/// Puerto mínimo de ejecución material.
///
/// El adaptador implementa esta interfaz fuera de la semántica de autoridad.
/// La única forma de obtener un `ExecutionRequest` productivo es atravesar
/// `execute_mediated`.
pub trait EffectExecutor {
    type Error;

    fn execute(&mut self, request: &ExecutionRequest<'_>) -> Result<(), Self::Error>;
}

/// Confirmación lógica local de un ejercicio tras respuesta positiva del
/// adaptador.
///
/// No constituye atestación independiente del mundo externo ni autoridad.
#[derive(Debug, PartialEq, Eq)]
pub struct ExerciseConfirmation {
    exercise: ExerciseRef,
    authority: AuthorityRef,
    form: FormRef,
    effect: EffectDescriptor,
    context: ContextRef,
}

impl ExerciseConfirmation {
    #[inline]
    pub fn exercise(&self) -> &ExerciseRef {
        &self.exercise
    }

    #[inline]
    pub fn authority(&self) -> &AuthorityRef {
        &self.authority
    }

    #[inline]
    pub fn form(&self) -> &FormRef {
        &self.form
    }

    #[inline]
    pub fn effect(&self) -> &EffectDescriptor {
        &self.effect
    }

    #[inline]
    pub fn context(&self) -> &ContextRef {
        &self.context
    }
}

/// Rechazo anterior al despacho o resultado técnico indeterminado posterior.
///
/// Ninguna variante pertenece a `Tri` ni a `CheckResult`.
#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionError<E> {
    UnsupportedTransitionClass(TransitionClass),
    NonAccreditedCommitment,
    MediationRevalidation(MediationError),
    CurrentFormMissing(FormRef),
    CurrentFormBindingChanged(FormRef),
    CurrentAuthorityMissing(AuthorityRef),
    CurrentAuthorityBindingChanged(AuthorityRef),
    EffectOutsideCurrentScope {
        authority: AuthorityRef,
        effect: EffectRef,
    },
    SingleUseAlreadyDispatched(ExerciseRef),
    GovernedAggregatorUnavailable(AccumulationRuleRef),
    TracePredicateUnavailable(AccumulationRuleRef),
    AdapterIndeterminate {
        exercise: ExerciseRef,
        error: E,
    },
}

fn current_binding_is_compatible(
    continuity: &AuthorityContinuity,
    commitment: &MediatedEffectCommitment,
) -> Result<(), ExecutionError<core::convert::Infallible>> {
    let form = continuity
        .form(commitment.form())
        .ok_or_else(|| ExecutionError::CurrentFormMissing(commitment.form().clone()))?;

    let form_matches = form.transition_class() == commitment.transition_class()
        && form.effect_family() == commitment.form_effect_family()
        && form.accumulation() == commitment.accumulation()
        && form
            .context_bindings()
            .any(|context| context == commitment.context())
        && form.requires_authority() == Some(commitment.authority());

    if !form_matches {
        return Err(ExecutionError::CurrentFormBindingChanged(
            commitment.form().clone(),
        ));
    }

    let authority = continuity
        .authority(commitment.authority())
        .ok_or_else(|| ExecutionError::CurrentAuthorityMissing(commitment.authority().clone()))?;

    if authority.holder() != commitment.authority_holder()
        || authority.context() != commitment.authority_context()
    {
        return Err(ExecutionError::CurrentAuthorityBindingChanged(
            commitment.authority().clone(),
        ));
    }

    if !form.describes_effect(commitment.effect())
        || !authority.contains_effect_scope(commitment.effect())
    {
        return Err(ExecutionError::EffectOutsideCurrentScope {
            authority: commitment.authority().clone(),
            effect: commitment.effect_reference().clone(),
        });
    }

    Ok(())
}

fn map_infallible_gate_error<E>(
    error: ExecutionError<core::convert::Infallible>,
) -> ExecutionError<E> {
    match error {
        ExecutionError::UnsupportedTransitionClass(class) => {
            ExecutionError::UnsupportedTransitionClass(class)
        }
        ExecutionError::NonAccreditedCommitment => ExecutionError::NonAccreditedCommitment,
        ExecutionError::MediationRevalidation(error) => ExecutionError::MediationRevalidation(error),
        ExecutionError::CurrentFormMissing(form) => ExecutionError::CurrentFormMissing(form),
        ExecutionError::CurrentFormBindingChanged(form) => {
            ExecutionError::CurrentFormBindingChanged(form)
        }
        ExecutionError::CurrentAuthorityMissing(authority) => {
            ExecutionError::CurrentAuthorityMissing(authority)
        }
        ExecutionError::CurrentAuthorityBindingChanged(authority) => {
            ExecutionError::CurrentAuthorityBindingChanged(authority)
        }
        ExecutionError::EffectOutsideCurrentScope { authority, effect } => {
            ExecutionError::EffectOutsideCurrentScope { authority, effect }
        }
        ExecutionError::SingleUseAlreadyDispatched(exercise) => {
            ExecutionError::SingleUseAlreadyDispatched(exercise)
        }
        ExecutionError::GovernedAggregatorUnavailable(rule) => {
            ExecutionError::GovernedAggregatorUnavailable(rule)
        }
        ExecutionError::TracePredicateUnavailable(rule) => {
            ExecutionError::TracePredicateUnavailable(rule)
        }
        ExecutionError::AdapterIndeterminate { error, .. } => match error {},
    }
}

/// Ejecuta una T-E mediada contra un puerto explícito.
///
/// El compromiso se consume. Antes de abrir el despacho se repite la misma
/// revalidación completa de mediación sobre `Req`, 3E y las aplicabilidades
/// participantes. La traza registra `DispatchCommitted` antes de invocar el
/// adaptador. Un error posterior queda como `Indeterminate`; no se interpreta
/// como ausencia de efecto ni devuelve el compromiso para reintento.
pub fn execute_mediated<A: EffectExecutor>(
    continuity: &mut ExecutionContinuity,
    commitment: MediatedEffectCommitment,
    adapter: &mut A,
) -> Result<ExerciseConfirmation, ExecutionError<A::Error>> {
    if commitment.transition_class() != TransitionClass::Exercise {
        return Err(ExecutionError::UnsupportedTransitionClass(
            commitment.transition_class(),
        ));
    }

    if commitment.technical_result() != CheckResult::Accredited {
        return Err(ExecutionError::NonAccreditedCommitment);
    }

    revalidate_mediated_commitment(continuity.authority(), &commitment)
        .map_err(ExecutionError::MediationRevalidation)?;

    current_binding_is_compatible(continuity.authority(), &commitment)
        .map_err(map_infallible_gate_error)?;

    match commitment.accumulation() {
        AccumulationContract::NotApplicable | AccumulationContract::Idempotent => {}
        AccumulationContract::SingleUse => {
            if let Some(previous) = continuity.prior_dispatch_in_scope(&commitment) {
                return Err(ExecutionError::SingleUseAlreadyDispatched(previous.clone()));
            }
        }
        AccumulationContract::GovernedAggregator(rule) => {
            return Err(ExecutionError::GovernedAggregatorUnavailable(rule.clone()));
        }
        AccumulationContract::DecidableTracePredicate(rule) => {
            return Err(ExecutionError::TracePredicateUnavailable(rule.clone()));
        }
    }

    let exercise = continuity.next_exercise_ref();
    continuity.append_event(
        &exercise,
        &commitment,
        ExerciseAttemptState::DispatchCommitted,
    );

    let request = ExecutionRequest {
        exercise: &exercise,
        commitment: &commitment,
    };

    match adapter.execute(&request) {
        Ok(()) => {
            continuity.append_event(&exercise, &commitment, ExerciseAttemptState::Confirmed);
            Ok(ExerciseConfirmation {
                exercise,
                authority: commitment.authority().clone(),
                form: commitment.form().clone(),
                effect: commitment.effect().clone(),
                context: commitment.context().clone(),
            })
        }
        Err(error) => {
            continuity.append_event(
                &exercise,
                &commitment,
                ExerciseAttemptState::Indeterminate,
            );
            Err(ExecutionError::AdapterIndeterminate { exercise, error })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::decimal_successor;

    #[test]
    fn decimal_successor_is_unbounded_by_machine_word_width() {
        assert_eq!(decimal_successor("0"), "1");
        assert_eq!(decimal_successor("9"), "10");
        assert_eq!(
            decimal_successor("999999999999999999999999999999"),
            "1000000000000000000000000000000"
        );
    }
}
