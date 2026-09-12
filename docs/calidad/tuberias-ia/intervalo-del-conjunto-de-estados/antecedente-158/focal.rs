use sv_core::{SourceProfile as P, SourceUnit as U};
use sv_core::compiler_diagnostics::{self as d, Cause, FrontendCause, SiteRole, DeclarationId};
use sv_core::audit_diagnostics::Language;
use std::time::Instant;
fn check(id: &str, source: &str, profile: P, cause: &str, code: Option<&str>) -> d::Report {
    let start=Instant::now();
    let r=d::compile(source,"original.svp",profile).unwrap_err();
    assert_eq!(r.cause().id(),cause,"{id}");assert_eq!(r.cause().code(),code,"{id}");
    assert_eq!(r.legacy(),&sv_core::compile_svp_profile(source,"original.svp",profile).unwrap_err());
    assert_eq!(r.sources()[0].byte_len,source.len());
    assert_eq!(r.messages().len(),1);assert_eq!(r.messages()[0].0,if profile==P::Es {Language::Es}else{Language::En});
    for s in r.sites(){ if let Some(span)=&s.span {assert!(source.get(span.clone()).is_some());}}
    println!("CASE\t{id}\t{cause}\t{}",start.elapsed().as_nanos());r
}
fn main(){
 for (profile,word,label) in [(P::En,"codomain","EN"),(P::Es,"codominio","ES")] {
  let source=format!("-- prólogo é\r\n\t{word} K = {{}};\r\n");
  let r=check(&format!("EMPTY_{label}"),&source,profile,"CD.EMPTY_CODOMAIN",Some("E004"));
  assert_eq!(&source[r.sites()[0].span.clone().unwrap()],format!("{word} K = {{}};"));
  let source=format!("{word} K = {{B, A, B, A}};");
  let r=check(&format!("DUP_{label}"),&source,profile,"CD.DUPLICATE_CODOMAIN_MEMBERS",Some("E004"));
  assert!(matches!(r.cause(),Cause::DuplicateCodomainMembers{duplicates,..} if duplicates==&vec!["A".to_owned(),"B".to_owned()]));
  let source=format!("{word} K = {{A");
  let r=check(&format!("EOF_{label}"),&source,profile,"CD.UNEXPECTED_END",None);
  assert_eq!(r.sites()[0].span,Some(source.len()..source.len()));
 }
 for (p,key,label) in [(P::En,"output_semantics","EN"),(P::Es,"semántica_de_salida","ES")] {
  let source=format!("{key} S {{ A -> \"<script>SECRETO</script>\"; A -> \"otro\"; }}");
  let r=check(&format!("STANDALONE_{label}"),&source,p,"CD.OUTPUT_SEMANTICS_KEYS",Some("E115"));
  assert!(matches!(r.cause(),Cause::OutputSemanticsKeys{cell:None,codomain:None,duplicates,missing,extra,..} if duplicates==&vec!["A".to_owned()] && missing.is_empty() && extra.is_empty()));
  assert!(!format!("{r:?}").contains("SECRETO"));
  for (_,m) in r.messages(){assert!(!m.contains("SECRETO"));}
 }
 for (mappings,dups,missing,extra,label) in [
  ("A -> \"a\"; A -> \"a2\"; B -> \"b\";",vec!["A"],vec![],vec![],"REPEATED"),
  ("A -> \"a\";",vec![],vec!["B"],vec![],"MISSING"),
  ("A -> \"a\"; B -> \"b\"; X -> \"x\";",vec![],vec![],vec!["X"],"EXTRA"),
  ("A -> \"a\"; A -> \"a2\"; X -> \"x\";",vec!["A"],vec!["B"],vec!["X"],"ALL"),
 ] {
  for (p,cod,sem,cell,sk,rk,labelp) in [(P::En,"codomain","output_semantics","cellspec","semantics","role","EN"),(P::Es,"codominio","semántica_de_salida","especificación_de_celda","semántica","rol","ES")] {
   let source=format!("{cod} K = {{A, B}}; {sem} S {{ {mappings} }} {cell} C {{ b: 3; {cod}: K; {sk}: S; {rk}: Base; }}");
   let r=check(&format!("LINKED_{label}_{labelp}"),&source,p,"CD.OUTPUT_SEMANTICS_KEYS",Some("E115"));
   match r.cause(){Cause::OutputSemanticsKeys{cell,semantics,codomain,duplicates,missing:mi,extra:ex}=>{
    assert_eq!(cell.as_deref(),Some("C"));assert_eq!(semantics,"S");assert_eq!(codomain.as_deref(),Some("K"));
    assert_eq!(duplicates,&dups);assert_eq!(mi,&missing);assert_eq!(ex,&extra);
   },_=>panic!()}
   assert_eq!(r.sites().iter().map(|s|s.role).collect::<Vec<_>>(),vec![SiteRole::Cell,SiteRole::Semantics,SiteRole::Codomain]);
  }
 }
 for reverse in [false,true] {
  let mut units=[U::new("codomain K = {A};","same.svp",P::En),U::new("codominio K = {B};","same.svp",P::Es)];
  if reverse {units.reverse();}
  let start=Instant::now();let r=d::compile_assembly(&units).unwrap_err();
  assert_eq!(r.cause().id(),"CD.DUPLICATE_IDENTIFIER");assert_eq!(r.cause().code(),None);
  assert_eq!(r.messages().len(),2);assert_eq!(r.sites()[0].unit,0);assert_eq!(r.sites()[1].unit,1);
  assert_ne!(r.sources()[0].sha256,r.sources()[1].sha256);assert_eq!(r.sources()[0].file,r.sources()[1].file);
  assert_eq!(r.legacy(),&sv_core::compile_svp_assembly(&units).unwrap_err());
  println!("CASE\tCOLLISION_{reverse}\tCD.DUPLICATE_IDENTIFIER\t{}",start.elapsed().as_nanos());
 }
 for eof_first in [true,false] {
  let incomplete="codominio L = {";let empty="codomain K = {A};";
  let mut u=[U::new(incomplete,"same.svp",P::Es),U::new(empty,"same.svp",P::En)];if !eof_first {u.reverse();}
  let start=Instant::now();let r=d::compile_assembly(&u).unwrap_err();
  assert_eq!(r.cause(),&Cause::Frontend(FrontendCause::UnexpectedEnd));assert_eq!(r.sites()[0].unit,if eof_first{0}else{1});
  assert_eq!(r.sites()[0].span,Some(incomplete.len()..incomplete.len()));assert_eq!(r.messages().len(),2);
  println!("CASE\tASSEMBLY_EOF_{eof_first}\tCD.UNEXPECTED_END\t{}",start.elapsed().as_nanos());
 }
 for (p,source,label) in [(P::Es,"codomain K = {A};","ES"),(P::En,"codominio K = {A};","EN")] {
  let r=check(&format!("FOREIGN_{label}"),source,p,"CD.UNSUPPORTED",None);assert_eq!(r.sites()[0].span,None);
 }
 let source="output_semantics S { A -> \"SECRETO";
 let r=check("QUOTE_EOF",source,P::En,"CD.UNEXPECTED_END",None);assert_eq!(r.sites()[0].span,Some(source.len()..source.len()));assert!(!format!("{r:?}").contains("SECRETO"));
 let source="-- é\r\n\t💣";
 let r=check("LEXICAL_UTF8",source,P::En,"CD.UNEXPECTED_TOKEN",None);assert_eq!(&source[r.sites()[0].span.clone().unwrap()],"💣");
 let r=check("UNMIGRATED","cellspec C { b: 2; codomain: K; semantics: S; role: Base; }",P::En,"CD.UNMIGRATED",None);assert!(r.sites().is_empty());
 let source="let V = evaluate(S); codomain V = {A};";
 let r=check("OBJECT_OPERATION_COLLISION",source,P::En,"CD.DUPLICATE_IDENTIFIER",None);
 assert!(matches!(r.cause(),Cause::DuplicateIdentifier{first:DeclarationId::Object(0),second:DeclarationId::Operation(0),..}));
 assert_eq!(&source[r.sites()[0].span.clone().unwrap()],"codomain V = {A};");assert_eq!(&source[r.sites()[1].span.clone().unwrap()],"let V = evaluate(S);");
 let start=Instant::now();let r=d::compile_assembly(&[]).unwrap_err();assert_eq!(r.cause(),&Cause::InsufficientAssemblyUnits{actual:0});assert!(r.messages().is_empty());println!("CASE\tZERO_UNITS\tCD.INSUFFICIENT_ASSEMBLY_UNITS\t{}",start.elapsed().as_nanos());
 let start=Instant::now();let valid=d::compile("codomain K = {A};","x",P::En).unwrap();let old=sv_core::compile_svp("codomain K = {A};","x").unwrap();assert_eq!(valid,old);println!("CASE\tVALID\tOK\t{}",start.elapsed().as_nanos());
}
