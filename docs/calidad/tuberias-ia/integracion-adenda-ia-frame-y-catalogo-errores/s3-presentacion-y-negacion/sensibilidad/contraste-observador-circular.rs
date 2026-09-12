#![forbid(unsafe_code)]
mod destino;
use std::path::Path;
use lote::{EnlacePublico,lectura::{PropuestaDeLectura,FalloLectura,VERSION,OPERACION}};
use cobertura::{Referencia,Seleccion,FalloCobertura};
use destino::FalloDestino;
const L:&[u8]=include_bytes!("../fuentes-base/lote-p3/LOTE_SINTETICO_PUBLICO.json");
const M:&[u8]=include_bytes!("../fuentes-base/lote-p3/MONTAJES_PUBLICOS.json");
const ORIGINAL:&[u8]=include_bytes!("../especimenes/literal.json");
const ESPACIADA:&[u8]=include_bytes!("../especimenes/espaciada.json");
const ALTERADA:&[u8]=include_bytes!("../especimenes/sin-negacion.json");
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();assert_eq!(s.len(),64);std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn propuesta<'a>(r:&Referencia<'_>,texto:&'a[u8],caso:Option<&'a[u8]>,vigencia:Option<&'a[u8]>)->Seleccion<'a>{Seleccion{lectura:PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:r.identidad(),texto},caso,vigencia}}
fn comprobar(id:&str,ok:bool,resultado:&str){if !ok{eprintln!("FALLO {id}: {resultado}");std::process::exit(1)}println!("{id}\t{resultado}");}
fn guardar(dir:&Path,name:&str,b:&[u8]){std::fs::write(dir.join(name),b).unwrap();}
fn distinto(e:&FalloCobertura)->bool{matches!(e,FalloCobertura::Lectura(FalloLectura::Presentacion(g1::FalloPresentacion::ContenidoDistinto)))}
fn main(){
 let dir=std::path::PathBuf::from(std::env::var("S3_CAPTURAS").unwrap());
 let mut e=EnlacePublico::preparar(L,M,g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}).unwrap();
 e.ejecutar(0).unwrap();let marco=e.recuperar(0,g1::Papel::Marco).unwrap().to_vec();e.comprobar_entrega(0,&marco).unwrap();
 assert_eq!(e.cuerpo(0).unwrap(),ORIGINAL);
 let r=Referencia::desde(&e,0).unwrap();let caso=r.caso_requerido();let vigencia=r.vigencia_requerida();
 guardar(&dir,"referencia.cuerpo",e.cuerpo(0).unwrap());guardar(&dir,"referencia.caso",caso);guardar(&dir,"referencia.vigencia",vigencia);
 guardar(&dir,"referencia.solicitud",e.recuperar(0,g1::Papel::Entrada).unwrap());guardar(&dir,"referencia.traza",e.recuperar(0,g1::Papel::Traza).unwrap());
 guardar(&dir,"referencia.marco",&marco);guardar(&dir,"referencia.montaje",M);guardar(&dir,"referencia.lote",L);
 for (id,texto,estado) in [("D01",ORIGINAL,"LITERAL_RECUPERADO"),("D02",ESPACIADA,"FORMATO_RECUPERADO")]{
  let validada=r.comprobar(propuesta(&r,texto,Some(caso),Some(vigencia))).unwrap();let path=dir.join(format!("{id}.destino"));
  destino::escribir(&path,&validada).unwrap();let recuperado=destino::leer(&path).unwrap();destino::verificar(&r,&recuperado,caso,vigencia).unwrap();
  guardar(&dir,&format!("{id}.recuperado"),&recuperado);comprobar(id,recuperado==texto&&e.cuerpo(0).unwrap()==ORIGINAL,estado);
 }
 // Ataque antes de la escritura: el archivo de destino no se abre al rechazar la propuesta.
 let path=dir.join("D03.destino");let pre=r.comprobar(propuesta(&r,ALTERADA,Some(caso),Some(vigencia)));
 let rechazado=match pre{Err(ref causa)=>distinto(causa),Ok(ref entrega)=>{destino::escribir(&path,entrega).unwrap();false}};
 guardar(&dir,"D03.propuesta",ALTERADA);guardar(&dir,"D03.destino-existe",if path.exists(){b"true"}else{b"false"});
 comprobar("D03",rechazado&&!path.exists(),"PERDIDA_PREVIA_RECHAZADA_SIN_ESCRITURA");guardar(&dir,"D03.causa",b"Lectura(Presentacion(ContenidoDistinto))");
 // Ataque posterior: se conserva el objeto validado y se altera sólo el archivo escrito.
 let valida=r.comprobar(propuesta(&r,ESPACIADA,Some(caso),Some(vigencia))).unwrap();let path=dir.join("D04.destino");destino::escribir(&path,&valida).unwrap();
 let previa=destino::leer(&path).unwrap();assert_eq!(previa,ESPACIADA);guardar(&dir,"D04.antes",&previa);
 std::fs::write(&path,ALTERADA).unwrap();let recuperado=destino::leer(&path).unwrap();guardar(&dir,"D04.recuperado",&recuperado);
 let observado=destino::verificar(&r,&previa,caso,vigencia);
 comprobar("D04",matches!(&observado,Err(FalloDestino::Cobertura(c)) if distinto(c))&&valida.lectura().texto()==ESPACIADA&&e.cuerpo(0).unwrap()==ORIGINAL,"PERDIDA_POSTERIOR_DETECTADA_EN_DESTINO");guardar(&dir,"D04.causa",b"Cobertura(Lectura(Presentacion(ContenidoDistinto)))");
 let path=dir.join("D05.destino");let incompleta=r.comprobar(propuesta(&r,ESPACIADA,Some(caso),None));
 let omitida=match incompleta{Err(FalloCobertura::FaltaVigencia)=>true,Ok(ref entrega)=>{destino::escribir(&path,entrega).unwrap();false},_=>false};
 guardar(&dir,"D05.propuesta",ESPACIADA);guardar(&dir,"D05.destino-existe",if path.exists(){b"true"}else{b"false"});
 comprobar("D05",omitida&&!path.exists(),"OMISION_RECHAZADA_SIN_ESCRITURA");guardar(&dir,"D05.causa",b"FaltaVigencia");
 let path=dir.join("D06.destino");std::fs::write(&path,include_bytes!("../especimenes/exceso.bin")).unwrap();
 comprobar("D06",matches!(destino::leer(&path),Err(FalloDestino::Limite)),"LIMITE_DE_LECTURA");guardar(&dir,"D06.causa",b"Limite");
 let path=dir.join("D07.destino");std::fs::write(&path,b"PREEXISTENTE").unwrap();let previo=destino::escribir(&path,&valida);
 comprobar("D07",matches!(previo,Err(FalloDestino::Io(std::io::ErrorKind::AlreadyExists)))&&destino::leer(&path).unwrap()==b"PREEXISTENTE","DESTINO_EXISTENTE_CONSERVADO");guardar(&dir,"D07.causa",b"Io(AlreadyExists)");
 comprobar("D08",matches!(destino::leer(&dir.join("D08.ausente")),Err(FalloDestino::Io(std::io::ErrorKind::NotFound))),"DESTINO_AUSENTE");guardar(&dir,"D08.causa",b"Io(NotFound)");
 assert_eq!(e.cuerpo(0).unwrap(),ORIGINAL);guardar(&dir,"referencia.final",e.cuerpo(0).unwrap());
}
