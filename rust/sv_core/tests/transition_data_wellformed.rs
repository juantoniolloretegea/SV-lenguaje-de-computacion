use sv_core::{
    compile_svp_assembly, compile_svp_profile, CompileError, IrProgram, SourceProfile, SourceUnit,
};

const BASE_EN: &str = r#"
codomain K = { A };
output_semantics S { A -> "a"; }
cellspec C { b: 3; codomain: K; semantics: S; role: Base; }
coupledspec CC { cell: C; bridges: [1, 9]; }
coupledspec CC2 { cell: C; bridges: [1, 9]; }
semantic_relation R { kind: DeclaredRelation; constraints: [Local]; }
graph G { nodes: [CC, CC2]; edges: []; relation: R; regime: Simple; }
horizon H { architecture: G; events: [Cambio, Revision]; }
"#;

const BASE_ES: &str = r#"
codominio K = { A };
semántica_de_salida S { A -> "a"; }
especificación_de_celda C { b: 3; codominio: K; semántica: S; rol: Base; }
especificación_acoplada CC { celda: C; puentes: [1, 9]; }
especificación_acoplada CC2 { celda: C; puentes: [1, 9]; }
relación_semántica R { clase: RelaciónDeclarada; restricciones: [Local]; }
grafo G { nodos: [CC, CC2]; aristas: []; relación: R; régimen: Simple; }
horizonte H { arquitectura: G; sucesos: [Cambio, Revision]; }
"#;

fn transition_en(events: &str, induced: &str) -> String {
    format!(
        "transition_data T {{ horizon_ref: H; events: [{events}]; induced_parameters: [{induced}]; }}"
    )
}

fn transition_es(events: &str, induced: &str) -> String {
    format!(
        "datos_de_transición T {{ referencia_de_horizonte: H; sucesos: [{events}]; parámetros_inducidos: [{induced}]; }}"
    )
}

fn reject(result: Result<IrProgram, CompileError>, expected: &str) {
    match result {
        Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected),
        other => panic!("debía rechazarse antes de admitir la IR: {other:?}"),
    }
}

#[test]
fn transition_data_admite_nodos_declarados_posiciones_extremas_e_instancias_compartidas() {
    let source = format!(
        "{BASE_EN}{}",
        transition_en(
            "(Cambio, One), (Revision, Zero)",
            "(CC, 1, One), (CC, 9, Zero), (CC2, 1, U)"
        )
    );
    compile_svp_profile(&source, "valid-en.svp", SourceProfile::En).unwrap();

    let source = format!(
        "{BASE_ES}{}",
        transition_es(
            "(Cambio, Uno), (Revision, Cero)",
            "(CC, 1, Uno), (CC, 9, Cero), (CC2, 1, U)"
        )
    );
    compile_svp_profile(&source, "valid-es.svp", SourceProfile::Es).unwrap();
}

#[test]
fn transition_data_rechaza_nodo_ausente_tipo_incorrecto_o_fuera_de_arquitectura() {
    for (node_ref, expected) in [
        (
            "Fantasma",
            "E406 (InsufficientTransitionData): TransitionData T: referencia de nodo no declarada: Fantasma",
        ),
        (
            "K",
            "E406 (InsufficientTransitionData): TransitionData T: K: se esperaba CoupledSpec",
        ),
        (
            "C",
            "E406 (InsufficientTransitionData): TransitionData T: C: se esperaba CoupledSpec",
        ),
    ] {
        let source = format!(
            "{BASE_EN}{}",
            transition_en("(Cambio, One)", &format!("({node_ref}, 1, One)"))
        );
        reject(
            compile_svp_profile(&source, "bad-target.svp", SourceProfile::En),
            expected,
        );
    }

    let source = format!(
        "{BASE_EN}coupledspec CCFuera {{ cell: C; bridges: [1]; }}{}",
        transition_en("(Cambio, One)", "(CCFuera, 1, One)")
    );
    reject(
        compile_svp_profile(&source, "outside-architecture.svp", SourceProfile::En),
        "E406 (InsufficientTransitionData): TransitionData T: nodo CCFuera ajeno a la arquitectura G",
    );
}

#[test]
fn transition_data_rechaza_cero_y_posicion_superior_a_n_sin_estrechar_nat() {
    for (position, expected) in [
        (
            "0",
            "E406 (InsufficientTransitionData): TransitionData T: posición 0 fuera de [1, 9] para el nodo CC",
        ),
        (
            "10",
            "E406 (InsufficientTransitionData): TransitionData T: posición 10 fuera de [1, 9] para el nodo CC",
        ),
        (
            "184467440737095516160",
            "E406 (InsufficientTransitionData): TransitionData T: posición 184467440737095516160 fuera de [1, 9] para el nodo CC",
        ),
    ] {
        let source = format!(
            "{BASE_EN}{}",
            transition_en("(Cambio, One)", &format!("(CC, {position}, One)"))
        );
        reject(
            compile_svp_profile(&source, "bad-position.svp", SourceProfile::En),
            expected,
        );
    }
}

#[test]
fn transition_data_rechaza_tipo_de_suceso_repetido_con_valor_igual_o_distinto() {
    for second in ["One", "Zero"] {
        let source = format!(
            "{BASE_EN}{}",
            transition_en(
                &format!("(Cambio, One), (Cambio, {second})"),
                "(CC, 1, One)"
            )
        );
        reject(
            compile_svp_profile(&source, "duplicate-event.svp", SourceProfile::En),
            "E406 (InsufficientTransitionData): TransitionData T: tipo de suceso repetido: Cambio",
        );
    }
}

#[test]
fn transition_data_rechaza_destino_inducido_repetido_con_valor_igual_o_distinto() {
    for second in ["One", "Zero"] {
        let source = format!(
            "{BASE_EN}{}",
            transition_en(
                "(Cambio, One)",
                &format!("(CC, 1, One), (CC, 1, {second})")
            )
        );
        reject(
            compile_svp_profile(&source, "duplicate-target.svp", SourceProfile::En),
            "E406 (InsufficientTransitionData): TransitionData T: destino inducido repetido: (CC, 1)",
        );
    }
}

#[test]
fn ensamblaje_mixto_no_rescata_transition_data_invalido() {
    let invalid = transition_es("(Cambio, Uno)", "(CC, 10, Uno)");
    for reversed in [false, true] {
        let mut units = vec![
            SourceUnit::new(BASE_EN, "base.svp", SourceProfile::En),
            SourceUnit::new(&invalid, "transition.svp", SourceProfile::Es),
        ];
        if reversed {
            units.reverse();
        }
        reject(
            compile_svp_assembly(&units),
            "E406 (InsufficientTransitionData): TransitionData T: posición 10 fuera de [1, 9] para el nodo CC",
        );
    }
}
