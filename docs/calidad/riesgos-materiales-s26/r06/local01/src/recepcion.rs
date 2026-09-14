//! Recepción experimental local. Ninguna etiqueta introduce estados SV.
use std::{
    collections::BTreeSet,
    panic::{catch_unwind, AssertUnwindSafe},
};
use sv_bis_i0205::{
    certify,
    json::{self, obj, s, J},
    AdmittedDelivery, Capture, TrustedRegistry,
};

pub const REPORT_LIMIT: usize = 4096;
const MAGIC: &[u8; 8] = b"R06L01\0\x01";
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Profile {
    ContentRefs,
    SupportContinuity,
}
impl Profile {
    pub fn name(self) -> &'static str {
        match self {
            Self::ContentRefs => "contenido_referentes",
            Self::SupportContinuity => "continuidad_soporte",
        }
    }
}
#[derive(Clone, Copy)]
pub enum Fault {
    None,
    Absent,
    Truncated,
    WrongAttempt,
    WrongReceipt,
    Excess,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Stop {
    None,
    BeforeDispatch,
    AfterCapture,
}

pub struct Session {
    issued: BTreeSet<String>,
}
impl Session {
    pub fn new() -> Self {
        Self {
            issued: BTreeSet::new(),
        }
    }
    pub fn open(&mut self, id: &str, profile: Profile, custody: J) -> Result<Opening, String> {
        if id.is_empty()
            || id.len() > 64
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err("intento_invalido".into());
        }
        if self.issued.contains(id) {
            return Err("intento_duplicado".into());
        }
        if self.issued.len() >= 32 {
            return Err("cuota_intentos".into());
        }
        self.issued.insert(id.into());
        Ok(Opening {
            id: id.into(),
            profile,
            custody,
        })
    }
}
pub struct Opening {
    id: String,
    profile: Profile,
    custody: J,
}
pub struct Observation {
    pub opening: J,
    pub termination: &'static str,
    pub status: &'static str,
    pub barriers: Vec<&'static str>,
    pub capture: Option<Capture>,
    pub raw: Vec<u8>,
    pub accepted: Option<J>,
    pub cause: Option<String>,
}
impl Observation {
    pub fn documentary(&self) -> J {
        obj(vec![
            ("apertura", self.opening.clone()),
            ("terminacion", s(self.termination)),
            ("recepcion", s(self.status)),
            (
                "causa",
                self.cause.as_ref().map(|v| s(v)).unwrap_or(J::Null),
            ),
            (
                "barreras",
                J::Array(self.barriers.iter().map(|v| s(v)).collect()),
            ),
            (
                "captura",
                self.capture
                    .as_ref()
                    .map(|c| {
                        obj(vec![
                            ("captor", s(&c.captor)),
                            ("contexto", c.context.clone()),
                            ("bytes", bytes(&c.bytes)),
                        ])
                    })
                    .unwrap_or(J::Null),
            ),
            ("informe_recibido_bytes", bytes(&self.raw)),
            ("recibo_aceptado", self.accepted.clone().unwrap_or(J::Null)),
            ("efecto_externo", s("NO_ACREDITADO")),
            ("reintento_automatico", J::Bool(false)),
        ])
    }
}
pub fn bytes(b: &[u8]) -> J {
    J::Array(b.iter().map(|v| json::n(*v as usize)).collect())
}
fn envelope(id: &str, receipt: J) -> Result<Vec<u8>, String> {
    let body = json::encode(
        &obj(vec![("intento", s(id)), ("recibo", receipt)]),
        REPORT_LIMIT - 12,
    )
    .map_err(|e| format!("{e:?}"))?;
    let mut raw = MAGIC.to_vec();
    raw.extend_from_slice(&(body.len() as u32).to_be_bytes());
    raw.extend(body);
    Ok(raw)
}
fn receive(raw: &[u8], id: &str, expected: &J) -> Result<J, &'static str> {
    if raw.is_empty() {
        return Err("informe_ausente");
    }
    if raw.len() > REPORT_LIMIT {
        return Err("informe_fuera_cuota");
    }
    if raw.len() < 12 || &raw[..8] != MAGIC {
        return Err("envoltura_invalida");
    }
    let len = u32::from_be_bytes([raw[8], raw[9], raw[10], raw[11]]) as usize;
    if len != raw.len() - 12 {
        return Err("informe_incompleto_o_sobrante");
    }
    let j = json::decode(&raw[12..]).map_err(|_| "informe_json_invalido")?;
    if j.pairs().map(|p| p.len()) != Some(2) {
        return Err("campos_informe_invalidos");
    }
    if j.field("intento").text() != Some(id) {
        return Err("informe_otro_intento");
    }
    if j.field("recibo") != expected {
        return Err("recibo_no_concordante");
    }
    Ok(j.field("recibo").clone())
}

