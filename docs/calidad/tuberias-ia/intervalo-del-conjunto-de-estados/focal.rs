use sv_core::{SourceProfile as P,SourceUnit as U};
use sv_core::compiler_diagnostics as d;
use sv_core::audit_diagnostics::Language;
fn check(id:&str, source:&str, p:P, cause:&str, span:Option<std::ops::Range<usize>>, digest:&str) {
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
   assert_eq!(report.sources()[0].sha256,digest,"{id}: huella original");
   assert_eq!(report.sources()[0].profile,p);
   assert_eq!(report.stage(),d::Stage::Frontend);
   if let Some(ref bytes)=span {assert!(source.is_char_boundary(bytes.start) && source.is_char_boundary(bytes.end));}
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
    let index=if reverse{1}else{0};
    assert_eq!(a.sources()[index].sha256,digest);assert_eq!(a.sources()[index].profile,p);
    assert_eq!(a.sources()[index].byte_len,source.len());
    assert_eq!(a.messages().len(),2);assert_ne!(a.sources()[0].sha256,a.sources()[1].sha256);
   }
  }
 }
 println!("CASE\t{id}\t{cause}\t{}",start.elapsed().as_nanos());
}
fn main(){for (id,s,p,cause,span,digest) in [("SEMICOLON_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok} rule: R; }",P::En,"CD.UNEXPECTED_TOKEN",Some(90..94),"424ca7e3c9bb3500f7b1be5323390cc37703608e3315c79c628474324793f55a"),
("COUNT_ONE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..89),"e5e5ae593359eed6fcf0969bee9fea1691cc36098ae1f4f8d4bf0dc7f40272b6"),
("COUNT_TWO_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, Degraded}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..99),"b7b5df19ef33b29e32d904d388fd4a7ead3e5ef3f65d33eff6fc92aaabfa6677"),
("COUNT_FOUR_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, Degraded, NotAdmitted, Ok}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..116),"a5b1257d4e61078bde7fe907a4b8da1be3956ba90bff41e3205155718e647365"),
("COUNT_SIX_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, Degraded, NotAdmitted, Ok, Degraded, NotAdmitted}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..139),"21579f811463c26a64d1070e4b7c7053802e2d2ddba03b1c832a40855ac45088"),
("COUNT_COMMENTS_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, -- interno é { }\r\n\tDegraded}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..119),"98f4cf8c3d9c6c0b646b977bd38cd293262c2b398afecc04d01dc2a4370b3c37"),
("COUNT_BEFORE_EOF_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok};",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..89),"f5cb33815d3e42d6b58efdf21553bc9cae6811d0395ba430d483a0aad35d0531"),
("COUNT_BEFORE_RULE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok}; inventada: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..89),"6f296e9ec7e918750a5ce1e8b6e9b68130d6489737b434bc2b77cd00726c57eb"),
("COUNT_EXCLUDES_TRAILING_COMMENT_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok} -- fuera é { }\r\n; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(85..89),"2895b4b34dc33106e65d944213b7b5d83593b1aa58192b70694f747b08968d32"),
("EMPTY_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {}; rule: R; }",P::En,"CD.UNEXPECTED_TOKEN",Some(86..87),"cb5caed947795f6ed0ef982040e1a5aed00bdfa6f0a1c0317a9a8d50de1bd068"),
("TRAILING_COMMA_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok,}; rule: R; }",P::En,"CD.UNEXPECTED_TOKEN",Some(89..90),"e294a92267844dbaa6978c52bc48d4fd3b478860ac4760a1909b4b8d108c9d92"),
("LABEL_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, inventado}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(90..99),"21e179c457b03d99afb10234180502b1a7b6f55319f121f0e8eb2de9833c023a"),
("FOREIGN_LABEL_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Admitido}; rule: R; }",P::En,"CD.FOREIGN_SURFACE",Some(86..94),"84b61cdc48fc9c45921a4361797f4ce746ebae69afd87f84f78075b2ec7c3932"),
("TOKEN_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {7}; rule: R; }",P::En,"CD.UNEXPECTED_TOKEN",Some(86..87),"ab92e4907f9f48babf5b2283cee19a266d2c5bcefc0226eff978e38df88bf5b9"),
("CLOSING_BRACE_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok];",P::En,"CD.UNEXPECTED_TOKEN",Some(88..89),"4671e765a76c911455d2f1f7abfa32abc21e13399ea344dbc645746a61f1b144"),
("LIST_EOF_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok",P::En,"CD.UNEXPECTED_END",Some(88..88),"6da7abdc162eb2f82ca8f8483533a13d30537d9b8aff9c06e0268298c2418c65"),
("AFTER_BRACE_EOF_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok}",P::En,"CD.UNEXPECTED_END",Some(89..89),"f784745ecd1990f07119c086e2c29ebeccb1c436141b8f8e0d8741f12fe8d1bd"),
("LEXICAL_PRECEDENCE_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok}; rule: R; }💣",P::En,"CD.UNEXPECTED_TOKEN",Some(101..105),"5ce510657b84b97d8be5fedb75c32811b376e501f270d5ac475058e3465f7c34"),
("VALID_ORDER_1_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, Degraded, NotAdmitted}; rule: R; }",P::En,"OK",None,"99ab6792ee773745850136b1c5705a2d6f8f5caba0fcdd59dcd03e2b389292df"),
("VALID_ORDER_2_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, NotAdmitted, Degraded}; rule: R; }",P::En,"OK",None,"b4891f4f92ca4892a0bd21bf42bf203ad2114059f11c79800407e87bbe233e27"),
("VALID_ORDER_3_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Degraded, Ok, NotAdmitted}; rule: R; }",P::En,"OK",None,"1c75b0a072f485dd031fc01d580f68c6d6872e4e4bfb0070a3e5fca224f462fb"),
("VALID_ORDER_4_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Degraded, NotAdmitted, Ok}; rule: R; }",P::En,"OK",None,"f6c38121e2ad6a00dc24b21b354ddf7d3fb4f6d577a7d59ef86d6d28dd1891b0"),
("VALID_ORDER_5_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {NotAdmitted, Ok, Degraded}; rule: R; }",P::En,"OK",None,"e457ab9f825d24e6e1963686f72dfa488eb64302cea495f22ea35fb6b53098d0"),
("VALID_ORDER_6_en","-- prólogo é: origen y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {NotAdmitted, Degraded, Ok}; rule: R; }",P::En,"OK",None,"0513d70b1880289804a521aedda9681d11f1fecac57a9a584d78126f5e2d4241"),
("SEMICOLON_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido} regla: R; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(126..131),"a2d896bab4353a4f94870bdeb0dfc3ae21d5089d3dd52874e95d69c2bf043780"),
("COUNT_ONE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..125),"e5b2a5fae8179a7d8ab73b0e1348e50b8a315a19d15da8c1a7296d9790c1a4bb"),
("COUNT_TWO_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, Degradado}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..136),"bfa026a4be5252173053941f23281faadefe9a881bbf82a5b513c173c78cd849"),
("COUNT_FOUR_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, Degradado, NoAdmitido, Admitido}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..158),"03fed1b81af41b815dc189f46f427f156eea11b224a8a0efdc5b17f3dbb4f584"),
("COUNT_SIX_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, Degradado, NoAdmitido, Admitido, Degradado, NoAdmitido}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..181),"987404ae23c2beac8e41a64b473204fa55d3d0315d128f93ec50e0216d57dc5d"),
("COUNT_COMMENTS_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, -- interno é { }\r\n\tDegradado}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..156),"30f70d9b841f31edd26e5d77b2b401b380184c43dd5455e6459f374626ea0fcb"),
("COUNT_BEFORE_EOF_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido};",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..125),"c8a11932894881479ebf4a87cb7410e20f6d95982b00482adc04c5224a48ddf1"),
("COUNT_BEFORE_RULE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido}; inventada: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..125),"e64453ba0ef7567ce86409a13c46358a0742c2e86d48db0fddc812032ef17bfc"),
("COUNT_EXCLUDES_TRAILING_COMMENT_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido} -- fuera é { }\r\n; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(115..125),"b20603b128f6efaabac9c5e3ae335076f43791c21c580e1db73da5e1a1a59bad"),
("EMPTY_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {}; regla: R; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(116..117),"93652dd9b57aceda3dd9684a4ff7cc3faf6b92480fa016801c8be9d972a27e3e"),
("TRAILING_COMMA_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido,}; regla: R; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(125..126),"befb4977d71c664249706400aeb1c7727b6447bb9dbea7634cf007b147798644"),
("LABEL_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, inventado}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(126..135),"1e6abdd308aa5274e80f0f98c7fd93b3f20e5e4b85edd442e4b4410c156efde6"),
("FOREIGN_LABEL_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Ok}; regla: R; }",P::Es,"CD.FOREIGN_SURFACE",Some(116..118),"61bfb44b426e30a424d6be4fb127f5156ccdcc70d78e5c0f78c5a907b5b22129"),
("TOKEN_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {7}; regla: R; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(116..117),"dcfd3db4626fe5f4b43451812d5461d9f02fc3fcf58e9dc95bafc502a0f1bfae"),
("CLOSING_BRACE_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido];",P::Es,"CD.UNEXPECTED_TOKEN",Some(124..125),"7ed485f3d836eff8f0f1c5efa7c1794a8d7d8f906c8d3152d6860d42abca1777"),
("LIST_EOF_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido",P::Es,"CD.UNEXPECTED_END",Some(124..124),"9226a72ded33ecee782632ba658e3929a01398a829b3699dab259f282e96cd0f"),
("AFTER_BRACE_EOF_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido}",P::Es,"CD.UNEXPECTED_END",Some(125..125),"a74c06391d58bc0282b4be2e776f97fe60fef2ae5f07dd90839f258c2ada0789"),
("LEXICAL_PRECEDENCE_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido}; regla: R; }💣",P::Es,"CD.UNEXPECTED_TOKEN",Some(138..142),"5e4f2c5f086a263c93861591fd2522ba7f18cd57fefc88e91b971046ce7862aa"),
("VALID_ORDER_1_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, Degradado, NoAdmitido}; regla: R; }",P::Es,"OK",None,"689b952b22e1734de4fb2ee73657c93404175d3246f97665980173c4f42ef11a"),
("VALID_ORDER_2_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, NoAdmitido, Degradado}; regla: R; }",P::Es,"OK",None,"100d71dfa57ad2230ecefd5ab76c9c09b94451efb21f269a12c66c6e9b00b165"),
("VALID_ORDER_3_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Degradado, Admitido, NoAdmitido}; regla: R; }",P::Es,"OK",None,"ebba4963e2c51e67c735d1a344d9edc5215a6f0f6a9f51eefef9c2ab70658dc2"),
("VALID_ORDER_4_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Degradado, NoAdmitido, Admitido}; regla: R; }",P::Es,"OK",None,"e1c17ce8c82d085a1b757313be20133ae1e8346d3675cb1989e3f7da0abb7473"),
("VALID_ORDER_5_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {NoAdmitido, Admitido, Degradado}; regla: R; }",P::Es,"OK",None,"8fe8e87940b0057bc2bf826e1bc02277d7f2c9c27a1ad77d67b6bbb994b99f95"),
("VALID_ORDER_6_es","-- prólogo é: origen y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {NoAdmitido, Degradado, Admitido}; regla: R; }",P::Es,"OK",None,"0af807ba44af4b33f5e8c37a7e6063dcc7e01783cdbfae46532f68700810543b"),
("rel_REPEAT_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; table: TabA; table: TabA; }",P::En,"CD.UNEXPECTED_TOKEN",Some(107..112),"f3fa60f080f2de4fde8767b9da1e981b7dcb83111be486b1e566261c74a4e066"),
("rel_ORDER_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; constraints: [Local]; table: TabA; }",P::En,"CD.UNEXPECTED_TOKEN",Some(116..121),"6614ad013e7a075cc784386108327013a5e811a114ce244ef07019d03bb0d728"),
("rel_CONSTRAINTS_REPEAT_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; constraints: [Local]; constraints: [Local]; }",P::En,"CD.UNEXPECTED_TOKEN",Some(116..127),"4410600cdf6a0bc52944123d5cfc7f78f565bdd6f8d11c3131699727bd870272"),
("rel_UNKNOWN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; inventado: 3; }",P::En,"CD.UNSUPPORTED",Some(94..103),"519da50058c5b58968b9aee5b098287cbc245192bc977cc9e10461e589baea59"),
("rel_FOREIGN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; aridad: 3; }",P::En,"CD.FOREIGN_SURFACE",Some(94..100),"dcf2d10ed98f4de1e855411eedeefda2c29d13bc5642718f82369370429b043a"),
("rel_COLON_PRECEDENCE_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; aridad; 3; }",P::En,"CD.UNEXPECTED_TOKEN",Some(100..101),"4a4b44b74a9c07a1c32a041046427c78285f3bcc8342cc5d241b8c2d93259910"),
("rel_EOF_PRECEDENCE_en","-- prólogo é: frame, fuente y fidelidad\r\n\tsemantic_relation Probe { kind: DeclaredRelation; aridad",P::En,"CD.UNEXPECTED_END",Some(100..100),"152ea57788547148e8c1cc5866887c4a7142fd296caa27580113682f808545ed"),
("pat_REPEAT_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; arity: 3; arity: 3; }",P::En,"CD.UNEXPECTED_TOKEN",Some(93..98),"dad94a50cbc922d0924c3cb0f3182162a31ca4f4e052bd6245f3773619644b85"),
("pat_ORDER_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; constraints: [Local]; arity: 3; }",P::En,"CD.UNEXPECTED_TOKEN",Some(105..110),"84103825fa63eea255b71fe531025a34daf27d07315fdafa75724144939d39b7"),
("pat_CONSTRAINTS_REPEAT_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; constraints: [Local]; constraints: [Local]; }",P::En,"CD.UNEXPECTED_TOKEN",Some(105..116),"61a84558f91b8a5f625c7f03ed25e71f4a4393bae9e66e4327e24dc73281084d"),
("pat_UNKNOWN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; inventado: 3; }",P::En,"CD.UNSUPPORTED",Some(83..92),"8b4e2ac940f5820f5c80ad2962675e8f732e0e839ed455dbb834041820aed2f1"),
("pat_FOREIGN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; aridad: 3; }",P::En,"CD.FOREIGN_SURFACE",Some(83..89),"7250f03d362a14966b83f65bfd9bf464501a364bfa324ab4020423309188b083"),
("pat_COLON_PRECEDENCE_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; aridad; 3; }",P::En,"CD.UNEXPECTED_TOKEN",Some(89..90),"69f51596bb836f6afd1d60e54fec79bde65a8118fc69fae9dfb2172e7307b524"),
("pat_EOF_PRECEDENCE_en","-- prólogo é: frame, fuente y fidelidad\r\n\tpattern Probe { kind: DeclaredPattern; aridad",P::En,"CD.UNEXPECTED_END",Some(89..89),"94c45613423f3b5a59aac359768ef6d137b7aa26544df68680626d10c257a9be"),
("SUPERVISE_UNKNOWN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = supervise(Meta, target: inventado(Ref));",P::En,"CD.UNSUPPORTED",Some(76..85),"419cf3c8029d1d85edaa900e423ba547ac5bf0086f7b9697b331e1c06de5f7c0"),
("SUPERVISE_FOREIGN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = supervise(Meta, target: ObjetivoCelda(Ref));",P::En,"CD.FOREIGN_SURFACE",Some(76..89),"d632d7762ea70c795b47f88e7ead8531a7ca78dc50346edbf0920a89edc8cd87"),
("SUPERVISE_PRECEDENCE_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = supervise(Meta, target: ObjetivoCelda(Ref);",P::En,"CD.UNEXPECTED_TOKEN",Some(94..95),"0d96145d75b7b23988f3f4e509e2fe91cd430167f830b1b4d7573d78ce8c9847"),
("QUERY_UNKNOWN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = query(Spec, by: AgentX, in: inventado(Ref));",P::En,"CD.UNSUPPORTED",Some(80..89),"bfba4e77e42d930f0b4e5eb0cb2149c6ef748bf3dc5e815223fa765c0e7b464b"),
("QUERY_FOREIGN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = query(Spec, by: AgentX, in: VistaEvaluaciónPuntual(Ref));",P::En,"CD.FOREIGN_SURFACE",Some(80..103),"0959672d7c992277c6cdaac57176683131ba978d58ff588a7fa1e378e2e65ba2"),
("QUERY_PRECEDENCE_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = query(Spec, by: AgentX, in: VistaEvaluaciónPuntual;",P::En,"CD.UNEXPECTED_TOKEN",Some(103..104),"a747424d530e55c7e870f28022f5a324df0d80ebc370756ca7a4db70a998e2cb"),
("LET_PROTECTED_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = codomain;",P::En,"CD.UNEXPECTED_TOKEN",Some(52..60),"c8dd93481e0485741de4b7c92b04fa61a8d67a242c056f2321b5e055e4f80568"),
("LET_FOREIGN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tlet X = codominio;",P::En,"CD.FOREIGN_SURFACE",Some(52..61),"81f6308173c266f0eef9d823b57760592f250fed18e00249f989fb729a6a53f9"),
("STATE_UNKNOWN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {inventado}; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(93..102),"f805b355115cd0f39316057cdc30f47a66b0e8c6a8a24cb9ef8349c9f89f9195"),
("STATE_FOREIGN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Admitido}; }",P::En,"CD.FOREIGN_SURFACE",Some(93..101),"948d7e483a5a97086b2a1f343130538346bee619af47d0084973a289db05417b"),
("STATE_TOKEN_en","-- prólogo é: frame, fuente y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {7}; }",P::En,"CD.UNEXPECTED_TOKEN",Some(93..94),"01f77ccb2b5c797e14a51478e9b998974f303e50cc1d000d12f36261945b980a"),
("STATE_CARDINALITY_PENDING_en","-- prólogo é: frame, fuente y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, Degraded}; rule: R; }",P::En,"CD.INVALID_ADMISSIBILITY_STATE",Some(92..106),"cbdb6fe1e0247cac1bd365f2d8e3889ede39a4679e35045064e5866c478da8fa"),
("STATE_VALID_en","-- prólogo é: frame, fuente y fidelidad\r\n\tadmissibility_spec A { parameter_id: 1; states: {Ok, Degraded, NotAdmitted}; rule: R; }",P::En,"OK",None,"9238f0576458221fdfa82d719e3a0aff9891ca8b38ec886b044ca26e62a5abb3"),
("rel_REPEAT_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; tabla: TabA; tabla: TabA; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(113..118),"ac0b1b842c47c20699c71f264672512d5aa355f414d22b8fb2b5e8e3f2deddf9"),
("rel_ORDER_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; restricciones: [Local]; tabla: TabA; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(124..129),"3b5e66a4cb47a6af376bf2e90376ce4bd5566b8b56cc2f9986590cc37812e18f"),
("rel_CONSTRAINTS_REPEAT_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; restricciones: [Local]; restricciones: [Local]; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(124..137),"362b8182130c20dde7b9c2f377bb4f1a174d881814cf71cf181148ecc2b2a257"),
("rel_UNKNOWN_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; inventado: 3; }",P::Es,"CD.UNSUPPORTED",Some(100..109),"35afeb124e78ebd4546c9a94e2e1367d9442d298b4ff2ecab969567f914c9114"),
("rel_FOREIGN_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; arity: 3; }",P::Es,"CD.FOREIGN_SURFACE",Some(100..105),"9c881424346f27e56ab6a6a26e4188931504fba1998a30e25bfa0c0844411387"),
("rel_COLON_PRECEDENCE_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; arity; 3; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(105..106),"4361c6f2b09a2c281fbcd7bb43820606fd01725d4ab0937a6e2c5a58b2a51ba4"),
("rel_EOF_PRECEDENCE_es","-- prólogo é: frame, fuente y fidelidad\r\n\trelación_semántica Probe { clase: RelaciónDeclarada; arity",P::Es,"CD.UNEXPECTED_END",Some(105..105),"7cfd0e030c262b20728f06326f6e2e73f9d2bf47570fde717f8896b7e2f8edfe"),
("pat_REPEAT_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; aridad: 3; aridad: 3; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(96..102),"7c5328e2b22436bc3678ab21d385083f4cbd0b09522a3fb282ac1fdc6e55f01e"),
("pat_ORDER_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; restricciones: [Local]; aridad: 3; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(109..115),"5617328601de4507c6a9909dc08998dac4cd1b34c31ae2161199a493266d3a0f"),
("pat_CONSTRAINTS_REPEAT_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; restricciones: [Local]; restricciones: [Local]; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(109..122),"73eeb06c2e17f71bba76d53ee405f0f48939ab11f0a78b7dd7a2f2ac087ab28c"),
("pat_UNKNOWN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; inventado: 3; }",P::Es,"CD.UNSUPPORTED",Some(85..94),"99078e876427a47d9b5829c06f70a983a11315a410baab1c3b9523aa098d6652"),
("pat_FOREIGN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; arity: 3; }",P::Es,"CD.FOREIGN_SURFACE",Some(85..90),"ef9607146f241744c00167c6390e7b17afc4e59d47c93c0bf9081fcf70d5b4d8"),
("pat_COLON_PRECEDENCE_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; arity; 3; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(90..91),"df10d6131aec93e0019c37957cb0a74eac2ceb292cd1e2edd57749d6286bb497"),
("pat_EOF_PRECEDENCE_es","-- prólogo é: frame, fuente y fidelidad\r\n\tpatrón Probe { clase: PatrónDeclarado; arity",P::Es,"CD.UNEXPECTED_END",Some(90..90),"189871872eb242988b5e8cff3a3102695157e4631e6a62da6e143261b7424535"),
("SUPERVISE_UNKNOWN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = supervisar(Meta, objetivo: inventado(Ref));",P::Es,"CD.UNSUPPORTED",Some(79..88),"4f1caba03de3103254fbb53fb94612f128bce0e604db735a67e0cd0e4dcfcd15"),
("SUPERVISE_FOREIGN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = supervisar(Meta, objetivo: CellTarget(Ref));",P::Es,"CD.FOREIGN_SURFACE",Some(79..89),"69ff24d6fa293ffceec2c860a38468c74035891ad01c0982cbab66cf818f2568"),
("SUPERVISE_PRECEDENCE_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = supervisar(Meta, objetivo: CellTarget(Ref);",P::Es,"CD.UNEXPECTED_TOKEN",Some(94..95),"24d97bf329ce5f8115476a9235a19ef06d62955ecbea7d58fd39d0af2b9ad796"),
("QUERY_UNKNOWN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = consultar(Spec, por: AgentX, en: inventado(Ref));",P::Es,"CD.UNSUPPORTED",Some(85..94),"7d7cd30929933c9481b2406bc1196af2156e7b739b646a72831b3e0a55a3f28f"),
("QUERY_FOREIGN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = consultar(Spec, por: AgentX, en: PointEval(Ref));",P::Es,"CD.FOREIGN_SURFACE",Some(85..94),"0372fb483d06a2e02dd4ba9929b7fdc56b68b366506c049c79a8778dd11a6988"),
("QUERY_PRECEDENCE_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = consultar(Spec, por: AgentX, en: PointEval;",P::Es,"CD.UNEXPECTED_TOKEN",Some(94..95),"21604b94278d12df2b43959bc4cb566cb0b6663dddda088988e62bb96c083476"),
("LET_PROTECTED_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = codominio;",P::Es,"CD.UNEXPECTED_TOKEN",Some(52..61),"1cda6d195fe3e5abbaed6ac7c43b8ba6e9bacac0d64e0b22b45d14b8db90a8f8"),
("LET_FOREIGN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tsea X = codomain;",P::Es,"CD.FOREIGN_SURFACE",Some(52..60),"059b1ac21e2e9b9dbe9e29244cf64d3c5bf4185b4cef9cbd860fea09c26cd21e"),
("STATE_UNKNOWN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {inventado}; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(123..132),"7907b2e1c98ae9414357a3c1467a8ce96eee56ac09069e1f3929a7ea10604c5a"),
("STATE_FOREIGN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Ok}; }",P::Es,"CD.FOREIGN_SURFACE",Some(123..125),"a06240376e09dabed64fc72446ca77b97e789eb9222c59cf1205c918d17d8604"),
("STATE_TOKEN_es","-- prólogo é: frame, fuente y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {7}; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(123..124),"13723ecaed993c0611b18d7decbf27e896a66b4b5db7d36a1740667f9a8339ca"),
("STATE_CARDINALITY_PENDING_es","-- prólogo é: frame, fuente y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, Degradado}; regla: R; }",P::Es,"CD.INVALID_ADMISSIBILITY_STATE",Some(122..143),"0c855d99464568be561ce14e9c68c5576efb4ffb01787285ba75a9b06bea56a8"),
("STATE_VALID_es","-- prólogo é: frame, fuente y fidelidad\r\n\tespecificación_de_admisibilidad A { identificador_de_parámetro: 1; estados: {Admitido, Degradado, NoAdmitido}; regla: R; }",P::Es,"OK",None,"0a3517634e14f9883c50e5e49cc41bbecf875fec40bb1e9c8ef1ce3d6a161e0d"),
("ROOT_FOREIGN_en","-- prólogo é\r\n\tcodominio K = {A};",P::En,"CD.FOREIGN_SURFACE",Some(17..26),"34bd75c32b59ec354a38bbe07c57440c6ede00159a70c21aa332d415f58378f5"),
("ROOT_UNKNOWN_en","-- prólogo é\r\n\tdesconocida K = {A};",P::En,"CD.UNSUPPORTED",Some(17..28),"03ba1c3c6c74068c36bfed1ca8e5cb8d1ded47fa7c7a8830e8dd4a02854fcec8"),
("ROOT_SYMBOL_en","-- prólogo é\r\n\t;",P::En,"CD.UNEXPECTED_TOKEN",Some(17..18),"bcce07bd0be442f9bf40705011f8b6dfc30705050899b5491399380fad690bb4"),
("IDENT_PROTECTED_en","codomain codomain = {A};",P::En,"CD.UNEXPECTED_TOKEN",Some(9..17),"25831afb2d68e7fe04ec6d49e65c3f76ca5a0754ac7c68b2cfaed6516dd9d3aa"),
("IDENT_FOREIGN_en","codomain codominio = {A};",P::En,"CD.FOREIGN_SURFACE",Some(9..18),"cf6362cc7b1af9618f4f8bd94c8cddc8fa580cb5b2abf07797aa607aa27f7036"),
("IDENT_TOKEN_en","codomain 7 = {A};",P::En,"CD.UNEXPECTED_TOKEN",Some(9..10),"c863a7a6ea2c33d21347d5da356e42a0e11677b6dabc001ca316f6b122fcef34"),
("SYMBOL_en","codomain K : {A};",P::En,"CD.UNEXPECTED_TOKEN",Some(11..12),"105484768b4da0a077d47930e371f4657e5a7d5f2c15625da8854f064ae6c729"),
("TEXT_en","output_semantics S { A -> 7; }",P::En,"CD.UNEXPECTED_TOKEN",Some(26..27),"522657d5beef5206e8dc32a9db01a91aedfa46556c6f8863b0c547d8b2752e9b"),
("ARROW_en","output_semantics S { A : \"dato\"; }",P::En,"CD.UNEXPECTED_TOKEN",Some(23..24),"98796cc602967bb9e6be4bbbd7bed12bcbd3894ffa2926bd1ee5de7c1e966633"),
("NAT_TYPE_en","cellspec C { b: palabra; }",P::En,"CD.UNEXPECTED_TOKEN",Some(16..23),"9c4549d33e8902c73f9cad306ee7152ad2939f5dd21b88d65a81c9645db81700"),
("FIELD_FOREIGN_en","cellspec C { b: 3; codominio: K; }",P::En,"CD.FOREIGN_SURFACE",Some(19..28),"4366cd29d8545beffc9d8292fb02d445bf92f96b9f32cd64bbcf835c449f43ce"),
("FIELD_UNKNOWN_en","cellspec C { b: 3; desconocido: K; }",P::En,"CD.UNEXPECTED_TOKEN",Some(19..30),"3d98f4773d8da5a5f1ff223671abd466e31639e0843888405bacd632dda22cc0"),
("TRI_en","cellstate Q { spec: C; vector: [Tal]; }",P::En,"CD.INVALID_TRI",Some(32..35),"a3c1a9c80fdd75c1feecd3503a664e69718d0a61d93029a017173ba8869f04de"),
("EOF_en","-- prólogo é\r\n\tcodomain K = {A",P::En,"CD.UNEXPECTED_END",Some(32..32),"5d5fb06bbc22958c217495948c86113a26972a495984ca7840f03d558c562cd0"),
("LEXICAL_en","-- prólogo é\r\n\t💣",P::En,"CD.UNEXPECTED_TOKEN",Some(17..21),"4dbe2b16f138ca72305f1984dc2042bf3d2998a90fef4483a43a7645f690b9d5"),
("CONTEXT_IDENTIFIER_en","codomain aridad = {A};",P::En,"OK",None,"fab7ac11c97097fc7bfbb2f02e51e47a1f2c222b2b65f356b80b9db983cdc368"),
("TEXT_DATA_en","output_semantics S { A -> \"codominio <script>SECRETO</script>\"; }",P::En,"OK",None,"d1264cb74d73c5c80659962c29d78fefa3b2fd732d2ea3d9dc5856eed6a4bb55"),
("COMMENT_en","-- codominio\r\ncodomain K = {A};",P::En,"OK",None,"1da89b4ef47c4ec585baca0e8b732460cca9a010ef0d5b6a796ed28af1b066d5"),
("VALID_en","codomain K = {A};",P::En,"OK",None,"9d7ed44ec1ed3d1f11619020fd20f0d1ab900f36dbdf2c8f013fae745a78bc8d"),
("UNMIGRATED_en","cellspec C { b: 2; }",P::En,"CD.UNEXPECTED_TOKEN",Some(19..20),"7623f9471db2dd92163b73ef442c34c9c392e3e06cb12cbd2def1785263963b3"),
("ROOT_FOREIGN_es","-- prólogo é\r\n\tcodomain K = {A};",P::Es,"CD.FOREIGN_SURFACE",Some(17..25),"ab71bdccf05826ffa8d50db877b1a5853908c66d96da16bd0db629d03eda78d2"),
("ROOT_UNKNOWN_es","-- prólogo é\r\n\tdesconocida K = {A};",P::Es,"CD.UNSUPPORTED",Some(17..28),"03ba1c3c6c74068c36bfed1ca8e5cb8d1ded47fa7c7a8830e8dd4a02854fcec8"),
("ROOT_SYMBOL_es","-- prólogo é\r\n\t;",P::Es,"CD.UNEXPECTED_TOKEN",Some(17..18),"bcce07bd0be442f9bf40705011f8b6dfc30705050899b5491399380fad690bb4"),
("IDENT_PROTECTED_es","codominio codominio = {A};",P::Es,"CD.UNEXPECTED_TOKEN",Some(10..19),"e10728c6ea4a4afb784d36f9c2648a48e8292f6c02fb50f0d82e723e4621da04"),
("IDENT_FOREIGN_es","codominio codomain = {A};",P::Es,"CD.FOREIGN_SURFACE",Some(10..18),"2ca2b273f6271bd55a4720c6a804782da51cae222512dc5fcf91294753d50c29"),
("IDENT_TOKEN_es","codominio 7 = {A};",P::Es,"CD.UNEXPECTED_TOKEN",Some(10..11),"550a4d3abf97f31cf59b6bf0af42af5b5ecbb7a54a42380a4a0eede579897567"),
("SYMBOL_es","codominio K : {A};",P::Es,"CD.UNEXPECTED_TOKEN",Some(12..13),"6925b3d75a43cc0b7971d4e7f62f32544bf8e8466c0e8151264e2490cc357351"),
("TEXT_es","semántica_de_salida S { A -> 7; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(30..31),"5b131401acec640ef2062606e5da914cb20f6d00f8808e1dc2af8aa509b644cb"),
("ARROW_es","semántica_de_salida S { A : \"dato\"; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(27..28),"9f7f745c822f80f31ff7ef7a0d34924766a673949e479b2cd3bb6295ae017cc0"),
("NAT_TYPE_es","especificación_de_celda C { b: palabra; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(32..39),"b4c25b48d6331b692140ac42e6e583e25bb7436a7098d90f4888afee2c75d2a6"),
("FIELD_FOREIGN_es","especificación_de_celda C { b: 3; codomain: K; }",P::Es,"CD.FOREIGN_SURFACE",Some(35..43),"66c000230f1e9cb560c51c0727b9fb54107ebdff1c11e1d2b775d3b411d9297e"),
("FIELD_UNKNOWN_es","especificación_de_celda C { b: 3; desconocido: K; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(35..46),"b5c2899c3f2defab8151d36dee77f3784982b1bca53744d76fea68e21aa2c236"),
("TRI_es","estado_de_celda Q { especificación: C; vector: [Tal]; }",P::Es,"CD.INVALID_TRI",Some(49..52),"90a61d5b9c5d7c60e5f2292ad194f25cc1173cb62775eced83104d47b3b70f1e"),
("EOF_es","-- prólogo é\r\n\tcodominio K = {A",P::Es,"CD.UNEXPECTED_END",Some(33..33),"181f804eaf466d3b234d558c7865b45fe55ccccf586b78ec46c2aba711ce63f9"),
("LEXICAL_es","-- prólogo é\r\n\t💣",P::Es,"CD.UNEXPECTED_TOKEN",Some(17..21),"4dbe2b16f138ca72305f1984dc2042bf3d2998a90fef4483a43a7645f690b9d5"),
("CONTEXT_IDENTIFIER_es","codominio arity = {A};",P::Es,"OK",None,"673443d943e39357ef62cf42deef7985d451717eb7159434523e9069b91cd29c"),
("TEXT_DATA_es","semántica_de_salida S { A -> \"codomain <script>SECRETO</script>\"; }",P::Es,"OK",None,"1a5c5e6ccd82be2d40cfba1acb6ef870af5dd19f9b5d89208048b943f49a82c4"),
("COMMENT_es","-- codomain\r\ncodominio K = {A};",P::Es,"OK",None,"c204e02aa76e09e51da71df22c8a1d71185cf2ab3b85f1094ee003feb07e1ca9"),
("VALID_es","codominio K = {A};",P::Es,"OK",None,"ca10f01c0d9754affe478789f11c463a0d64e413dda98314198d031035dbe7d1"),
("UNMIGRATED_es","especificación_de_celda C { b: 2; }",P::Es,"CD.UNEXPECTED_TOKEN",Some(35..36),"29a1318f6806bd47312d147253af897cfc9fda6df197842c9363ea1d6589448c")] {check(id,s,p,cause,span,digest);}}
