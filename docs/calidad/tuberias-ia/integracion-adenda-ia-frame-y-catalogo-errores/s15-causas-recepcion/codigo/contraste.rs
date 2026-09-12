#![forbid(unsafe_code)]
mod receptor;
use std::{io::{Read,Error,ErrorKind},path::Path};
use lote::EnlacePublico;
use cobertura::Referencia;
const L:&[u8]=include_bytes!("../fuentes-base/lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../fuentes-base/lote-p3/MONTAJES_PUBLICOS.json");
const NEG:&[u8]=include_bytes!("../especimenes/negativo.cuerpo");
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn save(d:&Path,n:&str,b:&[u8]){std::fs::write(d.join(n),b).unwrap()}
// Constructor de estímulos independiente del decodificador del receptor.
fn campo(v:&mut Vec<u8>,b:&[u8]){v.extend_from_slice(&(b.len() as u32).to_be_bytes());v.extend_from_slice(b)}
fn respuesta(id:(u64,u64),caso:Option<&[u8]>,vigencia:Option<&[u8]>)->Vec<u8>{let mut v=b"SV15\x01R".to_vec();v.extend_from_slice(&id.0.to_be_bytes());v.extend_from_slice(&id.1.to_be_bytes());campo(&mut v,NEG);for o in [caso,vigencia]{v.push(u8::from(o.is_some()));if let Some(b)=o{campo(&mut v,b)}}v}
fn negativa(b:&[u8])->Vec<u8>{let mut v=b"SV15\x01N".to_vec();campo(&mut v,b);v}
struct Flujo<'a>{bytes:&'a[u8],p:usize,error:Option<ErrorKind>}
impl Read for Flujo<'_>{fn read(&mut self,b:&mut[u8])->std::io::Result<usize>{if self.p==self.bytes.len(){return match self.error{Some(e)=>Err(Error::from(e)),None=>Ok(0)}}let n=b.len().min(7).min(self.bytes.len()-self.p);b[..n].copy_from_slice(&self.bytes[self.p..self.p+n]);self.p+=n;Ok(n)}}
fn main(){
 let dir=std::path::PathBuf::from(std::env::var("S15_CAPTURAS").unwrap());
 let mut e=EnlacePublico::preparar(L,M,g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}).unwrap();
 for j in 0..11{e.ejecutar(j).unwrap();let m=e.recuperar(j,g1::Papel::Marco).unwrap().to_vec();e.comprobar_entrega(j,&m).unwrap()}
 assert_eq!(e.cuerpo(10).unwrap(),NEG);let r=Referencia::desde(&e,10).unwrap();let id=r.identidad();let c=r.caso_requerido();let v=r.vigencia_requerida();
 for (n,b) in [("referencia.cuerpo",NEG),("referencia.caso",c),("referencia.vigencia",v),("referencia.lote",L),("referencia.montaje",M),("referencia.traza",e.recuperar(10,g1::Papel::Traza).unwrap())]{save(&dir,n,b)}
 let normal=respuesta(id,Some(c),Some(v));assert!(normal.len()<receptor::LIMITE);
 let mut cab=normal.clone();cab[0]=b'X';let mut trunc=normal.clone();trunc.pop();let mut sobra=normal.clone();sobra.push(b' ');
 let entradas=vec![normal.clone(),negativa(b"Servicio no disponible"),cab,trunc,respuesta(id,Some(c),None),respuesta((id.0,id.1+1),Some(c),Some(v)),vec![],normal.clone(),normal[..12].to_vec(),negativa(NEG),sobra,respuesta(id,None,Some(v)),vec![b' ';16385],vec![]];
 let esperado=include_str!("ESPERADO.tsv");let lineas:Vec<_>=esperado.lines().collect();assert_eq!(lineas.len(),entradas.len());
 for (j,b) in entradas.iter().enumerate(){
  let nombre=format!("F{:02}",j+1);let err=match j{6=>Some(ErrorKind::TimedOut),7|8=>Some(ErrorKind::ConnectionReset),_=>None};
  save(&dir,&format!("{nombre}.entrada-prevista"),b);save(&dir,&format!("{nombre}.evento-previsto"),format!("{err:?}").as_bytes());
  let mut flujo=Flujo{bytes:b,p:0,error:err};let inf=receptor::recibir(&r,&mut flujo);
  save(&dir,&format!("{nombre}.entrada-recibida"),inf.entrada());let registro=inf.registro();
  let path=dir.join(format!("{nombre}.registro"));std::fs::write(&path,registro.as_bytes()).unwrap();let recuperado=std::fs::read(&path).unwrap();
  // Presentación textual íntegra desde el registro recuperado; no resume ni traduce causas.
  let presentado=[b"SV-S15-DIAGNOSTICO/1\n".as_slice(),recuperado.as_slice()].concat();save(&dir,&format!("{nombre}.presentado"),&presentado);
  if let Some(x)=inf.cuerpo(){save(&dir,&format!("{nombre}.cuerpo"),x)}
  if let Some(x)=inf.motivo(){save(&dir,&format!("{nombre}.motivo"),x)}
  let observado=format!("{nombre}\t{registro}");let wanted=format!("{}\n",lineas[j]);let body_ok=if j==0{inf.cuerpo()==Some(NEG)}else{inf.cuerpo().is_none()};
  let motivo_ok=match j{1=>inf.motivo()==Some(b"Servicio no disponible".as_slice()),9=>inf.motivo()==Some(NEG),_=>inf.motivo().is_none()};
  if observado!=wanted||inf.entrada()!=b||recuperado!=registro.as_bytes()||!body_ok||!motivo_ok{eprintln!("FALLO {nombre}: observado={observado:?}; entrada={}, cuerpo={}, motivo={}",inf.entrada()==b,body_ok,motivo_ok);std::process::exit(1)}
  print!("{observado}");
 }
 save(&dir,"referencia.final",e.cuerpo(10).unwrap());assert_eq!(e.cuerpo(10).unwrap(),NEG)
}
