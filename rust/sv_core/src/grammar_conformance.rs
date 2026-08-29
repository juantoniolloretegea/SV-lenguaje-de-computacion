use crate::{IrObjectKind, IrProgram};

/// Verifica dominios cerrados heredados por la Gramática canónica 0.2.
///
/// La etapa de perfiles fuente ya ha canonicalizado las grafías constitutivas
/// ES/EN antes de que esta comprobación observe la IR. Por ello la validación
/// se expresa una sola vez sobre las identidades canónicas y no contiene
/// lógica dependiente del idioma de la fuente.
pub(crate) fn validate_closed_domains(program: &IrProgram) -> Result<(), String> {
    for object in program.objects() {
        match object.kind() {
            IrObjectKind::SemanticRelation { kind, .. } => {
                if kind != "DeclaredRelation" {
                    return Err(format!(
                        "SemanticRelation {}: kind fuera del dominio cerrado de Gramática 0.2: {kind}",
                        object.name()
                    ));
                }
            }
            IrObjectKind::Pattern { kind, .. } => {
                if kind != "DeclaredPattern" {
                    return Err(format!(
                        "Pattern {}: kind fuera del dominio cerrado de Gramática 0.2: {kind}",
                        object.name()
                    ));
                }
            }
            IrObjectKind::CompositionGraph { regime, .. } => {
                if !matches!(regime.as_str(), "Simple" | "General") {
                    return Err(format!(
                        "CompositionGraph {}: regime fuera del dominio cerrado de Gramática 0.2: {regime}",
                        object.name()
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(())
}
