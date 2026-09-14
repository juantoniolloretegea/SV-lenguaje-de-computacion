mod recepcion;
use recepcion::{Fault, Profile, Session, Stop};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};
use sv_bis_i0205::{
    admit,
    json::{self, obj, s, J},
    ReceivedBytes, TrustedContext, TrustedRegistry,
};

fn read(path: &Path, max: usize) -> Result<Vec<u8>, String> {
    let mut b = Vec::new();
    File::open(path)
        .map_err(|e| e.to_string())?
        .take((max + 1) as u64)
        .read_to_end(&mut b)
        .map_err(|e| e.to_string())?;
    if b.len() > max {
        return Err(format!("cuota: {}", path.display()));
    }
    Ok(b)
}
fn json_file(path: &Path) -> Result<J, String> {
    json::decode(&read(path, 8192)?).map_err(|e| e.to_string())
}
fn write_new(path: &Path, b: &[u8]) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(b).map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())
}
fn save(path: &Path, j: &J) -> Result<(), String> {
    write_new(path, &json::encode(j, 262144).map_err(|e| e.to_string())?)
}
fn admission(
    dir: &Path,
    ctx: &TrustedContext,
    reg: &TrustedRegistry,
) -> Result<sv_bis_i0205::AdmittedDelivery, String> {
    let f = |n: &str| File::open(dir.join(n)).map_err(|e| e.to_string());
    let request = ReceivedBytes::read(
        &mut f("request.json")?,
        &mut f("source.bin")?,
        &mut f("state.bin")?,
        Some(&mut f("support.bin")?),
        &mut f("geometry.bin")?,
    )
    .map_err(|e| format!("{}: {}", e.guard, e.cause))?;
    admit(request, ctx, reg).map_err(|e| format!("{}: {}", e.guard, e.cause))
}
fn same_support(a: &std::fs::Metadata, b: &std::fs::Metadata) -> bool {
    a.dev() == b.dev() && a.ino() == b.ino()
}
fn campaign(fixture: &Path, out: &Path) -> Result<(), String> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let custody = json_file(&fixture.join("context.json"))?;
    let ctx = TrustedContext::from_custody(&read(&fixture.join("context.json"), 8192)?)?;
    let reg = TrustedRegistry::from_custody(
        &read(&fixture.join("registry.json"), 8192)?,
        &read(&fixture.join("constitution.bin"), 8192)?,
        &read(&fixture.join("convention.bin"), 8192)?,
        &read(&fixture.join("transforms.bin"), 8192)?,
    )?;
    let plan = json_file(&fixture.join("plan.json"))?;
    let oracle = json_file(&fixture.join("oracle.json"))?;
    let expected = read(&fixture.join("geometry.bin"), 8192)?;
    let expected_state = read(&fixture.join("state.bin"), 1024)?;
    let mut session = Session::new();
    let mut rows = Vec::new();
    // Oráculos literales previos. La geometría y el recibo esperado proceden de R05.
    let cases = [
        (
            "P01",
            Fault::None,
            Stop::None,
            Profile::ContentRefs,
            "INFORME_LOCAL_ACEPTADO",
            "RETORNO",
            None,
        ),
        (
            "P02",
            Fault::Absent,
            Stop::None,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "RETORNO",
            Some("informe_ausente"),
        ),
        (
            "P03",
            Fault::Truncated,
            Stop::None,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "RETORNO",
            Some("informe_incompleto_o_sobrante"),
        ),
        (
            "P04",
            Fault::WrongAttempt,
            Stop::None,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "RETORNO",
            Some("informe_otro_intento"),
        ),
        (
            "P05",
            Fault::None,
            Stop::BeforeDispatch,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "PANICO_OBSERVADO",
            Some("sin_informe_final"),
        ),
        (
            "P06",
            Fault::None,
            Stop::AfterCapture,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "PANICO_OBSERVADO",
            Some("sin_informe_final"),
        ),
        (
            "P07",
            Fault::None,
            Stop::None,
            Profile::ContentRefs,
            "INFORME_LOCAL_ACEPTADO",
            "RETORNO",
            None,
        ),
        (
            "P08",
            Fault::None,
            Stop::None,
            Profile::ContentRefs,
            "INFORME_LOCAL_ACEPTADO",
            "RETORNO",
            None,
        ),
        (
            "P09",
            Fault::None,
            Stop::None,
            Profile::SupportContinuity,
            "NO_ACREDITADO",
            "NO_EJECUTADO",
            Some("continuidad_soporte_no_ofrecida"),
        ),
        (
            "P10",
            Fault::WrongReceipt,
            Stop::None,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "RETORNO",
            Some("recibo_no_concordante"),
        ),
        (
            "P11",
            Fault::Excess,
            Stop::None,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "RETORNO",
            Some("informe_fuera_cuota"),
        ),
        (
            "P12",
            Fault::None,
            Stop::None,
            Profile::ContentRefs,
            "NO_ACREDITADO",
            "NO_EJECUTADO",
            Some("apertura_no_ligada_al_admitido"),
        ),
    ];
    for (id, fault, stop, profile, status, termination, cause) in cases {
        let dir = out.join(id);
        fs::create_dir(&dir).map_err(|e| e.to_string())?;
        for name in [
            "request.json",
            "source.bin",
            "state.bin",
            "support.bin",
            "geometry.bin",
        ] {
            write_new(&dir.join(name), &read(&fixture.join(name), 8192)?)?;
        }
        let mut opening_custody = custody.clone();
        if id == "P12" {
            let mut b = opening_custody.field("vinculo").clone();
            b.set("revision", s("r-ajena"));
            opening_custody.set("vinculo", b);
        }
        let open = session.open(id, profile, opening_custody)?;
        // La admisión ocurre antes de cualquier sustitución. Un fallo se conserva
        // como error de campaña, nunca se reinterpreta como caso conforme.
        let admitted = admission(&dir, &ctx, &reg)?;
        let geo = dir.join("geometry.bin");
        let before = fs::metadata(&geo).map_err(|e| e.to_string())?;
        // Mantener abierto el original hace inequívoco el test de sustitución inode.
        // Es testigo del soporte del ensayo, no mecanismo de custodia del consumidor.
        let _held = File::open(&geo).map_err(|e| e.to_string())?;
        let replacement = if id == "P07" {
            b"contenido_sustituido".to_vec()
        } else {
            expected.clone()
        };
        let after_admission = || -> Result<(), String> {
            if id == "P07" || id == "P08" {
                let tmp = dir.join("replacement.bin");
                write_new(&tmp, &replacement)?;
                fs::rename(tmp, &geo).map_err(|e| e.to_string())?;
            }
            Ok(())
        };
        let after_capture = || -> Result<(), String> {
            if id == "P06" {
                fs::write(dir.join("state.bin"), b"efecto_local_antes_panico")
                    .map_err(|e| e.to_string())?;
            }
            Ok(())
        };
        let obs = recepcion::run(
            open,
            admitted,
            &reg,
            plan.field("captor").text().ok_or("captor ausente")?.into(),
            plan.field("contexto_captura").clone(),
            stop,
            fault,
            after_admission,
            after_capture,
        );
        let after = fs::metadata(&geo).map_err(|e| e.to_string())?;
        let after_bytes = read(&geo, 8192)?;
        let state_after = read(&dir.join("state.bin"), 1024)?;
        let has_capture = !matches!(id, "P05" | "P09" | "P12");
        let mut checks = vec![
            ("recepcion_literal", obs.status == status),
            ("terminacion_literal", obs.termination == termination),
            ("causa_literal", obs.cause.as_deref() == cause),
            ("presencia_captura", obs.capture.is_some() == has_capture),
            (
                "captura_literal",
                obs.capture.as_ref().is_none_or(|c| {
                    c.bytes == expected && c.context == *oracle.field("contexto_entrega_esperado")
                }),
            ),
            (
                "recibo_literal",
                if status == "INFORME_LOCAL_ACEPTADO" {
                    obs.accepted.as_ref() == Some(oracle.field("recibo_esperado"))
                } else {
                    obs.accepted.is_none()
                },
            ),
            (
                "apertura_conservada",
                obs.opening.field("intento").text() == Some(id)
                    && obs.opening.field("perfil").text() == Some(profile.name()),
            ),
        ];
        if id == "P05" {
            checks.push((
                "barrera_panico_antes",
                obs.barriers.contains(&"inyeccion_panico_antes_despacho") && obs.raw.is_empty(),
            ));
        }
        if id == "P06" {
            checks.push((
                "parcial_sin_rollback",
                obs.barriers.contains(&"inyeccion_panico_tras_captura")
                    && obs.raw.is_empty()
                    && state_after == b"efecto_local_antes_panico",
            ));
        } else {
            checks.push(("estado_releido_literal", state_after == expected_state));
        }
        if id == "P07" || id == "P08" {
            checks.push((
                "sustitucion_real",
                !same_support(&before, &after) && after_bytes == replacement,
            ));
        }
        let pass = checks.iter().all(|(_, v)| *v);
        let document = obj(vec![
            ("caso", s(id)),
            ("observacion", obs.documentary()),
            (
                "soporte_antes",
                obj(vec![
                    ("dev", s(&before.dev().to_string())),
                    ("ino", s(&before.ino().to_string())),
                ]),
            ),
            (
                "soporte_despues",
                obj(vec![
                    ("dev", s(&after.dev().to_string())),
                    ("ino", s(&after.ino().to_string())),
                ]),
            ),
            ("geometria_releida", recepcion::bytes(&after_bytes)),
            ("estado_releido", recepcion::bytes(&state_after)),
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
        ]);
        save(&dir.join("observacion.json"), &document)?;
        rows.push(obj(vec![("caso", s(id)), ("conforme", J::Bool(pass))]));
        println!("{id}: {}", if pass { "CONFORME" } else { "DISCREPANCIA" });
    }
    let duplicate = session
        .open("P01", Profile::ContentRefs, custody.clone())
        .err()
        .as_deref()
        == Some("intento_duplicado");
    let invalid = session
        .open("", Profile::ContentRefs, custody.clone())
        .err()
        .as_deref()
        == Some("intento_invalido");
    for i in 12..32 {
        session.open(
            &format!("RESERVA-{i}"),
            Profile::ContentRefs,
            custody.clone(),
        )?;
    }
    let quota = session
        .open("EXCESO", Profile::ContentRefs, custody)
        .err()
        .as_deref()
        == Some("cuota_intentos");
    let controls = duplicate && invalid && quota;
    save(
        &out.join("P13.json"),
        &obj(vec![
            ("duplicado_rechazado", J::Bool(duplicate)),
            ("vacio_rechazado", J::Bool(invalid)),
            ("cuota_rechazada", J::Bool(quota)),
            ("conforme", J::Bool(controls)),
        ]),
    )?;
    rows.push(obj(vec![
        ("caso", s("P13")),
        ("conforme", J::Bool(controls)),
    ]));
    println!(
        "P13: {}",
        if controls { "CONFORME" } else { "DISCREPANCIA" }
    );
    let all = rows
        .iter()
        .all(|j| j.field("conforme").boolean() == Some(true));
    save(
        &out.join("resultado.json"),
        &obj(vec![
            ("version", s("S26-R06-LOCAL01/1")),
            ("casos", J::Array(rows)),
            ("conforme", J::Bool(all)),
        ]),
    )?;
    if all {
        Ok(())
    } else {
        Err("discrepancia preservada".into())
    }
}
fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!("uso: sv_s26_r06_local01 FIXTURE DIRECTORIO_NUEVO");
        std::process::exit(2);
    }
    if let Err(e) = campaign(Path::new(&args[1]), Path::new(&args[2])) {
        eprintln!("CAMPAÑA_NO_CONFORME: {e}");
        std::process::exit(1);
    }
}
