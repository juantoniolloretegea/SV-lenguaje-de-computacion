use sv_core::authority::transitions::{ExternalGenesisPremise, GenesisPlan, reception::PreparedGenesisReception};
fn forbidden(premise: ExternalGenesisPremise, plan: GenesisPlan) { let _ = PreparedGenesisReception::prepare(premise, plan, b"act", "issuer", "version"); }
fn main() {}