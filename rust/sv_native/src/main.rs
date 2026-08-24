use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use sv_core::{
    compile_svp, IrObjectKind, IrOperationKind, IrProgram, IrQueryContext, IrSupervisableTarget,
    Nat, Tri,
};

fn main() -> ExitCode {
    let mut args = env::args_os();
    let _exe = args.next();
    let Some(path) = args.next() else {
        eprintln!("uso: sv-native <archivo.svp>");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("uso: sv-native <archivo.svp>");
        return ExitCode::from(2);
    }

    let path = Path::new(&path);
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("no se pudo leer {}: {error}", path.display());
            return ExitCode::from(2);
        }
    };
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        eprintln!("nombre de archivo no UTF-8");
        return ExitCode::from(2);
    };

    match compile_svp(&source, file_name) {
        Ok(program) => {
            println!("{}", equivalence_json(&program));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("SVP no admitido por el frente R0-7: {error:?}");
            ExitCode::from(1)
        }
    }
}

/// Proyección JSON de equivalencia para R0-7.
///
/// No constituye el serializador canónico completo del Lenguaje. Su única
/// función es comparar el `IrProgram` soberano producido desde el mismo `.svp`
/// con la referencia diferencial vigente.
fn equivalence_json(program: &IrProgram) -> String {
    let objects = program
        .objects()
        .iter()
        .map(object_json)
        .collect::<Vec<_>>()
        .join(",");
    let operations = program
        .operations()
        .iter()
        .map(operation_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"grammar_version\":{},\"ir_version\":{},\"objects\":[{}],\"operations\":[{}],\"serializer_version\":{},\"source_file\":{},\"source_sha256\":{}}}",
        js(program.grammar_version()),
        js(program.ir_version()),
        objects,
        operations,
        js(program.serializer_version()),
        js(program.source_file()),
        js(program.source_sha256()),
    )
}

fn object_json(object: &sv_core::IrObject) -> String {
    format!(
        "{{\"fields\":{},\"level\":{},\"name\":{},\"type\":{}}}",
        fields_json(object.kind()),
        js(object.level().label()),
        js(object.name()),
        js(object.ir_type()),
    )
}

