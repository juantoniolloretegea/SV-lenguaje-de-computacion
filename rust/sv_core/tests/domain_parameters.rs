//! J-D0 checks nominal identity without inventing a (cell, position) binding.
use sv_core::{compile_svp_profile, CompileError, IrObjectKind, SourceProfile};

const DUPLICATE: &str = include_str!("../../../tests/conformance/invalid/domain_parametro_nominal_repetido.svp");

fn compile(source: &str) -> Result<sv_core::IrProgram, CompileError> {
    compile_svp_profile(source, "domain.svp", SourceProfile::En)
}

#[test]
fn unconsumed_domain_rejects_repeated_nominal_identity() {
    match compile(DUPLICATE) {
        Err(CompileError::InvalidProgram(message)) => assert_eq!(message, "Domain D: parámetro nominal repetido: B"),
        other => panic!("expected controlled rejection, got {other:?}"),
    }
}

#[test]
fn exact_names_and_order_are_preserved_without_numeric_binding() {
    let source = DUPLICATE.replace("parameters: [B,A,B]", "parameters: [B,A,Aa,AA]");
    let program = compile(&source).unwrap();
    let parameters = program.objects().iter().find_map(|object| match object.kind() {
        IrObjectKind::Domain { parameters, .. } => Some(parameters), _ => None,
    }).unwrap();
    assert_eq!(parameters, &["B", "A", "Aa", "AA"]);
}

#[test]
fn previous_chain_and_reference_rejections_remain_first() {
    for (source, expected) in [
        (DUPLICATE.replace("capture_specs: [Cap]", "capture_specs: [Missing]"), "referencia no declarada: Missing"),
        (DUPLICATE.replace("capture_specs: [Cap]", "capture_specs: []"), "Domain D: cadenas de constitución vacías"),
        (DUPLICATE.replace("architecture: G; events: [Ev]", "architecture: K; events: [Ev]"), "K: se esperaba CompositionGraph"),
    ] {
        match compile(&source) {
            Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected),
            other => panic!("expected previous diagnostic, got {other:?}"),
        }
    }
}
