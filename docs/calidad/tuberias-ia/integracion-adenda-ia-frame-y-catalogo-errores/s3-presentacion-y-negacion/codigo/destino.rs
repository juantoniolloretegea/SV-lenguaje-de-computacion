//! Conductor de archivo del ensayo S3; no constituye un transporte productivo.
use std::{fs::OpenOptions,io::{Read,Write},path::Path};
use cobertura::{Entrega,Referencia,Seleccion,FalloCobertura};
use lote::lectura::{PropuestaDeLectura,VERSION,OPERACION};
#[derive(Debug,PartialEq,Eq)]
pub enum FalloDestino { Io(std::io::ErrorKind), Limite, Cobertura(FalloCobertura) }
impl From<std::io::Error> for FalloDestino{fn from(e:std::io::Error)->Self{Self::Io(e.kind())}}
pub fn escribir(path:&Path,entrega:&Entrega<'_,'_>)->Result<(),FalloDestino>{
 let mut f=OpenOptions::new().write(true).create_new(true).open(path)?;
 f.write_all(entrega.lectura().texto())?;f.flush()?;Ok(())
}
pub fn leer(path:&Path)->Result<Vec<u8>,FalloDestino>{
 let mut f=std::fs::File::open(path)?;let mut buffer=[0u8;16385];let mut n=0;
 loop{
  if n==buffer.len(){return Err(FalloDestino::Limite)}
  let k=f.read(&mut buffer[n..])?;if k==0{return Ok(buffer[..n].to_vec())}n+=k;
 }
}
pub fn verificar(r:&Referencia<'_>,recuperado:&[u8],caso:&[u8],vigencia:&[u8])->Result<(),FalloDestino>{
 let seleccion=Seleccion{lectura:PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:r.identidad(),texto:recuperado},caso:Some(caso),vigencia:Some(vigencia)};
 r.comprobar(seleccion).map(|_|()).map_err(FalloDestino::Cobertura)
}
