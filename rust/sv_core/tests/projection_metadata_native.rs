use sv_core::{compile_svp_profile, equivalence_json, SourceProfile};

const SOURCE: &str = r#"
codomain K = { A };
output_semantics S { A -> "a"; }
cellspec C { b: 3; codomain: K; semantics: S; role: Base; }
"#;

fn occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}

#[test]
fn proyeccion_emite_una_vez_version_fichero_y_huella_reales() {
    let program = compile_svp_profile(SOURCE, "caso.svp", SourceProfile::En).unwrap();
    let json = equivalence_json(&program);

    assert_eq!(occurrences(&json, "\"serializer_version\":"), 1);
    assert_eq!(occurrences(&json, "\"source_file\":"), 1);
    assert_eq!(occurrences(&json, "\"source_sha256\":"), 1);
    assert!(json.contains("\"serializer_version\":\"0.1.0\""));
    assert!(json.contains("\"source_file\":\"caso.svp\""));
    assert!(json.contains(&format!(
        "\"source_sha256\":\"{}\"",
        program.source_sha256()
    )));
    assert!(json.ends_with(&format!(
        "\"serializer_version\":\"0.1.0\",\"source_file\":\"caso.svp\",\"source_sha256\":\"{}\"}}",
        program.source_sha256()
    )));
}

#[test]
fn source_file_modifica_la_proyeccion_sin_alterar_la_huella_del_contenido() {
    let first = compile_svp_profile(SOURCE, "primero.svp", SourceProfile::En).unwrap();
    let second = compile_svp_profile(SOURCE, "segundo.svp", SourceProfile::En).unwrap();

    assert_eq!(first.source_sha256(), second.source_sha256());
    let first_json = equivalence_json(&first);
    let second_json = equivalence_json(&second);
    assert_ne!(first_json, second_json);
    assert!(first_json.contains("\"source_file\":\"primero.svp\""));
    assert!(second_json.contains("\"source_file\":\"segundo.svp\""));
}

#[test]
fn bytes_fuente_distintos_modifican_la_huella_y_su_emision() {
    let first = compile_svp_profile(SOURCE, "caso.svp", SourceProfile::En).unwrap();
    let second_source = format!("{SOURCE}\n-- comentario constitutivamente inocuo\n");
    let second = compile_svp_profile(&second_source, "caso.svp", SourceProfile::En).unwrap();

    assert_ne!(first.source_sha256(), second.source_sha256());
    let first_json = equivalence_json(&first);
    let second_json = equivalence_json(&second);
    assert_ne!(first_json, second_json);
    assert!(first_json.contains(first.source_sha256()));
    assert!(second_json.contains(second.source_sha256()));
}
