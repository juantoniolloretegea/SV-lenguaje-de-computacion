//! ES: Datos y claves exclusivamente sintéticos; sin red ni efectos externos.
//! EN: Synthetic data and keys only; no network or external effects.
use super::*;
use ed25519_dalek::{Signer, SigningKey};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(1);
fn issuer() -> SigningKey {
    SigningKey::from_bytes(&[1; 32])
}
fn proposer() -> SigningKey {
    SigningKey::from_bytes(&[2; 32])
}
fn mandate() -> InstitutionalMandate {
    InstitutionalMandate::new(MandateDefinition {
        institution: "institution-synthetic",
        issuer: "issuer-synthetic",
        domain: "domain-A",
        context: "context-A",
        operation: "deliver-checked",
        object: "object-A",
        edition: "v1",
        mandate_id: &format!("test-{}", NEXT.fetch_add(1, Ordering::Relaxed)),
        proposer_key: proposer().verifying_key().to_bytes(),
        valid_from: 100,
        valid_until: 200,
        equality_contract: b"exact-synthetic-reference-v1",
        expected: b"evidence-synthetic",
    })
    .unwrap()
}
fn installation(m: InstitutionalMandate) -> InstitutionalInstallation {
    InstitutionalInstallation::configure(m, issuer().verifying_key().to_bytes()).unwrap()
}
fn active() -> AdmittedInstitutionalOperation {
    let m = mandate();
    let signature = issuer().sign(&m.signing_bytes()).to_bytes();
    installation(m)
        .admit_at(signature, Ok(100), Instant::now())
        .unwrap()
}
fn request() -> InstitutionalInvocation<'static> {
    InstitutionalInvocation {
        domain: "domain-A",
        context: "context-A",
        operation: "deliver-checked",
        object: "object-A",
        nonce: [3; 16],
        observed: Some(b"evidence-synthetic"),
    }
}
fn signed(r: &InstitutionalInvocation<'_>, a: &AdmittedInstitutionalOperation) -> [u8; 64] {
    proposer()
        .sign(&r.signing_bytes(a.mandate()).unwrap())
        .to_bytes()
}
#[derive(Default)]
struct Recorder {
    calls: usize,
    evidence: Vec<u8>,
    fail: bool,
}
impl InstitutionalExecutor for Recorder {
    fn execute(
        &mut self,
        request: &ExecutionRequest<'_>,
        evidence: &[u8],
    ) -> Result<(), AdapterFailure> {
        assert_eq!(request.effect().object().id().as_str(), "object-A");
        self.calls += 1;
        self.evidence = evidence.to_vec();
        if self.fail {
            Err(AdapterFailure::Indeterminate)
        } else {
            Ok(())
        }
    }
}
fn run<A: InstitutionalExecutor>(
    a: &mut AdmittedInstitutionalOperation,
    r: &InstitutionalInvocation<'_>,
    s: [u8; 64],
    recorder: &mut A,
) -> Result<InstitutionalOutcome, AdmissionError> {
    a.perform_at(r, s, recorder, Ok(100), Instant::now())
}

#[test]
fn ia01_signed_admission_checks_and_actual_delivery() {
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let mut e = Recorder::default();
    assert!(matches!(
        run(&mut a, &r, s, &mut e),
        Ok(InstitutionalOutcome::Confirmed(_))
    ));
    assert_eq!(e.calls, 1);
    assert_eq!(e.evidence, b"evidence-synthetic");
    assert_eq!(a.trace().decision_trace_count(), 1);
    assert_eq!(a.trace().exercise_event_count(), 2);
    assert_eq!(a.audit().last().unwrap().outcome(), AuditOutcome::Confirmed);
    assert_eq!(
        a.audit().last().unwrap().request_bytes(),
        Some(&*r.signing_bytes(a.mandate()).unwrap())
    );
}

#[test]
fn ia02_bad_issuer_signature_cannot_admit_and_does_not_consume() {
    let m = mandate();
    let good = issuer().sign(&m.signing_bytes()).to_bytes();
    let bad = proposer().sign(&m.signing_bytes()).to_bytes();
    let mut i = installation(m);
    assert_eq!(
        i.admit_at(bad, Ok(100), Instant::now()).unwrap_err(),
        AdmissionError::InvalidSignature
    );
    assert_eq!(
        i.admit_at([0; 64], Ok(100), Instant::now()).unwrap_err(),
        AdmissionError::InvalidSignature
    );
    assert!(i.admit_at(good, Ok(100), Instant::now()).is_ok());
}

