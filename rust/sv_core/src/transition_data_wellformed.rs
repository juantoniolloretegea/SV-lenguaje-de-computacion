use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use crate::{IrObjectKind, IrProgram};

const DIAGNOSTIC: &str = "E406 (InsufficientTransitionData)";

/// Completa la bienformación local de `TransitionData` después de la validación
/// general del programa.
///
/// La comprobación no ejecuta una transición ni decide su causalidad. Exige
/// que las valoraciones y asignaciones representadas sean funciones parciales
/// deterministas sobre nodos y posiciones ya constituidos en la arquitectura
/// del horizonte referenciado.
pub(crate) fn validate_program(program: &IrProgram) -> Result<(), String> {
    let objects = program
        .objects()
        .iter()
        .map(|object| (object.name(), object.kind()))
        .collect::<BTreeMap<_, _>>();
    let operation_names = program
        .operations()
        .iter()
        .map(|operation| operation.name())
        .collect::<BTreeSet<_>>();

    for object in program.objects() {
        let IrObjectKind::TransitionData {
            horizon_ref,
            events,
            induced_parameters,
            ..
        } = object.kind()
        else {
            continue;
        };

        let architecture = match objects.get(horizon_ref.as_str()).copied() {
            Some(IrObjectKind::Horizon { architecture, .. }) => architecture,
            Some(_) => {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: {horizon_ref}: se esperaba Horizon",
                    object.name()
                ));
            }
            None => {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: referencia de horizonte no declarada: {horizon_ref}",
                    object.name()
                ));
            }
        };

        let graph_nodes = match objects.get(architecture.as_str()).copied() {
            Some(IrObjectKind::CompositionGraph { nodes, .. }) => nodes,
            Some(_) => {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: {architecture}: se esperaba CompositionGraph",
                    object.name()
                ));
            }
            None => {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: arquitectura no declarada: {architecture}",
                    object.name()
                ));
            }
        };

        let mut event_types = BTreeSet::new();
        for (event_type, _) in events {
            if !event_types.insert(event_type.as_str()) {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: tipo de suceso repetido: {event_type}",
                    object.name()
                ));
            }
        }

        let mut induced_targets = BTreeSet::new();
        for (node_ref, position, _) in induced_parameters {
            let cell_ref = match objects.get(node_ref.as_str()).copied() {
                Some(IrObjectKind::CoupledSpec { cell, .. }) => cell,
                Some(_) => {
                    return Err(format!(
                        "{DIAGNOSTIC}: TransitionData {}: {node_ref}: se esperaba CoupledSpec",
                        object.name()
                    ));
                }
                None if operation_names.contains(node_ref.as_str()) => {
                    return Err(format!(
                        "{DIAGNOSTIC}: TransitionData {}: {node_ref}: se esperaba CoupledSpec",
                        object.name()
                    ));
                }
                None => {
                    return Err(format!(
                        "{DIAGNOSTIC}: TransitionData {}: referencia de nodo no declarada: {node_ref}",
                        object.name()
                    ));
                }
            };

            if !graph_nodes.iter().any(|declared| declared == node_ref) {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: nodo {node_ref} ajeno a la arquitectura {architecture}",
                    object.name()
                ));
            }

            let n = match objects.get(cell_ref.as_str()).copied() {
                Some(IrObjectKind::CellSpec { n, .. }) => n,
                Some(_) => {
                    return Err(format!(
                        "{DIAGNOSTIC}: TransitionData {}: {cell_ref}: se esperaba CellSpec",
                        object.name()
                    ));
                }
                None => {
                    return Err(format!(
                        "{DIAGNOSTIC}: TransitionData {}: CellSpec no declarada para el nodo {node_ref}: {cell_ref}",
                        object.name()
                    ));
                }
            };

            if position.as_decimal() == "0"
                || decimal_cmp(position.as_decimal(), n.as_decimal()) == Ordering::Greater
            {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: posición {} fuera de [1, {}] para el nodo {node_ref}",
                    object.name(),
                    position.as_decimal(),
                    n.as_decimal()
                ));
            }

            if !induced_targets.insert((node_ref.as_str(), position.as_decimal())) {
                return Err(format!(
                    "{DIAGNOSTIC}: TransitionData {}: destino inducido repetido: ({node_ref}, {})",
                    object.name(),
                    position.as_decimal()
                ));
            }
        }
    }

    Ok(())
}

fn decimal_cmp(left: &str, right: &str) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| left.as_bytes().cmp(right.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_order_does_not_narrow_unbounded_naturals() {
        assert_eq!(decimal_cmp("9", "10"), Ordering::Less);
        assert_eq!(decimal_cmp("184467440737095516160", "9"), Ordering::Greater);
    }
}
