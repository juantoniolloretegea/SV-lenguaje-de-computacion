//! Esperados de RETP-114 fijados antes de la realización y de su ejecución.
use sv_core::{compile_svp_assembly, compile_svp_profile, CompileError, ProgramCause,
    ProgramDiagnostic, SourceProfile, SourceUnit};

fn diagnostic(error: &CompileError) -> &ProgramDiagnostic {
    error.program_diagnostic().expect("causa estructurada requerida")
}
fn error(source: &str, profile: SourceProfile) -> CompileError {
    compile_svp_profile(source, "igual.svp", profile).unwrap_err()
}
fn context(d: &ProgramDiagnostic, i: usize, unit: usize, source: &str, declaration: &str, profile: SourceProfile) {
    let c = &d.contexts()[i];
    assert_eq!(c.unit_index(), unit);
    assert_eq!(c.source_file(), "igual.svp");
    assert_eq!(c.profile(), profile);
    let start = source.find(declaration).unwrap();
    assert_eq!(c.byte_range(), Some((start, start + declaration.len())));
    assert_eq!(c.source_sha256().len(), 64);
}

#[test]
fn codominio_causa_codigo_prosa_y_control_en_ambos_perfiles() {
    for (profile, word, explanation) in [
        (SourceProfile::Es, "codominio", "El codominio \"K\" no contiene valores."),
        (SourceProfile::En, "codomain", "Codomain \"K\" contains no values."),
    ] {
        let source = format!("{word} K = {{ }};");
        let e = error(&source, profile);
        let d = diagnostic(&e);
        assert_eq!(d.version(), "diagnostico-validacion/1");
        assert_eq!(d.code(), Some("E004"));
        assert_eq!(d.cause(), &ProgramCause::EmptyCodomain { object: "K".into() });
        assert_eq!(d.explanations(), vec![(profile, explanation.into())]);
        context(d, 0, 0, &source, &source, profile);
        assert_eq!(format!("{e:?}"), "InvalidProgram(\"E004 (InvalidCodomain): codomain K vacío\")");
        assert!(compile_svp_profile(&format!("{word} K = {{ A }};"), "igual.svp", profile).is_ok());
        let repeated = error(&format!("{word} K = {{ Z, A, Z, A }};"), profile);
        assert_eq!(diagnostic(&repeated).cause(), &ProgramCause::RepeatedCodomainValues {
            object: "K".into(), values: vec!["A".into(), "Z".into()] });
        assert_eq!(diagnostic(&repeated).code(), Some("E004"));
    }
}

#[test]
fn semantica_distingue_repetidas_ausentes_y_ajenas_sin_revelar_textos() {
    for (mappings, repeated, missing, extra) in [
        ("", vec![], vec!["A", "B"], vec![]),
        ("A -> \"reservado\";", vec![], vec!["B"], vec![]),
        ("A -> \"reservado\"; B -> \"secreto\"; X -> \"dato\";", vec![], vec![], vec!["X"]),
        ("A -> \"reservado\"; A -> \"secreto\"; X -> \"dato\";", vec!["A"], vec!["B"], vec!["X"]),
    ] {
        for (profile, kd, sd, cd) in [
            (SourceProfile::En, "codomain K = { A, B };", "output_semantics", "cellspec C { b: 3; codomain: K; semantics: S; role: Base; }"),
            (SourceProfile::Es, "codominio K = { A, B };", "semántica_de_salida", "especificación_de_celda C { b: 3; codominio: K; semántica: S; rol: Base; }"),
        ] {
            let sem = format!("{sd} S {{ {mappings} }}");
            let source = format!("{cd}\r\n{kd}\t{sem}");
            let e = error(&source, profile);
            let d = diagnostic(&e);
            assert_eq!(d.code(), Some("E115"));
            assert_eq!(d.cause(), &ProgramCause::OutputSemanticsCoverage {
                cell: "C".into(), semantics: "S".into(), codomain: "K".into(),
                repeated: repeated.iter().map(|v| v.to_string()).collect(),
                missing: missing.iter().map(|v| v.to_string()).collect(),
                extra: extra.iter().map(|v| v.to_string()).collect(),
            });
            assert_eq!(d.contexts().len(), 3);
            context(d, 0, 0, &source, cd, profile);
            context(d, 1, 0, &source, &sem, profile);
            context(d, 2, 0, &source, kd, profile);
            let prose = format!("{:?}", d.explanations());
            for secret in ["reservado", "secreto", "dato"] { assert!(!prose.contains(secret)); }
        }
    }
    let standalone = error("semántica_de_salida S { A -> \"a\"; A -> \"b\"; }", SourceProfile::Es);
    assert_eq!(diagnostic(&standalone).cause(), &ProgramCause::RepeatedOutputKeys {
        object: "S".into(), keys: vec!["A".into()] });
}

#[test]
fn referencias_cruzadas_conservan_contextos_por_unidad_y_orden() {
    let defs = "codominio K = { A, B }; semántica_de_salida S { A -> \"a\"; }";
    let cell = "cellspec C { b: 3; codomain: K; semantics: S; role: Base; }";
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(defs, "igual.svp", SourceProfile::Es), SourceUnit::new(cell, "igual.svp", SourceProfile::En)];
        if reversed { units.reverse(); }
        let e = compile_svp_assembly(&units).unwrap_err();
        let d = diagnostic(&e);
        assert_eq!(d.contexts().len(), 3);
        context(d, 0, if reversed {0} else {1}, cell, cell, SourceProfile::En);
        context(d, 1, if reversed {1} else {0}, defs, "semántica_de_salida S { A -> \"a\"; }", SourceProfile::Es);
        context(d, 2, if reversed {1} else {0}, defs, "codominio K = { A, B };", SourceProfile::Es);
        assert_ne!(d.contexts()[0].source_sha256(), d.contexts()[1].source_sha256());
        assert_eq!(d.explanations(), vec![
            (SourceProfile::Es, "La semántica de salida \"S\" de la celda \"C\" no corresponde al codominio \"K\": repetidas=[]; ausentes=[\"B\"]; ajenas=[].".into()),
            (SourceProfile::En, "Output semantics \"S\" of cell \"C\" does not match codomain \"K\": repeated=[]; missing=[\"B\"]; extra=[].".into()),
        ]);
        let complete = defs.replace("A -> \"a\";", "A -> \"a\"; B -> \"b\";");
        assert!(compile_svp_assembly(&[SourceUnit::new(&complete, "igual.svp", SourceProfile::Es), SourceUnit::new(cell, "igual.svp", SourceProfile::En)]).is_ok());
    }
}

