use sv_core::{ProtectedDecisionContinuity, TracedPermit, TracedMediatedCommitment};
fn main(){
 let c=ProtectedDecisionContinuity::from_authority(sv_core::authority::transitions::AuthorityContinuity::uninhabited());
 assert_eq!(c.decision_trace_count(),0);assert_eq!(c.authority().authority_count(),0);
 println!("{{\"continuidad_bytes\":{},\"permiso_bytes\":{},\"compromiso_bytes\":{},\"decision_count\":0,\"autoridades\":0}}",std::mem::size_of::<ProtectedDecisionContinuity>(),std::mem::size_of::<TracedPermit>(),std::mem::size_of::<TracedMediatedCommitment>());
}
