//! Exploración material subordinada al Selector. No produce valores semánticos SV.
//! Los proveedores de observaciones y capacidades pertenecen al proceso de confianza.
//! No se admiten inventarios o capacidades deserializados desde solicitudes de una IA.
#![deny(unsafe_op_in_unsafe_fn)]

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
pub mod enlace;
pub mod sondas;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Clase {
    Cpu,
    Gpu,
    Tpu,
    Npu,
    AceleradorSinClasificar,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Acceso {
    Identificado,
    Consultable,
    NoDisponible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum EstadoConsulta {
    Completa,
    Parcial,
    NoDisponible,
    NoAplicable,
}

#[derive(Clone, Debug, Serialize)]
pub struct Observacion {
    /// Identidad local a la fuente; no equivale por sí sola a identidad física atestada.
    pub id: String,
    pub clase: Clase,
    pub nombre: String,
    pub acceso: Acceso,
    pub memoria_total_bytes: Option<u64>,
    pub memoria_libre_bytes: Option<u64>,
    /// Identidad del controlador/interfaz observable. Nunca acredita cálculo.
    pub interfaz: String,
}
impl Observacion {
    pub fn huella_identidad(&self) -> String {
        // La memoria libre y el estado son variables: se reconsultan, no son identidad.
        let bytes = serde_json::to_vec(&(
            &self.id,
            self.clase,
            &self.nombre,
            &self.interfaz,
            self.memoria_total_bytes,
        ))
        .expect("Campos serializables");
        format!("{:x}", Sha256::digest(bytes))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Consulta {
    pub fuente: String,
    pub estado: EstadoConsulta,
    pub observaciones: Vec<Observacion>,
    pub incidencias: Vec<String>,
    pub duracion_us: u64,
}
impl Consulta {
    pub fn nueva(fuente: &str) -> Self {
        Self {
            fuente: fuente.into(),
            estado: EstadoConsulta::Completa,
            observaciones: vec![],
            incidencias: vec![],
            duracion_us: 0,
        }
    }
    pub fn fallo(fuente: &str, estado: EstadoConsulta, detalle: String) -> Self {
        Self {
            estado,
            incidencias: vec![detalle],
            ..Self::nueva(fuente)
        }
    }
}

pub trait Sonda {
    fn consultar(&self) -> Consulta;
}

#[derive(Debug, Serialize)]
pub struct Inventario {
    pub esquema: &'static str,
    pub(crate) consultas: Vec<Consulta>,
    pub(crate) observado_ms: u64,
    pub(crate) duracion_us: u64,
    #[serde(skip)]
    sesion: u64,
}
impl Inventario {
    pub fn consultas(&self) -> &[Consulta] {
        &self.consultas
    }
    pub fn observado_ms(&self) -> u64 {
        self.observado_ms
    }
    pub fn duracion_us(&self) -> u64 {
        self.duracion_us
    }
}

static SIGUIENTE: AtomicU64 = AtomicU64::new(1);
pub struct Explorador {
    origen: Instant,
    sesion: u64,
}
impl Default for Explorador {
    fn default() -> Self {
        Self::nuevo()
    }
}
impl Explorador {
    pub fn nuevo() -> Self {
        Self {
            origen: Instant::now(),
            sesion: SIGUIENTE.fetch_add(1, Ordering::Relaxed),
        }
    }
    pub fn ahora_ms(&self) -> u64 {
        self.origen
            .elapsed()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX)
    }
    pub fn explorar(&self, sondas: &[&dyn Sonda]) -> Result<Inventario, String> {
        let inicio = Instant::now();
        let observado_ms = self.ahora_ms();
        let mut consultas = Vec::new();
        let mut fuentes = BTreeSet::new();
        for s in sondas {
            let t = Instant::now();
            let mut c = s.consultar();
            c.duracion_us = t.elapsed().as_micros().try_into().unwrap_or(u64::MAX);
            if c.fuente.trim().is_empty() || !fuentes.insert(c.fuente.clone()) {
                return Err("Fuente vacía o repetida".into());
            }
            let mut ids = BTreeSet::new();
            for o in &c.observaciones {
                if o.id.trim().is_empty()
                    || o.nombre.trim().is_empty()
                    || o.interfaz.trim().is_empty()
                    || !ids.insert(o.id.clone())
                {
                    return Err("Identidad vacía o repetida en una fuente".into());
                }
                if let (Some(total), Some(libre)) = (o.memoria_total_bytes, o.memoria_libre_bytes) {
                    if libre > total {
                        return Err("Memoria libre superior al total".into());
                    }
                }
            }
            if matches!(
                c.estado,
                EstadoConsulta::NoDisponible | EstadoConsulta::NoAplicable
            ) && !c.observaciones.is_empty()
            {
                return Err("Consulta no disponible con observaciones utilizables".into());
            }
            consultas.push(c);
        }
        Ok(Inventario {
            esquema: "explorador-tensorial-sv/0.1",
            consultas,
            observado_ms,
            duracion_us: inicio.elapsed().as_micros().try_into().unwrap_or(u64::MAX),
            sesion: self.sesion,
        })
    }
}

/// Clase técnica declarada por el sistema, sin inferencia por nombre comercial.
pub fn clase_pci(clase: u32, controlador: &str) -> Option<Clase> {
    if matches!(controlador, "amdxdna" | "ivpu") {
        Some(Clase::Npu)
    } else if clase >> 16 == 3 {
        Some(Clase::Gpu)
    } else if clase >> 16 == 0x12 {
        Some(Clase::AceleradorSinClasificar)
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
