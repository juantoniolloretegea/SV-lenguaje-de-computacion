use sv_core::{compile_svp_profile, CompileError, IrProgram, SourceProfile};

fn reject(result: Result<IrProgram, CompileError>, expected: &str) {
    match result {
        Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected),
        other => panic!("debía rechazarse antes de admitir la IR: {other:?}"),
    }
}

#[test]
fn frame_architecture_exige_composition_graph_en_ambos_perfiles() {
    for (source, profile) in [
        (
            r#"
codomain K = { A };
frame F {
  index: 0;
  architecture: K;
  cell_states: [];
  eval_results: [];
  gate_results: [];
  supervision: [];
  criticalities: [];
}
"#,
            SourceProfile::En,
        ),
        (
            r#"
codominio K = { A };
marco F {
  índice: 0;
  arquitectura: K;
  estados_de_celda: [];
  resultados_de_evaluación: [];
  resultados_de_compuerta: [];
  supervisión: [];
  criticidades: [];
}
"#,
            SourceProfile::Es,
        ),
    ] {
        reject(
            compile_svp_profile(source, "frame-wrong-type.svp", profile),
            "K: se esperaba CompositionGraph",
        );
    }
}

#[test]
fn compose_graph_exige_composition_graph_en_ambos_perfiles() {
    for (source, profile) in [
        (
            r#"
codomain K = { A };
semantic_relation R { kind: DeclaredRelation; }
pattern P { kind: DeclaredPattern; }
let X = compose(K, relations: [R], patterns: [P]);
"#,
            SourceProfile::En,
        ),
        (
            r#"
codominio K = { A };
relación_semántica R { clase: RelaciónDeclarada; }
patrón P { clase: PatrónDeclarado; }
sea X = componer(K, relaciones: [R], patrones: [P]);
"#,
            SourceProfile::Es,
        ),
    ] {
        reject(
            compile_svp_profile(source, "compose-wrong-type.svp", profile),
            "K: se esperaba CompositionGraph",
        );
    }
}

#[test]
fn frame_y_compose_conservan_sus_controles_validos() {
    let source = r#"
codomain K = { A };
output_semantics S { A -> "a"; }
cellspec C { b: 3; codomain: K; semantics: S; role: Base; }
coupledspec CC { cell: C; bridges: []; }
semantic_relation R { kind: DeclaredRelation; }
pattern P { kind: DeclaredPattern; }
graph G { nodes: [CC]; edges: []; relation: R; regime: Simple; }
frame F {
  index: 0;
  architecture: G;
  cell_states: [];
  eval_results: [];
  gate_results: [];
  supervision: [];
  criticalities: [];
}
let X = compose(G, relations: [R], patterns: [P]);
"#;

    compile_svp_profile(source, "valid-guards.svp", SourceProfile::En).unwrap();
}
