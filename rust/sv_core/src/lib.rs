//! Núcleo soberano del Lenguaje SV.
//!
//! R0 materializa la semántica compartida por los destinos nativo y
//! WebAssembly. R1 añade, sobre el mismo `sv_core`, las fronteras de control
//! necesarias para autoridad, mediación y decisiones protegidas. Los tipos de
//! control no constituyen una segunda semántica y no alteran `Tri`, la gramática
//! o la IR canónica.
//!
//! Desde R1-5 las entradas productivas de decisión, mediación y ejecución deben
//! atravesar la envolvente trazada. Las funciones crudas de R1-4 permanecen
//! internas al núcleo y no forman parte de la API pública ordinaria:
//!
//! ```compile_fail
//! use sv_core::decide_permit;
//! ```
//!
//! ```compile_fail
//! use sv_core::mediate_permit;
//! ```
//!
//! ```compile_fail
//! use sv_core::execute_mediated;
//! ```
//!
//! Tampoco puede alcanzarse la función cruda mediante un módulo interno:
//!
//! ```compile_fail
//! use sv_core::permission::decide_permit;
//! ```

pub mod admissibility;
pub mod authority;
pub mod control;
pub mod decision_trace;
mod equivalence;
mod execution;
mod frontend;
mod identifier_profile;
pub mod frame;
pub mod ir;
mod mediation;
pub mod nat;
mod permission;
pub mod requirements;
mod requirements_bridge;
pub mod requirements_conflict;
pub mod requirements_coverage;
pub mod requirements_reuse;
pub mod resolution;
mod wellformed;

pub use admissibility::{
    AdmissibilitySpec, AdmissibilityState, CaptureOutcome, InvalidAdmissibilitySpec,
    ADMISSIBILITY_DIAGNOSTIC_CODE,
};
pub use authority::{
    AccumulationContract, ConstitutedAuthority, EffectDescriptor, EffectEnvelope, FormDescriptor,
    GovernedDomain,
};
pub use control::{
    AccumulationRuleRef, AdmittedEvidenceRef, ApplicabilityRuleRef, AuthorityHolderRef,
    AuthorityRef, CheckResult, ConflictResolutionRuleRef, ConstitutedFactRef, ContextRef,
    ContinuityOccupancy, ControlId, CoverageRuleRef, EffectFamilyRef, EffectRef, EnablementRef,
    ExerciseRef, FormRef, GovernedObjectRef, InformationRef, InvalidControlId, RequirementRef,
    ReuseBindingKeyRef, ReuseBindingValueRef, ReuseRuleRef, TransitionClass, VerifierFamilyRef,
    VerifierRef,
};
pub use decision_trace::{
    decide_permit_traced, execute_traced_mediated, mediate_traced_permit, ConflictRuleTrace,
    CoverageRuleTrace, DecisionTrace, DecisionTraceRef, IndividualCheckTrace,
    ProtectedDecisionContinuity, RequirementDecisionTrace, ReuseRuleTrace, TraceAssemblyError,
    TracedAdapterError, TracedBlockedDecision, TracedDecisionError, TracedExecutionError,
    TracedExerciseConfirmation, TracedMediatedCommitment, TracedMediationError, TracedPermit,
    TracedPermitDecision, TracedPermitDisposition,
};
pub use equivalence::equivalence_json;
pub use execution::{
    EffectExecutor, ExecutionContinuity, ExecutionError, ExecutionRequest, ExerciseAttemptState,
    ExerciseConfirmation, ExerciseTraceEntry,
};
pub use frontend::{FrontendError, SourceProfile};
pub use frame::{Frame, FrameClosureViolation, FRAME_CLOSURE_DIAGNOSTIC_CODE};
pub use ir::{
    IrLevel, IrObject, IrObjectKind, IrOperation, IrOperationKind, IrProgram, IrQueryContext,
    IrSupervisableTarget,
};
pub use mediation::{MediatedEffectCommitment, MediationError};
pub use nat::{InvalidNat, Nat};
pub use permission::{Permit, PermitDecision, PermitDecisionError, PermitRejection};
pub use requirements::{
    CheckFormationError, CoreRequirementKind, InvalidRequirementDescriptor, InvalidRequirementSet,
    RequirementCheck, RequirementClass, RequirementDescriptor, RequirementSet,
    VerifierApplicability,
};
pub use requirements_bridge::{
    resolve_requirement_result, ResolvedAggregationError, ResolvedCheckObservation,
    ResolvedRequirementResult,
};
pub use requirements_conflict::{
    resolve_requirement_checks, resolve_requirement_checks_without_rule, ConflictResolutionRule,
    RequirementConflictError,
};
pub use requirements_coverage::{
    aggregate_covered_requirement_results, assess_requirement_coverage, CoveredAggregationError,
    CoverageAssessment, CoverageAssessmentError, CoverageDisposition, CoverageRule,
    CoverageRuleFormationError,
};
pub use requirements_reuse::{
    reuse_historical_requirement_result, seal_historical_qualified_result,
    HistoricalQualificationError, HistoricalQualifiedRequirementResult, ReuseAssessment,
    ReuseAssessmentError, ReuseDisposition, ReuseRejectionReason, ReuseRule,
    ReuseRuleFormationError,
};
pub use resolution::{
    ResSpec, ResolutionRecord, ResolutionTarget, UnsafeUResolution,
    U_RESOLUTION_DIAGNOSTIC_CODE,
};

