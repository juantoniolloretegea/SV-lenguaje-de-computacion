use sv_core::{SourceUnit as U,SourceProfile as P};
use sv_core::compiler_diagnostics::{self as d,SiteRole,Cause};
fn main(){
 let a="-- é\r\ncodomain K = {A, B};";
 let b="semántica_de_salida S { A -> \"SECRETO\"; X -> \"x\"; }";
 let c="cellspec C { b: 3; codomain: K; semantics: S; role: Base; }";
 let extra="codomain Z = {A};";
 let units=[U::new(a,"same.svp",P::En),U::new(b,"same.svp",P::Es),U::new(c,"same.svp",P::En),U::new(extra,"same.svp",P::En)];
 let start=std::time::Instant::now();let r=d::compile_assembly(&units).unwrap_err();
 assert_eq!(r.cause().code(),Some("E115"));
 assert!(matches!(r.cause(),Cause::OutputSemanticsKeys{missing,extra,..} if missing==&vec!["B".to_owned()] && extra==&vec!["X".to_owned()]));
 assert_eq!(r.sources().len(),4);assert_eq!(r.sites().len(),3);assert_eq!(r.messages().len(),2);
 for (s,idx,role,expected) in [( &r.sites()[0],2,SiteRole::Cell,c),(&r.sites()[1],1,SiteRole::Semantics,b),(&r.sites()[2],0,SiteRole::Codomain,"codomain K = {A, B};")]{
  assert_eq!(s.unit,idx);assert_eq!(s.role,role);assert_eq!(&units[idx].source()[s.span.clone().unwrap()],expected);
 }
 for (i,s) in r.sources().iter().enumerate(){
  assert_eq!(s.index,i);assert_eq!(s.file,"same.svp");assert_eq!(s.profile,units[i].profile());
  println!("HASH\t{i}\t{}",s.sha256);
 }
 println!("CASE\tE115_FOUR_UNITS\tCD.OUTPUT_SEMANTICS_KEYS\t{}",start.elapsed().as_nanos());
}
