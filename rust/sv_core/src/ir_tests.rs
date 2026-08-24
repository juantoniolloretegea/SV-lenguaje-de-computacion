use crate::ir::construction::{object, operation, program};
use crate::{
    AdmissibilityState, IrLevel, IrObjectKind, IrOperationKind, IrQueryContext,
    IrSupervisableTarget, Nat, Tri,
};

fn n(value: u64) -> Nat {
    Nat::from_u64(value)
}

#[test]
fn program_header_is_fixed_and_order_is_preserved() {
    let objects = vec![
        object(
            "K3",
            IrObjectKind::Codomain {
                values: vec!["APTO".into(), "NO_APTO".into(), "INDETERMINADO".into()],
            },
        ),
        object(
            "S1",
            IrObjectKind::CellState {
                spec: "C1".into(),
                vector: vec![Tri::Zero, Tri::One, Tri::U],
            },
        ),
    ];
    let operations = vec![operation(
        "E1",
        IrOperationKind::Evaluate { state: "S1".into() },
    )];

    let ir = program("caso.svp", "abc123", objects, operations);

    assert_eq!(ir.grammar_version(), "0.2");
    assert_eq!(ir.ir_version(), "0.3");
    assert_eq!(ir.serializer_version(), "0.1.0");
    assert_eq!(ir.source_file(), "caso.svp");
    assert_eq!(ir.source_sha256(), "abc123");
    assert_eq!(ir.objects()[0].name(), "K3");
    assert_eq!(ir.objects()[1].name(), "S1");
    assert_eq!(ir.operations()[0].name(), "E1");
}

#[test]
fn object_type_and_level_are_derived_not_supplied() {
    let cases = vec![
        (object("x", IrObjectKind::Codomain { values: vec![] }), IrLevel::N0, "Codomain"),
        (object("x", IrObjectKind::OutputSemantics { mappings: vec![] }), IrLevel::N0, "OutputSemantics"),
        (object("x", IrObjectKind::CellSpec { b: n(3), n: n(9), codomain: "K".into(), semantics: "S".into(), role: "Base".into() }), IrLevel::N0, "CellSpec"),
        (object("x", IrObjectKind::CoupledSpec { cell: "C".into(), bridges: vec![n(1)] }), IrLevel::N0, "CoupledSpec"),
        (object("x", IrObjectKind::Connector { source_codomain: "K".into(), target_position: n(1), mapping: vec![("APTO".into(), Tri::One)] }), IrLevel::N0, "Connector"),
        (object("x", IrObjectKind::AdmissibilityTable { input_codomains: vec!["K".into()], output_codomain: "KO".into(), table: vec![(vec!["APTO".into()], "APTO".into())] }), IrLevel::N0, "AdmissibilityTable"),
        (object("x", IrObjectKind::CaptureSpec { parameter_id: n(1), observation_domain: "W".into(), observation_space: "O".into(), failure_symbol: "Bottom".into(), mapping: "phi".into() }), IrLevel::N0, "CaptureSpec"),
        (object("x", IrObjectKind::AdmissibilitySpec { parameter_id: n(1), states: [AdmissibilityState::Ok, AdmissibilityState::Degraded, AdmissibilityState::NotAdmitted], rule: "R".into() }), IrLevel::N0, "AdmissibilitySpec"),
        (object("x", IrObjectKind::Ternarizer { observation_space: "O".into(), partition_zero: "B0".into(), partition_one: "B1".into(), partition_u: "BU".into(), mapping: "tau".into() }), IrLevel::N0, "Ternarizer"),
        (object("x", IrObjectKind::ResSpec { context: "X".into(), mechanism: "R".into(), mapping: "m".into() }), IrLevel::N0, "ResSpec"),
        (object("x", IrObjectKind::CellState { spec: "C".into(), vector: vec![Tri::U] }), IrLevel::N1, "CellState"),
        (object("x", IrObjectKind::CoupledState { spec: "C".into(), base_vector: vec![Tri::Zero], updated_vector: vec![Tri::U] }), IrLevel::N1, "CoupledState"),
        (object("x", IrObjectKind::CompositionGraph { nodes: vec![], edges: vec![], relation: "R".into(), regime: "Simple".into() }), IrLevel::N1, "CompositionGraph"),
        (object("x", IrObjectKind::SemanticRelation { kind: "DeclaredRelation".into(), table: None, constraints: None }), IrLevel::N0, "SemanticRelation"),
        (object("x", IrObjectKind::Pattern { kind: "DeclaredPattern".into(), arity: None, constraints: None }), IrLevel::N0, "Pattern"),
        (object("x", IrObjectKind::Horizon { architecture: "A".into(), events: vec![] }), IrLevel::N3, "Horizon"),
        (object("x", IrObjectKind::Frame { index: n(0), architecture: "A".into(), cell_states: vec![], eval_results: vec![], gate_results: vec![], supervision: vec![], criticalities: vec![] }), IrLevel::N3, "Frame"),
        (object("x", IrObjectKind::TransitionData { horizon_ref: "H".into(), events: vec![], induced_parameters: vec![], metadata: None }), IrLevel::N3, "TransitionData"),
        (object("x", IrObjectKind::Trajectory { entries: vec![] }), IrLevel::N3, "Trajectory"),
        (object("x", IrObjectKind::Domain { parameters: vec![], interface: "I".into(), horizon: "H".into(), capture_specs: vec![], admissibility_specs: vec![], ternarizers: vec![], exogeneity_mask: "E".into(), silent_u: "SU".into(), transduction_policy: "T".into(), u_policy: "U".into(), closure_criterion: "C".into() }), IrLevel::N4, "Domain"),
        (object("x", IrObjectKind::Agent { architecture: "A".into(), domain: "D".into(), query_engine: "Q".into() }), IrLevel::N4, "Agent"),
        (object("x", IrObjectKind::QuerySpec { query_type: "Point".into(), scope: "S".into(), restrictions: vec![] }), IrLevel::N4, "QuerySpec"),
    ];

    for (object, level, ir_type) in cases {
        assert_eq!(object.level(), level);
        assert_eq!(object.ir_type(), ir_type);
    }
}

