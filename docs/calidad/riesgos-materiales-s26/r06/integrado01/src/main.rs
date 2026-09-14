//! Observador experimental de un proceso local. No introduce estados soberanos SV.
use std::{
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Read, Write},
    os::{fd::FromRawFd, unix::process::ExitStatusExt},
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc::{sync_channel, RecvTimeoutError},
    time::{Duration, Instant},
};
use sv_bis_i0205::{
    admit, certify,
    json::{self, n, obj, s, J},
    Capture, ReceivedBytes, TrustedContext, TrustedRegistry,
};

type R<T> = Result<T, String>;
const MAGIC: &[u8; 8] = b"R06I01\0\x01";
const FRAME_LIMIT: usize = 8192;
const CHANNEL_LIMIT: usize = 65536;
const FILE_LIMIT: usize = 1048576;
const EFFECT: &[u8] = b"efecto_local_anterior_a_terminacion";
const INITIAL: &[u8] = b"estado_inicial";
fn io<T>(r: std::io::Result<T>) -> R<T> {
    r.map_err(|e| e.to_string())
}
fn need(b: bool, e: &str) -> R<()> {
    if b {
        Ok(())
    } else {
        Err(e.into())
    }
}
fn read(p: &Path, max: usize) -> R<Vec<u8>> {
    let mut b = Vec::new();
    io(io(File::open(p))?
        .take((max + 1) as u64)
        .read_to_end(&mut b))?;
    need(b.len() <= max, "cuota_archivo")?;
    Ok(b)
}
fn document(p: &Path) -> R<J> {
    json::decode(&read(p, 8192)?).map_err(|e| e.to_string())
}
fn write_new(p: &Path, b: &[u8]) -> R<()> {
    need(b.len() <= FILE_LIMIT, "cuota_salida")?;
    let mut f = io(OpenOptions::new().write(true).create_new(true).open(p))?;
    io(f.write_all(b))?;
    io(f.sync_all())
}
fn save(p: &Path, j: &J) -> R<()> {
    write_new(p, &json::encode(j, FILE_LIMIT).map_err(|e| e.to_string())?)
}
fn encoded(j: &J) -> R<Vec<u8>> {
    json::encode(j, FRAME_LIMIT).map_err(|e| e.to_string())
}
fn frame(id: &str, class: &str, payload: J) -> R<Vec<u8>> {
    let body = encoded(&obj(vec![
        ("intento", s(id)),
        ("clase", s(class)),
        ("contenido", payload),
    ]))?;
    let mut b = MAGIC.to_vec();
    b.extend_from_slice(&(body.len() as u32).to_be_bytes());
    b.extend(body);
    Ok(b)
}
fn send(sock: &mut Channel, id: &str, class: &str, payload: J) -> R<()> {
    io(sock.write_all(&frame(id, class, payload)?))
}
fn ack(sock: &mut Channel) -> R<()> {
    let mut b = [0];
    io(sock.read_exact(&mut b))?;
    need(b == [b'G'], "control_invalido")
}
#[allow(dead_code)]
fn material(f: &Path) -> R<(Capture, J)> {
    let ctx = TrustedContext::from_custody(&read(&f.join("context.json"), 8192)?)?;
    let reg = TrustedRegistry::from_custody(
        &read(&f.join("registry.json"), 8192)?,
        &read(&f.join("constitution.bin"), 8192)?,
        &read(&f.join("convention.bin"), 8192)?,
        &read(&f.join("transforms.bin"), 8192)?,
    )?;
    let open = |name: &str| io(File::open(f.join(name)));
    let received = ReceivedBytes::read(
        &mut open("request.json")?,
        &mut open("source.bin")?,
        &mut open("state.bin")?,
        Some(&mut open("support.bin")?),
        &mut open("geometry.bin")?,
    )
    .map_err(|e| format!("{}: {}", e.guard, e.cause))?;
    let a = admit(received, &ctx, &reg).map_err(|e| format!("{}: {}", e.guard, e.cause))?;
    let plan = document(&f.join("plan.json"))?;
    let cap = Capture {
        captor: plan.field("captor").text().ok_or("captor")?.into(),
        context: plan.field("contexto_captura").clone(),
        bytes: a.descriptor().to_vec(),
    };
    let receipt =
        certify(&a, Some(&cap), false, &reg).map_err(|e| format!("{}: {}", e.guard, e.cause))?;
    Ok((cap, receipt))
}
fn capture(sock: &mut Channel, id: &str, cap: &Capture) -> R<()> {
    need(cap.bytes.len() <= 8192, "cuota_captura")?;
    for (index, chunk) in cap.bytes.chunks(3072).enumerate() {
        let text = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
        send(
            sock,
            id,
            "fragmento",
            obj(vec![
                ("desplazamiento", n(index * 3072)),
                ("bytes_utf8", s(text)),
            ]),
        )?;
    }
    send(
        sock,
        id,
        "captura",
        obj(vec![
            ("contexto", cap.context.clone()),
            ("captor", s(&cap.captor)),
        ]),
    )
}
struct Channel {
    input: File,
    output: Option<File>,
}
impl Channel {
    fn close_output(&mut self) -> R<()> {
        self.output.take();
        Ok(())
    }
}
impl Read for Channel {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        self.input.read(b)
    }
}
impl Write for Channel {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.output
            .as_mut()
            .ok_or_else(|| std::io::Error::from(ErrorKind::BrokenPipe))?
            .write(b)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.output
            .as_mut()
            .ok_or_else(|| std::io::Error::from(ErrorKind::BrokenPipe))?
            .flush()
    }
}
#[allow(dead_code)]
fn child(mode: &str, id: &str, dir: &Path, fixture: &Path) -> R<i32> {
    // SAFETY: esta rama se ejecuta en un proceso hijo nuevo, con fd 0/1
    // constituidos por Command/Stdio::piped. Se transfiere su propiedad una
    // sola vez; no se usan stdin()/stdout() ni otros propietarios de esos fd.
    let mut sock = unsafe {
        Channel {
            input: File::from_raw_fd(0),
            output: Some(File::from_raw_fd(1)),
        }
    };

    if mode == "plazo" {
        send(&mut sock, id, "barrera", s("espera_sin_informe"))?;
        ack(&mut sock)?;
        return Ok(0);
    }
    if mode == "eof_vivo" {
        sock.close_output()?;
        ack(&mut sock)?;
        return Ok(0);
    }
    if mode == "exceso" {
        let mut b = MAGIC.to_vec();
        b.extend_from_slice(&((FRAME_LIMIT + 1) as u32).to_be_bytes());
        io(sock.write_all(&b))?;
        ack(&mut sock)?;
        return Ok(0);
    }
    if mode == "magia" {
        io(sock.write_all(b"XXXXXXXX\0\0\0\0"))?;
        ack(&mut sock)?;
        return Ok(0);
    }
    if mode == "abort_antes" {
        send(&mut sock, id, "barrera", s("antes_captura"))?;
        ack(&mut sock)?;
        std::process::abort();
    }
    let (cap, receipt) = material(fixture)?;
    capture(&mut sock, id, &cap)?;
    if mode == "abort_despues" || mode == "muerte_despues" {
        io(fs::write(dir.join("efecto.bin"), EFFECT))?;
        send(&mut sock, id, "barrera", s("tras_escritura"))?;
        ack(&mut sock)?;
        if mode == "abort_despues" {
            std::process::abort();
        }
        return Err("el_proceso_debia_ser_terminado_en_barrera".into());
    }
    if mode == "tardio" {
        send(&mut sock, id, "barrera", s("seleccionar_B"))?;
        ack(&mut sock)?;
        send(&mut sock, id, "informe", receipt.clone())?;
        let other = id.strip_suffix('A').ok_or("id_A")?.to_owned() + "B";
        capture(&mut sock, &other, &cap)?;
        send(&mut sock, &other, "informe", receipt)?;
        return Ok(0);
    }
    if mode == "ajeno" {
        send(&mut sock, "AJENO", "informe", receipt)?;
        return Ok(0);
    }
    if mode == "truncado" {
        let b = frame(id, "informe", receipt)?;
        io(sock.write_all(&b[..9]))?;
        sock.close_output()?;
        ack(&mut sock)?;
        return Ok(0);
    }
    send(&mut sock, id, "informe", receipt.clone())?;
    if mode == "duplicado" {
        send(&mut sock, id, "informe", receipt)?;
    }
    if mode == "conflicto" {
        send(&mut sock, id, "informe", J::Null)?;
    }
    Ok(if mode == "salida_7" { 7 } else { 0 })
}

