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
