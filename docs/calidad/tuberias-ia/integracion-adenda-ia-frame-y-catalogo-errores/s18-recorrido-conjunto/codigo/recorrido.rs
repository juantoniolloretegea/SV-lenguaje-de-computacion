#![forbid(unsafe_code)]
//! Realización de laboratorio de S17. Ningún tipo aquí concede autoridad de actuación.
use std::{io::{Read,Write,ErrorKind},path::Path};
pub const MAX_CONTEXTO:usize=8192;
pub const MAX_RECEPCION:usize=16384;
#[derive(Debug,PartialEq,Eq)]
pub enum Causa{Conforme,BaseDistinta,FormatoBase,LimiteBase,DocumentoDistinto,Identidad,FaltaSolicitud,SolicitudDistinta,FaltaBase,Negativa,Esquema(Esquema),Comunicacion(ErrorKind),Io(ErrorKind),LimiteContexto,LimiteRecepcion,Presentacion(g1::FalloPresentacion),G1(g1::Fallo),EntregaG1(g1::FalloEntrega),Capacidad,BaseSinCambio}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Esquema{Cabecera,Tipo,Truncado,Sobrante,Bandera}
#[derive(Debug,PartialEq,Eq)]
pub struct Diagnostico{pub etapa:&'static str,pub causa:Causa}
impl Diagnostico{
 fn nuevo(etapa:&'static str,causa:Causa)->Self{Self{etapa,causa}}
 pub fn codigo(&self)->&'static str{match &self.causa{
  Causa::Conforme=>"CONFORME",Causa::BaseDistinta=>"BASE_DISTINTA",Causa::DocumentoDistinto=>"DOCUMENTO_DISTINTO",Causa::Identidad=>"IDENTIDAD",Causa::FaltaSolicitud=>"FALTA_SOLICITUD",Causa::SolicitudDistinta=>"SOLICITUD_DISTINTA",Causa::FaltaBase=>"FALTA_BASE",Causa::Negativa=>"NEGATIVA_PROVEEDOR",Causa::Esquema(_)=>"ESQUEMA_INVALIDO",Causa::Comunicacion(_)=>"COMUNICACION",Causa::Io(_)=>"IO",Causa::LimiteContexto=>"LIMITE_CONTEXTO",Causa::LimiteRecepcion=>"LIMITE_RECEPCION",Causa::Presentacion(g1::FalloPresentacion::ContenidoDistinto)=>"CONTENIDO_DISTINTO",Causa::Presentacion(_)=>"PRESENTACION",Causa::G1(_)=>"G1",Causa::EntregaG1(_)=>"ENTREGA_G1",Causa::FormatoBase=>"FORMATO_BASE",Causa::LimiteBase=>"LIMITE_BASE",Causa::Capacidad=>"CAPACIDAD",Causa::BaseSinCambio=>"BASE_SIN_CAMBIO"}}
 pub fn rotulo(&self,ingles:bool)->&'static str{match (&self.causa,ingles){
  (Causa::Conforme,false)=>"Conforme",(Causa::Conforme,true)=>"Conforming",(Causa::BaseDistinta,false)=>"Base distinta",(Causa::BaseDistinta,true)=>"Different base",(Causa::DocumentoDistinto,false)=>"Documento distinto",(Causa::DocumentoDistinto,true)=>"Different document",(Causa::Identidad,false)=>"Identidad distinta",(Causa::Identidad,true)=>"Identity mismatch",(Causa::FaltaSolicitud,false)=>"Falta solicitud",(Causa::FaltaSolicitud,true)=>"Missing request",(Causa::SolicitudDistinta,false)=>"Solicitud distinta",(Causa::SolicitudDistinta,true)=>"Different request",(Causa::FaltaBase,false)=>"Falta base",(Causa::FaltaBase,true)=>"Missing base",(Causa::Presentacion(g1::FalloPresentacion::ContenidoDistinto),false)=>"Contenido distinto",(Causa::Presentacion(g1::FalloPresentacion::ContenidoDistinto),true)=>"Different content",(Causa::Negativa,false)=>"Negativa del proveedor",(Causa::Negativa,true)=>"Provider refusal",(Causa::Esquema(_),false)=>"Esquema inválido",(Causa::Esquema(_),true)=>"Invalid schema",(Causa::Comunicacion(_),false)=>"Fallo de comunicación",(Causa::Comunicacion(_),true)=>"Communication failure",(Causa::Io(_),false)=>"Fallo de entrada/salida",(Causa::Io(_),true)=>"Input/output failure",(Causa::LimiteContexto,false)=>"Límite de contexto",(Causa::LimiteContexto,true)=>"Context limit",(Causa::LimiteRecepcion,false)=>"Límite de recepción",(Causa::LimiteRecepcion,true)=>"Reception limit",(_,false)=>"Causa fuera del banco",(_,true)=>"Cause outside the bank"}}
 pub fn registro(&self)->String{format!("{}\t{}\t{:?}\t{}\t{}\n",self.etapa,self.codigo(),self.causa,self.rotulo(false),self.rotulo(true))}
}
#[derive(Debug)]
pub struct RechazoCarga{pub diagnostico:Diagnostico,pub recibidos:Vec<u8>}
pub struct Base{bytes:Vec<u8>,vigente:bool}
impl Base{
 pub fn cargar(path:&Path,requerida:&[u8])->Result<Self,RechazoCarga>{
  let mut recibidos=Vec::with_capacity(65);
  let fail=|causa,recibidos|RechazoCarga{diagnostico:Diagnostico::nuevo("carga_base",causa),recibidos};
  let mut f=std::fs::File::open(path).map_err(|e|fail(Causa::Io(e.kind()),Vec::new()))?;
  loop{let mut b=[0u8;65];let max=65-recibidos.len();match f.read(&mut b[..max]){Ok(0)=>break,Ok(n)=>{recibidos.extend_from_slice(&b[..n]);if recibidos.len()>64{return Err(fail(Causa::LimiteBase,recibidos))}},Err(e)=>return Err(fail(Causa::Io(e.kind()),recibidos))}}
  if requerida.len()>64{return Err(fail(Causa::LimiteBase,recibidos))}
  if recibidos!=requerida{return Err(fail(Causa::BaseDistinta,recibidos))}
  let vigente=match recibidos.as_slice(){b"SV-S14-VIGENCIA/1\nvigente=1\n"=>true,b"SV-S14-VIGENCIA/1\nvigente=0\n"=>false,_=>return Err(fail(Causa::FormatoBase,recibidos))};
  Ok(Self{bytes:recibidos,vigente})
 }
 pub fn bytes(&self)->&[u8]{&self.bytes}
 fn consumir(&self)->bool{if cfg!(forzar_consumo_positivo){true}else{self.vigente}}
}
pub struct Contexto{bytes:Vec<u8>}
impl Contexto{
 pub fn instalar(bytes:&[u8])->Result<Self,Diagnostico>{if bytes.len()>MAX_CONTEXTO{return Err(Diagnostico::nuevo("instalacion_contexto",Causa::LimiteContexto))}Ok(Self{bytes:bytes.to_vec()})}
}
struct Episodio{h:g1::Manejador,base:Base,padre:Option<(u64,u64)>}
pub struct Historia{custodio:g1::Custodio,contexto:Contexto,original:Episodio,nueva:Option<Episodio>}
pub struct Referencia<'a>{original:g1::EntregaLiteral<'a>,solicitud:&'a[u8],base:&'a[u8],documento:&'a[u8],padre:Option<(u64,u64)>}
impl Referencia<'_>{
 pub fn identidad(&self)->(u64,u64){self.original.identidad()}
 pub fn padre(&self)->Option<(u64,u64)>{self.padre}
 pub fn solicitud(&self)->&[u8]{self.solicitud}
 pub fn base(&self)->&[u8]{self.base}
 pub fn documento(&self)->&[u8]{self.documento}
 pub fn cuerpo(&self)->&[u8]{self.original.cuerpo()}
}
impl Historia{
 fn producir(c:&mut g1::Custodio,base:Base,solicitud:&[u8],padre:Option<(u64,u64)>)->Result<Episodio,Diagnostico>{
  let error=|e|Diagnostico::nuevo("produccion",Causa::G1(e));let mut entrada=solicitud;
  let h=c.abrir(&mut entrada,base.consumir()).map_err(error)?;c.ejecutar_a(&h).map_err(error)?;c.no_solicitar_v(&h).map_err(error)?;
  let marco=c.recuperar(&h,g1::Papel::Marco).map_err(error)?.to_vec();
  c.comprobar_entrega(&h,&marco).map_err(|e|Diagnostico::nuevo("entrega_g1",Causa::EntregaG1(e)))?;
  c.entrega(&h).map_err(|e|Diagnostico::nuevo("entrega_g1",Causa::EntregaG1(e)))?;
  Ok(Episodio{h,base,padre})
 }
 pub fn crear(base:Base,solicitud:&[u8],contexto:Contexto,realizacion:g1::Realizacion)->Result<Self,Diagnostico>{
  let mut custodio=g1::Custodio::nuevo(realizacion).map_err(|e|Diagnostico::nuevo("produccion",Causa::G1(e)))?;
  let original=Self::producir(&mut custodio,base,solicitud,None)?;Ok(Self{custodio,contexto,original,nueva:None})
 }
 pub fn reevaluar(&mut self,base:Base,solicitud:&[u8])->Result<(),Diagnostico>{
  if self.nueva.is_some(){return Err(Diagnostico::nuevo("reevaluacion",Causa::Capacidad))}
  if base.bytes()==self.original.base.bytes(){return Err(Diagnostico::nuevo("reevaluacion",Causa::BaseSinCambio))}
  let anterior=self.custodio.recuperar(&self.original.h,g1::Papel::Entrada).map_err(|e|Diagnostico::nuevo("reevaluacion",Causa::G1(e)))?;
  if solicitud!=anterior{return Err(Diagnostico::nuevo("reevaluacion",Causa::SolicitudDistinta))}
  let id=self.custodio.identidad(&self.original.h).map_err(|e|Diagnostico::nuevo("reevaluacion",Causa::G1(e)))?;
  self.nueva=Some(Self::producir(&mut self.custodio,base,solicitud,Some(id))?);Ok(())
 }
 fn episodio(&self,nuevo:bool)->Result<&Episodio,Diagnostico>{if nuevo{self.nueva.as_ref().ok_or(Diagnostico::nuevo("referencia",Causa::Capacidad))}else{Ok(&self.original)}}
 pub fn referencia(&self,nuevo:bool)->Result<Referencia<'_>,Diagnostico>{let e=self.episodio(nuevo)?;
  Ok(Referencia{original:self.custodio.entrega(&e.h).map_err(|x|Diagnostico::nuevo("entrega_g1",Causa::EntregaG1(x)))?,solicitud:self.custodio.recuperar(&e.h,g1::Papel::Entrada).map_err(|x|Diagnostico::nuevo("referencia",Causa::G1(x)))?,base:e.base.bytes(),documento:&self.contexto.bytes,padre:e.padre})
 }
 pub fn atribuir_original(&self,id:(u64,u64))->Result<Referencia<'_>,Diagnostico>{let r=self.referencia(false)?;if r.identidad()!=id{return Err(Diagnostico::nuevo("atribucion",Causa::Identidad))}Ok(r)}
 pub fn recuperar(&self,nuevo:bool,papel:g1::Papel)->Result<&[u8],Diagnostico>{let e=self.episodio(nuevo)?;self.custodio.recuperar(&e.h,papel).map_err(|x|Diagnostico::nuevo("recuperacion_g1",Causa::G1(x)))}
}
pub struct Entrada{bytes:Vec<u8>,fallo:Option<Causa>}
impl Entrada{
 pub fn leer(lector:&mut impl Read)->Self{
  let mut bytes=Vec::with_capacity(MAX_RECEPCION+1);let mut fallo=None;
  loop{let mut b=[0u8;512];let max=b.len().min(MAX_RECEPCION+1-bytes.len());match lector.read(&mut b[..max]){Ok(0)=>break,Ok(n)=>{bytes.extend_from_slice(&b[..n]);if bytes.len()>MAX_RECEPCION{fallo=Some(Causa::LimiteRecepcion);break}},Err(e)=>{fallo=Some(Causa::Comunicacion(e.kind()));break}}}
  Self{bytes,fallo}
 }
 pub fn bytes(&self)->&[u8]{&self.bytes}
 pub fn comprobar<'a>(&'a self,r:&'a Referencia<'a>)->Informe<'a>{
  let mut i=Informe{entrada:&self.bytes,diagnostico:Diagnostico::nuevo("cobertura",Causa::Conforme),puertas:[0;5],entrega:None,motivo:None};
  if let Some(c)=&self.fallo{i.diagnostico=Diagnostico::nuevo("recepcion",match c{Causa::LimiteRecepcion=>Causa::LimiteRecepcion,Causa::Comunicacion(k)=>Causa::Comunicacion(*k),_=>unreachable!()});return i}
  i.puertas[0]+=1;
  let mensaje=match decodificar(&self.bytes){Ok(m)=>m,Err(e)=>{i.diagnostico=Diagnostico::nuevo("protocolo",Causa::Esquema(e));return i}};
  let (id,documento,cuerpo,solicitud,base)=match mensaje{
   Mensaje::Negativa(m)=>{i.motivo=Some(m);i.diagnostico=Diagnostico::nuevo("protocolo",Causa::Negativa);
    #[cfg(admitir_negativa)] {let p=r.original.comprobar_presentacion(r.original.cuerpo()).unwrap();i.entrega=Some(EntregaComprobada{presentacion:p,referencia:r});i.diagnostico=Diagnostico::nuevo("cobertura",Causa::Conforme);}
    return i},
   Mensaje::Respuesta{id,documento,cuerpo,solicitud,base}=>(id,documento,cuerpo,solicitud,base)};
  i.puertas[1]+=1;
  if documento.len()>MAX_CONTEXTO{i.diagnostico=Diagnostico::nuevo("contexto",Causa::LimiteContexto);return i}
  if id!=r.identidad(){i.diagnostico=Diagnostico::nuevo("contexto",Causa::Identidad);return i}
  if !cfg!(omitir_contexto)&&documento!=r.documento{i.diagnostico=Diagnostico::nuevo("contexto",Causa::DocumentoDistinto);return i}
  i.puertas[2]+=1;
  let p=match r.original.comprobar_presentacion(cuerpo){Ok(p)=>p,Err(e)=>{i.diagnostico=Diagnostico::nuevo("cuerpo",Causa::Presentacion(e));return i}};
  i.puertas[3]+=1;
  match solicitud{None=>{i.diagnostico=Diagnostico::nuevo("cobertura",Causa::FaltaSolicitud);return i},Some(s) if s!=r.solicitud=>{i.diagnostico=Diagnostico::nuevo("cobertura",Causa::SolicitudDistinta);return i},_=>{}}
  i.puertas[4]+=1;
  match base{None=>{i.diagnostico=Diagnostico::nuevo("cobertura",Causa::FaltaBase);return i},Some(b) if !cfg!(omitir_base_citada)&&b!=r.base=>{i.diagnostico=Diagnostico::nuevo("cobertura",Causa::BaseDistinta);return i},_=>{}}
  i.entrega=Some(EntregaComprobada{presentacion:p,referencia:r});i
 }
}
pub struct Informe<'a>{entrada:&'a[u8],pub diagnostico:Diagnostico,pub puertas:[u32;5],entrega:Option<EntregaComprobada<'a>>,motivo:Option<&'a[u8]>}
impl<'a> Informe<'a>{pub fn entrada(&self)->&[u8]{self.entrada}pub fn entrega(&self)->Option<&EntregaComprobada<'a>>{self.entrega.as_ref()}pub fn motivo(&self)->Option<&[u8]>{self.motivo}}
pub struct EntregaComprobada<'a>{presentacion:g1::PresentacionComprobada<'a>,referencia:&'a Referencia<'a>}
impl EntregaComprobada<'_>{pub fn cuerpo(&self)->&[u8]{self.presentacion.texto()}pub fn identidad(&self)->(u64,u64){self.referencia.identidad()}}
struct Cursor<'a>{b:&'a[u8],p:usize}
impl<'a> Cursor<'a>{fn tomar(&mut self,n:usize)->Result<&'a[u8],Esquema>{let f=self.p.checked_add(n).ok_or(Esquema::Truncado)?;let b=self.b.get(self.p..f).ok_or(Esquema::Truncado)?;self.p=f;Ok(b)}fn byte(&mut self)->Result<u8,Esquema>{Ok(self.tomar(1)?[0])}fn campo(&mut self)->Result<&'a[u8],Esquema>{let n=u32::from_be_bytes(self.tomar(4)?.try_into().unwrap()) as usize;self.tomar(n)}fn opcional(&mut self)->Result<Option<&'a[u8]>,Esquema>{match self.byte()?{0=>Ok(None),1=>Ok(Some(self.campo()?)),_=>Err(Esquema::Bandera)}}}
enum Mensaje<'a>{Negativa(&'a[u8]),Respuesta{id:(u64,u64),documento:&'a[u8],cuerpo:&'a[u8],solicitud:Option<&'a[u8]>,base:Option<&'a[u8]>}}
fn decodificar(b:&[u8])->Result<Mensaje<'_>,Esquema>{let mut c=Cursor{b,p:0};if c.tomar(5)?!=b"SV17\x01"{return Err(Esquema::Cabecera)}let m=match c.byte()?{b'N'=>Mensaje::Negativa(c.campo()?),b'R'=>{let a=u64::from_be_bytes(c.tomar(8)?.try_into().unwrap());let z=u64::from_be_bytes(c.tomar(8)?.try_into().unwrap());Mensaje::Respuesta{id:(a,z),documento:c.campo()?,cuerpo:c.campo()?,solicitud:c.opcional()?,base:c.opcional()?}},_=>return Err(Esquema::Tipo)};if c.p!=b.len(){return Err(Esquema::Sobrante)}Ok(m)}
pub fn escribir_en(e:&EntregaComprobada<'_>,w:&mut impl Write)->Diagnostico{
 if let Err(x)=w.write_all(e.cuerpo()){return Diagnostico::nuevo("escritura_archivo",Causa::Io(x.kind()))}
 if let Err(x)=w.flush(){return Diagnostico::nuevo("flush_archivo",Causa::Io(x.kind()))}Diagnostico::nuevo("archivo",Causa::Conforme)
}
pub fn escribir_archivo(e:&EntregaComprobada<'_>,p:&Path)->Diagnostico{match std::fs::OpenOptions::new().write(true).create_new(true).open(p){Ok(mut f)=>escribir_en(e,&mut f),Err(x)=>Diagnostico::nuevo("apertura_archivo",Causa::Io(x.kind()))}}
pub fn comprobar_archivo(r:&Referencia<'_>,p:&Path)->(Diagnostico,Vec<u8>){
 let mut f=match std::fs::File::open(p){Ok(f)=>f,Err(e)=>return(Diagnostico::nuevo("recuperacion",Causa::Io(e.kind())),Vec::new())};
 let mut bytes=Vec::with_capacity(MAX_RECEPCION+1);
 loop{let mut b=[0u8;512];let max=b.len().min(MAX_RECEPCION+1-bytes.len());match f.read(&mut b[..max]){Ok(0)=>break,Ok(n)=>{bytes.extend_from_slice(&b[..n]);if bytes.len()>MAX_RECEPCION{return(Diagnostico::nuevo("recuperacion",Causa::LimiteRecepcion),bytes)}},Err(e)=>return(Diagnostico::nuevo("recuperacion",Causa::Io(e.kind())),bytes)}}
 let d=match r.original.comprobar_presentacion(&bytes){Ok(_)=>Diagnostico::nuevo("recuperacion",Causa::Conforme),Err(e)=>Diagnostico::nuevo("recuperacion",Causa::Presentacion(e))};(d,bytes)
}
