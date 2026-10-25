use super::*;

// RETP-153: montaje sintético del banco R1; no autoridad profesional.
fn pc_preparar() -> (
    crate::ProtectedDecisionContinuity,
    EffectDescriptor,
    crate::TracedPermit,
) {
    let raw = permission_continuity();
    let effect = permission_effect(
        &raw,
        &authority_ref("authority:permit"),
        "effect:permit:allowed",
    )
    .clone();
    let results = permission_results(&raw, CheckResult::Accredited, false);
    let mut c = crate::ProtectedDecisionContinuity::from_authority(raw);
    let decision =
        crate::decide_permit_traced(&mut c, &form_ref("form:permit"), &effect, &results).unwrap();
    let crate::TracedPermitDecision::Granted(permit) = decision else {
        panic!("montaje de prueba no concedido")
    };
    (c, effect, permit)
}
#[test]
fn pc01_cadena_propia() {
    let (mut a, effect, permit) = pc_preparar();
    let commitment = crate::mediate_traced_permit(&mut a, permit, &effect).unwrap();
    let mut executor = TestExecutor::default();
    crate::execute_traced_mediated(&mut a, commitment, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
    println!("PC01 propia: llamadas={}", executor.calls);
}
#[test]
fn pc02_permiso_cruzado_igual_contenido() {
    let (a, ea, pa) = pc_preparar();
    let (mut b, eb, pb) = pc_preparar();
    assert_eq!(ea, eb);
    assert_eq!(pa.decision_trace(), pb.decision_trace());
    assert_eq!(
        a.decision_trace(pa.decision_trace()),
        b.decision_trace(pb.decision_trace())
    );
    let referencia = pa.decision_trace().clone();
    let resultado = crate::mediate_traced_permit(&mut b, pa, &eb);
    println!(
        "PC02 cruzado: aceptado={} mediado={}",
        resultado.is_ok(),
        b.is_mediated(&referencia)
    );
    assert!(
        matches!(
            resultado,
            Err(crate::TracedMediationError::ForeignContinuity)
        ),
        "permiso ajeno no rechazado por pertenencia"
    );
    assert!(!b.is_mediated(&referencia));
}
#[test]
fn pc03_compromiso_cruzado_igual_contenido() {
    let (mut a, ea, pa) = pc_preparar();
    let (mut b, eb, pb) = pc_preparar();
    assert_eq!(ea, eb);
    assert_eq!(pa.decision_trace(), pb.decision_trace());
    let ca = crate::mediate_traced_permit(&mut a, pa, &ea).unwrap();
    let _cb = crate::mediate_traced_permit(&mut b, pb, &eb).unwrap();
    let mut executor = TestExecutor::default();
    let resultado = crate::execute_traced_mediated(&mut b, ca, &mut executor);
    println!(
        "PC03 cruzado: aceptado={} llamadas={} eventos={}",
        resultado.is_ok(),
        executor.calls,
        b.exercise_event_count()
    );
    assert!(
        matches!(
            resultado,
            Err(crate::TracedExecutionError::ForeignContinuity)
        ),
        "compromiso ajeno no rechazado por pertenencia"
    );
    assert_eq!(executor.calls, 0);
    assert_eq!(b.exercise_event_count(), 0);
}

#[test]
fn pc04_exhaustion_stops_before_decision_and_mediation() {
    let (mut c, effect, permit) = pc_preparar();
    c.simulate_identity_exhaustion();
    let previous = c.decision_trace_count();
    let r = crate::decide_permit_traced(&mut c, &form_ref("form:permit"), &effect, &[]);
    assert!(matches!(
        r,
        Err(crate::TracedDecisionError::ContinuityExhausted)
    ));
    assert_eq!(c.decision_trace_count(), previous);
    assert!(matches!(
        crate::mediate_traced_permit(&mut c, permit, &effect),
        Err(crate::TracedMediationError::ForeignContinuity)
    ));
    assert_eq!(c.exercise_event_count(), 0);
}

// ES: El movimiento de Rust cambia residencia, no pertenencia ni facultades.
// EN: A Rust move changes placement, not membership or authority.
#[test]
fn pc07_move_preserves_membership() {
    let (mut original, effect, permit) = pc_preparar();
    let commitment = crate::mediate_traced_permit(&mut original, permit, &effect).unwrap();
    let mut moved = Box::new(original);
    let mut executor = TestExecutor::default();
    let confirmation =
        crate::execute_traced_mediated(&mut moved, commitment, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
    assert!(moved
        .exercise_decision_trace(confirmation.exercise())
        .is_some());
}

// ES: Rechazar la pieza ajena no consume el permiso legítimo del receptor.
// EN: Rejecting a foreign token does not consume the recipient's own permit.
#[test]
fn pc08_foreign_rejection_preserves_own_operation() {
    let (_, _, foreign_permit) = pc_preparar();
    let (mut receiver, effect, own_permit) = pc_preparar();
    let reference = own_permit.decision_trace().clone();
    let trace_before = receiver.decision_trace(&reference).unwrap().clone();
    assert!(matches!(
        crate::mediate_traced_permit(&mut receiver, foreign_permit, &effect),
        Err(crate::TracedMediationError::ForeignContinuity)
    ));
    assert_eq!(receiver.decision_trace(&reference), Some(&trace_before));
    assert!(!receiver.is_mediated(&reference));
    let commitment = crate::mediate_traced_permit(&mut receiver, own_permit, &effect).unwrap();
    let mut executor = TestExecutor::default();
    crate::execute_traced_mediated(&mut receiver, commitment, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
}

#[test]
fn pc09_foreign_dispatch_preserves_own_commitment() {
    let (mut source, source_effect, source_permit) = pc_preparar();
    let foreign = crate::mediate_traced_permit(&mut source, source_permit, &source_effect).unwrap();
    drop(source);
    let (mut receiver, effect, permit) = pc_preparar();
    let own = crate::mediate_traced_permit(&mut receiver, permit, &effect).unwrap();
    let mut executor = TestExecutor::default();
    assert!(matches!(
        crate::execute_traced_mediated(&mut receiver, foreign, &mut executor),
        Err(crate::TracedExecutionError::ForeignContinuity)
    ));
    assert_eq!(executor.calls, 0);
    assert_eq!(receiver.exercise_event_count(), 0);
    crate::execute_traced_mediated(&mut receiver, own, &mut executor).unwrap();
    assert_eq!(executor.calls, 1);
}

// ES: Un fallo posterior al despacho conserva su causa y consume el uso único;
// una decisión nueva no convierte el efecto incierto en permiso para repetirlo.
// EN: Failure after dispatch retains its cause and consumes the single use;
// a new decision cannot turn an uncertain effect into permission to repeat it.
#[test]
fn pc10_traced_indeterminate_effect_preserves_cause_and_blocks_retry() {
    let (mut continuity, effect, permit) = pc_preparar();
    let first_decision = permit.decision_trace().clone();
    let commitment = crate::mediate_traced_permit(&mut continuity, permit, &effect).unwrap();
    let mut failing = TestExecutor {
        fail: true,
        ..Default::default()
    };
    let result = crate::execute_traced_mediated(&mut continuity, commitment, &mut failing);
    let Err(crate::TracedExecutionError::Execution {
        decision_trace,
        error: ExecutionError::AdapterIndeterminate { exercise, error },
    }) = result
    else {
        panic!("el fallo posterior al despacho debe conservar su decisión y causa");
    };
    assert_eq!(decision_trace, first_decision);
    assert_eq!(error, crate::TracedAdapterError::Adapter("adapter-error"));
    assert_eq!(failing.calls, 1);
    assert_eq!(
        continuity.exercise_decision_ref(&exercise),
        Some(&first_decision)
    );
    assert_eq!(
        continuity.exercise_state(&exercise),
        Some(ExerciseAttemptState::Indeterminate)
    );
    let events_before = continuity.exercise_events().cloned().collect::<Vec<_>>();
    assert_eq!(events_before.len(), 2);
    assert_eq!(
        events_before[0].state(),
        ExerciseAttemptState::DispatchCommitted
    );
    assert_eq!(
        events_before[1].state(),
        ExerciseAttemptState::Indeterminate
    );

    let results = permission_results(continuity.authority(), CheckResult::Accredited, false);
    let crate::TracedPermitDecision::Granted(second_permit) =
        crate::decide_permit_traced(&mut continuity, &form_ref("form:permit"), &effect, &results)
            .unwrap()
    else {
        panic!("el montaje debe llegar a comprobar el consumo en el despacho");
    };
    let second_decision = second_permit.decision_trace().clone();
    assert_ne!(first_decision, second_decision);
    let second = crate::mediate_traced_permit(&mut continuity, second_permit, &effect).unwrap();
    let mut retry = TestExecutor::default();
    assert_eq!(
        crate::execute_traced_mediated(&mut continuity, second, &mut retry),
        Err(crate::TracedExecutionError::Execution {
            decision_trace: second_decision,
            error: ExecutionError::SingleUseAlreadyDispatched(exercise.clone()),
        })
    );
    assert_eq!(retry.calls, 0);
    assert_eq!(
        continuity.exercise_events().cloned().collect::<Vec<_>>(),
        events_before
    );
    assert_eq!(
        continuity.exercise_decision_ref(&exercise),
        Some(&first_decision)
    );
}

// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
