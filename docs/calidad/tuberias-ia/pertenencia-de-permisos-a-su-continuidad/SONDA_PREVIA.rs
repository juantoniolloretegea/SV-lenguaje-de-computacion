
// RETP-153: montaje sintético del banco R1; no autoridad profesional.
fn pc_preparar() -> (crate::ProtectedDecisionContinuity, EffectDescriptor, crate::TracedPermit) {
    let raw = permission_continuity();
    let effect = permission_effect(&raw, &authority_ref("authority:permit"), "effect:permit:allowed").clone();
    let results = permission_results(&raw, CheckResult::Accredited, false);
    let mut c = crate::ProtectedDecisionContinuity::from_authority(raw);
    let decision = crate::decide_permit_traced(&mut c, &form_ref("form:permit"), &effect, &results).unwrap();
    let crate::TracedPermitDecision::Granted(permit) = decision else { panic!("montaje de prueba no concedido") };
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
    assert_eq!(a.decision_trace(pa.decision_trace()), b.decision_trace(pb.decision_trace()));
    let referencia = pa.decision_trace().clone();
    let resultado = crate::mediate_traced_permit(&mut b, pa, &eb);
    println!("PC02 cruzado: aceptado={} mediado={}", resultado.is_ok(), b.is_mediated(&referencia));
    assert!(resultado.is_err(), "permiso ajeno aceptado con igual ordinal y contenido");
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
    println!("PC03 cruzado: aceptado={} llamadas={} eventos={}", resultado.is_ok(), executor.calls, b.exercise_event_count());
    assert!(resultado.is_err(), "compromiso ajeno ejecutado con igual ordinal y contenido");
    assert_eq!(executor.calls, 0);
    assert_eq!(b.exercise_event_count(), 0);
}
