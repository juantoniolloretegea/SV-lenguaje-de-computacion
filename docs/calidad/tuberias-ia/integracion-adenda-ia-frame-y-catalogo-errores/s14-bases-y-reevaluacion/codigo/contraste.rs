#![forbid(unsafe_code)]
mod bases;
use bases::{Base,Historia,Fallo,Vista};
use std::{path::Path,io::Write};
const B0:&[u8]=include_bytes!("../especimenes/base-original.bin");
const B1:&[u8]=include_bytes!("../especimenes/base-nueva.bin");
const BAD:&[u8]=include_bytes!("../especimenes/base-invalida.bin");
const BIG:&[u8]=include_bytes!("../especimenes/base-excesiva.bin");
const REQ:&[u8]=include_bytes!("../especimenes/solicitud.bin");
const POS:&[u8]=include_bytes!("../especimenes/positivo.cuerpo");
const NEG:&[u8]=include_bytes!("../especimenes/negativo.cuerpo");
fn hash(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn guardar(d:&Path,n:&str,b:&[u8]){let mut f=std::fs::OpenOptions::new().write(true).create_new(true).open(d.join(n)).unwrap();f.write_all(b).unwrap();}
fn base(d:&Path,id:&str,requerida:&[u8],cargada:&[u8])->Result<Base,Fallo>{
 let p=d.join(bases::NOMBRE);std::fs::write(&p,cargada).unwrap(); // única ruta de entrada reemplazada por el conductor adversarial
 guardar(d,&format!("{id}.base-requerida"),requerida);guardar(d,&format!("{id}.archivo-cargado"),&std::fs::read(&p).unwrap());Base::cargar(&p,requerida)
}
fn vista(d:&Path,id:&str,v:&Vista<'_>){
 for(n,b)in[("base-consumida",v.base),("cuerpo",v.cuerpo),("solicitud",v.solicitud),("traza",v.traza),("marco",v.marco)]{guardar(d,&format!("{id}.{n}"),b)}
 guardar(d,&format!("{id}.ligadura"),format!("S14-ACTO/1\nnombre={}\nidentidad={},{}\npadre={:?}\n",bases::NOMBRE,v.identidad.0,v.identidad.1,v.padre).as_bytes());
}
fn atribuir<'a>(d:&Path,id:&str,h:&'a Historia,identidad:(u64,u64),base:&[u8],cuerpo:&[u8])->Result<Vista<'a>,Fallo>{guardar(d,&format!("{id}.base-atribuida"),base);guardar(d,&format!("{id}.cuerpo-atribuido"),cuerpo);guardar(d,&format!("{id}.identidad-atribuida"),format!("{},{}",identidad.0,identidad.1).as_bytes());h.atribuir_original(identidad,base,cuerpo)}
fn comprobar(d:&Path,id:&str,ok:bool,observado:&str,etiqueta:&str){guardar(d,&format!("{id}.resultado-real"),observado.as_bytes());if !ok{eprintln!("FALLO {id}: {observado}");std::process::exit(1)}println!("{id}\t{etiqueta}");}
fn error<T>(d:&Path,id:&str,r:Result<T,Fallo>,esperado:Fallo,tag:&str){match r{Ok(_)=>comprobar(d,id,false,"ACEPTADO",tag),Err(e)=>comprobar(d,id,e==esperado,&format!("{e:?}"),tag)}}
fn main(){
 let d=std::path::PathBuf::from(std::env::var("S14_CAPTURAS").unwrap());
 let b0=base(&d,"GJ01",B0,B0).unwrap();let mut h=Historia::crear(b0,REQ,g1::Realizacion{fuentes_sha256:hash("LG1_FUENTES"),binario_sha256:hash("LG1_BINARIO")}).unwrap();
 let id0=h.original().unwrap().identidad;{let v=h.original().unwrap();vista(&d,"GJ01",&v);comprobar(&d,"GJ01",v.cuerpo==POS&&v.base==B0&&v.solicitud==REQ&&v.padre.is_none(),"ORIGINAL_PRODUCIDO","DATO_ORIGINAL");}
 error(&d,"GJ02",base(&d,"GJ02",B0,B1),Fallo::BaseDistinta,"BaseDistinta");
 let b=base(&d,"GJ03",B0,B0).unwrap();error(&d,"GJ03",h.reevaluar(b,REQ),Fallo::BaseSinCambio,"BaseSinCambio");
 let b=base(&d,"GJ04",B1,B1).unwrap();h.reevaluar(b,REQ).unwrap();
 let id1=h.nueva().unwrap().identidad;{let v=h.nueva().unwrap();vista(&d,"GJ04",&v);comprobar(&d,"GJ04",v.cuerpo==NEG&&v.base==B1&&v.solicitud==REQ&&v.padre==Some(id0)&&id0!=id1,"NUEVA_PRODUCIDA","PERMISO_REVOCADO_NUEVO");}
 {let v=atribuir(&d,"GJ05",&h,id0,B0,POS).unwrap();vista(&d,"GJ05",&v);comprobar(&d,"GJ05",v.cuerpo==POS&&v.base==B0&&v.padre.is_none(),"ORIGINAL_RECUPERADO","ORIGINAL_INTACTO");}
 error(&d,"GJ06",atribuir(&d,"GJ06",&h,id0,B1,POS),Fallo::BaseDistinta,"BaseDistinta");
 error(&d,"GJ07",atribuir(&d,"GJ07",&h,id1,B1,NEG),Fallo::Identidad,"Identidad");
 error(&d,"GJ08",atribuir(&d,"GJ08",&h,id0,B0,NEG),Fallo::ReciboDistinto,"ReciboDistinto");
 {let v=h.nueva().unwrap();vista(&d,"GJ09",&v);comprobar(&d,"GJ09",v.cuerpo==NEG&&v.base==B1&&v.padre==Some(id0)&&v.identidad==id1,"REEVALUACION_RECUPERADA","REEVALUACION_DISTINGUIDA");}
 let b=base(&d,"GJ10",B1,B1).unwrap();error(&d,"GJ10",h.reevaluar(b,REQ),Fallo::Capacidad,"Capacidad");
 error(&d,"GJ11",base(&d,"GJ11",BAD,BAD),Fallo::FormatoBase,"FormatoBase");
 error(&d,"GJ12",base(&d,"GJ12",BIG,BIG),Fallo::LimiteBase,"LimiteBase");
 {let v=h.original().unwrap();vista(&d,"original-final",&v);assert_eq!(v.cuerpo,POS);assert_eq!(v.base,B0);}
}
