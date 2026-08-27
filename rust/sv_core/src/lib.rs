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
pub use frontend::FrontendError;
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
