//! Diagnóstico frontal/1: datos de la guarda y procedencia original.
//! La presentación no decide admisión ni confiere autoridad. El adaptador Debug
//! conserva temporalmente la salida histórica; nunca se analiza para localizar.

use crate::SourceProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LegacyFrontendError {
    UnexpectedEnd,
    UnexpectedToken(String),
    Unsupported(String),
    InvalidNatural(String),
    InvalidAdmissibilityState(String),
    InvalidTri(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontendExpectation { Word, Identifier, QuotedText, Natural, Arrow }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontendCause {
    UnexpectedEnd,
    InvalidLexicalCharacter(char),
    InvalidTextEncoding,
    ForeignSurface(String),
    UnsupportedForm(String),
    ExpectedToken { expected: FrontendExpectation },
    ExpectedWord { expected: String },
    ExpectedSymbol(char),
    ProtectedIdentifier(String),
    InvalidNatural(String),
    InvalidAdmissibilityState(String),
    AdmissibilityStateCount(usize),
    InvalidTri(String),
    RepeatedOptionalField { ir_type: &'static str, object: String, field: &'static str },
    OptionalFieldOrder { ir_type: &'static str, object: String, field: &'static str },
}

impl FrontendCause {
    pub fn key(&self) -> &'static str {
        match self {
            Self::UnexpectedEnd => "UnexpectedEnd",
            Self::InvalidLexicalCharacter(_) => "InvalidLexicalCharacter",
            Self::InvalidTextEncoding => "InvalidTextEncoding",
            Self::ForeignSurface(_) => "ForeignSurface",
            Self::UnsupportedForm(_) => "UnsupportedForm",
            Self::ExpectedToken { .. } => "ExpectedToken",
            Self::ExpectedWord { .. } => "ExpectedWord",
            Self::ExpectedSymbol(_) => "ExpectedSymbol",
            Self::ProtectedIdentifier(_) => "ProtectedIdentifier",
            Self::InvalidNatural(_) => "InvalidNatural",
            Self::InvalidAdmissibilityState(_) => "InvalidAdmissibilityState",
            Self::AdmissibilityStateCount(_) => "AdmissibilityStateCount",
            Self::InvalidTri(_) => "InvalidTri",
            Self::RepeatedOptionalField { .. } => "RepeatedOptionalField",
            Self::OptionalFieldOrder { .. } => "OptionalFieldOrder",
        }
    }

    fn explain(&self, profile: SourceProfile) -> String {
        let es = profile == SourceProfile::Es;
        match self {
            Self::UnexpectedEnd => if es {
                "La fuente terminó antes de completar la construcción actual.".into()
            } else { "The source ended before the current construct was complete.".into() },
            Self::InvalidLexicalCharacter(ch) => if es {
                format!("Carácter léxico no admitido: U+{:04X}.", *ch as u32)
            } else { format!("Unsupported lexical character: U+{:04X}.", *ch as u32) },
            Self::InvalidTextEncoding => if es {
                "La cadena no tiene codificación UTF-8 válida.".into()
            } else { "The string does not have valid UTF-8 encoding.".into() },
            Self::ForeignSurface(word) => if es {
                format!("La grafía {word:?} pertenece al otro perfil fuente.")
            } else { format!("The spelling {word:?} belongs to the other source profile.") },
            Self::UnsupportedForm(word) => if es {
                format!("Forma no admitida en esta construcción: {word:?}.")
            } else { format!("Unsupported form in this construct: {word:?}.") },
            Self::ExpectedToken { expected } => {
                let (es_name, en_name) = match expected {
                    FrontendExpectation::Word => ("una palabra", "a word"),
                    FrontendExpectation::Identifier => ("un identificador", "an identifier"),
                    FrontendExpectation::QuotedText => ("una cadena entre comillas", "a quoted string"),
                    FrontendExpectation::Natural => ("un número natural", "a natural number"),
                    FrontendExpectation::Arrow => ("la flecha ->", "the arrow ->"),
                };
                if es { format!("Se esperaba {es_name} en esta posición.") }
                else { format!("Expected {en_name} at this position.") }
            }
            Self::ExpectedWord { expected } => if es {
                format!("Se esperaba la palabra canónica {expected:?} en esta posición.")
            } else { format!("Expected the canonical word {expected:?} at this position.") },
            Self::ExpectedSymbol(ch) => if es {
                format!("Se esperaba el símbolo {ch:?} en esta posición.")
            } else { format!("Expected the symbol {ch:?} at this position.") },
            Self::ProtectedIdentifier(word) => if es {
                format!("La palabra protegida {word:?} no puede ocupar esta posición de identificador.")
            } else { format!("The reserved word {word:?} cannot occupy this identifier position.") },
            Self::InvalidNatural(value) => if es {
                format!("Representación de número natural no admitida: {value:?}.")
            } else { format!("Unsupported natural-number representation: {value:?}.") },
            Self::InvalidAdmissibilityState(value) => if es {
                format!("Estado de admisibilidad no admitido: {value:?}.")
            } else { format!("Unsupported admissibility state: {value:?}.") },
            Self::AdmissibilityStateCount(count) => if es {
                format!("La declaración exige tres estados de admisibilidad; contiene {count}.")
            } else { format!("The declaration requires three admissibility states; it contains {count}.") },
            Self::InvalidTri(value) => if es {
                format!("Valor no admitido en Tri: {value:?}.")
            } else { format!("Unsupported value in Tri: {value:?}.") },
            Self::RepeatedOptionalField { ir_type, object, field } => if es {
                format!("{ir_type} {object:?}: campo opcional repetido: {field}.")
            } else { format!("{ir_type} {object:?}: repeated optional field: {field}.") },
            Self::OptionalFieldOrder { ir_type, object, field } => if es {
                format!("{ir_type} {object:?}: campo opcional fuera de orden: {field}.")
            } else { format!("{ir_type} {object:?}: optional field out of order: {field}.") },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticContext {
    unit_index: usize,
    source_file: String,
    profile: SourceProfile,
    source_sha256: String,
    byte_range: Option<(usize, usize)>,
}

impl DiagnosticContext {
    pub fn unit_index(&self) -> usize { self.unit_index }
    pub fn source_file(&self) -> &str { &self.source_file }
    pub fn profile(&self) -> SourceProfile { self.profile }
    pub fn source_sha256(&self) -> &str { &self.source_sha256 }
    pub fn byte_range(&self) -> Option<(usize, usize)> { self.byte_range }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendDiagnostic {
    cause: FrontendCause,
    context: DiagnosticContext,
    profiles: Vec<SourceProfile>,
}

impl FrontendDiagnostic {
    pub fn version(&self) -> &'static str { "diagnostico-frontal/1" }
    pub fn cause(&self) -> &FrontendCause { &self.cause }
    pub fn cause_key(&self) -> &'static str { self.cause.key() }
    // Las variantes históricas de Frontend no emitían códigos E. No se
    // establece correspondencia nueva por parecido con una variante Rust.
    pub fn code(&self) -> Option<&'static str> { None }
    pub fn context(&self) -> &DiagnosticContext { &self.context }
    pub fn explanations(&self) -> Vec<(SourceProfile, String)> {
        self.profiles.iter().map(|p| (*p, self.cause.explain(*p))).collect()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FrontendError {
    legacy: LegacyFrontendError,
    diagnostic: FrontendDiagnostic,
}

impl std::fmt::Debug for FrontendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.legacy, f)
    }
}

impl FrontendError {
    pub fn diagnostic(&self) -> &FrontendDiagnostic { &self.diagnostic }

    pub(crate) fn new(
        legacy: LegacyFrontendError, cause: FrontendCause,
        byte_range: Option<(usize, usize)>, source: &str,
        source_file: &str, profile: SourceProfile,
    ) -> Self {
        Self { legacy, diagnostic: FrontendDiagnostic {
            cause,
            context: DiagnosticContext { unit_index: 0, source_file: source_file.into(),
                profile, source_sha256: crate::frontend::sha256_hex(source.as_bytes()), byte_range },
            profiles: vec![profile],
        } }
    }

    pub(crate) fn in_assembly(mut self, index: usize, profiles: &[SourceProfile]) -> Self {
        self.diagnostic.context.unit_index = index;
        // Orden de presentación fijado, independiente del orden del fallo.
        self.diagnostic.profiles = [SourceProfile::Es, SourceProfile::En].into_iter()
            .filter(|p| profiles.contains(p)).collect();
        self
    }
}
