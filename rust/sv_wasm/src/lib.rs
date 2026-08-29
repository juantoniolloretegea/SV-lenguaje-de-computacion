//! Adaptador WebAssembly del núcleo Rust de R0.
//!
//! No contiene semántica propia: todas las decisiones se delegan a `sv_core`.
//! La ABI de navegador sólo transporta bytes entre la memoria lineal y el
//! mismo `compile_svp` soberano usado por el destino nativo.

use sv_core::{
    Tri, GRAMMAR_VERSION_MAJOR, GRAMMAR_VERSION_MINOR, IR_VERSION_MAJOR,
    IR_VERSION_MINOR, SERIALIZER_VERSION_MAJOR, SERIALIZER_VERSION_MINOR,
    SERIALIZER_VERSION_PATCH,
};

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use sv_core::{compile_svp, compile_svp_assembly, compile_svp_profile, equivalence_json, SourceProfile, SourceUnit};

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

// ABI de bytes para `wasm32-unknown-unknown`.
//
// La memoria pertenece siempre al módulo: el host sólo recibe punteros a
// búferes internos reutilizables. No existe transferencia manual de propiedad
// ni reconstrucción de punteros crudos en Rust.
#[cfg(target_arch = "wasm32")]
const RESULT_ERROR: u64 = 1_u64 << 63;

#[cfg(target_arch = "wasm32")]
std::thread_local! {
    static SOURCE_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static FILE_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static SOURCE_BUFFER_B: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static FILE_BUFFER_B: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static OUTPUT_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

#[cfg(target_arch = "wasm32")]
fn resize_buffer(buffer: &RefCell<Vec<u8>>, len: u32) -> u32 {
    let mut bytes = buffer.borrow_mut();
    bytes.resize(len as usize, 0);
    bytes.as_mut_ptr() as u32
}

/// Ajusta el búfer interno de texto SVP y devuelve su posición en memoria.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_source_buffer(len: u32) -> u32 {
    SOURCE_BUFFER.with(|buffer| resize_buffer(buffer, len))
}

/// Ajusta el búfer interno del nombre de archivo y devuelve su posición.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_file_buffer(len: u32) -> u32 {
    FILE_BUFFER.with(|buffer| resize_buffer(buffer, len))
}


/// Segundo búfer de fuente para el ensamblaje multifuente experimental.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_assembly_source_b_buffer(len: u32) -> u32 {
    SOURCE_BUFFER_B.with(|buffer| resize_buffer(buffer, len))
}

/// Segundo búfer de nombre de archivo para el ensamblaje multifuente experimental.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_assembly_file_b_buffer(len: u32) -> u32 {
    FILE_BUFFER_B.with(|buffer| resize_buffer(buffer, len))
}

#[cfg(target_arch = "wasm32")]
fn packed_result(bytes: Vec<u8>, error: bool) -> u64 {
    OUTPUT_BUFFER.with(|buffer| {
        let mut output = buffer.borrow_mut();
        *output = bytes;
        assert!(
            output.len() <= 0x7fff_ffff,
            "resultado WebAssembly demasiado grande"
        );
        let ptr = output.as_ptr() as u32;
        let len = output.len() as u32;
        let mut packed = (ptr as u64) | ((len as u64) << 32);
        if error {
            packed |= RESULT_ERROR;
        }
        packed
    })
}

/// Compila directamente el texto presente en los búferes internos y devuelve
/// la misma proyección diferencial compartida por el binario nativo.
///
/// Esta función no acepta IR preconstituida ni JSON de Python. El texto cruza
/// `sv_core::compile_svp`, incluida la bienformación soberana, antes de poder
/// producir salida observable.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_compile_svp_json() -> u64 {
    let source = SOURCE_BUFFER.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let source = match source {
        Ok(value) => value,
        Err(_) => return packed_result(b"entrada SVP no UTF-8".to_vec(), true),
    };

    let source_file = FILE_BUFFER.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let source_file = match source_file {
        Ok(value) => value,
        Err(_) => return packed_result(b"nombre de archivo no UTF-8".to_vec(), true),
    };

    match compile_svp(&source, &source_file) {
        Ok(program) => packed_result(equivalence_json(&program).into_bytes(), false),
        Err(error) => packed_result(format!("{error:?}").into_bytes(), true),
    }
}


#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_compile_svp_json_profile(profile: u32) -> u64 {
    let Some(profile) = SourceProfile::from_abi_code(profile) else {
        return packed_result(b"perfil SVP no admitido".to_vec(), true);
    };
    let source = SOURCE_BUFFER.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let source = match source {
        Ok(value) => value,
        Err(_) => return packed_result(b"entrada SVP no UTF-8".to_vec(), true),
    };
    let source_file = FILE_BUFFER.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let source_file = match source_file {
        Ok(value) => value,
        Err(_) => return packed_result(b"nombre de archivo no UTF-8".to_vec(), true),
    };
    match compile_svp_profile(&source, &source_file, profile) {
        Ok(program) => packed_result(equivalence_json(&program).into_bytes(), false),
        Err(error) => packed_result(format!("{error:?}").into_bytes(), true),
    }
}


/// Compila y valida conjuntamente dos unidades fuente con perfiles explícitos.
/// La unidad A usa los búferes ordinarios; la unidad B usa los búferes de
/// ensamblaje. No existe detección automática de idioma ni concatenación de
/// texto entre ambas fuentes.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_compile_svp_assembly_json(profile_a: u32, profile_b: u32) -> u64 {
    let Some(profile_a) = SourceProfile::from_abi_code(profile_a) else {
        return packed_result(b"perfil SVP A no admitido".to_vec(), true);
    };
    let Some(profile_b) = SourceProfile::from_abi_code(profile_b) else {
        return packed_result(b"perfil SVP B no admitido".to_vec(), true);
    };

    let source_a = SOURCE_BUFFER.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let source_a = match source_a {
        Ok(value) => value,
        Err(_) => return packed_result(b"entrada SVP A no UTF-8".to_vec(), true),
    };
    let file_a = FILE_BUFFER.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let file_a = match file_a {
        Ok(value) => value,
        Err(_) => return packed_result(b"nombre de archivo A no UTF-8".to_vec(), true),
    };
    let source_b = SOURCE_BUFFER_B.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let source_b = match source_b {
        Ok(value) => value,
        Err(_) => return packed_result(b"entrada SVP B no UTF-8".to_vec(), true),
    };
    let file_b = FILE_BUFFER_B.with(|buffer| String::from_utf8(buffer.borrow().clone()));
    let file_b = match file_b {
        Ok(value) => value,
        Err(_) => return packed_result(b"nombre de archivo B no UTF-8".to_vec(), true),
    };

    let units = [
        SourceUnit::new(&source_a, &file_a, profile_a),
        SourceUnit::new(&source_b, &file_b, profile_b),
    ];
    match compile_svp_assembly(&units) {
        Ok(program) => packed_result(equivalence_json(&program).into_bytes(), false),
        Err(error) => packed_result(format!("{error:?}").into_bytes(), true),
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
