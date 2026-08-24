use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{
    IrObjectKind, IrOperationKind, IrProgram, IrQueryContext, IrSupervisableTarget, Nat, Tri,
};

#[derive(Clone, Copy)]
enum Symbol<'a> {
    Object(&'a IrObjectKind),
    Operation(&'a IrOperationKind),
}

struct Symbols<'a> {
    by_name: BTreeMap<&'a str, Symbol<'a>>,
}

impl<'a> Symbols<'a> {
    fn new(program: &'a IrProgram) -> Result<Self, String> {
        let mut by_name = BTreeMap::new();
        for object in program.objects() {
            if by_name
                .insert(object.name(), Symbol::Object(object.kind()))
                .is_some()
            {
                return Err(format!("identificador duplicado: {}", object.name()));
            }
        }
        for operation in program.operations() {
            if by_name
                .insert(operation.name(), Symbol::Operation(operation.kind()))
                .is_some()
            {
                return Err(format!("identificador duplicado: {}", operation.name()));
            }
        }
        Ok(Self { by_name })
    }

    fn object(&self, name: &str) -> Result<&'a IrObjectKind, String> {
        match self.by_name.get(name).copied() {
            Some(Symbol::Object(kind)) => Ok(kind),
            Some(Symbol::Operation(_)) => Err(format!("{name} no es un objeto declarado")),
            None => Err(format!("referencia no declarada: {name}")),
        }
    }

    fn operation(&self, name: &str) -> Result<&'a IrOperationKind, String> {
        match self.by_name.get(name).copied() {
            Some(Symbol::Operation(kind)) => Ok(kind),
            Some(Symbol::Object(_)) => Err(format!("{name} no es un resultado de operación")),
            None => Err(format!("referencia no declarada: {name}")),
        }
    }
}

pub(crate) fn validate_program(program: &IrProgram) -> Result<(), String> {
    let symbols = Symbols::new(program)?;

    for object in program.objects() {
        validate_object(object.name(), object.kind(), &symbols)?;
    }
    for operation in program.operations() {
        validate_operation(operation.name(), operation.kind(), &symbols)?;
    }

    Ok(())
}

