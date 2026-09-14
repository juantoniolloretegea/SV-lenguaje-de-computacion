//! ES: Prepara entradas SVG y oráculos independientes; no enlaza al admisor.
//! EN: Prepares SVG inputs and independent oracles; never links admission.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/json.rs"]
mod json;
#[path = "../bis04-realizacion-i0205-v0_1/proyecto/src/sha.rs"]
mod sha;
use json::{n, obj, s, J};
use std::{fs, path::Path};
fn read(p: &Path) -> Vec<u8> {
    fs::read(p).unwrap()
}
fn doc(p: &Path) -> J {
    json::decode(&read(p)).unwrap()
}
fn write(p: &Path, j: &J) {
    fs::write(p, json::encode(j, 65536).unwrap()).unwrap();
}
fn inventory(root: &Path, p: &Path, out: &mut Vec<J>) {
    let mut paths: Vec<_> = fs::read_dir(p)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for f in paths {
        if f.is_dir() {
            inventory(root, &f, out)
        } else {
            let b = read(&f);
            out.push(obj(vec![
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
    let seeds = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    let corpus = Path::new(&a[3]);
    assert!(!out.exists());
    fs::create_dir(out).unwrap();
    let svg = read(&corpus.join("ORACULO_SVG_CONSUMO.svg"));
    let math = doc(&corpus.join("ORACULO_SVG_CONSUMO.json"));
    let mut alternate = String::from_utf8(svg.clone()).unwrap();
    let old = "1000000,0 1847760,-765366";
    assert_eq!(alternate.matches(old).count(), 1);
    alternate = alternate.replace(old, "1847760,-765366 1000000,0");
    fs::write(out.join("alternativo.svg"), alternate).unwrap();
    fs::write(out.join("esperado.svg"), &svg).unwrap();
    write(&out.join("matematico.json"), &math);
    // ES: Cota racional de las tres constantes; sólo aritmética entera i128.
    // EN: Rational bound on the three constants; i128 integer arithmetic only.
    let d = 1_000_000i128;
    let d2 = d * d;
    let t = 2 * d2 * d2;
    let c_lo = 4 * 923879i128.pow(2) - 2 * d2;
    let c_hi = 4 * 923881i128.pow(2) - 2 * d2;
    let s_lo = 2 * d2 - 4 * 382684i128.pow(2);
    let s_hi = 2 * d2 - 4 * 382682i128.pow(2);
    assert!(c_lo > 0 && s_lo > 0);
    assert!(c_lo * c_lo < t && t < c_hi * c_hi);
    assert!(s_lo * s_lo < t && t < s_hi * s_hi);
    assert!(2 * 707106i128.pow(2) < d2 && d2 < 2 * 707108i128.pow(2));
    write(
        &out.join("COTA_ENTERA.json"),
        &obj(vec![
            ("escala", n(1000000)),
            (
                "error_coordenada_menor_que",
                s("3/1000000 de unidad canonica para radio <= 3"),
            ),
            ("cotas_radicales", J::Bool(true)),
            ("tipo_aritmetico", s("i128")),
            ("estado_ternario_modificado", J::Bool(false)),
        ]),
    );
    let points = math.field("coordenadas").array().unwrap();
    let edges = math.field("aristas_diferencia").array().unwrap();
    assert_eq!(points.len(), 16);
    assert_eq!(edges.len(), 16);
    for i in 0..16 {
        for k in 0..2 {
            let value = |j: &J| j.lexeme().unwrap().parse::<i64>().unwrap();
            let x = value(&points[(i + 1) % 16].array().unwrap()[k])
                - value(&points[i].array().unwrap()[k]);
            assert_eq!(x, value(&edges[i].array().unwrap()[k]));
        }
    }
    let mut cases = vec![];
    for i in 1..=10 {
        let id = format!("RC{i:02}");
        let dst = out.join(&id);
        fs::create_dir(&dst).unwrap();
        let seed = seeds.join(if i == 2 { "ES02" } else { "ES01" });
        for name in [
            "source.bin",
            "state.bin",
            "support.bin",
            "constitution.bin",
            "convention.bin",
            "transforms.bin",
            "registry.json",
            "geometry.bin",
        ] {
            fs::write(dst.join(name), read(&seed.join(name))).unwrap();
        }
        let mut q = doc(&seed.join("request.json"));
        let mut c = doc(&seed.join("context.json"));
        c.set("id", s(&id));
        let mut delivery = q.field("entrega").clone();
        delivery.set("invocacion", s(&id));
        if i == 3 {
            let mut binding = q.field("vinculo").clone();
            binding.set("revision", s("r2"));
            q.set("vinculo", binding.clone());
            c.set("vinculo", binding);
            delivery.set("revision", s("r2"));
        }
        q.set("entrega", delivery.clone());
        c.set("entrega", delivery.clone());
        write(&dst.join("request.json"), &q);
        write(&dst.join("context.json"), &c);
        let envelope = obj(vec![
            ("formato", s("SV-SVG16-MATERIAL/1")),
            ("identidad", c.field("vinculo").clone()),
            ("entrega_origen", delivery),
            (
                "descriptor_sha256",
                s(&sha::hash(&read(&seed.join("geometry.bin")))),
            ),
            ("proyeccion", s("micro16-pantalla/1")),
            ("operacion", s("diferencias-ciclicas-svg/1")),
            ("consumidor", s("consumidor-svg16-local/1")),
            ("svg_sha256", s(&sha::hash(&svg))),
            ("svg_bytes", n(svg.len())),
        ]);
        write(&dst.join("envoltura_esperada.json"), &envelope);
        let mut offered = envelope.clone();
        if i == 5 {
            let mut v = offered.field("identidad").clone();
            v.set("revision", s("r2"));
            offered.set("identidad", v);
        }
        if i == 6 {
            let mut v = offered.field("entrega_origen").clone();
            v.set("invocacion", s("RC06-ajena"));
            offered.set("entrega_origen", v);
        }
        write(&dst.join("envoltura_ofrecida.json"), &offered);
        let channel = match i {
            4 => String::from_utf8(svg.clone())
                .unwrap()
                .replacen("1000000,0 ", "1000001,0 ", 1)
                .into_bytes(),
            8 => vec![b' '; 4097],
            10 => String::from_utf8(svg.clone())
                .unwrap()
                .replacen("<title>", "<script>alert(1)</script><title>", 1)
                .into_bytes(),
            _ => svg.clone(),
        };
        fs::write(dst.join("canal.svg"), &channel).unwrap();
        let guard = match i {
            1..=3 => "",
            4 => "C04_BYTES",
            5 | 6 => "C02_CONTEXTO",
            7 => "C01_AUSENCIA",
            8 => "C03_CUOTA",
            9 => "C03_IO",
            10 => "C05_PERFIL",
            _ => unreachable!(),
        };
        let count = match i {
            5 | 6 | 7 | 10 => 0,
            8 => 4097,
            9 => 64,
            _ => channel.len(),
        };
        write(
            &dst.join("oraculo.json"),
            &obj(vec![
                ("id", s(&id)),
                ("guard", s(guard)),
                ("leidos_svg", n(count)),
                ("consumos", n(if i <= 3 { 1 } else { 0 })),
                ("instrumental", J::Bool(i == 10)),
                ("estado_preservado", J::Bool(true)),
            ]),
        );
        cases.push(s(&id));
    }
    write(&out.join("CASOS.json"), &J::Array(cases));
    let mut hashes = vec![];
    inventory(out, out, &mut hashes);
    write(&out.join("HUELLAS.json"), &J::Array(hashes));
    println!("10 casos preparados (9 integrados, 1 instrumental); cota entera verificada; 0 llamadas de admisión, renderizado o consumo.");
}
