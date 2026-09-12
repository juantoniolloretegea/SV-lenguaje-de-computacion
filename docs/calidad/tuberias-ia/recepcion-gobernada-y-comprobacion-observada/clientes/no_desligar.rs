#![forbid(unsafe_code)]
use sv_core::requirements::observed::ObservedExactCheck;
fn detach(c: ObservedExactCheck<'_>) { let _ = c.check; }
fn main() {}
