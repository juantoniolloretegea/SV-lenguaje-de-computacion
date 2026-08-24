//! Núcleo semántico de R0 para el Lenguaje SV.
//!
//! Esta biblioteca Rust contiene la realización compartida por los destinos
//! nativo y WebAssembly. `Tri`, `Nat`, `Frame`, C01 y C02 residen en el mismo
//! núcleo. C03 corresponde al cierre relacional y causal J-F0…J-F5 de `Frame`:
//! no introduce un segundo constructor ni una semántica paralela, sino que
//! consolida y somete a regresión la misma autoridad de constitución.

pub mod admissibility;
pub mod frontend;
pub mod frame;
pub mod ir;
pub mod nat;
pub mod resolution;

pub use admissibility::{
    AdmissibilitySpec, AdmissibilityState, CaptureOutcome, InvalidAdmissibilitySpec,
    ADMISSIBILITY_DIAGNOSTIC_CODE,
};
pub use frontend::{compile_svp, FrontendError};
pub use frame::{Frame, FrameClosureViolation, FRAME_CLOSURE_DIAGNOSTIC_CODE};
pub use ir::{
    IrLevel, IrObject, IrObjectKind, IrOperation, IrOperationKind, IrProgram, IrQueryContext,
    IrSupervisableTarget,
};
pub use nat::{InvalidNat, Nat};
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

/// Valor ternario constitutivo del Lenguaje SV.
///
/// No representa estados técnicos de captura, admisibilidad, disponibilidad
/// de plataforma ni deuda de realización.
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

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
