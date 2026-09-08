use sv_core::{compile_svp, compile_svp_profile, compile_svp_assembly, CompileError, IrProgram, SourceProfile, SourceUnit};
const BASE: &str = include_str!("../../../tests/row7_context/base.svp");
const BASE_ES: &str = include_str!("../../../tests/row7_context/base.es.svp");

fn reject(result: Result<IrProgram, CompileError>, expected: &str) {
    match result { Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected), other => panic!("rechazo causal esperado: {other:?}") }
}

#[test]
fn trajectory_frame_arquitectura_distinta() {
    let source = include_str!("../../../tests/conformance/invalid/trajectory_frame_arquitectura_distinta.svp");
    let tail = "trajectory TR { entries: [entry(frame: F1, transition: TD1), entry(frame: FAjena)]; }";
    let tail_es = "trayectoria TR { entradas_de_trayectoria: [entrada(marco: F1, transición: TD1), entrada(marco: FAjena)]; }";
    let expected = "Trajectory TR: Frame FAjena: arquitectura G2 distinta de G1";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("FAjena").count(), 1);
    let repaired = tail.replacen("FAjena", "F2", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn trajectory_horizon_arquitectura_distinta() {
    let source = include_str!("../../../tests/conformance/invalid/trajectory_horizon_arquitectura_distinta.svp");
    let tail = "trajectory TR { entries: [entry(frame: F1, transition: TD2), entry(frame: F2)]; }";
    let tail_es = "trayectoria TR { entradas_de_trayectoria: [entrada(marco: F1, transición: TD2), entrada(marco: F2)]; }";
    let expected = "Trajectory TR: TransitionData TD2: horizonte H2 de G2 distinto de G1";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("TD2").count(), 1);
    let repaired = tail.replacen("TD2", "TD1", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_point_frame_ajeno() {
    let source = include_str!("../../../tests/conformance/invalid/query_point_frame_ajeno.svp");
    let tail = "let Q = query(QP, by: AG, in: PointEval(FAjena));";
    let tail_es = "sea Q = consultar(QP, por: AG, en: VistaEvaluaciónPuntual(FAjena));";
    let expected = "Query Q: Frame FAjena: arquitectura G2 ajena al Agent AG (G1)";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("FAjena").count(), 1);
    let repaired = tail.replacen("FAjena", "F1", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_trajectory_ajena() {
    let source = include_str!("../../../tests/conformance/invalid/query_trajectory_ajena.svp");
    let tail = "trajectory TR { entries: [entry(frame: FAjena)]; }\nlet Q = query(QT, by: AG, in: TrajectoryView(TR));";
    let tail_es = "trayectoria TR { entradas_de_trayectoria: [entrada(marco: FAjena)]; }\nsea Q = consultar(QT, por: AG, en: VistaTrayectoria(TR));";
    let expected = "Query Q: Trajectory TR: arquitectura G2 ajena al Agent AG (G1)";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("FAjena").count(), 1);
    let repaired = tail.replacen("FAjena", "F1", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_comparison_frame_ajeno() {
    let source = include_str!("../../../tests/conformance/invalid/query_comparison_frame_ajeno.svp");
    let tail = "let Q = query(QF, by: AG, in: FrameComparison(F1, FAjena));";
    let tail_es = "sea Q = consultar(QF, por: AG, en: ComparaciónMarcos(F1, FAjena));";
    let expected = "Query Q: Frame FAjena: arquitectura G2 ajena al Agent AG (G1)";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("FAjena").count(), 1);
    let repaired = tail.replacen("FAjena", "F2", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_architecture_cell_ajena() {
    let source = include_str!("../../../tests/conformance/invalid/query_architecture_cell_ajena.svp");
    let tail = "let Q = query(QA, by: AG, in: ArchitectureView(G1, [C2], [], []));";
    let tail_es = "sea Q = consultar(QA, por: AG, en: VistaArquitectura(G1, [C2], [], []));";
    let expected = "Query Q: CellSpec C2 ajena a la arquitectura G1";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("C2").count(), 1);
    let repaired = tail.replacen("C2", "C1", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_architecture_eval_nodo_ajeno() {
    let source = include_str!("../../../tests/conformance/invalid/query_architecture_eval_nodo_ajeno.svp");
    let tail = "let Q = query(QA, by: AG, in: ArchitectureView(G1, [], [EFuera], []));";
    let tail_es = "sea Q = consultar(QA, por: AG, en: VistaArquitectura(G1, [], [EFuera], []));";
    let expected = "Query Q: EvalResult EFuera ajeno a la arquitectura G1";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("EFuera").count(), 1);
    let repaired = tail.replacen("EFuera", "ECompartida", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_architecture_eval_celda_ajena() {
    let source = include_str!("../../../tests/conformance/invalid/query_architecture_eval_celda_ajena.svp");
    let tail = "let Q = query(QA, by: AG, in: ArchitectureView(G1, [], [EAjena], []));";
    let tail_es = "sea Q = consultar(QA, por: AG, en: VistaArquitectura(G1, [], [EAjena], []));";
    let expected = "Query Q: EvalResult EAjena ajeno a la arquitectura G1";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("EAjena").count(), 1);
    let repaired = tail.replacen("EAjena", "ELocal", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_architecture_gate_entrada_ajena() {
    let source = include_str!("../../../tests/conformance/invalid/query_architecture_gate_entrada_ajena.svp");
    let tail = "let Q = query(QA, by: AG, in: ArchitectureView(G1, [], [], [GateFuera]));";
    let tail_es = "sea Q = consultar(QA, por: AG, en: VistaArquitectura(G1, [], [], [GateFuera]));";
    let expected = "Query Q: GateResult GateFuera: EvalResult EFuera ajeno a la arquitectura G1";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("GateFuera").count(), 1);
    let repaired = tail.replacen("GateFuera", "Gate1", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_coverage_interface_ajena() {
    let source = include_str!("../../../tests/conformance/invalid/query_coverage_interface_ajena.svp");
    let tail = "let Q = query(QC, by: AG, in: CoverageReport(D, IAjena, SU));";
    let tail_es = "sea Q = consultar(QC, por: AG, en: InformeCobertura(D, IAjena, SU));";
    let expected = "Query Q: CoverageReport: interface IAjena distinta de Domain D (I)";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("IAjena").count(), 1);
    let repaired = tail.replacen("IAjena", "I", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}

#[test]
fn query_coverage_silent_u_ajena() {
    let source = include_str!("../../../tests/conformance/invalid/query_coverage_silent_u_ajena.svp");
    let tail = "let Q = query(QC, by: AG, in: CoverageReport(D, I, SUAjena));";
    let tail_es = "sea Q = consultar(QC, por: AG, en: InformeCobertura(D, I, SUAjena));";
    let expected = "Query Q: CoverageReport: silent_u SUAjena distinta de Domain D (SU)";
    assert_eq!(source.split("-- OBJETO_ATACADO\n").nth(1).unwrap().trim(), tail);
    reject(compile_svp(source, "testigo.svp"), expected);
    reject(compile_svp_profile(source, "testigo.svp", SourceProfile::En), expected);
    reject(compile_svp_profile(&format!("{BASE_ES}{tail_es}"), "testigo.es.svp", SourceProfile::Es), expected);
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(BASE, "base.svp", SourceProfile::En), SourceUnit::new(tail_es, "testigo.es.svp", SourceProfile::Es)];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), expected);
    }
    assert_eq!(tail.matches("SUAjena").count(), 1);
    let repaired = tail.replacen("SUAjena", "SU", 1);
    compile_svp(&format!("{BASE}{repaired}"), "control.svp").expect("reparar sólo la condición atacada debe admitir el control");
}
