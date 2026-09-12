#![forbid(unsafe_code)]
use lote::{EnlacePublico,ErrorEnlace,lectura::{PropuestaDeLectura,FalloLectura,VERSION,OPERACION}};
use cobertura::{Referencia,Seleccion,FalloCobertura};
const L:&[u8]=include_bytes!("../fuentes-base/lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../fuentes-base/lote-p3/MONTAJES_PUBLICOS.json");
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn realizacion()->g1::Realizacion{g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}}
fn nuevo()->EnlacePublico{EnlacePublico::preparar(L,M,realizacion()).unwrap()}
fn ejecutar(e:&mut EnlacePublico,i:usize){e.ejecutar(i).unwrap();let m=e.recuperar(i,g1::Papel::Marco).unwrap().to_vec();e.comprobar_entrega(i,&m).unwrap();}
fn propuesta<'a>(id:(u64,u64),texto:&'a[u8],caso:Option<&'a[u8]>,vigencia:Option<&'a[u8]>)->Seleccion<'a>{Seleccion{lectura:PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:id,texto},caso,vigencia}}
fn comprobar(id:&str,ok:bool,resultado:&str){if !ok{eprintln!("FALLO {id}: {resultado}");std::process::exit(1)}println!("{id}\t{resultado}");}
fn guardar(dir:&std::path::Path,id:&str,part:&str,b:&[u8]){std::fs::write(dir.join(format!("{id}.{part}")),b).unwrap()}
fn main(){
 let dir=std::path::PathBuf::from(std::env::var("S1_CAPTURAS").unwrap());
 let mut e=nuevo();for i in 0..4{ejecutar(&mut e,i);guardar(&dir,&format!("P3-{:02}",i+1),"cuerpo",e.cuerpo(i).unwrap());}
 // El conductor fija ambas referencias antes de formar las selecciones.
 let r=Referencia::desde(&e,3).unwrap();let r0=Referencia::desde(&e,0).unwrap();
 let texto=e.cuerpo(3).unwrap();let caso=r.caso_requerido();let vigencia=r.vigencia_requerida();
 let completa=r.comprobar(propuesta(r.identidad(),texto,Some(caso),Some(vigencia))).unwrap();
 comprobar("CI01",completa.lectura().texto()==texto&&completa.caso_citado()==caso&&completa.vigencia_citada()==vigencia,"ENTREGADO_CON_COBERTURA");
 guardar(&dir,"CI01","entregado",completa.lectura().texto());guardar(&dir,"CI01","caso-citado",completa.caso_citado());guardar(&dir,"CI01","vigencia-citada",completa.vigencia_citada());
 guardar(&dir,"CI01","solicitud",completa.lectura().solicitud());guardar(&dir,"CI01","traza",completa.lectura().traza_a());guardar(&dir,"CI01","identidad",format!("{},{}",r.identidad().0,r.identidad().1).as_bytes());
 let sin_vigencia=r.comprobar(propuesta(r.identidad(),texto,Some(caso),None));
 guardar(&dir,"CI02","cuerpo-propuesto",texto);guardar(&dir,"CI02","caso-citado",caso);guardar(&dir,"CI02","vigencia-omitida",vigencia);guardar(&dir,"CI02","identidad",format!("{},{}",r.identidad().0,r.identidad().1).as_bytes());
 comprobar("CI02",matches!(sin_vigencia,Err(FalloCobertura::FaltaVigencia)),"FaltaVigencia");guardar(&dir,"CI02","rechazo",b"FaltaVigencia");
 let reducido=b"{\"casos\":[]}";let circular=EnlacePublico::preparar(L,reducido,realizacion());
 comprobar("CI03",matches!(circular,Err(ErrorEnlace::IdentidadMontaje)),"IdentidadMontaje");guardar(&dir,"CI03","montaje-propuesto",reducido);guardar(&dir,"CI03","rechazo",b"IdentidadMontaje");
 let mut alterada=vigencia.to_vec();let last=alterada.len()-1;alterada[last]=b'!';
 let alteracion=r.comprobar(propuesta(r.identidad(),texto,Some(caso),Some(&alterada)));
 comprobar("CI04",matches!(alteracion,Err(FalloCobertura::VigenciaDistinta)),"VigenciaDistinta");guardar(&dir,"CI04","vigencia-propuesta",&alterada);guardar(&dir,"CI04","rechazo",b"VigenciaDistinta");
 comprobar("CI05",matches!(r.comprobar(propuesta(r.identidad(),texto,None,Some(vigencia))),Err(FalloCobertura::FaltaCaso)),"FaltaCaso");guardar(&dir,"CI05","rechazo",b"FaltaCaso");
 let mut otra=nuevo();ejecutar(&mut otra,0);let ajena=otra.identidad(0).unwrap();
 comprobar("CI06",matches!(r.comprobar(propuesta(ajena,texto,Some(caso),Some(vigencia))),Err(FalloCobertura::Lectura(FalloLectura::Identidad))),"Identidad");guardar(&dir,"CI06","rechazo",b"Identidad");guardar(&dir,"CI06","identidad-propuesta",format!("{},{}",ajena.0,ajena.1).as_bytes());
 let mut cuerpo_alterado=texto.to_vec();cuerpo_alterado[0]=b'[';
 comprobar("CI07",matches!(r.comprobar(propuesta(r.identidad(),&cuerpo_alterado,Some(caso),Some(vigencia))),Err(FalloCobertura::Lectura(FalloLectura::Presentacion(g1::FalloPresentacion::ContenidoDistinto)))) ,"ContenidoDistinto");guardar(&dir,"CI07","cuerpo-propuesto",&cuerpo_alterado);guardar(&dir,"CI07","rechazo",b"ContenidoDistinto");
 let body0=e.cuerpo(0).unwrap();let c0=r0.caso_requerido();let v0=r0.vigencia_requerida();
 let control=r0.comprobar(propuesta(r0.identidad(),body0,Some(c0),Some(v0))).unwrap();
 comprobar("CI08",control.lectura().texto()==body0&&control.caso_citado()==c0&&control.vigencia_citada()==v0,"ENTREGADO_CON_COBERTURA");
 guardar(&dir,"CI08","entregado",control.lectura().texto());guardar(&dir,"CI08","caso-citado",control.caso_citado());guardar(&dir,"CI08","vigencia-citada",control.vigencia_citada());
 guardar(&dir,"comun","lote",e.originales().0);guardar(&dir,"comun","montaje",e.originales().1);
}
