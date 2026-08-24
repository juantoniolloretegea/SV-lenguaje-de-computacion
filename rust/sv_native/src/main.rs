use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use sv_core::{compile_svp, IrObjectKind, IrOperationKind, IrProgram, Nat, Tri};

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
/// función es permitir que la prueba diferencial compare el `IrProgram`
/// soberano producido desde el mismo `.svp` con la referencia vigente.
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
            let mut mappings = mappings.clone();
            mappings.sort_by(|a, b| a.0.cmp(&b.0));
            let body = mappings
                .iter()
                .map(|(key, value)| format!("{}:{}", js(key), js(value)))
                .collect::<Vec<_>>()
                .join(",");
            format!("{{\"mappings\":{{{body}}}}}")
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
                nat(parameter_id), js(rule), js(&format!("{{{labels}}}")),
            )
        }
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
        IrObjectKind::CoupledSpec { cell, bridges } => format!(
            "{{\"bridges\":{},\"cell\":{}}}",
            nats(bridges), js(cell),
        ),
        IrObjectKind::CoupledState {
            spec,
            base_vector,
            updated_vector,
        } => format!(
            "{{\"base_vector\":{},\"spec\":{},\"updated_vector\":{}}}",
            tris(base_vector), js(spec), tris(updated_vector),
        ),
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
        IrObjectKind::CompositionGraph {
            nodes,
            edges,
            relation,
            regime,
        } => {
            let edges = edges
                .iter()
                .map(|(source, target, position, connector)| {
                    format!("[{},{},{},{}]", js(source), js(target), nat(position), js(connector))
                })
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"edges\":[{}],\"nodes\":{},\"regime\":{},\"relation\":{}}}",
                edges, strings(nodes), js(regime), js(relation),
            )
        }
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
        other => panic!(
            "la proyección de equivalencia R0-7 aún no cubre {}",
            other.ir_type()
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
        IrOperationKind::Projection { source, field } => {
            format!("{{\"field\":{},\"source\":{}}}", js(field), js(source))
        }
        other => panic!(
            "la proyección de equivalencia R0-7 aún no cubre {}",
            other.op_type()
        ),
    }
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

fn strings(values: &[String]) -> String {
    format!(
        "[{}]",
        values.iter().map(|value| js(value)).collect::<Vec<_>>().join(",")
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
