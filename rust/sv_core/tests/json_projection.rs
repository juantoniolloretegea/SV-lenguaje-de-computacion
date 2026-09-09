//! N0-03: unicidad por mapa, cierre previo a proyección y ensamblaje ES/EN.
use sv_core::{
    compile_svp_assembly, compile_svp_profile, equivalence_json, CompileError, IrObjectKind,
    IrProgram, SourceProfile, SourceUnit,
};

fn reject(result: Result<IrProgram, CompileError>, expected: &str) {
    match result {
        Err(error) if error.legacy_program_message().is_some() => assert_eq!(error.legacy_program_message().unwrap(), expected),
        other => panic!("debía rechazarse antes de exponer una proyección: {other:?}"),
    }
}

#[test]
fn n0_03_rechaza_semantica_no_enlazada_en_es_en() {
    for (word, profile) in [("output_semantics", SourceProfile::En), ("semántica_de_salida", SourceProfile::Es)] {
        for second in ["igual", "distinto"] {
            reject(compile_svp_profile(
                &format!("{word} S {{ A -> \"igual\"; A -> \"{second}\"; }}"), "duplicate.svp", profile,
            ), "E115 (InvalidOutputSemantics): OutputSemantics S: repetidas=[A]");
        }
    }
}

#[test]
fn n0_03_conserva_claves_compartidas_entre_mapas_y_mapa_vacio() {
    for (word, profile) in [("output_semantics", SourceProfile::En), ("semántica_de_salida", SourceProfile::Es)] {
        let source = format!("{word} First {{ B -> \"igual\"; A -> \"igual\"; a -> \"otro\"; }} {word} Second {{ A -> \"dos\"; }} {word} Empty {{}}");
        let program = compile_svp_profile(&source, "maps.svp", profile).unwrap();
        match program.objects()[0].kind() {
            IrObjectKind::OutputSemantics { mappings } => {
                assert_eq!(mappings.len(), 3);
                assert_eq!(mappings[0].0, "B");
                assert_eq!(mappings[1].0, "A");
                assert_eq!(mappings[2].0, "a");
            }
            other => panic!("objeto inesperado: {other:?}"),
        }
        let before = program.clone();
        let json = equivalence_json(&program);
        assert!(json.contains("\"mappings\":{\"A\":\"igual\",\"B\":\"igual\",\"a\":\"otro\"}"));
        assert!(json.contains("\"mappings\":{\"A\":\"dos\"}"));
        assert!(json.contains("\"mappings\":{}"));
        assert_eq!(json, equivalence_json(&program));
        assert_eq!(program, before);
    }
}

#[test]
fn n0_03_ensamblaje_no_rescata_duplicados_sin_celda() {
    let valid = "output_semantics First { A -> \"uno\"; }";
    let duplicate = "semántica_de_salida Second { A -> \"dos\"; A -> \"tres\"; }";
    for reversed in [false, true] {
        let mut units = vec![
            SourceUnit::new(valid, "first.svp", SourceProfile::En),
            SourceUnit::new(duplicate, "second.svp", SourceProfile::Es),
        ];
        if reversed { units.reverse(); }
        reject(compile_svp_assembly(&units), "E115 (InvalidOutputSemantics): OutputSemantics Second: repetidas=[A]");
    }
}

#[test]
fn n0_03_ensamblaje_conserva_ambitos_y_referencias_de_n0_02() {
    let first = "codomain K = { A, B }; output_semantics First { A -> \"uno\"; B -> \"dos\"; }";
    let second = "semántica_de_salida Second { A -> \"otro\"; } especificación_de_celda C { b: 3; codominio: K; semántica: First; rol: Base; }";
    for reversed in [false, true] {
        let mut units = vec![
            SourceUnit::new(first, "first.svp", SourceProfile::En),
            SourceUnit::new(second, "second.svp", SourceProfile::Es),
        ];
        if reversed { units.reverse(); }
        let json = equivalence_json(&compile_svp_assembly(&units).unwrap());
        assert!(json.contains("\"mappings\":{\"A\":\"uno\",\"B\":\"dos\"}"));
        assert!(json.contains("\"mappings\":{\"A\":\"otro\"}"));
    }
}

#[test]
fn n0_03_conserva_guarda_previa_del_otro_mapa_variable() {
    for (source, profile) in [
        ("codomain K = { A }; connector Conn { source_codomain: K; target_position: 1; mapping: { A -> Zero; A -> One; } }", SourceProfile::En),
        ("codominio K = { A }; conector Conn { codominio_de_origen: K; posición_objetivo: 1; correspondencia: { A -> Cero; A -> Uno; } }", SourceProfile::Es),
    ] {
        reject(compile_svp_profile(source, "connector.svp", profile), "Connector Conn: clave duplicada");
    }
}
