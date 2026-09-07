use sv_core::{compile_svp_profile, CompileError, IrObjectKind, IrProgram, SourceProfile};

fn reject(result: Result<IrProgram, CompileError>, expected: &str) {
    match result {
        Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected),
        other => panic!("debía rechazarse antes de admitir la IR: {other:?}"),
    }
}

fn base_en(b: &str) -> String {
    format!(
        "codomain K = {{ A }}; output_semantics S {{ A -> \"a\"; }} cellspec C {{ b: {b}; codomain: K; semantics: S; role: Base; }}"
    )
}

#[test]
fn b_menor_que_tres_se_rechaza_nativamente_en_ambos_perfiles() {
    reject(
        compile_svp_profile(&base_en("2"), "b2-en.svp", SourceProfile::En),
        "CellSpec C: b debe ser >= 3",
    );
    reject(
        compile_svp_profile(
            "codominio K = { A }; semántica_de_salida S { A -> \"a\"; } especificación_de_celda C { b: 2; codominio: K; semántica: S; rol: Base; }",
            "b2-es.svp",
            SourceProfile::Es,
        ),
        "CellSpec C: b debe ser >= 3",
    );
}

#[test]
fn b_tres_y_b_cuatro_derivan_nueve_y_dieciseis_posiciones() {
    for (b, expected_n) in [("3", "9"), ("4", "16")] {
        let program = compile_svp_profile(&base_en(b), "geometry.svp", SourceProfile::En)
            .expect("geometría admisible");
        let cell = program
            .objects()
            .iter()
            .find(|object| object.name() == "C")
            .expect("CellSpec C");
        match cell.kind() {
            IrObjectKind::CellSpec { b: actual_b, n, .. } => {
                assert_eq!(actual_b.as_decimal(), b);
                assert_eq!(n.as_decimal(), expected_n);
            }
            other => panic!("objeto inesperado: {other:?}"),
        }
    }
}

#[test]
fn longitud_de_vector_distinta_de_n_se_rechaza_nativamente() {
    for length in [8usize, 10usize] {
        let vector = std::iter::repeat_n("Zero", length).collect::<Vec<_>>().join(", ");
        let source = format!(
            "{} cellstate State {{ spec: C; vector: [{vector}]; }}",
            base_en("3")
        );
        reject(
            compile_svp_profile(&source, "bad-length.svp", SourceProfile::En),
            "CellState State: longitud de vector incompatible",
        );
    }
}

#[test]
fn vector_de_longitud_dieciseis_es_admisible_para_b_cuatro() {
    let vector = std::iter::repeat_n("Zero", 16).collect::<Vec<_>>().join(", ");
    let source = format!(
        "{} cellstate State {{ spec: C; vector: [{vector}]; }}",
        base_en("4")
    );
    compile_svp_profile(&source, "b4-state.svp", SourceProfile::En)
        .expect("vector de longitud b²");
}
