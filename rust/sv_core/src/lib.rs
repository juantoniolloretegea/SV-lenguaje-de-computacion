//! Núcleo semántico mínimo de R0 para el Lenguaje SV.
//!
//! Este crate inicia la realización Rust compartida por los destinos nativo y
//! WebAssembly. En este corte sólo fija `Tri` y los metadatos canónicos de
//! versión necesarios para evitar divergencias tempranas entre adaptadores.

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

    #[inline]
    pub const fn label(self) -> &'static str {
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
mod tests {
    use super::*;

    #[test]
    fn tri_is_exactly_ternary_at_the_public_boundary() {
        assert_eq!(Tri::try_from(0), Ok(Tri::Zero));
        assert_eq!(Tri::try_from(1), Ok(Tri::One));
        assert_eq!(Tri::try_from(2), Ok(Tri::U));
        assert_eq!(Tri::try_from(3), Err(InvalidTriValue(3)));
        assert_eq!(Tri::try_from(u8::MAX), Err(InvalidTriValue(u8::MAX)));
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
