//! ES: Conductor local, receptor y observador experimental, sin servicios externos.
//! EN: Local harness, receiver and experimental observer, without external services.
#[path="observer.rs"]
mod observer;
pub use observer::{inspect, Evidence};
use std::{
    fs::{self, File},
    io::Cursor,
    path::{Path, PathBuf},
};
use sv_bis_i0205::{
    json::{n, obj, s, J},
    *,
};
fn bytes(p: &Path, max: usize) -> Vec<u8> {
    json::receive(
        &mut File::open(p).unwrap_or_else(|e| panic!("loader {}: {e}", p.display())),
        max,
    )
    .unwrap_or_else(|e| panic!("loader {}: {e}", p.display()))
}
fn document(p: &Path) -> J {
    json::decode(&bytes(p, 8192))
        .unwrap_or_else(|e| panic!("custody document {}: {e}", p.display()))
}
fn write(p: &Path, j: &J) {
    fs::write(p, json::encode(j, 300000).expect("bounded report")).expect("report write")
}
fn rejection(e: &mut Evidence, r: Rejection) {
    e.result = if matches!(r.guard, "D01" | "D07") {
        "NO_ACREDITADO"
    } else {
        "RECHAZADO"
    }
    .into();
    e.guard = Some(r.guard.into());
    e.cause = r.cause;
    e.passed = r.passed.iter().map(|x| (*x).into()).collect();
}
// ES: Captura los bytes que entran en el receptor, no los declarados en el plan.
// EN: Captures bytes entering the receiver, not bytes asserted by the plan.
fn receiver(input: &[u8], context: J, captor: String) -> Capture {
    Capture {
        bytes: input.to_vec(),
        context,
        captor,
    }
}
fn run_case_antecedente(dir: &Path, registry: &TrustedRegistry, _canonical: &[u8]) -> Evidence {
    let context = TrustedContext::from_custody(&bytes(&dir.join("context.json"), 8192))
        .expect("trusted context");
    let mut meta = File::open(dir.join("request.json")).expect("metadata");
    let mut source = File::open(dir.join("source.bin")).expect("source");
    let mut state = File::open(dir.join("state.bin")).expect("state");
    let mut support = if dir.join("support.bin").exists() {
        Some(File::open(dir.join("support.bin")).expect("support"))
    } else {
        None
    };
    let mut geometry = File::open(dir.join("geometry.bin")).expect("geometry");
    let mut e = Evidence {
        result: String::new(),
        guard: None,
        cause: String::new(),
        passed: vec![],
        receipt: None,
        capture: None,
        dispatches: Some(0),
        attempts: 0,
        before: vec![],
        after: vec![],
        admitted_vector: None,
        preservation_claim: false,
    };
    match ReceivedBytes::read(
        &mut meta,
        &mut source,
        &mut state,
        support.as_mut(),
        &mut geometry,
    )
    .and_then(|r| admit(r, &context, registry))
    {
        Err(r) => rejection(&mut e, r),
        Ok(a) => {
            e.admitted_vector = Some(J::Array(
                a.vector().iter().map(|t| s(t.ir_label())).collect(),
            ));
            let plan = document(&dir.join("plan.json"));
            e.attempts = 1;
            // ES: El plan sólo introduce una sustitución externa cuando difiere del buffer.
            // EN: The plan introduces an external substitution only when it differs from the buffer.
            if plan.field("producir_captura").boolean() == Some(true) {
                let replacement = if plan.field("buffer_capturado").field("sha256").text()
                    != Some(sha::hash(a.descriptor()).as_str())
                {
                    Some(bytes(&dir.join("injection.bin"), 8192))
                } else {
                    None
                };
                let transported = replacement.as_deref().unwrap_or(a.descriptor());
                e.capture = Some(receiver(
                    transported,
                    plan.field("contexto_captura").clone(),
                    plan.field("captor").text().expect("captor").into(),
                ));
                e.dispatches = Some(1);
            } else {
                e.dispatches = None;
            }
            match certify(
                &a,
                e.capture.as_ref(),
                plan.field("fallo_despues_de_despacho").boolean() == Some(true),
                registry,
            ) {
                Err(r) => rejection(&mut e, r),
                Ok(receipt) => {
                    e.result = "ENTREGA_DOCUMENTAL_CONCORDANTE".into();
                    e.receipt = Some(receipt);
                    e.passed = a
                        .passed()
                        .iter()
                        .chain(["D01", "D02", "D03", "D04", "D05", "D06"].iter())
                        .map(|x| (*x).into())
                        .collect();
                }
            }
        }
    }
    e
}

include!("recepcion.rs");
