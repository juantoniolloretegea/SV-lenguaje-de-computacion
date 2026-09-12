#![forbid(unsafe_code)]
use lote::{EnlacePublico, lectura::*};
const L:&[u8]=include_bytes!("../lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../lote-p3/MONTAJES_PUBLICOS.json");
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn nuevo()->EnlacePublico{EnlacePublico::preparar(L,M,g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}).unwrap()}
fn ejecutar(l:&mut EnlacePublico,i:usize){l.ejecutar(i).unwrap();let m=l.recuperar(i,g1::Papel::Marco).unwrap().to_vec();l.comprobar_entrega(i,&m).unwrap();}
fn propuesta(id:(u64,u64),texto:&[u8])->PropuestaDeLectura<'_>{PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:id,texto}}
#[test]fn lv01_24_entregas_y_procedencia(){
 let mut l=nuevo();let dir=std::path::PathBuf::from(std::env::var("LV_CAPTURAS").unwrap());
 for i in 0..24{
  ejecutar(&mut l,i);let q=l.consulta_lectura(i).unwrap();let b=l.cuerpo(i).unwrap();
  let p=[b" \n\t".as_slice(),b,b"\r\n"].concat();let v=q.comprobar(propuesta(q.identidad(),&p)).unwrap();
  assert_eq!(v.texto(),p);assert_eq!(v.original(),b);assert_eq!(v.identidad(),l.identidad(i).unwrap());
  assert_eq!(v.version(),VERSION);assert_eq!(v.operacion(),OPERACION);assert_eq!(v.perfil(),"IE004-PRESENTACION-ESPACIOS/1");
  assert_eq!(v.solicitud(),l.recuperar(i,g1::Papel::Entrada).unwrap());assert_eq!(v.traza_a(),l.recuperar(i,g1::Papel::Traza).unwrap());
  assert_eq!(v.lote_original(),L);assert_eq!(v.montaje_original(),M);assert_eq!(v.posicion(),i);
  let (a,z)=l.rango(i).unwrap();assert_eq!(v.rango(),(a,z));assert_eq!(v.caso_original(),&L[a..z]);
  assert_eq!(v.traza_adaptador(),l.productos_adaptador().1);
  assert!(std::ptr::eq(v.original().as_ptr(),b.as_ptr()));
  assert!(std::ptr::eq(v.solicitud().as_ptr(),l.recuperar(i,g1::Papel::Entrada).unwrap().as_ptr()));
  for(ext,bytes)in[("original",v.original()),("presentacion",v.texto()),("solicitud",v.solicitud()),("caso",v.caso_original()),("traza_a",v.traza_a())]{std::fs::write(dir.join(format!("P3-{:02}.{ext}",i+1)),bytes).unwrap();}
 }
 l.completo().unwrap();println!("LV01: 24 entregas y sus fuentes recuperadas");
}
#[test]fn lv02_cuerpos_identicos_invocaciones_distintas(){
 let(mut a,mut b)=(nuevo(),nuevo());ejecutar(&mut a,0);ejecutar(&mut b,0);
 assert_eq!(a.cuerpo(0).unwrap(),b.cuerpo(0).unwrap());let qa=a.consulta_lectura(0).unwrap();let qb=b.consulta_lectura(0).unwrap();assert_ne!(qa.identidad(),qb.identidad());
 assert!(matches!(qa.comprobar(propuesta(qb.identidad(),b.cuerpo(0).unwrap())),Err(FalloLectura::Identidad)));
 assert!(matches!(qb.comprobar(propuesta(qa.identidad(),a.cuerpo(0).unwrap())),Err(FalloLectura::Identidad)));
 qa.comprobar(propuesta(qa.identidad(),a.cuerpo(0).unwrap())).unwrap();qb.comprobar(propuesta(qb.identidad(),b.cuerpo(0).unwrap())).unwrap();
 println!("LV02: igualdad de bytes confirmada; cruces rechazados; propios aceptados");
}
#[test]fn lv03_ordinal_ajeno(){let mut a=nuevo();ejecutar(&mut a,0);ejecutar(&mut a,1);let q=a.consulta_lectura(0).unwrap();let id=a.identidad(1).unwrap();assert_eq!(q.identidad().0,id.0);assert_ne!(q.identidad().1,id.1);assert!(matches!(q.comprobar(propuesta(id,a.cuerpo(0).unwrap())),Err(FalloLectura::Identidad)));}
#[test]fn lv04_operacion_ajena(){let mut a=nuevo();ejecutar(&mut a,0);let q=a.consulta_lectura(0).unwrap();for op in ["EJECUTAR","INSCRIBIR_FRAME"]{let mut p=propuesta(q.identidad(),a.cuerpo(0).unwrap());p.operacion=op;assert!(matches!(q.comprobar(p),Err(FalloLectura::Operacion)));}}
#[test]fn lv05_version_ajena(){let mut a=nuevo();ejecutar(&mut a,0);let q=a.consulta_lectura(0).unwrap();let mut p=propuesta(q.identidad(),a.cuerpo(0).unwrap());p.version="IE004-LECTURA-VINCULADA/2";assert!(matches!(q.comprobar(p),Err(FalloLectura::Version)));}
#[test]fn lv06_cabeceras_acotadas(){let mut a=nuevo();ejecutar(&mut a,0);let q=a.consulta_lectura(0).unwrap();let x="x".repeat(65);for version in [true,false]{let mut p=propuesta(q.identidad(),b"JSON incorrecto");if version{p.version=&x}else{p.operacion=&x};assert!(matches!(q.comprobar(p),Err(FalloLectura::LimiteCabecera)));}let x="x".repeat(64);let mut p=propuesta(q.identidad(),b"");p.version=&x;assert!(matches!(q.comprobar(p),Err(FalloLectura::Version)));}
#[test]fn lv07_contenido_ajeno(){let mut a=nuevo();for i in 0..3{ejecutar(&mut a,i);}let q=a.consulta_lectura(0).unwrap();assert_ne!(a.cuerpo(0).unwrap(),a.cuerpo(2).unwrap());assert!(matches!(q.comprobar(propuesta(q.identidad(),a.cuerpo(2).unwrap())),Err(FalloLectura::Presentacion(g1::FalloPresentacion::ContenidoDistinto))));}
#[test]fn lv08_limite_presentacion(){let mut a=nuevo();ejecutar(&mut a,0);let q=a.consulta_lectura(0).unwrap();let mut p=a.cuerpo(0).unwrap().to_vec();p.resize(16384,b' ');q.comprobar(propuesta(q.identidad(),&p)).unwrap();p.push(b' ');assert!(matches!(q.comprobar(propuesta(q.identidad(),&p)),Err(FalloLectura::Presentacion(g1::FalloPresentacion::Limite))));}
#[test]fn lv09_sin_entrega_previa(){let mut a=nuevo();assert!(a.consulta_lectura(0).is_err());a.ejecutar(0).unwrap();assert!(a.consulta_lectura(0).is_err());assert!(a.consulta_lectura(usize::MAX).is_err());let m=a.recuperar(0,g1::Papel::Marco).unwrap().to_vec();a.comprobar_entrega(0,&m).unwrap();assert!(a.consulta_lectura(0).is_ok());}
