use lote::{EnlacePublico,lectura::{PropuestaDeLectura,VERSION,OPERACION}};
fn h(k:&str)->[u8;32]{let s=std::env::var(k).unwrap();std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
fn main(){
 let l=include_bytes!("../fuentes-base/lote-p3/LOTE_SINTETICO_PUBLICO.json");let m=include_bytes!("../fuentes-base/lote-p3/MONTAJES_PUBLICOS.json");
 let mut e=EnlacePublico::preparar(l,m,g1::Realizacion{fuentes_sha256:h("LG1_FUENTES"),binario_sha256:h("LG1_BINARIO")}).unwrap();
 for i in 0..4{e.ejecutar(i).unwrap();let marco=e.recuperar(i,g1::Papel::Marco).unwrap().to_vec();e.comprobar_entrega(i,&marco).unwrap();}
 let r=cobertura::Referencia::desde(&e,3).unwrap();let mut texto=e.cuerpo(3).unwrap().to_vec();texto[0]=b'[';
 let dir=std::path::PathBuf::from(std::env::var("S1_CAPTURAS").unwrap());std::fs::write(dir.join("CI07.propuesta-inicial"),&texto).unwrap();
 let p=cobertura::Seleccion{lectura:PropuestaDeLectura{version:VERSION,operacion:OPERACION,identidad:r.identidad(),texto:&texto},caso:Some(r.caso_requerido()),vigencia:Some(r.vigencia_requerida())};
 match r.comprobar(p){Ok(_)=>panic!("Se esperaba rechazo"),Err(e)=>{let s=format!("{e:?}");println!("{s}");std::fs::write(dir.join("CI07.causa-observada"),s).unwrap();}}
}
