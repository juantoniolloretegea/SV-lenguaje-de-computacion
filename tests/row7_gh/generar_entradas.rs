//! Preparación documental autónoma. Compilar con rustc, sin importar sv_core.
mod documentary_json;
use documentary_json::{parse, Value};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const SOURCE_SHA256: &str = "037944fe4ca28524d7e89403d08e881462f7ec349c21e7d9127ccf4c7c7b1f62";
const ROOT: &str = "tests/row7_gh";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn sha256(bytes: &[u8]) -> Result<String> {
    // Dependencia de preparación explícita: GNU Coreutils. No se introduce otra criptografía.
    let mut child = Command::new("sha256sum").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    child.stdin.take().ok_or("SHA256_ENTRADA")?.write_all(bytes)?;
    let result = child.wait_with_output()?;
    if !result.status.success() { return Err("SHA256_PROCESO".into()); }
    let text = std::str::from_utf8(&result.stdout)?;
    let digest = text.strip_suffix("  -\n").ok_or("SHA256_FORMATO")?;
    if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err("SHA256_FORMATO".into());
    }
    Ok(digest.to_owned())
}
fn literal(text: &str) -> Result<String> {
    if text.contains("\"###") { return Err("GH_DELIMITADOR_LITERAL".into()); }
    Ok(format!("r###\"{text}\"###"))
}
fn generate(bytes: &[u8]) -> Result<String> {
    if sha256(bytes)? != SOURCE_SHA256 { return Err("GH_FUENTE_ALTERADA".into()); }
    let source = parse(bytes)?;
    let mut lines = vec![
        "// Generado exclusivamente desde el testigo G/H: generar_entradas.rs --check.".to_owned(),
        "pub const INPUTS: &[Input] = &[".to_owned(),
    ];
    for witness in source.get("witnesses")?.array()? {
        let operation = witness.get("operation")?;
        let definition = operation.compact();
        for state in witness.get("domain")?.get("states")?.array()? {
            let packet = state.get("packet")?;
            let full = packet.compact();
            let reduced = Value::compact_members(packet.members()?.iter().filter(|(k, _)| k != "detail").map(|(k, v)| (k.as_str(), v)));
            let side = Value::compact_members(std::iter::once(("detail", packet.get("detail")?)));
            for variant in ["F0", "H", "HS"] {
                let payload = if variant == "F0" { &full } else { &reduced };
                let lateral = if variant == "HS" { side.as_str() } else { "" };
                let definition_sha = sha256(definition.as_bytes())?;
                let payload_sha = sha256(payload.as_bytes())?;
                let side_sha = sha256(lateral.as_bytes())?;
                let fields = [
                    ("witness", witness.get("id")?.text()?), ("state", state.get("id")?.text()?),
                    ("variant", variant), ("operation", operation.get("id")?.text()?),
                    ("definition", definition.as_str()), ("payload", payload.as_str()), ("side", lateral),
                    ("definition_sha", &definition_sha), ("payload_sha", &payload_sha), ("side_sha", &side_sha),
                ];
                let rendered = fields.iter().map(|(key, value)| Ok(format!("{key}: {}", literal(value)?))).collect::<Result<Vec<_>>>()?;
                lines.push(format!("    Input {{ {} }},", rendered.join(", ")));
            }
        }
    }
    lines.extend(["];".to_owned(), String::new()]);
    Ok(lines.join("\n"))
}
fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() && args != ["--check"] { return Err("Uso desde la raíz del repositorio: generar_entradas [--check]".into()); }
    let root = Path::new(ROOT);
    let text = generate(&std::fs::read(root.join("testigos-gh.json"))?)?;
    let target = root.join("entradas.rs");
    if args == ["--check"] {
        if std::fs::read(&target)? != text.as_bytes() { return Err("GH_ENTRADAS_DIVERGENTES".into()); }
        println!("GH-LIG: 48 entradas regenerables desde el testigo G/H exacto");
    } else { std::fs::write(target, text)?; }
    Ok(())
}
fn main() {
    if let Err(error) = run() { eprintln!("{error}"); std::process::exit(1); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_source_regenerates_all_committed_input_bytes() {
        let generated = generate(include_bytes!("testigos-gh.json")).unwrap();
        assert_eq!(generated, include_str!("entradas.rs"));
        assert_eq!(generated.lines().filter(|line| line.starts_with("    Input {")).count(), 48);
    }
    #[test]
    fn source_and_literal_controls_have_their_own_causes() {
        assert_eq!(generate(b"{}").unwrap_err().to_string(), "GH_FUENTE_ALTERADA");
        assert_eq!(literal("\"###").unwrap_err().to_string(), "GH_DELIMITADOR_LITERAL");
        assert_eq!(sha256(b"").unwrap(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }
}
