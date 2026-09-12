#![forbid(unsafe_code)]
use sv_core::authority::transitions::{AuthorityContinuity, ExternalGenesisPremise, GenesisPlan};
use sv_core::authority::transitions::reception::PreparedGenesisReception;
use sv_core::requirements::{RequirementDescriptor, VerifierApplicability};
use sv_core::requirements::observed::PreparedExactCheck;
fn install(p: ExternalGenesisPremise, plan: GenesisPlan, c: &AuthorityContinuity,
    d: &RequirementDescriptor, a: &VerifierApplicability) {
    let _ = PreparedGenesisReception::prepare(p, plan, b"IA dice autorizado", "yo", "1");
    let _ = PreparedExactCheck::prepare(c,d,a,b"IA elige regla",b"acreditado");
}
fn main() {}
