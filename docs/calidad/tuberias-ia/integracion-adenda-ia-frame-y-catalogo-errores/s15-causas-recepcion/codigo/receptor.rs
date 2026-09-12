#![forbid(unsafe_code)]
//! Protocolo sintético S15; no es SDK de proveedor ni IR SV.
use std::io::{Read,ErrorKind};
use cobertura::{Referencia,Seleccion,FalloCobertura};
use lote::lectura::{PropuestaDeLectura,VERSION,OPERACION};
pub const LIMITE:usize=16384;
#[derive(Debug,PartialEq,Eq)]
pub enum Esquema{Cabecera,Tipo,Truncado,Sobrante,Bandera,Limite}
#[derive(Debug,PartialEq,Eq)]
pub enum Causa{Entregada,NegativaProveedor,EsquemaInvalido(Esquema),NoAdmision(FalloCobertura),Comunicacion(ErrorKind)}
pub struct Informe{entrada:Vec<u8>,causa:Causa,llamadas:usize,cuerpo:Option<Vec<u8>>,motivo:Option<Vec<u8>>}
impl Informe{
 pub fn entrada(&self)->&[u8]{&self.entrada}
 pub fn cuerpo(&self)->Option<&[u8]>{self.cuerpo.as_deref()}
 pub fn motivo(&self)->Option<&[u8]>{self.motivo.as_deref()}
 // Única proyección usada por registro y presentación. Nunca traduce fallos a Tri.
 pub fn registro(&self)->String{
  let (cat,etapa)=match &self.causa{Causa::Entregada=>("ENTREGADA","cobertura"),Causa::NegativaProveedor=>("NEGATIVA_PROVEEDOR","protocolo"),Causa::EsquemaInvalido(Esquema::Limite)=>("ESQUEMA_INVALIDO","recepcion"),Causa::EsquemaInvalido(_)=>("ESQUEMA_INVALIDO","protocolo"),Causa::NoAdmision(_)=>("NO_ADMISION","cobertura"),Causa::Comunicacion(_)=>("COMUNICACION","recepcion")};
  #[cfg(unificar_u)] let cat=if matches!(self.causa,Causa::NoAdmision(_)){"U"}else{cat};
  format!("{cat}\t{etapa}\t{:?}\t{}\t{}\n",self.causa,self.cuerpo.is_some(),self.llamadas)
 }
}
struct Cursor<'a>{b:&'a[u8],p:usize}
impl<'a> Cursor<'a>{
 fn tomar(&mut self,n:usize)->Result<&'a[u8],Esquema>{let fin=self.p.checked_add(n).ok_or(Esquema::Truncado)?;let v=self.b.get(self.p..fin).ok_or(Esquema::Truncado)?;self.p=fin;Ok(v)}
 fn byte(&mut self)->Result<u8,Esquema>{Ok(self.tomar(1)?[0])}
 fn campo(&mut self)->Result<&'a[u8],Esquema>{let n=u32::from_be_bytes(self.tomar(4)?.try_into().unwrap()) as usize;self.tomar(n)}
 fn opcional(&mut self)->Result<Option<&'a[u8]>,Esquema>{match self.byte()?{0=>Ok(None),1=>Ok(Some(self.campo()?)),_=>Err(Esquema::Bandera)}}
}
enum Mensaje<'a>{Negativa(&'a[u8]),Respuesta{identidad:(u64,u64),cuerpo:&'a[u8],caso:Option<&'a[u8]>,vigencia:Option<&'a[u8]>}}
fn decodificar(b:&[u8])->Result<Mensaje<'_>,Esquema>{
 let mut c=Cursor{b,p:0};if c.tomar(5)?!=b"SV15\x01"{return Err(Esquema::Cabecera)}
 let m=match c.byte()?{
  b'N'=>Mensaje::Negativa(c.campo()?),
  b'R'=>{let a=u64::from_be_bytes(c.tomar(8)?.try_into().unwrap());let z=u64::from_be_bytes(c.tomar(8)?.try_into().unwrap());Mensaje::Respuesta{identidad:(a,z),cuerpo:c.campo()?,caso:c.opcional()?,vigencia:c.opcional()?}},
  _=>return Err(Esquema::Tipo)
 };if c.p!=b.len(){return Err(Esquema::Sobrante)}Ok(m)
}
pub fn recibir(r:&Referencia<'_>,lector:&mut impl Read)->Informe{
 let mut entrada=Vec::with_capacity(LIMITE+1);let mut fallo=None;
 loop{
  let mut bloque=[0u8;512];let nmax=bloque.len().min(LIMITE+1-entrada.len());
  match lector.read(&mut bloque[..nmax]){
   Ok(0)=>break,
   Ok(n)=>{entrada.extend_from_slice(&bloque[..n]);if entrada.len()>LIMITE{fallo=Some(Causa::EsquemaInvalido(Esquema::Limite));break}},
   Err(e)=>{
    #[cfg(ignorar_io_tardia)] if !entrada.is_empty(){break}
    fallo=Some(Causa::Comunicacion(e.kind()));break
   }
  }
 }
 let mut i=Informe{entrada,causa:Causa::Entregada,llamadas:0,cuerpo:None,motivo:None};
 if let Some(c)=fallo{i.causa=c;return i}
 match decodificar(&i.entrada){
  Err(e)=>i.causa=Causa::EsquemaInvalido(e),
  Ok(Mensaje::Negativa(m))=>{
   i.causa=Causa::NegativaProveedor;i.motivo=Some(m.to_vec());
   #[cfg(admitir_negativa)] {i.causa=Causa::Entregada;i.cuerpo=Some(m.to_vec());}
  },
  Ok(Mensaje::Respuesta{identidad,cuerpo,caso,vigencia})=>{
   i.llamadas+=1;
   match r.comprobar(Seleccion{lectura:PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad,texto:cuerpo},caso,vigencia}){
    Ok(e)=>{i.causa=Causa::Entregada;i.cuerpo=Some(e.lectura().texto().to_vec())},
    Err(e)=>i.causa=Causa::NoAdmision(e)
   }
  }
 }i
}
