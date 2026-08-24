//! Tipos cerrados de control para R1.
//!
//! Este módulo fija las distinciones nominales mínimas que las fases de
//! autoridad, mediación y fallo cerrado necesitan conservar. No concede
//! autoridad, no decide permisos y no ejecuta efectos protegidos.

use core::fmt;

/// Identificador opaco para objetos del dominio de control de R1.
///
/// Un `ControlId` sólo identifica. Su mera existencia no acredita que el
/// objeto nombrado esté constituido, admitido, autorizado o habilitado.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlId(String);

impl ControlId {
    /// Construye un identificador no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidControlId> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(InvalidControlId);
        }
        Ok(Self(value))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ControlId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error de construcción de un identificador de control vacío.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidControlId;

impl fmt::Display for InvalidControlId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("el identificador de control no puede estar vacío")
    }
}

/// Clase abstracta T-* de una transición SEC.0-A.
///
/// La etiqueta no constituye por sí misma una forma válida ni confiere
/// autoridad. La clasificación efectiva de una forma corresponde a cortes
/// posteriores de R1 y deberá derivar de constitución válida.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionClass {
    Information,
    Verification,
    Enablement,
    Exercise,
    Governance,
    Constitutive,
    Genesis,
    Recovery,
}

impl TransitionClass {
    /// Etiqueta contractual estable de la clase T-*.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Information => "T-I",
            Self::Verification => "T-V",
            Self::Enablement => "T-H",
            Self::Exercise => "T-E",
            Self::Governance => "T-G",
            Self::Constitutive => "T-C",
            Self::Genesis => "T-0",
            Self::Recovery => "T-R",
        }
    }
}

/// Resultado técnico cerrado de una comprobación SEC.0-D.
///
/// Este tipo no pertenece a `Tri`. Construir uno de sus valores tampoco
/// acredita una obligación por sí solo: R1 deberá gobernar qué comprobación
/// puede producir un resultado aplicable a una decisión protegida.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckResult {
    Accredited,
    Refuted,
    NotVerifiable,
}

impl CheckResult {
    /// Etiqueta contractual estable del resultado técnico.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Accredited => "D-A",
            Self::Refuted => "D-R",
            Self::NotVerifiable => "D-N",
        }
    }
}

/// Estado lógico mínimo de ocupación de una continuidad autoritativa.
///
/// R1-0 sólo fija el tipo cerrado. La regla que restringe T-0 y cualquier
/// transición de estado pertenecen al corte específico de transiciones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContinuityOccupancy {
    Uninhabited,
    Inhabited,
}

macro_rules! opaque_control_ref {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(ControlId);

        impl $name {
            /// Devuelve el identificador asociado sin conferir capacidades
            /// adicionales sobre el objeto referido.
            #[inline]
            pub fn id(&self) -> &ControlId {
                &self.0
            }

            /// Frontera interna de construcción.
            ///
            /// Los adaptadores externos no pueden convertir un identificador
            /// ordinario en una referencia constituida de esta categoría.
            #[inline]
            pub(crate) fn from_constituted_id(id: ControlId) -> Self {
                Self(id)
            }
        }
    };
}

opaque_control_ref!(
    InformationRef,
    "Referencia nominal a información dentro del dominio de control."
);
opaque_control_ref!(
    AdmittedEvidenceRef,
    "Referencia a evidencia cuya admisión debe haber sido constituida por una vía gobernada."
);
opaque_control_ref!(
    ConstitutedFactRef,
    "Referencia a un hecho semántico constituido; no equivale a autoridad."
);
opaque_control_ref!(
    AuthorityRef,
    "Referencia a autoridad constituida; su constructor no forma parte de la API pública."
);
opaque_control_ref!(
    EnablementRef,
    "Referencia a una habilitación de autoridad ya existente; no amplía autoridad."
);
opaque_control_ref!(
    ExerciseRef,
    "Referencia a un ejercicio materializado; no confiere titularidad por sí mismo."
);

/// Las categorías protegidas no pueden fabricarse desde un identificador por
/// la API pública.
///
/// ```compile_fail
/// use sv_core::{AuthorityRef, ControlId};
///
/// let id = ControlId::new("authority:1").unwrap();
/// let _authority = AuthorityRef::from_constituted_id(id);
/// ```
///
/// La construcción existe únicamente dentro de `sv_core`, donde los cortes
/// posteriores de R1 podrán ligarla a las condiciones contractuales.
pub fn protected_references_are_core_constructed() {}
