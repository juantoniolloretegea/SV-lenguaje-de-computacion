#![forbid(unsafe_code)]
use sv_core::authority::transitions::ExternalGenesisPremise;
fn main() { let _ = ExternalGenesisPremise { consumed: false }; }
