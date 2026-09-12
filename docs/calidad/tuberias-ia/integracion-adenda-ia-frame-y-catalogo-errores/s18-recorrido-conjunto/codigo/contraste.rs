#![forbid(unsafe_code)]
use recorrido::{Base,Contexto,Historia,Referencia,Entrada,Diagnostico,Causa,Esquema};
use std::{path::{Path,PathBuf},io::{Read,Write,ErrorKind}};
const B0:&[u8]=include_bytes!("../esperados/base-original.bin");
const B1:&[u8]=include_bytes!("../esperados/base-nueva.bin");
const REQ:&[u8]=include_bytes!("../esperados/solicitud.bin");
const POS:&[u8]=include_bytes!("../esperados/positivo.cuerpo");
const NEG:&[u8]=include_bytes!("../esperados/negativo.cuerpo");
const DOC:&[u8]=include_bytes!("../esperados/contexto.txt");
const HOSTIL:&[u8]=include_bytes!("../esperados/contexto-instruccion.txt");
const ESP:&str=include_str!("ESPERADO.tsv");
fn hash(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn guardar(d:&Path,id:&str,n:&str,b:&[u8]){let mut f=std::fs::OpenOptions::new().write(true).create_new(true).open(d.join(format!("{id}.{n}"))).unwrap();f.write_all(b).unwrap();}
fn base(d:&Path,id:&str,n:&str,requerida:&[u8],actual:&[u8])->Result<Base,recorrido::RechazoCarga>{let p=d.join("vigencia.s18");std::fs::write(&p,actual).unwrap();guardar(d,id,&format!("{n}.requerida"),requerida);guardar(d,id,&format!("{n}.archivo"),&std::fs::read(&p).unwrap());Base::cargar(&p,requerida)}
fn referencia(d:&Path,id:&str,n:&str,h:&Historia,nuevo:bool){let r=h.referencia(nuevo).unwrap();
 for(k,b)in[("base",r.base()),("solicitud",r.solicitud()),("documento",r.documento()),("cuerpo",r.cuerpo())]{guardar(d,id,&format!("{n}.{k}"),b)}
 guardar(d,id,&format!("{n}.identidad"),format!("{},{}\npadre={:?}\n",r.identidad().0,r.identidad().1,r.padre()).as_bytes());
 for(k,p)in[("marco",g1::Papel::Marco),("traza",g1::Papel::Traza)]{guardar(d,id,&format!("{n}.{k}"),h.recuperar(nuevo,p).unwrap())}
}
fn campo(out:&mut Vec<u8>,b:&[u8]){out.extend_from_slice(&(u32::try_from(b.len()).unwrap()).to_be_bytes());out.extend_from_slice(b)}
fn opcional(out:&mut Vec<u8>,b:Option<&[u8]>){match b{None=>out.push(0),Some(b)=>{out.push(1);campo(out,b)}}}
fn respuesta(id:(u64,u64),doc:&[u8],cuerpo:&[u8],sol:Option<&[u8]>,base:Option<&[u8]>)->Vec<u8>{let mut b=b"SV17\x01R".to_vec();b.extend_from_slice(&id.0.to_be_bytes());b.extend_from_slice(&id.1.to_be_bytes());campo(&mut b,doc);campo(&mut b,cuerpo);opcional(&mut b,sol);opcional(&mut b,base);b}
struct Tardia{bytes:Vec<u8>,p:usize,llamadas:usize}
impl Read for Tardia{fn read(&mut self,b:&mut[u8])->std::io::Result<usize>{self.llamadas+=1;if self.p==self.bytes.len(){return Err(ErrorKind::ConnectionReset.into())}let n=b.len().min(self.bytes.len()-self.p);b[..n].copy_from_slice(&self.bytes[self.p..self.p+n]);self.p+=n;Ok(n)}}
struct Parcial{bytes:Vec<u8>,llamadas:usize}
impl Write for Parcial{fn write(&mut self,b:&[u8])->std::io::Result<usize>{self.llamadas+=1;if self.bytes.len()==7{return Err(ErrorKind::WriteZero.into())}let n=7-self.bytes.len();self.bytes.extend_from_slice(&b[..n]);Ok(n)}fn flush(&mut self)->std::io::Result<()>{panic!("flush no debe alcanzarse tras escritura fallida")}}
fn fin(d:&Path,id:&str,n:usize,diag:&Diagnostico,ok:bool){guardar(d,id,"diagnostico",diag.registro().as_bytes());let linea=format!("{id}\t{}\t{}",diag.etapa,diag.codigo());let expected=ESP.lines().nth(n-1).unwrap();if !ok||linea!=expected{eprintln!("FALLO {id}: {linea}; predicados={ok}");std::process::exit(1)}println!("{linea}");}
fn estado(d:&Path,id:&str,p:&Path){match std::fs::read(p){Ok(b)=>{guardar(d,id,"archivo-observado",&b);guardar(d,id,"archivo-estado",b"presente\n")},Err(e)if e.kind()==ErrorKind::NotFound=>guardar(d,id,"archivo-estado",b"ausente\n"),Err(e)=>guardar(d,id,"archivo-estado",format!("no_observable:{:?}\n",e.kind()).as_bytes())}}
fn main(){let d=PathBuf::from(std::env::var("S18_CAPTURAS").unwrap());
 for n in 1..=24{let id=format!("I{n:02}");let dest=d.join(format!("destino-{id}"));
  if n==3{std::fs::write(d.join("vigencia.s18"),B0).unwrap();let er=match base(&d,&id,"base",B0,B1){Err(e)=>e,Ok(_)=>panic!("carga inesperada")};guardar(&d,&id,"recibidos",&er.recibidos);guardar(&d,&id,"puertas",b"produccion=0\nemisor=0\nescritura=0\n");estado(&d,&id,&dest);fin(&d,&id,n,&er.diagnostico,er.recibidos==B1&&!dest.exists());continue}
  if n==20{let bytes=vec![b'x';8193];guardar(&d,&id,"contexto-intentado",&bytes);let er=match Contexto::instalar(&bytes){Err(e)=>e,Ok(_)=>panic!("contexto inesperado")};guardar(&d,&id,"puertas",b"produccion=0\nemisor=0\nescritura=0\n");estado(&d,&id,&dest);fin(&d,&id,n,&er,!dest.exists());continue}
  let documento=if n==2{HOSTIL.to_vec()}else if n==23{vec![b'x';8192]}else{DOC.to_vec()};
  let b=base(&d,&id,"original",B0,B0).unwrap();let mut h=Historia::crear(b,REQ,Contexto::instalar(&documento).unwrap(),g1::Realizacion{fuentes_sha256:hash("LG1_FUENTES"),binario_sha256:hash("LG1_BINARIO")}).unwrap();
  referencia(&d,&id,"original",&h,false);let id0=h.referencia(false).unwrap().identidad();let mut id1=None;
  if matches!(n,5|14|15|16|17){let b=base(&d,&id,"nueva",B1,B1).unwrap();h.reevaluar(b,REQ).unwrap();referencia(&d,&id,"nueva",&h,true);id1=Some(h.referencia(true).unwrap().identidad())}
  if n==17{let er=match h.atribuir_original(id1.unwrap()){Err(e)=>e,Ok(_)=>panic!("atribucion inesperada")};guardar(&d,&id,"identidad-atribuida",format!("{:?}",id1.unwrap()).as_bytes());guardar(&d,&id,"puertas",b"emisor=0\nescritura=0\n");estado(&d,&id,&dest);fin(&d,&id,n,&er,h.referencia(false).unwrap().cuerpo()==POS&&!dest.exists());continue}
  let r=h.referencia(n==14).unwrap();let mut doc=r.documento().to_vec();let mut cuerpo=r.cuerpo().to_vec();let mut solicitud=Some(REQ.to_vec());let mut bc=Some(r.base().to_vec());let mut identidad=r.identidad();
  match n{4=>doc[0]^=1,5=>identidad=id1.unwrap(),6=>solicitud=None,7=>solicitud.as_mut().unwrap()[0]^=1,8=>bc=None,9=>bc=Some(B1.to_vec()),10=>{let p=cuerpo.windows(4).position(|s|s==b"8.40").unwrap();cuerpo[p+3]=b'1'},16=>cuerpo=NEG.to_vec(),24=>cuerpo.insert(0,b' '),_=>{}}
  let mut wire=respuesta(identidad,&doc,&cuerpo,solicitud.as_deref(),bc.as_deref());if n==12{wire=b"SV17\x01N".to_vec();campo(&mut wire,b"no_entrego")}if n==13{wire.pop();}if n==21{wire=vec![b'x';16385]}
  guardar(&d,&id,"propuesta",&wire);let entrada=if n==11{let mut t=Tardia{bytes:wire.clone(),p:0,llamadas:0};let e=Entrada::leer(&mut t);guardar(&d,&id,"lecturas",t.llamadas.to_string().as_bytes());e}else{Entrada::leer(&mut wire.as_slice())};
  let info=entrada.comprobar(&r);guardar(&d,&id,"recibidos",info.entrada());guardar(&d,&id,"admision",info.diagnostico.registro().as_bytes());guardar(&d,&id,"puertas",format!("{:?}\nentrega={}\n",info.puertas,info.entrega().is_some()).as_bytes());if let Some(m)=info.motivo(){guardar(&d,&id,"motivo",m)}
  let admitida=info.entrega().is_some();let mut parcial=Vec::new();let mut escrito=false;
  let diag=if let Some(e)=info.entrega(){
   guardar(&d,&id,"entrega",e.cuerpo());
   if n==22{let mut w=Parcial{bytes:Vec::new(),llamadas:0};escrito=true;let v=recorrido::escribir_en(e,&mut w);parcial=w.bytes;guardar(&d,&id,"parcial",&parcial);guardar(&d,&id,"escrituras",w.llamadas.to_string().as_bytes());v}
   else{if n==18{std::fs::write(&dest,b"preexistente").unwrap();}escrito=true;let v=recorrido::escribir_archivo(e,&dest);guardar(&d,&id,"escritura",v.registro().as_bytes());if v.causa==Causa::Conforme{if n==19{std::fs::write(&dest,NEG).unwrap();}let (v,b)=recorrido::comprobar_archivo(&r,&dest);guardar(&d,&id,"recuperado",&b);v}else{v}}
  }else{info.diagnostico};
  guardar(&d,&id,"escritura-intentada",if escrito{b"si\n"}else{b"no\n"});estado(&d,&id,&dest);
  let no_archivo=!dest.exists();let original=h.referencia(false).unwrap();let mut ok=original.cuerpo()==POS&&original.base()==B0&&original.solicitud()==REQ&&original.identidad()==id0&&original.padre().is_none()&&entrada.bytes()==wire;
  ok&=match n{
   1|2=>admitida&&std::fs::read(&dest).unwrap_or_default()==POS,
   4|5=>!admitida&&!escrito&&no_archivo&&info.puertas==[1,1,0,0,0],
   6|7=>!admitida&&!escrito&&no_archivo&&info.puertas==[1,1,1,1,0],
   8|9=>!admitida&&!escrito&&no_archivo&&info.puertas==[1,1,1,1,1],
   10|16=>!admitida&&!escrito&&no_archivo&&info.puertas==[1,1,1,0,0]&&diag.causa==Causa::Presentacion(g1::FalloPresentacion::ContenidoDistinto),
   11=>!admitida&&!escrito&&no_archivo&&info.puertas==[0;5]&&diag.causa==Causa::Comunicacion(ErrorKind::ConnectionReset),
   12=>!admitida&&!escrito&&no_archivo&&info.puertas==[1,0,0,0,0],
   13=>!admitida&&!escrito&&no_archivo&&info.puertas==[1,0,0,0,0]&&diag.causa==Causa::Esquema(Esquema::Truncado),
   14=>admitida&&r.cuerpo()==NEG&&r.base()==B1&&r.solicitud()==REQ&&r.identidad()!=id0&&r.padre()==Some(id0)&&std::fs::read(&dest).unwrap_or_default()==NEG,
   15=>admitida&&r.identidad()==id0&&std::fs::read(&dest).unwrap_or_default()==POS,
   18=>admitida&&escrito&&diag.causa==Causa::Io(ErrorKind::AlreadyExists)&&std::fs::read(&dest).unwrap_or_default()==b"preexistente",
   19=>admitida&&escrito&&std::fs::read(&dest).unwrap_or_default()==NEG&&diag.causa==Causa::Presentacion(g1::FalloPresentacion::ContenidoDistinto),
   21=>!admitida&&!escrito&&no_archivo&&info.puertas==[0;5]&&entrada.bytes().len()==16385,
   22=>admitida&&escrito&&no_archivo&&parcial==POS[..7]&&diag.causa==Causa::Io(ErrorKind::WriteZero),
   23=>admitida&&doc.len()==8192&&wire.len()<=16384&&std::fs::read(&dest).unwrap_or_default()==POS,
   24=>admitida&&std::fs::read(&dest).unwrap_or_default()==[b" ".as_slice(),POS].concat(),
   _=>false};
  fin(&d,&id,n,&diag,ok);
 }
}