pub const GRAMMAR_VERSION: &str = "0.2";
pub const IR_VERSION: &str = "0.3";
pub const SERIALIZER_VERSION: &str = "0.1.0";

pub const GRAMMAR_VERSION_MAJOR: u16 = 0;
pub const GRAMMAR_VERSION_MINOR: u16 = 2;
pub const IR_VERSION_MAJOR: u16 = 0;
pub const IR_VERSION_MINOR: u16 = 3;
pub const SERIALIZER_VERSION_MAJOR: u16 = 0;
pub const SERIALIZER_VERSION_MINOR: u16 = 1;
pub const SERIALIZER_VERSION_PATCH: u16 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    Frontend(FrontendError),
    InvalidProgram(String),
}

impl From<FrontendError> for CompileError {
    fn from(error: FrontendError) -> Self {
        Self::Frontend(error)
    }
}

/// Compila texto SVP a la representación soberana IR 0.3 y valida su
/// bienformación antes de exponerla fuera del núcleo.
///
/// El analizador sintáctico y el descenso permanecen internos. Un adaptador
/// externo no puede solicitar una `IrProgram` aceptada sin atravesar también
/// la validación soberana de este núcleo.
pub fn compile_svp(source: &str, source_file: &str) -> Result<IrProgram, CompileError> {
    let program = frontend::compile_svp(source, source_file)?;
    wellformed::validate_program(&program).map_err(CompileError::InvalidProgram)?;
    Ok(program)
}

pub fn compile_svp_profile(
    source: &str,
    source_file: &str,
    profile: SourceProfile,
) -> Result<IrProgram, CompileError> {
    let program = frontend::compile_svp_with_profile(source, source_file, profile)?;
    wellformed::validate_program(&program).map_err(CompileError::InvalidProgram)?;
    Ok(program)
}


/// Unidad fuente de un ensamblaje multifuente experimental.
///
/// Cada unidad conserva su frontera, nombre y perfil. La superficie se analiza
/// por separado; sólo la representación canónica resultante participa en la
/// reunión previa a la validación global.
#[derive(Debug, Clone, Copy)]
pub struct SourceUnit<'a> {
    source: &'a str,
    source_file: &'a str,
    profile: SourceProfile,
}

impl<'a> SourceUnit<'a> {
    pub const fn new(source: &'a str, source_file: &'a str, profile: SourceProfile) -> Self {
        Self { source, source_file, profile }
    }

    pub const fn source(&self) -> &'a str { self.source }
    pub const fn source_file(&self) -> &'a str { self.source_file }
    pub const fn profile(&self) -> SourceProfile { self.profile }
}

