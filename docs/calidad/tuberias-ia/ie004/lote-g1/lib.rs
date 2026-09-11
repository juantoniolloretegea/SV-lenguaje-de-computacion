#![forbid(unsafe_code)]
#![allow(dead_code)]
//! Enlace experimental del lote público fijado con G1. No inscribe SUCESO ni Frame.
extern crate g1;
include!("../recepcion-av/json_estricto.rs");
include!("../recepcion-av/sha256.rs");
include!("../semantica-a/recursos.rs");
include!("../lote-p3/adaptador.rs");

pub const VERSION_ENLACE:&str="IE004-LOTE-G1-PUBLICO/1";
pub const LOTE_FIJADO:&str="88bda419f98d75c10be7cf91acc849e852904495a9e6deb4403a8429a1c2990a";
pub const MONTAJE_FIJADO:&str="92750c47b491139f3abe4c007dcad9298346449bf1be9cd1779007278bae9eff";
const AGREGADO:u64=96*1024*1024;
const MARGEN:u64=20*1024*1024;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ErrorEnlace{IdentidadLote,IdentidadMontaje,Limite,Integridad,Correspondencia,Esquema,Orden,Incompleto,Receptor(g1::Fallo)}
type ER<T>=Result<T,ErrorEnlace>;
impl From<FalloT> for ErrorEnlace{fn from(_:FalloT)->Self{Self::Esquema}}
impl From<g1::Fallo> for ErrorEnlace{fn from(e:g1::Fallo)->Self{Self::Receptor(e)}}
struct Posicion{solicitud:Vec<u8>,inicio:usize,fin:usize,vigente:bool,manejador:Option<g1::Manejador>,identidad:Option<(u64,u64)>}
pub struct EnlacePublico{original:Vec<u8>,montaje:Vec<u8>,marco_lote:Vec<u8>,traza_lote:Vec<u8>,posiciones:[Option<Posicion>;24],custodios:[g1::Custodio;6],siguiente:usize,detenido:bool,preparacion:(u64,u64),cuenta_enlace:(u64,u64)}
fn frontera(actual:u64)->ER<()>{if actual.checked_add(MARGEN).ok_or(ErrorEnlace::Limite)?>AGREGADO{Err(ErrorEnlace::Limite)}else{Ok(())}}
fn copiar(b:&[u8],max:usize,c:&mut CuentaT)->RT<Vec<u8>>{if b.len()>max{return Err(FalloT::Bytes)}let mut v=Vec::new();c.reservar(&mut v,b.len())?;v.extend_from_slice(b);Ok(v)}
fn huella(b:&[u8])->RT<String>{Ok(sha_str(&sha_texto(b)?)?.to_owned())}
fn fijados(b:&[u8],m:&[u8])->ER<()>{
 if b.len()>MAX_LOTE||m.len()>16384{return Err(ErrorEnlace::Limite)}
 if huella(b)?!=LOTE_FIJADO{return Err(ErrorEnlace::IdentidadLote)}
 if huella(m)?!=MONTAJE_FIJADO{return Err(ErrorEnlace::IdentidadMontaje)}Ok(())
}
fn payload(m:&[u8])->ER<&[u8]>{
 if m.len()<48||&m[..8]!=b"SVLT0001"{return Err(ErrorEnlace::Integridad)}
 let n=u64::from_le_bytes(m[8..16].try_into().map_err(|_|ErrorEnlace::Integridad)?);
 if n>MAX_SALIDA as u64||n.checked_add(48)!=Some(m.len()as u64){return Err(ErrorEnlace::Integridad)}
 let b=&m[16..m.len()-32];if sha256(b)?.as_slice()!=&m[m.len()-32..]{return Err(ErrorEnlace::Integridad)}Ok(b)
}
fn verificar_productos(original:&[u8],montaje:&[u8],marco:&[u8],traza:&[u8],c:&mut CuentaT)->ER<[Option<Posicion>;24]>{
 if traza.len()>65536{return Err(ErrorEnlace::Limite)}
 let body=payload(marco)?;
 let lote=Decoder{b:original,i:0,c}.lote()?;
 let mt=decodificar(montaje,c)?;let mo=objeto(&mt,&["version","fuente","fuente_sha256","fuente_vigencia","fuente_vigencia_sha256","casos"],c)?;
 let ms=lista(campo(mo,"casos",c)?)?;if ms.len()!=24{return Err(ErrorEnlace::Correspondencia)}
 let tj=decodificar(traza,c)?;let to=objeto(&tj,&["version","solicitudes_sha256","casos","bytes_lote","trabajo_lote","reserva_acumulada_bytes","a_iniciada","v_abierta","auxiliar_abierto"],c)?;
 if texto(campo(to,"version",c)?)?!="IE004-LOTE-TRAZA/1"||texto(campo(to,"solicitudes_sha256",c)?)?!=LOTE_FIJADO||numero(campo(to,"bytes_lote",c)?)?!=original.len()as u64{return Err(ErrorEnlace::Correspondencia)}
 for k in ["a_iniciada","v_abierta","auxiliar_abierto"]{if !matches!(campo(to,k,c)?,J::B(false)){return Err(ErrorEnlace::Correspondencia)}}
 if numero(campo(to,"trabajo_lote",c)?)?>24_000_000||numero(campo(to,"reserva_acumulada_bytes",c)?)?>16*1024*1024{return Err(ErrorEnlace::Limite)}
 let ts=lista(campo(to,"casos",c)?)?;if ts.len()!=24{return Err(ErrorEnlace::Correspondencia)}
 let mut prefix=salida(256,c)?;prefix.raw("{\"version\":\"IE004-LOTE-EXTRAIDO/1\",\"solicitudes_sha256\":")?;prefix.str(LOTE_FIJADO)?;prefix.raw(",\"solicitudes\":[")?;
 if !body.starts_with(&prefix.b){return Err(ErrorEnlace::Correspondencia)}let mut offset=prefix.b.len();
 let mut posiciones:[Option<Posicion>;24]=std::array::from_fn(|_|None);
 for (i,s) in lote.casos.iter().enumerate(){
  if i>0{if body.get(offset)!=Some(&b','){return Err(ErrorEnlace::Correspondencia)}offset+=1;}
  let req=solicitud_canonica(s,c)?;let end=offset.checked_add(req.b.len()).ok_or(ErrorEnlace::Limite)?;
  if body.get(offset..end)!=Some(req.b.as_slice()){return Err(ErrorEnlace::Correspondencia)}offset=end;
  let m=objeto(&ms[i],&["id","fuente","vigente"],c)?;if texto(campo(m,"id",c)?)?!=s.id{return Err(ErrorEnlace::Correspondencia)}
  let vigente=match campo(m,"vigente",c)?{J::B(v)=>*v,_=>return Err(ErrorEnlace::Esquema)};
  let t=objeto(&ts[i],&["id","inicio","fin","tokens","original_caso_sha256","pregunta_sha256","solicitud_sha256"],c)?;
  if texto(campo(t,"id",c)?)?!=s.id||numero(campo(t,"inicio",c)?)?!=s.inicio as u64||numero(campo(t,"fin",c)?)?!=s.fin as u64||numero(campo(t,"tokens",c)?)?!=s.tokens as u64{return Err(ErrorEnlace::Correspondencia)}
  for(k,b)in[("original_caso_sha256",&original[s.inicio..s.fin]),("pregunta_sha256",s.pregunta.as_bytes()),("solicitud_sha256",req.b.as_slice())]{c.cobrar(b.len()as u64)?;if texto(campo(t,k,c)?)?!=huella(b)?{return Err(ErrorEnlace::Correspondencia)}}
  posiciones[i]=Some(Posicion{solicitud:req.b,inicio:s.inicio,fin:s.fin,vigente,manejador:None,identidad:None});
 }
 if body.get(offset..)!=Some(b"]}\n"){return Err(ErrorEnlace::Correspondencia)}Ok(posiciones)
}
impl EnlacePublico{
 pub fn preparar(original:&[u8],montaje:&[u8],realizacion:g1::Realizacion)->ER<Self>{
  fijados(original,montaje)?;let mut c=cuenta_lote();let (marco,traza)=adaptar(original,LOTE_FIJADO,&mut c)?;
  Self::con_productos(original,montaje,realizacion,marco,traza.b,(c.trabajo,c.reservado))
 }
 fn con_productos(original:&[u8],montaje:&[u8],r:g1::Realizacion,marco:Vec<u8>,traza:Vec<u8>,prep:(u64,u64))->ER<Self>{
  fijados(original,montaje)?;let mut c=cuenta_lote();let posiciones=verificar_productos(original,montaje,&marco,&traza,&mut c)?;
  let b=copiar(original,MAX_LOTE,&mut c)?;let m=copiar(montaje,16384,&mut c)?;
  // Seis custodios, cuatro slots cada uno, sin ampliación ni descarte automático.
  let custodios=[g1::Custodio::nuevo(r)?,g1::Custodio::nuevo(r)?,g1::Custodio::nuevo(r)?,g1::Custodio::nuevo(r)?,g1::Custodio::nuevo(r)?,g1::Custodio::nuevo(r)?];
  Ok(Self{original:b,montaje:m,marco_lote:marco,traza_lote:traza,posiciones,custodios,siguiente:0,detenido:false,preparacion:prep,cuenta_enlace:(c.trabajo,c.reservado)})
 }
 pub fn reservas_g1(&self)->ER<u64>{self.custodios.iter().try_fold(0u64,|sum,c|sum.checked_add(c.reservas()).ok_or(ErrorEnlace::Limite))}
 pub fn cuentas(&self)->((u64,u64),(u64,u64)){(self.preparacion,self.cuenta_enlace)}
 pub fn originales(&self)->(&[u8],&[u8]){(&self.original,&self.montaje)}
 pub fn productos_adaptador(&self)->(&[u8],&[u8]){(&self.marco_lote,&self.traza_lote)}
 pub fn ejecutar(&mut self,i:usize)->ER<()>{
  if self.detenido||i>=24||i!=self.siguiente{return Err(ErrorEnlace::Orden)}
  frontera(self.reservas_g1()?)?;
  let p=self.posiciones[i].as_mut().ok_or(ErrorEnlace::Incompleto)?;let c=&mut self.custodios[i/4];
  let h=c.abrir(&mut p.solicitud.as_slice(),p.vigente)?;p.identidad=Some(c.identidad(&h)?);p.manejador=Some(h);
  let r=c.ejecutar_a(p.manejador.as_ref().unwrap());
  if let Err(e)=r{self.detenido=true;return Err(e.into())}
  if let Err(e)=c.no_solicitar_v(p.manejador.as_ref().unwrap()){self.detenido=true;return Err(e.into())}
  if self.reservas_g1()?>AGREGADO{self.detenido=true;return Err(ErrorEnlace::Limite)}
  self.siguiente+=1;Ok(())
 }
 fn pareja(&self,i:usize)->ER<(&Posicion,&g1::Custodio,&g1::Manejador)>{
  let p=self.posiciones.get(i).and_then(Option::as_ref).ok_or(ErrorEnlace::Orden)?;let c=&self.custodios[i/4];let h=p.manejador.as_ref().ok_or(ErrorEnlace::Incompleto)?;
  if Some(c.identidad(h)?)!=p.identidad{return Err(ErrorEnlace::Correspondencia)}
  if c.recuperar(h,g1::Papel::Entrada)?!=p.solicitud{return Err(ErrorEnlace::Correspondencia)}Ok((p,c,h))
 }
 pub fn recuperar(&self,i:usize,papel:g1::Papel)->ER<&[u8]>{let(_,c,h)=self.pareja(i)?;Ok(c.recuperar(h,papel)?)}
 pub fn cuerpo(&self,i:usize)->ER<&[u8]>{let(_,c,h)=self.pareja(i)?;let m=c.recuperar(h,g1::Papel::Marco)?;if m.len()<48{return Err(ErrorEnlace::Integridad)}c.recibo(h)?;Ok(&m[16..m.len()-32])}
 pub fn identidad(&self,i:usize)->ER<(u64,u64)>{let(_,c,h)=self.pareja(i)?;Ok(c.identidad(h)?)}
 pub fn montaje(&self,i:usize)->ER<bool>{Ok(self.posiciones.get(i).and_then(Option::as_ref).ok_or(ErrorEnlace::Orden)?.vigente)}
 pub fn rango(&self,i:usize)->ER<(usize,usize)>{let p=self.posiciones.get(i).and_then(Option::as_ref).ok_or(ErrorEnlace::Orden)?;Ok((p.inicio,p.fin))}
 pub fn completo(&self)->ER<()>{if self.detenido||self.siguiente!=24{return Err(ErrorEnlace::Incompleto)}for i in 0..24{let(_,c,h)=self.pareja(i)?;c.recibo(h)?;if c.estado(h)?.1!=g1::EstadoV::NoSolicitada{return Err(ErrorEnlace::Incompleto)}}Ok(())}
}
#[cfg(test)]mod tests;
