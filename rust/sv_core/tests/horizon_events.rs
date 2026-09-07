//! J-H1: declaration of event types is distinct from instantiated occurrences.
use sv_core::{compile_svp_profile, CompileError, IrObjectKind, SourceProfile};

const BASE: &str = "codomain K = { A }; output_semantics S { A -> \"a\"; } cellspec C { b: 3; codomain: K; semantics: S; role: Base; } coupledspec CC { cell: C; bridges: [3]; } semantic_relation R { kind: DeclaredRelation; constraints: [Local]; } graph G { nodes: [CC]; edges: []; relation: R; regime: Simple; }";

fn reject(source: &str, expected: &str) {
    match compile_svp_profile(source, "horizon.svp", SourceProfile::En) {
        Err(CompileError::InvalidProgram(message)) => assert_eq!(message, expected),
        other => panic!("debe rechazarse antes de admitir IR: {other:?}"),
    }
}

#[test]
fn duplicate_types_are_rejected_in_an_unconsumed_horizon() {
    for events in ["B,B", "B,A,B"] {
        reject(&format!("{BASE} horizon H {{ architecture: G; events: [{events}]; }}"), "Horizon H: tipo de suceso repetido: B");
    }
}

#[test]
fn uniqueness_is_local_and_preserves_declared_order() {
    let program = compile_svp_profile(&format!("horizon H {{ architecture: G; events: [B,A]; }} {BASE} horizon Other {{ architecture: G; events: [B,Aa,AA]; }}"), "order.svp", SourceProfile::En).unwrap();
    let events: Vec<_> = program.objects().iter().filter_map(|o| match o.kind() {
        IrObjectKind::Horizon { events, .. } => Some(events.clone()), _ => None,
    }).collect();
    assert_eq!(events, vec![vec!["B", "A"],vec!["B", "Aa", "AA"]]);
}

#[test]
fn previous_reference_empty_and_membership_diagnostics_remain_first() {
    reject("horizon H { architecture: Missing; events: [B,B]; }", "referencia no declarada: Missing");
    reject(&format!("{BASE} horizon H {{ architecture: K; events: [B,B]; }}"), "K: se esperaba CompositionGraph");
    reject(&format!("{BASE} horizon H {{ architecture: G; events: []; }}"), "Horizon H: definición incompleta");
    reject(&format!("{BASE} horizon H {{ architecture: G; events: [B,B]; }} transition_data TD {{ horizon_ref: H; events: [(Unknown, One)]; induced_parameters: [(C,3,One)]; }}"), "TransitionData TD: suceso fuera del Horizon");
}
