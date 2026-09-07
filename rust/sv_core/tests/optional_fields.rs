//! Grammar 0.1 §§5.4–5.5 retained by 0.2: optional fields are single and ordered.
use sv_core::{compile_svp_profile, CompileError, IrObjectKind, SourceProfile};

#[test]
fn malformed_declarations_are_rejected_without_a_consumer() {
    for source in [
        include_str!("../../../tests/conformance/invalid/semantic_relation_table_repetida.svp"),
        include_str!("../../../tests/conformance/invalid/semantic_relation_constraints_repetida.svp"),
        include_str!("../../../tests/conformance/invalid/pattern_arity_repetida.svp"),
        include_str!("../../../tests/conformance/invalid/pattern_constraints_repetida.svp"),
        include_str!("../../../tests/conformance/invalid/semantic_relation_campos_invertidos.svp"),
        include_str!("../../../tests/conformance/invalid/pattern_campos_invertidos.svp"),
    ] {
        assert!(matches!(compile_svp_profile(source, "optional.svp", SourceProfile::En), Err(CompileError::Frontend(_))));
    }
}

#[test]
fn optional_absence_and_list_occurrences_are_preserved() {
    let program = compile_svp_profile(
        "semantic_relation R { kind: DeclaredRelation; constraints: [B,A,B]; }\n\
         pattern P { kind: DeclaredPattern; constraints: []; }\n\
         pattern Q { kind: DeclaredPattern; }",
        "optional.svp", SourceProfile::En,
    ).unwrap();
    match program.objects()[0].kind() {
        IrObjectKind::SemanticRelation { table, constraints, .. } => {
            assert_eq!(table, &None);
            assert_eq!(constraints.as_ref().unwrap(), &["B", "A", "B"]);
        }
        other => panic!("unexpected object {other:?}"),
    }
    match program.objects()[1].kind() {
        IrObjectKind::Pattern { arity, constraints, .. } => {
            assert_eq!(arity, &None);
            assert_eq!(constraints, &Some(vec![]));
        }
        other => panic!("unexpected object {other:?}"),
    }
    match program.objects()[2].kind() {
        IrObjectKind::Pattern { arity, constraints, .. } => {
            assert_eq!(arity, &None);
            assert_eq!(constraints, &None);
        }
        other => panic!("unexpected object {other:?}"),
    }
}
