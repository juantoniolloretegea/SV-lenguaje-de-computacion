use sv_core::{SourceProfile as P,SourceUnit as U};
use sv_core::compiler_diagnostics as d;
use sv_core::audit_diagnostics::Language;
fn check(id:&str, source:&str, p:P, cause:&str, span:Option<std::ops::Range<usize>>) {
 let start=std::time::Instant::now();
 let old=sv_core::compile_svp_profile(source,"igual.svp",p);
 let result=d::compile(source,"igual.svp",p);
 assert_eq!(old.is_ok(),cause=="OK","{id}: aceptación");
 assert_eq!(old,result.clone().map_err(|r|r.into_legacy()),"{id}: API");
 println!("LEGACY\t{id}\t{old:?}");
 if let Err(report)=result {
  if cfg!(candidate) {
   assert_eq!(report.cause().id(),cause,"{id}: causa");
   assert_eq!(report.sites().len(),1,"{id}: sitios");
   assert_eq!(report.sites()[0].span,span,"{id}: intervalo");
   assert_eq!(report.sites()[0].unit,0);
   assert_eq!(report.sources()[0].byte_len,source.len());
   assert_eq!(report.messages()[0].0,if p==P::Es {Language::Es}else{Language::En});
   assert!(!format!("{report:?}").contains("__SVP_FOREIGN_SURFACE__"));
   for (_,m) in report.messages(){assert!(!m.contains("__SVP_FOREIGN_SURFACE__"));}
  }
  let healthy=if p==P::Es {U::new("codomain Sano = {X};","igual.svp",P::En)}else{U::new("codominio Sano = {X};","igual.svp",P::Es)};
  for reverse in [false,true] {
   let unit=U::new(source,"igual.svp",p);let mut units=[unit,healthy];if reverse {units.reverse();}
   let a=d::compile_assembly(&units).unwrap_err();
   let old=sv_core::compile_svp_assembly(&units).unwrap_err();assert_eq!(a.legacy(),&old);
   println!("LEGACY_ASSEMBLY\t{id}\t{reverse}\t{old:?}");
   if cfg!(candidate) {
    assert_eq!(a.cause().id(),cause);assert_eq!(a.sites().len(),1);
    assert_eq!(a.sites()[0].unit,if reverse{1}else{0});assert_eq!(a.sites()[0].span,span);
    assert_eq!(a.messages().len(),2);assert_ne!(a.sources()[0].sha256,a.sources()[1].sha256);
   }
  }
 }
 println!("CASE\t{id}\t{cause}\t{}",start.elapsed().as_nanos());
}
fn main(){for (id,s,p,cause,span) in [("ROOT_FOREIGN_en","-- prólogo é\r\n\tcodominio K = {A};",P::En,"CD.FOREIGN_SURFACE",Some(17..26)),
("ROOT_UNKNOWN_en","-- prólogo é\r\n\tdesconocida K = {A};",P::En,"CD.UNSUPPORTED",Some(17..28)),
("ROOT_SYMBOL_en","-- prólogo é\r\n\t;",P::En,"CD.UNEXPECTED_TOKEN",Some(17..18)),
("IDENT_PROTECTED_en","codomain codomain = {A};",P::En,"CD.UNEXPECTED_TOKEN",Some(9..17)),
("IDENT_FOREIGN_en","codomain codominio = {A};",P::En,"CD.FOREIGN_SURFACE",Some(9..18)),
("IDENT_TOKEN_en","codomain 7 = {A};",P::En,"CD.UNEXPECTED_TOKEN",Some(9..10)),
("SYMBOL_en","codomain K : {A};",P::En,"CD.UNEXPECTED_TOKEN",Some(11..12)),
("TEXT_en","output_semantics S { A -> 7; }",P::En,"CD.UNEXPECTED_TOKEN",Some(26..27)),
("ARROW_en","output_semantics S { A : \"dato\"; }",P::En,"CD.UNEXPECTED_TOKEN",Some(23..24)),
("NAT_TYPE_en","cellspec C { b: palabra; }",P::En,"CD.UNEXPECTED_TOKEN",Some(16..23)),
("FIELD_FOREIGN_en","cellspec C { b: 3; codominio: K; }",P::En,"CD.FOREIGN_SURFACE",Some(19..28)),
("FIELD_UNKNOWN_en","cellspec C { b: 3; desconocido: K; }",P::En,"CD.UNEXPECTED_TOKEN",Some(19..30)),
("TRI_en","cellstate Q { spec: C; vector: [Tal]; }",P::En,"CD.INVALID_TRI",Some(32..35)),
("EOF_en","-- prólogo é\r\n\tcodomain K = {A",P::En,"CD.UNEXPECTED_END",Some(32..32)),
("LEXICAL_en","-- prólogo é\r\n\t💣",P::En,"CD.UNEXPECTED_TOKEN",Some(17..21)),
("CONTEXT_IDENTIFIER_en","codomain aridad = {A};",P::En,"OK",None),
("TEXT_DATA_en","output_semantics S { A -> \"codominio <script>SECRETO</script>\"; }",P::En,"OK",None),
("COMMENT_en","-- codominio\r\ncodomain K = {A};",P::En,"OK",None),
("VALID_en","codomain K = {A};",P::En,"OK",None),
("UNMIGRATED_en","cellspec C { b: 2; }",P::En,"CD.UNEXPECTED_TOKEN",Some(19..20)),
("ROOT_FOREIGN_es","-- prólogo é\r\n\tcodomain K = {A};",P::Es,"CD.FOREIGN_SURFACE",Some(17..25)),
("ROOT_UNKNOWN_es","-- prólogo é\r\n\tdesconocida K = {A};",P::Es,"CD.UNSUPPORTED",Some(17..28)),
("ROOT_SYMBOL_es","-- prólogo é\r\n\t;",P::Es,"CD.UNEXPECTED_TOKEN",Some(17..18)),
("IDENT_PROTECTED_es","codominio codominio = {A};",P::Es,"CD.UNEXPECTED_TOKEN",Some(10..19)),
("IDENT_FOREIGN_es","codominio codomain = {A};",P::Es,"CD.FOREIGN_SURFACE",Some(10..18)),
("IDENT_TOKEN_es","codominio 7 = {A};",P::Es,"CD.UNEXPECTED_TOKEN",Some(10..11)),
("SYMBOL_es","codominio K : {A};",P::Es,"CD.UNEXPECTED_TOKEN",Some(12..13)),
("TEXT_es","semántica_de_salida S { A -> 7; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(30..31)),
("ARROW_es","semántica_de_salida S { A : \"dato\"; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(27..28)),
("NAT_TYPE_es","especificación_de_celda C { b: palabra; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(32..39)),
("FIELD_FOREIGN_es","especificación_de_celda C { b: 3; codomain: K; }",P::Es,"CD.FOREIGN_SURFACE",Some(35..43)),
("FIELD_UNKNOWN_es","especificación_de_celda C { b: 3; desconocido: K; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(35..46)),
("TRI_es","estado_de_celda Q { especificación: C; vector: [Tal]; }",P::Es,"CD.INVALID_TRI",Some(49..52)),
("EOF_es","-- prólogo é\r\n\tcodominio K = {A",P::Es,"CD.UNEXPECTED_END",Some(33..33)),
("LEXICAL_es","-- prólogo é\r\n\t💣",P::Es,"CD.UNEXPECTED_TOKEN",Some(17..21)),
("CONTEXT_IDENTIFIER_es","codominio arity = {A};",P::Es,"OK",None),
("TEXT_DATA_es","semántica_de_salida S { A -> \"codomain <script>SECRETO</script>\"; }",P::Es,"OK",None),
("COMMENT_es","-- codomain\r\ncodominio K = {A};",P::Es,"OK",None),
("VALID_es","codominio K = {A};",P::Es,"OK",None),
("UNMIGRATED_es","especificación_de_celda C { b: 2; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(35..36))] {check(id,s,p,cause,span);}}
