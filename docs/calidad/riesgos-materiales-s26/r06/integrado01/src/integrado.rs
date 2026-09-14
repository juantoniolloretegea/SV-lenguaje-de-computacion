//! Composición experimental del observador PROCESO03 con el consumidor LOCAL01.
use super::*;
use std::io::{Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
#[allow(dead_code)]
#[path = "../../local01/src/recepcion.rs"]
mod local;

pub(super) fn profile(id: &str) -> local::Profile {
    if id == "I11" {
        local::Profile::SupportContinuity
    } else {
        local::Profile::ContentRefs
    }
}
const fn case(
    id: &'static str,
    mode: &'static str,
    channel: &'static str,
    code: Option<i32>,
    signal: Option<i32>,
    capture: bool,
    valid: bool,
    effect: bool,
) -> Case {
    Case {
        id,
        mode,
        channel,
        code,
        signal,
        capture,
        valid,
        effect,
    }
}
pub(super) const CASES: [Case; 17] = [
    case("I01", "normal", "eof", Some(0), None, true, true, false),
    case("I02", "normal", "eof", Some(0), None, true, true, false),
    case(
        "I03",
        "abort_despues",
        "eof",
        None,
        Some(6),
        true,
        false,
        true,
    ),
    case(
        "I04",
        "muerte_despues",
        "eof",
        None,
        Some(9),
        true,
        false,
        true,
    ),
    case(
        "I05",
        "abort_antes",
        "eof",
        None,
        Some(6),
        false,
        false,
        false,
    ),
    case(
        "I06",
        "truncado",
        "mensaje_truncado",
        Some(0),
        None,
        true,
        false,
        false,
    ),
    case("I07", "rechazo", "eof", Some(0), None, false, false, false),
    case("I08", "conflicto", "eof", Some(0), None, true, false, false),
    case(
        "I09",
        "eof_vivo",
        "captura_no_concordante",
        Some(0),
        None,
        false,
        false,
        false,
    ),
    case("I10", "tardio", "eof", Some(0), None, true, true, false),
    case("I11", "rechazo", "eof", Some(0), None, false, false, false),
    case(
        "I12",
        "panico_local",
        "eof",
        Some(0),
        None,
        true,
        false,
        true,
    ),
    case(
        "I13",
        "plazo",
        "plazo_agotado",
        None,
        Some(9),
        true,
        false,
        false,
    ),
    case("I14", "ajeno", "eof", Some(0), None, true, false, false),
    case("I15", "duplicado", "eof", Some(0), None, true, true, false),
    case(
        "I16",
        "rechazo_con_informe",
        "eof",
        Some(0),
        None,
        true,
        false,
        false,
    ),
    case(
        "I17",
        "rechazo_antes_informe",
        "eof",
        Some(0),
        None,
        true,
        false,
        false,
    ),
];
fn object(f: &File) -> R<(u64, u64)> {
    let m = io(f.metadata())?;
    Ok((m.dev(), m.ino()))
}
fn change(path: &Path, b: &[u8]) -> R<()> {
    need(b.len() <= 8192, "cuota_mutacion")?;
    let mut f = io(OpenOptions::new().write(true).open(path))?;
    io(f.seek(SeekFrom::Start(0)))?;
    io(f.write_all(b))?;
    io(f.set_len(b.len() as u64))
}
fn fd_bytes(f: &mut File) -> R<Vec<u8>> {
    io(f.seek(SeekFrom::Start(0)))?;
    let mut b = vec![];
    io(f.take(8193).read_to_end(&mut b))?;
    need(b.len() <= 8192, "cuota_testigo")?;
    Ok(b)
}
fn named(id: (u64, u64)) -> J {
    obj(vec![
        ("dev", s(&id.0.to_string())),
        ("inode", s(&id.1.to_string())),
    ])
}
pub(super) struct Witness {
    fixture: PathBuf,
    dir: PathBuf,
    original: Vec<u8>,
    state_a: Vec<u8>,
    state_b: Vec<u8>,
    held: File,
    geo_id: (u64, u64),
    state_id: (u64, u64),
    samples: Vec<J>,
    barriers: Vec<J>,
}
impl Witness {
    pub(super) fn new(c: &Case, fixture: &Path, dir: &Path) -> R<Self> {
        io(fs::create_dir(dir.join("entrada")))?;
        for name in [
            "request.json",
            "source.bin",
            "state.bin",
            "support.bin",
            "geometry.bin",
        ] {
            write_new(
                &dir.join("entrada").join(name),
                &read(&fixture.join(name), 8192)?,
            )?;
        }
        let held = io(File::open(dir.join("entrada/geometry.bin")))?;
        let geo_id = object(&held)?;
        let state_id = object(&io(File::open(dir.join("entrada/state.bin")))?)?;
        let mut w = Self {
            fixture: fixture.into(),
            dir: dir.into(),
            original: read(&fixture.join("geometry.bin"), 8192)?,
            state_a: read(&fixture.join("state.bin"), 1024)?,
            state_b: read(&fixture.join("vector-alterado.bin"), 1024)?,
            held,
            geo_id,
            state_id,
            samples: vec![],
            barriers: vec![],
        };
        need(
            w.original.len() == 6237 && w.original.last() == Some(&b'\n') && w.state_a != w.state_b,
            "fixture_previo",
        )?;
        w.sample("inicio")?;
        save(
            &dir.join("plan_integrado.json"),
            &obj(vec![
                ("caso", s(c.id)),
                ("modo", s(c.mode)),
                ("perfil", s(profile(c.id).name())),
                ("unidad", s("W-S26")),
                ("segundo_intento_programado", J::Bool(c.id == "I10")),
            ]),
        )?;
        Ok(w)
    }
    fn sample(&mut self, label: &str) -> R<()> {
        need(self.samples.len() < 12, "cuota_muestras")?;
        let mut route = io(File::open(self.dir.join("entrada/geometry.bin")))?;
        let mut state = io(File::open(self.dir.join("entrada/state.bin")))?;
        let a = fd_bytes(&mut self.held)?;
        let b = fd_bytes(&mut route)?;
        let st = fd_bytes(&mut state)?;
        for (suffix, bytes) in [
            ("objeto", a.as_slice()),
            ("ruta", b.as_slice()),
            ("estado", st.as_slice()),
        ] {
            write_new(&self.dir.join(format!("{label}-{suffix}.bin")), bytes)?;
        }
        let item = obj(vec![
            ("muestra", s(label)),
            ("objeto", named(object(&self.held)?)),
            ("ruta", named(object(&route)?)),
            ("estado", named(object(&state)?)),
            ("objeto_sha256", s(&sv_bis_i0205::sha::hash(&a))),
            ("ruta_sha256", s(&sv_bis_i0205::sha::hash(&b))),
            ("estado_sha256", s(&sv_bis_i0205::sha::hash(&st))),
        ]);
        save(&self.dir.join(format!("{label}.json")), &item)?;
        self.samples.push(item);
        Ok(())
    }
    pub(super) fn barrier(&mut self, c: &Case, label: &str, sender: &str) -> R<bool> {
        need(self.barriers.len() < 24, "cuota_barreras")?;
        self.barriers
            .push(obj(vec![("barrera", s(label)), ("intento", s(sender))]));
        let first = sender == format!("{}-A", c.id);
        let path = self.dir.join("entrada/geometry.bin");
        if label == "antes_recepcion" && first && c.id == "I05" {
            change(&self.dir.join("entrada/state.bin"), &self.state_b)?;
            self.sample("mutacion")?;
        }
        if label == "prefijo_leido" && first {
            let mut b = self.original.clone();
            match c.id {
                "I07" => {
                    let last = b.len() - 1;
                    b[last] = b'!';
                }
                "I08" => b[0] = b'!',
                _ => return Err("prefijo_no_previsto".into()),
            }
            change(&path, &b)?;
            self.sample("mutacion")?;
        }
        if label == "piezas_recibidas" && first {
            if matches!(c.id, "I07" | "I08") {
                change(&path, &self.original)?;
                self.sample("restitucion")?;
            }
            if c.id == "I06" {
                change(&self.dir.join("entrada/state.bin"), &self.state_b)?;
                self.sample("mutacion")?;
            }
        }
        if label == "admision" && first {
            if matches!(c.id, "I02" | "I10" | "I11" | "I14" | "I15") {
                let mut b = self.original.clone();
                if !matches!(c.id, "I11" | "I15") {
                    b[0] = b'!';
                }
                write_new(&self.dir.join("sustituto.bin"), &b)?;
                io(fs::rename(self.dir.join("sustituto.bin"), &path))?;
                self.sample("mutacion")?;
            }
            if matches!(c.id, "I03" | "I04" | "I09" | "I12" | "I13" | "I16" | "I17") {
                let mut b = self.original.clone();
                b[0] = b'!';
                change(&path, &b)?;
                self.sample("mutacion")?;
                if c.id == "I04" {
                    change(&path, &self.original)?;
                    self.sample("restitucion")?;
                }
            }
        }
        if label == "seleccionar_B" && c.id == "I10" {
            change(&path, &self.original)?;
            self.sample("entrada_segundo_intento")?;
        }
        Ok(matches!(
            label,
            "antes_recepcion" | "prefijo_leido" | "piezas_recibidas" | "admision"
        ))
    }
    pub(super) fn checks(
        &mut self,
        c: &Case,
        dir: &Path,
        ledger: &Ledger,
    ) -> R<Vec<(&'static str, bool)>> {
        self.sample("final")?;
        save(
            &dir.join("testigo_parental.json"),
            &obj(vec![
                ("muestras", J::Array(self.samples.clone())),
                ("barreras", J::Array(self.barriers.clone())),
            ]),
        )?;
        let renamed = matches!(c.id, "I02" | "I10" | "I11" | "I14" | "I15");
        let mut expected = self.original.clone();
        if matches!(
            c.id,
            "I02" | "I03" | "I09" | "I12" | "I13" | "I14" | "I16" | "I17"
        ) {
            expected[0] = b'!';
        }
        let final_route = read(&dir.join("final-ruta.bin"), 8192)?;
        let state = read(&dir.join("final-estado.bin"), 1024)?;
        let route_id = object(&io(File::open(dir.join("entrada/geometry.bin")))?)?;
        let slot = &ledger.slots[0];
        let reject = match c.id {
            "I05" => Some("I03"),
            "I16" | "I17" => Some("I03"),
            "I07" => Some("M01"),
            "I11" => Some("continuidad_soporte_no_ofrecida"),
            "I12" => Some("PANICO_OBSERVADO"),
            _ => None,
        };
        let mut checks = vec![
            (
                "objeto_testigo_conservado",
                object(&self.held)? == self.geo_id,
            ),
            (
                "sustitucion_objeto_prevista",
                (route_id != self.geo_id) == renamed,
            ),
            ("bytes_finales_previstos", final_route == expected),
            (
                "dependencia_final_observada",
                state
                    == if matches!(c.id, "I05" | "I06") {
                        &self.state_b
                    } else {
                        &self.state_a
                    }
                    .as_slice(),
            ),
            (
                "rechazo_ligado_literal",
                slot.rejection
                    .as_ref()
                    .and_then(|j| j.field("guarda").text())
                    == reject,
            ),
            (
                "identidad_objetos_muestreados",
                self.samples.iter().all(|j| {
                    j.field("objeto") == &named(self.geo_id)
                        && j.field("estado") == &named(self.state_id)
                }),
            ),
        ];
        if c.id != "I01" {
            let mutation = read(&dir.join("mutacion-ruta.bin"), 8192)?;
            checks.push((
                "muestra_material_de_mutacion",
                if matches!(c.id, "I05" | "I06") {
                    read(&dir.join("mutacion-estado.bin"), 1024)? == self.state_b
                } else if matches!(c.id, "I11" | "I15") {
                    mutation == self.original
                        && self
                            .samples
                            .iter()
                            .find(|j| j.field("muestra").text() == Some("mutacion"))
                            .is_some_and(|j| j.field("ruta") != &named(self.geo_id))
                } else {
                    mutation != self.original
                },
            ));
        }
        if matches!(c.id, "I04" | "I07" | "I08") {
            checks.push((
                "restitucion_no_borra_mutacion",
                read(&dir.join("restitucion-ruta.bin"), 8192)? == self.original
                    && read(&dir.join("mutacion-ruta.bin"), 8192)? != self.original,
            ));
        }
        let id_a = format!("{}-A", c.id);
        let mut expected_received = self.original.clone();
        if c.id == "I07" {
            let end = expected_received.len() - 1;
            expected_received[end] = b'!';
        }
        checks.push((
            "bytes_recibidos_por_hijo",
            read(&dir.join(format!("recibido-{id_a}.bin")), 8192)? == expected_received,
        ));
        if matches!(c.id, "I05" | "I07" | "I11") {
            checks.push((
                "sin_consumo_tras_rechazo",
                !dir.join(format!("consumido-{id_a}.bin")).exists()
                    && slot.capture.is_none()
                    && slot.first.is_none(),
            ));
        } else {
            checks.push((
                "consumo_literal_admitido",
                read(&dir.join(format!("consumido-{id_a}.bin")), 8192)? == self.original
                    && read(&dir.join(format!("admitido-{id_a}.bin")), 8192)? == self.original,
            ));
        }
        if c.id == "I09" {
            let local = document(&dir.join(format!("estado-local-{id_a}.json")))?;
            checks.push((
                "revision_ajena_con_bytes_iguales",
                local.field("causa").text() == Some("D04: capture context mismatch")
                    && slot.fragments == self.original
                    && slot.capture.is_none()
                    && slot.first.is_none(),
            ));
        }
        if c.id == "I10" {
            let id_b = format!("{}-B", c.id);
            checks.push((
                "B_con_recepcion_y_consumo_propios",
                read(&dir.join(format!("recibido-{id_b}.bin")), 8192)? == self.original
                    && read(&dir.join(format!("admitido-{id_b}.bin")), 8192)? == self.original
                    && read(&dir.join(format!("consumido-{id_b}.bin")), 8192)? == self.original,
            ));
        }
        if c.id == "I12" {
            let local = document(&dir.join(format!("estado-local-{id_a}.json")))?;
            checks.push((
                "panico_local_y_proceso_separados",
                local.field("terminacion").text() == Some("PANICO_OBSERVADO")
                    && local.field("causa").text() == Some("sin_informe_final")
                    && slot.capture.is_some()
                    && slot.first.is_none(),
            ));
        }
        if matches!(c.id, "I16" | "I17") {
            let pos = |class| {
                ledger
                    .events
                    .iter()
                    .position(|j| j.field("mensaje").field("clase").text() == Some(class))
            };
            let order = pos("informe").zip(pos("rechazo")).is_some_and(|(a, b)| {
                if c.id == "I16" {
                    a < b
                } else {
                    b < a
                }
            });
            checks.push((
                "rechazo_y_recibo_incompatibles_conservados",
                slot.conflict
                    && slot.first.as_ref() == Some(&encoded_report_body(&id_a, &self.fixture)?)
                    && slot.valid == (c.id == "I16")
                    && order,
            ));
        }
        Ok(checks)
    }
}
fn barrier(sock: &mut Channel, id: &str, name: &str) -> R<()> {
    send(sock, id, "barrera", s(name))?;
    ack(sock)
}
struct HookReader<'a> {
    input: File,
    sock: &'a mut Channel,
    id: &'a str,
    split: bool,
    raw: Vec<u8>,
    paused: bool,
}
impl Read for HookReader<'_> {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        if b.is_empty() {
            return Ok(0);
        }
        if self.split && self.raw.len() == 3072 && !self.paused {
            barrier(self.sock, self.id, "prefijo_leido").map_err(std::io::Error::other)?;
            self.paused = true;
        }
        let remaining = 8193usize
            .checked_sub(self.raw.len())
            .ok_or_else(|| std::io::Error::other("cuota_lector"))?;
        if remaining == 0 {
            return Err(std::io::Error::other("cuota_lector"));
        }
        let len = if self.split && self.raw.len() < 3072 {
            b.len().min(3072 - self.raw.len()).min(remaining)
        } else {
            b.len().min(remaining)
        };
        let count = self.input.read(&mut b[..len])?;
        self.raw.extend_from_slice(&b[..count]);
        Ok(count)
    }
}
fn rejection(sock: &mut Channel, id: &str, stage: &str, guard: &str, cause: &str) -> R<()> {
    send(
        sock,
        id,
        "rechazo",
        obj(vec![
            ("etapa", s(stage)),
            ("guarda", s(guard)),
            ("causa", s(cause)),
        ]),
    )
}
fn run_material(
    sock: &mut Channel,
    case_id: &str,
    id: &str,
    dir: &Path,
    fixture: &Path,
) -> R<Option<local::Observation>> {
    let opening = document(&dir.join("apertura.json"))?;
    need(
        opening
            .field("intentos")
            .array()
            .is_some_and(|v| v.contains(&s(id))),
        "intento_no_abierto",
    )?;
    let custody = opening.field("encargo").clone();
    need(
        custody == document(&fixture.join("context.json"))?,
        "custodia_apertura_no_concordante",
    )?;
    let profile = match opening.field("perfil").text() {
        Some("contenido_referentes") => local::Profile::ContentRefs,
        Some("continuidad_soporte") => local::Profile::SupportContinuity,
        _ => return Err("perfil_apertura_invalido".into()),
    };
    let ctx = TrustedContext::from_custody(&encoded(&custody)?)?;
    let reg = TrustedRegistry::from_custody(
        &read(&fixture.join("registry.json"), 8192)?,
        &read(&fixture.join("constitution.bin"), 8192)?,
        &read(&fixture.join("convention.bin"), 8192)?,
        &read(&fixture.join("transforms.bin"), 8192)?,
    )?;
    let mut session = local::Session::new();
    let opening = session.open(id, profile, custody)?;
    barrier(sock, id, "antes_recepcion")?;
    let input = |name: &str| io(File::open(dir.join("entrada").join(name)));
    let mut reader = HookReader {
        input: input("geometry.bin")?,
        sock,
        id,
        split: matches!(case_id, "I07" | "I08"),
        raw: vec![],
        paused: false,
    };
    let received = ReceivedBytes::read(
        &mut input("request.json")?,
        &mut input("source.bin")?,
        &mut input("state.bin")?,
        Some(&mut input("support.bin")?),
        &mut reader,
    )
    .map_err(|e| format!("{}: {}", e.guard, e.cause))?;
    let raw = std::mem::take(&mut reader.raw);
    drop(reader);
    write_new(&dir.join(format!("recibido-{id}.bin")), &raw)?;
    barrier(sock, id, "piezas_recibidas")?;
    let admitted = match admit(received, &ctx, &reg) {
        Ok(a) => a,
        Err(e) => {
            rejection(sock, id, "admision", e.guard, &e.cause)?;
            return Ok(None);
        }
    };
    write_new(
        &dir.join(format!("admitido-{id}.bin")),
        admitted.descriptor(),
    )?;
    barrier(sock, id, "admision")?;
    let plan = document(&fixture.join("plan.json"))?;
    let mut context = plan.field("contexto_captura").clone();
    if case_id == "I09" {
        context.set("revision", s("r2"));
    }
    let stop = if case_id == "I12" {
        local::Stop::AfterCapture
    } else {
        local::Stop::None
    };
    let obs = local::run(
        opening,
        admitted,
        &reg,
        plan.field("captor").text().ok_or("captor")?.into(),
        context,
        stop,
        local::Fault::None,
        || Ok(()),
        || {
            if case_id == "I12" {
                io(fs::write(dir.join("efecto.bin"), EFFECT))?;
            }
            Ok(())
        },
    );
    if let Some(cap) = &obs.capture {
        write_new(&dir.join(format!("consumido-{id}.bin")), &cap.bytes)?;
    }
    save(
        &dir.join(format!("estado-local-{id}.json")),
        &obj(vec![
            ("terminacion", s(obs.termination)),
            ("recepcion", s(obs.status)),
            ("causa", obs.cause.as_ref().map(|c| s(c)).unwrap_or(J::Null)),
            ("informe_aceptado_local", J::Bool(obs.accepted.is_some())),
        ]),
    )?;
    Ok(Some(obs))
}
pub(super) fn child(mode: &str, id: &str, dir: &Path, fixture: &Path) -> R<i32> {
    // SAFETY: el proceso nuevo recibe fd 0/1 de Stdio::piped. Propiedad transferida
    // una sola vez; no se crean otros propietarios ni se usan stdin()/stdout().
    let mut sock = unsafe {
        Channel {
            input: File::from_raw_fd(0),
            output: Some(File::from_raw_fd(1)),
        }
    };
    let case_id = id.strip_suffix("-A").ok_or("intento_inicial")?;
    let c = CASES.iter().find(|c| c.id == case_id).ok_or("caso")?;
    need(c.mode == mode, "modo_no_precomprometido")?;
    let result = run_material(&mut sock, case_id, id, dir, fixture)?;
    let Some(obs) = result else {
        if mode == "abort_antes" {
            barrier(&mut sock, id, "antes_captura")?;
            std::process::abort();
        }
        return Ok(0);
    };
    if let Some(cap) = &obs.capture {
        capture(&mut sock, id, cap)?;
    }
    if mode == "eof_vivo" {
        sock.close_output()?;
        ack(&mut sock)?;
        return Ok(0);
    }
    if obs.termination == "PANICO_OBSERVADO" {
        rejection(
            &mut sock,
            id,
            "recorrido",
            "PANICO_OBSERVADO",
            obs.cause.as_deref().unwrap_or("sin_causa"),
        )?;
        return Ok(0);
    }
    let Some(receipt) = obs.accepted else {
        rejection(
            &mut sock,
            id,
            "recepcion",
            obs.cause.as_deref().unwrap_or("NO_ACREDITADO"),
            obs.status,
        )?;
        return Ok(0);
    };
    if matches!(mode, "abort_despues" | "muerte_despues") {
        io(fs::write(dir.join("efecto.bin"), EFFECT))?;
        barrier(&mut sock, id, "tras_escritura")?;
        if mode == "abort_despues" {
            std::process::abort();
        }
        return Err("debia_terminarse_en_barrera".into());
    }
    if mode == "plazo" {
        barrier(&mut sock, id, "espera_sin_informe")?;
        return Err("plazo_debia_terminarse".into());
    }
    if mode == "truncado" {
        let b = frame(id, "informe", receipt)?;
        io(sock.write_all(&b[..9]))?;
        sock.close_output()?;
        ack(&mut sock)?;
        return Ok(0);
    }
    if mode == "tardio" {
        barrier(&mut sock, id, "seleccionar_B")?;
        send(&mut sock, id, "informe", receipt)?;
        let id_b = format!("{case_id}-B");
        let second =
            run_material(&mut sock, case_id, &id_b, dir, fixture)?.ok_or("segundo_rechazado")?;
        capture(
            &mut sock,
            &id_b,
            second.capture.as_ref().ok_or("segunda_captura")?,
        )?;
        send(
            &mut sock,
            &id_b,
            "informe",
            second.accepted.ok_or("segundo_informe")?,
        )?;
        return Ok(0);
    }
    if mode == "ajeno" {
        send(&mut sock, "AJENO", "informe", receipt)?;
        return Ok(0);
    }
    if mode == "rechazo_antes_informe" {
        rejection(
            &mut sock,
            id,
            "declaracion_adversarial",
            "I03",
            "rechazo_contradictorio_inyectado",
        )?;
    }
    send(&mut sock, id, "informe", receipt.clone())?;
    if mode == "rechazo_con_informe" {
        rejection(
            &mut sock,
            id,
            "declaracion_adversarial",
            "I03",
            "rechazo_contradictorio_inyectado",
        )?;
    }
    if mode == "duplicado" {
        send(&mut sock, id, "informe", receipt)?;
    }
    if mode == "conflicto" {
        send(&mut sock, id, "informe", J::Null)?;
    }
    Ok(0)
}
