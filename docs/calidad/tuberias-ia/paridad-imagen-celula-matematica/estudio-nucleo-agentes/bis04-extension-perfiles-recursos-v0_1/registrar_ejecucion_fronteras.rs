//! Registro administrativo de comandos y canales. Sin oráculos del SV.
#![forbid(unsafe_code)]
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
fn q(s: &str) -> String {
    let mut r = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => r.push_str("\\\""),
            '\\' => r.push_str("\\\\"),
            '\n' => r.push_str("\\n"),
            '\r' => r.push_str("\\r"),
            '\t' => r.push_str("\\t"),
            c if c < ' ' => r += &format!("\\u{:04x}", c as u32),
            _ => r.push(c),
        }
    }
    r.push('"');
    r
}
fn timestamp() -> String {
    String::from_utf8(
        Command::new("date")
            .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
            .output()
            .expect("date")
            .stdout,
    )
    .expect("UTF8")
    .trim()
    .into()
}
fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() < 4 {
        return Err("uso: ejecutar DIR NOMBRE PROGRAMA [ARGS]".into());
    }
    let dir = Path::new(&a[1]);
    let name = &a[2];
    if name.contains('/') || name.contains("..") {
        return Err("nombre".into());
    }
    let mut meta = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join(format!("{name}.json")))?;
    let out = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join(format!("{name}.stdout")))?;
    let err = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join(format!("{name}.stderr")))?;
    let start = timestamp();
    let epoch = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let timer = Instant::now();
    let result = Command::new(&a[3])
        .args(&a[4..])
        .stdin(Stdio::null())
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .status();
    let (rc, spawn_error) = match result {
        Ok(s) => (s.code().unwrap_or(128), String::new()),
        Err(e) => (127, e.to_string()),
    };
    let elapsed = timer.elapsed().as_nanos();
    let end = timestamp();
    let text=format!("{{\n  \"nombre\": {},\n  \"directorio\": {},\n  \"argv\": [{}],\n  \"inicio_utc\": {},\n  \"fin_utc\": {},\n  \"inicio_epoch_ns\": {},\n  \"duracion_ns\": {},\n  \"retorno\": {},\n  \"error_lanzamiento\": {},\n  \"stdout\": {},\n  \"stderr\": {}\n}}\n",q(name),q(&std::env::current_dir()?.to_string_lossy()),a[3..].iter().map(|s|q(s)).collect::<Vec<_>>().join(", "),q(&start),q(&end),epoch,elapsed,rc,q(&spawn_error),q(&format!("{name}.stdout")),q(&format!("{name}.stderr")));
    meta.write_all(text.as_bytes())?;
    meta.sync_all()?;
    println!("{name}: retorno={rc} inicio={start} fin={end}");
    if rc != 0 {
        print!(
            "{}",
            fs::read_to_string(dir.join(format!("{name}.stderr")))?
        );
    }
    let _ = File::open(dir.join(format!("{name}.stdout")))?;
    Ok(rc)
}
fn main() {
    match run() {
        Ok(rc) => std::process::exit(rc),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
}
