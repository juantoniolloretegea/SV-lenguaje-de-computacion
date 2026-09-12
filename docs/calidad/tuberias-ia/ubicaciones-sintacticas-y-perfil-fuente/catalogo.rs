use sv_core::compiler_diagnostics::{Cause as C,FrontendCause as F,Stage,DeclarationId as D};
use sv_core::audit_diagnostics::Language as L;
fn main(){
 assert_eq!(sv_core::compiler_diagnostics::VERSION,"COMPILER-DIAGNOSTICS/2");
 for (s,p) in [("pattern P { kind: Simple; arity: 3; arity: 4; }",sv_core::SourceProfile::En),
 ("patrón P { clase: Simple; aridad: 3; aridad: 4; }",sv_core::SourceProfile::Es)] {
  let report=sv_core::compiler_diagnostics::compile(s,"original.svp",p).unwrap_err();
  assert_eq!(report.cause().id(),"CD.UNEXPECTED_TOKEN");assert_eq!(report.sites()[0].span,None);
 }

 for c in [C::EmptyCodomain{name:String::new()},C::DuplicateCodomainMembers{name:String::new(),duplicates:vec![]},
 C::OutputSemanticsKeys{cell:None,semantics:String::new(),codomain:None,duplicates:vec![],missing:vec![],extra:vec![]},
 C::DuplicateIdentifier{name:String::new(),first:D::Object(0),second:D::Object(1)},
 C::Frontend(F::ForeignSurface),C::Frontend(F::UnexpectedEnd),C::Frontend(F::UnexpectedToken),C::Frontend(F::Unsupported),C::Frontend(F::InvalidNatural),
 C::Frontend(F::InvalidAdmissibilityState),C::Frontend(F::InvalidTri),C::InsufficientAssemblyUnits{actual:0},C::Unmigrated{stage:Stage::Wellformed}] {
 println!("CAT\t{}\t{}\t{}\t{}",c.id(),c.code().unwrap_or(""),c.message(L::Es),c.message(L::En));
 }
}
