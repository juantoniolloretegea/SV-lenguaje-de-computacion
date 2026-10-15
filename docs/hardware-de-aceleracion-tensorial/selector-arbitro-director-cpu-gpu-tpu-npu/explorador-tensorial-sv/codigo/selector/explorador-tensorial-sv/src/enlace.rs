//! Enlace con el Selector existente, sin modificar su representación ni sus reglas.
use crate::*;
use selector::{Capacidad, Cobertura, Destino, Estado, Recurso};

/// Resultado recibido de un adaptador de confianza que ha contrastado la carga.
/// El Explorador no crea esta evidencia ni comprueba su contenido por el nombre.
pub struct CapacidadRecibida {
    pub(crate) fuente: String,
    pub(crate) identidad: String,
    pub(crate) huella_identidad: String,
    pub(crate) destino: Destino,
    pub(crate) capacidad: Capacidad,
    pub(crate) contrastada_ms: u64,
    pub(crate) vigencia_ms: u64,
    pub(crate) sesion: u64,
}

pub struct EvidenciaAdaptador {
    pub fuente: String,
    pub identidad: String,
    pub destino: Destino,
    pub capacidad: Capacidad,
    pub contrastada_ms: u64,
    pub vigencia_ms: u64,
}

/// Decisión consumible una vez por esta interfaz; no constituye un permiso R1.
pub struct SeleccionObservada {
    decision: selector::Decision,
    solicitud: selector::Solicitud,
    politica: selector::Politica,
    sesion: u64,
}
impl SeleccionObservada {
    pub fn decision(&self) -> &selector::Decision {
        &self.decision
    }
}

