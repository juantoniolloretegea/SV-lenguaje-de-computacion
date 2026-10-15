//! Sysfs expone dispositivos del entorno; no demuestra acceso al cálculo.
use super::*;
use std::{fs, path::Path};
fn hex(p: &Path) -> Result<u32, String> {
    let s = fs::read_to_string(p).map_err(|e| e.to_string())?;
    u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}
pub fn consultar() -> Consulta {
    let mut c = Consulta::nueva("linux-pci");
    let entradas = match fs::read_dir("/sys/bus/pci/devices") {
        Ok(e) => e,
        Err(e) => return Consulta::fallo(&c.fuente, EstadoConsulta::NoDisponible, e.to_string()),
    };
    for e in entradas {
        let e = match e {
            Ok(e) => e,
            Err(e) => {
                c.estado = EstadoConsulta::Parcial;
                c.incidencias.push(e.to_string());
                continue;
            }
        };
        let p = e.path();
        let datos = || -> Result<Option<Observacion>, String> {
            let cl = hex(&p.join("class"))?;
            let driver = fs::read_link(p.join("driver"))
                .ok()
                .and_then(|d| d.file_name().map(|s| s.to_string_lossy().into_owned()))
                .unwrap_or_default();
            let Some(clase) = clase_pci(cl, &driver) else {
                return Ok(None);
            };
            let fabricante = hex(&p.join("vendor"))?;
            let dispositivo = hex(&p.join("device"))?;
            Ok(Some(Observacion {
                id: e.file_name().to_string_lossy().into_owned(),
                clase,
                nombre: format!("PCI {fabricante:04x}:{dispositivo:04x}"),
                acceso: Acceso::Identificado,
                memoria_total_bytes: None,
                memoria_libre_bytes: None,
                interfaz: format!("sysfs;clase={cl:06x};controlador={driver}"),
            }))
        };
        match datos() {
            Ok(Some(o)) => c.observaciones.push(o),
            Ok(None) => {}
            Err(e) => {
                c.estado = EstadoConsulta::Parcial;
                c.incidencias.push(e);
            }
        }
    }
    c
}
