use super::*;
const L:&[u8]=include_bytes!("../lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../lote-p3/MONTAJES_PUBLICOS.json");
fn realizacion()->g1::Realizacion{
 fn h(k:&str)->[u8;32]{let s=std::env::var(k).expect("conductor debe fijar huellas");assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[2*i..2*i+2],16).unwrap())}
 g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}
}
fn nuevo()->EnlacePublico{EnlacePublico::preparar(L,M,realizacion()).unwrap()}
fn pasa(id:&str){println!("TESTIGO_LOTE_G1:{id}:CONFORME");}
fn productos()->(Vec<u8>,Vec<u8>,(u64,u64)){let mut c=cuenta_lote();let(m,t)=adaptar(L,LOTE_FIJADO,&mut c).unwrap();(m,t.b,(c.trabajo,c.reservado))}
fn con(m:Vec<u8>,t:Vec<u8>,c:(u64,u64))->ER<EnlacePublico>{EnlacePublico::con_productos(L,M,realizacion(),m,t,c)}
fn rehacer(b:&[u8])->Vec<u8>{let mut v=b"SVLT0001".to_vec();v.extend_from_slice(&(b.len()as u64).to_le_bytes());v.extend_from_slice(b);v.extend_from_slice(&sha256(b).unwrap());v}
#[test]fn a_lg01_lote_completo(){
 let mut e=nuevo();assert_eq!(e.completo(),Err(ErrorEnlace::Incompleto));for i in 0..24{e.ejecutar(i).unwrap();}e.completo().unwrap();
 assert_eq!(e.originales(),(L,M));assert_eq!(e.custodios.len(),6);assert!(e.reservas_g1().unwrap()<=AGREGADO);
 let out=std::path::PathBuf::from(std::env::var("LG1_CAPTURAS").unwrap());let save=|p:&str,b:&[u8]|{let mut f=std::fs::OpenOptions::new().write(true).create_new(true).open(out.join(p)).unwrap();f.write_all(b).unwrap();};
 save("lote-original.json",L);save("montaje-original.json",M);let(m,t)=e.productos_adaptador();save("adaptador.marco",m);save("adaptador.traza",t);
 let mut filas=Vec::new();let mut identidades=std::collections::BTreeSet::new();
 for i in 0..24{
  let nombre=format!("P3-{:02}",i+1);let h=e.identidad(i).unwrap();assert!(identidades.insert(h));
  for (ext,p) in [("solicitud.json",g1::Papel::Entrada),("marco",g1::Papel::Marco),("traza",g1::Papel::Traza)]{save(&format!("{nombre}.{ext}"),e.recuperar(i,p).unwrap());}
  let b=e.cuerpo(i).unwrap();save(&format!("{nombre}.cuerpo.json"),b);let(a,z)=e.rango(i).unwrap();
  assert_eq!(e.montaje(i).unwrap(),![3,13,23].contains(&i));
  filas.push(format!("{{\"posicion\":{i},\"id\":\"{nombre}\",\"ambito\":{},\"ordinal\":{},\"inicio\":{a},\"fin\":{z},\"vigente\":{},\"solicitud_sha256\":\"{}\",\"cuerpo_sha256\":\"{}\",\"v\":\"NO_SOLICITADA\"}}",h.0,h.1,e.montaje(i).unwrap(),huella(e.recuperar(i,g1::Papel::Entrada).unwrap()).unwrap(),huella(b).unwrap()));
 }
 let (p,c)=e.cuentas();save("INDICE.json",format!("{{\"version\":\"{VERSION_ENLACE}\",\"lote_sha256\":\"{LOTE_FIJADO}\",\"montaje_sha256\":\"{MONTAJE_FIJADO}\",\"preparacion_trabajo\":{},\"preparacion_reserva\":{},\"enlace_trabajo\":{},\"enlace_reserva\":{},\"g1_reserva_agregada\":{},\"casos\":[{}]}}\n",p.0,p.1,c.0,c.1,e.reservas_g1().unwrap(),filas.join(",")).as_bytes());pasa("LG1-01");
}
#[test]fn b_lg02_identidad_lote(){let mut b=L.to_vec();b.push(b' ');assert!(matches!(EnlacePublico::preparar(&b,M,realizacion()),Err(ErrorEnlace::IdentidadLote)));pasa("LG1-02");}
#[test]fn c_lg03_identidad_montaje(){let m=std::str::from_utf8(M).unwrap().replacen("false","true",1);assert!(matches!(EnlacePublico::preparar(L,m.as_bytes(),realizacion()),Err(ErrorEnlace::IdentidadMontaje)));pasa("LG1-03");}
#[test]fn d_lg04_integridad(){let(m,t,c)=productos();let mut cut=m.clone();cut.pop();assert!(matches!(con(cut,t.clone(),c),Err(ErrorEnlace::Integridad)));let mut change=m;change[20]^=1;assert!(matches!(con(change,t,c),Err(ErrorEnlace::Integridad)));pasa("LG1-04");}
#[test]fn e_lg05_peticion_intercambiada(){let(m,t,c)=productos();let b=std::str::from_utf8(payload(&m).unwrap()).unwrap().replacen("P3-01","P3-02",1);assert!(matches!(con(rehacer(b.as_bytes()),t,c),Err(ErrorEnlace::Correspondencia)));pasa("LG1-05");}
#[test]fn f_lg06_traza_intercambiada(){let(m,t,c)=productos();let s=std::str::from_utf8(&t).unwrap();let start=s.find("\"inicio\":").unwrap()+9;let stop=start+s[start..].find(',').unwrap();let bad=format!("{}0{}",&s[..start],&s[stop..]);assert!(matches!(con(m,toc(bad),c),Err(ErrorEnlace::Correspondencia)));pasa("LG1-06");}
fn toc(s:String)->Vec<u8>{s.into_bytes()}
#[test]fn g_lg07_duplicado(){let mut e=nuevo();e.ejecutar(0).unwrap();let h=huella(e.cuerpo(0).unwrap()).unwrap();assert_eq!(e.ejecutar(0),Err(ErrorEnlace::Orden));assert_eq!(huella(e.cuerpo(0).unwrap()).unwrap(),h);pasa("LG1-07");}
#[test]fn h_lg08_orden(){let mut e=nuevo();assert_eq!(e.ejecutar(1),Err(ErrorEnlace::Orden));assert_eq!(e.ejecutar(24),Err(ErrorEnlace::Orden));assert_eq!(e.reservas_g1().unwrap(),0);pasa("LG1-08");}
#[test]fn i_lg09_manejador_cruzado(){let mut e=nuevo();for i in 0..5{e.ejecutar(i).unwrap();}let h0=e.posiciones[0].as_mut().unwrap().manejador.take();let h4=e.posiciones[4].as_mut().unwrap().manejador.take();e.posiciones[0].as_mut().unwrap().manejador=h4;e.posiciones[4].as_mut().unwrap().manejador=h0;assert!(matches!(e.cuerpo(0),Err(ErrorEnlace::Receptor(g1::Fallo::Pertenencia))));pasa("LG1-09");}
#[test]fn j_lg10_incompleto(){let mut e=nuevo();for i in 0..23{e.ejecutar(i).unwrap();}assert_eq!(e.completo(),Err(ErrorEnlace::Incompleto));for i in 0..23{e.cuerpo(i).unwrap();}assert!(matches!(e.cuerpo(23),Err(ErrorEnlace::Incompleto)));pasa("LG1-10");}
#[test]fn k_lg11_cupo(){assert_eq!(frontera(AGREGADO-MARGEN),Ok(()));assert_eq!(frontera(AGREGADO-MARGEN+1),Err(ErrorEnlace::Limite));assert_eq!(frontera(u64::MAX),Err(ErrorEnlace::Limite));pasa("LG1-11");}
#[test]fn l_lg12_invocaciones_distintas(){let(mut a,mut b)=(nuevo(),nuevo());a.ejecutar(0).unwrap();b.ejecutar(0).unwrap();assert_eq!(a.cuerpo(0).unwrap(),b.cuerpo(0).unwrap());assert_ne!(a.identidad(0).unwrap(),b.identidad(0).unwrap());pasa("LG1-12");}
