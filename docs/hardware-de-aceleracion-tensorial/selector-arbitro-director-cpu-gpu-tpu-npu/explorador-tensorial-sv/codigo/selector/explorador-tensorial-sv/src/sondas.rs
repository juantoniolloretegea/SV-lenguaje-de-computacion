//! Lecturas de sólo consulta. No instalan controladores ni ejecutan modelos.
use crate::*;
pub struct Cpu;
impl Sonda for Cpu {
    fn consultar(&self) -> Consulta {
        let mut s = sysinfo::System::new();
        s.refresh_cpu_list(sysinfo::CpuRefreshKind::new());
        s.refresh_memory();
        let mut c = Consulta::nueva("cpu-sistema");
        if s.cpus().is_empty() {
            return Consulta::fallo(
                "cpu-sistema",
                EstadoConsulta::NoDisponible,
                "CPU no observable".into(),
            );
        }
        let marcas: BTreeSet<_> = s.cpus().iter().map(|p| p.brand().to_string()).collect();
        c.observaciones.push(Observacion {
            id: "cpu-anfitrion".into(),
            clase: Clase::Cpu,
            nombre: marcas.into_iter().collect::<Vec<_>>().join("; "),
            acceso: Acceso::Consultable,
            memoria_total_bytes: Some(s.total_memory()),
            memoria_libre_bytes: Some(s.available_memory()),
            interfaz: format!(
                "sysinfo/0.32.1;{};hilos={}",
                std::env::consts::OS,
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(0)
            ),
        });
        c
    }
}

pub struct Nvidia;
impl Sonda for Nvidia {
    fn consultar(&self) -> Consulta {
        let mut c = Consulta::nueva("nvidia-nvml");
        let nv = match nvml_wrapper::Nvml::init() {
            Ok(v) => v,
            Err(e) => {
                return Consulta::fallo(
                    &c.fuente,
                    EstadoConsulta::NoDisponible,
                    format!("NVML no consultable: {e}"),
                )
            }
        };
        let cantidad = match nv.device_count() {
            Ok(n) => n,
            Err(e) => {
                return Consulta::fallo(
                    &c.fuente,
                    EstadoConsulta::NoDisponible,
                    format!("Enumeración NVML: {e}"),
                )
            }
        };
        let version = nv
            .sys_driver_version()
            .unwrap_or_else(|_| "version-no-consultable".into());
        for n in 0..cantidad {
            let consultar = || -> Result<Observacion, nvml_wrapper::error::NvmlError> {
                let d = nv.device_by_index(n)?;
                let m = d.memory_info()?;
                Ok(Observacion {
                    id: d.uuid()?,
                    clase: Clase::Gpu,
                    nombre: d.name()?,
                    acceso: Acceso::Consultable,
                    memoria_total_bytes: Some(m.total),
                    memoria_libre_bytes: Some(m.free),
                    interfaz: format!("NVML;controlador={version};PCI={}", d.pci_info()?.bus_id),
                })
            };
            match consultar() {
                Ok(o) => c.observaciones.push(o),
                Err(e) => {
                    c.estado = EstadoConsulta::Parcial;
                    c.incidencias.push(format!("Dispositivo {n}: {e}"));
                }
            }
        }
        c
    }
}

/// Inventario del sistema operativo, distinto de la consulta del motor de cálculo.
pub struct Sistema;
impl Sonda for Sistema {
    fn consultar(&self) -> Consulta {
        #[cfg(windows)]
        {
            windows::consultar()
        }
        #[cfg(target_os = "linux")]
        {
            linux::consultar()
        }
        #[cfg(not(any(windows, target_os = "linux")))]
        {
            Consulta::fallo(
                "sistema",
                EstadoConsulta::NoAplicable,
                "Sistema sin realización de inventario".into(),
            )
        }
    }
}
#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

/// Frontera tipada para una interfaz PJRT ya abierta por el adaptador autorizado.
/// Implementar este rasgo requiere consultar el cliente; no es entrada JSON.
pub trait ConsultaTpu {
    fn plataforma(&self) -> Result<String, String>;
    fn dispositivos(&self) -> Result<Vec<Observacion>, String>;
}
pub struct Tpu<'a>(pub &'a dyn ConsultaTpu);
/// El ejecutable de inventario no recibe por sí solo un cliente PJRT autorizado.
pub struct SinClienteTpu;
impl Sonda for SinClienteTpu {
    fn consultar(&self) -> Consulta {
        Consulta::fallo(
            "tpu-pjrt",
            EstadoConsulta::NoAplicable,
            "Cliente PJRT no suministrado por un adaptador; hardware TPU no evaluado".into(),
        )
    }
}
#[cfg(feature = "pjrt")]
pub mod pjrt;
impl Sonda for Tpu<'_> {
    fn consultar(&self) -> Consulta {
        let fuente = "tpu-pjrt";
        match self.0.plataforma() {
            Ok(p) if p.eq_ignore_ascii_case("tpu") => {}
            Ok(_) => {
                return Consulta::fallo(
                    fuente,
                    EstadoConsulta::NoDisponible,
                    "PJRT identifica otra plataforma".into(),
                )
            }
            Err(e) => return Consulta::fallo(fuente, EstadoConsulta::NoDisponible, e),
        }
        match self.0.dispositivos() {
            Ok(ds) if ds.iter().all(|d| d.clase == Clase::Tpu) => Consulta {
                observaciones: ds,
                ..Consulta::nueva(fuente)
            },
            Ok(_) => Consulta::fallo(
                fuente,
                EstadoConsulta::NoDisponible,
                "Clase PJRT discordante".into(),
            ),
            Err(e) => Consulta::fallo(fuente, EstadoConsulta::NoDisponible, e),
        }
    }
}
