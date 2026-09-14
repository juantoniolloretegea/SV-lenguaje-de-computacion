//! ES: Conductor y observador externo del SVG; oráculos de custodia separados.
//! EN: External SVG harness and observer; separate custody oracles.
#![forbid(unsafe_code)]
use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
use sv_bis_i0205::{
    admit,
    json::{self, n, obj, s, J},
    sha, ReceivedBytes, TrustedContext, TrustedRegistry,
};
use svg_consumo::{consume, inspect_svg, render};
fn read(p: &Path) -> Vec<u8> {
    fs::read(p).unwrap()
}
fn doc(p: &Path) -> J {
    json::decode(&read(p)).unwrap()
}
fn put(p: &Path, j: &J) {
    fs::write(p, json::encode(j, 65536).unwrap()).unwrap()
}
fn coords(v: &[[i64; 2]; 16]) -> J {
    J::Array(
        v.iter()
            .map(|p| J::Array(p.iter().map(|x| J::Number(x.to_string())).collect()))
            .collect(),
    )
}
struct Channel {
    file: File,
    capture: Vec<u8>,
    fail_after: Option<usize>,
}
impl Read for Channel {
    fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        if self.fail_after == Some(self.capture.len()) {
            return Err(io::Error::other("fallo previsto del canal SVG"));
        }
        let limit = self
            .fail_after
            .map(|n| n - self.capture.len())
            .unwrap_or(b.len())
            .min(b.len());
        let n = self.file.read(&mut b[..limit])?;
        self.capture.extend_from_slice(&b[..n]);
        Ok(n)
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4, "entradas salida_nueva modo");
    let input = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    let mode = &a[3];
    assert!(matches!(
        mode.as_str(),
        "normal" | "svg_permutado" | "consumo_falso_svg"
    ));
    assert!(!out.exists());
    fs::create_dir(out).unwrap();
    let expected_svg = read(&input.join("esperado.svg"));
    let math = doc(&input.join("matematico.json"));
    let alternate = read(&input.join("alternativo.svg"));
    let ids = if mode == "normal" {
        doc(&input.join("CASOS.json"))
    } else {
        J::Array(vec![s("RC01")])
    };
    let mut failures = 0;
    let mut integrated = 0;
    let mut instrumental = 0;
    let mut observations = vec![];
    for id in ids.array().unwrap() {
        let id = id.text().unwrap();
        let dir = input.join(id);
        let dst = out.join(id);
        fs::create_dir(&dst).unwrap();
        let oracle = doc(&dir.join("oraculo.json"));
        let before = read(&dir.join("state.bin"));
        let mut guard = String::new();
        let mut capture = vec![];
        let mut produced = vec![];
        let mut vertices = J::Null;
        let mut differences = J::Null;
        let mut reported = String::new();
        let mut count = 0;
        let mut record = J::Null;
        let mut admission = J::Null;
        let mut offered = J::Null;
        let mut faults = vec![];
        if id == "RC10" {
            instrumental += 1;
            match inspect_svg(&read(&dir.join("canal.svg"))) {
                Ok(_) => {}
                Err(g) => guard = g.into(),
            }
        } else {
            integrated += 1;
            let context = TrustedContext::from_custody(&read(&dir.join("context.json"))).unwrap();
            let registry = TrustedRegistry::from_custody(
                &read(&dir.join("registry.json")),
                &read(&dir.join("constitution.bin")),
                &read(&dir.join("convention.bin")),
                &read(&dir.join("transforms.bin")),
            )
            .unwrap();
            let mut metadata = File::open(dir.join("request.json")).unwrap();
            let mut source = File::open(dir.join("source.bin")).unwrap();
            let mut state = File::open(dir.join("state.bin")).unwrap();
            let mut support = File::open(dir.join("support.bin")).unwrap();
            let mut geometry = File::open(dir.join("geometry.bin")).unwrap();
            let result = ReceivedBytes::read(
                &mut metadata,
                &mut source,
                &mut state,
                Some(&mut support),
                &mut geometry,
            )
            .and_then(|b| admit(b, &context, &registry));
            match result {
                Err(e) => {
                    guard = format!("ADMISION/{}", e.guard);
                    admission = obj(vec![
                        ("causa", s(&e.cause)),
                        ("guardas", J::Array(e.passed.iter().map(|x| s(x)).collect())),
                    ]);
                }
                Ok(admitted) => {
                    admission = J::Array(admitted.passed().iter().map(|x| s(x)).collect());
                    match render(&admitted) {
                        Err(g) => guard = g.into(),
                        Ok(rendered) => {
                            produced = rendered.bytes().to_vec();
                            record = rendered.record().clone();
                            fs::write(dst.join("producido.svg"), &produced).unwrap();
                            offered = if matches!(id, "RC05" | "RC06") {
                                doc(&dir.join("envoltura_ofrecida.json"))
                            } else {
                                record.clone()
                            };
                            let path = if matches!(id, "RC04" | "RC08") {
                                dir.join("canal.svg")
                            } else {
                                dst.join("producido.svg")
                            };
                            let mut channel = Channel {
                                file: File::open(path).unwrap(),
                                capture: vec![],
                                fail_after: if id == "RC09" { Some(64) } else { None },
                            };
                            let result = consume(
                                &rendered,
                                &offered,
                                if id == "RC07" {
                                    None
                                } else {
                                    Some(&mut channel)
                                },
                                &alternate,
                            );
                            capture = channel.capture;
                            match result {
                                Err(g) => guard = g.into(),
                                Ok(c) => {
                                    count = 1;
                                    vertices = coords(&c.vertices);
                                    differences = coords(&c.differences);
                                    reported = c.reported_sha256;
                                }
                            }
                            if produced != expected_svg {
                                faults.push(s("SVG producido distinto del oraculo literal"))
                            }
                            if record != doc(&dir.join("envoltura_esperada.json")) {
                                faults.push(s("envoltura producida distinta"))
                            }
                        }
                    }
                }
            }
        }
        let preserved = before == read(&dir.join("state.bin"));
        let observed = obj(vec![
            ("id", s(id)),
            ("modo", s(mode)),
            ("guard", s(&guard)),
            ("leidos_svg", n(capture.len())),
            ("consumos", n(count)),
            ("instrumental", J::Bool(id == "RC10")),
            ("estado_preservado", J::Bool(preserved)),
            ("admision", admission),
            ("envoltura_producida", record),
            ("envoltura_ofrecida", offered),
            ("captura_sha256", s(&sha::hash(&capture))),
            ("hash_declarado_consumidor", s(&reported)),
            ("vertices_consumidos", vertices.clone()),
            ("diferencias_consumidas", differences.clone()),
        ]);
        for key in [
            "guard",
            "leidos_svg",
            "consumos",
            "instrumental",
            "estado_preservado",
        ] {
            if observed.field(key) != oracle.field(key) {
                faults.push(s(&format!("{key} discordante")))
            }
        }
        if count == 1 {
            if capture != expected_svg {
                faults.push(s("captura literal discordante"))
            }
            if vertices != *math.field("coordenadas") {
                faults.push(s("vertices realmente utilizados discordantes"))
            }
            if differences != *math.field("aristas_diferencia") {
                faults.push(s("operacion sobre dependencia discordante"))
            }
            if reported != sha::hash(&expected_svg) {
                faults.push(s("hash declarado discordante"))
            }
        }
        let expected_read = oracle.field("leidos_svg").uint().unwrap();
        if expected_read > 0 {
            let expected_channel = if matches!(id, "RC04" | "RC08") {
                read(&dir.join("canal.svg"))
            } else {
                expected_svg.clone()
            };
            if capture != expected_channel[..expected_read] {
                faults.push(s("bytes observados en canal discordantes"))
            }
        }
        if !faults.is_empty() {
            failures += 1;
        }
        fs::write(dst.join("captura.bin"), &capture).unwrap();
        put(&dst.join("observado.json"), &observed);
        put(&dst.join("discrepancias.json"), &J::Array(faults.clone()));
        println!(
            "{id}: guard={guard:?}; leidos={}; consumos={count}; discrepancias={}",
            capture.len(),
            faults.len()
        );
        observations.push(observed);
    }
    put(&out.join("OBSERVACIONES.json"), &J::Array(observations));
    put(
        &out.join("RESUMEN.json"),
        &obj(vec![
            ("modo", s(mode)),
            ("casos", n(ids.array().unwrap().len())),
            ("integrados", n(integrated)),
            ("instrumentales", n(instrumental)),
            ("casos_discordantes", n(failures)),
        ]),
    );
    if failures > 0 {
        std::process::exit(1)
    }
}
