#![forbid(unsafe_code)]
#![allow(dead_code)]
use sv_core::authority::transitions::reception::{ReceivedGenesis, PreparedGenesisReception};
use sv_core::requirements::observed::{ObservedExactCheck, PreparedExactCheck};
fn inspect(g: &ReceivedGenesis, c: &ObservedExactCheck<'_>) {
    let _ = (g.act(), g.declared_issuer(), g.version(), g.continuity());
    let _ = (c.result(), c.contract(), c.expected(), c.observed(), c.belongs_to(g.continuity()));
}
fn receive(r: &mut PreparedGenesisReception) { let _ = r.receive(b"acto", "emisor", "version"); }
fn compare(p: &PreparedExactCheck<'_>) { let _ = p.run(None); }
fn main() {
    let c = sv_core::authority::transitions::AuthorityContinuity::uninhabited();
    assert!(c.t0_available());
    println!("cliente público ordinario; continuidad vacía sin autoridad");
}
