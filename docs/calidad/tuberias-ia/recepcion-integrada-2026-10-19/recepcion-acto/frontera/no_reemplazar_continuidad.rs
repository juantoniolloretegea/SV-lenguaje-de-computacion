use sv_core::authority::transitions::reception::ReceivedGenesis;
use sv_core::ProtectedDecisionContinuity;
fn forbidden(received: &mut ReceivedGenesis, other: ProtectedDecisionContinuity) { received.continuity = other; }
fn main() {}