use sv_core::{
    compile_svp_assembly, compile_svp_profile, CompileError, SourceProfile, SourceUnit,
};

fn assert_closed_domain_rejection(source: &str, profile: SourceProfile, field: &str) {
    match compile_svp_profile(source, "probe.svp", profile) {
        Err(CompileError::InvalidProgram(message)) => {
            assert!(message.contains(field), "diagnóstico inesperado: {message}");
            assert!(message.contains("dominio cerrado"), "diagnóstico inesperado: {message}");
        }
        other => panic!("la sonda debía rechazarse en la frontera canónica: {other:?}"),
    }
}

#[test]
fn dg01_rechaza_kind_ajeno_en_y_es() {
    assert_closed_domain_rejection(
        "semantic_relation R { kind: ForeignRelation; }",
        SourceProfile::En,
        "kind",
    );
    assert_closed_domain_rejection(
        "relación_semántica R { clase: RelaciónExtranjera; }",
        SourceProfile::Es,
        "kind",
    );
}

#[test]
fn dg02_rechaza_pattern_ajeno_en_y_es() {
    assert_closed_domain_rejection(
        "pattern P { kind: ForeignPattern; arity: 1; }",
        SourceProfile::En,
        "kind",
    );
    assert_closed_domain_rejection(
        "patrón P { clase: PatrónExtranjero; aridad: 1; }",
        SourceProfile::Es,
        "kind",
    );
}

#[test]
fn dg03_rechaza_regime_ajeno_en_y_es() {
    assert_closed_domain_rejection(
        "semantic_relation R { kind: DeclaredRelation; } graph G { nodes: []; edges: []; relation: R; regime: ForeignRegime; }",
        SourceProfile::En,
        "regime",
    );
    assert_closed_domain_rejection(
        "relación_semántica R { clase: RelaciónDeclarada; } grafo G { nodos: []; aristas: []; relación: R; régimen: RégimenExtranjero; }",
        SourceProfile::Es,
        "regime",
    );
}

#[test]
fn dominios_canónicos_siguen_admitidos_en_y_es() {
    for regime in ["Simple", "General"] {
        let en = format!(
            "semantic_relation R {{ kind: DeclaredRelation; }} pattern P {{ kind: DeclaredPattern; arity: 1; }} graph G {{ nodes: []; edges: []; relation: R; regime: {regime}; }}"
        );
        assert!(compile_svp_profile(&en, "ok-en.svp", SourceProfile::En).is_ok());

        let es = format!(
            "relación_semántica R {{ clase: RelaciónDeclarada; }} patrón P {{ clase: PatrónDeclarado; aridad: 1; }} grafo G {{ nodos: []; aristas: []; relación: R; régimen: {regime}; }}"
        );
        assert!(compile_svp_profile(&es, "ok-es.svp", SourceProfile::Es).is_ok());
    }
}

#[test]
fn ensamblaje_no_rescata_un_literal_ajeno_de_otra_unidad() {
    let valid = "semantic_relation R { kind: DeclaredRelation; }";
    let foreign = "patrón P { clase: PatrónExtranjero; aridad: 1; }";
    match compile_svp_assembly(&[
        SourceUnit::new(valid, "a.svp", SourceProfile::En),
        SourceUnit::new(foreign, "b.svp", SourceProfile::Es),
    ]) {
        Err(CompileError::InvalidProgram(message)) => {
            assert!(message.contains("Pattern P"));
            assert!(message.contains("dominio cerrado"));
        }
        other => panic!("el ensamblaje debía conservar el rechazo: {other:?}"),
    }
}
