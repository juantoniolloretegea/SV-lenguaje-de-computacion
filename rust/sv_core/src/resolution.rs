use crate::{Nat, Tri};

/// Código diagnóstico canónico para una revisión de `U` no constituida de forma segura.
pub const U_RESOLUTION_DIAGNOSTIC_CODE: &str = "E305";

/// Especificación de la clase de revisión aplicable a una `U`.
///
/// `context` y `mechanism` son identidades opacas del contrato canónico. R0-4
/// exige coincidencia exacta de la instancia mientras no exista una relación
/// ampliada expresamente constituida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResSpec {
    name: String,
    context: String,
    mechanism: String,
    mapping: String,
}

impl ResSpec {
    pub fn new(
        name: impl Into<String>,
        context: impl Into<String>,
        mechanism: impl Into<String>,
        mapping: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            context: context.into(),
            mechanism: mechanism.into(),
            mapping: mapping.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn mechanism(&self) -> &str {
        &self.mechanism
    }

    pub fn mapping(&self) -> &str {
        &self.mapping
    }
}

/// Objetivo constituido de una revisión.
///
/// La posición es uno-basada. La construcción efectiva permanece dentro de
/// `sv_core`, donde puede comprobarse contra un estado evaluable real.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionTarget {
    state: String,
    position: Nat,
}

impl ResolutionTarget {
    fn new(state: impl Into<String>, position: Nat) -> Self {
        Self {
            state: state.into(),
            position,
        }
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn position(&self) -> &Nat {
        &self.position
    }
}

/// Registro de revisión de una `U` constituida.
///
/// En R0-4 el registro no dispone de constructor público. El núcleo puede
/// conservar material de revisión en `reviewed_to`, pero la revisión por sí
/// sola no confiere autoridad de clausura: `resolved_to` permanece `Tri::U`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionRecord {
    target: ResolutionTarget,
    previous: Tri,
    reviewed_to: Option<Tri>,
    resolved_to: Tri,
    context_ref: String,
    mechanism_ref: String,
}

impl ResolutionRecord {
    pub fn target(&self) -> &ResolutionTarget {
        &self.target
    }

    pub const fn previous(&self) -> Tri {
        self.previous
    }

    pub const fn reviewed_to(&self) -> Option<Tri> {
        self.reviewed_to
    }

    pub const fn resolved_to(&self) -> Tri {
        self.resolved_to
    }

    pub fn context_ref(&self) -> &str {
        &self.context_ref
    }

    pub fn mechanism_ref(&self) -> &str {
        &self.mechanism_ref
    }
}

/// Violación del contrato de revisión identificada de `U`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsafeUResolution {
    TargetNotEvaluable {
        state: String,
        kind: String,
    },
    PositionOutOfRange {
        state: String,
        position: Nat,
        vector_len: usize,
    },
    TargetIsNotU {
        state: String,
        position: Nat,
        found: Tri,
    },
    ContextMismatch {
        spec: String,
        expected: String,
        actual: String,
    },
    MechanismMismatch {
        spec: String,
        expected: String,
        actual: String,
    },
}

impl UnsafeUResolution {
    pub const fn diagnostic_code(&self) -> &'static str {
        U_RESOLUTION_DIAGNOSTIC_CODE
    }
}

/// Proyección interna de símbolos ya resueltos necesaria para aplicar C02.
///
/// La resolución general de símbolos permanece fuera de R0-4; por ello estas
/// formas no constituyen una interfaz pública ni permiten a un adaptador declarar
/// por sí mismo que una referencia ha sido resuelta.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "R0-4 materializa C02 antes de enlazar el resolvedor interno posterior"
    )
)]
pub(crate) mod resolved {
    use crate::Tri;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum ResolvedTargetState {
        Cell {
            name: String,
            vector: Vec<Tri>,
        },
        Coupled {
            name: String,
            updated_vector: Vec<Tri>,
        },
        Other {
            name: String,
            kind: String,
        },
    }