fn validate_object(name: &str, kind: &IrObjectKind, symbols: &Symbols<'_>) -> Result<(), String> {
    match kind {
        IrObjectKind::Codomain { values } => {
            if values.is_empty() {
                return Err(format!("codomain {name} vacío"));
            }
        }
        IrObjectKind::OutputSemantics { .. } => {}
        IrObjectKind::CellSpec {
            b,
            codomain,
            semantics,
            role,
            ..
        } => {
            if nat_cmp_text(b, "3") == Ordering::Less {
                return Err(format!("CellSpec {name}: b debe ser >= 3"));
            }
            expect_object(symbols, codomain, "Codomain", |kind| {
                matches!(kind, IrObjectKind::Codomain { .. })
            })?;
            expect_object(symbols, semantics, "OutputSemantics", |kind| {
                matches!(kind, IrObjectKind::OutputSemantics { .. })
            })?;
            if !matches!(role.as_str(), "Base" | "Supervisor" | "Composite") {
                return Err(format!("CellSpec {name}: rol no reconocido: {role}"));
            }
        }
        IrObjectKind::CoupledSpec { cell, bridges } => {
            let cell_kind = expect_object(symbols, cell, "CellSpec", |kind| {
                matches!(kind, IrObjectKind::CellSpec { .. })
            })?;
            let n = match cell_kind {
                IrObjectKind::CellSpec { n, .. } => n,
                _ => unreachable!(),
            };
            for position in bridges {
                if nat_is_zero(position) || nat_cmp(position, n) == Ordering::Greater {
                    return Err(format!(
                        "CoupledSpec {name}: posición puente {} fuera de rango",
                        position.as_decimal()
                    ));
                }
            }
        }
        IrObjectKind::Connector {
            source_codomain,
            mapping,
            ..
        } => {
            let codomain = expect_object(symbols, source_codomain, "Codomain", |kind| {
                matches!(kind, IrObjectKind::Codomain { .. })
            })?;
            let values = match codomain {
                IrObjectKind::Codomain { values } => values,
                _ => unreachable!(),
            };
            let expected: BTreeSet<&str> = values.iter().map(String::as_str).collect();
            let mut seen = BTreeSet::new();
            for (key, _) in mapping {
                if !seen.insert(key.as_str()) {
                    return Err(format!("Connector {name}: clave duplicada {key}"));
                }
            }
            if seen != expected {
                return Err(format!("Connector {name}: mapping incompleto o inconsistente"));
            }
        }
        IrObjectKind::AdmissibilityTable {
            input_codomains,
            output_codomain,
            table,
        } => validate_admissibility_table(name, input_codomains, output_codomain, table, symbols)?,
        IrObjectKind::CaptureSpec {
            parameter_id,
            observation_space,
            failure_symbol,
            ..
        } => {
            if nat_is_zero(parameter_id) {
                return Err(format!("CaptureSpec {name}: parameter_id no positivo"));
            }
            if observation_space.is_empty() {
                return Err(format!("CaptureSpec {name}: observation_space vacío"));
            }
            if failure_symbol != "Bottom" {
                return Err(format!("CaptureSpec {name}: failure_symbol debe ser Bottom"));
            }
        }
        IrObjectKind::AdmissibilitySpec {
            parameter_id,
            states,
            rule,
        } => {
            if nat_is_zero(parameter_id) {
                return Err(format!("AdmissibilitySpec {name}: parameter_id no positivo"));
            }
            let labels: BTreeSet<&str> = states.iter().map(|state| state.label()).collect();
            let expected = BTreeSet::from(["Ok", "Degraded", "NotAdmitted"]);
            if labels != expected {
                return Err(format!("AdmissibilitySpec {name}: estados no canónicos"));
            }
            if rule.is_empty() {
                return Err(format!("AdmissibilitySpec {name}: rule vacío"));
            }
        }
        IrObjectKind::Ternarizer {
            observation_space,
            partition_zero,
            partition_one,
            partition_u,
            ..
        } => {
            if observation_space.is_empty()
                || partition_zero.is_empty()
                || partition_one.is_empty()
                || partition_u.is_empty()
            {
                return Err(format!("Ternarizer {name}: definición incompleta"));
            }
        }
        IrObjectKind::ResSpec { .. } => {}
        IrObjectKind::CellState { spec, vector } => {
            let cell = expect_object(symbols, spec, "CellSpec", |kind| {
                matches!(kind, IrObjectKind::CellSpec { .. })
            })?;
            let n = match cell {
                IrObjectKind::CellSpec { n, .. } => n,
                _ => unreachable!(),
            };
            if !nat_eq_usize(n, vector.len()) {
                return Err(format!("CellState {name}: longitud de vector incompatible"));
            }
        }
        IrObjectKind::CoupledState {
            spec,
            base_vector,
            updated_vector,
        } => validate_coupled_state(name, spec, base_vector, updated_vector, symbols)?,
        IrObjectKind::CompositionGraph {
            nodes,
            edges,
            relation,
            regime,
        } => validate_graph(name, nodes, edges, relation, regime, symbols)?,
        IrObjectKind::SemanticRelation { .. } | IrObjectKind::Pattern { .. } => {}
        IrObjectKind::Horizon {
            architecture,
            events,
        } => {
            if architecture.is_empty() || events.is_empty() {
                return Err(format!("Horizon {name}: definición incompleta"));
            }
        }
        IrObjectKind::Frame {
            architecture,
            cell_states,
            eval_results,
            gate_results,
            supervision,
            criticalities,
            ..
        } => validate_frame(
            name,
            architecture,
            cell_states,
            eval_results,
            gate_results,
            supervision,
            criticalities,
            symbols,
        )?,
        IrObjectKind::TransitionData {
            horizon_ref,
            events,
            induced_parameters,
            ..
        } => {
            let horizon = expect_object(symbols, horizon_ref, "Horizon", |kind| {
                matches!(kind, IrObjectKind::Horizon { .. })
            })?;
            let declared_events: BTreeSet<&str> = match horizon {
                IrObjectKind::Horizon { events, .. } => {
                    events.iter().map(String::as_str).collect()
                }
                _ => unreachable!(),
            };
            for (event, _) in events {
                if !declared_events.contains(event.as_str()) {
                    return Err(format!(
                        "TransitionData {name}: suceso {event} fuera del Horizon"
                    ));
                }
            }
            if induced_parameters.is_empty() {
                return Err(format!(
                    "TransitionData {name}: induced_parameters no puede estar vacío"
                ));
            }
        }
        IrObjectKind::Trajectory { entries } => {
            if entries.is_empty() {
                return Err(format!("Trajectory {name}: entries vacío"));
            }
            let last = entries.len() - 1;
            for (index, (frame, transition)) in entries.iter().enumerate() {
                expect_object(symbols, frame, "Frame", |kind| {
                    matches!(kind, IrObjectKind::Frame { .. })
                })?;
                if index < last && transition.is_none() {
                    return Err(format!(
                        "Trajectory {name}: entrada no final sin transición"
                    ));
                }
                if index == last && transition.is_some() {
                    return Err(format!(
                        "Trajectory {name}: última entrada con transición"
                    ));
                }
                if let Some(transition) = transition {
                    expect_object(symbols, transition, "TransitionData", |kind| {
                        matches!(kind, IrObjectKind::TransitionData { .. })
                    })?;
                }
            }
        }
        IrObjectKind::Domain {
            horizon,
            capture_specs,
            admissibility_specs,
            ternarizers,
            ..
        } => validate_domain(
            name,
            horizon,
            capture_specs,
            admissibility_specs,
            ternarizers,
            symbols,
        )?,
        IrObjectKind::Agent {
            architecture,
            domain,
            ..
        } => {
            let domain_kind = expect_object(symbols, domain, "Domain", |kind| {
                matches!(kind, IrObjectKind::Domain { .. })
            })?;
            let horizon_name = match domain_kind {
                IrObjectKind::Domain { horizon, .. } => horizon,
                _ => unreachable!(),
            };
            let horizon = expect_object(symbols, horizon_name, "Horizon", |kind| {
                matches!(kind, IrObjectKind::Horizon { .. })
            })?;
            let domain_architecture = match horizon {
                IrObjectKind::Horizon { architecture, .. } => architecture,
                _ => unreachable!(),
            };
            if architecture != domain_architecture {
                return Err(format!("Agent {name}: architecture incompatible con Domain"));
            }
        }
        IrObjectKind::QuerySpec {
            query_type, scope, ..
        } => {
            if query_type == "PendingU" {
                return Err(format!("QuerySpec {name}: PendingU no habilitado"));
            }
            let expected_scope = match query_type.as_str() {
                "PointEvaluation" => "Cell",
                "TrajectoryState" => "Trajectory",
                "FrameComparison" => "Pair",
                "CoverageState" | "GlobalCriticality" => "Architecture",
                _ => return Err(format!("QuerySpec {name}: query_type no reconocido")),
            };
            if scope != expected_scope {
                return Err(format!("QuerySpec {name}: scope incompatible"));
            }
        }
    }
    Ok(())
}