fn fields_json(kind: &IrObjectKind) -> String {
    match kind {
        IrObjectKind::Codomain { values } => format!("{{\"values\":{}}}", strings(values)),
        IrObjectKind::OutputSemantics { mappings } => {
            format!("{{\"mappings\":{}}}", string_map(mappings))
        }
        IrObjectKind::CellSpec {
            b,
            n,
            codomain,
            semantics,
            role,
        } => format!(
            "{{\"b\":{},\"codomain\":{},\"n\":{},\"role\":{},\"semantics\":{}}}",
            nat(b), js(codomain), nat(n), js(role), js(semantics),
        ),
        IrObjectKind::CoupledSpec { cell, bridges } => format!(
            "{{\"bridges\":{},\"cell\":{}}}",
            nats(bridges), js(cell),
        ),
        IrObjectKind::Connector {
            source_codomain,
            target_position,
            mapping,
        } => {
            let mut mapping = mapping.clone();
            mapping.sort_by(|a, b| a.0.cmp(&b.0));
            let body = mapping
                .iter()
                .map(|(key, value)| format!("{}:{}", js(key), js(value.ir_label())))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"mapping\":{{{}}},\"source_codomain\":{},\"target_position\":{}}}",
                body,
                js(source_codomain),
                nat(target_position),
            )
        }
        IrObjectKind::AdmissibilityTable {
            input_codomains,
            output_codomain,
            table,
        } => {
            let table = table
                .iter()
                .map(|(inputs, output)| {
                    format!(
                        "{{\"inputs\":{},\"output\":{}}}",
                        strings(inputs),
                        js(output)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"input_codomains\":{},\"output_codomain\":{},\"table\":[{}]}}",
                strings(input_codomains),
                js(output_codomain),
                table,
            )
        }
        IrObjectKind::CaptureSpec {
            parameter_id,
            observation_domain,
            observation_space,
            failure_symbol,
            mapping,
        } => format!(
            "{{\"failure_symbol\":{},\"mapping\":{},\"observation_domain\":{},\"observation_space\":{},\"parameter_id\":{}}}",
            js(failure_symbol), js(mapping), js(observation_domain), js(observation_space), nat(parameter_id),
        ),
        IrObjectKind::AdmissibilitySpec {
            parameter_id,
            states,
            rule,
        } => {
            let labels = states
                .iter()
                .map(|state| state.label())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{{\"parameter_id\":{},\"rule\":{},\"states\":{}}}",
                nat(parameter_id),
                js(rule),
                js(&format!("{{{labels}}}")),
            )
        }
        IrObjectKind::Ternarizer {
            observation_space,
            partition_zero,
            partition_one,
            partition_u,
            mapping,
        } => format!(
            "{{\"mapping\":{},\"observation_space\":{},\"partition_one\":{},\"partition_u\":{},\"partition_zero\":{}}}",
            js(mapping), js(observation_space), js(partition_one), js(partition_u), js(partition_zero),
        ),
        IrObjectKind::ResSpec {
            context,
            mechanism,
            mapping,
        } => format!(
            "{{\"context\":{},\"mapping\":{},\"mechanism\":{}}}",
            js(context), js(mapping), js(mechanism),
        ),
        IrObjectKind::CellState { spec, vector } => format!(
            "{{\"spec\":{},\"vector\":{}}}",
            js(spec), tris(vector),
        ),
        IrObjectKind::CoupledState {
            spec,
            base_vector,
            updated_vector,
        } => format!(
            "{{\"base_vector\":{},\"spec\":{},\"updated_vector\":{}}}",
            tris(base_vector), js(spec), tris(updated_vector),
        ),
        IrObjectKind::CompositionGraph {
            nodes,
            edges,
            relation,
            regime,
        } => {
            let edges = edges
                .iter()
                .map(|(source, target, position, connector)| {
                    format!(
                        "{{\"connector\":{},\"position\":{},\"source\":{},\"target\":{}}}",
                        js(connector), nat(position), js(source), js(target)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"edges\":[{}],\"nodes\":{},\"regime\":{},\"relation\":{}}}",
                edges, strings(nodes), js(regime), js(relation),
            )
        }
        IrObjectKind::SemanticRelation {
            kind,
            table,
            constraints,
        } => {
            let mut fields = Vec::new();
            if let Some(constraints) = constraints {
                fields.push(format!("\"constraints\":{}", strings(constraints)));
            }
            fields.push(format!("\"kind\":{}", js(kind)));
            if let Some(table) = table {
                fields.push(format!("\"table\":{}", js(table)));
            }
            format!("{{{}}}", fields.join(","))
        }
        IrObjectKind::Pattern {
            kind,
            arity,
            constraints,
        } => {
            let mut fields = Vec::new();
            if let Some(arity) = arity {
                fields.push(format!("\"arity\":{}", nat(arity)));
            }
            if let Some(constraints) = constraints {
                fields.push(format!("\"constraints\":{}", strings(constraints)));
            }
            fields.push(format!("\"kind\":{}", js(kind)));
            format!("{{{}}}", fields.join(","))
        }
        IrObjectKind::Horizon {
            architecture,
            events,
        } => format!(
            "{{\"architecture\":{},\"events\":{}}}",
            js(architecture), strings(events),
        ),
        IrObjectKind::Frame {
            index,
            architecture,
            cell_states,
            eval_results,
            gate_results,
            supervision,
            criticalities,
        } => format!(
            "{{\"architecture\":{},\"cell_states\":{},\"criticalities\":{},\"eval_results\":{},\"gate_results\":{},\"immutable\":true,\"index\":{},\"supervision\":{}}}",
            js(architecture), strings(cell_states), strings(criticalities), strings(eval_results),
            strings(gate_results), nat(index), strings(supervision),
        ),
        IrObjectKind::TransitionData {
            horizon_ref,
            events,
            induced_parameters,
            metadata,
        } => {
            let events = events
                .iter()
                .map(|(event_type, state)| {
                    format!(
                        "{{\"event_type\":{},\"state\":{}}}",
                        js(event_type),
                        js(state.ir_label())
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            let induced = induced_parameters
                .iter()
                .map(|(cell_ref, position, value)| {
                    format!(
                        "{{\"cell_ref\":{},\"position\":{},\"value\":{}}}",
                        js(cell_ref),
                        nat(position),
                        js(value.ir_label())
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            let mut fields = vec![
                format!("\"events\":[{}]", events),
                format!("\"horizon_ref\":{}", js(horizon_ref)),
                format!("\"induced_parameters\":[{}]", induced),
            ];
            if let Some(metadata) = metadata {
                fields.push(format!("\"metadata\":{}", strings(metadata)));
            }
            format!("{{{}}}", fields.join(","))
        }
        IrObjectKind::Trajectory { entries } => {
            let entries = entries
                .iter()
                .map(|(frame, transition)| match transition {
                    Some(transition) => format!(
                        "{{\"frame\":{},\"transition\":{}}}",
                        js(frame), js(transition)
                    ),
                    None => format!("{{\"frame\":{}}}", js(frame)),
                })
                .collect::<Vec<_>>()
                .join(",");
            format!("{{\"append_only\":true,\"entries\":[{}]}}", entries)
        }
        IrObjectKind::Domain {
            parameters,
            interface,
            horizon,
            capture_specs,
            admissibility_specs,
            ternarizers,
            exogeneity_mask,
            silent_u,
            transduction_policy,
            u_policy,
            closure_criterion,
        } => format!(
            "{{\"admissibility_specs\":{},\"capture_specs\":{},\"closure_criterion\":{},\"exogeneity_mask\":{},\"horizon\":{},\"interface\":{},\"parameters\":{},\"silent_u\":{},\"ternarizers\":{},\"transduction_policy\":{},\"u_policy\":{}}}",
            strings(admissibility_specs), strings(capture_specs), js(closure_criterion), js(exogeneity_mask),
            js(horizon), js(interface), strings(parameters), js(silent_u), strings(ternarizers),
            js(transduction_policy), js(u_policy),
        ),
        IrObjectKind::Agent {
            architecture,
            domain,
            query_engine,
        } => format!(
            "{{\"architecture\":{},\"domain\":{},\"query_engine\":{}}}",
            js(architecture), js(domain), js(query_engine),
        ),
        IrObjectKind::QuerySpec {
            query_type,
            scope,
            restrictions,
        } => format!(
            "{{\"query_type\":{},\"restrictions\":{},\"scope\":{}}}",
            js(query_type), strings(restrictions), js(scope),
        ),
    }
}

fn operation_json(operation: &sv_core::IrOperation) -> String {
    format!(
        "{{\"inputs\":{},\"name\":{},\"result_type\":{},\"type\":{}}}",
        inputs_json(operation.kind()),
        js(operation.name()),
        js(operation.result_type()),
        js(operation.op_type()),
    )
}

fn inputs_json(kind: &IrOperationKind) -> String {
    match kind {
        IrOperationKind::Evaluate { state } => format!("{{\"state\":{}}}", js(state)),
        IrOperationKind::Gate {
            eval_results,
            table,
        } => format!(
            "{{\"eval_results\":{},\"table\":{}}}",
            strings(eval_results), js(table),
        ),
        IrOperationKind::Resolve {
            target_state,
            target_position,
            with_spec,
            context_instance,
            mechanism_instance,
        } => format!(
            "{{\"context_instance\":{},\"mechanism_instance\":{},\"target\":{{\"position\":{},\"state\":{}}},\"with_spec\":{}}}",
            js(context_instance), js(mechanism_instance), nat(target_position), js(target_state), js(with_spec),
        ),
        IrOperationKind::Query { spec, by, context } => format!(
            "{{\"by\":{},\"context\":{},\"spec\":{}}}",
            js(by), query_context_json(context), js(spec),
        ),
        IrOperationKind::Supervise { meta_eval, target } => format!(
            "{{\"meta_eval\":{},\"target\":{}}}",
            js(meta_eval), supervision_target_json(target),
        ),
        IrOperationKind::Compose {
            graph,
            relations,
            patterns,
        } => format!(
            "{{\"graph\":{},\"patterns\":{},\"relations\":{}}}",
            js(graph), strings(patterns), strings(relations),
        ),
        IrOperationKind::Projection { source, field } => {
            format!("{{\"field\":{},\"source\":{}}}", js(field), js(source))
        }
    }
}

fn query_context_json(context: &IrQueryContext) -> String {
    match context {
        IrQueryContext::PointEval { reference } => format!(
            "{{\"ref\":{},\"variant\":{}}}",
            js(reference), js(context.variant_label()),
        ),
        IrQueryContext::TrajectoryView { reference } => format!(
            "{{\"ref\":{},\"variant\":{}}}",
            js(reference), js(context.variant_label()),
        ),
        IrQueryContext::FrameComparison { references } => format!(
            "{{\"refs\":{},\"variant\":{}}}",
            strings(references), js(context.variant_label()),
        ),
        IrQueryContext::ArchitectureView {
            architecture,
            cells,
            evals,
            gates,
        } => format!(
            "{{\"architecture\":{},\"cells\":{},\"evals\":{},\"gates\":{},\"variant\":{}}}",
            js(architecture), strings(cells), strings(evals), strings(gates), js(context.variant_label()),
        ),
        IrQueryContext::CoverageReport { references } => format!(
            "{{\"refs\":{},\"variant\":{}}}",
            strings(references), js(context.variant_label()),
        ),
    }
}

fn supervision_target_json(target: &IrSupervisableTarget) -> String {
    match target {
        IrSupervisableTarget::Cell { reference }
        | IrSupervisableTarget::Composed { reference }
        | IrSupervisableTarget::System { reference } => format!(
            "{{\"ref\":{},\"variant\":{}}}",
            js(reference), js(target.variant_label()),
        ),
    }
}

fn string_map(values: &[(String, String)]) -> String {
    let mut values = values.to_vec();
    values.sort_by(|a, b| a.0.cmp(&b.0));
    let body = values
        .iter()
        .map(|(key, value)| format!("{}:{}", js(key), js(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{body}}}")
}

fn nat(value: &Nat) -> &str {
    value.as_decimal()
}

fn nats(values: &[Nat]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| value.as_decimal())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn tris(values: &[Tri]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| js(value.ir_label()))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn strings<T: AsRef<str>>(values: &[T]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| js(value.as_ref()))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn js(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            ch if ch < '\u{20}' => {
                use std::fmt::Write;
                write!(&mut out, "\\u{:04x}", ch as u32).unwrap();
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_escape_is_deterministic() {
        assert_eq!(js("a\"b\\c\n"), "\"a\\\"b\\\\c\\n\"");
    }
}
