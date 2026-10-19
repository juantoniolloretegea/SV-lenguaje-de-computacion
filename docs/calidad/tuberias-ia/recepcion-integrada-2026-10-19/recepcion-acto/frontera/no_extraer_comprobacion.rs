use sv_core::requirements::observed::ObservedExactCheck;
fn forbidden(e: ObservedExactCheck<'_>) { let _ = e.check; }
fn main() {}