fn validate_admissibility_table(
    name: &str,
    input_codomains: &[String],
    output_codomain: &str,
    table: &[(Vec<String>, String)],
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let mut domains = Vec::with_capacity(input_codomains.len());
    for codomain in input_codomains {
        let kind = expect_object(symbols, codomain, "Codomain", |kind| {
            matches!(kind, IrObjectKind::Codomain { .. })
        })?;
        let values = match kind {
            IrObjectKind::Codomain { values } => values,
            _ => unreachable!(),
        };
        domains.push(values);
    }
    let out_kind = expect_object(symbols, output_codomain, "Codomain", |kind| {
        matches!(kind, IrObjectKind::Codomain { .. })
    })?;
    let out_values: BTreeSet<&str> = match out_kind {
        IrObjectKind::Codomain { values } => values.iter().map(String::as_str).collect(),
        _ => unreachable!(),
    };

    let expected_count = domains.iter().try_fold(1usize, |acc, values| {
        acc.checked_mul(values.len())
    });
    let Some(expected_count) = expected_count else {
        return Err(format!("AdmissibilityTable {name}: tabla necesariamente incompleta"));
    };

    let domain_sets: Vec<BTreeSet<&str>> = domains
        .iter()
        .map(|values| values.iter().map(String::as_str).collect())
        .collect();
    let mut seen = BTreeSet::new();
    for (inputs, output) in table {
        if inputs.len() != domain_sets.len() {
            return Err(format!("AdmissibilityTable {name}: aridad de fila incompatible"));
        }
        for (index, input) in inputs.iter().enumerate() {
            if !domain_sets[index].contains(input.as_str()) {
                return Err(format!("AdmissibilityTable {name}: entrada fuera de codominio"));
            }
        }
        if !out_values.contains(output.as_str()) {
            return Err(format!("AdmissibilityTable {name}: salida fuera de codominio"));
        }
        if !seen.insert(inputs.clone()) {
            return Err(format!("AdmissibilityTable {name}: fila duplicada"));
        }
    }
    if seen.len() != expected_count {
        return Err(format!("AdmissibilityTable {name}: tabla incompleta"));
    }
    Ok(())
}

