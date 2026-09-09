//! Diagnóstico de validación/1. Causa emitida en la guarda y procedencia
//! paralela a la IR. La prosa no interviene en el juicio de admisión.
use crate::{DiagnosticContext, IrProgram, Nat, SourceProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosedDomain { RelationKind, PatternKind, GraphRegime }

impl ClosedDomain {
    pub fn ir_type(self) -> &'static str {
        match self { Self::RelationKind => "SemanticRelation", Self::PatternKind => "Pattern", Self::GraphRegime => "CompositionGraph" }
    }
    pub fn field(self) -> &'static str {
        match self { Self::RelationKind | Self::PatternKind => "kind", Self::GraphRegime => "regime" }
    }
    pub fn expected(self) -> &'static [&'static str] {
        match self { Self::RelationKind => &["DeclaredRelation"], Self::PatternKind => &["DeclaredPattern"], Self::GraphRegime => &["Simple", "General"] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramCause {
    ClosedDomainValue { domain: ClosedDomain, object: String, received: String },
    DuplicateDeclaration { name: String },
    EmptyCodomain { object: String },
    RepeatedCodomainValues { object: String, values: Vec<String> },
    CellBaseTooSmall { object: String, received: Nat },
    RepeatedOutputKeys { object: String, keys: Vec<String> },
    OutputSemanticsCoverage { cell: String, semantics: String, codomain: String,
        repeated: Vec<String>, missing: Vec<String>, extra: Vec<String> },
}

impl ProgramCause {
    pub fn key(&self) -> &'static str {
        match self {
            Self::ClosedDomainValue { .. } => "ClosedDomainValue",
            Self::DuplicateDeclaration { .. } => "DuplicateDeclaration",
            Self::EmptyCodomain { .. } => "EmptyCodomain",
            Self::RepeatedCodomainValues { .. } => "RepeatedCodomainValues",
            Self::CellBaseTooSmall { .. } => "CellBaseTooSmall",
            Self::RepeatedOutputKeys { .. } => "RepeatedOutputKeys",
            Self::OutputSemanticsCoverage { .. } => "OutputSemanticsCoverage",
        }
    }
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::EmptyCodomain { .. } | Self::RepeatedCodomainValues { .. } => Some("E004"),
            Self::RepeatedOutputKeys { .. } | Self::OutputSemanticsCoverage { .. } => Some("E115"),
            _ => None,
        }
    }
    fn explain(&self, profile: SourceProfile) -> String {
        let es = profile == SourceProfile::Es;
        match self {
            Self::ClosedDomainValue { domain, object, received } => {
                let (kind, field, expected) = (domain.ir_type(), domain.field(), domain.expected());
                if es { format!("{kind} {object:?}: valor {received:?} no admitido en {field}; valores canónicos permitidos: {expected:?}.") }
                else { format!("{kind} {object:?}: value {received:?} is not allowed in {field}; allowed canonical values: {expected:?}.") }
            }
            Self::DuplicateDeclaration { name } => if es {
                format!("El identificador {name:?} tiene más de una declaración.")
            } else { format!("Identifier {name:?} has more than one declaration.") },
            Self::EmptyCodomain { object } => if es {
                format!("El codominio {object:?} no contiene valores.")
            } else { format!("Codomain {object:?} contains no values.") },
            Self::RepeatedCodomainValues { object, values } => if es {
                format!("El codominio {object:?} repite los valores {values:?}.")
            } else { format!("Codomain {object:?} repeats values {values:?}.") },
            Self::CellBaseTooSmall { object, received } => {
                let value = received.as_decimal();
                if es { format!("La especificación de celda {object:?} declara b={value}; b debe ser mayor o igual que 3.") }
                else { format!("Cell specification {object:?} declares b={value}; b must be greater than or equal to 3.") }
            }
            Self::RepeatedOutputKeys { object, keys } => if es {
                format!("La semántica de salida {object:?} repite las claves {keys:?}.")
            } else { format!("Output semantics {object:?} repeats keys {keys:?}.") },
            Self::OutputSemanticsCoverage { cell, semantics, codomain, repeated, missing, extra } => if es {
                format!("La semántica de salida {semantics:?} de la celda {cell:?} no corresponde al codominio {codomain:?}: repetidas={repeated:?}; ausentes={missing:?}; ajenas={extra:?}.")
            } else { format!("Output semantics {semantics:?} of cell {cell:?} does not match codomain {codomain:?}: repeated={repeated:?}; missing={missing:?}; extra={extra:?}.") },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramDiagnostic {
    cause: ProgramCause,
    contexts: Vec<DiagnosticContext>,
    provenance_complete: bool,
    profiles: Vec<SourceProfile>,
    legacy: String,
}
impl ProgramDiagnostic {
    pub fn version(&self) -> &'static str { "diagnostico-validacion/1" }
    pub fn cause(&self) -> &ProgramCause { &self.cause }
    pub fn cause_key(&self) -> &'static str { self.cause.key() }
    pub fn code(&self) -> Option<&'static str> { self.cause.code() }
    pub fn contexts(&self) -> &[DiagnosticContext] { &self.contexts }
    pub fn provenance_complete(&self) -> bool { self.provenance_complete }
    pub fn explanations(&self) -> Vec<(SourceProfile, String)> {
        self.profiles.iter().map(|p| (*p, self.cause.explain(*p))).collect()
    }
    pub(crate) fn legacy_message(&self) -> &str { &self.legacy }
}

// Los índices identifican declaraciones concretas, incluso antes de resolver
// nombres duplicados. Los nombres relacionados sólo se resuelven después de
// que Symbols haya comprobado la unicidad global.
#[derive(Debug, Clone, Copy)]
pub(crate) enum DeclarationId { Object(usize), Operation(usize) }
#[derive(Debug)]
enum Site { Declaration(DeclarationId), RelatedObject(String) }
#[derive(Default)]
pub(crate) struct Provenance {
    pub(crate) objects: Vec<DiagnosticContext>,
    pub(crate) operations: Vec<DiagnosticContext>,
}
impl Provenance {
    pub(crate) fn append_unit(&mut self, other: Self, index: usize) {
        self.objects.extend(other.objects.into_iter().map(|c| c.in_unit(index)));
        self.operations.extend(other.operations.into_iter().map(|c| c.in_unit(index)));
    }
    fn get(&self, id: DeclarationId) -> Option<&DiagnosticContext> {
        match id { DeclarationId::Object(i) => self.objects.get(i), DeclarationId::Operation(i) => self.operations.get(i) }
    }
}

#[derive(Debug)]
pub(crate) struct ProgramFailure {
    legacy: String,
    cause: Option<ProgramCause>,
    sites: Vec<Site>,
}
impl From<String> for ProgramFailure {
    fn from(legacy: String) -> Self { Self { legacy, cause: None, sites: Vec::new() } }
}
impl ProgramFailure {
    pub(crate) fn new(legacy: String, cause: ProgramCause) -> Self {
        Self { legacy, cause: Some(cause), sites: Vec::new() }
    }
    pub(crate) fn at(mut self, id: DeclarationId) -> Self {
        self.sites.push(Site::Declaration(id)); self
    }
    pub(crate) fn primary(mut self, id: DeclarationId) -> Self {
        self.sites.insert(0, Site::Declaration(id)); self
    }
    pub(crate) fn related(mut self, name: &str) -> Self {
        self.sites.push(Site::RelatedObject(name.into())); self
    }
    pub(crate) fn locate(self, program: &IrProgram, provenance: &Provenance, profiles: &[SourceProfile]) -> crate::CompileError {
        let Some(cause) = self.cause else { return crate::CompileError::InvalidProgram(self.legacy); };
        let mut contexts = Vec::new();
        let mut complete = !self.sites.is_empty();
        for site in self.sites {
            let id = match site {
                Site::Declaration(id) => Some(id),
                Site::RelatedObject(name) => program.objects().iter().position(|o| o.name() == name).map(DeclarationId::Object),
            };
            match id.and_then(|id| provenance.get(id)) {
                Some(context) => contexts.push(context.clone()),
                None => complete = false,
            }
        }
        crate::CompileError::Diagnostic(ProgramDiagnostic {
            cause, contexts, provenance_complete: complete,
            profiles: [SourceProfile::Es, SourceProfile::En].into_iter().filter(|p| profiles.contains(p)).collect(),
            legacy: self.legacy,
        })
    }
}
