//! Selector del Árbitro - Director CPU/GPU/TPU/NPU.
//!
//! Realización preliminar de la selección, sin exploración ni ejecución física.
//! Sus entradas proceden de la admisión, el catálogo y los adaptadores del SV;
//! no se reciben directamente de la IA. No concede permisos ni cambia semántica.
//! Una decisión no reserva memoria ni acredita que se haya ejecutado el cálculo.
#![forbid(unsafe_code)]

use std::collections::BTreeSet;

pub const NOMBRE: &str = "Selector del Árbitro - Director CPU/GPU/TPU/NPU";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Clase {
    Cpu,
    Gpu,
    Tpu,
    Npu,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Destino {
    pub servidor: String,
    pub dispositivo: String,
    pub adaptador_revision: String,
}

/// Identifica una realización concreta, no sólo una familia de operaciones.
/// Perfil incluye exactitud o precisión, cuantización, contexto y límites.
/// La revisión incluye la identidad del modelo cuando la carga es inferencia.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Carga {
    pub contrato: String,
    pub revision: String,
    pub perfil: String,
    pub dimensiones: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct Solicitud {
    pub id: String,
    pub carga: Carga,
    /// Memoria adicional total de esta solicitud: incluye espacios temporales.
    /// Para inferencia debe contemplar pesos no residentes y caché de contexto.
    pub memoria_adicional_bytes: u64,
    pub margen_bytes: u64,
    /// Si existe una sesión residente no se traslada implícitamente.
    pub residencia_obligatoria: Option<Destino>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Estado {
    Detectado,
    DisponibleSinContraste,
    Comprobado,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cobertura {
    Completa,
    Parcial,
}

/// Registro suministrado por un adaptador de confianza y un catálogo cotejado.
/// El nombre de un dispositivo o una biblioteca instalada no basta para crearlo.
#[derive(Clone, Debug)]
pub struct Capacidad {
    pub carga: Carga,
    pub cobertura: Cobertura,
    pub evidencia: String,
    /// Estimación comparable para esta carga, incluidos preparación, espera,
    /// transferencias y recuperación. No representa una medición de este envío.
    pub estimacion_total_us: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Recurso {
    pub destino: Destino,
    pub clase: Clase,
    pub estado: Estado,
    /// Tiempo monotónico del supervisor, común para todas las observaciones.
    pub observado_ms: u64,
    pub memoria_disponible_bytes: Option<u64>,
    pub disponible: bool,
    pub capacidades: Vec<Capacidad>,
}

#[derive(Clone, Debug)]
pub struct Politica {
    pub revision: String,
    /// Destinos previamente permitidos. La función no amplía esta lista.
    pub destinos_permitidos: Vec<Destino>,
    pub preferencia: Vec<Clase>,
    pub clase_obligatoria: Option<Clase>,
    pub vigencia_ms: u64,
    /// Sólo compara tiempos si todos los candidatos aptos tienen estimación.
    /// Si falta alguna, utiliza el orden explícito y declara esa limitación.
    pub comparar_estimaciones: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Motivo {
    NoPermitido,
    ClaseExcluida,
    ClaseObligatoria,
    Residencia,
    NoComprobado,
    NoDisponible,
    ObservacionFutura,
    ObservacionVencida,
    MemoriaDesconocida,
    MemoriaInsuficiente,
    CargaNoComprobada,
    CoberturaParcial,
    EvidenciaAusente,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Descarte {
    pub destino: Destino,
    pub motivo: Motivo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Criterio {
    PreferenciaExplicita,
    PreferenciaPorEstimacionesIncompletas,
    MenorTiempoTotalEstimado,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    pub solicitud: String,
    pub politica_revision: String,
    pub destino: Destino,
    pub clase: Clase,
    pub criterio: Criterio,
    pub evidencia_capacidad: String,
    pub observado_ms: u64,
    pub decidida_ms: u64,
    pub descartes: Vec<Descarte>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    SolicitudIncompleta,
    PoliticaInvalida,
    DesbordamientoMemoria,
    InventarioAmbiguo,
    NingunDestino(Vec<Descarte>),
}

fn texto(s: &str) -> bool {
    !s.trim().is_empty()
}
fn destino_valido(d: &Destino) -> bool {
    texto(&d.servidor) && texto(&d.dispositivo) && texto(&d.adaptador_revision)
}
fn carga_valida(c: &Carga) -> bool {
    texto(&c.contrato)
        && texto(&c.revision)
        && texto(&c.perfil)
        && c.dimensiones.iter().all(|&n| n > 0)
}

fn evaluar<'a>(
    s: &Solicitud,
    p: &Politica,
    r: &'a Recurso,
    ahora: u64,
    memoria: u64,
) -> Result<&'a Capacidad, Motivo> {
    if !p.destinos_permitidos.contains(&r.destino) {
        return Err(Motivo::NoPermitido);
    }
    if !p.preferencia.contains(&r.clase) {
        return Err(Motivo::ClaseExcluida);
    }
    if p.clase_obligatoria.is_some_and(|c| c != r.clase) {
        return Err(Motivo::ClaseObligatoria);
    }
    if s.residencia_obligatoria
        .as_ref()
        .is_some_and(|d| d != &r.destino)
    {
        return Err(Motivo::Residencia);
    }
    if r.estado != Estado::Comprobado {
        return Err(Motivo::NoComprobado);
    }
    if !r.disponible {
        return Err(Motivo::NoDisponible);
    }
    let edad = ahora
        .checked_sub(r.observado_ms)
        .ok_or(Motivo::ObservacionFutura)?;
    if edad > p.vigencia_ms {
        return Err(Motivo::ObservacionVencida);
    }
    if r.memoria_disponible_bytes
        .ok_or(Motivo::MemoriaDesconocida)?
        < memoria
    {
        return Err(Motivo::MemoriaInsuficiente);
    }
    let cap = r
        .capacidades
        .iter()
        .find(|c| c.carga == s.carga)
        .ok_or(Motivo::CargaNoComprobada)?;
    if cap.cobertura != Cobertura::Completa {
        return Err(Motivo::CoberturaParcial);
    }
    if !texto(&cap.evidencia) {
        return Err(Motivo::EvidenciaAusente);
    }
    Ok(cap)
}

/// Decide exclusivamente entre destinos declarados y contrastados para la carga.
/// No hay descubrimiento de hardware, ejecución, reserva ni sustitución posterior.
pub fn seleccionar(
    s: &Solicitud,
    p: &Politica,
    recursos: &[Recurso],
    ahora: u64,
) -> Result<Decision, Error> {
    if !texto(&s.id)
        || !carga_valida(&s.carga)
        || s.residencia_obligatoria
            .as_ref()
            .is_some_and(|d| !destino_valido(d))
    {
        return Err(Error::SolicitudIncompleta);
    }
    let clases: BTreeSet<_> = p.preferencia.iter().collect();
    let permitidos: BTreeSet<_> = p.destinos_permitidos.iter().collect();
    if !texto(&p.revision)
        || clases.is_empty()
        || clases.len() != p.preferencia.len()
        || permitidos.len() != p.destinos_permitidos.len()
        || p.destinos_permitidos.iter().any(|d| !destino_valido(d))
        || p.clase_obligatoria.is_some_and(|c| !clases.contains(&c))
    {
        return Err(Error::PoliticaInvalida);
    }
    let memoria = s
        .memoria_adicional_bytes
        .checked_add(s.margen_bytes)
        .ok_or(Error::DesbordamientoMemoria)?;
    let mut vistos = BTreeSet::new();
    for r in recursos {
        if !destino_valido(&r.destino)
            || !vistos.insert(&r.destino)
            || r.capacidades.iter().filter(|c| c.carga == s.carga).count() > 1
        {
            return Err(Error::InventarioAmbiguo);
        }
    }
    let mut descartes = Vec::new();
    let mut aptos = Vec::new();
    for r in recursos {
        match evaluar(s, p, r, ahora, memoria) {
            Ok(c) => aptos.push((r, c)),
            Err(motivo) => descartes.push(Descarte {
                destino: r.destino.clone(),
                motivo,
            }),
        }
    }
    if aptos.is_empty() {
        return Err(Error::NingunDestino(descartes));
    }
    let todos_medidos = aptos.iter().all(|(_, c)| c.estimacion_total_us.is_some());
    let por_tiempo = p.comparar_estimaciones && todos_medidos;
    let criterio = if por_tiempo {
        Criterio::MenorTiempoTotalEstimado
    } else if p.comparar_estimaciones {
        Criterio::PreferenciaPorEstimacionesIncompletas
    } else {
        Criterio::PreferenciaExplicita
    };
    aptos.sort_by_key(|(r, c)| {
        (
            if por_tiempo {
                c.estimacion_total_us.unwrap()
            } else {
                0
            },
            p.preferencia.iter().position(|k| k == &r.clase).unwrap(),
            &r.destino,
        )
    });
    let (r, c) = aptos[0];
    Ok(Decision {
        solicitud: s.id.clone(),
        politica_revision: p.revision.clone(),
        destino: r.destino.clone(),
        clase: r.clase,
        criterio,
        evidencia_capacidad: c.evidencia.clone(),
        observado_ms: r.observado_ms,
        decidida_ms: ahora,
        descartes,
    })
}

#[cfg(test)]
mod tests;
