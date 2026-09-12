#![forbid(unsafe_code)]
mod destino;
mod contexto;
use std::path::Path;
use lote::{EnlacePublico,lectura::{PropuestaDeLectura,VERSION,OPERACION}};
use cobertura::{Referencia,Seleccion,FalloCobertura};
use contexto::{Contexto,FalloContexto};
const L:&[u8]=include_bytes!("../fuentes-base/lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../fuentes-base/lote-p3/MONTAJES_PUBLICOS.json");
const NEGATIVO:&[u8]=include_bytes!("../especimenes/negativo.cuerpo");
const NEUTRO:&[u8]=include_bytes!("../especimenes/neutral.txt");
const ORDEN:&[u8]=include_bytes!("../especimenes/orden-omitir.txt");
const LIMITE:&[u8]=include_bytes!("../especimenes/limite.bin");
const EXCESO:&[u8]=include_bytes!("../especimenes/exceso.bin");
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn guardar(dir:&Path,n:&str,b:&[u8]){std::fs::write(dir.join(n),b).unwrap();}
fn comprobar(id:&str,ok:bool,resultado:&str){if !ok{eprintln!("FALLO {id}: {resultado}");std::process::exit(1)}println!("{id}\t{resultado}");}
fn seleccion<'v>(r:&Referencia<'_>,caso:&'v[u8],vigencia:Option<&'v[u8]>)->Seleccion<'v>{Seleccion{lectura:PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:r.identidad(),texto:NEGATIVO},caso:Some(caso),vigencia}}
struct Caso<'a>{id:&'static str,documento_requerido:&'a[u8],documento_presentado:&'a[u8],identidad:(u64,u64),vigencia:Option<&'a[u8]>,esperado:Option<FalloContexto>,etiqueta:&'static str}
fn main(){
 let dir=std::path::PathBuf::from(std::env::var("S11_CAPTURAS").unwrap());
 let mut e=EnlacePublico::preparar(L,M,g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}).unwrap();
 for i in 0..11{e.ejecutar(i).unwrap();let m=e.recuperar(i,g1::Papel::Marco).unwrap().to_vec();e.comprobar_entrega(i,&m).unwrap();}
 assert_eq!(e.cuerpo(10).unwrap(),NEGATIVO);
 let r=Referencia::desde(&e,10).unwrap();let r0=Referencia::desde(&e,0).unwrap();
 let id=r.identidad();let caso=r.caso_requerido();let v=r.vigencia_requerida();
 guardar(&dir,"referencia.cuerpo",NEGATIVO);guardar(&dir,"referencia.caso",caso);guardar(&dir,"referencia.vigencia",v);
 guardar(&dir,"referencia.lote",L);guardar(&dir,"referencia.montaje",M);
 guardar(&dir,"referencia.solicitud",e.recuperar(10,g1::Papel::Entrada).unwrap());
 guardar(&dir,"referencia.traza",e.recuperar(10,g1::Papel::Traza).unwrap());
 guardar(&dir,"referencia.marco",e.recuperar(10,g1::Papel::Marco).unwrap());
 guardar(&dir,"referencia.identidad",format!("{},{}",id.0,id.1).as_bytes());
 let casos=[
 Caso{id:"AH01",documento_requerido:NEUTRO,documento_presentado:NEUTRO,identidad:id,vigencia:Some(v),esperado:None,etiqueta:"DOCUMENTO_NEUTRO_ENTREGADO"},
 Caso{id:"AH02",documento_requerido:ORDEN,documento_presentado:ORDEN,identidad:id,vigencia:Some(v),esperado:None,etiqueta:"ORDEN_CONSERVADA_SIN_AUTORIDAD"},
 Caso{id:"AH03",documento_requerido:ORDEN,documento_presentado:ORDEN,identidad:id,vigencia:None,esperado:Some(FalloContexto::Cobertura(FalloCobertura::FaltaVigencia)),etiqueta:"FaltaVigencia"},
 Caso{id:"AH04",documento_requerido:NEUTRO,documento_presentado:NEUTRO,identidad:id,vigencia:None,esperado:Some(FalloContexto::Cobertura(FalloCobertura::FaltaVigencia)),etiqueta:"FaltaVigencia"},
 Caso{id:"AH05",documento_requerido:ORDEN,documento_presentado:ORDEN,identidad:id,vigencia:Some(r0.vigencia_requerida()),esperado:Some(FalloContexto::Cobertura(FalloCobertura::VigenciaDistinta)),etiqueta:"VigenciaDistinta"},
 Caso{id:"AH06",documento_requerido:ORDEN,documento_presentado:ORDEN,identidad:r0.identidad(),vigencia:Some(v),esperado:Some(FalloContexto::IdentidadContexto),etiqueta:"IdentidadContexto"},
 Caso{id:"AH07",documento_requerido:ORDEN,documento_presentado:&ORDEN[..ORDEN.len()-1],identidad:id,vigencia:Some(v),esperado:Some(FalloContexto::DocumentoDistinto),etiqueta:"DocumentoDistinto"},
 Caso{id:"AH08",documento_requerido:ORDEN,documento_presentado:NEUTRO,identidad:id,vigencia:Some(v),esperado:Some(FalloContexto::DocumentoDistinto),etiqueta:"DocumentoDistinto"},
 Caso{id:"AH09",documento_requerido:LIMITE,documento_presentado:EXCESO,identidad:id,vigencia:Some(v),esperado:Some(FalloContexto::LimiteDocumento),etiqueta:"LimiteDocumento"},
 Caso{id:"AH10",documento_requerido:LIMITE,documento_presentado:LIMITE,identidad:id,vigencia:Some(v),esperado:None,etiqueta:"DOCUMENTO_LIMITE_ENTREGADO"}];
 // Las referencias de caso y vigencia preceden a todas las selecciones.
 for c in casos{
  let contexto=Contexto::desde(&r,c.documento_requerido).unwrap();
  guardar(&dir,&format!("{}.documento-requerido",c.id),c.documento_requerido);
  guardar(&dir,&format!("{}.documento-presentado",c.id),c.documento_presentado);
  guardar(&dir,&format!("{}.identidad-presentada",c.id),format!("{},{}",c.identidad.0,c.identidad.1).as_bytes());
  guardar(&dir,&format!("{}.cuerpo-propuesto",c.id),NEGATIVO);
  guardar(&dir,&format!("{}.caso-citado",c.id),caso);
  guardar(&dir,&format!("{}.vigencia-presente",c.id),if c.vigencia.is_some(){b"true"}else{b"false"});
  if let Some(b)=c.vigencia{guardar(&dir,&format!("{}.vigencia-citada",c.id),b)}
  let resultado=contexto.comprobar(c.identidad,c.documento_presentado,seleccion(&r,caso,c.vigencia));
  let path=dir.join(format!("{}.destino",c.id));
  let ok=match resultado{
   Ok(ref entrega)=>{
    destino::escribir(&path,entrega).unwrap();let rec=destino::leer(&path).unwrap();
    destino::verificar(&r,&rec,caso,v).unwrap();
    guardar(&dir,&format!("{}.recuperado",c.id),&rec);guardar(&dir,&format!("{}.resultado-real",c.id),b"ENTREGA");
    c.esperado.is_none()&&rec==NEGATIVO&&entrega.caso_citado()==caso&&entrega.vigencia_citada()==v
   },
   Err(ref error)=>{guardar(&dir,&format!("{}.resultado-real",c.id),format!("{error:?}").as_bytes());c.esperado.as_ref()==Some(error)&&!path.exists()}
  };
  guardar(&dir,&format!("{}.destino-existe",c.id),if path.exists(){b"true"}else{b"false"});
  comprobar(c.id,ok,c.etiqueta);
 }
 assert!(matches!(Contexto::desde(&r,EXCESO),Err(FalloContexto::LimiteDocumento)));
 guardar(&dir,"referencia.final",e.cuerpo(10).unwrap());assert_eq!(e.cuerpo(10).unwrap(),NEGATIVO);
}