fn assembly_identity(units: &[SourceUnit<'_>]) -> String {
    let mut framed = b"SVP-ASSEMBLY-2B\0".to_vec();
    framed.extend_from_slice(&(units.len() as u64).to_be_bytes());
    for unit in units {
        let file = unit.source_file.as_bytes();
        let source = unit.source.as_bytes();
        framed.extend_from_slice(&(file.len() as u64).to_be_bytes());
        framed.extend_from_slice(file);
        framed.extend_from_slice(&(unit.profile.abi_code() as u64).to_be_bytes());
        framed.extend_from_slice(&(source.len() as u64).to_be_bytes());
        framed.extend_from_slice(source);
    }
    frontend::sha256_hex(&framed)
}

/// Ensambla dos o más unidades fuente mediante una única IR y una única
/// validación global.
///
/// No concatena texto ni tokens entre archivos. Cada fuente alcanza EOF dentro
/// de su propia frontera y se canonicaliza bajo su perfil explícito. Los
/// objetos y operaciones resultantes se reúnen conservando el orden de las
/// unidades; sólo entonces se ejecuta la validación de bienformación sobre el
/// programa conjunto. Esto permite referencias entre unidades sin permitir que
/// una producción sintáctica atraviese la frontera entre archivos.
pub fn compile_svp_assembly(units: &[SourceUnit<'_>]) -> Result<IrProgram, CompileError> {
    if units.len() < 2 {
        return Err(CompileError::InvalidProgram(
            "el ensamblaje multifuente exige al menos dos unidades".to_owned(),
        ));
    }

    let mut objects = Vec::new();
    let mut operations = Vec::new();
    for unit in units {
        let parsed = frontend::compile_svp_with_profile(
            unit.source,
            unit.source_file,
            unit.profile,
        )?;
        objects.extend(parsed.objects().iter().cloned());
        operations.extend(parsed.operations().iter().cloned());
    }

    let identity = assembly_identity(units);
    let program = ir::construction::program(
        format!("@assembly/2b/{}-units", units.len()),
        identity,
        objects,
        operations,
    );
    wellformed::validate_program(&program).map_err(CompileError::InvalidProgram)?;
    Ok(program)
}

#[cfg(test)]
mod multisource_assembly_tests_2b {
    use super::*;

    const A_EN: &str = r#"
codomain K3 = { APTO, NO_APTO, INDETERMINADO };
output_semantics Klin {
  APTO -> "favorable";
  NO_APTO -> "desfavorable";
  INDETERMINADO -> "sin cierre";
}
"#;

    const A_ES: &str = r#"
codominio K3 = { APTO, NO_APTO, INDETERMINADO };
semántica_de_salida Klin {
  APTO -> "favorable";
  NO_APTO -> "desfavorable";
  INDETERMINADO -> "sin cierre";
}
"#;

    const B_EN: &str = r#"
cellspec C1 {
  b: 3;
  codomain: K3;
  semantics: Klin;
  role: Base;
}
cellstate S1 {
  spec: C1;
  vector: [Zero, One, U, Zero, Zero, One, U, Zero, One];
}
let E1 = evaluate(S1);
"#;

    const B_ES: &str = r#"
especificación_de_celda C1 {
  b: 3;
  codominio: K3;
  semántica: Klin;
  rol: Base;
}
estado_de_celda S1 {
  especificación: C1;
  vector: [Cero, Uno, U, Cero, Cero, Uno, U, Cero, Uno];
}
sea E1 = evaluar(S1);
"#;

    fn assemble(a: &str, pa: SourceProfile, b: &str, pb: SourceProfile) -> IrProgram {
        compile_svp_assembly(&[
            SourceUnit::new(a, "a.svp", pa),
            SourceUnit::new(b, "b.svp", pb),
        ]).expect("ensamblaje admisible")
    }

    #[test]
    fn admite_las_cuatro_combinaciones_de_perfil() {
        for (a, pa, b, pb) in [
            (A_EN, SourceProfile::En, B_EN, SourceProfile::En),
            (A_EN, SourceProfile::En, B_ES, SourceProfile::Es),
            (A_ES, SourceProfile::Es, B_EN, SourceProfile::En),
            (A_ES, SourceProfile::Es, B_ES, SourceProfile::Es),
        ] {
            let p = assemble(a, pa, b, pb);
            assert_eq!(p.objects().len(), 4);
            assert_eq!(p.operations().len(), 1);
        }
    }

    #[test]
    fn referencia_transfronteriza_exige_el_ensamblaje_global() {
        assert!(compile_svp_profile(B_EN, "b.svp", SourceProfile::En).is_err());
        assert!(compile_svp_profile(B_ES, "b.svp", SourceProfile::Es).is_err());
        assert!(compile_svp_assembly(&[
            SourceUnit::new(B_ES, "b.svp", SourceProfile::Es),
            SourceUnit::new(A_EN, "a.svp", SourceProfile::En),
        ]).is_ok());
    }

    #[test]
    fn una_produccion_no_puede_cruzar_la_frontera_de_archivo() {
        let left = "codomain K3 = {";
        let right = "APTO, NO_APTO, INDETERMINADO };";
        assert!(compile_svp_assembly(&[
            SourceUnit::new(left, "left.svp", SourceProfile::En),
            SourceUnit::new(right, "right.svp", SourceProfile::En),
        ]).is_err());
    }

    #[test]
    fn el_perfil_erroneo_no_se_rescata_por_la_otra_unidad() {
        assert!(compile_svp_assembly(&[
            SourceUnit::new(A_EN, "a.svp", SourceProfile::Es),
            SourceUnit::new(B_ES, "b.svp", SourceProfile::Es),
        ]).is_err());
    }

    #[test]
    fn las_colisiones_globales_se_rechazan() {
        assert!(compile_svp_assembly(&[
            SourceUnit::new(A_EN, "a.svp", SourceProfile::En),
            SourceUnit::new(A_ES, "b.svp", SourceProfile::Es),
        ]).is_err());
    }

    #[test]
    fn la_identidad_de_ensamblaje_es_determinista_y_sensible_al_orden() {
        let ab = assemble(A_EN, SourceProfile::En, B_ES, SourceProfile::Es);
        let ab2 = assemble(A_EN, SourceProfile::En, B_ES, SourceProfile::Es);
        let ba = assemble(B_ES, SourceProfile::Es, A_EN, SourceProfile::En);
        assert_eq!(ab.source_sha256(), ab2.source_sha256());
        assert_ne!(ab.source_sha256(), ba.source_sha256());
    }
}

/// Valor ternario constitutivo del Lenguaje SV.
///
/// No representa estados técnicos de captura, admisibilidad, comprobación,
/// disponibilidad de plataforma ni deuda de realización.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tri {
    Zero = 0,
    One = 1,
    U = 2,
}

impl Tri {
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Representación textual canónica de la superficie del Lenguaje SV.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::U => "U",
        }
    }

    /// Etiqueta nominal usada por la IR canónica 0.3 para valores ternarios.
    #[inline]
    pub const fn ir_label(self) -> &'static str {
        match self {
            Self::Zero => "Zero",
            Self::One => "One",
            Self::U => "U",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTriValue(pub u8);

impl TryFrom<u8> for Tri {
    type Error = InvalidTriValue;

    fn try_from(value: u8) -> Result<Self, InvalidTriValue> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::U),
            other => Err(InvalidTriValue(other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineVersions {
    pub grammar: &'static str,
    pub ir: &'static str,
    pub serializer: &'static str,
}

pub const ENGINE_VERSIONS: EngineVersions = EngineVersions {
    grammar: GRAMMAR_VERSION,
    ir: IR_VERSION,
    serializer: SERIALIZER_VERSION,
};

#[cfg(test)]
mod c03_tests;

#[cfg(test)]
mod control_tests;

#[cfg(test)]
mod frame_tests;

#[cfg(test)]
mod ir_tests;

#[cfg(test)]
mod resolution_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tri_has_a_stable_one_byte_representation() {
        assert_eq!(core::mem::size_of::<Tri>(), 1);
    }

    #[test]
    fn tri_is_exactly_ternary_at_the_public_boundary() {
        assert_eq!(Tri::try_from(0), Ok(Tri::Zero));
        assert_eq!(Tri::try_from(1), Ok(Tri::One));
        assert_eq!(Tri::try_from(2), Ok(Tri::U));

        for value in 3..=u8::MAX {
            assert_eq!(Tri::try_from(value), Err(InvalidTriValue(value)));
        }
    }

    #[test]
    fn tri_numeric_representation_is_canonical() {
        assert_eq!(Tri::Zero.as_u8(), 0);
        assert_eq!(Tri::One.as_u8(), 1);
        assert_eq!(Tri::U.as_u8(), 2);
    }

    #[test]
    fn tri_textual_representation_is_canonical() {
        assert_eq!(Tri::Zero.label(), "0");
        assert_eq!(Tri::One.label(), "1");
        assert_eq!(Tri::U.label(), "U");
    }

    #[test]
    fn tri_roundtrip_is_canonical() {
        for tri in [Tri::Zero, Tri::One, Tri::U] {
            assert_eq!(Tri::try_from(tri.as_u8()), Ok(tri));
        }
    }

    #[test]
    fn technical_invalidity_is_not_u() {
        assert_ne!(Tri::try_from(3), Ok(Tri::U));
    }

    #[test]
    fn version_constants_match_the_integrated_frontend_contract() {
        assert_eq!(ENGINE_VERSIONS.grammar, "0.2");
        assert_eq!(ENGINE_VERSIONS.ir, "0.3");
        assert_eq!(ENGINE_VERSIONS.serializer, "0.1.0");
    }
}
