//! ES: Recepción acotada de S22/S26 con historial; no ejecuta pruebas SV.
//! EN: Bounded S22/S26 reception with history preservation; no SV tests.
#![forbid(unsafe_code)]
#[allow(dead_code)]
#[path = "../../../../riesgos-materiales-s26/r08/auxiliar.rs"]
mod auxiliar;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn row(v: &[String]) -> String {
    v.iter()
        .map(|s| format!("\"{}\"", s.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(",")
        + "\n"
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 6 {
        return Err("uso: actualizar ROOT ID REV CAMBIOS_TSV COPIA_PREVIA".into());
    }
    let root = Path::new(&a[1]);
    let id = &a[2];
    if !matches!(id.as_str(), "S26" | "S22") {
        return Err("id fuera del alcance".into());
    }
    let rev: u64 = a[3].parse()?;
    let dir = root.join("docs/calidad/Inventario-sv/sucesos");
    let names = [
        "SUCESOS_SV.csv",
        "HISTORIAL_SUCESOS_SV.csv",
        "SUCESOS_SV.md",
    ];
    let b: Vec<Vec<u8>> = names
        .iter()
        .map(|n| fs::read(dir.join(n)))
        .collect::<Result<_, _>>()?;
    let mut c = auxiliar::csv(&b[0])?;
    let hist = auxiliar::csv(&b[1])?;
    let mut ids = BTreeSet::new();
    for r in &c[1..] {
        if !ids.insert(&r[0]) {
            return Err("id duplicado".into());
        }
    }
    let pos = c.iter().position(|r| &r[0] == id).ok_or("id ausente")?;
    let old = c[pos].clone();
    let latest = hist[1..]
        .iter()
        .filter(|r| &r[1] == id)
        .max_by_key(|r| r[0].parse::<u64>().unwrap());
    let latest = latest.ok_or("historial ausente")?;
    if latest[0].parse::<u64>()? + 1 != rev || latest[1..] != old {
        return Err("revision o concordancia".into());
    }
    let mut changes = BTreeMap::new();
    let input = fs::read_to_string(&a[4])?;
    for l in input.lines() {
        let (k, v) = l.split_once('\t').ok_or("TSV")?;
        if changes.insert(k, v).is_some() {
            return Err("campo repetido".into());
        }
    }
    for (k, v) in changes {
        if matches!(k, "id" | "fecha_alta_utc" | "fecha_inicio_utc") {
            return Err("campo protegido".into());
        }
        let i = c[0].iter().position(|h| h == k).ok_or("campo")?;
        c[pos][i] = v.into();
    }
    if !matches!(
        c[pos][1].as_str(),
        "pendiente" | "en ejecución" | "finalizado"
    ) {
        return Err("estado".into());
    }
    let mut csv = String::new();
    let oldtext = std::str::from_utf8(&b[0])?;
    let lines: Vec<_> = oldtext.split_inclusive('\n').collect();
    if lines.len() != c.len() {
        return Err("perfil fisico CSV".into());
    }
    for (i, l) in lines.iter().enumerate() {
        if i == pos {
            csv += &row(&c[i]);
        } else {
            csv += l;
        }
    }
    let mut h = b[1].clone();
    let mut hr = vec![rev.to_string()];
    hr.extend(c[pos].clone());
    h.extend(row(&hr).as_bytes());
    let md = std::str::from_utf8(&b[2])?;
    let heading = format!("## {id} · ");
    let start = md.find(&heading).ok_or("MD")?;
    let end = md[start + heading.len()..]
        .find("\n## ")
        .map(|n| start + heading.len() + n + 1)
        .unwrap_or(md.len());
    let activity = c[0]
        .iter()
        .position(|s| s == "actividad")
        .ok_or("actividad")?;
    let mut section = format!("{heading}{}\n\n", c[pos][activity]);
    for (k, v) in c[0].iter().zip(&c[pos]) {
        if k == "id" || k == "actividad" {
            continue;
        }
        section += &format!("**{k}:** {}\n\n", if v.is_empty() { "—" } else { v });
    }
    let m = [&md[..start], &section, &md[end..]].concat();
    let ac = auxiliar::csv(csv.as_bytes())?;
    let ah = auxiliar::csv(&h)?;
    if ac != c || !h.starts_with(&b[1]) || ah.last().ok_or("hist")?[1..] != c[pos] {
        return Err("cotejo candidato".into());
    }
    fs::create_dir(&a[5])?;
    for (n, bytes) in names.iter().zip(&b) {
        fs::write(Path::new(&a[5]).join(n), bytes)?;
    }
    for (n, bytes) in names
        .iter()
        .zip([csv.as_bytes(), h.as_slice(), m.as_bytes()])
    {
        fs::write(dir.join(n), bytes)?;
        if fs::read(dir.join(n))? != bytes {
            return Err("relectura".into());
        }
    }
    println!("{id} revision={rev}; CSV/MD/historial concordantes; otras filas y prefijo historico conservados; RETP no accedida.");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
