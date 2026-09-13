mod auxiliar;
fn main(){match auxiliar::cli(&std::env::args().collect::<Vec<_>>()){
    Ok(())=>println!("COMPROBACION_LOCAL_CONFORME"),
    Err(e)=>{eprintln!("RECHAZO_AUXILIAR: {e}");std::process::exit(1);}
}}