fn validate_coupled_state(
    name: &str,
    spec: &str,
    base_vector: &[Tri],
    updated_vector: &[Tri],
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let coupled = expect_object(symbols, spec, "CoupledSpec", |kind| {
        matches!(kind, IrObjectKind::CoupledSpec { .. })
    })?;
    let (cell_name, bridges) = match coupled {
        IrObjectKind::CoupledSpec { cell, bridges } => (cell, bridges),
        _ => unreachable!(),
    };
    let cell = expect_object(symbols, cell_name, "CellSpec", |kind| {
        matches!(kind, IrObjectKind::CellSpec { .. })
    })?;
    let n = match cell {
        IrObjectKind::CellSpec { n, .. } => n,
        _ => unreachable!(),
    };
    if !nat_eq_usize(n, base_vector.len()) || !nat_eq_usize(n, updated_vector.len()) {
        return Err(format!("CoupledState {name}: longitud incompatible"));
    }
    let bridge_set: BTreeSet<&str> = bridges.iter().map(Nat::as_decimal).collect();
    for (index, (before, after)) in base_vector.iter().zip(updated_vector).enumerate() {
        if before != after {
            let position = (index + 1).to_string();
            if !bridge_set.contains(position.as_str()) {
                return Err(format!(
                    "CoupledState {name}: actualización fuera de BridgeSet"
                ));
            }
        }
    }
    Ok(())
}

