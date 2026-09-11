#![forbid(unsafe_code)]
//! Candidata G1/1: custodia intra-proceso. No emite autoridad ni Frame.
mod av;
mod json_vista;
use json_vista::Vista;
use std::io::Read;
use std::sync::atomic::{AtomicU64,Ordering};
const MAX_A:usize=65536;
const MAX_TRACE:usize=16842752;
const MAX_V:usize=262144;
const MAX_MEMORY:u64=67108864;
const MAX_WORK:u64=1000000000;
static AMBITOS:AtomicU64=AtomicU64::new(1);
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Fallo{Limite,Reserva,Trabajo,Profundidad,Json,Esquema,Integridad,Correlacion,Discordancia,Montaje,Pertenencia,Duplicado,Estado,Recuperacion,Productor,Interrupcion,Io,Ordinal}
type R<T>=Result<T,Fallo>;
struct Trabajo{usado:u64,max:u64}
impl Trabajo{fn nuevo()->Self{Self{usado:0,max:MAX_WORK}}fn cobrar(&mut self,n:u64)->R<()>{let x=self.usado.checked_add(n).ok_or(Fallo::Trabajo)?;if x>self.max{return Err(Fallo::Trabajo)}self.usado=x;Ok(())}}
#[derive(Debug)]
pub struct Manejador{ambito:u64,ordinal:u64,slot:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum EstadoA{Abierta,Completa,Incompleta(Fallo)}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum EstadoV{SinDecision,Abierta,Recuperable,Incompleta(Fallo),NoSolicitada}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Papel{Entrada,Marco,Traza,Propuesta,Anexo}
struct Archivo{b:Option<Vec<u8>>,hash:[u8;32],len:usize,completo:bool}
impl Archivo{fn recuperar(&self)->R<&[u8]>{let b=self.b.as_deref().ok_or(Fallo::Recuperacion)?;if b.len()!=self.len||av::hash(b)?!=self.hash{return Err(Fallo::Recuperacion)}Ok(b)}}
struct Entrada{ordinal:u64,vigente:bool,original:Archivo,a:EstadoA,v:EstadoV,marco:Option<Archivo>,traza:Option<Archivo>,propuesta:Option<Archivo>,anexo:Option<Archivo>,lote:Option<[u8;64]>,trabajo_a:u64,trabajo_v:u64}
/// Recibo inmutable visto a través de su custodio; no hay constructor público.
pub struct Recibo<'a>{entrada:&'a Entrada}
impl Recibo<'_>{pub fn cuerpo(&self)->R<&[u8]>{av::frame(self.entrada.marco.as_ref().ok_or(Fallo::Estado)?.recuperar()?)}pub fn estado_a(&self)->EstadoA{self.entrada.a}pub fn estado_v(&self)->EstadoV{self.entrada.v}pub fn trabajo(&self)->(u64,u64){(self.entrada.trabajo_a,self.entrada.trabajo_v)}}
#[derive(Clone,Copy,Debug)]
pub struct Realizacion { pub fuentes_sha256:[u8;32], pub binario_sha256:[u8;32] }
pub struct Custodio{ambito:u64,siguiente:u64,slots:[Option<Entrada>;4],reservado:u64,realizacion:Realizacion,#[cfg(test)]perder_traza:bool}
impl Custodio{
 pub fn nuevo(realizacion:Realizacion)->R<Self>{let a=AMBITOS.fetch_update(Ordering::SeqCst,Ordering::SeqCst,|x|x.checked_add(1)).map_err(|_|Fallo::Ordinal)?;Ok(Self{ambito:a,siguiente:1,slots:std::array::from_fn(|_|None),reservado:0,realizacion,#[cfg(test)]perder_traza:false})}
 fn indice(&self,h:&Manejador)->R<usize>{if h.ambito!=self.ambito{return Err(Fallo::Pertenencia)}let s=self.slots.get(h.slot).and_then(Option::as_ref).ok_or(Fallo::Pertenencia)?;if s.ordinal!=h.ordinal{return Err(Fallo::Pertenencia)}Ok(h.slot)}
 fn cargo(&mut self,n:u64)->R<()>{let total=self.reservado.checked_add(n).ok_or(Fallo::Limite)?;if total>MAX_MEMORY{return Err(Fallo::Limite)}self.reservado=total;Ok(())}
 fn archivo(&mut self,b:Vec<u8>,max:usize,completo:bool)->R<Archivo>{if b.len()>max{return Err(Fallo::Limite)}self.cargo(b.capacity()as u64)?;Ok(Archivo{hash:av::hash(&b)?,len:b.len(),b:Some(b),completo})}
 // Entrada acotada con un byte de detección. El Read y la política de espera
 // pertenecen al conductor confiable. Una lectura bloqueada requiere supervisor.
 pub fn abrir<Rd:Read>(&mut self,input:&mut Rd,vigente:bool)->R<Manejador>{
  let slot=self.slots.iter().position(Option::is_none).ok_or(Fallo::Limite)?;
  let next=self.siguiente.checked_add(1).ok_or(Fallo::Ordinal)?;
  if self.reservado.checked_add(MAX_A as u64).ok_or(Fallo::Limite)?>MAX_MEMORY{return Err(Fallo::Limite)}
  let mut b=Vec::new();b.try_reserve_exact(MAX_A).map_err(|_|Fallo::Reserva)?;
  let mut complete=true;let mut error=None;let mut scratch=[0u8;4096];
  loop{let nmax=scratch.len().min(MAX_A-b.len()+1);match input.read(&mut scratch[..nmax]){
   Ok(0)=>break,Ok(n)=>{let keep=n.min(MAX_A-b.len());b.extend_from_slice(&scratch[..keep]);if keep<n{complete=false;error=Some(Fallo::Limite);break}},Err(_)=>{complete=false;error=Some(Fallo::Io);break}}}
  let original=self.archivo(b,MAX_A,complete)?;let ordinal=self.siguiente;self.siguiente=next;
  self.slots[slot]=Some(Entrada{ordinal,vigente,original,a:error.map_or(EstadoA::Abierta,EstadoA::Incompleta),v:EstadoV::SinDecision,marco:None,traza:None,propuesta:None,anexo:None,lote:None,trabajo_a:0,trabajo_v:0});Ok(Manejador{ambito:self.ambito,ordinal,slot})
 }
 pub fn ejecutar_a(&mut self,h:&Manejador)->R<()>{let i=self.indice(h)?;let s=self.slots[i].as_ref().ok_or(Fallo::Estado)?;if s.a!=EstadoA::Abierta{return Err(Fallo::Duplicado)}
  let product=av::a(s.original.recuperar()?,s.vigente);
  match product{Ok((marco,traza))=>self.cerrar_a(h,marco,traza),Err(e)=>{self.slots[i].as_mut().unwrap().a=EstadoA::Incompleta(e);Err(e)}}
 }
 fn cerrar_a(&mut self,h:&Manejador,marco:Vec<u8>,traza:Vec<u8>)->R<()>{let i=self.indice(h)?;if self.slots[i].as_ref().unwrap().a!=EstadoA::Abierta{return Err(Fallo::Duplicado)}
  let mut work=Trabajo::nuevo();
  let r=(||{let m=self.archivo(marco,4144,true)?;self.slots[i].as_mut().unwrap().marco=Some(m);let t=self.archivo(traza,MAX_TRACE,true)?;self.slots[i].as_mut().unwrap().traza=Some(t);
   #[cfg(test)]if self.perder_traza{self.slots[i].as_mut().unwrap().traza.as_mut().unwrap().b=None;}
   self.cargo(2*1024*1024)?;validar_a(self.slots[i].as_ref().unwrap(),&mut work)?;recuperar_a(self.slots[i].as_ref().unwrap())?;Ok(())})();
  let s=self.slots[i].as_mut().unwrap();s.trabajo_a=work.usado;s.a=match r{Ok(())=>EstadoA::Completa,Err(e)=>EstadoA::Incompleta(e)};r
 }
 pub fn recibo(&self,h:&Manejador)->R<Recibo<'_>>{let s=self.slots[self.indice(h)?].as_ref().unwrap();if s.a!=EstadoA::Completa{return Err(Fallo::Estado)}recuperar_a(s)?;Ok(Recibo{entrada:s})}
 pub fn estado(&self,h:&Manejador)->R<(EstadoA,EstadoV)>{let s=self.slots[self.indice(h)?].as_ref().unwrap();Ok((s.a,s.v))}
 pub fn recuperar(&self,h:&Manejador,p:Papel)->R<&[u8]>{let s=self.slots[self.indice(h)?].as_ref().unwrap();let a=match p{Papel::Entrada=>Some(&s.original),Papel::Marco=>s.marco.as_ref(),Papel::Traza=>s.traza.as_ref(),Papel::Propuesta=>s.propuesta.as_ref(),Papel::Anexo=>s.anexo.as_ref()}.ok_or(Fallo::Recuperacion)?;a.recuperar()}
 pub fn realizacion(&self)->Realizacion{self.realizacion}
 pub fn identidad(&self,h:&Manejador)->R<(u64,u64)>{self.indice(h)?;Ok((h.ambito,h.ordinal))}
 pub fn reservas(&self)->u64{self.reservado}
 pub fn iniciar_v(&mut self,h:&Manejador,lote:[u8;64])->R<()>{let i=self.indice(h)?;self.recibo(h)?;if !lote.iter().all(|x|x.is_ascii_digit()||(b'a'..=b'f').contains(x)){return Err(Fallo::Esquema)}let s=self.slots[i].as_mut().unwrap();if s.v!=EstadoV::SinDecision{return Err(Fallo::Duplicado)}s.lote=Some(lote);s.v=EstadoV::Abierta;Ok(())}
 pub fn no_solicitar_v(&mut self,h:&Manejador)->R<()>{let i=self.indice(h)?;self.recibo(h)?;let s=self.slots[i].as_mut().unwrap();if s.v!=EstadoV::SinDecision{return Err(Fallo::Duplicado)}s.v=EstadoV::NoSolicitada;Ok(())}
 pub fn interrumpir_v(&mut self,h:&Manejador)->R<()>{let i=self.indice(h)?;let s=self.slots[i].as_mut().unwrap();if s.v!=EstadoV::Abierta{return Err(Fallo::Estado)}s.v=EstadoV::Incompleta(Fallo::Interrupcion);Ok(())}
 pub fn ejecutar_v<Rd:Read>(&mut self,h:&Manejador,input:&mut Rd)->R<()>{let i=self.indice(h)?;if self.slots[i].as_ref().unwrap().v!=EstadoV::Abierta{return Err(Fallo::Estado)}
  let r=(||{if self.reservado.checked_add(MAX_V as u64).ok_or(Fallo::Limite)?>MAX_MEMORY{return Err(Fallo::Limite)}let mut b=Vec::new();b.try_reserve_exact(MAX_V).map_err(|_|Fallo::Reserva)?;let mut scratch=[0u8;4096];let mut complete=true;let mut read_error=None;
   loop{let nmax=scratch.len().min(MAX_V-b.len()+1);let n=match input.read(&mut scratch[..nmax]){Ok(n)=>n,Err(_)=>{complete=false;read_error=Some(Fallo::Io);break}};if n==0{break}let k=n.min(MAX_V-b.len());b.extend_from_slice(&scratch[..k]);if k<n{complete=false;break}}
   let a=self.archivo(b,MAX_V,complete)?;self.slots[i].as_mut().unwrap().propuesta=Some(a);if !complete{return Err(read_error.unwrap_or(Fallo::Limite))}
   let s=self.slots[i].as_ref().unwrap();let body=av::hex(&av::hash(av::frame(s.marco.as_ref().ok_or(Fallo::Estado)?.recuperar()?)?)?);let lote=s.lote.ok_or(Fallo::Estado)?;
   let annex=av::v(s.original.recuperar()?,s.propuesta.as_ref().unwrap().recuperar()?,std::str::from_utf8(&lote).map_err(|_|Fallo::Esquema)?,std::str::from_utf8(&body).map_err(|_|Fallo::Esquema)?)?;self.cerrar_v(h,annex)})();
  if let Err(e)=r{let s=self.slots[i].as_mut().unwrap();if s.v==EstadoV::Abierta{s.v=EstadoV::Incompleta(e)}}r
 }
 fn cerrar_v(&mut self,h:&Manejador,annex:Vec<u8>)->R<()>{let i=self.indice(h)?;if self.slots[i].as_ref().unwrap().a!=EstadoA::Completa{return Err(Fallo::Estado)}if self.slots[i].as_ref().unwrap().v!=EstadoV::Abierta{return Err(Fallo::Duplicado)}let mut work=Trabajo::nuevo();
  let r=(||{self.cargo(2*1024*1024)?;let a=self.archivo(annex,8192,true)?;self.slots[i].as_mut().unwrap().anexo=Some(a);let s=self.slots[i].as_ref().unwrap();recuperar_a(s)?;let p=s.propuesta.as_ref().ok_or(Fallo::Recuperacion)?;if !p.completo{return Err(Fallo::Recuperacion)}p.recuperar()?;validar_v(s,&mut work)})();let s=self.slots[i].as_mut().unwrap();s.trabajo_v=work.usado;s.v=match r{Ok(())=>EstadoV::Recuperable,Err(e)=>EstadoV::Incompleta(e)};r
 }
}
fn recuperar_a(s:&Entrada)->R<()>{if !s.original.completo{return Err(Fallo::Recuperacion)}s.original.recuperar()?;s.marco.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?;s.traza.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?;Ok(())}
fn igual(a:&[u8],b:&[u8],c:&mut Trabajo,err:Fallo)->R<()>{c.cobrar(a.len().max(b.len())as u64)?;if a!=b{return Err(err)}Ok(())}
fn cadena(a:&[u8],s:&[u8],c:&mut Trabajo,err:Fallo)->R<()>{if a.len()!=s.len()+2||a.first()!=Some(&b'"')||a.last()!=Some(&b'"'){return Err(err)}igual(&a[1..a.len()-1],s,c,err)}
const CUERPO_KEYS:[&str;10]=["version","montaje","perfil","base","politica","fuente_fijada_sha256","vigente","admision","contexto","resolucion"];
fn validar_a(s:&Entrada,c:&mut Trabajo)->R<()>{let wire=s.original.recuperar()?;let m=s.marco.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?;let t=s.traza.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?;
 c.cobrar((wire.len()+m.len()+t.len())as u64*3)?;let body=av::frame(m)?;let b=Vista::nueva(body,c)?;b.claves(&CUERPO_KEYS,c)?;let tr=Vista::nueva(t,c)?;
 cadena(b.campo("version",c)?,b"IE004-A-CUERPO/1",c,Fallo::Esquema)?;cadena(tr.campo("version",c)?,b"IE004-A-TRAZA/1",c,Fallo::Esquema)?;
 for (key,v)in ["montaje","perfil","base","politica","fuente_fijada_sha256"].into_iter().zip(av::versiones()){cadena(b.campo(key,c)?,v.as_bytes(),c,Fallo::Montaje)?;}
 igual(b.campo("vigente",c)?,if s.vigente{b"true"}else{b"false"},c,Fallo::Montaje)?;
 cadena(tr.campo("original_transporte_sha256",c)?,&av::hex(&av::hash(wire)?),c,Fallo::Correlacion)?;cadena(tr.campo("cuerpo_sha256",c)?,&av::hex(&av::hash(body)?),c,Fallo::Correlacion)?;
 validar_resolucion(b.campo("resolucion",c)?,c)?;
 let started=tr.campo("iniciado_a",c)?;
 if started==b"true"{
  tr.claves(&["version","id","original","original_transporte_sha256","cuerpo_sha256","iniciado_a","analisis_completo","admision_trabajo","admision_reserva_acumulada_bytes","unidades_a","categorias_a","capacidad_a_bytes","pico_solicitado_a_bytes","salida_reservada_bytes","resolucion","normalizado","tokens","significados"],c)?;
  for key in ["admision_trabajo","admision_reserva_acumulada_bytes","unidades_a","capacidad_a_bytes","pico_solicitado_a_bytes","salida_reservada_bytes"]{tipo(tr.campo(key,c)?,b'0',false)?;}
  tipo(tr.campo("categorias_a",c)?,b'[',false)?;tipo(tr.campo("normalizado",c)?,b'"',true)?;tipo(tr.campo("tokens",c)?,b'[',true)?;tipo(tr.campo("significados",c)?,b'[',true)?;
  validar_resolucion(tr.campo("resolucion",c)?,c)?;
  cadena(b.campo("admision",c)?,b"ADMITIDA",c,Fallo::Discordancia)?;let req=av::solicitud_canonica(wire)?;
  igual(tr.campo("id",c)?,&req.id,c,Fallo::Correlacion)?;igual(tr.campo("original",c)?,&req.original,c,Fallo::Correlacion)?;igual(b.campo("contexto",c)?,&req.contexto,c,Fallo::Correlacion)?;
  igual(tr.campo("resolucion",c)?,b.campo("resolucion",c)?,c,Fallo::Discordancia)?;
  let complete=tr.campo("analisis_completo",c)?;if complete!=b"true"&&complete!=b"false"{return Err(Fallo::Esquema)}if complete==b"false"&&tr.campo("significados",c)?!=b"null"{return Err(Fallo::Discordancia)}
 }else if started==b"false"{
  tr.claves(&["version","iniciado_a","error_transporte","recibidos","original_transporte_sha256","cuerpo_sha256"],c)?;
  tipo(tr.campo("recibidos",c)?,b'0',false)?;tipo(tr.campo("error_transporte",c)?,b'"',false)?;
  cadena(b.campo("admision",c)?,b"RECHAZADA",c,Fallo::Discordancia)?;if av::solicitud_canonica(wire).is_ok(){return Err(Fallo::Discordancia)}
  igual(b.campo("contexto",c)?,b"null",c,Fallo::Discordancia)?;
 }else{return Err(Fallo::Esquema)}Ok(())
}
fn validar_v(s:&Entrada,c:&mut Trabajo)->R<()>{let bytes=s.anexo.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?;let v=Vista::nueva(bytes,c)?;v.claves(&["version","estado","causa","id","cuerpo_a_sha256","solicitudes_sha256","bytes_v_observados","bytes_instantanea_a","unidades_v","reserva_acumulada_v_bytes","autoridad_sobre_cuerpo_a","prueba"],c)?;
 for key in ["bytes_v_observados","bytes_instantanea_a","unidades_v","reserva_acumulada_v_bytes"]{tipo(v.campo(key,c)?,b'0',false)?;}
 tipo(v.campo("estado",c)?,b'"',false)?;tipo(v.campo("causa",c)?,b'"',true)?;tipo(v.campo("prueba",c)?,b'{',true)?;
 cadena(v.campo("version",c)?,b"IE004-V-ANEXO/1",c,Fallo::Esquema)?;igual(v.campo("autoridad_sobre_cuerpo_a",c)?,b"false",c,Fallo::Discordancia)?;
 let wire=s.original.recuperar()?;let body=av::frame(s.marco.as_ref().ok_or(Fallo::Recuperacion)?.recuperar()?)?;
 cadena(v.campo("cuerpo_a_sha256",c)?,&av::hex(&av::hash(body)?),c,Fallo::Correlacion)?;cadena(v.campo("solicitudes_sha256",c)?,&s.lote.ok_or(Fallo::Estado)?,c,Fallo::Correlacion)?;
 if let Ok(req)=av::solicitud_canonica(wire){igual(v.campo("id",c)?,&req.id,c,Fallo::Correlacion)?;}else{igual(v.campo("id",c)?,b"null",c,Fallo::Correlacion)?;}Ok(())}
fn tipo(v:&[u8],t:u8,nulo:bool)->R<()>{if nulo&&v==b"null"{return Ok(())}let valid=if t==b'0'{v.first().is_some_and(u8::is_ascii_digit)}else{v.first()==Some(&t)};if valid{Ok(())}else{Err(Fallo::Esquema)}}
fn validar_resolucion(b:&[u8],c:&mut Trabajo)->R<()>{let r=Vista::nueva(b,c)?;r.claves(&["estado","contenido","ruta","causas","llamadas_politica","fuente","alcance"],c)?;tipo(r.campo("estado",c)?,b'"',false)?;for key in ["contenido","fuente","alcance"]{tipo(r.campo(key,c)?,b'"',true)?;}tipo(r.campo("ruta",c)?,b'[',false)?;for key in ["causas","llamadas_politica"]{tipo(r.campo(key,c)?,b'0',false)?;}Ok(())}
#[cfg(test)]mod tests;