#[test]
fn operation_type_and_result_are_derived_not_supplied() {
    let cases = vec![
        (operation("x", IrOperationKind::Evaluate { state: "S".into() }), "evaluate", "EvalResult"),
        (operation("x", IrOperationKind::Gate { eval_results: vec!["E".into()], table: "T".into() }), "gate", "GateResult"),
        (operation("x", IrOperationKind::Resolve { target_state: "S".into(), target_position: n(1), with_spec: "RS".into(), context_instance: "X".into(), mechanism_instance: "R".into() }), "resolve", "ResolutionRecord"),
        (operation("x", IrOperationKind::Query { spec: "QS".into(), by: "A".into(), context: IrQueryContext::PointEval { reference: "F".into() } }), "query", "QueryResult"),
        (operation("x", IrOperationKind::Supervise { meta_eval: "E".into(), target: IrSupervisableTarget::Cell { reference: "E".into() } }), "supervise", "SupervisionResult"),
        (operation("x", IrOperationKind::Compose { graph: "G".into(), relations: vec![], patterns: vec![] }), "compose", "Architecture"),
        (operation("x", IrOperationKind::Projection { source: "R".into(), field: "resolved_to".into() }), "projection", "Projected"),
    ];

    for (operation, op_type, result_type) in cases {
        assert_eq!(operation.op_type(), op_type);
        assert_eq!(operation.result_type(), result_type);
    }
}

#[test]
fn admissibility_state_order_from_ir_is_preserved() {
    let object = object(
        "AS1",
        IrObjectKind::AdmissibilitySpec {
            parameter_id: n(1),
            states: [
                AdmissibilityState::NotAdmitted,
                AdmissibilityState::Ok,
                AdmissibilityState::Degraded,
            ],
            rule: "ReglaPermutada".into(),
        },
    );

    match object.kind() {
        IrObjectKind::AdmissibilitySpec { states, .. } => {
            assert_eq!(
                states,
                &[
                    AdmissibilityState::NotAdmitted,
                    AdmissibilityState::Ok,
                    AdmissibilityState::Degraded,
                ]
            );
        }
        other => panic!("tipo inesperado: {other:?}"),
    }
}

#[test]
fn frame_ir_preserves_unbounded_index_declared_order_and_immutability() {
    let huge = Nat::from_decimal("184467440737095516160000000000000000000")
        .expect("natural válido");
    let object = object(
        "F0",
        IrObjectKind::Frame {
            index: huge,
            architecture: "A0".into(),
            cell_states: vec!["S2".into(), "S1".into()],
            eval_results: vec!["E2".into(), "E1".into()],
            gate_results: vec!["G1".into()],
            supervision: vec!["SUP1".into()],
            criticalities: vec![],
        },
    );

    assert_eq!(object.kind().frame_is_immutable(), Some(true));
    match object.kind() {
        IrObjectKind::Frame { index, cell_states, eval_results, .. } => {
            assert_eq!(index.as_decimal(), "184467440737095516160000000000000000000");
            assert_eq!(cell_states, &vec!["S2".to_string(), "S1".to_string()]);
            assert_eq!(eval_results, &vec!["E2".to_string(), "E1".to_string()]);
        }
        other => panic!("tipo inesperado: {other:?}"),
    }
}

#[test]
fn trajectory_append_only_is_constitutive_in_the_representation() {
    let object = object(
        "T0",
        IrObjectKind::Trajectory {
            entries: vec![("F0".into(), Some("TR0".into())), ("F1".into(), None)],
        },
    );
    assert_eq!(object.kind().trajectory_is_append_only(), Some(true));
}

#[test]
fn resolve_operation_preserves_identified_target_and_instances() {
    let operation = operation(
        "RR1",
        IrOperationKind::Resolve {
            target_state: "S1".into(),
            target_position: n(3),
            with_spec: "RS1".into(),
            context_instance: "ContextoClinico".into(),
            mechanism_instance: "RevisionExperto".into(),
        },
    );

    match operation.kind() {
        IrOperationKind::Resolve { target_state, target_position, with_spec, context_instance, mechanism_instance } => {
            assert_eq!(target_state, "S1");
            assert_eq!(target_position.as_decimal(), "3");
            assert_eq!(with_spec, "RS1");
            assert_eq!(context_instance, "ContextoClinico");
            assert_eq!(mechanism_instance, "RevisionExperto");
        }
        other => panic!("operación inesperada: {other:?}"),
    }
}

#[test]
fn query_and_supervision_variants_keep_canonical_labels() {
    assert_eq!(
        IrQueryContext::FrameComparison { references: ["F0".into(), "F1".into()] }.variant_label(),
        "FrameComparison"
    );
    assert_eq!(
        IrSupervisableTarget::System { reference: "A0".into() }.variant_label(),
        "SystemTarget"
    );
}

#[test]
fn tri_has_distinct_language_and_ir_labels() {
    assert_eq!(Tri::Zero.label(), "0");
    assert_eq!(Tri::One.label(), "1");
    assert_eq!(Tri::U.label(), "U");

    assert_eq!(Tri::Zero.ir_label(), "Zero");
    assert_eq!(Tri::One.ir_label(), "One");
    assert_eq!(Tri::U.ir_label(), "U");
}
