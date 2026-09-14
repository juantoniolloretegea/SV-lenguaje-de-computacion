//! ES: Deriva doce estímulos y oráculos de las semillas ES01/ES02 ya comprometidas.
//! EN: Derives twelve stimuli and oracles from the previously committed ES01/ES02 seeds.
//! ES: No enlaza ni invoca el admisor. Conserva geometría y estado originales.
//! EN: Does not link or invoke admission. Preserves original geometry and state.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/json.rs"]
mod json;
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/sha.rs"]
mod sha;
use json::{n, obj, s, J};
use std::{fs, path::Path};
fn read(p: &Path) -> Vec<u8> {
    fs::read(p).expect("lectura de semilla")
}
fn doc(p: &Path) -> J {
    json::decode(&read(p)).expect("JSON de semilla")
}
fn enc(j: &J) -> Vec<u8> {
    json::encode(j, 32768).expect("serialización preparatoria")
}
fn write(p: &Path, j: &J) {
    fs::write(p, enc(j)).expect("escritura preparatoria")
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 3, "semillas destino_nuevo");
    let seed = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    assert!(!out.exists(), "el destino debe ser nuevo");
    fs::create_dir(out).unwrap();
    let mut cases = vec![];
    for (profile, seed_id) in [("en", "ES01"), ("es", "ES02")] {
        let src = seed.join(seed_id);
        for (index, geometry_len, receipt_len, unicode, guard, cause) in [
            (1, 8191usize, 2048usize, false, "", ""),
            (2, 8192, 2048, false, "", ""),
            (3, 8192, 2049, false, "R01", "output receipt BYTES"),
            (4, 8193, 2048, false, "R01", "BYTES"),
            (5, 8192, 2048, true, "", ""),
            (6, 8192, 2049, true, "R01", "output receipt BYTES"),
        ] {
            let id = format!("FS{index:02}-{profile}");
            let dir = out.join(&id);
            fs::create_dir(&dir).unwrap();
            for name in [
                "source.bin",
                "state.bin",
                "support.bin",
                "canonical.bin",
                "constitution.bin",
                "convention.bin",
                "transforms.bin",
                "registry.json",
            ] {
                fs::write(dir.join(name), read(&src.join(name))).unwrap();
            }
            let mut q = doc(&src.join("request.json"));
            let mut c = doc(&src.join("context.json"));
            let mut receipt = doc(&src.join("oracle.json"))
                .field("recibo_esperado")
                .clone();
            let original = read(&src.join("geometry.bin"));
            assert!(original.len() < geometry_len);
            let mut geo = original.clone();
            geo.resize(geometry_len, b' ');
            fs::write(dir.join("geometry.bin"), &geo).unwrap();
            let mut geo_ref = q.field("geometria").clone();
            geo_ref.set("bytes", n(geo.len()));
            geo_ref.set("sha256", s(&sha::hash(&geo)));
            q.set("geometria", geo_ref.clone());
            c.set("geometria", geo_ref);
            c.set("id", s(&id));
            receipt.set("caso", s(&id));
            receipt.set("geometria_bytes", n(geo.len()));
            receipt.set("geometria_sha256", s(&sha::hash(&geo)));
            let mut delivery = q.field("entrega").clone();
            delivery.set("invocacion", s(&id));
            delivery.set("consumidor", s(""));
            receipt.set("contexto_entrega", delivery.clone());
            let room = receipt_len
                .checked_sub(enc(&receipt).len())
                .expect("presupuesto de recibo");
            let consumer = if unicode {
                "ñ".repeat(room / 2) + &"x".repeat(room % 2)
            } else {
                "c".repeat(room)
            };
            assert_eq!(consumer.len(), room);
            delivery.set("consumidor", s(&consumer));
            q.set("entrega", delivery.clone());
            c.set("entrega", delivery.clone());
            receipt.set("contexto_entrega", delivery);
            let expected = enc(&receipt);
            assert_eq!(expected.len(), receipt_len);
            fs::write(dir.join("recibo_esperado.bin"), &expected).unwrap();
            write(&dir.join("request.json"), &q);
            write(&dir.join("context.json"), &c);
            let mut lengths = Vec::new();
            for name in [
                "request.json",
                "source.bin",
                "state.bin",
                "support.bin",
                "geometry.bin",
            ] {
                lengths.push(read(&dir.join(name)).len());
            }
            assert!(
                lengths[0] <= 4096
                    && lengths[1] <= 4096
                    && lengths[2] <= 1024
                    && lengths[3] <= 1024
            );
            assert!(lengths.iter().sum::<usize>() <= 16384);
            let passed: Vec<J> = if index == 4 {
                vec![]
            } else {
                "R01 P01 P02 S01 S02 I01 I02 I03 A01 M01 M02 M03"
                    .split_whitespace()
                    .map(s)
                    .collect()
            };
            let oracle = obj(vec![
                ("id", s(&id)),
                ("perfil", s(profile)),
                ("guard", s(guard)),
                ("cause", s(cause)),
                ("guardas_admision", J::Array(passed)),
                ("longitudes", J::Array(lengths.into_iter().map(n).collect())),
                ("descriptor_bytes", n(geometry_len)),
                ("recibo_bytes", n(receipt_len)),
                ("salida_hipotetica_bytes", n(geometry_len + receipt_len)),
                ("despachos", n(if guard.is_empty() { 1 } else { 0 })),
                (
                    "vector",
                    doc(&src.join("state.bin")).field("vector").clone(),
                ),
                ("unicode", J::Bool(unicode)),
                ("ejecucion", s("PENDIENTE")),
            ]);
            write(&dir.join("oraculo.json"), &oracle);
            cases.push(s(&id));
        }
    }
    write(&out.join("CASOS.json"), &J::Array(cases));
    let mut entries = Vec::new();
    for d in fs::read_dir(out).unwrap() {
        let d = d.unwrap().path();
        if d.is_dir() {
            for p in fs::read_dir(&d).unwrap() {
                entries.push(p.unwrap().path());
            }
        } else {
            entries.push(d);
        }
    }
    entries.sort();
    let rows = entries
        .iter()
        .map(|p| {
            obj(vec![
                ("ruta", s(p.strip_prefix(out).unwrap().to_str().unwrap())),
                ("bytes", n(read(p).len())),
                ("sha256", s(&sha::hash(&read(p)))),
            ])
        })
        .collect();
    write(&out.join("HUELLAS.json"), &J::Array(rows));
    println!("12 variantes preparadas; 0 llamadas de admisión; oráculos previos conservados.");
}
