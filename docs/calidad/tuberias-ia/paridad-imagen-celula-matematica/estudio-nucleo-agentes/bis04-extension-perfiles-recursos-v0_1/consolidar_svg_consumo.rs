//! ES: Verifica custodia y consolida observaciones ya producidas; no repite pruebas.
//! EN: Verifies custody and consolidates existing observations; never reruns tests.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/json.rs"]
mod json;
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/sha.rs"]
mod sha;
use json::{n, obj, s, J};
use std::{collections::BTreeSet, fs, path::Path};
fn read(p: &Path) -> Vec<u8> {
    fs::read(p).unwrap()
}
fn doc(p: &Path) -> J {
    json::decode(&read(p)).unwrap()
}
fn put(p: &Path, j: &J) {
    assert!(!p.exists());
    fs::write(p, json::encode(j, 65536).unwrap()).unwrap()
}
fn manifest(p: &Path) -> Vec<J> {
    let raw = fs::read_to_string(p).unwrap();
    assert!(raw.len() <= 65536);
    let body = raw
        .trim()
        .strip_prefix("[{")
        .unwrap()
        .strip_suffix("}]")
        .unwrap();
    body.split("},{")
        .map(|r| json::decode(format!("{{{r}}}").as_bytes()).unwrap())
        .collect()
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 3);
    let d = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    let frozen = d.join("entradas");
    let input = d.join("comprometidas/entradas");
    assert_eq!(
        read(&frozen.join("HUELLAS.json")),
        read(&input.join("HUELLAS.json"))
    );
    let entries = manifest(&frozen.join("HUELLAS.json"));
    let mut names = BTreeSet::new();
    for e in &entries {
        let name = e.field("ruta").text().unwrap();
        assert!(names.insert(name));
        assert!(!name.starts_with('/') && !name.split('/').any(|x| x == ".."));
        let b = read(&input.join(name));
        assert_eq!(b.len(), e.field("bytes").uint().unwrap());
        assert_eq!(sha::hash(&b), e.field("sha256").text().unwrap());
    }
    let expected = read(&frozen.join("esperado.svg"));
    let math = doc(&frozen.join("matematico.json"));
    let mut campaigns = vec![];
    let mut integrated = 0;
    let mut instrumental = 0;
    for profile in ["debug", "optimizado"] {
        for mode in ["normal", "svg_permutado", "consumo_falso_svg"] {
            let run = format!("{profile}-{mode}");
            let result = d.join(format!("resultado-{run}"));
            let command = doc(&d.join(format!("comandos/campana-{run}.json")));
            let sum = doc(&result.join("RESUMEN.json"));
            let normal = mode == "normal";
            assert_eq!(
                command.field("retorno").uint(),
                Some(if normal { 0 } else { 1 })
            );
            assert_eq!(sum.field("casos").uint(), Some(if normal { 10 } else { 1 }));
            assert_eq!(
                sum.field("casos_discordantes").uint(),
                Some(if normal { 0 } else { 1 })
            );
            integrated += sum.field("integrados").uint().unwrap();
            instrumental += sum.field("instrumentales").uint().unwrap();
            let ids = if normal {
                doc(&input.join("CASOS.json"))
            } else {
                J::Array(vec![s("RC01")])
            };
            let mut cases = vec![];
            for id in ids.array().unwrap() {
                let id = id.text().unwrap();
                let observed = doc(&result.join(id).join("observado.json"));
                let faults = doc(&result.join(id).join("discrepancias.json"));
                if normal {
                    assert!(faults.array().unwrap().is_empty());
                }
                let mut proof = J::Null;
                if !normal {
                    assert_eq!(observed.field("guard").text(), Some(""));
                    assert_eq!(observed.field("consumos").uint(), Some(1));
                    assert!(!faults.array().unwrap().is_empty());
                    let cap = read(&result.join(id).join("captura.bin"));
                    let produced = read(&result.join(id).join("producido.svg"));
                    assert_eq!(
                        observed.field("hash_declarado_consumidor").text(),
                        Some(sha::hash(&cap).as_str())
                    );
                    assert_ne!(
                        observed.field("diferencias_consumidas"),
                        math.field("aristas_diferencia")
                    );
                    if mode == "consumo_falso_svg" {
                        assert_eq!(cap, expected);
                        assert_eq!(produced, expected);
                    } else {
                        assert_ne!(produced, expected);
                        assert_eq!(cap, produced);
                    }
                    proof = obj(vec![
                        ("captura_correcta", J::Bool(cap == expected)),
                        ("hash_declarado_corresponde_captura", J::Bool(true)),
                        ("operacion_discordante", J::Bool(true)),
                    ]);
                }
                cases.push(obj(vec![
                    ("id", s(id)),
                    ("guard", observed.field("guard").clone()),
                    ("leidos_svg", observed.field("leidos_svg").clone()),
                    ("consumos", observed.field("consumos").clone()),
                    ("discrepancias", faults),
                    ("contraste_mutante", proof),
                ]));
            }
            campaigns.push(obj(vec![
                ("perfil", s(profile)),
                ("modo", s(mode)),
                ("retorno", command.field("retorno").clone()),
                ("resumen", sum),
                ("casos", J::Array(cases)),
            ]));
        }
    }
    assert_eq!((integrated, instrumental), (22, 2));
    put(
        out,
        &obj(vec![
            ("registro", s("RETP-2026-239")),
            ("huellas_entradas_conformes", n(entries.len())),
            ("observaciones_normales", n(20)),
            ("observaciones_mutadas", n(4)),
            ("invocaciones_integradas", n(integrated)),
            ("pruebas_lector_aislado", n(instrumental)),
            ("campanas", J::Array(campaigns)),
            ("cierre_bis", J::Bool(false)),
            ("rasterizacion_acreditada", J::Bool(false)),
        ]),
    );
    println!("Custodia intacta: {} archivos. Normales: 20 concordancias. Mutantes: 4 discrepancias funcionales previstas. Sin nuevas ejecuciones.",entries.len());
}
