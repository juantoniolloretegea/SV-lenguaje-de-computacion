use sv_core::{compile_svp_profile, equivalence_json, SourceProfile};
fn main() {
 let path=std::env::args().nth(1).expect("archivo SVP");
 let source=std::fs::read_to_string(&path).expect("lectura");
 match compile_svp_profile(&source, &path, SourceProfile::En) {
  Ok(program)=>println!("{}",equivalence_json(&program)),
  Err(error)=>{eprintln!("{:?}",error); std::process::exit(2);}
 }
}
