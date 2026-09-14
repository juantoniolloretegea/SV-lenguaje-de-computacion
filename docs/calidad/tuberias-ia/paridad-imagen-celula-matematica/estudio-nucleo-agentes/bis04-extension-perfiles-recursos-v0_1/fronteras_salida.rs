//! ES: Contraste externo del límite conjunto; conserva canales, capturas y rechazos.
//! EN: External joint-limit check; preserves channels, captures and rejections.
//! ES: Los oráculos son entradas separadas. No se deriva el esperado del admisor.
//! EN: Oracles are separate inputs. Expected results are not derived from admission.
#![forbid(unsafe_code)]
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    time::Instant,
};
use sv_bis_i0205::{
    json::{n, obj, s, J},
    *,
};
fn read(p: &Path) -> Vec<u8> {
    fs::read(p).expect("lectura")
}
fn doc(p: &Path) -> J {
    json::decode(&read(p)).expect("JSON")
}
fn write(p: &Path, j: &J) {
    fs::write(p, json::encode(j, 65536).unwrap()).unwrap()
}
struct Channel {
    file: File,
    count: usize,
}
impl Channel {
    fn open(p: &Path) -> Self {
        Self {
            file: File::open(p).unwrap(),
            count: 0,
        }
    }
}
impl Read for Channel {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        let len = self.file.read(b)?;
        self.count += len;
        Ok(len)
    }
}
// ES: Esta función representa el receptor; recibe el mismo buffer validado.
// EN: This function represents the receiver; it receives the validated buffer.
fn capture(b: &[u8], c: &J, captor: &str) -> Capture {
    Capture {
        bytes: b.to_vec(),
        context: c.clone(),
        captor: captor.into(),
    }
}
fn inspect(
    observed: &J,
    oracle: &J,
    receipt: &[u8],
    expected: &[u8],
    captured: &[u8],
    geometry: &[u8],
) -> Vec<String> {
    let mut faults = vec![];
    for (a, b) in [
        ("guard", "guard"),
        ("cause", "cause"),
        ("guardas_admision", "guardas_admision"),
        ("leidos", "longitudes"),
        ("despachos", "despachos"),
    ] {
        if observed.field(a) != oracle.field(b) {
            faults.push(format!("{a} discordante"));
        }
    }
    if observed.field("estado_preservado").boolean() != Some(true) {
        faults.push("archivo de estado alterado".into());
    }
    if oracle.field("guard").text() == Some("") {
        if receipt != expected {
            faults.push("recibo literal".into());
        }
        if captured != geometry {
            faults.push("descriptor capturado".into());
        }
        if observed.field("vector") != oracle.field("vector") {
            faults.push("vector consumido".into());
        }
        if observed.field("descriptor_bytes") != oracle.field("descriptor_bytes")
            || observed.field("recibo_bytes") != oracle.field("recibo_bytes")
        {
            faults.push("longitudes de salida".into());
        }
        if observed.field("salida_bytes").uint() != Some(captured.len() + receipt.len()) {
            faults.push("suma observada".into());
        }
        if captured.len() + receipt.len() > 10240 {
            faults.push("exceso agregado".into());
        }
    } else if !receipt.is_empty()
        || !captured.is_empty()
        || observed.field("vector") != &J::Null
        || observed.field("salida_bytes").uint() != Some(0)
    {
        faults.push("salida tras rechazo previo".into());
    }
    faults
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3, "entradas salida_nueva");
    let input = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    assert!(!out.exists());
    fs::create_dir(out).unwrap();
    let ids = doc(&input.join("CASOS.json"));
    let mut failures = 0;
    let mut sensitivities = 0;
    let start = Instant::now();
    for id in ids.array().unwrap() {
        let id = id.text().unwrap();
        let dir = input.join(id);
        let dst = out.join(id);
        fs::create_dir(&dst).unwrap();
        let oracle = doc(&dir.join("oraculo.json"));
        let expected = read(&dir.join("recibo_esperado.bin"));
        let geometry = read(&dir.join("geometry.bin"));
        let before = read(&dir.join("state.bin"));
        let context = TrustedContext::from_custody(&read(&dir.join("context.json"))).unwrap();
        let registry = TrustedRegistry::from_custody(
            &read(&dir.join("registry.json")),
            &read(&dir.join("constitution.bin")),
            &read(&dir.join("convention.bin")),
            &read(&dir.join("transforms.bin")),
        )
        .unwrap();
        let mut meta = Channel::open(&dir.join("request.json"));
        let mut source = Channel::open(&dir.join("source.bin"));
        let mut state = Channel::open(&dir.join("state.bin"));
        let mut support = Channel::open(&dir.join("support.bin"));
        let mut geo = Channel::open(&dir.join("geometry.bin"));
        let mut guard = String::new();
        let mut cause = String::new();
        let passed;
        let mut receipt = vec![];
        let mut captured = vec![];
        let mut vector = J::Null;
        let mut dispatches = 0;
        let tick = Instant::now();
        match ReceivedBytes::read(
            &mut meta,
            &mut source,
            &mut state,
            Some(&mut support),
            &mut geo,
        )
        .and_then(|r| admit(r, &context, &registry))
        {
            Err(e) => {
                guard = e.guard.into();
                cause = e.cause;
                passed = e.passed;
            }
            Ok(a) => {
                passed = a.passed().to_vec();
                vector = J::Array(a.vector().iter().map(|t| s(t.ir_label())).collect());
                let rj = doc(&dir.join("registry.json"));
                let captor = rj.field("captores_admitidos").array().unwrap()[0]
                    .text()
                    .unwrap();
                let cap = capture(a.descriptor(), a.context().documentary(), captor);
                dispatches += 1;
                captured = cap.bytes.clone();
                match certify(&a, Some(&cap), false, &registry) {
                    Ok(r) => {
                        receipt = json::encode(&r, 2048).expect("recibo acotado");
                    }
                    Err(e) => {
                        guard = e.guard.into();
                        cause = e.cause;
                    }
                }
            }
        }
        let nanos = tick.elapsed().as_nanos().to_string();
        let observed = obj(vec![
            ("id", s(id)),
            ("guard", s(&guard)),
            ("cause", s(&cause)),
            (
                "guardas_admision",
                J::Array(passed.iter().map(|x| s(x)).collect()),
            ),
            (
                "leidos",
                J::Array(
                    [
                        meta.count,
                        source.count,
                        state.count,
                        support.count,
                        geo.count,
                    ]
                    .into_iter()
                    .map(n)
                    .collect(),
                ),
            ),
            ("despachos", n(dispatches)),
            ("vector", vector),
            ("descriptor_bytes", n(captured.len())),
            ("recibo_bytes", n(receipt.len())),
            ("salida_bytes", n(captured.len() + receipt.len())),
            (
                "estado_preservado",
                J::Bool(before == read(&dir.join("state.bin"))),
            ),
            ("duracion_ns", s(&nanos)),
            ("captura_sha256", s(&sha::hash(&captured))),
            ("recibo_sha256", s(&sha::hash(&receipt))),
        ]);
        let faults = inspect(
            &observed, &oracle, &receipt, &expected, &captured, &geometry,
        );
        if !faults.is_empty() {
            failures += 1;
        }
        fs::write(dst.join("captura.bin"), &captured).unwrap();
        fs::write(dst.join("recibo.bin"), &receipt).unwrap();
        write(&dst.join("observado.json"), &observed);
        write(
            &dst.join("discrepancias.json"),
            &J::Array(faults.iter().map(|x| s(x)).collect()),
        );
        println!(
            "{id}: guarda={guard:?}; salida={}; discrepancias={faults:?}",
            captured.len() + receipt.len()
        );
        // ES: Sensibilidades sobre evidencia observada, sin repetir el sujeto.
        // EN: Sensitivities modify observed evidence without rerunning the subject.
        if id == "FS02-en" {
            let mut altered = receipt.clone();
            altered.push(b' ');
            if !inspect(
                &observed, &oracle, &altered, &expected, &captured, &geometry,
            )
            .is_empty()
            {
                sensitivities += 1;
            }
            if !inspect(
                &observed,
                &oracle,
                &receipt,
                &expected,
                &captured[..captured.len() - 1],
                &geometry,
            )
            .is_empty()
            {
                sensitivities += 1;
            }
            let mut altered = observed.clone();
            altered.set("salida_bytes", n(10239));
            if !inspect(&altered, &oracle, &receipt, &expected, &captured, &geometry).is_empty() {
                sensitivities += 1;
            }
        }
        if id == "FS03-en" {
            let mut altered = observed.clone();
            altered.set("despachos", n(1));
            if !inspect(&altered, &oracle, &receipt, &expected, &captured, &geometry).is_empty() {
                sensitivities += 1;
            }
        }
    }
    if sensitivities != 4 {
        failures += 1;
    }
    write(
        &out.join("RESUMEN.json"),
        &obj(vec![
            ("variantes", n(ids.array().unwrap().len())),
            ("discrepancias", n(failures)),
            ("sensibilidades_detectadas", n(sensitivities)),
            (
                "duracion_total_ns",
                s(&start.elapsed().as_nanos().to_string()),
            ),
        ]),
    );
    if failures != 0 {
        std::process::exit(1);
    }
}
