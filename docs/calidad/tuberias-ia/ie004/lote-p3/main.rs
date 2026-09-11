#![forbid(unsafe_code)]
#![allow(dead_code)]
// Fuentes heredadas intactas: contabilidad/JSON/SHA y léxico, sin motor semántico.
include!("../recepcion-av/json_estricto.rs");
include!("../recepcion-av/sha256.rs");
include!("../semantica-a/recursos.rs");
include!("adaptador.rs");
fn main(){if let Err(e)=programa(){if e==FalloT::Profundidad{eprintln!("SV_LOTE_ERROR:TOKENS_LIMITE")}else{eprintln!("SV_LOTE_ERROR:{:?}",e)}std::process::exit(2)}}