fn validate_graph(
    name: &str,
    nodes: &[String],
    edges: &[(String, String, Nat, String)],
    relation: &str,
    regime: &str,
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    expect_object(symbols, relation, "SemanticRelation", |kind| {
        matches!(kind, IrObjectKind::SemanticRelation { .. })
    })?;
    for node in nodes {
        expect_object(symbols, node, "CoupledSpec", |kind| {
            matches!(kind, IrObjectKind::CoupledSpec { .. })
        })?;
    }
    let node_set: BTreeSet<&str> = nodes.iter().map(String::as_str).collect();
    let mut simple_targets = BTreeSet::new();

    let mut indegree: BTreeMap<&str, usize> =
        node_set.iter().map(|node| (*node, 0usize)).collect();
    let mut adjacency: BTreeMap<&str, Vec<&str>> =
        node_set.iter().map(|node| (*node, Vec::new())).collect();

    for (source, target, position, connector_name) in edges {
        if !node_set.contains(source.as_str()) || !node_set.contains(target.as_str()) {
            return Err(format!("Graph {name}: arista fuera del conjunto de nodos"));
        }
        let connector = expect_object(symbols, connector_name, "Connector", |kind| {
            matches!(kind, IrObjectKind::Connector { .. })
        })?;
        let source_spec = symbols.object(source)?;
        let target_spec = symbols.object(target)?;
        let (source_cell_name, target_bridges) = match (source_spec, target_spec) {
            (
                IrObjectKind::CoupledSpec { cell, .. },
                IrObjectKind::CoupledSpec { bridges, .. },
            ) => (cell, bridges),
            _ => return Err(format!("Graph {name}: nodo no CoupledSpec")),
        };
        if !target_bridges.iter().any(|bridge| bridge == position) {
            return Err(format!("Graph {name}: posición fuera del BridgeSet destino"));
        }
        let (connector_codomain, connector_position) = match connector {
            IrObjectKind::Connector {
                source_codomain,
                target_position,
                ..
            } => (source_codomain, target_position),
            _ => unreachable!(),
        };
        if connector_position != position {
            return Err(format!("Graph {name}: target_position del connector incompatible"));
        }
        let source_cell = expect_object(symbols, source_cell_name, "CellSpec", |kind| {
            matches!(kind, IrObjectKind::CellSpec { .. })
        })?;
        let source_codomain = match source_cell {
            IrObjectKind::CellSpec { codomain, .. } => codomain,
            _ => unreachable!(),
        };
        if connector_codomain != source_codomain {
            return Err(format!("Graph {name}: source_codomain del connector incompatible"));
        }

        adjacency.get_mut(source.as_str()).unwrap().push(target.as_str());
        *indegree.get_mut(target.as_str()).unwrap() += 1;

        if regime == "Simple"
            && !simple_targets.insert((target.as_str(), position.as_decimal().to_owned()))
        {
            return Err(format!("Graph {name}: concurrencia en régimen Simple"));
        }
    }

    let mut queue: VecDeque<&str> = indegree
        .iter()
        .filter_map(|(node, degree)| (*degree == 0).then_some(*node))
        .collect();
    let mut visited = 0usize;
    while let Some(node) = queue.pop_front() {
        visited += 1;
        if let Some(targets) = adjacency.get(node) {
            for target in targets {
                let degree = indegree.get_mut(target).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(target);
                }
            }
        }
    }
    if visited != node_set.len() {
        return Err(format!("Graph {name}: ciclo detectado"));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_frame(
    name: &str,
    architecture: &str,
    cell_states: &[String],
    eval_results: &[String],
    gate_results: &[String],
    supervision: &[String],
    criticalities: &[String],
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let graph = expect_object(symbols, architecture, "CompositionGraph", |kind| {
        matches!(kind, IrObjectKind::CompositionGraph { .. })
    })?;
    let graph_nodes: BTreeSet<&str> = match graph {
        IrObjectKind::CompositionGraph { nodes, .. } => {
            nodes.iter().map(String::as_str).collect()
        }
        _ => unreachable!(),
    };
    if !criticalities.is_empty() {
        return Err(format!("Frame {name}: criticalities debe permanecer vacío"));
    }

    let mut state_refs = BTreeSet::new();
    let mut state_specs = BTreeSet::new();
    for state_ref in cell_states {
        let state = expect_object(symbols, state_ref, "CoupledState", |kind| {
            matches!(kind, IrObjectKind::CoupledState { .. })
        })?;
        if !state_refs.insert(state_ref.as_str()) {
            return Err(format!("Frame {name}: CoupledState duplicado"));
        }
        let spec = match state {
            IrObjectKind::CoupledState { spec, .. } => spec,
            _ => unreachable!(),
        };
        if !graph_nodes.contains(spec.as_str()) {
            return Err(format!("Frame {name}: estado fuera de arquitectura"));
        }
        if !state_specs.insert(spec.as_str()) {
            return Err(format!("Frame {name}: más de un estado por nodo"));
        }
    }

    let frame_cells: BTreeSet<&str> = cell_states.iter().map(String::as_str).collect();
    let mut eval_sources = BTreeSet::new();
    for eval_ref in eval_results {
        let operation = symbols.operation(eval_ref)?;
        let state = match operation {
            IrOperationKind::Evaluate { state } => state,
            _ => return Err(format!("Frame {name}: eval_results contiene no-Evaluate")),
        };
        if !frame_cells.contains(state.as_str()) {
            return Err(format!("Frame {name}: evaluación externa"));
        }
        if !eval_sources.insert(state.as_str()) {
            return Err(format!("Frame {name}: evaluación duplicada"));
        }
    }

    let frame_evals: BTreeSet<&str> = eval_results.iter().map(String::as_str).collect();
    for gate_ref in gate_results {
        let operation = symbols.operation(gate_ref)?;
        let inputs = match operation {
            IrOperationKind::Gate { eval_results, .. } => eval_results,
            _ => return Err(format!("Frame {name}: gate_results contiene no-Gate")),
        };
        if inputs.iter().any(|input| !frame_evals.contains(input.as_str())) {
            return Err(format!("Frame {name}: Gate depende de evaluación externa"));
        }
    }

    let frame_gates: BTreeSet<&str> = gate_results.iter().map(String::as_str).collect();
    for supervision_ref in supervision {
        let operation = symbols.operation(supervision_ref)?;
        let (meta_eval, target) = match operation {
            IrOperationKind::Supervise { meta_eval, target } => (meta_eval, target),
            _ => return Err(format!("Frame {name}: supervision contiene no-Supervise")),
        };
        if !frame_evals.contains(meta_eval.as_str()) {
            return Err(format!("Frame {name}: meta_eval externo"));
        }
        match target {
            IrSupervisableTarget::Cell { reference } => {
                if !frame_evals.contains(reference.as_str()) {
                    return Err(format!("Frame {name}: CellTarget externo"));
                }
            }
            IrSupervisableTarget::Composed { reference } => {
                if !frame_gates.contains(reference.as_str()) {
                    return Err(format!("Frame {name}: ComposedTarget externo"));
                }
            }
            IrSupervisableTarget::System { reference } => {
                if reference != architecture {
                    return Err(format!("Frame {name}: SystemTarget externo"));
                }
            }
        }
    }
    Ok(())
}

fn validate_domain(
    name: &str,
    horizon: &str,
    capture_specs: &[String],
    admissibility_specs: &[String],
    ternarizers: &[String],
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    expect_object(symbols, horizon, "Horizon", |kind| {
        matches!(kind, IrObjectKind::Horizon { .. })
    })?;
    if capture_specs.is_empty() || admissibility_specs.is_empty() || ternarizers.is_empty() {
        return Err(format!("Domain {name}: cadenas de constitución vacías"));
    }

    let mut capture_ids = BTreeSet::new();
    let mut capture_spaces = BTreeSet::new();
    for reference in capture_specs {
        let kind = expect_object(symbols, reference, "CaptureSpec", |kind| {
            matches!(kind, IrObjectKind::CaptureSpec { .. })
        })?;
        if let IrObjectKind::CaptureSpec {
            parameter_id,
            observation_space,
            ..
        } = kind
        {
            capture_ids.insert(parameter_id.as_decimal());
            capture_spaces.insert(observation_space.as_str());
        }
    }

    let mut admissibility_ids = BTreeSet::new();
    for reference in admissibility_specs {
        let kind = expect_object(symbols, reference, "AdmissibilitySpec", |kind| {
            matches!(kind, IrObjectKind::AdmissibilitySpec { .. })
        })?;
        if let IrObjectKind::AdmissibilitySpec { parameter_id, .. } = kind {
            admissibility_ids.insert(parameter_id.as_decimal());
        }
    }
    if capture_ids != admissibility_ids {
        return Err(format!("Domain {name}: parameter_id incompatibles"));
    }

    let mut ternarizer_spaces = BTreeSet::new();
    for reference in ternarizers {
        let kind = expect_object(symbols, reference, "Ternarizer", |kind| {
            matches!(kind, IrObjectKind::Ternarizer { .. })
        })?;
        if let IrObjectKind::Ternarizer {
            observation_space, ..
        } = kind
        {
            ternarizer_spaces.insert(observation_space.as_str());
        }
    }
    if !capture_spaces.is_subset(&ternarizer_spaces) {
        return Err(format!("Domain {name}: observation_space sin Ternarizer"));
    }
    Ok(())
}

fn validate_operation(
    name: &str,
    kind: &IrOperationKind,
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    match kind {
        IrOperationKind::Evaluate { state } => {
            let object = symbols.object(state)?;
            if !matches!(
                object,
                IrObjectKind::CellState { .. } | IrObjectKind::CoupledState { .. }
            ) {
                return Err(format!("Evaluate {name}: fuente no evaluable"));
            }
        }
        IrOperationKind::Gate {
            eval_results,
            table,
        } => validate_gate(name, eval_results, table, symbols)?,
        IrOperationKind::Resolve {
            target_state,
            target_position,
            with_spec,
            context_instance,
            mechanism_instance,
        } => validate_resolve(
            name,
            target_state,
            target_position,
            with_spec,
            context_instance,
            mechanism_instance,
            symbols,
        )?,
        IrOperationKind::Query { spec, by, context } => {
            validate_query(name, spec, by, context, symbols)?
        }
        IrOperationKind::Supervise { meta_eval, target } => {
            validate_supervise(name, meta_eval, target, symbols)?
        }
        IrOperationKind::Compose {
            graph,
            relations,
            patterns,
        } => {
            expect_object(symbols, graph, "CompositionGraph", |kind| {
                matches!(kind, IrObjectKind::CompositionGraph { .. })
            })?;
            if relations.is_empty() {
                return Err(format!("Compose {name}: relations vacío"));
            }
            if patterns.is_empty() {
                return Err(format!("Compose {name}: patterns vacío"));
            }
            for relation in relations {
                expect_object(symbols, relation, "SemanticRelation", |kind| {
                    matches!(kind, IrObjectKind::SemanticRelation { .. })
                })?;
            }
            for pattern in patterns {
                expect_object(symbols, pattern, "Pattern", |kind| {
                    matches!(kind, IrObjectKind::Pattern { .. })
                })?;
            }
        }
        IrOperationKind::Projection { source, field } => {
            let source_kind = symbols.operation(source)?;
            let allowed: &[&str] = match source_kind {
                IrOperationKind::Evaluate { .. } => &[
                    "source_state",
                    "counts",
                    "threshold",
                    "classification",
                    "criticality",
                    "deltas",
                ],
                IrOperationKind::Gate { .. } => &["inputs", "table", "output"],
                IrOperationKind::Resolve { .. } => &[
                    "target",
                    "previous",
                    "reviewed_to",
                    "resolved_to",
                    "context_ref",
                    "mechanism_ref",
                ],
                IrOperationKind::Query { .. } => &["response", "justification", "metadata"],
                IrOperationKind::Supervise { .. } => &["meta_eval", "target", "verdict"],
                IrOperationKind::Compose { .. } | IrOperationKind::Projection { .. } => {
                    return Err(format!("Projection {name}: fuente no proyectable"));
                }
            };
            if !allowed.contains(&field.as_str()) {
                return Err(format!("Projection {name}: campo inexistente"));
            }
        }
    }
    Ok(())
}

fn validate_gate(
    name: &str,
    eval_results: &[String],
    table: &str,
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let table_kind = expect_object(symbols, table, "AdmissibilityTable", |kind| {
        matches!(kind, IrObjectKind::AdmissibilityTable { .. })
    })?;
    let expected = match table_kind {
        IrObjectKind::AdmissibilityTable {
            input_codomains, ..
        } => input_codomains,
        _ => unreachable!(),
    };
    if eval_results.len() != expected.len() {
        return Err(format!("Gate {name}: número de entradas incompatible"));
    }
    let mut actual = Vec::with_capacity(eval_results.len());
    for input in eval_results {
        let operation = symbols.operation(input)?;
        if !matches!(operation, IrOperationKind::Evaluate { .. }) {
            return Err(format!("Gate {name}: entrada no EvalResult"));
        }
        actual.push(
            codomain_of_eval(operation, symbols)
                .ok_or_else(|| format!("Gate {name}: no se pudo resolver codominio"))?,
        );
    }
    if actual
        .iter()
        .map(|value| value.as_str())
        .ne(expected.iter().map(String::as_str))
    {
        return Err(format!("Gate {name}: codominios posicionales incompatibles"));
    }
    Ok(())
}

fn codomain_of_eval(kind: &IrOperationKind, symbols: &Symbols<'_>) -> Option<String> {
    let state_name = match kind {
        IrOperationKind::Evaluate { state } => state,
        _ => return None,
    };
    match symbols.object(state_name).ok()? {
        IrObjectKind::CellState { spec, .. } => match symbols.object(spec).ok()? {
            IrObjectKind::CellSpec { codomain, .. } => Some(codomain.clone()),
            _ => None,
        },
        IrObjectKind::CoupledState { spec, .. } => match symbols.object(spec).ok()? {
            IrObjectKind::CoupledSpec { cell, .. } => match symbols.object(cell).ok()? {
                IrObjectKind::CellSpec { codomain, .. } => Some(codomain.clone()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_resolve(
    name: &str,
    target_state: &str,
    target_position: &Nat,
    with_spec: &str,
    context_instance: &str,
    mechanism_instance: &str,
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let res_spec = expect_object(symbols, with_spec, "ResSpec", |kind| {
        matches!(kind, IrObjectKind::ResSpec { .. })
    })?;
    let (expected_context, expected_mechanism) = match res_spec {
        IrObjectKind::ResSpec {
            context, mechanism, ..
        } => (context, mechanism),
        _ => unreachable!(),
    };
    let state = symbols.object(target_state)?;
    let vector = match state {
        IrObjectKind::CellState { vector, .. } => vector,
        IrObjectKind::CoupledState { updated_vector, .. } => updated_vector,
        _ => return Err(format!("Resolve {name}: target no evaluable")),
    };
    let Some(index) = one_based_index(target_position, vector.len()) else {
        return Err(format!("Resolve {name}: posición fuera de rango"));
    };
    if vector[index] != Tri::U {
        return Err(format!("Resolve {name}: target no es U constituida"));
    }
    if context_instance != expected_context || mechanism_instance != expected_mechanism {
        return Err(format!("Resolve {name}: instancia incompatible con ResSpec"));
    }
    Ok(())
}

fn validate_query(
    name: &str,
    spec: &str,
    by: &str,
    context: &IrQueryContext,
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let spec_kind = expect_object(symbols, spec, "QuerySpec", |kind| {
        matches!(kind, IrObjectKind::QuerySpec { .. })
    })?;
    let query_type = match spec_kind {
        IrObjectKind::QuerySpec { query_type, .. } => query_type,
        _ => unreachable!(),
    };
    let agent = expect_object(symbols, by, "Agent", |kind| {
        matches!(kind, IrObjectKind::Agent { .. })
    })?;
    let (agent_architecture, agent_domain) = match agent {
        IrObjectKind::Agent {
            architecture, domain, ..
        } => (architecture, domain),
        _ => unreachable!(),
    };

    let actual_type = match context {
        IrQueryContext::PointEval { reference } => {
            expect_object(symbols, reference, "Frame", |kind| {
                matches!(kind, IrObjectKind::Frame { .. })
            })?;
            "PointEvaluation"
        }
        IrQueryContext::TrajectoryView { reference } => {
            expect_object(symbols, reference, "Trajectory", |kind| {
                matches!(kind, IrObjectKind::Trajectory { .. })
            })?;
            "TrajectoryState"
        }
        IrQueryContext::FrameComparison { references } => {
            for reference in references {
                expect_object(symbols, reference, "Frame", |kind| {
                    matches!(kind, IrObjectKind::Frame { .. })
                })?;
            }
            "FrameComparison"
        }
        IrQueryContext::ArchitectureView {
            architecture,
            cells,
            evals,
            gates,
        } => {
            if architecture != agent_architecture {
                return Err(format!("Query {name}: ArchitectureView fuera del Agent"));
            }
            for reference in cells {
                expect_object(symbols, reference, "CellSpec", |kind| {
                    matches!(kind, IrObjectKind::CellSpec { .. })
                })?;
            }
            for reference in evals {
                if !matches!(symbols.operation(reference)?, IrOperationKind::Evaluate { .. }) {
                    return Err(format!("Query {name}: eval no EvalResult"));
                }
            }
            for reference in gates {
                if !matches!(symbols.operation(reference)?, IrOperationKind::Gate { .. }) {
                    return Err(format!("Query {name}: gate no GateResult"));
                }
            }
            "GlobalCriticality"
        }
        IrQueryContext::CoverageReport { references } => {
            expect_object(symbols, &references[0], "Domain", |kind| {
                matches!(kind, IrObjectKind::Domain { .. })
            })?;
            if &references[0] != agent_domain {
                return Err(format!("Query {name}: CoverageReport fuera del Agent"));
            }
            "CoverageState"
        }
    };
    if query_type != actual_type {
        return Err(format!("Query {name}: QueryContext incompatible con QuerySpec"));
    }
    Ok(())
}

fn validate_supervise(
    name: &str,
    meta_eval: &str,
    target: &IrSupervisableTarget,
    symbols: &Symbols<'_>,
) -> Result<(), String> {
    let meta = symbols.operation(meta_eval)?;
    let state_name = match meta {
        IrOperationKind::Evaluate { state } => state,
        _ => return Err(format!("Supervise {name}: meta_eval no EvalResult")),
    };
    match target {
        IrSupervisableTarget::Cell { reference } => {
            if !matches!(symbols.operation(reference)?, IrOperationKind::Evaluate { .. }) {
                return Err(format!("Supervise {name}: CellTarget de tipo incorrecto"));
            }
        }
        IrSupervisableTarget::Composed { reference } => {
            if !matches!(symbols.operation(reference)?, IrOperationKind::Gate { .. }) {
                return Err(format!("Supervise {name}: ComposedTarget de tipo incorrecto"));
            }
        }
        IrSupervisableTarget::System { reference } => {
            expect_object(symbols, reference, "CompositionGraph", |kind| {
                matches!(kind, IrObjectKind::CompositionGraph { .. })
            })?;
        }
    }

    let role = role_of_state(state_name, symbols)
        .ok_or_else(|| format!("Supervise {name}: no se pudo resolver rol de meta_eval"))?;
    if role != "Supervisor" {
        return Err(format!("Supervise {name}: meta_eval no procede de Supervisor"));
    }
    Ok(())
}

fn role_of_state<'a>(state_name: &str, symbols: &'a Symbols<'a>) -> Option<&'a str> {
    match symbols.object(state_name).ok()? {
        IrObjectKind::CellState { spec, .. } => match symbols.object(spec).ok()? {
            IrObjectKind::CellSpec { role, .. } => Some(role.as_str()),
            _ => None,
        },
        IrObjectKind::CoupledState { spec, .. } => match symbols.object(spec).ok()? {
            IrObjectKind::CoupledSpec { cell, .. } => match symbols.object(cell).ok()? {
                IrObjectKind::CellSpec { role, .. } => Some(role.as_str()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn expect_object<'a, F>(
    symbols: &'a Symbols<'a>,
    name: &str,
    expected: &str,
    predicate: F,
) -> Result<&'a IrObjectKind, String>
where
    F: FnOnce(&IrObjectKind) -> bool,
{
    let kind = symbols.object(name)?;
    if predicate(kind) {
        Ok(kind)
    } else {
        Err(format!("{name}: se esperaba {expected}"))
    }
}

fn nat_is_zero(value: &Nat) -> bool {
    value.as_decimal() == "0"
}

fn nat_cmp(left: &Nat, right: &Nat) -> Ordering {
    decimal_cmp(left.as_decimal(), right.as_decimal())
}

fn nat_cmp_text(left: &Nat, right: &str) -> Ordering {
    decimal_cmp(left.as_decimal(), right)
}

fn decimal_cmp(left: &str, right: &str) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| left.as_bytes().cmp(right.as_bytes()))
}

fn nat_eq_usize(value: &Nat, expected: usize) -> bool {
    value.as_decimal() == expected.to_string()
}

fn one_based_index(position: &Nat, len: usize) -> Option<usize> {
    if nat_is_zero(position) || nat_cmp_text(position, &len.to_string()) == Ordering::Greater {
        return None;
    }
    let one_based = position.as_decimal().parse::<usize>().ok()?;
    one_based.checked_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_order_does_not_narrow_unbounded_nat() {
        assert_eq!(decimal_cmp("9", "10"), Ordering::Less);
        assert_eq!(decimal_cmp("184467440737095516160", "3"), Ordering::Greater);
    }
}
