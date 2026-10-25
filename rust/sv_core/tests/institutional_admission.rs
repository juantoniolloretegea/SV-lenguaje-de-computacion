//! ES: Cliente externo del módulo compilado sin cfg(test), con datos sintéticos.
//! EN: External client of the library built without cfg(test), synthetic data.
#![cfg(feature = "institutional-admission")]
use ed25519_dalek::{Signer, SigningKey};
use sv_core::authority::transitions::institutional::*;
use sv_core::ExecutionRequest;

struct LocalRecorder {
    calls: usize,
}
impl InstitutionalExecutor for LocalRecorder {
    fn execute(
        &mut self,
        request: &ExecutionRequest<'_>,
        evidence: &[u8],
    ) -> Result<(), AdapterFailure> {
        assert_eq!(request.effect().object().id().as_str(), "synthetic-object");
        assert_eq!(evidence, b"synthetic exact reference");
        self.calls += 1;
        Ok(())
    }
}
#[test]
fn public_client_admits_and_dispatches_without_test_premise() {
    let issuer = SigningKey::from_bytes(&[31; 32]);
    let proposer = SigningKey::from_bytes(&[32; 32]);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let m = InstitutionalMandate::new(MandateDefinition {
        institution: "synthetic-institution",
        issuer: "recognized-issuer",
        domain: "synthetic-domain",
        context: "synthetic-context",
        operation: "checked-delivery",
        object: "synthetic-object",
        edition: "v1",
        mandate_id: "external-client-test",
        proposer_key: proposer.verifying_key().to_bytes(),
        valid_from: now.saturating_sub(1),
        valid_until: now + 60,
        equality_contract: b"synthetic-contract-v1",
        expected: b"synthetic exact reference",
    })
    .unwrap();
    let signature = issuer.sign(&m.signing_bytes()).to_bytes();
    let mut installation =
        InstitutionalInstallation::configure(m, issuer.verifying_key().to_bytes()).unwrap();
    let mut operation = installation.admit(signature).unwrap();
    let r = InstitutionalInvocation {
        domain: "synthetic-domain",
        context: "synthetic-context",
        operation: "checked-delivery",
        object: "synthetic-object",
        nonce: [33; 16],
        observed: Some(b"synthetic exact reference"),
    };
    let signature = proposer
        .sign(&r.signing_bytes(operation.mandate()).unwrap())
        .to_bytes();
    let mut recorder = LocalRecorder { calls: 0 };
    assert!(matches!(
        operation.perform(&r, signature, &mut recorder),
        Ok(InstitutionalOutcome::Confirmed(_))
    ));
    assert_eq!(recorder.calls, 1);
    assert_eq!(
        operation.audit().last().unwrap().outcome(),
        AuditOutcome::Confirmed
    );
    assert_eq!(
        operation.perform(&r, signature, &mut recorder).unwrap_err(),
        AdmissionError::AlreadyDispatched
    );
}


// SPDX-License-Identifier: CC-BY-NC-ND-4.0
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
// https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