    impl ResolvedTargetState {
        pub(crate) fn cell(name: impl Into<String>, vector: Vec<Tri>) -> Self {
            Self::Cell {
                name: name.into(),
                vector,
            }
        }

        pub(crate) fn coupled(name: impl Into<String>, updated_vector: Vec<Tri>) -> Self {
            Self::Coupled {
                name: name.into(),
                updated_vector,
            }
        }

        pub(crate) fn other(name: impl Into<String>, kind: impl Into<String>) -> Self {
            Self::Other {
                name: name.into(),
                kind: kind.into(),
            }
        }

        pub(crate) fn name(&self) -> &str {
            match self {
                Self::Cell { name, .. }
                | Self::Coupled { name, .. }
                | Self::Other { name, .. } => name,
            }
        }

        pub(crate) fn kind(&self) -> &str {
            match self {
                Self::Cell { .. } => "CellState",
                Self::Coupled { .. } => "CoupledState",
                Self::Other { kind, .. } => kind,
            }
        }

        pub(crate) fn effective_vector(&self) -> Option<&[Tri]> {
            match self {
                Self::Cell { vector, .. } => Some(vector.as_slice()),
                Self::Coupled { updated_vector, .. } => Some(updated_vector.as_slice()),
                Self::Other { .. } => None,
            }
        }
    }
}

use resolved::ResolvedTargetState;

/// Constituye el registro de revisión para una `U` identificada.
///
/// La función permanece interna a `sv_core` hasta enlazar la resolución general
/// de símbolos. No recibe autoridad de clausura positiva: por construcción,
/// `resolved_to` es `Tri::U` en este frente.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "R0-4 materializa la operación interna antes de enlazar el resolvedor posterior"
    )
)]
pub(crate) fn review_u(
    state: &ResolvedTargetState,
    position: Nat,
    with_spec: &ResSpec,
    context_instance: impl Into<String>,
    mechanism_instance: impl Into<String>,
    reviewed_to: Option<Tri>,
) -> Result<ResolutionRecord, UnsafeUResolution> {
    let vector = state
        .effective_vector()
        .ok_or_else(|| UnsafeUResolution::TargetNotEvaluable {
            state: state.name().to_owned(),
            kind: state.kind().to_owned(),
        })?;

    let index = one_based_index(&position, vector.len()).ok_or_else(|| {
        UnsafeUResolution::PositionOutOfRange {
            state: state.name().to_owned(),
            position: position.clone(),
            vector_len: vector.len(),
        }
    })?;

    let previous = vector[index];
    if previous != Tri::U {
        return Err(UnsafeUResolution::TargetIsNotU {
            state: state.name().to_owned(),
            position,
            found: previous,
        });
    }

    let context_instance = context_instance.into();
    if context_instance != with_spec.context {
        return Err(UnsafeUResolution::ContextMismatch {
            spec: with_spec.name.clone(),
            expected: with_spec.context.clone(),
            actual: context_instance,
        });
    }

    let mechanism_instance = mechanism_instance.into();
    if mechanism_instance != with_spec.mechanism {
        return Err(UnsafeUResolution::MechanismMismatch {
            spec: with_spec.name.clone(),
            expected: with_spec.mechanism.clone(),
            actual: mechanism_instance,
        });
    }

    Ok(ResolutionRecord {
        target: ResolutionTarget::new(state.name(), position),
        previous: Tri::U,
        reviewed_to,
        resolved_to: Tri::U,
        context_ref: context_instance,
        mechanism_ref: mechanism_instance,
    })
}

fn one_based_index(position: &Nat, len: usize) -> Option<usize> {
    let mut value: usize = 0;

    for byte in position.as_decimal().bytes() {
        value = value
            .checked_mul(10)?
            .checked_add(usize::from(byte - b'0'))?;

        if value > len {
            return None;
        }
    }

    if value == 0 {
        None
    } else {
        Some(value - 1)
    }
}
