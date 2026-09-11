//! Frontera literal experimental. La referencia procede del custodio, nunca del proponente.
use super::{av, Archivo, Custodio, Fallo, Manejador};
const MAX_MARCO: usize = 4144;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FalloEntrega { Custodia(Fallo), YaIntentada, Limite, Integridad, ContenidoDistinto }
impl From<Fallo> for FalloEntrega { fn from(e:Fallo)->Self { Self::Custodia(e) } }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EstadoEntrega { SinIntento, Comprobada, Rechazada(FalloEntrega) }
pub(super) struct IntentoEntrega { estado:EstadoEntrega, propuesta:Option<Archivo>, bytes:usize }
impl IntentoEntrega { pub(super) fn nuevo()->Self { Self{estado:EstadoEntrega::SinIntento,propuesta:None,bytes:0} } }
/// Sólo el custodio construye esta vista. No es permiso de actuación ni verdad clínica.
pub struct EntregaLiteral<'a> { cuerpo:&'a [u8], identidad:(u64,u64) }
impl EntregaLiteral<'_> {
    pub fn cuerpo(&self)->&[u8] { self.cuerpo }
    pub fn identidad(&self)->(u64,u64) { self.identidad }
}
impl Custodio {
    /// Un intento por recibo, sin reparación o reintento silencioso. Guarda la causa y
    /// los bytes acotados del intento. El conductor es responsable de acotar su lectura.
    pub fn comprobar_entrega(&mut self,h:&Manejador,propuesta:&[u8])->Result<(),FalloEntrega> {
        let i=self.indice(h)?;
        self.recibo(h)?;
        if self.slots[i].as_ref().unwrap().entrega.estado!=EstadoEntrega::SinIntento {
            return Err(FalloEntrega::YaIntentada);
        }
        self.slots[i].as_mut().unwrap().entrega.bytes=propuesta.len();
        let result=(|| {
            if propuesta.len()>MAX_MARCO { return Err(FalloEntrega::Limite); }
            // Reserva contabilizada antes de asignar; no se amplía el cupo de G1.
            self.cargo(propuesta.len() as u64)?;
            let mut bytes=Vec::new();
            bytes.try_reserve_exact(propuesta.len()).map_err(|_|Fallo::Reserva)?;
            if bytes.capacity()>propuesta.len() { self.cargo((bytes.capacity()-propuesta.len()) as u64)?; }
            bytes.extend_from_slice(propuesta);
            let archivo=Archivo{hash:av::hash(&bytes)?,len:bytes.len(),b:Some(bytes),completo:true};
            self.slots[i].as_mut().unwrap().entrega.propuesta=Some(archivo);
            let body=av::frame(propuesta).map_err(|_|FalloEntrega::Integridad)?;
            let recibo=self.recibo(h)?;
            if body!=recibo.cuerpo()? { return Err(FalloEntrega::ContenidoDistinto); }
            Ok(())
        })();
        self.slots[i].as_mut().unwrap().entrega.estado=match result {
            Ok(())=>EstadoEntrega::Comprobada, Err(e)=>EstadoEntrega::Rechazada(e)
        };
        result
    }
    pub fn estado_entrega(&self,h:&Manejador)->Result<(EstadoEntrega,usize),FalloEntrega> {
        let e=&self.slots[self.indice(h)?].as_ref().unwrap().entrega; Ok((e.estado,e.bytes))
    }
    pub fn recuperar_propuesta_entrega(&self,h:&Manejador)->Result<&[u8],FalloEntrega> {
        let e=&self.slots[self.indice(h)?].as_ref().unwrap().entrega;
        Ok(e.propuesta.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?)
    }
    /// Revalida la custodia y entrega una vista del cuerpo original, no del proponente.
    pub fn entrega(&self,h:&Manejador)->Result<EntregaLiteral<'_>,FalloEntrega> {
        let s=self.slots[self.indice(h)?].as_ref().unwrap();
        match s.entrega.estado {
            EstadoEntrega::SinIntento=>return Err(Fallo::Estado.into()),
            EstadoEntrega::Rechazada(e)=>return Err(e),
            EstadoEntrega::Comprobada=>{}
        }
        self.recibo(h)?;
        let body=av::frame(s.marco.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?)?;
        let propuesta=s.entrega.propuesta.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?;
        if av::frame(propuesta).map_err(|_|FalloEntrega::Integridad)?!=body {
            return Err(FalloEntrega::ContenidoDistinto);
        }
        Ok(EntregaLiteral{cuerpo:body,identidad:self.identidad(h)?})
    }
}
#[cfg(test)] mod tests;
