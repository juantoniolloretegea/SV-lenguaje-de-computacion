//! Inventario PnP mediante la API del sistema, sin comandos auxiliares.
use super::*;
use windows_sys::{
    core::GUID,
    Win32::{
        Devices::DeviceAndDriverInstallation::*,
        Foundation::{GetLastError, ERROR_NO_MORE_ITEMS},
    },
};
struct Lista(HDEVINFO);
impl Drop for Lista {
    fn drop(&mut self) {
        // SAFETY: este valor procede de SetupDiGetClassDevsW y se libera una sola vez.
        unsafe {
            SetupDiDestroyDeviceInfoList(self.0);
        }
    }
}
fn propiedad(h: HDEVINFO, d: &SP_DEVINFO_DATA, k: u32) -> Option<String> {
    let mut b = vec![0u16; 8192];
    let mut tipo = 0;
    let mut requerido = 0;
    // SAFETY: búfer UTF-16 alineado, tamaño en bytes y estructuras válidas de la API.
    let ok = unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            h,
            d,
            k,
            &mut tipo,
            b.as_mut_ptr().cast(),
            (b.len() * 2) as u32,
            &mut requerido,
        )
    };
    if ok == 0 || requerido as usize > b.len() * 2 || requerido % 2 != 0 {
        return None;
    }
    // REG_SZ y REG_MULTI_SZ. Sólo se interpreta la primera cadena de cada propiedad.
    if tipo != 1 && tipo != 7 {
        return None;
    }
    let n = b.iter().position(|&v| v == 0)?;
    String::from_utf16(&b[..n]).ok()
}
fn id(h: HDEVINFO, d: &SP_DEVINFO_DATA) -> Option<String> {
    let mut b = vec![0u16; 8192];
    let mut requerido = 0;
    // SAFETY: búfer UTF-16 y longitud en unidades UTF-16, sin retención por la API.
    let ok = unsafe {
        SetupDiGetDeviceInstanceIdW(h, d, b.as_mut_ptr(), b.len() as u32, &mut requerido)
    };
    if ok == 0 || requerido as usize > b.len() {
        return None;
    }
    let n = b.iter().position(|&v| v == 0)?;
    String::from_utf16(&b[..n]).ok()
}
pub fn consultar() -> Consulta {
    let mut c = Consulta::nueva("windows-pnp");
    for (guid, clase) in [
        (GUID_DEVCLASS_DISPLAY, Clase::Gpu),
        (
            GUID_DEVCLASS_COMPUTEACCELERATOR,
            Clase::AceleradorSinClasificar,
        ),
    ] {
        // SAFETY: GUID de clase válida, sin ventana ni enumerador; consulta sólo presentes.
        let h = unsafe {
            SetupDiGetClassDevsW(
                &guid as *const GUID,
                std::ptr::null(),
                std::ptr::null_mut(),
                DIGCF_PRESENT,
            )
        };
        if h == -1 {
            c.estado = EstadoConsulta::Parcial;
            c.incidencias.push("Clase PnP no consultable".into());
            continue;
        }
        let lista = Lista(h);
        let mut n = 0;
        loop {
            let mut d = SP_DEVINFO_DATA {
                cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
                ..Default::default()
            };
            // SAFETY: estructura inicializada con tamaño requerido y lista aún abierta.
            if unsafe { SetupDiEnumDeviceInfo(lista.0, n, &mut d) } == 0 {
                // SAFETY: lectura del último error del hilo inmediatamente tras la llamada.
                if unsafe { GetLastError() } != ERROR_NO_MORE_ITEMS {
                    c.estado = EstadoConsulta::Parcial;
                    c.incidencias.push("Enumeración PnP incompleta".into());
                }
                break;
            }
            n += 1;
            let (Some(id), Some(nombre)) =
                (id(lista.0, &d), propiedad(lista.0, &d, SPDRP_DEVICEDESC))
            else {
                c.estado = EstadoConsulta::Parcial;
                c.incidencias
                    .push("Dispositivo sin identidad recuperable".into());
                continue;
            };
            let servicio = propiedad(lista.0, &d, SPDRP_SERVICE)
                .unwrap_or_else(|| "servicio-no-observable".into());
            let hw = propiedad(lista.0, &d, SPDRP_HARDWAREID)
                .unwrap_or_else(|| "identidad-hardware-no-observable".into());
            let mut estado = 0;
            let mut problema = 0;
            // SAFETY: dos resultados escalares válidos y DevInst recibido de SetupAPI.
            let cr = unsafe { CM_Get_DevNode_Status(&mut estado, &mut problema, d.DevInst, 0) };
            let acceso = if cr == CR_SUCCESS && problema != 0 {
                Acceso::NoDisponible
            } else {
                Acceso::Identificado
            };
            if cr != CR_SUCCESS {
                c.incidencias.push("Estado PnP no consultable".into());
                c.estado = EstadoConsulta::Parcial;
            }
            c.observaciones.push(Observacion {
                id,
                clase,
                nombre,
                acceso,
                memoria_total_bytes: None,
                memoria_libre_bytes: None,
                interfaz: format!("SetupAPI;servicio={servicio};hardware={hw};problema={problema}"),
            });
        }
    }
    c
}
