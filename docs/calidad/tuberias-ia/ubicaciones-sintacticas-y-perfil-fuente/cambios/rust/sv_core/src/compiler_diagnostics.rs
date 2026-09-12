//! Diagnósticos experimentales COMPILER-DIAGNOSTICS/2, externos a IR 0.3.
//! La causa procede del emisor. Los emisores pendientes no se clasifican por texto.
use std::ops::Range;
use crate::{CompileError, FrontendError, IrProgram, SourceProfile, SourceUnit};
use crate::audit_diagnostics::Language;

pub const VERSION: &str = "COMPILER-DIAGNOSTICS/2";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationId { Object(usize), Operation(usize) }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage { Frontend, ClosedDomains, Wellformed, TransitionData, Context, Assembly }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendCause { ForeignSurface, UnexpectedEnd, UnexpectedToken, Unsupported, InvalidNatural, InvalidAdmissibilityState, InvalidTri }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cause {
    EmptyCodomain { name: String },
    DuplicateCodomainMembers { name: String, duplicates: Vec<String> },
    OutputSemanticsKeys { cell: Option<String>, semantics: String, codomain: Option<String>, duplicates: Vec<String>, missing: Vec<String>, extra: Vec<String> },
    DuplicateIdentifier { name: String, first: DeclarationId, second: DeclarationId },
    Frontend(FrontendCause),
    InsufficientAssemblyUnits { actual: usize },
    Unmigrated { stage: Stage },
}
impl Cause {
    pub const fn id(&self) -> &'static str {
        match self {
            Self::EmptyCodomain { .. } => "CD.EMPTY_CODOMAIN",
            Self::DuplicateCodomainMembers { .. } => "CD.DUPLICATE_CODOMAIN_MEMBERS",
            Self::OutputSemanticsKeys { .. } => "CD.OUTPUT_SEMANTICS_KEYS",
            Self::DuplicateIdentifier { .. } => "CD.DUPLICATE_IDENTIFIER",
            Self::Frontend(FrontendCause::ForeignSurface) => "CD.FOREIGN_SURFACE",
            Self::Frontend(FrontendCause::UnexpectedEnd) => "CD.UNEXPECTED_END",
            Self::Frontend(FrontendCause::UnexpectedToken) => "CD.UNEXPECTED_TOKEN",
            Self::Frontend(FrontendCause::Unsupported) => "CD.UNSUPPORTED",
            Self::Frontend(FrontendCause::InvalidNatural) => "CD.INVALID_NATURAL",
            Self::Frontend(FrontendCause::InvalidAdmissibilityState) => "CD.INVALID_ADMISSIBILITY_STATE",
            Self::Frontend(FrontendCause::InvalidTri) => "CD.INVALID_TRI",
            Self::InsufficientAssemblyUnits { .. } => "CD.INSUFFICIENT_ASSEMBLY_UNITS",
            Self::Unmigrated { .. } => "CD.UNMIGRATED",
        }
    }
    pub const fn code(&self) -> Option<&'static str> {
        match self {
            Self::EmptyCodomain { .. } | Self::DuplicateCodomainMembers { .. } => Some("E004"),
            Self::OutputSemanticsKeys { .. } => Some("E115"),
            _ => None,
        }
    }
    /// Plantilla estática: los datos tipados se consultan aparte. No incluye fuentes ni literales.
    pub const fn message(&self, language: Language) -> &'static str {
        let pair = match self {
            Self::EmptyCodomain { .. } => ("El codominio está vacío.", "The codomain is empty."),
            Self::DuplicateCodomainMembers { .. } => ("El codominio contiene miembros repetidos.", "The codomain contains duplicate members."),
            Self::OutputSemanticsKeys { .. } => ("Las claves de la semántica de salida no cumplen la unicidad o la cobertura exigida.", "Output semantics keys do not satisfy the required uniqueness or coverage."),
            Self::DuplicateIdentifier { .. } => ("Dos declaraciones comparten el mismo identificador.", "Two declarations share the same identifier."),
            Self::Frontend(FrontendCause::ForeignSurface) => ("La grafía no está admitida en esta posición bajo el perfil fuente seleccionado.", "The spelling is not allowed at this position under the selected source profile."),
            Self::Frontend(FrontendCause::UnexpectedEnd) => ("La unidad fuente termina antes de completar la construcción.", "The source unit ends before the construct is complete."),
            Self::Frontend(FrontendCause::UnexpectedToken) => ("Se encontró un elemento léxico o sintáctico inesperado.", "An unexpected lexical or syntactic element was encountered."),
            Self::Frontend(FrontendCause::Unsupported) => ("La construcción no está admitida por este perfil y analizador.", "The construct is not supported by this profile and parser."),
            Self::Frontend(FrontendCause::InvalidNatural) => ("El valor no es un natural admitido.", "The value is not an accepted natural number."),
            Self::Frontend(FrontendCause::InvalidAdmissibilityState) => ("El estado de admisibilidad no es válido.", "The admissibility state is invalid."),
            Self::Frontend(FrontendCause::InvalidTri) => ("El valor ternario no es válido.", "The ternary value is invalid."),
            Self::InsufficientAssemblyUnits { .. } => ("El ensamblaje exige al menos dos unidades fuente.", "Assembly requires at least two source units."),
            Self::Unmigrated { .. } => ("El emisor aún no dispone de causa estructurada; consulte el error heredado bajo su política de acceso.", "The emitter has no structured cause yet; consult the legacy error under its access policy."),
        };
        match language { Language::Es => pair.0, Language::En => pair.1 }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInfo {
    pub index: usize,
    pub file: String,
    pub profile: SourceProfile,
    pub sha256: String,
    pub byte_len: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub unit: usize,
    pub span: Option<Range<usize>>,
    pub role: SiteRole,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiteRole { Failure, FirstDeclaration, SecondDeclaration, Cell, Semantics, Codomain }
#[derive(Clone, PartialEq, Eq)]
pub struct Report {
    legacy: CompileError,
    cause: Cause,
    stage: Stage,
    sources: Vec<SourceInfo>,
    sites: Vec<Site>,
}
// Debug es deliberadamente seguro por defecto: no expone el payload heredado.
impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Report").field("version", &VERSION).field("cause_id", &self.cause.id())
            .field("stage", &self.stage).field("source_count", &self.sources.len()).field("sites", &self.sites).finish()
    }
}
impl Report {
    pub fn cause(&self) -> &Cause { &self.cause }
    pub fn stage(&self) -> Stage { self.stage }
    pub fn sources(&self) -> &[SourceInfo] { &self.sources }
    pub fn sites(&self) -> &[Site] { &self.sites }
    /// Acceso explícito compatible; puede contener texto suministrado por la fuente.
    pub fn legacy(&self) -> &CompileError { &self.legacy }
    pub fn into_legacy(self) -> CompileError { self.legacy }
    pub fn messages(&self) -> Vec<(Language, &'static str)> {
        let es = self.sources.iter().any(|s| s.profile == SourceProfile::Es);
        let en = self.sources.iter().any(|s| s.profile == SourceProfile::En);
        let mut out = Vec::new();
        if es { out.push((Language::Es,self.cause.message(Language::Es))); }
        if en { out.push((Language::En,self.cause.message(Language::En))); }
        out
    }
}
pub(crate) struct ValidationFailure { pub legacy: String, pub cause: Option<Cause> }
impl ValidationFailure {
    pub(crate) fn known(legacy: String, cause: Cause) -> Self { Self { legacy, cause: Some(cause) } }
}
impl From<String> for ValidationFailure {
    fn from(legacy: String) -> Self { Self { legacy, cause: None } }
}
struct Origins { objects: Vec<Site>, operations: Vec<Site> }
impl Origins {
    fn declaration(&self, id: DeclarationId, role: SiteRole) -> Option<Site> {
        let item = match id { DeclarationId::Object(i) => self.objects.get(i), DeclarationId::Operation(i) => self.operations.get(i) };
        item.map(|s| Site { role, ..s.clone() })
    }
    fn object(&self, p: &IrProgram, name: &str, role: SiteRole) -> Option<Site> {
        p.objects().iter().position(|o| o.name() == name).and_then(|i| self.declaration(DeclarationId::Object(i),role))
    }
    fn sites(&self, p: &IrProgram, cause: &Cause) -> Vec<Site> {
        let mut sites = Vec::new();
        match cause {
            Cause::EmptyCodomain { name } | Cause::DuplicateCodomainMembers { name, .. } => sites.extend(self.object(p,name,SiteRole::Codomain)),
            Cause::DuplicateIdentifier { first, second, .. } => {
                sites.extend(self.declaration(*first,SiteRole::FirstDeclaration));
                sites.extend(self.declaration(*second,SiteRole::SecondDeclaration));
            }
            Cause::OutputSemanticsKeys { cell, semantics, codomain, .. } => {
                if let Some(c) = cell { sites.extend(self.object(p,c,SiteRole::Cell)); }
                sites.extend(self.object(p,semantics,SiteRole::Semantics));
                if let Some(c) = codomain { sites.extend(self.object(p,c,SiteRole::Codomain)); }
            }
            _ => {}
        }
        sites
    }
}
fn info(units: &[SourceUnit<'_>]) -> Vec<SourceInfo> {
    units.iter().enumerate().map(|(index,u)| SourceInfo { index, file: u.source_file().to_owned(), profile: u.profile(),
        sha256: crate::frontend::sha256_hex(u.source().as_bytes()), byte_len: u.source().len() }).collect()
}
fn failure(legacy: String, stage: Stage, sources: &[SourceInfo]) -> Report {
    Report { legacy: CompileError::InvalidProgram(legacy), cause: Cause::Unmigrated { stage }, stage, sources: sources.to_vec(), sites: Vec::new() }
}
fn validate(program: IrProgram, origins: Origins, sources: Vec<SourceInfo>) -> Result<IrProgram, Report> {
    crate::grammar_conformance::validate_closed_domains(&program).map_err(|e| failure(e,Stage::ClosedDomains,&sources))?;
    crate::wellformed::validate_program(&program).map_err(|e| {
        let cause = e.cause.unwrap_or(Cause::Unmigrated { stage: Stage::Wellformed });
        let sites = origins.sites(&program, &cause);
        Report { legacy: CompileError::InvalidProgram(e.legacy), cause, stage: Stage::Wellformed, sources: sources.clone(), sites }
    })?;
    crate::transition_data_wellformed::validate_program(&program).map_err(|e| failure(e,Stage::TransitionData,&sources))?;
    crate::context_wellformed::validate_program(&program).map_err(|e| failure(e,Stage::Context,&sources))?;
    Ok(program)
}
fn parse(unit: SourceUnit<'_>, index: usize, sources: &[SourceInfo]) -> Result<crate::frontend::ParsedUnit, Report> {
    crate::frontend::compile_with_origins(unit.source(),unit.source_file(),unit.profile()).map_err(|e| {
        let cause = if e.foreign_surface { FrontendCause::ForeignSurface } else { match &e.error {
            FrontendError::UnexpectedEnd => FrontendCause::UnexpectedEnd,
            FrontendError::UnexpectedToken(_) => FrontendCause::UnexpectedToken,
            FrontendError::Unsupported(_) => FrontendCause::Unsupported,
            FrontendError::InvalidNatural(_) => FrontendCause::InvalidNatural,
            FrontendError::InvalidAdmissibilityState(_) => FrontendCause::InvalidAdmissibilityState,
            FrontendError::InvalidTri(_) => FrontendCause::InvalidTri,
        } };
        Report { cause: Cause::Frontend(cause), stage: Stage::Frontend, legacy: CompileError::Frontend(e.error),
            sources: sources.to_vec(), sites: vec![Site { unit: index, span: e.span, role: SiteRole::Failure }] }
    })
}
fn origin_list(spans: Vec<Range<usize>>, unit: usize) -> Vec<Site> {
    spans.into_iter().map(|span| Site { unit, span: Some(span), role: SiteRole::Failure }).collect()
}
pub fn compile(source: &str, file: &str, profile: SourceProfile) -> Result<IrProgram, Report> {
    let unit = SourceUnit::new(source,file,profile);
    let sources = info(&[unit]);
    let parsed = parse(unit,0,&sources)?;
    validate(parsed.program, Origins { objects: origin_list(parsed.objects,0), operations: origin_list(parsed.operations,0) }, sources)
}
pub fn compile_assembly(units: &[SourceUnit<'_>]) -> Result<IrProgram, Report> {
    let sources = info(units);
    if units.len() < 2 {
        return Err(Report { legacy: CompileError::InvalidProgram("el ensamblaje multifuente exige al menos dos unidades".to_owned()),
            cause: Cause::InsufficientAssemblyUnits { actual: units.len() }, stage: Stage::Assembly, sources, sites: Vec::new() });
    }
    let mut objects = Vec::new(); let mut operations = Vec::new();
    let mut origins = Origins { objects: Vec::new(), operations: Vec::new() };
    for (index,unit) in units.iter().enumerate() {
        let parsed = parse(*unit,index,&sources)?;
        objects.extend(parsed.program.objects().iter().cloned());
        operations.extend(parsed.program.operations().iter().cloned());
        origins.objects.extend(origin_list(parsed.objects,index));
        origins.operations.extend(origin_list(parsed.operations,index));
    }
    let program = crate::ir::construction::program(format!("@assembly/2b/{}-units",units.len()),crate::assembly_identity(units),objects,operations);
    validate(program,origins,sources)
}