#[test]
fn colision_conserva_ambas_declaraciones_aunque_coincida_el_archivo() {
    let a = "-- á\r\ncodominio K = { A };";
    let b = "\tcodomain K = { B };";
    for reversed in [false, true] {
        let mut units = vec![SourceUnit::new(a, "igual.svp", SourceProfile::Es), SourceUnit::new(b, "igual.svp", SourceProfile::En)];
        if reversed { units.reverse(); }
        let e = compile_svp_assembly(&units).unwrap_err();
        let d = diagnostic(&e);
        assert_eq!(d.code(), None);
        assert_eq!(d.cause(), &ProgramCause::DuplicateDeclaration { name: "K".into() });
        assert_eq!(d.contexts().len(), 2);
        for i in 0..2 {
            let u = units[i];
            let decl = if u.profile() == SourceProfile::Es { "codominio K = { A };" } else { "codomain K = { B };" };
            context(d, i, i, u.source(), decl, u.profile());
        }
        assert_eq!(d.explanations(), vec![
            (SourceProfile::Es, "El identificador \"K\" tiene más de una declaración.".into()),
            (SourceProfile::En, "Identifier \"K\" has more than one declaration.".into()),
        ]);
    }
}

#[test]
fn colision_objeto_operacion_y_operacion_operacion_conserva_precedencia() {
    for source in ["let K = evaluate(Ausente); codomain K = { A };", "let K = evaluate(Ausente); let K = evaluate(Otro);"] {
        let e = error(source, SourceProfile::En);
        let d = diagnostic(&e);
        assert_eq!(d.cause(), &ProgramCause::DuplicateDeclaration { name: "K".into() });
        assert_eq!(d.contexts().len(), 2);
        let ranges: Vec<_> = d.contexts().iter().map(|c| c.byte_range().unwrap()).collect();
        assert_ne!(ranges[0], ranges[1]);
        assert_eq!(format!("{e:?}"), "InvalidProgram(\"identificador duplicado: K\")");
    }
}

#[test]
fn fallo_local_no_atribuye_defecto_a_unidad_sana_y_huella_es_original() {
    let valid = "codomain Otro = { B };";
    let bad = "-- á\r\n\tcodominio K = { };";
    let e = compile_svp_assembly(&[SourceUnit::new(valid, "igual.svp", SourceProfile::En), SourceUnit::new(bad, "igual.svp", SourceProfile::Es)]).unwrap_err();
    let d = diagnostic(&e);
    assert_eq!(d.contexts().len(), 1);
    context(d, 0, 1, bad, "codominio K = { };", SourceProfile::Es);
    assert_eq!(d.explanations().iter().map(|p| p.0).collect::<Vec<_>>(), vec![SourceProfile::Es, SourceProfile::En]);
    let changed = error(&format!("{bad} "), SourceProfile::Es);
    assert_ne!(d.contexts()[0].source_sha256(), diagnostic(&changed).contexts()[0].source_sha256());
}

#[test]
fn base_y_dominios_cerrados_se_localizan_sin_crear_codigos() {
    for (source, profile, cause) in [
        ("semantic_relation R { kind: Ajena; }", SourceProfile::En, "ClosedDomainValue"),
        ("relación_semántica R { clase: Ajena; }", SourceProfile::Es, "ClosedDomainValue"),
        ("pattern P { kind: Ajeno; arity: 1; }", SourceProfile::En, "ClosedDomainValue"),
        ("patrón P { clase: Ajeno; aridad: 1; }", SourceProfile::Es, "ClosedDomainValue"),
        ("graph G { nodes: []; edges: []; relation: R; regime: Ajeno; }", SourceProfile::En, "ClosedDomainValue"),
        ("grafo G { nodos: []; aristas: []; relación: R; régimen: Ajeno; }", SourceProfile::Es, "ClosedDomainValue"),
        ("cellspec C { b: 2; codomain: K; semantics: S; role: Base; }", SourceProfile::En, "CellBaseTooSmall"),
        ("especificación_de_celda C { b: 2; codominio: K; semántica: S; rol: Base; }", SourceProfile::Es, "CellBaseTooSmall"),
    ] {
        let e = error(source, profile);
        let d = diagnostic(&e);
        assert_eq!(d.cause_key(), cause);
        assert_eq!(d.code(), None);
        assert_eq!(d.explanations().len(), 1);
        assert_eq!(d.explanations()[0].0, profile);
        assert!(!d.explanations()[0].1.contains("__SVP_"));
        context(d, 0, 0, source, source, profile);
    }
}

#[test]
fn cobertura_no_se_inventa_para_causas_pendientes() {
    let e = error("cellspec C { b: 3; codomain: Ausente; semantics: S; role: Base; }", SourceProfile::En);
    assert!(e.program_diagnostic().is_none());
    assert_eq!(format!("{e:?}"), "InvalidProgram(\"referencia no declarada: Ausente\")");
}