/// El consumidor local recibe un préstamo del descriptor admitido; no reabre rutas.
/// La observación sobrevive únicamente a unwind dentro de este proceso.
pub fn run(
    open: Opening,
    admitted: AdmittedDelivery,
    registry: &TrustedRegistry,
    captor: String,
    capture_context: J,
    stop: Stop,
    fault: Fault,
    after_admission: impl FnOnce() -> Result<(), String>,
    after_capture: impl FnOnce() -> Result<(), String>,
) -> Observation {
    let mut obs = Observation {
        opening: obj(vec![
            ("emisor", s("arnes-R06-local01")),
            ("ambito", s("campana-local-una-instancia")),
            ("intento", s(&open.id)),
            ("perfil", s(open.profile.name())),
            ("encargo_autorizado", open.custody.clone()),
        ]),
        termination: "NO_EJECUTADO",
        status: "NO_ACREDITADO",
        barriers: vec!["apertura_conservada", "admision_observada"],
        capture: None,
        raw: vec![],
        accepted: None,
        cause: None,
    };
    // Defensa de la unión entre apertura y objeto admitido, además de admit().
    let expected = admitted.receipt_candidate();
    if open.custody.field("entrega") != admitted.context().documentary()
        || open.custody.field("vinculo") != admitted.identity().documentary()
        || open.custody.field("id") != expected.field("caso")
        || open.custody.field("perfil_fuente") != expected.field("perfil_fuente")
        || open.custody.field("fuente").field("sha256") != expected.field("fuente_sha256")
        || open.custody.field("estado_matematico").field("sha256")
            != expected.field("estado_sha256")
        || open.custody.field("geometria").field("sha256") != expected.field("geometria_sha256")
        || open.custody.field("geometria").field("bytes") != expected.field("geometria_bytes")
        || open.custody.field("soporte") != expected.field("soporte")
        || open.custody.field("dimensiones_ir_esperadas") != expected.field("dimensiones_ir")
    {
        obs.cause = Some("apertura_no_ligada_al_admitido".into());
        return obs;
    }
    if open.profile == Profile::SupportContinuity {
        obs.cause = Some("continuidad_soporte_no_ofrecida".into());
        return obs;
    }
    let mut certified: Option<J> = None;
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<Vec<u8>, String> {
        after_admission()?;
        obs.barriers.push("barrera_tras_admision");
        if stop == Stop::BeforeDispatch {
            obs.barriers.push("inyeccion_panico_antes_despacho");
            panic!("R06 control antes despacho");
        }
        // Captura efectiva en el receptor local, no intención del productor.
        obs.capture = Some(Capture {
            captor,
            context: capture_context,
            bytes: admitted.descriptor().to_vec(),
        });
        obs.barriers.push("consumidor_local_capturado");
        after_capture()?;
        if stop == Stop::AfterCapture {
            obs.barriers.push("inyeccion_panico_tras_captura");
            panic!("R06 control tras captura");
        }
        let receipt = certify(&admitted, obs.capture.as_ref(), false, registry)
            .map_err(|e| format!("{}: {}", e.guard, e.cause))?;
        certified = Some(receipt.clone());
        envelope(&open.id, receipt)
    }));
    match result {
        Err(_) => {
            obs.termination = "PANICO_OBSERVADO";
            obs.cause = Some("sin_informe_final".into());
            return obs;
        }
        Ok(Err(e)) => {
            obs.termination = "RETORNO_ERROR";
            obs.cause = Some(e);
            return obs;
        }
        Ok(Ok(mut raw)) => {
            obs.termination = "RETORNO";
            match fault {
                Fault::None => {}
                Fault::Absent => raw.clear(),
                Fault::Truncated => {
                    raw.pop();
                }
                Fault::WrongAttempt => {
                    if let Some(r) = &certified {
                        raw = envelope("INTENTO-AJENO", r.clone()).unwrap_or_default();
                    }
                }
                Fault::WrongReceipt => {
                    raw = envelope(&open.id, J::Null).unwrap_or_default();
                }
                Fault::Excess => raw.resize(REPORT_LIMIT + 1, 0),
            }
            obs.raw = raw; // Canal en memoria acotado del ensayo; conserva exceso centinela.
        }
    }
    if let Some(expected) = certified {
        match receive(&obs.raw, &open.id, &expected) {
            Ok(receipt) => {
                obs.accepted = Some(receipt);
                obs.status = "INFORME_LOCAL_ACEPTADO";
            }
            Err(e) => obs.cause = Some(e.into()),
        }
    } else {
        obs.cause = Some("certificacion_ausente".into());
    }
    obs
}
