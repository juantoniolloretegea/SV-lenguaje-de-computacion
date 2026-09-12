use sv_core::compiler_diagnostics::{Cause as C,FrontendCause as F,Stage,DeclarationId as D};
use sv_core::audit_diagnostics::Language as L;
fn main(){
 for c in [C::EmptyCodomain{name:String::new()},C::DuplicateCodomainMembers{name:String::new(),duplicates:vec![]},
 C::OutputSemanticsKeys{cell:None,semantics:String::new(),codomain:None,duplicates:vec![],missing:vec![],extra:vec![]},
 C::DuplicateIdentifier{name:String::new(),first:D::Object(0),second:D::Object(1)},
 C::Frontend(F::UnexpectedEnd),C::Frontend(F::UnexpectedToken),C::Frontend(F::Unsupported),C::Frontend(F::InvalidNatural),
 C::Frontend(F::InvalidAdmissibilityState),C::Frontend(F::InvalidTri),C::InsufficientAssemblyUnits{actual:0},C::Unmigrated{stage:Stage::Wellformed}] {
 println!("CAT\t{}\t{}\t{}\t{}",c.id(),c.code().unwrap_or(""),c.message(L::Es),c.message(L::En));
 }
}
