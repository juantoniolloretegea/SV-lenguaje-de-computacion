#![forbid(unsafe_code)]
//! Candidata documental limitada al banco fijado de S2; no crea autoridad.
use lote::{EnlacePublico, ErrorEnlace};
use lote::lectura::{ConsultaDeLectura, PropuestaDeLectura, LecturaEntregable, FalloLectura};
include!("rangos_fijados.rs");
pub const VERSION: &str = "S2-COBERTURA-BANCO/1";
#[derive(Debug,PartialEq,Eq)]
pub enum FalloReferencia { AlcanceNoConstituido, Enlace(ErrorEnlace), Intervalo }
#[derive(Debug,PartialEq,Eq)]
pub enum FalloCobertura { Lectura(FalloLectura), FaltaCaso, FaltaVigencia, CasoDistinto, VigenciaDistinta }
pub struct Seleccion<'p> {
 pub lectura: PropuestaDeLectura<'p>,
 pub caso: Option<&'p [u8]>,
 pub vigencia: Option<&'p [u8]>,
}
/// La referencia se obtiene del enlace fijado, antes de recibir la selección.
pub struct Referencia<'a> {
 consulta: ConsultaDeLectura<'a>,
 caso: &'a [u8],
 vigencia: &'a [u8],
}
/// No tiene constructor público; retiene la lectura y las citas comprobadas.
pub struct Entrega<'v,'a> {
 lectura: LecturaEntregable<'v,'a>,
 caso: &'v [u8],
 vigencia: &'v [u8],
}
impl<'a> Referencia<'a> {
 pub fn desde(enlace:&'a EnlacePublico,posicion:usize)->Result<Self,FalloReferencia> {
  let (_,inicio,fin)=RANGOS.iter().find(|(p,_,_)|*p==posicion).ok_or(FalloReferencia::AlcanceNoConstituido)?;
  let consulta=enlace.consulta_lectura(posicion).map_err(FalloReferencia::Enlace)?;
  let (a,z)=enlace.rango(posicion).map_err(FalloReferencia::Enlace)?;
  let (lote,montaje)=enlace.originales();
  let caso=lote.get(a..z).ok_or(FalloReferencia::Intervalo)?;
  let vigencia=montaje.get(*inicio..*fin).ok_or(FalloReferencia::Intervalo)?;
  Ok(Self{consulta,caso,vigencia})
 }
 pub fn identidad(&self)->(u64,u64){self.consulta.identidad()}
 pub fn caso_requerido(&self)->&'a [u8]{self.caso}
 pub fn vigencia_requerida(&self)->&'a [u8]{self.vigencia}
 pub fn comprobar<'v>(&'v self,seleccion:Seleccion<'v>)->Result<Entrega<'v,'a>,FalloCobertura> {
  let lectura=self.consulta.comprobar(seleccion.lectura).map_err(FalloCobertura::Lectura)?;
  let caso=seleccion.caso.ok_or(FalloCobertura::FaltaCaso)?;
  if caso!=self.caso{return Err(FalloCobertura::CasoDistinto)}
  let vigencia=seleccion.vigencia.ok_or(FalloCobertura::FaltaVigencia)?;
  if vigencia!=self.vigencia{return Err(FalloCobertura::VigenciaDistinta)}
  Ok(Entrega{lectura,caso,vigencia})
 }
}
impl Entrega<'_,'_> {
 pub fn version(&self)->&'static str{VERSION}
 pub fn lectura(&self)->&LecturaEntregable<'_,'_>{&self.lectura}
 pub fn caso_citado(&self)->&[u8]{self.caso}
 pub fn vigencia_citada(&self)->&[u8]{self.vigencia}
}
