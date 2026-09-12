//! Correspondencia documental experimental; no confiere autoridad de dominio.
use crate::{EnlacePublico, ErrorEnlace};

pub const VERSION: &str = "IE004-LECTURA-VINCULADA/1";
pub const OPERACION: &str = "LEER_RECIBO_PUBLICO";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FalloLectura {
    LimiteCabecera,
    Version,
    Operacion,
    Identidad,
    Presentacion(g1::FalloPresentacion),
}

/// Entrada no confiable. Ninguno de sus campos constituye autorización.
pub struct PropuestaDeLectura<'a> {
    pub version: &'a str,
    pub operacion: &'a str,
    pub identidad: (u64, u64),
    pub texto: &'a [u8],
}

/// El conductor obtiene esta referencia antes de recibir la propuesta.
pub struct ConsultaDeLectura<'a> {
    entrega: g1::EntregaLiteral<'a>,
    solicitud: &'a [u8],
    caso: &'a [u8],
    lote: &'a [u8],
    montaje: &'a [u8],
    traza_adaptador: &'a [u8],
    traza_a: &'a [u8],
    posicion: usize,
    rango: (usize, usize),
}

/// Vista inmutable, construible sólo tras comprobar la propuesta.
pub struct LecturaEntregable<'v, 'a> {
    consulta: &'v ConsultaDeLectura<'a>,
    presentacion: g1::PresentacionComprobada<'v>,
}

impl EnlacePublico {
    pub fn consulta_lectura(&self, posicion: usize) -> Result<ConsultaDeLectura<'_>, ErrorEnlace> {
        let entrega = self.entrega(posicion)?;
        let solicitud = self.recuperar(posicion, g1::Papel::Entrada)?;
        let traza_a = self.recuperar(posicion, g1::Papel::Traza)?;
        let rango = self.rango(posicion)?;
        let (lote, montaje) = self.originales();
        let caso = lote.get(rango.0..rango.1).ok_or(ErrorEnlace::Correspondencia)?;
        let (_, traza_adaptador) = self.productos_adaptador();
        Ok(ConsultaDeLectura { entrega, solicitud, caso, lote, montaje,
            traza_adaptador, traza_a, posicion, rango })
    }
}

impl<'a> ConsultaDeLectura<'a> {
    pub fn identidad(&self) -> (u64, u64) { self.entrega.identidad() }
    pub fn comprobar<'v>(&'v self, propuesta: PropuestaDeLectura<'v>)
        -> Result<LecturaEntregable<'v, 'a>, FalloLectura>
    {
        if propuesta.version.len() > 64 || propuesta.operacion.len() > 64 {
            return Err(FalloLectura::LimiteCabecera);
        }
        if propuesta.version != VERSION { return Err(FalloLectura::Version); }
        if propuesta.operacion != OPERACION { return Err(FalloLectura::Operacion); }
        if propuesta.identidad != self.identidad() { return Err(FalloLectura::Identidad); }
        let presentacion = self.entrega.comprobar_presentacion(propuesta.texto)
            .map_err(FalloLectura::Presentacion)?;
        Ok(LecturaEntregable { consulta: self, presentacion })
    }
}

impl LecturaEntregable<'_, '_> {
    pub fn version(&self) -> &'static str { VERSION }
    pub fn operacion(&self) -> &'static str { OPERACION }
    pub fn identidad(&self) -> (u64, u64) { self.consulta.identidad() }
    pub fn texto(&self) -> &[u8] { self.presentacion.texto() }
    pub fn original(&self) -> &[u8] { self.presentacion.original() }
    pub fn perfil(&self) -> &'static str { self.presentacion.perfil() }
    pub fn solicitud(&self) -> &[u8] { self.consulta.solicitud }
    pub fn caso_original(&self) -> &[u8] { self.consulta.caso }
    pub fn lote_original(&self) -> &[u8] { self.consulta.lote }
    pub fn montaje_original(&self) -> &[u8] { self.consulta.montaje }
    pub fn traza_adaptador(&self) -> &[u8] { self.consulta.traza_adaptador }
    pub fn traza_a(&self) -> &[u8] { self.consulta.traza_a }
    pub fn posicion(&self) -> usize { self.consulta.posicion }
    pub fn rango(&self) -> (usize, usize) { self.consulta.rango }
}
