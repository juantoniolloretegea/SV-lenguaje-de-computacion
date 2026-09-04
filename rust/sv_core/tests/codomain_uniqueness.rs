use sv_core::{
    compile_svp_assembly, compile_svp_profile, CompileError, SourceProfile, SourceUnit,
};

fn assert_invalid_codomain(source: &str, profile: SourceProfile, member: &str) {
    match compile_svp_profile(source, "codomain-duplicate.svp", profile) {
        Err(CompileError::InvalidProgram(message)) => {
            assert!(message.contains("E004"), "diagnóstico inesperado: {message}");
            assert!(
                message.contains("InvalidCodomain"),
                "diagnóstico inesperado: {message}"
            );
            assert!(message.contains(member), "diagnóstico inesperado: {message}");
        }
        other => panic!("el codominio duplicado debía rechazarse: {other:?}"),
    }
}

#[test]
fn n0_01_rechaza_miembro_duplicado_en_ambos_perfiles() {
    assert_invalid_codomain(
        "codomain K = { A, B, A };",
        SourceProfile::En,
        "A",
    );
    assert_invalid_codomain(
        "codominio K = { A, B, A };",
        SourceProfile::Es,
        "A",
    );
}

#[test]
fn n0_01_conserva_codominios_distintos_en_ambos_perfiles() {
    assert!(compile_svp_profile(
        "codomain K = { A, B, C };",
        "codomain-valid-en.svp",
        SourceProfile::En,
    )
    .is_ok());
    assert!(compile_svp_profile(
        "codominio K = { A, B, C };",
        "codomain-valid-es.svp",
        SourceProfile::Es,
    )
    .is_ok());
}

#[test]
fn n0_01_el_ensamblaje_no_rescata_un_codominio_duplicado() {
    let valid = "codomain K1 = { A, B, C };";
    let duplicate = "codominio K2 = { X, Y, X };";
    match compile_svp_assembly(&[
        SourceUnit::new(valid, "valid-en.svp", SourceProfile::En),
        SourceUnit::new(duplicate, "duplicate-es.svp", SourceProfile::Es),
    ]) {
        Err(CompileError::InvalidProgram(message)) => {
            assert!(message.contains("E004"), "diagnóstico inesperado: {message}");
            assert!(message.contains("K2"), "diagnóstico inesperado: {message}");
            assert!(message.contains("X"), "diagnóstico inesperado: {message}");
        }
        other => panic!("el ensamblaje debía conservar el rechazo: {other:?}"),
    }
}