#[test]
fn ia03_each_signed_mandate_field_is_bound() {
    for index in 0..13 {
        let m = mandate();
        let signature = issuer().sign(&m.signing_bytes()).to_bytes();
        let mut altered = m.clone();
        match index {
            0..=7 => altered.labels[index] = format!("other-{index}").into(),
            8 => altered.valid_from = 99,
            9 => altered.valid_until = 201,
            10 => altered.contract = b"other-contract".as_slice().into(),
            11 => altered.expected = b"other-reference".as_slice().into(),
            12 => altered.proposer = SigningKey::from_bytes(&[4; 32]).verifying_key(),
            _ => unreachable!(),
        }
        assert_eq!(
            installation(altered)
                .admit_at(signature, Ok(100), Instant::now())
                .unwrap_err(),
            AdmissionError::InvalidSignature,
            "field {index}"
        );
    }
}

#[test]
fn ia04_duplicate_admission_rejected_across_installation_objects() {
    let m = mandate();
    let s = issuer().sign(&m.signing_bytes()).to_bytes();
    let mut first = installation(m.clone());
    first.admit_at(s, Ok(100), Instant::now()).unwrap();
    assert_eq!(
        first.admit_at(s, Ok(100), Instant::now()).unwrap_err(),
        AdmissionError::AlreadyAdmitted
    );
    assert_eq!(
        installation(m)
            .admit_at(s, Ok(100), Instant::now())
            .unwrap_err(),
        AdmissionError::AlreadyAdmitted
    );
}

#[test]
fn ia05_validity_interval_is_half_open_and_clock_failure_rejects() {
    for (clock, expected) in [
        (Ok(99), AdmissionError::NotYetValid),
        (Ok(200), AdmissionError::Expired),
        (
            Err(AdmissionError::ClockUnavailable),
            AdmissionError::ClockUnavailable,
        ),
    ] {
        let m = mandate();
        let s = issuer().sign(&m.signing_bytes()).to_bytes();
        assert_eq!(
            installation(m)
                .admit_at(s, clock, Instant::now())
                .unwrap_err(),
            expected
        );
    }
}

#[test]
fn ia06_self_accreditation_and_weak_keys_rejected() {
    assert!(matches!(
        InstitutionalInstallation::configure(mandate(), proposer().verifying_key().to_bytes()),
        Err(AdmissionError::SelfAccreditation)
    ));
    assert!(matches!(
        InstitutionalInstallation::configure(mandate(), [0; 32]),
        Err(AdmissionError::InvalidKey)
    ));
    let mut m = mandate();
    m.proposer = issuer().verifying_key();
    assert!(matches!(
        InstitutionalInstallation::configure(m, issuer().verifying_key().to_bytes()),
        Err(AdmissionError::SelfAccreditation)
    ));
}

#[test]
fn ia07_signed_request_cannot_select_another_scope() {
    for index in 0..4 {
        let mut a = active();
        let mut r = request();
        match index {
            0 => r.domain = "domain-B",
            1 => r.context = "context-B",
            2 => r.operation = "other-operation",
            _ => r.object = "object-B",
        }
        let s = signed(&r, &a);
        let mut e = Recorder::default();
        assert_eq!(
            run(&mut a, &r, s, &mut e).unwrap_err(),
            AdmissionError::ScopeMismatch
        );
        assert_eq!(e.calls, 0);
    }
}

#[test]
fn ia08_bad_request_signature_and_cross_mandate_replay_rejected() {
    let mut a = active();
    let b = active();
    let r = request();
    let mut e = Recorder::default();
    let foreign = signed(&r, &b);
    assert_eq!(
        run(&mut a, &r, foreign, &mut e).unwrap_err(),
        AdmissionError::InvalidSignature
    );
    let issuer_signed = issuer()
        .sign(&r.signing_bytes(a.mandate()).unwrap())
        .to_bytes();
    assert_eq!(
        run(&mut a, &r, issuer_signed, &mut e).unwrap_err(),
        AdmissionError::InvalidSignature
    );
    assert_eq!(e.calls, 0);
}

