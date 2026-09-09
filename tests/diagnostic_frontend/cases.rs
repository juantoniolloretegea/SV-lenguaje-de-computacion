// Esperados comprometidos antes de la realización RETP-113.
// Fuentes sintéticas; ninguna constituye un dominio profesional.
use sv_core::{compile_svp_assembly, compile_svp_profile, CompileError, SourceProfile, SourceUnit};

fn rejected(source: &str, profile: SourceProfile) -> sv_core::FrontendError {
    match compile_svp_profile(source, "unidad.svp", profile) {
        Err(CompileError::Frontend(error)) => error,
        other => panic!("se esperaba rechazo frontal: {other:?}"),
    }
}

#[test]
fn eof_preserves_original_utf8_bytes_and_selected_language() {
    let cases = [
        ("codomain K = { A }", SourceProfile::En, "UnexpectedEnd", "The source ended before the current construct was complete."),
        ("codominio K = { A }", SourceProfile::Es, "UnexpectedEnd", "La fuente terminó antes de completar la construcción actual."),
        ("-- prólogo\r\n\tcodominio Niño = { A }", SourceProfile::Es, "UnexpectedEnd", "La fuente terminó antes de completar la construcción actual."),
        ("codominio K = { A }\n\t", SourceProfile::Es, "UnexpectedEnd", "La fuente terminó antes de completar la construcción actual."),
    ];
    for (source, profile, cause, prose) in cases {
        let error = rejected(source, profile);
        let d = error.diagnostic();
        assert_eq!(d.cause_key(), cause);
        assert_eq!(d.code(), None);
        assert_eq!(d.context().byte_range(), Some((source.len(), source.len())));
        assert_eq!(d.context().unit_index(), 0);
        assert_eq!(d.context().source_file(), "unidad.svp");
        assert_eq!(d.context().profile(), profile);
        assert_eq!(d.explanations(), vec![(profile, prose.to_owned())]);
        assert_eq!(format!("{error:?}"), "UnexpectedEnd");
    }
}

#[test]
fn lexical_character_and_foreign_surface_keep_original_ranges() {
    let source = "\t§";
    let error = rejected(source, SourceProfile::Es);
    let d = error.diagnostic();
    assert_eq!(d.cause_key(), "InvalidLexicalCharacter");
    assert_eq!(d.context().byte_range(), Some((1, 3)));
    assert_eq!(d.explanations(), vec![(SourceProfile::Es, "Carácter léxico no admitido: U+00A7.".to_owned())]);
    let source = "\r\ncodomain K = { A };";
    let error = rejected(source, SourceProfile::Es);
    let d = error.diagnostic();
    assert_eq!(d.cause_key(), "ForeignSurface");
    assert_eq!(d.context().byte_range(), Some((2, 10)));
    assert_eq!(d.explanations(), vec![(SourceProfile::Es, "La grafía \"codomain\" pertenece al otro perfil fuente.".to_owned())]);
    assert!(!d.explanations()[0].1.contains("__SVP_FOREIGN_SURFACE__"));
}

#[test]
fn mixed_assembly_attributes_only_the_faulty_unit_even_with_duplicate_file_names() {
    let valid_en = "codomain A = { X };";
    let invalid_es = "codominio B = { Y }";
    for bad_index in [0, 1] {
        let good = SourceUnit::new(valid_en, "igual.svp", SourceProfile::En);
        let bad = SourceUnit::new(invalid_es, "igual.svp", SourceProfile::Es);
        let units = if bad_index == 0 { [bad, good] } else { [good, bad] };
        let Err(CompileError::Frontend(error)) = compile_svp_assembly(&units) else { panic!("faltó rechazo"); };
        let d = error.diagnostic();
        assert_eq!(d.context().unit_index(), bad_index);
        assert_eq!(d.context().profile(), SourceProfile::Es);
        assert_eq!(d.context().source_file(), "igual.svp");
        assert_eq!(d.context().byte_range(), Some((invalid_es.len(), invalid_es.len())));
        assert_eq!(d.explanations(), vec![
            (SourceProfile::Es, "La fuente terminó antes de completar la construcción actual.".to_owned()),
            (SourceProfile::En, "The source ended before the current construct was complete.".to_owned()),
        ]);
    }
}

#[test]
fn source_hash_changes_with_original_bytes_without_normalization() {
    let a = rejected("codomain K = { A }", SourceProfile::En);
    let b = rejected("codomain K = { A }\r\n", SourceProfile::En);
    // Testigos SHA-256 independientes calculados sobre los literales antes del código.
    assert_eq!(a.diagnostic().context().source_sha256(), "4d4d014018e82ee87657fedda1e6b60178fddd7ea3a76ff9a04e5f54ed5a1dc3");
    assert_eq!(b.diagnostic().context().source_sha256(), "c8cf50fb9ef2a5dc5eb7c89454e97e1a25f0acb604492589049b48256f752efc");
}

#[test]
fn valid_foreign_contextual_identifiers_and_professional_literals_are_unchanged() {
    for (source, profile) in [
        ("codomain posición = { APTO };", SourceProfile::En),
        ("codominio position = { APTO };", SourceProfile::Es),
    ] {
        assert!(compile_svp_profile(source, "valida.svp", profile).is_ok());
    }
}

#[test]
fn optional_fields_repetition_and_order_are_distinct() {
    let repeated = "pattern P { kind: DeclaredPattern; arity: 1; arity: 2; }";
    let order = "pattern P { kind: DeclaredPattern; constraints: []; arity: 1; }";
    let a = rejected(repeated, SourceProfile::En);
    let b = rejected(order, SourceProfile::En);
    assert_eq!(a.diagnostic().cause_key(), "RepeatedOptionalField");
    assert_eq!(b.diagnostic().cause_key(), "OptionalFieldOrder");
    assert_eq!(a.diagnostic().context().byte_range(), Some((45, 50)));
    assert_eq!(b.diagnostic().context().byte_range(), Some((52, 57)));
    assert_ne!(a.diagnostic().explanations(), b.diagnostic().explanations());
}

#[test]
fn quoted_text_is_not_copied_into_the_new_explanation() {
    let error = rejected("codomain \"secreto-no-publicable\" = { A };", SourceProfile::En);
    assert_eq!(error.diagnostic().cause_key(), "ExpectedToken");
    let explanation = &error.diagnostic().explanations()[0].1;
    assert!(!explanation.contains("secreto-no-publicable"));
    assert_eq!(error.diagnostic().context().byte_range(), Some((9, 32)));
}
