//! Coherencia local de contexto, según el contrato de fila 7 de 08/09/2026.
//! No ejecuta consultas ni acredita cobertura, permisos o causalidad entre marcos.

use std::collections::BTreeMap;
use crate::{IrObjectKind, IrOperationKind, IrProgram, IrQueryContext};

struct Context<'a> {
    objects: BTreeMap<&'a str, &'a IrObjectKind>,
    operations: BTreeMap<&'a str, &'a IrOperationKind>,
}

impl<'a> Context<'a> {
    fn object(&self, name: &str) -> Result<&'a IrObjectKind, String> {
        self.objects.get(name).copied().ok_or_else(|| format!("Contexto: objeto no declarado: {name}"))
    }

    fn operation(&self, name: &str) -> Result<&'a IrOperationKind, String> {
        self.operations.get(name).copied().ok_or_else(|| format!("Contexto: operación no declarada: {name}"))
    }

    fn frame_architecture(&self, name: &str) -> Result<&'a str, String> {
        match self.object(name)? {
            IrObjectKind::Frame { architecture, .. } => Ok(architecture),
            _ => Err(format!("Contexto: {name}: se esperaba Frame")),
        }
    }

    fn trajectory_architecture(&self, name: &str) -> Result<&'a str, String> {
        match self.object(name)? {
            IrObjectKind::Trajectory { entries } => match entries.first() {
                Some((frame, _)) => self.frame_architecture(frame),
                None => Err(format!("Contexto: Trajectory {name}: vacía")),
            },
            _ => Err(format!("Contexto: {name}: se esperaba Trajectory")),
        }
    }

    fn graph_nodes(&self, name: &str) -> Result<&'a [String], String> {
        match self.object(name)? {
            IrObjectKind::CompositionGraph { nodes, .. } => Ok(nodes),
            _ => Err(format!("Contexto: {name}: se esperaba CompositionGraph")),
        }
    }

    fn cell_in_graph(&self, cell: &str, nodes: &[String]) -> Result<bool, String> {
        for node in nodes {
            if let IrObjectKind::CoupledSpec { cell: declared, .. } = self.object(node)? {
                if declared == cell { return Ok(true); }
            }
        }
        Ok(false)
    }

    fn evaluation_in_graph(&self, evaluation: &str, nodes: &[String]) -> Result<bool, String> {
        let state = match self.operation(evaluation)? {
            IrOperationKind::Evaluate { state } => state,
            _ => return Err(format!("Contexto: {evaluation}: se esperaba EvalResult")),
        };
        match self.object(state)? {
            IrObjectKind::CoupledState { spec, .. } => Ok(nodes.iter().any(|node| node == spec)),
            IrObjectKind::CellState { spec, .. } => self.cell_in_graph(spec, nodes),
            _ => Err(format!("Contexto: {state}: fuente no evaluable")),
        }
    }

    fn query_frame(&self, query: &str, agent: &str, frame: &str, architecture: &str) -> Result<(), String> {
        let actual = self.frame_architecture(frame)?;
        if actual != architecture {
            return Err(format!("Query {query}: Frame {frame}: arquitectura {actual} ajena al Agent {agent} ({architecture})"));
        }
        Ok(())
    }
}

pub(crate) fn validate_program(program: &IrProgram) -> Result<(), String> {
    let context = Context {
        objects: program.objects().iter().map(|o| (o.name(), o.kind())).collect(),
        operations: program.operations().iter().map(|o| (o.name(), o.kind())).collect(),
    };

    // La alternancia, las referencias y la bienformación individual ya se han
    // comprobado. Aquí se conserva la relación entre arquitecturas explícitas.
    for object in program.objects() {
        if let IrObjectKind::Trajectory { entries } = object.kind() {
            let name = object.name();
            let architecture = context.trajectory_architecture(name)?;
            for (frame, transition) in entries {
                let actual = context.frame_architecture(frame)?;
                if actual != architecture {
                    return Err(format!("Trajectory {name}: Frame {frame}: arquitectura {actual} distinta de {architecture}"));
                }
                if let Some(transition) = transition {
                    if let IrObjectKind::TransitionData { horizon_ref, .. } = context.object(transition)? {
                        if let IrObjectKind::Horizon { architecture: actual, .. } = context.object(horizon_ref)? {
                            if actual != architecture {
                                return Err(format!("Trajectory {name}: TransitionData {transition}: horizonte {horizon_ref} de {actual} distinto de {architecture}"));
                            }
                        }
                    }
                }
            }
        }
    }

    for operation in program.operations() {
        let (by, query_context) = match operation.kind() {
            IrOperationKind::Query { by, context, .. } => (by, context),
            _ => continue,
        };
        let name = operation.name();
        let (architecture, domain) = match context.object(by)? {
            IrObjectKind::Agent { architecture, domain, .. } => (architecture, domain),
            _ => return Err(format!("Query {name}: {by}: se esperaba Agent")),
        };
        let nodes = context.graph_nodes(architecture)?;
        match query_context {
            IrQueryContext::PointEval { reference } => context.query_frame(name, by, reference, architecture)?,
            IrQueryContext::FrameComparison { references } => {
                for frame in references { context.query_frame(name, by, frame, architecture)?; }
            }
            IrQueryContext::TrajectoryView { reference } => {
                let actual = context.trajectory_architecture(reference)?;
                if actual != architecture {
                    return Err(format!("Query {name}: Trajectory {reference}: arquitectura {actual} ajena al Agent {by} ({architecture})"));
                }
            }
            IrQueryContext::ArchitectureView { cells, evals, gates, .. } => {
                for cell in cells {
                    if !context.cell_in_graph(cell, nodes)? {
                        return Err(format!("Query {name}: CellSpec {cell} ajena a la arquitectura {architecture}"));
                    }
                }
                for evaluation in evals {
                    if !context.evaluation_in_graph(evaluation, nodes)? {
                        return Err(format!("Query {name}: EvalResult {evaluation} ajeno a la arquitectura {architecture}"));
                    }
                }
                for gate in gates {
                    let inputs = match context.operation(gate)? {
                        IrOperationKind::Gate { eval_results, .. } => eval_results,
                        _ => return Err(format!("Query {name}: {gate}: se esperaba GateResult")),
                    };
                    for evaluation in inputs {
                        if !context.evaluation_in_graph(evaluation, nodes)? {
                            return Err(format!("Query {name}: GateResult {gate}: EvalResult {evaluation} ajeno a la arquitectura {architecture}"));
                        }
                    }
                }
            }
            IrQueryContext::CoverageReport { references } => {
                let (interface, silent_u) = match context.object(domain)? {
                    IrObjectKind::Domain { interface, silent_u, .. } => (interface, silent_u),
                    _ => return Err(format!("Query {name}: {domain}: se esperaba Domain")),
                };
                if &references[1] != interface {
                    return Err(format!("Query {name}: CoverageReport: interface {} distinta de Domain {domain} ({interface})", references[1]));
                }
                if &references[2] != silent_u {
                    return Err(format!("Query {name}: CoverageReport: silent_u {} distinta de Domain {domain} ({silent_u})", references[2]));
                }
            }
        }
    }
    Ok(())
}
