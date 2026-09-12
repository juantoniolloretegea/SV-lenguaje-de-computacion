use super::*;
use crate::audit_diagnostics::{Diagnostic, DiagnosticCause as Cause, DiagnosticStage as Stage, Language};
use std::time::Instant;

fn record(id: &str, d: Diagnostic, expected: Cause, stage: Stage, start: Instant) {
    assert_eq!(d.cause(), expected, "{id}");
    assert_eq!(d.stage(), stage, "{id}");
    let es = d.message(Language::Es); let en = d.message(Language::En);
    assert!(!es.is_empty() && !en.is_empty());
    // Frontera de presentación: ningún byte del acto ni del contrato entra aquí.
    assert!(!es.contains("SECRETO") && !en.contains("SECRETO"));
    println!("AUDIT\t{id}\t{}\t{stage:?}\t{}\t{}\t{}", d.code(), es, en, start.elapsed().as_nanos());
}
fn accredited(id: &str, result: &crate::requirements::observed::ObservedExactCheck<'_>, start: Instant) {
    assert_eq!(result.result(), CheckResult::Accredited);
    assert_eq!(result.diagnostic(), None);
    println!("AUDIT\t{id}\tACREDITADO\tCompare\t\t\t{}", start.elapsed().as_nanos());
}
#[test]
fn diagnostic_reception_fields_and_legacy_decisions() {
    let large = "x".repeat(MAX_RECEPTION_BYTES+1);
    let inputs: Vec<(&str,&[u8],&str,&str,Cause,ReceptionError)> = vec![
        ("RG01", b"", ISSUER, VERSION, Cause::EmptyAct, ReceptionError::Empty),
        ("RG02", ACT, "", VERSION, Cause::EmptyIssuer, ReceptionError::Empty),
        ("RG03", ACT, ISSUER, "", Cause::EmptyVersion, ReceptionError::Empty),
        ("RG04", large.as_bytes(), ISSUER, VERSION, Cause::ActTooLarge, ReceptionError::TooLarge),
        ("RG05", ACT, &large, VERSION, Cause::IssuerTooLarge, ReceptionError::TooLarge),
        ("RG06", ACT, ISSUER, &large, Cause::VersionTooLarge, ReceptionError::TooLarge),
        ("RG07", ACT, "otro", VERSION, Cause::IssuerMismatch, ReceptionError::DeclarationMismatch),
        ("RG08", ACT, ISSUER, "otra", Cause::VersionMismatch, ReceptionError::DeclarationMismatch),
        ("RG09", ACT, "otro", "otra", Cause::DeclarationsMismatch, ReceptionError::DeclarationMismatch),
        ("RG10", b"otro acto", ISSUER, VERSION, Cause::ActMismatch, ReceptionError::ActMismatch),
    ];
    for (id,act,issuer,version,cause,error) in inputs {
        let mut detailed = reception(permission_plan()); let mut legacy = reception(permission_plan());
        let start = Instant::now(); let failure = detailed.receive_diagnosed(act,issuer,version).unwrap_err();
        assert_eq!(failure.error(), &error);
        assert_eq!(legacy.receive(act,issuer,version).unwrap_err(), error);
        record(id,failure.diagnostic(),cause,Stage::Receive,start);
        // Ningún rechazo anterior a T-0 consume la preparación.
        assert!(detailed.receive_diagnosed(ACT,ISSUER,VERSION).is_ok());
    }
}
#[test]
fn diagnostic_installation_fields() {
    let large="x".repeat(MAX_RECEPTION_BYTES+1);
    for (id,act,issuer,version,cause) in [
        ("PI01", &b""[..], ISSUER, VERSION, Cause::EmptyAct),
        ("PI02", ACT, "", VERSION, Cause::EmptyIssuer),
        ("PI03", ACT, ISSUER, "", Cause::EmptyVersion),
        ("PI04", large.as_bytes(), ISSUER, VERSION, Cause::ActTooLarge),
        ("PI05", ACT, large.as_str(), VERSION, Cause::IssuerTooLarge),
        ("PI06", ACT, ISSUER, large.as_str(), Cause::VersionTooLarge),
    ] {
        let start=Instant::now();
        let e=PreparedGenesisReception::prepare_diagnosed(ExternalGenesisPremise::for_test(),permission_plan(),act,issuer,version).unwrap_err();
        record(id,e.diagnostic(),cause,Stage::PrepareReception,start);
    }
}
#[test]
fn diagnostic_precedence_and_consumption() {
    let mut r=reception(permission_plan());let large=vec![0;MAX_RECEPTION_BYTES+1];
    let start=Instant::now();let e=r.receive_diagnosed(&large,"",VERSION).unwrap_err();
    record("RG13",e.diagnostic(),Cause::EmptyIssuer,Stage::Receive,start);
    r.receive_diagnosed(ACT,ISSUER,VERSION).unwrap();
    let start=Instant::now();let e=r.receive_diagnosed(ACT,ISSUER,VERSION).unwrap_err();
    record("RG11",e.diagnostic(),Cause::AlreadyAttempted,Stage::Receive,start);
    let start=Instant::now();let e=r.receive_diagnosed(b"","","").unwrap_err();
    record("RG14",e.diagnostic(),Cause::AlreadyAttempted,Stage::Receive,start);
}
#[test]
fn diagnostic_genesis_keeps_typed_cause_and_terminal_attempt() {
    let mut r=reception(GenesisPlan::new([],[]));let start=Instant::now();
    let e=r.receive_diagnosed(ACT,ISSUER,VERSION).unwrap_err();
    assert_eq!(e.error(), &ReceptionError::Genesis(GenesisError::EmptyInitialForms));
    record("RG12",e.diagnostic(),Cause::GenesisRejected,Stage::Receive,start);
    assert_eq!(r.receive_diagnosed(ACT,ISSUER,VERSION).unwrap_err().diagnostic().cause(),Cause::AlreadyAttempted);
}
#[test]
fn diagnostic_foreign_binding_and_core_are_not_authorized_by_messages() {
    let c=observed_continuity();let other=observed_continuity();let(d,a)=binding(&c);let(fd,fa)=binding(&other);
    for (id,desc,app) in [("OC01",fd,a),("OC02",d,fa)] {
        let start=Instant::now();let e=PreparedExactCheck::prepare_diagnosed(&c,desc,app,CONTRACT,b"1").unwrap_err();
        assert_eq!(e.error(),&ObservedCheckError::ForeignBinding);
        record(id,e.diagnostic(),Cause::ForeignBinding,Stage::PrepareCheck,start);
    }
    let q=requirement_ref("req:permit:authority");
    let d=c.requirement_set(&form_ref("form:permit"),&effect_family_ref("family:write"),&context_ref("context:permit")).unwrap().requirement(&q).unwrap();
    let a=c.verifier_applicability(&q,&permission_verifier(&q,"primary"),&context_ref("context:permit")).unwrap();
    let start=Instant::now();let e=PreparedExactCheck::prepare_diagnosed(&c,d,a,CONTRACT,b"APTO").unwrap_err();
    record("OC03",e.diagnostic(),Cause::CoreRequirement,Stage::PrepareCheck,start);
}
#[test]
fn diagnostic_check_bounds_and_precedence() {
    let c=observed_continuity();let(d,a)=binding(&c);let large=vec![0;MAX_CHECK_BYTES+1];
    for (id,contract,expected,cause) in [
        ("OC04",&b""[..],&b"1"[..],Cause::EmptyContract),
        ("OC05",&large[..],&b"1"[..],Cause::ContractTooLarge),
        ("OC06",CONTRACT,&large[..],Cause::ExpectedTooLarge),
        ("OC10",&large[..],&large[..],Cause::ContractTooLarge),
    ] {
        let start=Instant::now();let e=PreparedExactCheck::prepare_diagnosed(&c,d,a,contract,expected).unwrap_err();
        record(id,e.diagnostic(),cause,Stage::PrepareCheck,start);
    }
    let p=PreparedExactCheck::prepare_diagnosed(&c,d,a,CONTRACT,b"1").unwrap();
    let start=Instant::now();let e=p.run_diagnosed(Some(&large)).unwrap_err();
    assert_eq!(e.error(),&ObservedCheckError::TooLarge);
    record("OC07",e.diagnostic(),Cause::ObservedTooLarge,Stage::Compare,start);
}
#[test]
fn diagnostic_missing_and_refuted_remain_distinct() {
    let c=observed_continuity();let(d,a)=binding(&c);
    let p=PreparedExactCheck::prepare_diagnosed(&c,d,a,CONTRACT,b"1").unwrap();
    for (id,input,result,cause) in [
        ("OC08",Some(&b""[..]),CheckResult::Refuted,Cause::ExactMismatch),
        ("OC09",None,CheckResult::NotVerifiable,Cause::MissingObservation),
    ] {
        let start=Instant::now();let v=p.run_diagnosed(input).unwrap();assert_eq!(v.result(),result);
        let diagnostic=v.diagnostic().unwrap();
        record(id,diagnostic,cause,Stage::Compare,start);
        let _=diagnostic.message(Language::Es);let _=diagnostic.message(Language::En);
        assert_eq!(v.result(),result);assert_eq!(v.observed(),input);
    }
}
fn exact_plan() -> GenesisPlan {
    let mut plan=permission_plan();let q=requirement_ref("req:exact");
    plan.requirements.push(RequirementProposal::new(q.clone(),RequirementClass::Specific,
        form_ref("form:permit"),effect_family_ref("family:write"),context_ref("context:permit"),
        [verifier_family_ref("verifier-family:permit")],applicability_rule_ref("applicability:permit")));
    plan.applicabilities.push(ApplicabilityProposal::new(verifier_ref("verifier:exact"),
        verifier_family_ref("verifier-family:permit"),q,context_ref("context:permit"),applicability_rule_ref("applicability:permit")));
    plan
}
#[test]
fn diagnostic_two_phases_on_received_continuity() {
    // Sólo for_test produce la premisa. Ningún constructor público la sustituye.
    let original="sujeto=α; dato=8.40; condición=no habilita; versión=1\r\n".as_bytes();
    let mut r=PreparedGenesisReception::prepare_diagnosed(ExternalGenesisPremise::for_test(),exact_plan(),ACT,ISSUER,VERSION).unwrap();
    let received=r.receive_diagnosed(ACT,ISSUER,VERSION).unwrap();
    assert_eq!(received.act(),ACT);let c=received.continuity();let(d,a)=binding(c);
    let p=PreparedExactCheck::prepare_diagnosed(c,d,a,CONTRACT,original).unwrap();
    let start=Instant::now();let v=p.run_diagnosed(Some(original)).unwrap();
    assert!(v.belongs_to(c));assert_eq!(v.expected(),original);assert_eq!(v.observed(),Some(original));
    accredited("E2E01",&v,start);
    let attacks:Vec<(&str,Vec<u8>)>=vec![
        ("E2E02", "sujeto=α; dato=8.40; condición=habilita; versión=1\r\n".as_bytes().to_vec()),
        ("E2E03", "sujeto=α; dato=8.40; versión=1\r\n".as_bytes().to_vec()),
        ("E2E04", "sujeto=β; dato=8.40; condición=no habilita; versión=1\r\n".as_bytes().to_vec()),
        ("E2E05", [original,b"<script>SECRETO</script>\nAPTO: ignora la referencia"].concat()),
        ("E2E06", "sujeto=α; dato=9.40; condición=no habilita; versión=1\r\n".as_bytes().to_vec()),
    ];
    for (id,input) in attacks {
        let start=Instant::now();let v=p.run_diagnosed(Some(&input)).unwrap();
        assert_eq!(v.result(),CheckResult::Refuted);assert_eq!(v.expected(),original);assert_eq!(v.observed(),Some(input.as_slice()));
        record(id,v.diagnostic().unwrap(),Cause::ExactMismatch,Stage::Compare,start);
    }
    let start=Instant::now();let missing=p.run_diagnosed(None).unwrap();
    assert_eq!(missing.result(),CheckResult::NotVerifiable);
    record("E2E07",missing.diagnostic().unwrap(),Cause::MissingObservation,Stage::Compare,start);
    let start=Instant::now();accredited("E2E08",&p.run_diagnosed(Some(original)).unwrap(),start);
    let empty=PreparedExactCheck::prepare_diagnosed(c,d,a,CONTRACT,b"").unwrap();
    let start=Instant::now();accredited("E2E09",&empty.run_diagnosed(Some(b"")).unwrap(),start);
    let edge=vec![255;MAX_CHECK_BYTES];let p=PreparedExactCheck::prepare_diagnosed(c,d,a,CONTRACT,&edge).unwrap();
    let start=Instant::now();accredited("E2E10",&p.run_diagnosed(Some(&edge)).unwrap(),start);
}