#[test]
fn ia09_absent_and_refuted_evidence_never_dispatch() {
    for (observed, expected) in [
        (None, CheckResult::NotVerifiable),
        (Some(b"changed".as_slice()), CheckResult::Refuted),
    ] {
        let mut a = active();
        let mut r = request();
        r.observed = observed;
        let s = signed(&r, &a);
        let mut e = Recorder::default();
        assert!(
            matches!(run(&mut a, &r, s, &mut e), Ok(InstitutionalOutcome::Blocked(result)) if result == expected)
        );
        assert_eq!(e.calls, 0);
        assert_eq!(a.trace().decision_trace_count(), 1);
    }
}

#[test]
fn ia10_revocation_is_signed_separated_and_irreversible() {
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let wrong = issuer().sign(&a.mandate().signing_bytes()).to_bytes();
    assert_eq!(a.revoke(wrong), Err(AdmissionError::InvalidSignature));
    let rev = issuer().sign(&a.mandate().revocation_bytes()).to_bytes();
    a.revoke(rev).unwrap();
    a.revoke(rev).unwrap();
    let mut e = Recorder::default();
    assert_eq!(
        run(&mut a, &r, s, &mut e).unwrap_err(),
        AdmissionError::Revoked
    );
    assert_eq!(e.calls, 0);
    assert_eq!(a.revocation().unwrap().0, rev);
}

#[test]
fn ia11_repeated_dispatch_is_blocked() {
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let mut e = Recorder::default();
    run(&mut a, &r, s, &mut e).unwrap();
    assert_eq!(
        run(&mut a, &r, s, &mut e).unwrap_err(),
        AdmissionError::AlreadyDispatched
    );
    assert_eq!(e.calls, 1);
}

#[test]
fn ia12_adapter_failure_is_indeterminate_and_cannot_retry() {
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let mut e = Recorder {
        fail: true,
        ..Recorder::default()
    };
    assert_eq!(
        run(&mut a, &r, s, &mut e).unwrap_err(),
        AdmissionError::AdapterIndeterminate
    );
    assert_eq!(
        a.audit().last().unwrap().outcome(),
        AuditOutcome::Indeterminate
    );
    assert_eq!(
        run(&mut a, &r, s, &mut e).unwrap_err(),
        AdmissionError::AlreadyDispatched
    );
    assert_eq!(e.calls, 1);
}

#[test]
fn ia13_expiry_during_verification_prevents_dispatch() {
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let mut e = Recorder::default();
    let mut calls = 0;
    let result = a.perform_with_clock(&r, s, &mut e, || {
        calls += 1;
        (Ok(if calls == 1 { 199 } else { 200 }), Instant::now())
    });
    assert_eq!(result.unwrap_err(), AdmissionError::Expired);
    assert_eq!(e.calls, 0);
    assert_eq!(calls, 2);
}

#[test]
fn ia14_clock_rollback_and_monotonic_expiry_prevent_dispatch() {
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let mut e = Recorder::default();
    assert_eq!(
        a.perform_at(&r, s, &mut e, Ok(99), Instant::now())
            .unwrap_err(),
        AdmissionError::ClockRollback
    );
    let deadline = a.deadline;
    assert_eq!(
        a.perform_at(&r, s, &mut e, Ok(100), deadline).unwrap_err(),
        AdmissionError::Expired
    );
    assert_eq!(e.calls, 0);
}

#[test]
fn ia15_audit_quota_blocks_work_but_never_revocation() {
    let mut a = active();
    let mut r = request();
    r.observed = None;
    let mut e = Recorder::default();
    let good = signed(&r, &a);
    for _ in 1..MAX_AUDIT_ENTRIES {
        assert!(matches!(
            run(&mut a, &r, good, &mut e),
            Ok(InstitutionalOutcome::Blocked(CheckResult::NotVerifiable))
        ));
    }
    assert_eq!(
        run(&mut a, &r, good, &mut e).unwrap_err(),
        AdmissionError::AuditFull
    );
    let rev = issuer().sign(&a.mandate().revocation_bytes()).to_bytes();
    a.revoke(rev).unwrap();
    assert_eq!(a.audit().len(), MAX_AUDIT_ENTRIES);
    assert_eq!(e.calls, 0);
}
#[test]
fn ia16_request_limits_are_checked_before_copy_or_dispatch() {
    let mut a = active();
    let bytes = vec![0; MAX_RECEPTION_BYTES + 1];
    let mut r = request();
    r.observed = Some(&bytes);
    let mut e = Recorder::default();
    assert_eq!(
        run(&mut a, &r, [0; 64], &mut e).unwrap_err(),
        AdmissionError::TooLarge
    );
    assert!(a.audit().last().unwrap().request_bytes().is_none());
    assert_eq!(e.calls, 0);
}

