#![forbid(unsafe_code)]
//! Recepción contextual de laboratorio S11. No interpreta órdenes ni crea autoridad.
use cobertura::{Referencia,Seleccion,Entrega,FalloCobertura};
pub const LIMITE_DOCUMENTO:usize=8192;
#[derive(Debug,PartialEq,Eq)]
pub enum FalloContexto { LimiteDocumento, IdentidadContexto, DocumentoDistinto, Cobertura(FalloCobertura) }
/// La referencia y los bytes se fijan antes de recibir la selección.
/// El documento sólo es un dato prestado; no altera caso ni vigencia requeridos.
pub struct Contexto<'r,'a,'d> { referencia:&'r Referencia<'a>, documento:&'d [u8] }
impl<'r,'a,'d> Contexto<'r,'a,'d> {
 pub fn desde(referencia:&'r Referencia<'a>,documento:&'d [u8])->Result<Self,FalloContexto>{
  if documento.len()>LIMITE_DOCUMENTO{return Err(FalloContexto::LimiteDocumento)}
  Ok(Self{referencia,documento})
 }
 pub fn comprobar<'v>(&'v self,identidad:(u64,u64),documento:&[u8],seleccion:Seleccion<'v>)->Result<Entrega<'v,'a>,FalloContexto>{
  if documento.len()>LIMITE_DOCUMENTO{return Err(FalloContexto::LimiteDocumento)}
  let _ = identidad; // MUTANTE: se elimina la comprobación contextual.
  if documento!=self.documento{return Err(FalloContexto::DocumentoDistinto)}
  self.referencia.comprobar(seleccion).map_err(FalloContexto::Cobertura)
 }
}
