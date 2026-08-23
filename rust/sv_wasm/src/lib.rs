//! Adaptador WebAssembly inicial del núcleo Rust de R0.
//!
//! No contiene semántica propia: todas las decisiones ternarias se delegan a
//! `sv_core`. La ABI es deliberadamente mínima y numérica en este primer corte.

use sv_core::{
    Tri, GRAMMAR_VERSION_MAJOR, GRAMMAR_VERSION_MINOR, IR_VERSION_MAJOR,
    IR_VERSION_MINOR, SERIALIZER_VERSION_MAJOR, SERIALIZER_VERSION_MINOR,
    SERIALIZER_VERSION_PATCH,
};

const INVALID_TRI: i32 = -1;

#[no_mangle]
pub extern "C" fn sv_grammar_version_major() -> u32 {
    GRAMMAR_VERSION_MAJOR as u32
}

#[no_mangle]
pub extern "C" fn sv_grammar_version_minor() -> u32 {
    GRAMMAR_VERSION_MINOR as u32
}

#[no_mangle]
pub extern "C" fn sv_ir_version_major() -> u32 {
    IR_VERSION_MAJOR as u32
}

#[no_mangle]
pub extern "C" fn sv_ir_version_minor() -> u32 {
    IR_VERSION_MINOR as u32
}

#[no_mangle]
pub extern "C" fn sv_serializer_version_major() -> u32 {
    SERIALIZER_VERSION_MAJOR as u32
}

#[no_mangle]
pub extern "C" fn sv_serializer_version_minor() -> u32 {
    SERIALIZER_VERSION_MINOR as u32
}

#[no_mangle]
pub extern "C" fn sv_serializer_version_patch() -> u32 {
    SERIALIZER_VERSION_PATCH as u32
}

/// Devuelve 0, 1 o 2 para valores ternarios válidos; -1 para cualquier valor
/// ajeno a `Tri`. La invalidez técnica no se convierte en `U`.
#[no_mangle]
pub extern "C" fn sv_tri_decode(value: u32) -> i32 {
    if value > u8::MAX as u32 {
        return INVALID_TRI;
    }

    match Tri::try_from(value as u8) {
        Ok(tri) => tri.as_u8() as i32,
        Err(_) => INVALID_TRI,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_boundary_delegates_tri_to_core() {
        assert_eq!(sv_tri_decode(0), 0);
        assert_eq!(sv_tri_decode(1), 1);
        assert_eq!(sv_tri_decode(2), 2);
        assert_eq!(sv_tri_decode(3), INVALID_TRI);
        assert_eq!(sv_tri_decode(256), INVALID_TRI);
    }

    #[test]
    fn exported_versions_match_core() {
        assert_eq!(sv_grammar_version_major(), 0);
        assert_eq!(sv_grammar_version_minor(), 2);
        assert_eq!(sv_ir_version_major(), 0);
        assert_eq!(sv_ir_version_minor(), 3);
        assert_eq!(sv_serializer_version_major(), 0);
        assert_eq!(sv_serializer_version_minor(), 1);
        assert_eq!(sv_serializer_version_patch(), 0);
    }
}
