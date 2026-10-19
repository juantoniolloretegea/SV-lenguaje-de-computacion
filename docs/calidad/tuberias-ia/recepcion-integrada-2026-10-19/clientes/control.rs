use sv_core::authority::transitions::AuthorityContinuity;
use sv_core::ProtectedDecisionContinuity;

fn main() {
    let continuity =
        ProtectedDecisionContinuity::from_authority(AuthorityContinuity::uninhabited());
    assert_eq!(continuity.decision_trace_count(), 0);
    assert_eq!(continuity.exercise_event_count(), 0);
}
