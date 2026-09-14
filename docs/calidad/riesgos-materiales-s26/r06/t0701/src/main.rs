//! T07: escrituras reales con barreras síncronas, copia admitida y referentes.
//! No prueba concurrencia hostil, continuidad física ni durabilidad.
#[allow(dead_code)]
#[path = "../../local01/src/recepcion.rs"]
mod recepcion;
use recepcion::{Fault, Profile, Session, Stop};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};
use sv_bis_i0205::{
    admit,
    json::{self, n, obj, s, J},
    sha, ReceivedBytes, TrustedContext, TrustedRegistry,
};
type R<T> = Result<T, String>;
const LIMIT: usize = 8192;
const SPLIT: usize = 3072;

fn need(ok: bool, why: &str) -> R<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn read(p: &Path, max: usize) -> R<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(p)
        .map_err(|e| e.to_string())?
        .take((max + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    need(bytes.len() <= max, "cuota_lectura")?;
    Ok(bytes)
}
fn write_new(p: &Path, bytes: &[u8]) -> R<()> {
    need(bytes.len() <= 262144, "cuota_salida")?;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .map_err(|e| e.to_string())?;
    f.write_all(bytes).map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())
}
fn encode(j: &J) -> R<Vec<u8>> {
    json::encode(j, 262144).map_err(str::to_owned)
}
fn save(p: &Path, j: &J) -> R<()> {
    write_new(p, &encode(j)?)
}
fn doc(p: &Path) -> R<J> {
    json::decode(&read(p, LIMIT)?).map_err(str::to_owned)
}
fn rw(p: &Path) -> R<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(p)
        .map_err(|e| e.to_string())
}
fn replace_in_place(f: &mut File, bytes: &[u8]) -> R<()> {
    need(bytes.len() <= LIMIT, "cuota_escritura")?;
    f.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    f.write_all(bytes).map_err(|e| e.to_string())?;
    f.set_len(bytes.len() as u64).map_err(|e| e.to_string())
}
fn identity(f: &File) -> R<(u64, u64)> {
    let m = f.metadata().map_err(|e| e.to_string())?;
    Ok((m.dev(), m.ino()))
}
struct Ledger {
    dir: PathBuf,
    events: Vec<J>,
}
impl Ledger {
    fn mark(&mut self, name: &str) -> R<()> {
        need(self.events.len() < 32, "cuota_barreras")?;
        self.events.push(obj(vec![
            ("paso", n(self.events.len() + 1)),
            ("barrera", s(name)),
        ]));
        Ok(())
    }
    fn sample(&mut self, name: &str, f: &mut File) -> R<Vec<u8>> {
        let id = identity(f)?;
        f.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        f.take((LIMIT + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        need(bytes.len() <= LIMIT, "cuota_testigo")?;
        write_new(&self.dir.join(format!("{name}.bin")), &bytes)?;
        self.mark(name)?;
        let event = self.events.last_mut().ok_or("barrera_ausente")?;
        event.set("dev", s(&id.0.to_string()));
        event.set("inode", s(&id.1.to_string()));
        event.set("bytes", n(bytes.len()));
        event.set("sha256", s(&sha::hash(&bytes)));
        Ok(bytes)
    }
}
// La primera lectura se corta exactamente a 3072 bytes. La escritura se produce
// al solicitar el siguiente tramo. No depende de tiempos ni del planificador.
struct SplitReader<'a> {
    input: File,
    writer: File,
    original: &'a [u8],
    mode: u8,
    raw: Vec<u8>,
    injected: bool,
    restored: bool,
    ledger: &'a mut Ledger,
}
impl Read for SplitReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let error = |s: String| io::Error::other(s);
        if buf.is_empty() {
            return Ok(0);
        }
        if self.mode != 0 && self.raw.len() == SPLIT && !self.injected {
            self.ledger.mark("prefijo_leido").map_err(error)?;
            let mut b = self.original.to_vec();
            let pos = if self.mode == 1 { b.len() - 1 } else { 0 };
            b[pos] = b'!';
            replace_in_place(&mut self.writer, &b).map_err(error)?;
            self.ledger
                .sample("durante_lectura_B", &mut self.writer)
                .map_err(error)?;
            self.injected = true;
        }
        let remaining = LIMIT + 1 - self.raw.len();
        if remaining == 0 {
            return Err(io::Error::other("cuota_canal_testigo"));
        }
        let len = if self.mode != 0 && self.raw.len() < SPLIT {
            buf.len().min(SPLIT - self.raw.len()).min(remaining)
        } else {
            buf.len().min(remaining)
        };
        let count = self.input.read(&mut buf[..len])?;
        self.raw.extend_from_slice(&buf[..count]);
        if self.injected && self.raw.len() == self.original.len() && !self.restored {
            self.ledger.mark("sufijo_leido").map_err(error)?;
            replace_in_place(&mut self.writer, self.original).map_err(error)?;
            self.ledger
                .sample("restitucion_durante_lectura_A", &mut self.writer)
                .map_err(error)?;
            self.restored = true;
        }
        Ok(count)
    }
}
fn r2(j: &mut J, key: &str) {
    let mut x = j.field(key).clone();
    x.set("revision", s("r2"));
    j.set(key, x);
}
fn check(rows: &mut Vec<J>, name: &str, ok: bool) {
    rows.push(obj(vec![("control", s(name)), ("conforme", J::Bool(ok))]));
}

