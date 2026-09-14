//! Edición acotada de tres archivos de Sucesos; no abre ni escribe RETP.
#[allow(dead_code)]
#[path = "../../../../r08/auxiliar.rs"]
mod auxiliar;
use std::{fs, path::Path};
use sv_bis_i0205::json::{self, J};
type R<T> = Result<T, String>;
const FILES: [&str; 3] = [
    "SUCESOS_SV.csv",
    "HISTORIAL_SUCESOS_SV.csv",
    "SUCESOS_SV.md",
];
fn need(b: bool, e: &str) -> R<()> {
    if b {
        Ok(())
    } else {
        Err(e.into())
    }
}
fn text(b: &[u8]) -> R<&str> {
    std::str::from_utf8(b).map_err(|e| e.to_string())
}
fn row(r: &[String]) -> String {
    r.iter()
        .map(|v| format!("\"{}\"", v.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(",")
        + "\n"
}
fn load(dir: &Path) -> R<Vec<Vec<u8>>> {
    FILES.iter().map(|n| auxiliar::read(&dir.join(n))).collect()
}
fn validate(b: &[Vec<u8>], a: &[Vec<u8>], rev: usize, updates: &J) -> R<()> {
    let old = auxiliar::csv(&b[0])?;
    let new = auxiliar::csv(&a[0])?;
    need(old[0] == new[0] && old.len() == new.len(), "forma")?;
    let targets: Vec<_> = old
        .iter()
        .enumerate()
        .filter(|(_, r)| r.first().is_some_and(|v| v == "S26"))
        .collect();
    need(targets.len() == 1, "S26_unico")?;
    let pos = targets[0].0;
    for i in 1..old.len() {
        if i != pos {
            need(old[i] == new[i], "otras_filas")?;
        }
    }
    let mut expected = old[pos].clone();
    for (k, v) in updates.pairs().ok_or("campos")? {
        need(k != "id", "identidad")?;
        let col = old[0].iter().position(|s| s == k).ok_or("columna")?;
        expected[col] = v.text().ok_or("texto")?.into();
    }
    need(new[pos] == expected, "edicion_exacta")?;
    let state = old[0].iter().position(|s| s == "estado").ok_or("estado")?;
    need(
        matches!(
            new[pos][state].as_str(),
            "pendiente" | "en ejecución" | "finalizado"
        ),
        "estado",
    )?;
    let hb = auxiliar::csv(&b[1])?;
    let ha = auxiliar::csv(&a[1])?;
    let last = hb
        .iter()
        .rev()
        .find(|r| r.get(1).is_some_and(|s| s == "S26"))
        .ok_or("historial")?;
    need(
        last[0]
            .parse::<usize>()
            .map_err(|_| "revision")?
            .checked_add(1)
            == Some(rev)
            && last[1..] == old[pos],
        "continuidad",
    )?;
    let mut hist = vec![rev.to_string()];
    hist.extend(expected);
    need(
        a[1].starts_with(&b[1])
            && ha.len() == hb.len() + 1
            && ha[..hb.len()] == hb
            && ha.last() == Some(&hist),
        "historial_append_only",
    )?;
    let om = text(&b[2])?;
    let nm = text(&a[2])?;
    let bounds = |s: &str| -> R<(usize, usize)> {
        let x = s.find("## S26 · ").ok_or("seccion")?;
        let y = s[x + 10..]
            .find("\n## ")
            .map(|n| x + 10 + n + 1)
            .unwrap_or(s.len());
        Ok((x, y))
    };
    let (x, y) = bounds(om)?;
    let (u, v) = bounds(nm)?;
    need(om[..x] == nm[..u] && om[y..] == nm[v..], "otras_secciones")?;
    for (k, val) in new[0].iter().zip(&new[pos]) {
        if k == "id" || k == "actividad" {
            continue;
        }
        let value = if val.is_empty() { "—" } else { val };
        need(
            nm[u..v].contains(&format!("**{k}:** {value}\n\n")),
            "markdown_concordante",
        )?;
    }
    Ok(())
}
fn run(before: &Path, input: &Path, out: &Path, check: bool) -> R<()> {
    let b = load(before)?;
    let j = json::decode(&auxiliar::read(input)?).map_err(|e| e.to_string())?;
    let rev = j.field("revision").uint().ok_or("revision")?;
    let updates = j.field("suceso");
    if check {
        return validate(&b, &load(out)?, rev, updates);
    }
    let table = auxiliar::csv(&b[0])?;
    let mut r = table
        .iter()
        .find(|r| r.first().is_some_and(|s| s == "S26"))
        .ok_or("S26")?
        .clone();
    for (k, v) in updates.pairs().ok_or("suceso")? {
        let i = table[0].iter().position(|s| s == k).ok_or("columna")?;
        r[i] = v.text().ok_or("valor")?.into();
    }
    need(
        r.iter().all(|s| !s.contains(['\n', '\r'])),
        "multilinea_fuera_perfil",
    )?;
    let mut a = b.clone();
    let csv = text(&b[0])?;
    let start = csv
        .find("\nS26,")
        .or_else(|| csv.find("\n\"S26\","))
        .ok_or("fila_fisica")?
        + 1;
    let end = start + csv[start..].find('\n').ok_or("fin_fila")? + 1;
    a[0] = [&csv[..start], &row(&r), &csv[end..]].concat().into_bytes();
    let mut hist = vec![rev.to_string()];
    hist.extend(r.clone());
    a[1].extend(row(&hist).as_bytes());
    let md = text(&b[2])?;
    let start = md.find("## S26 · ").ok_or("seccion")?;
    let end = md[start + 10..]
        .find("\n## ")
        .map(|n| start + 10 + n + 1)
        .unwrap_or(md.len());
    let title = table[0]
        .iter()
        .position(|s| s == "actividad")
        .ok_or("actividad")?;
    let mut section = format!("## S26 · {}\n\n", r[title]);
    for (k, val) in table[0].iter().zip(&r) {
        if k == "id" || k == "actividad" {
            continue;
        }
        section += &format!("**{k}:** {}\n\n", if val.is_empty() { "—" } else { val });
    }
    a[2] = [&md[..start], &section, &md[end..]].concat().into_bytes();
    validate(&b, &a, rev, updates)?;
    fs::create_dir(out).map_err(|e| e.to_string())?;
    for (n, bytes) in FILES.iter().zip(a) {
        fs::write(out.join(n), bytes).map_err(|e| e.to_string())?;
    }
    validate(&b, &load(out)?, rev, updates)
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 || !matches!(a[1].as_str(), "preparar" | "comprobar") {
        eprintln!("uso: sucesos preparar/comprobar ANTES ENTRADA_JSON SALIDA_NUEVA");
        std::process::exit(2);
    }
    match run(
        Path::new(&a[2]),
        Path::new(&a[3]),
        Path::new(&a[4]),
        a[1] == "comprobar",
    ) {
        Ok(()) => println!("SUCESOS_CONFORMES_RETP_NO_ABIERTA"),
        Err(e) => {
            eprintln!("RECHAZO: {e}");
            std::process::exit(1);
        }
    }
}