impl Explorador {
    pub fn recibir_capacidad(
        &self,
        i: &Inventario,
        e: EvidenciaAdaptador,
    ) -> Result<CapacidadRecibida, String> {
        if i.sesion != self.sesion {
            return Err("Inventario de otra sesión".into());
        }
        let o = i
            .consultas
            .iter()
            .find(|q| q.fuente == e.fuente)
            .and_then(|q| q.observaciones.iter().find(|o| o.id == e.identidad))
            .ok_or("Dispositivo no observado")?;
        let c = CapacidadRecibida {
            fuente: e.fuente,
            identidad: e.identidad,
            huella_identidad: o.huella_identidad(),
            destino: e.destino,
            capacidad: e.capacidad,
            contrastada_ms: e.contrastada_ms,
            vigencia_ms: e.vigencia_ms,
            sesion: self.sesion,
        };
        self.recursos(i, std::slice::from_ref(&c), e.vigencia_ms)?;
        Ok(c)
    }
    pub fn seleccionar(
        &self,
        i: &Inventario,
        cs: &[CapacidadRecibida],
        s: selector::Solicitud,
        p: selector::Politica,
    ) -> Result<SeleccionObservada, String> {
        let r = self.recursos(i, cs, p.vigencia_ms)?;
        let d = selector::seleccionar(&s, &p, &r, self.ahora_ms())
            .map_err(|e| format!("Selección rechazada: {e:?}"))?;
        Ok(SeleccionObservada {
            decision: d,
            solicitud: s,
            politica: p,
            sesion: self.sesion,
        })
    }
    pub fn recursos(
        &self,
        inventario: &Inventario,
        capacidades: &[CapacidadRecibida],
        vigencia_observacion_ms: u64,
    ) -> Result<Vec<Recurso>, String> {
        self.recursos_en(
            inventario,
            capacidades,
            vigencia_observacion_ms,
            self.ahora_ms(),
        )
    }
    pub(crate) fn recursos_en(
        &self,
        i: &Inventario,
        cs: &[CapacidadRecibida],
        vigencia: u64,
        ahora: u64,
    ) -> Result<Vec<Recurso>, String> {
        if i.sesion != self.sesion {
            return Err("Inventario de otra sesión".into());
        }
        if ahora
            .checked_sub(i.observado_ms)
            .is_none_or(|edad| edad > vigencia)
        {
            return Err("Inventario vencido o de instante futuro".into());
        }
        let mut recursos = Vec::new();
        let mut destinos = BTreeSet::new();
        for c in cs {
            if c.sesion != self.sesion {
                return Err("Capacidad de otra sesión".into());
            }
            if !destinos.insert(c.destino.clone()) {
                return Err("Destino duplicado en el catálogo".into());
            }
            let consulta = i
                .consultas
                .iter()
                .find(|q| q.fuente == c.fuente)
                .ok_or("Fuente no observada")?;
            // Una fuente incompleta nunca promueve una capacidad de ejecución.
            if consulta.estado != EstadoConsulta::Completa {
                return Err("Fuente no comprobada íntegramente".into());
            }
            let o = consulta
                .observaciones
                .iter()
                .find(|o| o.id == c.identidad)
                .ok_or("Dispositivo no observado")?;
            if o.acceso != Acceso::Consultable {
                return Err("Interfaz del dispositivo no consultable".into());
            }
            if c.huella_identidad != o.huella_identidad() || c.destino.dispositivo != o.id {
                return Err("Identidad del catálogo discordante".into());
            }
            if ahora
                .checked_sub(c.contrastada_ms)
                .is_none_or(|edad| edad > c.vigencia_ms)
            {
                return Err("Capacidad vencida o de instante futuro".into());
            }
            if c.capacidad.cobertura != Cobertura::Completa
                || c.capacidad.evidencia.trim().is_empty()
            {
                return Err("Operación sin contraste completo".into());
            }
            let clase = match o.clase {
                Clase::Cpu => selector::Clase::Cpu,
                Clase::Gpu => selector::Clase::Gpu,
                Clase::Tpu => selector::Clase::Tpu,
                Clase::Npu => selector::Clase::Npu,
                Clase::AceleradorSinClasificar => {
                    return Err("Acelerador sin clase comprobada".into())
                }
            };
            recursos.push(Recurso {
                destino: c.destino.clone(),
                clase,
                estado: Estado::Comprobado,
                observado_ms: i.observado_ms,
                memoria_disponible_bytes: o.memoria_libre_bytes,
                disponible: true,
                capacidades: vec![c.capacidad.clone()],
            });
        }
        Ok(recursos)
    }
    /// Reconsulta inmediata y nueva decisión; la función de ejecución recibe el
    /// destino comprobado. Su imposición física corresponde al adaptador.
    pub fn ejecutar_revalidado<T>(
        &self,
        sondas: &[&dyn Sonda],
        capacidades: &[CapacidadRecibida],
        seleccion: SeleccionObservada,
        ejecutar: impl FnOnce(&selector::Decision) -> Result<T, String>,
    ) -> Result<T, String> {
        if seleccion.sesion != self.sesion {
            return Err("Decisión de otra sesión".into());
        }
        let previa = &seleccion.decision;
        let solicitud = &seleccion.solicitud;
        let politica = &seleccion.politica;
        if self
            .ahora_ms()
            .checked_sub(previa.decidida_ms)
            .is_none_or(|edad| edad > politica.vigencia_ms)
        {
            return Err("Decisión vencida".into());
        }
        let inventario = self.explorar(sondas)?;
        let recursos = self.recursos(&inventario, capacidades, politica.vigencia_ms)?;
        let actual = selector::seleccionar(solicitud, politica, &recursos, self.ahora_ms())
            .map_err(|e| format!("Selección rechazada: {e:?}"))?;
        if previa.solicitud != actual.solicitud
            || previa.politica_revision != actual.politica_revision
            || previa.destino != actual.destino
            || previa.clase != actual.clase
            || previa.evidencia_capacidad != actual.evidencia_capacidad
            || previa.criterio != actual.criterio
        {
            return Err("La decisión ha cambiado; requiere nueva admisión".into());
        }
        ejecutar(&actual)
    }
}