fn case(fixture: &Path, out: &Path, case_no: usize, session: &mut Session) -> R<bool> {
    let id = format!("P{case_no:02}");
    let dir = out.join(&id);
    fs::create_dir(&dir).map_err(|e| e.to_string())?;
    for name in ["source.bin", "state.bin", "support.bin", "geometry.bin"] {
        write_new(&dir.join(name), &read(&fixture.join(name), LIMIT)?)?;
    }
    let original = read(&fixture.join("geometry.bin"), LIMIT)?;
    let state_a = read(&fixture.join("state.bin"), 1024)?;
    let state_b = read(&fixture.join("vector-alterado.bin"), 1024)?;
    need(
        original.len() == 6237 && original.last() == Some(&b'\n') && state_a != state_b,
        "fixture_previo",
    )?;
    let mut custody = doc(&fixture.join("context.json"))?;
    let mut meta = doc(&fixture.join("request.json"))?;
    let plan = doc(&fixture.join("plan.json"))?;
    let mut expected_receipt = doc(&fixture.join("oracle.json"))?
        .field("recibo_esperado")
        .clone();
    if case_no == 9 || case_no == 10 {
        r2(&mut meta, "vinculo");
        r2(&mut meta, "entrega");
    }
    if case_no == 9 {
        r2(&mut custody, "vinculo");
        r2(&mut custody, "entrega");
        r2(&mut expected_receipt, "vinculo");
        r2(&mut expected_receipt, "contexto_entrega");
    }
    if case_no == 12 {
        r2(&mut meta, "entrega");
    }
    let profile = if case_no == 4 {
        Profile::SupportContinuity
    } else {
        Profile::ContentRefs
    };
    let ctx = TrustedContext::from_custody(&encode(&custody)?)?;
    let reg = TrustedRegistry::from_custody(
        &read(&fixture.join("registry.json"), LIMIT)?,
        &read(&fixture.join("constitution.bin"), LIMIT)?,
        &read(&fixture.join("convention.bin"), LIMIT)?,
        &read(&fixture.join("transforms.bin"), LIMIT)?,
    )?;
    let opening = session.open(&id, profile, custody.clone())?;
    save(
        &dir.join("apertura.json"),
        &obj(vec![
            ("unidad", s("W-S26")),
            ("banco", s("S26-R06-T0701")),
            ("intento", s(&id)),
            ("perfil", s(profile.name())),
            ("custodia", custody),
        ]),
    )?;
    let mut ledger = Ledger {
        dir: dir.clone(),
        events: vec![],
    };
    ledger.mark("apertura_conservada")?;
    let mut held = File::open(dir.join("geometry.bin")).map_err(|e| e.to_string())?;
    let object = identity(&held)?;
    let before = ledger.sample("antes_geometria_A", &mut held)?;
    let mut state_writer = rw(&dir.join("state.bin"))?;
    let state_object = identity(&state_writer)?;
    ledger.sample("antes_estado_A", &mut state_writer)?;
    if case_no == 7 || case_no == 13 {
        replace_in_place(&mut state_writer, &state_b)?;
        ledger.sample("estado_B_antes_recepcion", &mut state_writer)?;
        if case_no == 13 {
            let mut asset = meta.field("estado_matematico").clone();
            asset.set("sha256", s(&sha::hash(&state_b)));
            asset.set("bytes", n(state_b.len()));
            meta.set("estado_matematico", asset);
        }
    }
    save(&dir.join("request.json"), &meta)?;
    let mut geometry = SplitReader {
        input: File::open(dir.join("geometry.bin")).map_err(|e| e.to_string())?,
        writer: rw(&dir.join("geometry.bin"))?,
        original: &original,
        mode: if case_no == 5 {
            1
        } else if case_no == 6 {
            2
        } else {
            0
        },
        raw: vec![],
        injected: false,
        restored: false,
        ledger: &mut ledger,
    };
    let input = |name: &str| File::open(dir.join(name)).map_err(|e| e.to_string());
    let received = ReceivedBytes::read(
        &mut input("request.json")?,
        &mut input("source.bin")?,
        &mut input("state.bin")?,
        Some(&mut input("support.bin")?),
        &mut geometry,
    )
    .map_err(|e| format!("recepcion {}: {}", e.guard, e.cause))?;
    let raw = std::mem::take(&mut geometry.raw);
    let injected = geometry.injected;
    let restored = geometry.restored;
    drop(geometry);
    write_new(&dir.join("geometria_recibida.bin"), &raw)?;
    ledger.mark("cinco_piezas_recibidas")?;
    if case_no == 8 {
        replace_in_place(&mut state_writer, &state_b)?;
        ledger.sample("estado_B_tras_recepcion", &mut state_writer)?;
    }
    let admitted = admit(received, &ctx, &reg);
    let mut guard: Option<String> = None;
    let mut observation = None;
    match admitted {
        Err(e) => {
            guard = Some(e.guard.into());
            save(
                &dir.join("rechazo_admision.json"),
                &obj(vec![("guarda", s(e.guard)), ("causa", s(&e.cause))]),
            )?;
            ledger.mark("admision_rechazada")?;
        }
        Ok(a) => {
            ledger.mark("admision_observada")?;
            write_new(&dir.join("descriptor_admitido.bin"), a.descriptor())?;
            if matches!(case_no, 2..=4) {
                let mut writer = rw(&dir.join("geometry.bin"))?;
                let mut b = original.clone();
                b[0] = b'!';
                replace_in_place(&mut writer, &b)?;
                ledger.sample("tras_admision_B", &mut held)?;
                if case_no == 3 || case_no == 4 {
                    replace_in_place(&mut writer, &original)?;
                    ledger.sample("restitucion_tras_admision_A", &mut held)?;
                }
                need(identity(&writer)? == object, "objeto_escritura_distinto")?;
            }
            let mut context = plan.field("contexto_captura").clone();
            if case_no == 9 || case_no == 11 {
                context.set("revision", s("r2"));
            }
            let obs = recepcion::run(
                opening,
                a,
                &reg,
                plan.field("captor").text().ok_or("captor")?.into(),
                context,
                Stop::None,
                Fault::None,
                || Ok(()),
                || Ok(()),
            );
            if let Some(cap) = &obs.capture {
                write_new(&dir.join("consumido.bin"), &cap.bytes)?;
            }
            save(&dir.join("recepcion.json"), &obs.documentary())?;
            ledger.mark("retorno_recepcion_local")?;
            observation = Some(obs);
        }
    }
    let after = ledger.sample("despues_geometria", &mut held)?;
    let state_after = ledger.sample("despues_estado", &mut state_writer)?;
    let same_object = identity(&held)? == object
        && identity(&File::open(dir.join("geometry.bin")).map_err(|e| e.to_string())?)? == object;
    let mut checks = vec![];
    check(&mut checks, "mismo_objeto_geometria", same_object);
    check(
        &mut checks,
        "mismo_objeto_estado",
        identity(&state_writer)? == state_object,
    );
    check(&mut checks, "lectura_inicial_literal", before == original);
    let expected_guard = match case_no {
        5 => Some("M01"),
        7 | 13 => Some("I03"),
        10 => Some("I02"),
        12 => Some("A01"),
        _ => None,
    };
    check(
        &mut checks,
        "primera_guarda_literal",
        guard.as_deref() == expected_guard,
    );
    if expected_guard.is_some() {
        check(
            &mut checks,
            "rechazo_antes_consumidor",
            observation.is_none() && !dir.join("consumido.bin").exists(),
        );
    } else if let Some(obs) = &observation {
        if case_no == 4 {
            check(
                &mut checks,
                "pretension_superior_sin_degradacion",
                obs.termination == "NO_EJECUTADO"
                    && obs.cause.as_deref() == Some("continuidad_soporte_no_ofrecida")
                    && obs.capture.is_none()
                    && obs.accepted.is_none(),
            );
        } else if case_no == 11 {
            check(
                &mut checks,
                "captura_otro_corte_rechazada",
                obs.termination == "RETORNO_ERROR"
                    && obs.cause.as_deref() == Some("D04: capture context mismatch")
                    && obs.accepted.is_none(),
            );
            check(
                &mut checks,
                "bytes_iguales_no_suprimen_revision",
                obs.capture.as_ref().is_some_and(|c| {
                    c.bytes == original && c.context.field("revision").text() == Some("r2")
                }),
            );
        } else {
            check(
                &mut checks,
                "recibo_literal_previo",
                obs.status == "INFORME_LOCAL_ACEPTADO"
                    && obs.termination == "RETORNO"
                    && obs.cause.is_none()
                    && obs.accepted.as_ref() == Some(&expected_receipt),
            );
            check(
                &mut checks,
                "consumo_del_descriptor_admitido",
                obs.capture.as_ref().is_some_and(|c| c.bytes == original)
                    && read(&dir.join("consumido.bin"), LIMIT)?
                        == read(&dir.join("descriptor_admitido.bin"), LIMIT)?,
            );
        }
    } else {
        check(&mut checks, "recepcion_esperada", false);
    }
    let mut expected_after = original.clone();
    if case_no == 2 {
        expected_after[0] = b'!';
    }
    check(
        &mut checks,
        "contenido_final_previsto",
        after == expected_after,
    );
    if matches!(case_no, 2..=4) {
        let mid = read(&dir.join("tras_admision_B.bin"), LIMIT)?;
        check(
            &mut checks,
            "mutacion_material_observada",
            mid != original && mid.len() == original.len() && mid[0] == b'!',
        );
    }
    if case_no == 5 || case_no == 6 {
        let mut expected_raw = original.clone();
        if case_no == 5 {
            let len = expected_raw.len();
            expected_raw[len - 1] = b'!';
        }
        check(
            &mut checks,
            "barreras_de_lectura",
            injected
                && restored
                && ledger
                    .events
                    .iter()
                    .any(|j| j.field("barrera").text() == Some("prefijo_leido")),
        );
        check(
            &mut checks,
            "bytes_intermedios_y_recibidos",
            raw == expected_raw && read(&dir.join("durante_lectura_B.bin"), LIMIT)? != original,
        );
        check(
            &mut checks,
            "extremos_iguales_con_mutacion_intermedia",
            before == after && restored,
        );
    }
    let expected_state = if matches!(case_no, 7 | 8 | 13) {
        &state_b
    } else {
        &state_a
    };
    check(
        &mut checks,
        "dependencia_material_observada",
        &state_after == expected_state,
    );
    check(
        &mut checks,
        "identidad_en_cada_muestra",
        ledger
            .events
            .iter()
            .filter(|j| j.field("dev").text().is_some())
            .all(|j| {
                let name = j.field("barrera").text().unwrap_or("");
                let expected = if name.contains("estado") {
                    state_object
                } else {
                    object
                };
                j.field("dev").text() == Some(expected.0.to_string().as_str())
                    && j.field("inode").text() == Some(expected.1.to_string().as_str())
            }),
    );
    let ok = checks
        .iter()
        .all(|r| r.field("conforme").boolean() == Some(true));
    save(
        &dir.join("observacion.json"),
        &obj(vec![
            ("caso", s(&id)),
            ("controles", J::Array(checks)),
            ("extremos_geometria_iguales", J::Bool(before == after)),
            ("mismo_objeto", J::Bool(same_object)),
            ("continuidad_no_ofrecida_por_el_montaje", J::Bool(true)),
            ("barreras", J::Array(ledger.events)),
            ("conforme", J::Bool(ok)),
        ]),
    )?;
    println!("{id}: {}", if ok { "CONFORME" } else { "DISCREPANTE" });
    Ok(ok)
}
fn campaign(fixture: &Path, out: &Path) -> R<bool> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let mut session = Session::new();
    let mut rows = vec![];
    for no in 1..=13 {
        match case(fixture, out, no, &mut session) {
            Ok(ok) => rows.push(obj(vec![
                ("caso", s(&format!("P{no:02}"))),
                ("conforme", J::Bool(ok)),
            ])),
            Err(e) => {
                save(
                    &out.join("interrupcion.json"),
                    &obj(vec![
                        ("caso", s(&format!("P{no:02}"))),
                        ("error", s(&e)),
                        ("anteriores", J::Array(rows)),
                    ]),
                )?;
                return Err(e);
            }
        }
    }
    let count = rows
        .iter()
        .filter(|j| j.field("conforme").boolean() == Some(true))
        .count();
    save(
        &out.join("resultado.json"),
        &obj(vec![
            ("banco", s("S26-R06-T0701")),
            ("unidad", s("W-S26")),
            ("casos", J::Array(rows)),
            ("conformes", n(count)),
            ("total", n(13)),
            ("conforme", J::Bool(count == 13)),
        ]),
    )?;
    Ok(count == 13)
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("uso: sv_s26_r06_t0701 FIXTURE_R05 SALIDA_NUEVA");
        std::process::exit(2);
    }
    match campaign(Path::new(&args[1]), Path::new(&args[2])) {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(e) => {
            eprintln!("IMPEDIMENTO: {e}");
            std::process::exit(2);
        }
    }
}
