#![forbid(unsafe_code)]
//! Ensayo G/J: archivo de vigencia acotado, mismo custodio y dos actos locales.
use std::{io::Read,path::Path};
pub const NOMBRE:&str="vigencia.s14";
const MAX:usize=64;
#[derive(Debug,PartialEq,Eq)]
pub enum Fallo { Io(std::io::ErrorKind), LimiteBase, BaseDistinta, FormatoBase, BaseSinCambio, Capacidad, SolicitudDistinta, Identidad, ReciboDistinto, G1(g1::Fallo) }
impl From<std::io::Error> for Fallo{fn from(e:std::io::Error)->Self{Self::Io(e.kind())}}
impl From<g1::Fallo> for Fallo{fn from(e:g1::Fallo)->Self{Self::G1(e)}}
pub struct Base{bytes:[u8;MAX],len:usize}
impl Base{
 pub fn cargar(path:&Path,requerida:&[u8])->Result<Self,Fallo>{
  let mut f=std::fs::File::open(path)?;let mut b=[0u8;MAX+1];let mut n=0;
  loop{if n==b.len(){return Err(Fallo::LimiteBase)}let k=f.read(&mut b[n..])?;if k==0{break}n+=k;}
  if requerida.len()>MAX{return Err(Fallo::LimiteBase)}
  if !cfg!(sin_identidad_carga)&&&b[..n]!=requerida{return Err(Fallo::BaseDistinta)}
  Self::interpretar(&b[..n])?;
  let mut bytes=[0u8;MAX];bytes[..n].copy_from_slice(&b[..n]);Ok(Self{bytes,len:n})
 }
 fn interpretar(b:&[u8])->Result<bool,Fallo>{match b{b"SV-S14-VIGENCIA/1\nvigente=1\n"=>Ok(true),b"SV-S14-VIGENCIA/1\nvigente=0\n"=>Ok(false),_=>Err(Fallo::FormatoBase)}}
 pub fn bytes(&self)->&[u8]{&self.bytes[..self.len]}
 fn vigente(&self)->Result<bool,Fallo>{if cfg!(consumo_falso){Ok(true)}else{Self::interpretar(self.bytes())}}
}
struct Episodio{manejador:g1::Manejador,base:Base,padre:Option<(u64,u64)>}
pub struct Historia{custodio:g1::Custodio,original:Episodio,nueva:Option<Episodio>}
pub struct Vista<'a>{pub identidad:(u64,u64),pub padre:Option<(u64,u64)>,pub base:&'a[u8],pub cuerpo:&'a[u8],pub solicitud:&'a[u8],pub traza:&'a[u8],pub marco:&'a[u8]}
impl Historia{
 pub fn crear(base:Base,solicitud:&[u8],realizacion:g1::Realizacion)->Result<Self,Fallo>{
  let mut custodio=g1::Custodio::nuevo(realizacion)?;let mut entrada=solicitud;let h=custodio.abrir(&mut entrada,base.vigente()?)?;custodio.ejecutar_a(&h)?;custodio.no_solicitar_v(&h)?;
  Ok(Self{custodio,original:Episodio{manejador:h,base,padre:None},nueva:None})
 }
 pub fn reevaluar(&mut self,base:Base,solicitud:&[u8])->Result<(),Fallo>{
  if self.nueva.is_some(){return Err(Fallo::Capacidad)}
  if base.bytes()==self.original.base.bytes(){return Err(Fallo::BaseSinCambio)}
  if solicitud!=self.custodio.recuperar(&self.original.manejador,g1::Papel::Entrada)?{return Err(Fallo::SolicitudDistinta)}
  let padre=self.custodio.identidad(&self.original.manejador)?;
  let mut entrada=solicitud;let h=self.custodio.abrir(&mut entrada,base.vigente()?)?;
  self.custodio.ejecutar_a(&h)?;self.custodio.no_solicitar_v(&h)?;
  self.nueva=Some(Episodio{manejador:h,base,padre:Some(padre)});Ok(())
 }
 fn vista<'a>(&'a self,e:&'a Episodio)->Result<Vista<'a>,Fallo>{
  let marco=self.custodio.recuperar(&e.manejador,g1::Papel::Marco)?;
  self.custodio.recibo(&e.manejador)?;
  if marco.len()<48{return Err(Fallo::ReciboDistinto)}
  Ok(Vista{identidad:self.custodio.identidad(&e.manejador)?,padre:e.padre,base:e.base.bytes(),cuerpo:&marco[16..marco.len()-32],solicitud:self.custodio.recuperar(&e.manejador,g1::Papel::Entrada)?,traza:self.custodio.recuperar(&e.manejador,g1::Papel::Traza)?,marco})
 }
 pub fn original(&self)->Result<Vista<'_>,Fallo>{self.vista(&self.original)}
 pub fn nueva(&self)->Result<Vista<'_>,Fallo>{self.vista(self.nueva.as_ref().ok_or(Fallo::Capacidad)?)}
 pub fn atribuir_original(&self,identidad:(u64,u64),base:&[u8],cuerpo:&[u8])->Result<Vista<'_>,Fallo>{
  let v=self.original()?;
  if identidad!=v.identidad{return Err(Fallo::Identidad)}
  if !cfg!(sin_base_original)&&base!=v.base{return Err(Fallo::BaseDistinta)}
  if cuerpo!=v.cuerpo{return Err(Fallo::ReciboDistinto)}
  Ok(v)
 }
}