#[derive(Default)]
struct Slot {
    rejection: Option<J>,
    fragments: Vec<u8>,
    capture: Option<J>,
    first: Option<Vec<u8>>,
    valid: bool,
    conflict: bool,
    duplicates: usize,
    late: usize,
}
struct Ledger {
    ids: [String; 2],
    slots: [Slot; 2],
    active: usize,
    foreign: usize,
    events: Vec<J>,
    expected_capture: J,
    expected_receipt: Vec<u8>,
}
impl Ledger {
    fn new(id: &str, fixture: &Path) -> R<Self> {
        let oracle = document(&fixture.join("oracle.json"))?;
        let plan = document(&fixture.join("plan.json"))?;
        let b = read(&fixture.join("geometry.bin"), 8192)?;
        let geo = std::str::from_utf8(&b).map_err(|e| e.to_string())?;
        Ok(Self {
            ids: [format!("{id}-A"), format!("{id}-B")],
            slots: std::array::from_fn(|_| Slot::default()),
            active: 0,
            foreign: 0,
            events: Vec::new(),
            expected_capture: obj(vec![
                ("bytes_utf8", s(geo)),
                (
                    "contexto",
                    oracle.field("contexto_entrega_esperado").clone(),
                ),
                ("captor", plan.field("captor").clone()),
            ]),
            expected_receipt: encoded(oracle.field("recibo_esperado"))?,
        })
    }
    fn receive(&mut self, body: &[u8]) -> R<Option<String>> {
        need(self.events.len() < 24, "cuota_mensajes")?;
        let j = json::decode(body).map_err(|_| "json_invalido".to_string())?;
        need(j.pairs().map(|p| p.len()) == Some(3), "campos_invalidos")?;
        let id = j.field("intento").text().ok_or("intento_ausente")?;
        let class = j.field("clase").text().ok_or("clase_ausente")?;
        let p = j.field("contenido");
        let mut decision = "registrado";
        let mut barrier = None;
        if let Some(k) = self.ids.iter().position(|x| x == id) {
            let slot = &mut self.slots[k];
            match class {
                "fragmento" => {
                    need(slot.capture.is_none(), "fragmento_tras_captura")?;
                    need(p.pairs().map(|x| x.len()) == Some(2), "campos_fragmento")?;
                    let offset = p.field("desplazamiento").uint().ok_or("desplazamiento")?;
                    let part = p
                        .field("bytes_utf8")
                        .text()
                        .ok_or("bytes_fragmento")?
                        .as_bytes();
                    need(
                        offset == slot.fragments.len() && !part.is_empty() && part.len() <= 3072,
                        "orden_fragmento",
                    )?;
                    need(slot.fragments.len() + part.len() <= 8192, "cuota_captura")?;
                    slot.fragments.extend_from_slice(part);
                }
                "captura" => {
                    need(slot.capture.is_none(), "captura_repetida")?;
                    need(p.pairs().map(|x| x.len()) == Some(2), "campos_captura")?;
                    need(
                        slot.fragments
                            == self
                                .expected_capture
                                .field("bytes_utf8")
                                .text()
                                .ok_or("referencia_captura")?
                                .as_bytes()
                            && encoded(p.field("contexto"))?
                                == encoded(self.expected_capture.field("contexto"))?
                            && p.field("captor") == self.expected_capture.field("captor"),
                        "captura_no_concordante",
                    )?;
                    let text = std::str::from_utf8(&slot.fragments).map_err(|e| e.to_string())?;
                    slot.capture = Some(obj(vec![
                        ("bytes_utf8", s(text)),
                        ("contexto", p.field("contexto").clone()),
                        ("captor", p.field("captor").clone()),
                    ]));
                }
                "informe" => {
                    if slot.rejection.is_some() {
                        slot.conflict = true;
                    }
                    if k != self.active {
                        slot.late += 1;
                        decision = "tardio_separado";
                    }
                    if let Some(first) = &slot.first {
                        if first == body {
                            slot.duplicates += 1;
                            decision = "duplicado_concordante";
                        } else {
                            slot.conflict = true;
                            decision = "conflicto_conservado";
                        }
                    } else {
                        slot.first = Some(body.to_vec());
                        slot.valid = slot.rejection.is_none()
                            && slot.capture.is_some()
                            && encoded(p)? == self.expected_receipt;
                        if !slot.valid {
                            decision = "informe_no_concordante";
                        }
                    }
                }
                "rechazo" => {
                    need(
                        slot.rejection.is_none() && p.pairs().map(|v| v.len()) == Some(3),
                        "rechazo_repetido_o_invalido",
                    )?;
                    need(
                        ["etapa", "guarda", "causa"]
                            .iter()
                            .all(|k| p.field(k).text().is_some()),
                        "campos_rechazo",
                    )?;
                    slot.rejection = Some(p.clone());
                    if slot.first.is_some() {
                        slot.conflict = true;
                    }
                    decision = "rechazo_del_hijo_conservado";
                }
                "barrera" => {
                    barrier = Some(p.text().ok_or("barrera_invalida")?.into());
                }
                _ => return Err("clase_desconocida".into()),
            }
        } else {
            self.foreign += 1;
            decision = "intento_ajeno";
        }
        self.events
            .push(obj(vec![("mensaje", j), ("recepcion", s(decision))]));
        Ok(barrier)
    }
    fn documentary(&self) -> J {
        obj(vec![
            ("activo", s(&self.ids[self.active])),
            ("ajenos", n(self.foreign)),
            ("eventos", J::Array(self.events.clone())),
            (
                "intentos",
                J::Array(
                    self.ids
                        .iter()
                        .zip(&self.slots)
                        .map(|(id, slot)| {
                            obj(vec![
                                ("id", s(id)),
                                ("fragmentos_recibidos", bytes(&slot.fragments)),
                                ("captura", slot.capture.clone().unwrap_or(J::Null)),
                                ("rechazo", slot.rejection.clone().unwrap_or(J::Null)),
                                (
                                    "primer_informe_bytes",
                                    bytes(slot.first.as_deref().unwrap_or(&[])),
                                ),
                                ("informe_valido", J::Bool(slot.valid)),
                                ("conflicto", J::Bool(slot.conflict)),
                                ("duplicados", n(slot.duplicates)),
                                ("tardios", n(slot.late)),
                            ])
                        })
                        .collect(),
                ),
            ),
        ])
    }
}
fn bytes(b: &[u8]) -> J {
    J::Array(b.iter().map(|v| n(*v as usize)).collect())
}
struct Guard(Child);
impl Drop for Guard {
    fn drop(&mut self) {
        if !matches!(self.0.try_wait(), Ok(Some(_))) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}
fn status(s: Option<ExitStatus>) -> J {
    match s {
        None => s_unseen(),
        Some(x) => obj(vec![
            ("observada", J::Bool(true)),
            ("codigo", x.code().map(|v| n(v as usize)).unwrap_or(J::Null)),
            (
                "senal",
                x.signal().map(|v| n(v as usize)).unwrap_or(J::Null),
            ),
        ]),
    }
}
fn s_unseen() -> J {
    obj(vec![
        ("observada", J::Bool(false)),
        ("causa", s("terminacion_no_observada")),
    ])
}

struct Case {
    id: &'static str,
    mode: &'static str,
    channel: &'static str,
    code: Option<i32>,
    signal: Option<i32>,
    capture: bool,
    valid: bool,
    effect: bool,
}
mod integrado;
use integrado::CASES;

fn observe(c: &Case, fixture: &Path, dir: &Path, exe: &Path) -> R<bool> {
    io(fs::create_dir(dir))?;
    let mut ledger = Ledger::new(c.id, fixture)?;
    let mut witness = integrado::Witness::new(c, fixture, dir)?;
    let opening = obj(vec![
        ("emisor", s("observador-R06-INTEGRADO01-W-S26")),
        ("ambito", s("una_instancia_de_campana")),
        (
            "intentos",
            J::Array(ledger.ids.iter().map(|id| s(id)).collect()),
        ),
        ("encargo", document(&fixture.join("context.json"))?),
        ("perfil", s(integrado::profile(c.id).name())),
    ]);
    save(&dir.join("apertura.json"), &opening)?;
    write_new(&dir.join("efecto.bin"), INITIAL)?;
    let mut worker = Guard(io(Command::new(exe)
        .arg("hijo")
        .arg(c.mode)
        .arg(&ledger.ids[0])
        .arg(dir)
        .arg(fixture)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::from(io(OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join("hijo.stderr.log")))?))
        .spawn())?);
    let mut sock = worker.0.stdin.take().ok_or("control_ausente")?;
    let mut output = worker.0.stdout.take().ok_or("salida_ausente")?;
    let (tx, rx) = sync_channel::<R<Vec<u8>>>(1);
    let pump = std::thread::spawn(move || loop {
        let mut part = [0u8; 512];
        match output.read(&mut part) {
            Ok(count) => {
                if tx.send(Ok(part[..count].to_vec())).is_err() || count == 0 {
                    break;
                }
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => {
                let _ = tx.send(Err(e.to_string()));
                break;
            }
        }
    });
    let mut deadline = Instant::now() + Duration::from_millis(1500);
    let mut raw = Vec::new();
    let mut offset = 0usize;
    let mut killed_at_barrier = false;
    let channel = 'receive: loop {
        if Instant::now() >= deadline {
            break "plazo_agotado".to_string();
        }
        match rx.recv_timeout(Duration::from_millis(2)) {
            Ok(Ok(part)) if part.is_empty() => {
                break if raw.len() == offset {
                    "eof"
                } else {
                    "mensaje_truncado"
                }
                .to_string()
            }
            Ok(Ok(part)) => {
                let count = part.len();
                if raw.len() + count > CHANNEL_LIMIT {
                    raw.extend_from_slice(&part[..(CHANNEL_LIMIT + 1 - raw.len()).min(count)]);
                    break "cuota_canal".into();
                }
                raw.extend_from_slice(&part[..count]);
                loop {
                    if raw.len() - offset < 12 {
                        break;
                    }
                    if &raw[offset..offset + 8] != MAGIC {
                        break 'receive "magia_invalida".into();
                    }
                    let len = u32::from_be_bytes(
                        raw[offset + 8..offset + 12]
                            .try_into()
                            .map_err(|_| "longitud")?,
                    ) as usize;
                    if len > FRAME_LIMIT {
                        break 'receive "cuota_mensaje".into();
                    }
                    if raw.len() - offset < 12 + len {
                        break;
                    }
                    let barrier = match ledger.receive(&raw[offset + 12..offset + 12 + len]) {
                        Ok(v) => v,
                        Err(e) => break 'receive e,
                    };
                    offset += 12 + len;
                    if let Some(barrier) = barrier {
                        let sender = ledger
                            .events
                            .last()
                            .ok_or("evento")?
                            .field("mensaje")
                            .field("intento")
                            .text()
                            .ok_or("emisor")?
                            .to_owned();
                        let handled = witness.barrier(c, &barrier, &sender)?;
                        if handled {
                            io(sock.write_all(b"G"))?;
                            continue;
                        }
                        match barrier.as_str() {
                            "espera_sin_informe" if c.mode == "plazo" => {
                                deadline = Instant::now() + Duration::from_millis(150);
                            }
                            "seleccionar_B" => {
                                ledger.active = 1;
                                io(sock.write_all(b"G"))?;
                            }
                            "tras_escritura" if c.mode == "muerte_despues" => {
                                io(worker.0.kill())?;
                                killed_at_barrier = true;
                            }
                            "antes_captura" | "tras_escritura" => io(sock.write_all(b"G"))?,
                            _ => return Err("barrera_desconocida".into()),
                        }
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => return Err("lector_perdido_sin_EOF".into()),
            Ok(Err(e)) => return Err(e),
        }
    };
    let boundary = io(worker.0.try_wait())?;
    write_new(&dir.join("canal.bin"), &raw)?;
    // Instantánea anterior a liberación o terminación administrativa.
    save(
        &dir.join("frontera.json"),
        &obj(vec![
            ("canal", s(&channel)),
            ("terminacion", status(boundary)),
            ("ledger", ledger.documentary()),
            ("efecto_externo", s("NO_ACREDITADO")),
            ("reintento_automatico", J::Bool(false)),
        ]),
    )?;
    let cleanup = match c.mode {
        "eof_vivo" | "truncado" | "exceso" | "magia" => {
            io(sock.write_all(b"G"))?;
            "liberacion_controlada"
        }
        "plazo" => {
            io(worker.0.kill())?;
            "terminacion_administrativa_tras_plazo"
        }
        _ => "ninguna",
    };
    let wait_start = Instant::now();
    let final_status = loop {
        if let Some(s) = io(worker.0.try_wait())? {
            break s;
        }
        need(
            wait_start.elapsed() < Duration::from_secs(2),
            "terminacion_final_no_observada",
        )?;
        std::thread::sleep(Duration::from_millis(2));
    };
    drop(rx);
    pump.join().map_err(|_| "lector_en_panico")?;
    let effect = read(&dir.join("efecto.bin"), 1024)?;
    let a = &ledger.slots[0];
    let selected = &ledger.slots[ledger.active];
    let valid = selected.valid && !selected.conflict && selected.rejection.is_none();
    let mut checks = vec![
        ("canal_literal", channel == c.channel),
        ("codigo_literal", final_status.code() == c.code),
        ("senal_literal", final_status.signal() == c.signal),
        ("captura_A", a.capture.is_some() == c.capture),
        ("informe_seleccionado", valid == c.valid),
        (
            "efecto_local_releido",
            effect == if c.effect { EFFECT } else { INITIAL },
        ),
        (
            "apertura_conservada",
            document(&dir.join("apertura.json"))? == opening,
        ),
    ];
    if matches!(c.mode, "abort_antes" | "abort_despues" | "muerte_despues") {
        checks.push((
            "barrera_y_ausencia_informe",
            ledger
                .events
                .iter()
                .any(|j| j.field("mensaje").field("clase").text() == Some("barrera"))
                && a.first.is_none(),
        ));
    }
    if c.mode == "muerte_despues" {
        checks.push(("terminacion_en_barrera", killed_at_barrier));
    }
    if c.mode == "duplicado" {
        checks.push((
            "duplicado_sin_sobrescritura",
            a.duplicates == 1 && !a.conflict && a.first.is_some(),
        ));
    }
    if c.mode == "conflicto" {
        checks.push((
            "conflicto_y_primero_conservados",
            a.conflict
                && a.valid
                && a.duplicates == 0
                && a.first.as_ref() == Some(&encoded_report_body(&ledger.ids[0], fixture)?),
        ));
    }
    if c.mode == "tardio" {
        checks.push((
            "A_tardio_B_propio",
            ledger.active == 1
                && a.late == 1
                && a.valid
                && ledger.slots[1].valid
                && ledger.slots[1].late == 0,
        ));
    }
    if c.mode == "ajeno" {
        checks.push((
            "ajeno_no_atribuido",
            ledger.foreign == 1 && a.first.is_none(),
        ));
    }
    if c.mode == "plazo" {
        checks.push((
            "espera_del_hijo_observada",
            ledger.events.iter().any(|j| {
                j.field("mensaje").field("contenido").text() == Some("espera_sin_informe")
            }),
        ));
    }
    if matches!(
        c.mode,
        "eof_vivo" | "plazo" | "exceso" | "truncado" | "magia"
    ) {
        checks.push(("terminacion_no_observada_en_frontera", boundary.is_none()));
    }
    checks.extend(witness.checks(c, dir, &ledger)?);
    let pass = checks.iter().all(|(_, v)| *v);
    save(
        &dir.join("observacion.json"),
        &obj(vec![
            ("caso", s(c.id)),
            ("modo", s(c.mode)),
            ("apertura", opening),
            ("canal", s(&channel)),
            ("bytes_canal", n(raw.len())),
            ("prefijo_procesado", n(offset)),
            ("terminacion_en_frontera", status(boundary)),
            ("terminacion_final", status(Some(final_status))),
            ("limpieza", s(cleanup)),
            ("ledger", ledger.documentary()),
            ("efecto_local_releido", bytes(&effect)),
            ("efecto_externo", s("NO_ACREDITADO")),
            ("accion_dependiente_habilitada", J::Bool(false)),
            ("reintento_automatico", J::Bool(false)),
            (
                "comprobaciones",
                J::Array(
                    checks
                        .iter()
                        .map(|(k, v)| obj(vec![("nombre", s(k)), ("conforme", J::Bool(*v))]))
                        .collect(),
                ),
            ),
            ("conforme", J::Bool(pass)),
        ]),
    )?;
    println!(
        "{}: {}",
        c.id,
        if pass { "CONFORME" } else { "DISCREPANCIA" }
    );
    Ok(pass)
}
fn encoded_report_body(id: &str, fixture: &Path) -> R<Vec<u8>> {
    encoded(&obj(vec![
        ("intento", s(id)),
        ("clase", s("informe")),
        (
            "contenido",
            document(&fixture.join("oracle.json"))?
                .field("recibo_esperado")
                .clone(),
        ),
    ]))
}
fn campaign(fixture: &Path, out: &Path) -> R<()> {
    io(fs::create_dir(out))?;
    let exe = io(std::env::current_exe())?;
    let mut rows = Vec::new();
    let mut all = true;
    for c in &CASES {
        let pass = observe(c, fixture, &out.join(c.id), &exe)?;
        all &= pass;
        rows.push(obj(vec![("caso", s(c.id)), ("conforme", J::Bool(pass))]));
    }
    save(
        &out.join("resultado.json"),
        &obj(vec![
            ("version", s("S26-R06-INTEGRADO01/1")),
            ("casos", J::Array(rows)),
            ("conforme", J::Bool(all)),
        ]),
    )?;
    need(all, "discrepancia_conservada")
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let r = if a.len() == 6 && a[1] == "hijo" {
        integrado::child(&a[2], &a[3], Path::new(&a[4]), Path::new(&a[5]))
    } else if a.len() == 3 {
        campaign(&PathBuf::from(&a[1]), &PathBuf::from(&a[2])).map(|_| 0)
    } else {
        Err("uso: sv_s26_r06_integrado01 FIXTURE DIRECTORIO_NUEVO".into())
    };
    match r {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("ENSAYO_NO_CONFORME: {e}");
            std::process::exit(1);
        }
    }
}
