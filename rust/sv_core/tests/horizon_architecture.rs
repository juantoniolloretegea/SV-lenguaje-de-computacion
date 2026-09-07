//! N0-04: referencias globales tipadas y coherencia de identidad del agente.
use sv_core::{compile_svp_profile, compile_svp_assembly, equivalence_json,
              CompileError, IrObjectKind, IrProgram, SourceProfile, SourceUnit};

const BASE_EN: &str = r#"
codomain K = { A };
output_semantics S { A -> "a"; }
cellspec C { b: 3; codomain: K; semantics: S; role: Base; }
coupledspec CC { cell: C; bridges: [3]; }
semantic_relation R { kind: DeclaredRelation; constraints: [Local]; }
graph G { nodes: [CC]; edges: []; relation: R; regime: Simple; }
"#;
const BASE_ES: &str = r#"
codominio K = { A };
semántica_de_salida S { A -> "a"; }
especificación_de_celda C { b: 3; codominio: K; semántica: S; rol: Base; }
especificación_acoplada CC { celda: C; puentes: [3]; }
relación_semántica R { clase: RelaciónDeclarada; restricciones: [Local]; }
grafo G { nodos: [CC]; aristas: []; relación: R; régimen: Simple; }
"#;

fn reject(result: Result<IrProgram, CompileError>, expected: &str) {
    match result {
        Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected),
        other => panic!("debía rechazarse antes de admitir IR: {other:?}"),
    }
}

fn horizon(profile: SourceProfile, reference: &str) -> String {
    match profile {
        SourceProfile::En => format!("horizon H {{ architecture: {reference}; events: [B,A]; }}"),
        SourceProfile::Es => format!("horizonte H {{ arquitectura: {reference}; sucesos: [B,A]; }}"),
    }
}

#[test]
fn n0_04_missing_wrong_type_self_and_operation_in_both_profiles() {
    for (profile, base, state) in [
        (SourceProfile::En, BASE_EN, "cellstate State { spec: C; vector: [U,U,U,U,U,U,U,U,U]; } let E = evaluate(State);"),
        (SourceProfile::Es, BASE_ES, "estado_de_celda State { especificación: C; vector: [U,U,U,U,U,U,U,U,U]; } sea E = evaluar(State);"),
    ] {
        for (reference, expected) in [
            ("Missing", "referencia no declarada: Missing"),
            ("K", "K: se esperaba CompositionGraph"),
            ("H", "H: se esperaba CompositionGraph"),
            ("E", "E no es un objeto declarado"),
        ] {
            reject(compile_svp_profile(&format!("{base}{state}{}", horizon(profile, reference)), "bad.svp", profile), expected);
        }
    }
}

#[test]
fn n0_04_forward_reference_preserves_graph_and_events() {
    for (profile, base) in [(SourceProfile::En, BASE_EN), (SourceProfile::Es, BASE_ES)] {
        for source in [format!("{}{base}", horizon(profile, "G")), format!("{base}{}", horizon(profile, "G"))] {
            let program = compile_svp_profile(&source, "forward.svp", profile).unwrap();
            let before = program.clone();
            let actual = program.objects().iter().find(|o| o.name() == "H").unwrap();
            match actual.kind() {
                IrObjectKind::Horizon { architecture, events } => {
                    assert_eq!(architecture, "G");
                    assert_eq!(events, &["B", "A"]);
                }
                other => panic!("objeto inesperado: {other:?}"),
            }
            assert_eq!(equivalence_json(&program), equivalence_json(&program));
            assert_eq!(program, before);
        }
    }
}

#[test]
fn n0_04_agent_identity_and_equal_missing_names() {
    let source = include_str!("../../../tests/conformance/invalid/agent_arquitecturas_reales_distintas.svp");
    reject(compile_svp_profile(source, "agent.svp", SourceProfile::En), "Agent AG: architecture incompatible con Domain");
    let matching = source.replace("agent AG { architecture: Arch2;", "agent AG { architecture: Arch1;");
    let agent = "agent AG { architecture: Arch1; domain: D; query_engine: QE; }";
    for source in [matching.clone(), format!("{agent}\n{}", matching.replace(agent, ""))] {
        compile_svp_profile(&source, "agent.svp", SourceProfile::En).unwrap();
        let absent = source.replace("architecture: Arch1;", "architecture: Missing;");
        reject(compile_svp_profile(&absent, "agent.svp", SourceProfile::En), "referencia no declarada: Missing");
    }
}

#[test]
fn n0_04_mixed_assembly_resolves_or_rejects_in_both_orders() {
    for (profile, base, other) in [(SourceProfile::En, BASE_EN, SourceProfile::Es), (SourceProfile::Es, BASE_ES, SourceProfile::En)] {
        for (reference, expected) in [("G", None), ("Missing", Some("referencia no declarada: Missing")), ("K", Some("K: se esperaba CompositionGraph"))] {
            let source = horizon(other, reference);
            for reversed in [false, true] {
                let mut units = vec![SourceUnit::new(base, "graph.svp", profile), SourceUnit::new(&source, "horizon.svp", other)];
                if reversed { units.reverse(); }
                let result = compile_svp_assembly(&units);
                if let Some(expected) = expected { reject(result, expected); }
                else {
                    let json = equivalence_json(&result.unwrap());
                    assert!(json.contains("\"architecture\":\"G\""));
                    assert!(json.contains("\"events\":[\"B\",\"A\"]"));
                }
            }
        }
    }
}

#[test]
fn n0_04_existing_graph_must_be_wellformed() {
    let source = format!("{}{}", horizon(SourceProfile::En, "G"), BASE_EN.replace("nodes: [CC]", "nodes: [S]"));
    reject(compile_svp_profile(&source, "badgraph.svp", SourceProfile::En), "S: se esperaba CoupledSpec");
}
