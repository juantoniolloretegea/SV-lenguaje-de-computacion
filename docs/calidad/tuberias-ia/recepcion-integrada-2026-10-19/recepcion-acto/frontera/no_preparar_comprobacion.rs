use sv_core::{authority::transitions::AuthorityContinuity, requirements::{RequirementDescriptor, VerifierApplicability, observed::PreparedExactCheck}};
fn forbidden(c: &AuthorityContinuity, d: &RequirementDescriptor, a: &VerifierApplicability) { let _ = PreparedExactCheck::prepare(c, d, a, b"contract", b"expected"); }
fn main() {}