#[test]
fn ia17_diagnostics_do_not_expose_identity_content_or_keys() {
    let mut a = active();
    let mut r = request();
    r.observed = Some(b"private-sentinel");
    let s = signed(&r, &a);
    let mut e = Recorder::default();
    let result = run(&mut a, &r, s, &mut e).unwrap();
    let text = format!("{a:?} {:?} {:?} {result:?}", a.mandate(), a.audit());
    for secret in [
        "private-sentinel",
        "issuer-synthetic",
        "evidence-synthetic",
        "domain-A",
    ] {
        assert!(!text.contains(secret));
    }
}

#[test]
fn ia18_all_six_core_checks_and_specific_check_are_produced() {
    let a = active();
    let r = request();
    let (form, effect) = plan::operation(a.received.continuity()).unwrap();
    let evidence = ValidatedInvocation {
        operation: &a,
        observed: r.observed,
        now: 100,
        tick: Instant::now(),
    };
    let results = crate::requirements::institutional::measure(&evidence, &form, &effect).unwrap();
    assert_eq!(results.len(), 7);
    assert!(results
        .iter()
        .all(|r| r.result() == CheckResult::Accredited));
    assert!(results.iter().all(|r| r.check_observations().count() == 1));
}

#[test]
fn ia19_core_validity_and_non_self_checks_are_not_constants() {
    for corrupt in 0..2 {
        let mut a = active();
        if corrupt == 0 {
            a.revoked = Some(([0; 64], Some(100)));
        } else {
            a.issuer = a.mandate.proposer;
        }
        let (form, effect) = plan::operation(a.received.continuity()).unwrap();
        let evidence = ValidatedInvocation {
            operation: &a,
            observed: Some(b"evidence-synthetic"),
            now: 100,
            tick: Instant::now(),
        };
        let results =
            crate::requirements::institutional::measure(&evidence, &form, &effect).unwrap();
        assert_eq!(
            results
                .iter()
                .filter(|r| r.result() == CheckResult::Refuted)
                .count(),
            1
        );
    }
}

#[test]
fn ia20_binary_encoding_separates_absence_empty_and_field_boundaries() {
    let m = mandate();
    let mut r = request();
    r.observed = None;
    let absent = r.signing_bytes(&m).unwrap();
    r.observed = Some(b"");
    assert_ne!(absent, r.signing_bytes(&m).unwrap());
    let mut left = mandate();
    let mut right = left.clone();
    left.labels[0] = "ab".into();
    left.labels[1] = "c".into();
    right.labels[0] = "a".into();
    right.labels[1] = "bc".into();
    assert_ne!(left.signing_bytes(), right.signing_bytes());
}

#[test]
fn ia21_adapter_panic_preserves_terminal_state_and_causal_trace() {
    struct Panicking;
    impl InstitutionalExecutor for Panicking {
        fn execute(&mut self, _: &ExecutionRequest<'_>, _: &[u8]) -> Result<(), AdapterFailure> {
            panic!("synthetic adapter panic");
        }
    }
    let mut a = active();
    let r = request();
    let s = signed(&r, &a);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = run(&mut a, &r, s, &mut Panicking);
    }));
    assert!(result.is_err());
    assert!(a.dispatched);
    assert_eq!(
        a.audit().last().unwrap().outcome(),
        AuditOutcome::DispatchCommitted
    );
    assert_eq!(a.trace().decision_trace_count(), 1);
    assert_eq!(a.trace().exercise_event_count(), 1);
    assert_eq!(
        run(&mut a, &r, s, &mut Recorder::default()).unwrap_err(),
        AdmissionError::AlreadyDispatched
    );
}

