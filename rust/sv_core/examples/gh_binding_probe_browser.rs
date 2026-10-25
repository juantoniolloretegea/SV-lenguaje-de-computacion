//! Sonda exclusiva de pruebas. No forma parte del adaptador sv_wasm productivo.
#[path = "../../../tests/row7_gh/probe.rs"]
mod cases;
use std::cell::RefCell;
thread_local! { static RESULT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) }; }

#[no_mangle]
pub extern "C" fn binding_probe_run() -> u32 {
    let (code, text) = match cases::report() { Ok(text) => (0,text), Err(text) => (1,text) };
    RESULT.with(|result| *result.borrow_mut() = text.into_bytes());
    code
}
#[no_mangle]
pub extern "C" fn binding_probe_ptr() -> *const u8 { RESULT.with(|r| r.borrow().as_ptr()) }
#[no_mangle]
pub extern "C" fn binding_probe_len() -> usize { RESULT.with(|r| r.borrow().len()) }


// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
