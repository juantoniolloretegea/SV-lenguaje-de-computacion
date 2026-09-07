//! J-B0 / J1.2: sources must preserve a BridgeSet, never silently repair it.
use sv_core::{compile_svp_profile, CompileError, IrObjectKind, SourceProfile};

const BASE: &str = "codomain K = { A }; output_semantics S { A -> \"a\"; } cellspec C { b: 3; codomain: K; semantics: S; role: Base; }";

#[test]
fn duplicate_nat_positions_are_rejected_before_ir_admission() {
    for bridges in ["3,3", "03,3", "9,1,9"] {
        let source = format!("{BASE} coupledspec CC {{ cell: C; bridges: [{bridges}]; }}");
        let expected = if bridges == "9,1,9" { "9" } else { "3" };
        match compile_svp_profile(&source, "duplicate.svp", SourceProfile::En) {
            Err(CompileError::InvalidProgram(message)) => assert_eq!(message, format!("CoupledSpec CC: posición puente repetida: {expected}")),
            other => panic!("debe rechazar repetición: {other:?}"),
        }
    }
}

#[test]
fn empty_set_and_declared_order_are_preserved() {
    for positions in [vec![], vec!["9", "1", "3"]] {
        let source = format!("{BASE} coupledspec CC {{ cell: C; bridges: [{}]; }}", positions.join(","));
        let program = compile_svp_profile(&source, "valid.svp", SourceProfile::En).unwrap();
        match program.objects().iter().find(|o| o.name() == "CC").unwrap().kind() {
            IrObjectKind::CoupledSpec { bridges, .. } => assert_eq!(bridges.iter().map(|n| n.as_decimal()).collect::<Vec<_>>(), positions),
            other => panic!("tipo inesperado: {other:?}"),
        }
    }
}

#[test]
fn existing_range_rejection_keeps_precedence() {
    for bridges in ["0,0", "10,10", "3,3,10"] {
        let source = format!("{BASE} coupledspec CC {{ cell: C; bridges: [{bridges}]; }}");
        match compile_svp_profile(&source, "range.svp", SourceProfile::En) {
            Err(CompileError::InvalidProgram(message)) => assert_eq!(message, "CoupledSpec CC: posición puente fuera de rango"),
            other => panic!("debe conservar rechazo de rango: {other:?}"),
        }
    }
}
