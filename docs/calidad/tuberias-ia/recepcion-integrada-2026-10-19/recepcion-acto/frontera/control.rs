use sv_core::authority::transitions::{AuthorityContinuity, reception::{ReceivedGenesis, MAX_RECEPTION_BYTES}};
use sv_core::requirements::observed::{PreparedExactCheck, ObservedExactCheck, MAX_CHECK_BYTES};
fn view(received: &ReceivedGenesis) { let _ = (received.act(), received.trace(), received.continuity()); }
fn observe<'a>(prepared: &PreparedExactCheck<'a>) -> ObservedExactCheck<'a> { prepared.run(None).unwrap() }
fn main() { let c = AuthorityContinuity::uninhabited(); assert_eq!(c.form_count(), 0); assert_eq!(MAX_RECEPTION_BYTES, MAX_CHECK_BYTES); let _ = view; let _ = observe; }