#[test]
fn ia22_concurrent_duplicate_admission_has_one_recipient() {
    let m = mandate();
    let s = issuer().sign(&m.signing_bytes()).to_bytes();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let handles = (0..2)
        .map(|_| {
            let b = barrier.clone();
            let m = m.clone();
            std::thread::spawn(move || {
                b.wait();
                installation(m)
                    .admit_at(s, Ok(100), Instant::now())
                    .map(|_| ())
            })
        })
        .collect::<Vec<_>>();
    let results = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| **r == Err(AdmissionError::AlreadyAdmitted))
            .count(),
        1
    );
}

#[test]
fn ia23_invalid_or_oversized_configuration_is_not_admitted() {
    let oversized = vec![0; MAX_RECEPTION_BYTES + 1];
    for (name, contract, expected, until, wanted) in [
        (
            "bad\nname",
            &b"contract"[..],
            &b"expected"[..],
            200,
            AdmissionError::InvalidField,
        ),
        (
            "institution",
            &b""[..],
            &b"expected"[..],
            200,
            AdmissionError::InvalidField,
        ),
        (
            "institution",
            &b"contract"[..],
            &b"expected"[..],
            100,
            AdmissionError::InvalidField,
        ),
        (
            "institution",
            &b"contract"[..],
            &oversized[..],
            200,
            AdmissionError::TooLarge,
        ),
    ] {
        let result = InstitutionalMandate::new(MandateDefinition {
            institution: name,
            issuer: "issuer",
            domain: "domain",
            context: "context",
            operation: "operation",
            object: "object",
            edition: "v1",
            mandate_id: "id",
            proposer_key: proposer().verifying_key().to_bytes(),
            valid_from: 100,
            valid_until: until,
            equality_contract: contract,
            expected,
        });
        assert_eq!(result.unwrap_err(), wanted);
    }
}

#[test]
fn ia24_unauthenticated_admission_cannot_exhaust_audit_capacity() {
    let m = mandate();
    let good = issuer().sign(&m.signing_bytes()).to_bytes();
    let mut i = installation(m);
    for _ in 0..128 {
        assert_eq!(
            i.admit_at([0; 64], Ok(100), Instant::now()).unwrap_err(),
            AdmissionError::InvalidSignature
        );
    }
    assert_eq!(i.audit().len(), 0);
    assert_eq!(i.input_rejections().count(), 128);
    let admitted = i.admit_at(good, Ok(100), Instant::now()).unwrap();
    assert_eq!(admitted.audit().len(), 1);
    assert_eq!(admitted.input_rejections().count(), 128);
}
#[test]
fn ia25_same_issuer_name_with_different_key_is_not_recognized() {
    let m = mandate();
    let counterfeit = SigningKey::from_bytes(&[9; 32])
        .sign(&m.signing_bytes())
        .to_bytes();
    assert_eq!(
        installation(m)
            .admit_at(counterfeit, Ok(100), Instant::now())
            .unwrap_err(),
        AdmissionError::InvalidSignature
    );
}

#[test]
fn ia26_signed_invocation_cannot_be_changed_after_signing() {
    for change in 0..3 {
        let mut a = active();
        let mut r = request();
        let s = signed(&r, &a);
        match change {
            0 => r.nonce = [4; 16],
            1 => r.observed = Some(b"other"),
            _ => r.observed = None,
        }
        let mut e = Recorder::default();
        assert_eq!(
            run(&mut a, &r, s, &mut e).unwrap_err(),
            AdmissionError::InvalidSignature
        );
        assert_eq!(e.calls, 0);
    }
}

#[test]
fn ia27_unauthenticated_requests_do_not_exhaust_authenticated_capacity() {
    let mut a = active();
    let r = request();
    let mut e = Recorder::default();
    for _ in 0..128 {
        assert_eq!(
            run(&mut a, &r, [0; 64], &mut e).unwrap_err(),
            AdmissionError::InvalidSignature
        );
    }
    assert_eq!(a.audit().len(), 1);
    assert_eq!(a.input_rejections().count(), 128);
    let good = signed(&r, &a);
    assert!(matches!(
        run(&mut a, &r, good, &mut e),
        Ok(InstitutionalOutcome::Confirmed(_))
    ));
    assert_eq!(e.calls, 1);
}
