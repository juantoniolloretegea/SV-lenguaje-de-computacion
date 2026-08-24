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
use sv_core::{compile_svp, equivalence_json};

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
// El bit 63 del resultado de `sv_compile_svp_json` indica error técnico o
// rechazo; los bits 32..62 contienen la longitud y los bits 0..31 el puntero.
// El host sólo copia esos bytes y no interpreta reglas del Lenguaje.
#[cfg(target_arch = "wasm32")]
const RESULT_ERROR: u64 = 1_u64 << 63;

#[cfg(target_arch = "wasm32")]
fn leak_bytes(bytes: Vec<u8>) -> (u32, u32) {
    let boxed = bytes.into_boxed_slice();
    let len = boxed.len();
    assert!(len <= 0x7fff_ffff, "resultado WebAssembly demasiado grande");
    let ptr = Box::into_raw(boxed) as *mut u8;
    (ptr as u32, len as u32)
}

#[cfg(target_arch = "wasm32")]
fn packed_result(bytes: Vec<u8>, error: bool) -> u64 {
    let (ptr, len) = leak_bytes(bytes);
    let mut packed = (ptr as u64) | ((len as u64) << 32);
    if error {
        packed |= RESULT_ERROR;
    }
    packed
}

/// Reserva un bloque de memoria lineal para que el host copie bytes de entrada.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_alloc(len: u32) -> u32 {
    let boxed = vec![0_u8; len as usize].into_boxed_slice();
    Box::into_raw(boxed) as *mut u8 as u32
}

/// Libera un bloque devuelto por `sv_alloc` o por `sv_compile_svp_json`.
///
/// # Seguridad de implementación
///
/// La única operación `unsafe` reconstruye el `Box<[u8]>` a partir del par
/// puntero/longitud emitido por esta misma ABI. No participa en la semántica.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_free(ptr: u32, len: u32) {
    let raw = core::ptr::slice_from_raw_parts_mut(ptr as *mut u8, len as usize);
    // SAFETY: `ptr,len` debe proceder de `sv_alloc` o `leak_bytes`; ambos
    // entregan exactamente un `Box<[u8]>` con esa longitud y transfieren su
    // propiedad al host hasta esta llamada.
    unsafe {
        drop(Box::from_raw(raw));
    }
}

/// Compila directamente texto `.svp` desde memoria lineal y devuelve la misma
/// proyección diferencial compartida por el binario nativo.
///
/// Esta función no acepta IR preconstituida ni JSON de Python. El texto cruza
/// `sv_core::compile_svp`, incluida la bienformación soberana, antes de poder
/// producir salida observable.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn sv_compile_svp_json(
    source_ptr: u32,
    source_len: u32,
    file_ptr: u32,
    file_len: u32,
) -> u64 {
    // SAFETY: los rangos son bloques de entrada previamente reservados con
    // `sv_alloc` por este módulo y escritos por el host antes de la llamada.
    let source_bytes = unsafe {
        core::slice::from_raw_parts(source_ptr as *const u8, source_len as usize)
    };
    let file_bytes = unsafe {
        core::slice::from_raw_parts(file_ptr as *const u8, file_len as usize)
    };

    let source = match core::str::from_utf8(source_bytes) {
        Ok(value) => value,
        Err(_) => return packed_result(b"entrada SVP no UTF-8".to_vec(), true),
    };
    let source_file = match core::str::from_utf8(file_bytes) {
        Ok(value) => value,
        Err(_) => return packed_result(b"nombre de archivo no UTF-8".to_vec(), true),
    };

    match compile_svp(source, source_file) {
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
