//! ES: Conserva huellas y resume evidencia ya producida; no llama al admisor.
//! EN: Preserves hashes and summarizes existing evidence; never calls admission.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/json.rs"]
mod json;
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/sha.rs"]
mod sha;
use json::{n, obj, s, J};
use std::{fs, path::Path};
fn doc(p: &Path) -> J {
    json::decode(&fs::read(p).unwrap()).unwrap()
}
// ES: Perfil plano del manifiesto generado; cada registro conserva la cuota JSON.
// EN: Flat generated-manifest profile; each record preserves the JSON quota.
fn manifest(p: &Path) -> J {
    let raw = fs::read_to_string(p).unwrap();
    assert!(raw.len() <= 65536);
    let body = raw
        .trim()
        .strip_prefix("[{")
        .unwrap()
        .strip_suffix("}]")
        .unwrap();
    let entries: Vec<_> = body
        .split("},{")
        .map(|r| json::decode(format!("{{{r}}}").as_bytes()).unwrap())
        .collect();
    let mut paths = std::collections::BTreeSet::new();
    for e in &entries {
        assert!(paths.insert(e.field("ruta").text().unwrap()));
    }
    J::Array(entries)
}
fn put(p: &Path, j: &J) {
    assert!(!p.exists());
    fs::write(p, json::encode(j, 4 * 1024 * 1024).unwrap()).unwrap();
}
fn inventory(root: &Path, p: &Path, v: &mut Vec<J>) {
    let mut paths: Vec<_> = fs::read_dir(p)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for f in paths {
        let m = fs::symlink_metadata(&f).unwrap();
        assert!(!m.file_type().is_symlink());
        if m.is_dir() {
            inventory(root, &f, v)
        } else {
            let b = fs::read(&f).unwrap();
            v.push(obj(vec![
                ("ruta", s(f.strip_prefix(root).unwrap().to_str().unwrap())),
                ("bytes", n(b.len())),
                ("sha256", s(&sha::hash(&b))),
            ]));
        }
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let d = Path::new(&a[2]);
    let out = Path::new(&a[3]);
    if a[1] == "inventario" {
        let mut v = vec![];
        inventory(d, d, &mut v);
        put(out, &J::Array(v));
        return;
    }
    assert_eq!(a[1], "resultados");
    let inp = d.join("comprometidas/entradas");
    let h = manifest(&inp.join("HUELLAS.json"));
    for e in h.array().unwrap() {
        let r = e.field("ruta").text().unwrap();
        assert!(!r.starts_with('/') && !r.split('/').any(|x| x == ".."));
        let b = fs::read(inp.join(r)).unwrap();
        assert_eq!(Some(b.len()), e.field("bytes").uint());
        assert_eq!(Some(sha::hash(&b).as_str()), e.field("sha256").text());
    }
    let ids = doc(&inp.join("CASOS.json"));
    assert_eq!(ids.array().unwrap().len(), 12);
    let mut campaigns = vec![];
    for p in ["debug", "optimizado"] {
        let dir = d.join(format!("resultado-{p}"));
        let sum = doc(&dir.join("RESUMEN.json"));
        assert_eq!(sum.field("discrepancias").uint(), Some(0));
        assert_eq!(sum.field("sensibilidades_detectadas").uint(), Some(4));
        let mut cases = vec![];
        let mut ok = 0;
        let mut reject = 0;
        for id in ids.array().unwrap() {
            let id = id.text().unwrap();
            let c = dir.join(id);
            let observed = doc(&c.join("observado.json"));
            assert_eq!(doc(&c.join("discrepancias.json")).array().unwrap().len(), 0);
            if observed.field("guard").text() == Some("") {
                ok += 1;
            } else {
                reject += 1;
            }
            cases.push(observed);
        }
        assert_eq!((ok, reject), (6, 6));
        campaigns.push(obj(vec![
            ("perfil", s(p)),
            ("resumen", sum),
            ("entregas", n(ok)),
            ("rechazos", n(reject)),
            ("casos", J::Array(cases)),
        ]));
    }
    put(
        out,
        &obj(vec![
            ("registro", s("RETP-2026-237")),
            ("variantes_unicas", n(12)),
            ("escenarios", n(6)),
            ("invocaciones", n(24)),
            ("huellas_de_entrada_releidas", n(h.array().unwrap().len())),
            ("campanas", J::Array(campaigns)),
            ("guarda_agregada_rechazo_ejecutada", J::Bool(false)),
            ("cierre_bis", J::Bool(false)),
        ]),
    );
    println!(
        "Huellas de entrada intactas: {}; dos campañas 12/12; ninguna invocación nueva.",
        h.array().unwrap().len()
    );
}
