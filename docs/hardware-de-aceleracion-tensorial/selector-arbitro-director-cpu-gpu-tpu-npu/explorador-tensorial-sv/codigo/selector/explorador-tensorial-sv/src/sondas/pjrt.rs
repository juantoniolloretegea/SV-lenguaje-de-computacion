//! Consulta del mismo cliente PJRT abierto por el adaptador de ejecución.
//! No carga bibliotecas indicadas por el solicitante ni inicia un modelo.
use super::*;

pub struct Cliente<'a>(pub &'a ::pjrt::Client);
impl ConsultaTpu for Cliente<'_> {
    fn plataforma(&self) -> Result<String, String> {
        self.0
            .platform_name()
            .map(|s| s.into_owned())
            .map_err(|e| e.to_string())
    }
    fn dispositivos(&self) -> Result<Vec<Observacion>, String> {
        let version = self.0.platform_version().map_err(|e| e.to_string())?;
        self.0
            .addressable_devices()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|d| {
                let desc = d.description().map_err(|e| e.to_string())?;
                let id = desc.id().map_err(|e| e.to_string())?;
                let nombre = desc.kind().map_err(|e| e.to_string())?.to_string();
                // La versión inicial conserva identidad y acceso. Las estadísticas
                // del asignador no se convierten en memoria física libre garantizada.
                Ok(Observacion {
                    id: format!("pjrt-tpu-{id}"),
                    clase: Clase::Tpu,
                    nombre,
                    acceso: Acceso::Consultable,
                    memoria_total_bytes: None,
                    memoria_libre_bytes: None,
                    interfaz: format!("PJRT;plataforma={version}"),
                })
            })
            .collect()
    }
}
