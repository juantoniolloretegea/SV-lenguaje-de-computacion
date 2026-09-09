use crate::diagnostic_validation::{ClosedDomain, DeclarationId, ProgramCause, ProgramFailure};
use crate::{IrObjectKind, IrProgram};

/// Verifica dominios cerrados heredados por la Gramática canónica 0.2.
///
/// La etapa de perfiles fuente ya ha canonicalizado las grafías constitutivas
/// ES/EN antes de que esta comprobación observe la IR. Por ello la validación
/// se expresa una sola vez sobre las identidades canónicas y no contiene
/// lógica dependiente del idioma de la fuente.
pub(crate) fn validate_closed_domains(program: &IrProgram) -> Result<(), ProgramFailure> {
    for (index, object) in program.objects().iter().enumerate() {
        match object.kind() {
            IrObjectKind::SemanticRelation { kind, .. } => {
                if kind != "DeclaredRelation" {
                    return Err(ProgramFailure::new(format!(
                        "SemanticRelation {}: kind fuera del dominio cerrado de Gramática 0.2: {kind}",
                        object.name()
                    ), ProgramCause::ClosedDomainValue { domain: ClosedDomain::RelationKind,
                        object: object.name().into(), received: kind.clone() }).at(DeclarationId::Object(index)));
                }
            }
            IrObjectKind::Pattern { kind, .. } => {
                if kind != "DeclaredPattern" {
                    return Err(ProgramFailure::new(format!(
                        "Pattern {}: kind fuera del dominio cerrado de Gramática 0.2: {kind}",
                        object.name()
                    ), ProgramCause::ClosedDomainValue { domain: ClosedDomain::PatternKind,
                        object: object.name().into(), received: kind.clone() }).at(DeclarationId::Object(index)));
                }
            }
            IrObjectKind::CompositionGraph { regime, .. } => {
                if !matches!(regime.as_str(), "Simple" | "General") {
                    return Err(ProgramFailure::new(format!(
                        "CompositionGraph {}: regime fuera del dominio cerrado de Gramática 0.2: {regime}",
                        object.name()
                    ), ProgramCause::ClosedDomainValue { domain: ClosedDomain::GraphRegime,
                        object: object.name().into(), received: regime.clone() }).at(DeclarationId::Object(index)));
                }
            }
            _ => {}
        }
    }
    Ok(())
}
